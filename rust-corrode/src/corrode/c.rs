use corollary_support::*;

use Control::Monad;
use Control::Monad::ST;
use Control::Monad::Trans::Class;
use Control::Monad::Trans::Except;
use Control::Monad::Trans::RWS::Strict;
use Data::Foldable;
use Data::Map::Lazy;
use Data::IntMap::Strict;
use Data::Maybe;
use Data::List;
use Data::STRef;
use Data::Set;
use Language::C;
use Language::C::Data::Ident;
use Language::Rust::AST;
use Language::Rust::Corrode::CFG;
use Language::Rust::Corrode::CrateMap;
use Text::PrettyPrint::HughesPJClass;

struct FunctionContext{
    functionReturnType: Option<CType>,
    functionName: Option<String>,
    itemRewrites: ItemRewrites
}

struct Output{
    outputItems: Vec<Rust::Item>,
    outputExterns: Map::Map<String, Rust::ExternItem>,
    outputIncomplete: Set::Set<String>
}

struct GlobalState{
    unique: isize,
    usedForwardRefs: Set::Set<Ident>
}

struct EnvState<s>{
    symbolEnvironment: Vec<(Ident, EnvMonad<s, Result>)>,
    typedefEnvironment: Vec<(Ident, EnvMonad<s, IntermediateType>)>,
    tagEnvironment: Vec<(Ident, EnvMonad<s, CType>)>,
    globalState: GlobalState
}

struct Initializer(Option<Rust::Expr>, IntMap::IntMap<Initializer>);

#[derive(Debug)]
pub enum Designator {
    Base(CType),
    From(CType, isize, Vec<CType>, Designator)
}
pub use self::Designator::*;

struct OuterLabels{
    onBreak: Option<Label>,
    onContinue: Option<Label>,
    switchExpression: Option<CExpr>
}

struct Result{
    resultType: CType,
    resultMutable: Rust::Mutable,
    result: Rust::Expr
}

#[derive(Debug, Eq)]
pub enum Signed {
    Signed,
    Unsigned
}
pub use self::Signed::*;

#[derive(Debug, Eq)]
pub enum IntWidth {
    BitWidth(isize),
    WordWidth
}
pub use self::IntWidth::*;

#[derive(Debug)]
pub enum CType {
    IsBool,
    IsInt(Signed, IntWidth),
    IsFloat(isize),
    IsVoid,
    IsFunc(CType, Vec<(Option<(Rust::Mutable, Ident)>, CType)>, bool),
    IsPtr(Rust::Mutable, CType),
    IsArray(Rust::Mutable, isize, CType),
    IsStruct(String, Vec<(String, CType)>),
    IsEnum(String),
    IsIncomplete(Ident)
}
pub use self::CType::*;

struct IntermediateType{
    typeMutable: Rust::Mutable,
    typeIsFunc: bool,
    typeRep: CType
}

pub fn addExternIdent(ident: Ident, deferred: EnvMonad<s, IntermediateType>, mkItem: fn(String) -> fn((Rust::Mutable, CType)) -> Rust::ExternItem) -> EnvMonad<s, ()> {
    /* do */ {
        let action = runOnce(/* do */ {
                let itype = deferred;

                let rewrites = lift(asks(itemRewrites));

                let path = match Map::lookup((Symbol, identToString(ident)), rewrites) {
                        Some(renamed) => {
                            (__op_concat("".to_string(), renamed))
                        },
                        None => {
                            /* do */ {
                                let name = applyRenames(ident);

                                let ty = (typeMutable(itype), typeRep(itype));

                                lift(tell(mempty {
                                        outputExterns: Map::singleton(name, (mkItem(name, ty)))
                                    }));
                                vec![name]
                            }
                        },
                    };

                (typeToResult(itype, (Rust::Path((Rust::PathSegments(path))))))
            });

        addSymbolIdentAction(ident, action)
    }
}

pub fn addSwitchCase(condition: Option<CExpr>, body: CStat, next: CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)>) -> CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)> {
    /* do */ {
        let condition_q = lift(lift(mapM((interpretExpr(true)), condition)));

        let next_q = interpretStatement(body, next);

        let label = match next_q {
                ([], Branch(to)) => {
                    to
                },
                (rest, end) => {
                    /* do */ {
                        let label = newLabel;

                        addBlock(label, rest, end);
                        label
                    }
                },
            };

        lift(tell(SwitchCases(IntMap::singleton(label, condition_q))));
        (vec![], Branch(label))
    }
}

pub fn addSymbolIdent(ident: Ident, (__mut, ty): (Rust::Mutable, CType)) -> EnvMonad<s, String> {
    /* do */ {
        let name = applyRenames(ident);

        addSymbolIdentAction(ident)(Result {
            resultType: ty,
            resultMutable: __mut,
            result: Rust::Path((Rust::PathSegments(vec![name])))
        });
        name
    }
}

pub fn addSymbolIdentAction(ident: Ident, action: EnvMonad<s, Result>) -> EnvMonad<s, ()> {
    lift(/* do */ {
        modify(|st| { st {
                symbolEnvironment: __op_concat((ident, action), symbolEnvironment(st))
            } })
    })
}

pub fn addTagIdent(ident: Ident, ty: EnvMonad<s, CType>) -> EnvMonad<s, ()> {
    lift(/* do */ {
        modify(|st| { st {
                tagEnvironment: __op_concat((ident, ty), tagEnvironment(st))
            } })
    })
}

pub fn addTypedefIdent(ident: Ident, ty: EnvMonad<s, IntermediateType>) -> EnvMonad<s, ()> {
    lift(/* do */ {
        modify(|st| { st {
                typedefEnvironment: __op_concat((ident, ty), typedefEnvironment(st))
            } })
    })
}

pub fn applyRenames(ident: Ident) -> String {
    match identToString(ident) {
        "final" => {
            "final_".to_string()
        },
        "fn" => {
            "fn_".to_string()
        },
        "in" => {
            "in_".to_string()
        },
        "let" => {
            "let_".to_string()
        },
        "main" => {
            "_c_main".to_string()
        },
        "match" => {
            "match_".to_string()
        },
        "mod" => {
            "mod_".to_string()
        },
        "proc" => {
            "proc_".to_string()
        },
        "type" => {
            "type_".to_string()
        },
        "where" => {
            "where_".to_string()
        },
        name => {
            name
        },
    }
}

pub fn badSource(node: node, msg: String) -> EnvMonad<s, a> {
    noTranslation(node, (__op_addadd("illegal ".to_string(), __op_addadd(msg, "; check whether a real C compiler accepts this".to_string()))))
}

pub fn baseTypeOf(specs: Vec<CDeclSpec>) -> EnvMonad<s, (Option<CStorageSpec>, EnvMonad<s, IntermediateType>)> {
    /* do */ {
        let (storage, _attributes, basequals, basespecs, _inlineNoReturn, _align) = partitionDeclSpecs(specs);

        let mstorage = match storage {
                [] => {
                    None
                },
                [spec] => {
                    (Some(spec))
                },
                [_, [excess, _]] => {
                    badSource(excess, "extra storage class specifier".to_string())
                },
            };

        let base = typedef((mutable(basequals)), basespecs);

        (mstorage, base)
    }
}

pub fn binop(expr: CExpr, op: CBinaryOp, lhs: Result, rhs: Result) -> EnvMonad<s, Result> {
    fmap(wrapping)(match op {
        CMulOp => {
            promote(expr, Rust::Mul, lhs, rhs)
        },
        CDivOp => {
            promote(expr, Rust::Div, lhs, rhs)
        },
        CRmdOp => {
            promote(expr, Rust::Mod, lhs, rhs)
        },
        CAddOp => {
            match (toPtr(lhs), toPtr(rhs)) {
                (Some(ptr), _) => {
                    (offset(ptr, rhs))
                },
                (_, Some(ptr)) => {
                    (offset(ptr, lhs))
                },
                _ => {
                    promote(expr, Rust::Add, lhs, rhs)
                },
            }
        },
        CSubOp => {
            match (toPtr(lhs), toPtr(rhs)) {
                (Some(lhs_q), Some(rhs_q)) => {
                    /* do */ {
                        let ptrTo = match compatiblePtr((resultType(lhs_q)), (resultType(rhs_q))) {
                                IsPtr(_, ptrTo) => {
                                    ptrTo
                                },
                                _ => {
                                    badSource(expr, "pointer subtraction of incompatible pointers".to_string())
                                },
                            };

                        let ty = IsInt(Signed, WordWidth);

                        let size = rustSizeOfType((toRustType(ptrTo)));

                        Result {
                            resultType: ty,
                            resultMutable: Rust::Immutable,
                            result: __op_div((Rust::MethodCall((castTo(ty, lhs_q)), (Rust::VarName("wrapping_sub".to_string())), vec![castTo(ty, rhs_q)])), castTo(ty, size))
                        }
                    }
                },
                (Some(ptr), _) => {
                    ptr {
                        result: Rust::MethodCall((result(ptr)), (Rust::VarName("offset".to_string())), vec![Rust::Neg((castTo((IsInt(Signed, WordWidth)), rhs)))])
                    }
                },
                _ => {
                    promote(expr, Rust::Sub, lhs, rhs)
                },
            }
        },
        CShlOp => {
            shift(Rust::ShiftL)
        },
        CShrOp => {
            shift(Rust::ShiftR)
        },
        CLeOp => {
            comparison(Rust::CmpLT)
        },
        CGrOp => {
            comparison(Rust::CmpGT)
        },
        CLeqOp => {
            comparison(Rust::CmpLE)
        },
        CGeqOp => {
            comparison(Rust::CmpGE)
        },
        CEqOp => {
            comparison(Rust::CmpEQ)
        },
        CNeqOp => {
            comparison(Rust::CmpNE)
        },
        CAndOp => {
            promote(expr, Rust::And, lhs, rhs)
        },
        CXorOp => {
            promote(expr, Rust::Xor, lhs, rhs)
        },
        COrOp => {
            promote(expr, Rust::Or, lhs, rhs)
        },
        CLndOp => {
            Result {
                resultType: IsBool,
                resultMutable: Rust::Immutable,
                result: Rust::LAnd((toBool(lhs)), (toBool(rhs)))
            }
        },
        CLorOp => {
            Result {
                resultType: IsBool,
                resultMutable: Rust::Immutable,
                result: Rust::LOr((toBool(lhs)), (toBool(rhs)))
            }
        },
    })
}

pub fn bitWidth(__0: isize, __1: IntWidth) -> isize {
    match (__0, __1) {
        (wordWidth, WordWidth) => {
            wordWidth
        },
        (_, BitWidth(w)) => {
            w
        },
    }
}

pub fn blockToStatements(Rust::Block(stmts, mexpr): Rust::Block) -> Vec<Rust::Stmt> {
    match mexpr {
        Some(expr) => {
            __op_addadd(stmts, exprToStatements(expr))
        },
        None => {
            stmts
        },
    }
}

pub fn castTo(__0: CType, __1: Result) -> Rust::Expr {
    match (__0, __1) {
        (target, source) => {
            /* Expr::Error */ Error
        },
        (target, Result {

            }) => {
            castTo(target, Result {
                    resultType: IsPtr(__mut, el),
                    resultMutable: Rust::Immutable,
                    result: Rust::MethodCall(source, (Rust::VarName(method)), vec![])
                })
        },
        (IsBool, source) => {
            toBool(source)
        },
        (target, __OP__, IsInt {

            }, Result {

            }) => {
            Rust::Lit((Rust::LitInt(n, repr, (toRustType(target)))))
        },
        (IsInt(Signed, w), Result {

            }) => {
            Rust::Neg((Rust::Lit((Rust::LitInt(n, repr, (toRustType((IsInt(Signed, w)))))))))
        },
        (target, source) => {
            Rust::Cast((result(source)), (toRustType(target)))
        },
    }
}

pub fn cfgToRust(_node: node, build: CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)>) -> EnvMonad<s, Vec<Rust::Stmt>> {
    /* do */ {
        let builder = buildCFG(/* do */ {
                let (early, term) = build;

                let entry = newLabel;

                addBlock(entry, early, term);
                entry
            });

        let (rawCFG, _) = evalRWST(builder, (OuterLabels(None, None, None)), Map::empty);

        let cfg = depthFirstOrder((removeEmptyBlocks(rawCFG)));

        let (hasGoto, structured) = structureCFG(mkBreak, mkContinue, mkLoop, mkIf, mkGoto, mkMatch, cfg);

        return(__op_concat(__TODO_if(hasGoto, then, declCurrent), structured(__TODO_else, structured)))
    }
}

pub fn charType() -> CType {
    IsInt(Unsigned, (BitWidth(8)))
}

pub fn compatibleInitializer(__0: CType, __1: CType) -> bool {
    match (__0, __1) {
        (IsStruct(name1, _), IsStruct(name2, _)) => {
            (name1 == name2)
        },
        (IsStruct {

        }, _) => {
            false
        },
        (_, IsStruct {

        }) => {
            false
        },
        (_, _) => {
            true
        },
    }
}

pub fn compatiblePtr(__0: CType, __1: CType) -> CType {
    match (__0, __1) {
        (IsPtr(_, IsVoid), b) => {
            b
        },
        (IsArray(__mut, _, el), b) => {
            compatiblePtr((IsPtr(__mut, el)), b)
        },
        (a, IsPtr(_, IsVoid)) => {
            a
        },
        (a, IsArray(__mut, _, el)) => {
            compatiblePtr(a, (IsPtr(__mut, el)))
        },
        (IsPtr(m1, a), IsPtr(m2, b)) => {
            IsPtr((leastMutable(m1, m2)), (compatiblePtr(a, b)))
        },
        (a, b) => {
            /* Expr::Error */ Error
        },
        (_, _) => {
            IsVoid
        },
    }
}

pub fn completeType(__0: CType, __1: EnvMonad<s, CType>) -> EnvMonad<s, CType> {
    match (__0, __1, __2) {
        (orig, __OP__, IsIncomplete(ident)) => {
            /* do */ {
                let mty = getTagIdent(ident);

                fromMaybe((orig), mty)
            }
        },
        ty => {
            ty
        },
    }
}

pub fn compound(expr: CExpr, returnOld: bool, demand: bool, op: CAssignOp, lhs: Result, rhs: Result) -> EnvMonad<s, Result> {
    /* do */ {
        let op_q = match op {
                CAssignOp => {
                    None
                },
                CMulAssOp => {
                    Some(CMulOp)
                },
                CDivAssOp => {
                    Some(CDivOp)
                },
                CRmdAssOp => {
                    Some(CRmdOp)
                },
                CAddAssOp => {
                    Some(CAddOp)
                },
                CSubAssOp => {
                    Some(CSubOp)
                },
                CShlAssOp => {
                    Some(CShlOp)
                },
                CShrAssOp => {
                    Some(CShrOp)
                },
                CAndAssOp => {
                    Some(CAndOp)
                },
                CXorAssOp => {
                    Some(CXorOp)
                },
                COrAssOp => {
                    Some(COrOp)
                },
            };

        let duplicateLHS = (isJust(op_q) || demand);

        let (bindings1, dereflhs, boundrhs) = (__TODO_if(not, duplicateLHS) || hasNoSideEffects((result(lhs)), then, (vec![], lhs, rhs), __TODO_else, {
                    let lhsvar = Rust::VarName("_lhs".to_string());

                    let rhsvar = Rust::VarName("_rhs".to_string());

                (vec![
                        Rust::Let(Rust::Immutable, rhsvar, None, (Some((result(rhs))))),
                        Rust::Let(Rust::Immutable, lhsvar, None, (Some((Rust::Borrow(Rust::Mutable, (result(lhs))))))),
                    ], lhs {
                        result: Rust::Deref((Rust::Var(lhsvar)))
                    }, rhs {
                        result: Rust::Var(rhsvar)
                    })                }));

        let rhs_q = match op_q {
                Some(o) => {
                    binop(expr, o, dereflhs, boundrhs)
                },
                None => {
                    boundrhs
                },
            };

        let assignment = Rust::Assign((result(dereflhs)), (Rust::__id_3a3d), (castTo((resultType(lhs)), rhs_q)));

        let (bindings2, ret) = __TODO_if(not, demand, then, (vec![], None), __TODO_else, __TODO_if, not, returnOld, then, (vec![], Some((result(dereflhs)))), __TODO_else, {
                    let oldvar = Rust::VarName("_old".to_string());

                (vec![Rust::Let(Rust::Immutable, oldvar, None, (Some((result(dereflhs)))))], Some((Rust::Var(oldvar))))                });

        return(match Rust::Block((__op_addadd(bindings1, __op_addadd(bindings2, exprToStatements(assignment)))), ret) {
            b(__OP__, Rust::Block(body, None)) => {
                Result {
                    resultType: IsVoid,
                    resultMutable: Rust::Immutable,
                    result: match body {
                            [Rust::Stmt(e)] => {
                                e
                            },
                            _ => {
                                Rust::BlockExpr(b)
                            },
                        }
                }
            },
            b => {
                lhs {
                    result: Rust::BlockExpr(b)
                }
            },
        })
    }
}

pub fn derivedDeferredTypeOf(deferred: EnvMonad<s, IntermediateType>, declr: CDeclr, __OP__: Vec<CDecl>, CDeclr(_, derived, _, _, _): EnvMonad<s, EnvMonad<s, IntermediateType>>) -> EnvMonad<s, EnvMonad<s, IntermediateType>> {
    /* do */ {
        let derived_q = mapM(derive, derived);

        return(/* do */ {
            let basetype = deferred;

            foldrM((__op_dollar), basetype, derived_q)
        })
    }
}

pub fn derivedTypeOf(deferred: EnvMonad<s, IntermediateType>, declr: CDeclr) -> EnvMonad<s, IntermediateType> {
    join((derivedDeferredTypeOf(deferred, declr, vec![])))
}

pub fn designatorType(__0: Designator) -> CType {
    match (__0) {
        Base(ty) => {
            ty
        },
        From(ty, _, _, _) => {
            ty
        },
    }
}

pub fn emitIncomplete(kind: ItemKind, ident: Ident) -> EnvMonad<s, CType> {
    /* do */ {
        let rewrites = lift((asks(itemRewrites)));

        unless((Map::member((kind, identToString(ident)), rewrites)))(lift(tell(mempty {
                outputIncomplete: Set::singleton((identToString(ident)))
            })));
        (IsIncomplete(ident))
    }
}

pub fn emitItems(items: Vec<Rust::Item>) -> EnvMonad<s, ()> {
    lift(tell(mempty {
            outputItems: items
        }))
}

pub fn enumReprType() -> CType {
    IsInt(Signed, (BitWidth(32)))
}

pub fn exprToStatements(__0: Rust::Expr) -> Vec<Rust::Stmt> {
    match (__0) {
        Rust::IfThenElse(c, t, f) => {
            vec![Rust::Stmt((Rust::IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))]
        },
        Rust::BlockExpr(b) => {
            blockToStatements(b)
        },
        e => {
            vec![Rust::Stmt(e)]
        },
    }
}

pub fn getSwitchCases(expr: CExpr) -> CSourceBuildCFGT<s, (a, SwitchCases)> {
    mapBuildCFGT(wrap)
}

pub fn getSwitchExpression(stmt: CStat) -> CSourceBuildCFGT<s, CExpr> {
    /* do */ {
        let mexpr = lift(asks(switchExpression));

        match mexpr {
            None => {
                lift(lift(badSource(stmt, "case outside switch".to_string())))
            },
            Some(expr) => {
                expr
            },
        }
    }
}

pub fn getSymbolIdent(ident: Ident) -> EnvMonad<s, Option<Result>> {
    /* do */ {
        let env = lift(get);

        match lookup(ident, (symbolEnvironment(env))) {
            Some(symbol) => {
                fmap(Some, symbol)
            },
            None => {
                match identToString(ident) {
                    "__func__" => {
                        getFunctionName("".to_string())
                    },
                    "__FUNCTION__" => {
                        getFunctionName("".to_string())
                    },
                    "__PRETTY_FUNCTION__" => {
                        getFunctionName("top level".to_string())
                    },
                    name => {
                        return(lookup(name, builtinSymbols))
                    },
                }
            },
        }
    }
}

pub fn getTagIdent(ident: Ident) -> EnvMonad<s, Option<EnvMonad<s, CType>>> {
    lift(/* do */ {
        let env = gets(tagEnvironment);

        return(lookup(ident, env))
    })
}

pub fn getTypedefIdent(ident: Ident) -> EnvMonad<s, (String, Option<EnvMonad<s, IntermediateType>>)> {
    lift(/* do */ {
        let env = gets(typedefEnvironment);

        (identToString(ident), lookup(ident, env))
    })
}

pub fn gotoLabel(ident: Ident) -> CSourceBuildCFGT<s, Label> {
    /* do */ {
        let labels = lift(get);

        match Map::lookup(ident, labels) {
            None => {
                /* do */ {
                    let label = newLabel;

                    lift((put((Map::insert(ident, label, labels)))));
                    label
                }
            },
            Some(label) => {
                label
            },
        }
    }
}

pub fn intPromote(__0: CType) -> CType {
    match (__0) {
        IsBool => {
            IsInt(Signed, (BitWidth(32)))
        },
        IsEnum(_) => {
            enumReprType
        },
        IsInt(_, BitWidth(w)) => {
            /* Expr::Error */ Error
        },
        x => {
            x
        },
    }
}

pub fn integerConversionRank(__0: IntWidth, __1: IntWidth) -> Option<Ordering> {
    match (__0, __1) {
        (BitWidth(a), BitWidth(b)) => {
            Some((compare(a, b)))
        },
        (WordWidth, WordWidth) => {
            Some(EQ)
        },
        (BitWidth(a), WordWidth) => {
            /* Expr::Error */ Error
        },
        (WordWidth, BitWidth(b)) => {
            /* Expr::Error */ Error
        },
        (_, _) => {
            None
        },
    }
}

pub fn interpretBlockItem(__0: CBlockItem, __1: CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)>) -> CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)> {
    match (__0, __1) {
        (CBlockStmt(stmt), next) => {
            interpretStatement(stmt, next)
        },
        (CBlockDecl(decl), next) => {
            /* do */ {
                let decl_q = lift(lift((interpretDeclarations(makeLetBinding, decl))));

                let (rest, end) = next;

                (__op_addadd(decl_q, rest), end)
            }
        },
        (item, _) => {
            lift(lift((unimplemented(item))))
        },
    }
}

pub fn interpretConstExpr(__0: CExpr) -> EnvMonad<s, Integer> {
    match (__0) {
        CConst(CIntConst(CInteger(v, _, _), _)) => {
            v
        },
        expr => {
            unimplemented(expr)
        },
    }
}

pub fn interpretDeclarations(__0: MakeBinding<s, b>, __1: CDecl, __2: EnvMonad<s, Vec<b>>) -> EnvMonad<s, Vec<b>> {
    match (__0, __1, __2, __3) {
        ((fromItem, makeBinding), declaration, __OP__, CDecl(specs, decls, _)) => {
            /* do */ {
                let (storagespecs, baseTy) = baseTypeOf(specs);

                let mbinds = forM(decls)(|declarator| { /* do */ {
                            let (decl, minit) = match declarator {
                                    (Some(decl), minit, None) => {
                                        (decl, minit)
                                    },
                                    (None, _, _) => {
                                        badSource(declaration, "absent declarator".to_string())
                                    },
                                    (_, _, Some(_)) => {
                                        badSource(declaration, "bitfield declarator".to_string())
                                    },
                                };

                            let (ident, derived) = match decl {
                                    CDeclr(Some(ident), derived, _, _, _) => {
                                        (ident, derived)
                                    },
                                    _ => {
                                        badSource(decl, "abstract declarator".to_string())
                                    },
                                };

                            let deferred = derivedDeferredTypeOf(baseTy, decl, vec![]);

                            match (storagespecs, derived) {
                                (Some(CTypedef(_)), _) => {
                                    /* do */ {
                                        when((isJust(minit)), (badSource(decl, "initializer on typedef".to_string())));
                                        addTypedefIdent(ident, deferred);
                                        None
                                    }
                                },
                                (Some(CStatic(_)), [CFunDeclr {

                                                }, _]) => {
                                    /* do */ {
                                        addSymbolIdentAction(ident)(/* do */ {
                                            let itype = deferred;

                                            useForwardRef(ident);
                                            (typeToResult(itype, (Rust::Path((Rust::PathSegments(vec![applyRenames(ident)]))))))
                                        });
                                        None
                                    }
                                },
                                (_, [CFunDeclr {

                                                }, _]) => {
                                    /* do */ {
                                        addExternIdent(ident, deferred)(|name, (_mut, ty)| { match ty {
                                                IsFunc(retTy, args, variadic) => {
                                                    {
                                                        let formals = /* Expr::Generator */ Generator;

                                                    Rust::ExternFn(name, formals, variadic, (toRustRetType(retTy)))                                                    }
                                                },
                                                _ => {
                                                    __error!((__op_addadd(show(ident), " is both a function and not a function?".to_string())))
                                                },
                                            } });
                                        None
                                    }
                                },
                                (Some(CExtern(_)), _) => {
                                    /* do */ {
                                        addExternIdent(ident, deferred)(|name, (__mut, ty)| { Rust::ExternStatic(__mut, (Rust::VarName(name)), (toRustType(ty))) });
                                        None
                                    }
                                },
                                (Some(CStatic(_)), _) => {
                                    /* do */ {
                                        let _TODO_RECORD_ {
                                            typeMutable: __mut,
                                            typeRep: ty
                                        } = deferred;

                                        let name = addSymbolIdent(ident, (__mut, ty));

                                        let expr = interpretInitializer(ty, (fromMaybe((CInitList(vec![], (nodeInfo(decl)))), minit)));

                                        (Some((fromItem((Rust::Static(__mut, (Rust::VarName(name)), (toRustType(ty)), expr))))))
                                    }
                                },
                                _ => {
                                    /* do */ {
                                        let _TODO_RECORD_ {
                                            typeMutable: __mut,
                                            typeRep: ty
                                        } = deferred;

                                        let name = addSymbolIdent(ident, (__mut, ty));

                                        let binding = makeBinding(__mut, (Rust::VarName(name)), ty, (nodeInfo(decl)), minit);

                                        (Some(binding))
                                    }
                                },
                            }
                        } });

                (catMaybes(mbinds))
            }
        },
        (_, node, __OP__, CStaticAssert {

            }) => {
            unimplemented(node)
        },
    }
}

pub fn interpretExpr(__0: bool, __1: CExpr) -> EnvMonad<s, Result> {
    match (__0, __1) {
        (demand, CComma(exprs, _)) => {
            /* do */ {
                let (effects, mfinal) = __TODO_if(demand, then, (init(exprs), Some((last(exprs)))), __TODO_else, (exprs, None));

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (demand, expr, __OP__, CAssign(op, lhs, rhs, _)) => {
            /* do */ {
                let lhs_q = interpretExpr(true, lhs);

                let rhs_q = interpretExpr(true, rhs);

                compound(expr, false, demand, op, lhs_q, rhs_q)
            }
        },
        (demand, expr, __OP__, CCond(c, Some(t), f, _)) => {
            /* do */ {
                let c_q = fmap(toBool, (interpretExpr(true, c)));

                let t_q = interpretExpr(demand, t);

                let f_q = interpretExpr(demand, f);

                __TODO_if(demand, then, promotePtr, expr, (mkIf(c_q)), t_q, f_q, __TODO_else, return, Result {
                        resultType: IsVoid,
                        resultMutable: Rust::Immutable,
                        result: mkIf(c_q, (result(t_q)), (result(f_q)))
                    })
            }
        },
        (_, expr, __OP__, CBinary(op, lhs, rhs, _)) => {
            /* do */ {
                let lhs_q = interpretExpr(true, lhs);

                let rhs_q = interpretExpr(true, rhs);

                binop(expr, op, lhs_q, rhs_q)
            }
        },
        (_, CCast(decl, expr, _)) => {
            /* do */ {
                let (_mut, ty) = typeName(decl);

                let expr_q = interpretExpr((__op_assign_div(ty, IsVoid)), expr);

                Result {
                    resultType: ty,
                    resultMutable: Rust::Immutable,
                    result: ((__TODO_if(ty) == IsVoid(then, result, __TODO_else, castTo, ty)))(expr_q)
                }
            }
        },
        (demand, node, __OP__, CUnary(op, expr, _)) => {
            match op {
                CPreIncOp => {
                    incdec(false, CAddAssOp)
                },
                CPreDecOp => {
                    incdec(false, CSubAssOp)
                },
                CPostIncOp => {
                    incdec(true, CAddAssOp)
                },
                CPostDecOp => {
                    incdec(true, CSubAssOp)
                },
                CAdrOp => {
                    /* do */ {
                        let expr_q = interpretExpr(true, expr);

                        let ty_q = IsPtr((resultMutable(expr_q)), (resultType(expr_q)));

                        Result {
                            resultType: ty_q,
                            resultMutable: Rust::Immutable,
                            result: Rust::Cast((Rust::Borrow((resultMutable(expr_q)), (result(expr_q)))), (toRustType(ty_q)))
                        }
                    }
                },
                CIndOp => {
                    /* do */ {
                        let expr_q = interpretExpr(true, expr);

                        match resultType(expr_q) {
                            IsPtr(mut_q, ty_q) => {
                                Result {
                                    resultType: ty_q,
                                    resultMutable: mut_q,
                                    result: Rust::Deref((result(expr_q)))
                                }
                            },
                            IsFunc {

                            } => {
                                expr_q
                            },
                            _ => {
                                badSource(node, "dereference of non-pointer".to_string())
                            },
                        }
                    }
                },
                CPlusOp => {
                    /* do */ {
                        let expr_q = interpretExpr(demand, expr);

                        let ty_q = intPromote((resultType(expr_q)));

                        Result {
                            resultType: ty_q,
                            resultMutable: Rust::Immutable,
                            result: castTo(ty_q, expr_q)
                        }
                    }
                },
                CMinOp => {
                    fmap(wrapping)(simple(Rust::Neg))
                },
                CCompOp => {
                    simple(Rust::Not)
                },
                CNegOp => {
                    /* do */ {
                        let expr_q = interpretExpr(true, expr);

                        Result {
                            resultType: IsBool,
                            resultMutable: Rust::Immutable,
                            result: toNotBool(expr_q)
                        }
                    }
                },
            }
        },
        (_, CSizeofExpr(e, _)) => {
            /* do */ {
                let e_q = interpretExpr(true, e);

                (rustSizeOfType((toRustType((resultType(e_q))))))
            }
        },
        (_, CSizeofType(decl, _)) => {
            /* do */ {
                let (_mut, ty) = typeName(decl);

                (rustSizeOfType((toRustType(ty))))
            }
        },
        (_, CAlignofExpr(e, _)) => {
            /* do */ {
                let e_q = interpretExpr(true, e);

                (rustAlignOfType((toRustType((resultType(e_q))))))
            }
        },
        (_, CAlignofType(decl, _)) => {
            /* do */ {
                let (_mut, ty) = typeName(decl);

                (rustAlignOfType((toRustType(ty))))
            }
        },
        (_, expr, __OP__, CIndex(lhs, rhs, _)) => {
            /* do */ {
                let lhs_q = interpretExpr(true, lhs);

                let rhs_q = interpretExpr(true, rhs);

                match (resultType(lhs_q), resultType(rhs_q)) {
                    (IsArray(__mut, _, el), _) => {
                        (subscript(__mut, el, (result(lhs_q)), rhs_q))
                    },
                    (_, IsArray(__mut, _, el)) => {
                        (subscript(__mut, el, (result(rhs_q)), lhs_q))
                    },
                    _ => {
                        /* do */ {
                            let ptr = binop(expr, CAddOp, lhs_q, rhs_q);

                            match resultType(ptr) {
                                IsPtr(__mut, ty) => {
                                    Result {
                                        resultType: ty,
                                        resultMutable: __mut,
                                        result: Rust::Deref((result(ptr)))
                                    }
                                },
                                _ => {
                                    badSource(expr, "array subscript of non-pointer".to_string())
                                },
                            }
                        }
                    },
                }
            }
        },
        (_, expr, __OP__, CCall(func, args, _)) => {
            /* do */ {
                let func_q = interpretExpr(true, func);

                match resultType(func_q) {
                    IsFunc(retTy, argTys, variadic) => {
                        /* do */ {
                            let args_q = castArgs(variadic, (map(snd, argTys)), args);

                            Result {
                                resultType: retTy,
                                resultMutable: Rust::Immutable,
                                result: Rust::Call((result(func_q)), args_q)
                            }
                        }
                    },
                    _ => {
                        badSource(expr, "function call to non-function".to_string())
                    },
                }
            }
        },
        (_, expr, __OP__, CMember(obj, ident, deref, node)) => {
            /* do */ {
                let obj_q = interpretExpr(true)(__TODO_if(deref, then, CUnary, CIndOp, obj, node, __TODO_else, obj));

                let objTy = completeType((resultType(obj_q)));

                let fields = match objTy {
                        IsStruct(_, fields) => {
                            fields
                        },
                        _ => {
                            badSource(expr, "member access of non-struct".to_string())
                        },
                    };

                let field = applyRenames(ident);

                let ty = match lookup(field, fields) {
                        Some(ty) => {
                            ty
                        },
                        None => {
                            badSource(expr, "request for non-existent field".to_string())
                        },
                    };

                Result {
                    resultType: ty,
                    resultMutable: resultMutable(obj_q),
                    result: Rust::Member((result(obj_q)), (Rust::VarName(field)))
                }
            }
        },
        (_, expr, __OP__, CVar(ident, _)) => {
            /* do */ {
                let sym = getSymbolIdent(ident);

                maybe((badSource(expr, "undefined variable".to_string())), return, sym)
            }
        },
        (_, expr, __OP__, CConst(c)) => {
            match c {
                CIntConst(CInteger(v, repr, flags), _) => {
                    {
                        let allow_signed = not((testFlag(FlagUnsigned, flags)));

                        let allow_unsigned = (not(allow_signed) || __op_assign_div(repr, DecRepr));

                        let widths = vec![
                                (32, __TODO_if(any, (testFlag(flags)), vec![FlagLongLong, FlagLong], then, WordWidth, __TODO_else, BitWidth, 32)),
                                (64, BitWidth(64)),
                            ];

                        let allowed_types = /* Expr::Generator */ Generator;

                        let repr_q = match repr {
                                DecRepr => {
                                    Rust::DecRepr
                                },
                                OctalRepr => {
                                    Rust::OctalRepr
                                },
                                HexRepr => {
                                    Rust::HexRepr
                                },
                            };

                    match allowed_types {
                            [] => {
                                badSource(expr, "integer (too big)".to_string())
                            },
                            [ty, _] => {
                                (literalNumber(ty, (Rust::LitInt(v, repr_q))))
                            },
                        }                    }
                },
                CFloatConst(CFloat(__str), _) => {
                    match span((notElem("fF".to_string())), __str) {
                        (v, "") => {
                            (literalNumber((IsFloat(64)), (Rust::LitFloat(v))))
                        },
                        (v, [_]) => {
                            (literalNumber((IsFloat(32)), (Rust::LitFloat(v))))
                        },
                        _ => {
                            badSource(expr, "float".to_string())
                        },
                    }
                },
                CCharConst(CChar(ch, false), _) => {
                    Result {
                        resultType: charType,
                        resultMutable: Rust::Immutable,
                        result: Rust::Lit((Rust::LitByteChar(ch)))
                    }
                },
                CStrConst(CString(__str, false), _) => {
                    Result {
                        resultType: IsArray(Rust::Immutable, ((length(__str) + 1)), charType),
                        resultMutable: Rust::Immutable,
                        result: Rust::Deref((Rust::Lit((Rust::LitByteStr((__op_addadd(__str, "\u{0}".to_string())))))))
                    }
                },
                _ => {
                    unimplemented(expr)
                },
            }
        },
        (_, CCompoundLit(decl, initials, info)) => {
            /* do */ {
                let (__mut, ty) = typeName(decl);

                let __final = interpretInitializer(ty, (CInitList(initials, info)));

                Result {
                    resultType: ty,
                    resultMutable: __mut,
                    result: __final
                }
            }
        },
        (demand, stat, __OP__, CStatExpr(CCompound([], stmts, _), _)) => {
            scope(/* do */ {
                let (effects, __final) = match last(stmts) {
                        CBlockStmt(CExpr(expr, _)) if demand => { (init(stmts), expr) }
                        _ => {
                            (stmts, None)
                        },
                    };

                let effects_q = cfgToRust(stat, (foldr(interpretBlockItem, ((vec![], Unreachable)), effects)));

                let final_q = mapM((interpretExpr(true)), __final);

                Result {
                    resultType: maybe(IsVoid, resultType, final_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, final_q),
                    result: Rust::BlockExpr((Rust::Block(effects_q, (fmap(result, final_q)))))
                }
            })
        },
        (_, expr) => {
            unimplemented(expr)
        },
    }
}

pub fn interpretFunction(CFunDef(specs, declr, __OP__, CDeclr(mident, _, _, _, _), argtypes, body, _): CFunDef) -> EnvMonad<s, ()> {
    /* do */ {
        let (storage, baseTy) = baseTypeOf(specs);

        let (attrs, vis) = match storage {
                None => {
                    (vec![Rust::Attribute("no_mangle".to_string())], Rust::Public)
                },
                Some(CStatic(_)) => {
                    (vec![], Rust::Private)
                },
                Some(s) => {
                    badSource(s, "storage class specifier for function".to_string())
                },
            };

        let go = |name, funTy| {
            /* do */ {
                let (retTy, args) = match funTy {
                        IsFunc(_, _, true) => {
                            unimplemented(declr)
                        },
                        IsFunc(retTy, args, false) => {
                            (retTy, args)
                        },
                        _ => {
                            badSource(declr, "function definition".to_string())
                        },
                    };

                when(((name == "_c_main".to_string())), (wrapMain(declr, name, (map(snd, args)))));
                let setRetTy = |flow| {
                    flow {
                        functionReturnType: Some(retTy),
                        functionName: Some(name)
                    }
                };

                let f_q = mapExceptT((local(setRetTy)))(scope(/* do */ {
                        let formals = sequence(/* Expr::Generator */ Generator);

                        let returnValue = (__TODO_if(name) == "_c_main".to_string()(then, Some, 0, __TODO_else, None));

                        let returnStatement = Rust::Stmt((Rust::Return(returnValue)));

                        let body_q = cfgToRust(declr, (interpretStatement(body, ((vec![returnStatement], Unreachable)))));

                        (Rust::Item(attrs, vis, (Rust::Function(vec![Rust::UnsafeFn, Rust::ExternABI(None)], name, formals, (toRustRetType(retTy)), (statementsToBlock(body_q))))))
                    }));

                emitItems(vec![f_q])
            }
        };

        let ident = match mident {
                None => {
                    badSource(declr, "anonymous function definition".to_string())
                },
                Some(ident) => {
                    ident
                },
            };

        let name = applyRenames(ident);

        let funTy = |itype| {
            typeToResult(itype, (Rust::Path((Rust::PathSegments(vec![name])))))
        };

        let deferred = fmap((fmap(funTy)), (derivedDeferredTypeOf(baseTy, declr, argtypes)));

        let alreadyUsed = lift(gets((usedForwardRefs(globalState))));

        match vis {
            Rust::Private if Set::notMember(ident, alreadyUsed) => { /* do */ {
                let action = runOnce(/* do */ {
                        let ty = deferred;

                        go(name, (resultType(ty)));
                        ty
                    });

                addSymbolIdentAction(ident, action)
            } }
            _ => {
                /* do */ {
                    let ty = deferred;

                    addSymbolIdentAction(ident)(ty);
                    go(name, (resultType(ty)))
                }
            },
        }
    }
}

pub fn interpretInitializer(ty: CType, initial: CInit) -> EnvMonad<s, Rust::Expr> {
    /* do */ {
        let initial_q = match initial {
                CInitExpr(expr, _) => {
                    /* do */ {
                        let expr_q = interpretExpr(true, expr);

                        compatibleInitializer(__TODO_if(resultType, expr_q), ty(then, __pure)(scalar((castTo(ty, expr_q)), __TODO_else, badSource, initial, "initializer for incompatible type".to_string())))
                    }
                },
                CInitList(list, _) => {
                    translateInitList(ty, list)
                },
            };

        let zeroed = zeroInitialize(initial_q, ty);

        helper(ty, zeroed)
    }
}

pub fn interpretStatement(__0: CStat, __1: CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)>) -> CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)> {
    match (__0, __1) {
        (CLabel(ident, body, _, _), next) => {
            /* do */ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (stmt, __OP__, CCase(expr, body, node), next) => {
            /* do */ {
                let selector = getSwitchExpression(stmt);

                let condition = CBinary(CEqOp, selector, expr, node);

                addSwitchCase((Some(condition)), body, next)
            }
        },
        (stmt, __OP__, CCases(lower, upper, body, node), next) => {
            /* do */ {
                let selector = getSwitchExpression(stmt);

                let condition = CBinary(CLndOp, (CBinary(CGeqOp, selector, lower, node)), (CBinary(CLeqOp, selector, upper, node)), node);

                addSwitchCase((Some(condition)), body, next)
            }
        },
        (CDefault(body, _), next) => {
            addSwitchCase(None, body, next)
        },
        (CExpr(None, _), next) => {
            next
        },
        (CExpr(Some(expr), _), next) => {
            /* do */ {
                let expr_q = lift(lift(interpretExpr(false, expr)));

                let (rest, end) = next;

                (__op_addadd(resultToStatements(expr_q), rest), end)
            }
        },
        (CCompound([], items, _), next) => {
            mapBuildCFGT((mapRWST(scope)))(/* do */ {
                foldr(interpretBlockItem, next, items)
            })
        },
        (CIf(c, t, mf, _), next) => {
            /* do */ {
                let c_q = lift(lift(interpretExpr(true, c)));

                let after = newLabel;

                let falseLabel = match mf {
                        None => {
                            after
                        },
                        Some(f) => {
                            /* do */ {
                                let (falseEntry, falseTerm) = interpretStatement(f, ((vec![], Branch(after))));

                                let falseLabel = newLabel;

                                addBlock(falseLabel, falseEntry, falseTerm);
                                falseLabel
                            }
                        },
                    };

                let (trueEntry, trueTerm) = interpretStatement(t, ((vec![], Branch(after))));

                let trueLabel = newLabel;

                addBlock(trueLabel, trueEntry, trueTerm);
                let (rest, end) = next;

                addBlock(after, rest, end);
                (vec![], CondBranch(c_q, trueLabel, falseLabel))
            }
        },
        (stmt, __OP__, CSwitch(expr, body, node), next) => {
            /* do */ {
                let (bindings, expr_q) = match expr {
                        CVar {

                        } => {
                            (vec![], expr)
                        },
                        _ => {
                            lift(lift(/* do */ {
                                let ident = fmap(internalIdent, (uniqueName("switch".to_string())));

                                let rhs = interpretExpr(true, expr);

                                let var = addSymbolIdent(ident, (Rust::Immutable, resultType(rhs)));

                                (vec![
                                    Rust::Let(Rust::Immutable, (Rust::VarName(var)), None, (Some((result(rhs))))),
                                ], CVar(ident, node))
                            }))
                        },
                    };

                let after = newLabel;

                let (_, SwitchCases(cases)) = getSwitchCases(expr_q)(setBreak(after)(interpretStatement(body, ((vec![], Branch(after))))));

                let isDefault = |Some(condition)| {
                    Left(condition)
                };

                let isDefault = |None| {
                    Right(())
                };

                let (conditions, defaults) = IntMap::mapEither(isDefault, cases);

                let defaultCase = match IntMap::keys(defaults) {
                        [] => {
                            after
                        },
                        [defaultCase] => {
                            defaultCase
                        },
                        _ => {
                            lift(lift(badSource(stmt, "duplicate default cases".to_string())))
                        },
                    };

                let entry = foldrM(conditionBlock, defaultCase, (IntMap::toList(conditions)));

                let (rest, end) = next;

                addBlock(after, rest, end);
                (bindings, Branch(entry))
            }
        },
        (CWhile(c, body, doWhile, _), next) => {
            /* do */ {
                let c_q = lift(lift(interpretExpr(true, c)));

                let after = newLabel;

                let headerLabel = newLabel;

                let (bodyEntry, bodyTerm) = setBreak(after)(setContinue(headerLabel)(interpretStatement(body, ((vec![], Branch(headerLabel))))));

                let bodyLabel = newLabel;

                addBlock(bodyLabel, bodyEntry, bodyTerm);
                addBlock(headerLabel, vec![])(match toBool(c_q) {
                    Rust::Lit(Rust::LitBool(cont)) if __op_assign_div(cont, doWhile) => { Branch((__TODO_if(cont, then, bodyLabel, __TODO_else, after))) }
                    _ => {
                        CondBranch(c_q, bodyLabel, after)
                    },
                });
                let (rest, end) = next;

                addBlock(after, rest, end);
                (vec![], Branch((__TODO_if(doWhile, then, bodyLabel, __TODO_else, headerLabel))))
            }
        },
        (CFor(initial, mcond, mincr, body, _), next) => {
            /* do */ {
                let after = newLabel;

                let ret = mapBuildCFGT((mapRWST(scope)))(/* do */ {
                        let prefix = match initial {
                                Left(None) => {
                                    vec![]
                                },
                                Left(Some(expr)) => {
                                    /* do */ {
                                        let expr_q = lift(lift(interpretExpr(false, expr)));

                                        (resultToStatements(expr_q))
                                    }
                                },
                                Right(decls) => {
                                    lift(lift(interpretDeclarations(makeLetBinding, decls)))
                                },
                            };

                        let headerLabel = newLabel;

                        let incrLabel = match mincr {
                                None => {
                                    headerLabel
                                },
                                Some(incr) => {
                                    /* do */ {
                                        let incr_q = lift(lift(interpretExpr(false, incr)));

                                        let incrLabel = newLabel;

                                        addBlock(incrLabel, (resultToStatements(incr_q)), (Branch(headerLabel)));
                                        incrLabel
                                    }
                                },
                            };

                        let (bodyEntry, bodyTerm) = setBreak(after)(setContinue(incrLabel)(interpretStatement(body, ((vec![], Branch(incrLabel))))));

                        let bodyLabel = newLabel;

                        addBlock(bodyLabel, bodyEntry, bodyTerm);
                        let cond = match mcond {
                                Some(cond) => {
                                    /* do */ {
                                        let cond_q = lift(lift(interpretExpr(true, cond)));

                                        (CondBranch(cond_q, bodyLabel, after))
                                    }
                                },
                                None => {
                                    (Branch(bodyLabel))
                                },
                            };

                        addBlock(headerLabel, vec![], cond);
                        (prefix, Branch(headerLabel))
                    });

                let (rest, end) = next;

                addBlock(after, rest, end);
                ret
            }
        },
        (CGoto(ident, _), next) => {
            /* do */ {
                let _ = next;

                let label = gotoLabel(ident);

                (vec![], Branch(label))
            }
        },
        (stmt, __OP__, CCont(_), next) => {
            /* do */ {
                let _ = next;

                let val = lift((asks(onContinue)));

                match val {
                    Some(label) => {
                        (vec![], Branch(label))
                    },
                    None => {
                        lift(lift(badSource(stmt, "continue outside loop".to_string())))
                    },
                }
            }
        },
        (stmt, __OP__, CBreak(_), next) => {
            /* do */ {
                let _ = next;

                let val = lift((asks(onBreak)));

                match val {
                    Some(label) => {
                        (vec![], Branch(label))
                    },
                    None => {
                        lift(lift(badSource(stmt, "break outside loop".to_string())))
                    },
                }
            }
        },
        (stmt, __OP__, CReturn(expr, _), next) => {
            /* do */ {
                let _ = next;

                lift(lift(/* do */ {
                    let val = lift((asks(functionReturnType)));

                    match val {
                        None => {
                            badSource(stmt, "return statement outside function".to_string())
                        },
                        Some(retTy) => {
                            /* do */ {
                                let expr_q = mapM((fmap((castTo(retTy)), interpretExpr(true))), expr);

                                (exprToStatements((Rust::Return(expr_q))), Unreachable)
                            }
                        },
                    }
                }))
            }
        },
        (stmt, _) => {
            lift(lift(unimplemented(stmt)))
        },
    }
}

pub fn interpretTranslationUnit(_thisModule: ModuleMap, rewrites: ItemRewrites, CTranslUnit(decls, _): CTranslUnit) -> Either<String, Vec<Rust::Item>> {
    match err {
        Left(msg) => {
            Left(msg)
        },
        Right(_) => {
            Right(items_q)
        },
    }
}

pub fn makeLetBinding() -> MakeBinding<s, Rust::Stmt> {
    (Rust::StmtItem(vec![]), makeBinding)
}

pub fn makeStaticBinding() -> MakeBinding<s, Rust::Item> {
    (Rust::Item(vec![], Rust::Private), makeBinding)
}

pub fn modifyGlobal(f: fn(GlobalState) -> (GlobalState, a)) -> EnvMonad<s, a> {
    lift(/* do */ {
        let st = get;

        let (global_q, a) = f((globalState(st)));

        put(st {
                globalState: global_q
            });
        a
    })
}

pub fn mutable(quals: Vec<CTypeQualifier<a>>) -> Rust::Mutable {
    __TODO_if(any, (|q| { match q {
                CConstQual(_) => {
                    true
                },
                _ => {
                    false
                },
            } }), quals, then, Rust::Immutable, __TODO_else, Rust::Mutable)
}

pub fn nestedObject(ty: CType, desig: Designator) -> Option<Designator> {
    match designatorType(desig) {
        IsArray(_, size, el) => {
            Some((From(el, 0, (replicate(((size - 1)), el)), desig)))
        },
        ty_q if compatibleInitializer(ty, ty_q) => { Some(desig) }
        IsStruct(_, [(_, ty_q), fields]) => {
            nestedObject(ty, (From(ty_q, 0, (map(snd, fields)), desig)))
        },
        _ => {
            None
        },
    }
}

pub fn nextObject(__0: Designator) -> CurrentObject {
    match (__0) {
        Base {

        } => {
            None
        },
        From(_, i, [ty, remaining], base) => {
            Some((From(ty, ((i + 1)), remaining, base)))
        },
        From(_, _, [], base) => {
            nextObject(base)
        },
    }
}

pub fn noTranslation(node: node, msg: String) -> EnvMonad<s, a> {
    throwE(concat(vec![
            show((posOf(node))),
            ": ".to_string(),
            msg,
            ":\n".to_string(),
            render((nest(4, (pretty(node))))),
        ]))
}

pub fn objectFromDesignators(__0: CType, __1: Vec<CDesignator>) -> EnvMonad<s, CurrentObject> {
    match (__0, __1) {
        (_, []) => {
            __pure(None)
        },
        (ty, desigs) => {
            __op_dollar_arrow(Some, go(ty, desigs, (Base(ty))))
        },
    }
}

pub fn promote(node: node, op: fn(Rust::Expr) -> fn(Rust::Expr) -> Rust::Expr, a: Result, b: Result) -> EnvMonad<s, Result> {
    match usual((resultType(a)), (resultType(b))) {
        Some(rt) => {
            Result {
                resultType: rt,
                resultMutable: Rust::Immutable,
                result: op((castTo(rt, a)), (castTo(rt, b)))
            }
        },
        None => {
            badSource(node)(concat(vec![
                    "arithmetic combination for ".to_string(),
                    show((resultType(a))),
                    " and ".to_string(),
                    show((resultType(b))),
                ]))
        },
    }
}

pub fn promotePtr(node: node, op: fn(Rust::Expr) -> fn(Rust::Expr) -> Rust::Expr, a: Result, b: Result) -> EnvMonad<s, Result> {
    match (resultType(a), resultType(b)) {
        (IsArray(_, _, _), _) => {
            ptrs
        },
        (IsPtr(_, _), _) => {
            ptrs
        },
        (_, IsArray(_, _, _)) => {
            ptrs
        },
        (_, IsPtr(_, _)) => {
            ptrs
        },
        _ => {
            promote(node, op, a, b)
        },
    }
}

pub fn resolveCurrentObject((obj0, prior): (CurrentObject, Initializer), (obj1, cinitial): (CurrentObject, CInit)) -> EnvMonad<s, (CurrentObject, Initializer)> {
    match mplus(obj1, obj0) {
        None => {
            (None, prior)
        },
        Some(obj) => {
            /* do */ {
                let (obj_q, initial) = match cinitial {
                        CInitList(list_q, _) => {
                            /* do */ {
                                let initial = translateInitList((designatorType(obj)), list_q);

                                (obj, initial)
                            }
                        },
                        CInitExpr(expr, _) => {
                            /* do */ {
                                let expr_q = interpretExpr(true, expr);

                                match nestedObject((resultType(expr_q)), obj) {
                                    None => {
                                        badSource(cinitial, "type in initializer".to_string())
                                    },
                                    Some(obj_q) => {
                                        /* do */ {
                                            let s = castTo((designatorType(obj_q)), expr_q);

                                            (obj_q, scalar(s))
                                        }
                                    },
                                }
                            }
                        },
                    };

                let indices = unfoldr((|o| { match o {
                                Base {

                                } => {
                                    None
                                },
                                From(_, j, _, p) => {
                                    Some((j, p))
                                },
                            } }), obj_q);

                let initializer = foldl((|a, j| { Initializer(None, (IntMap::singleton(j, a))) }), initial, indices);

                (nextObject(obj_q), mappend(prior, initializer))
            }
        },
    }
}

pub fn resultToStatements() -> Vec<Rust::Stmt> {
    exprToStatements(result)
}

pub fn runOnce(action: EnvMonad<s, a>) -> EnvMonad<s, EnvMonad<s, a>> {
    /* do */ {
        let cacheRef = lift(lift(newSTRef((Left(action)))));

        return(/* do */ {
            let cache = lift(lift(readSTRef(cacheRef)));

            match cache {
                Left(todo) => {
                    /* do */ {
                        lift(lift(writeSTRef(cacheRef)(Left(fail("internal error: runOnce action depends on itself, leading to an infinite loop".to_string())))));
                        let val = todo;

                        lift(lift(writeSTRef(cacheRef, (Right(val)))));
                        val
                    }
                },
                Right(val) => {
                    val
                },
            }
        })
    }
}

pub fn rustAlignOfType(Rust::TypeName(ty): Rust::Type) -> Result {
    Result {
        resultType: IsInt(Unsigned, WordWidth),
        resultMutable: Rust::Immutable,
        result: Rust::Call((Rust::Var((Rust::VarName((__op_addadd("::std::mem::align_of::<".to_string(), __op_addadd(ty, ">".to_string()))))))), vec![])
    }
}

pub fn rustSizeOfType(Rust::TypeName(ty): Rust::Type) -> Result {
    Result {
        resultType: IsInt(Unsigned, WordWidth),
        resultMutable: Rust::Immutable,
        result: Rust::Call((Rust::Var((Rust::VarName((__op_addadd("::std::mem::size_of::<".to_string(), __op_addadd(ty, ">".to_string()))))))), vec![])
    }
}

pub fn scalar(expr: Rust::Expr) -> Initializer {
    Initializer((Some(expr)), IntMap::empty)
}

pub fn scope(m: EnvMonad<s, a>) -> EnvMonad<s, a> {
    /* do */ {
        let old = lift(get);

        let a = m;

        lift((modify((|st| { old {
                        globalState: globalState(st)
                    } }))));
        a
    }
}

pub fn setBreak(label: Label) -> CSourceBuildCFGT<s, a> {
    mapBuildCFGT((local((|flow| { flow {
                    onBreak: Some(label)
                } }))))
}

pub fn setContinue(label: Label) -> CSourceBuildCFGT<s, a> {
    mapBuildCFGT((local((|flow| { flow {
                    onContinue: Some(label)
                } }))))
}

pub fn statementsToBlock(__0: Vec<Rust::Stmt>) -> Rust::Block {
    match (__0) {
        [Rust::Stmt(Rust::BlockExpr(stmts))] => {
            stmts
        },
        stmts => {
            Rust::Block(stmts, None)
        },
    }
}

pub fn toBool(__0: Result) -> Rust::Expr {
    match (__0) {
        Result {

            } => {
            Rust::Lit((Rust::LitBool(false)))
        },
        Result {

            } => {
            Rust::Lit((Rust::LitBool(true)))
        },
        Result {

            } => {
            match t {
                IsBool => {
                    v
                },
                IsPtr(_, _) => {
                    Rust::Not((Rust::MethodCall(v, (Rust::VarName("is_null".to_string())), vec![])))
                },
                _ => {
                    Rust::CmpNE(v, 0)
                },
            }
        },
    }
}

pub fn toNotBool(__0: Result) -> Rust::Expr {
    match (__0) {
        Result {

            } => {
            Rust::Lit((Rust::LitBool(true)))
        },
        Result {

            } => {
            Rust::Lit((Rust::LitBool(false)))
        },
        Result {

            } => {
            match t {
                IsBool => {
                    Rust::Not(v)
                },
                IsPtr(_, _) => {
                    Rust::MethodCall(v, (Rust::VarName("is_null".to_string())), vec![])
                },
                _ => {
                    Rust::CmpEQ(v, 0)
                },
            }
        },
    }
}

pub fn toPtr(__0: Result, __1: Option<Result>) -> Option<Result> {
    match (__0, __1, __2) {
        (ptr, __OP__, Result {

            }) => {
            Some(ptr {
                    resultType: IsPtr(__mut, el),
                    result: castTo((IsPtr(__mut, el)), ptr)
                })
        },
        (ptr, __OP__, Result {

            }) => {
            Some(ptr)
        },
        _ => {
            None
        },
    }
}

pub fn toRustRetType(__0: CType) -> Rust::Type {
    match (__0) {
        IsVoid => {
            Rust::TypeName("()".to_string())
        },
        ty => {
            toRustType(ty)
        },
    }
}

pub fn toRustType(__0: CType) -> Rust::Type {
    match (__0) {
        IsBool => {
            Rust::TypeName("bool".to_string())
        },
        IsInt(s, w) => {
            Rust::TypeName((__op_concat((match s {
                    Signed => {
                        'i'
                    },
                    Unsigned => {
                        'u'
                    },
                }), (match w {
                    BitWidth(b) => {
                        show(b)
                    },
                    WordWidth => {
                        "size".to_string()
                    },
                }))))
        },
        IsFloat(w) => {
            Rust::TypeName((__op_concat('f', show(w))))
        },
        IsVoid => {
            Rust::TypeName("::std::os::raw::c_void".to_string())
        },
        IsFunc(retTy, args, variadic) => {
            Rust::TypeName(concat(vec![
                    "unsafe extern fn(".to_string(),
                    args_q,
                    ")".to_string(),
                    __op_assign_div(__TODO_if(retTy), __op_addadd(IsVoid(then, " -> ".to_string()), typename(retTy, __TODO_else, "".to_string()))),
                ]))
        },
        IsPtr(__mut, to) => {
            {
                let Rust::TypeName = |to_q| {
                    toRustType(to)
                };

            Rust::TypeName((__op_addadd(rustMut(__mut), to_q)))            }
        },
        IsArray(_, size, el) => {
            Rust::TypeName((__op_addadd("[".to_string(), __op_addadd(typename(el), __op_addadd("; ".to_string(), __op_addadd(show(size), "]".to_string()))))))
        },
        IsStruct(name, _fields) => {
            Rust::TypeName(name)
        },
        IsEnum(name) => {
            Rust::TypeName(name)
        },
        IsIncomplete(ident) => {
            Rust::TypeName((identToString(ident)))
        },
    }
}

pub fn translateInitList(ty: CType, list: CInitList) -> EnvMonad<s, Initializer> {
    /* do */ {
        let objectsAndInitializers = forM(list)(|(desigs, initial)| { /* do */ {
                    let currObj = objectFromDesignators(ty, desigs);

                    __pure((currObj, initial))
                } });

        let base = match ty {
                IsArray(_, size, el) => {
                    From(el, 0, (replicate(((size - 1)), el)), (Base(ty)))
                },
                IsStruct(_, [(_, ty_q), fields]) => {
                    From(ty_q, 0, (map(snd, fields)), (Base(ty)))
                },
                _ => {
                    Base(ty)
                },
            };

        let (_, initializer) = foldM(resolveCurrentObject, (Some(base), mempty), objectsAndInitializers);

        initializer
    }
}

pub fn typeName(__0: CDecl, __1: EnvMonad<s, (Rust::Mutable, CType)>) -> EnvMonad<s, (Rust::Mutable, CType)> {
    match (__0, __1, __2) {
        (decl, __OP__, CStaticAssert {

            }) => {
            badSource(decl, "static assert in type name ".to_string())
        },
        (decl, __OP__, CDecl(spec, declarators, _)) => {
            /* do */ {
                let (storage, base) = baseTypeOf(spec);

                match storage {
                    Some(s) => {
                        badSource(s, "storage class specifier in type name".to_string())
                    },
                    None => {
                        ()
                    },
                };
                let itype = match declarators {
                        [] => {
                            base
                        },
                        [(Some(declr, __OP__, CDeclr(None, _, _, _, _)), None, None)] => {
                            derivedTypeOf(base, declr)
                        },
                        _ => {
                            badSource(decl, "type name".to_string())
                        },
                    };

                when((typeIsFunc(itype)), (badSource(decl, "use of function type".to_string())));
                (typeMutable(itype), typeRep(itype))
            }
        },
    }
}

pub fn typeToResult(itype: IntermediateType, expr: Rust::Expr) -> Result {
    Result {
        resultType: typeRep(itype),
        resultMutable: typeMutable(itype),
        result: expr
    }
}

pub fn unimplemented(node: node) -> EnvMonad<s, a> {
    noTranslation(node, "Corrode doesn\'t handle this yet".to_string())
}

pub fn uniqueName(base: String) -> EnvMonad<s, String> {
    modifyGlobal(|st| { (st {
            unique: (unique(st) + 1)
        }, __op_addadd(base, show((unique(st))))) })
}

pub fn useForwardRef(ident: Ident) -> EnvMonad<s, ()> {
    modifyGlobal(|st| { (st {
            usedForwardRefs: Set::insert(ident, (usedForwardRefs(st)))
        }, ()) })
}

pub fn usual(__0: CType, __1: CType) -> Option<CType> {
    match (__0, __1) {
        (IsFloat(aw), IsFloat(bw)) => {
            Some((IsFloat((max(aw, bw)))))
        },
        (a, __OP__, IsFloat(_), _) => {
            Some(a)
        },
        (_, b, __OP__, IsFloat(_)) => {
            Some(b)
        },
        (origA, origB) => {
            match (intPromote(origA), intPromote(origB)) {
                (a, b) if (a == b) => { Some(a) }
                (IsInt(Signed, sw), IsInt(Unsigned, uw)) => {
                    mixedSign(sw, uw)
                },
                (IsInt(Unsigned, uw), IsInt(Signed, sw)) => {
                    mixedSign(sw, uw)
                },
                (IsInt(__as, aw), IsInt(_bs, bw)) => {
                    /* do */ {
                        let rank = integerConversionRank(aw, bw);

                        Some((IsInt(__as, ((__TODO_if(rank) == GT(then, aw, __TODO_else, bw))))))
                    }
                },
                _ => {
                    None
                },
            }
        },
    }
}

pub fn wrapMain(declr: CDeclr, realName: String, argTypes: Vec<CType>) -> EnvMonad<s, ()> {
    /* do */ {
        let (setup, args) = wrapArgv(argTypes);

        let ret = Rust::VarName("ret".to_string());

        emitItems(vec![
                Rust::Item(vec![], Rust::Private, (Rust::Function(vec![], "main".to_string(), vec![], (Rust::TypeName("()".to_string())), (statementsToBlock((__op_addadd(setup, __op_addadd(vec![
                            bind(Rust::Immutable, ret)(Rust::UnsafeExpr(Rust::Block(vec![])(Some(call(realName, args))))),
                        ], exprToStatements((call("::std::process::exit".to_string(), vec![Rust::Var(ret)]))))))))))),
            ])
    }
}

pub fn wrapping(__0: Result, __1: Result) -> Result {
    match (__0, __1, __2) {
        (r, __OP__, Result {

            }) => {
            match result(r) {
                Rust::Add(lhs, rhs) => {
                    r {
                        result: Rust::MethodCall(lhs, (Rust::VarName("wrapping_add".to_string())), vec![rhs])
                    }
                },
                Rust::Sub(lhs, rhs) => {
                    r {
                        result: Rust::MethodCall(lhs, (Rust::VarName("wrapping_sub".to_string())), vec![rhs])
                    }
                },
                Rust::Mul(lhs, rhs) => {
                    r {
                        result: Rust::MethodCall(lhs, (Rust::VarName("wrapping_mul".to_string())), vec![rhs])
                    }
                },
                Rust::Div(lhs, rhs) => {
                    r {
                        result: Rust::MethodCall(lhs, (Rust::VarName("wrapping_div".to_string())), vec![rhs])
                    }
                },
                Rust::Mod(lhs, rhs) => {
                    r {
                        result: Rust::MethodCall(lhs, (Rust::VarName("wrapping_rem".to_string())), vec![rhs])
                    }
                },
                Rust::Neg(e) => {
                    r {
                        result: Rust::MethodCall(e, (Rust::VarName("wrapping_neg".to_string())), vec![])
                    }
                },
                _ => {
                    r
                },
            }
        },
        r => {
            r
        },
    }
}



