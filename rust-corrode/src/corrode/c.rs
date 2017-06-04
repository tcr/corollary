// Original file: "C.lhs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Control::Monad;
// use Control::Monad::ST;
// use Control::Monad::Trans::Class;
// use Control::Monad::Trans::Except;
// use Control::Monad::Trans::RWS::Strict;
// use Data::Foldable;
// use Data::Map::Lazy;
// use Data::IntMap::Strict;
// use Data::Maybe;
// use Data::List;
// use Data::STRef;
// use Data::Set;
// use Language::C;
// use Language::C::Data::Ident;
// use Language::Rust::AST;
// use Language::Rust::Corrode::CFG;
// use Language::Rust::Corrode::CrateMap;
// use Text::PrettyPrint::HughesPJClass;

pub type EnvMonad<s> = ExceptT<String, RWST<FunctionContext, Output, EnvState<s>, ST<s>>>;

pub struct FunctionContext{
    functionReturnType: Option<CType>,
    functionName: Option<String>,
    itemRewrites: ItemRewrites
}
fn functionReturnType(a: FunctionContext) -> Option<CType> { a.functionReturnType }
fn functionName(a: FunctionContext) -> Option<String> { a.functionName }
fn itemRewrites(a: FunctionContext) -> ItemRewrites { a.itemRewrites }

pub struct Output{
    outputItems: Vec<Rust::Item>,
    outputExterns: Map::Map<String, Rust::ExternItem>,
    outputIncomplete: Set::Set<String>
}
fn outputItems(a: Output) -> Vec<Rust::Item> { a.outputItems }
fn outputExterns(a: Output) -> Map::Map<String, Rust::ExternItem> { a.outputExterns }
fn outputIncomplete(a: Output) -> Set::Set<String> { a.outputIncomplete }

pub struct GlobalState{
    unique: isize,
    usedForwardRefs: Set::Set<Ident>
}
fn unique(a: GlobalState) -> isize { a.unique }
fn usedForwardRefs(a: GlobalState) -> Set::Set<Ident> { a.usedForwardRefs }

pub struct EnvState<s>{
    symbolEnvironment: Vec<(Ident, EnvMonad<s, Result>)>,
    typedefEnvironment: Vec<(Ident, EnvMonad<s, IntermediateType>)>,
    tagEnvironment: Vec<(Ident, EnvMonad<s, CType>)>,
    globalState: GlobalState
}
fn symbolEnvironment(a: EnvState) -> Vec<(Ident, EnvMonad<s, Result>)> { a.symbolEnvironment }
fn typedefEnvironment(a: EnvState) -> Vec<(Ident, EnvMonad<s, IntermediateType>)> { a.typedefEnvironment }
fn tagEnvironment(a: EnvState) -> Vec<(Ident, EnvMonad<s, CType>)> { a.tagEnvironment }
fn globalState(a: EnvState) -> GlobalState { a.globalState }

pub type MakeBinding<s, a> = (fn(Rust::ItemKind) -> a, fn(Rust::Mutable) -> fn(Rust::Var) -> fn(CType) -> fn(NodeInfo) -> fn(Option<CInit>) -> EnvMonad<s, a>);

pub struct Initializer(Option<Rust::Expr>, IntMap::IntMap<Initializer>);


pub type CurrentObject = Option<Designator>;

#[derive(Debug)]
pub enum Designator {
    Base(CType),
    From(CType, isize, Vec<CType>, Designator)
}
pub use self::Designator::*;

pub struct OuterLabels{
    onBreak: Option<Label>,
    onContinue: Option<Label>,
    switchExpression: Option<CExpr>
}
fn onBreak(a: OuterLabels) -> Option<Label> { a.onBreak }
fn onContinue(a: OuterLabels) -> Option<Label> { a.onContinue }
fn switchExpression(a: OuterLabels) -> Option<CExpr> { a.switchExpression }

pub struct SwitchCases(IntMap::IntMap<Option<Result>>);


pub type CSourceBuildCFGT<s> = BuildCFGT<RWST<OuterLabels, SwitchCases, Map::Map<Ident, Label>, EnvMonad<s>>, Vec<Rust::Stmt>, Result>;

pub struct Result{
    resultType: CType,
    resultMutable: Rust::Mutable,
    result: Rust::Expr
}
fn resultType(a: Result) -> CType { a.resultType }
fn resultMutable(a: Result) -> Rust::Mutable { a.resultMutable }
fn result(a: Result) -> Rust::Expr { a.result }

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

pub struct IntermediateType{
    typeMutable: Rust::Mutable,
    typeIsFunc: bool,
    typeRep: CType
}
fn typeMutable(a: IntermediateType) -> Rust::Mutable { a.typeMutable }
fn typeIsFunc(a: IntermediateType) -> bool { a.typeIsFunc }
fn typeRep(a: IntermediateType) -> CType { a.typeRep }

pub fn addExternIdent(ident: Ident, deferred: EnvMonad<s, IntermediateType>, mkItem: fn(String) -> fn((Rust::Mutable, CType)) -> Rust::ExternItem) -> EnvMonad<s, ()> {
    /*do*/ {
        let action = runOnce(/*do*/ {
                let itype = deferred;

                let rewrites = lift(asks(itemRewrites));

                let path = match Map::lookup((Symbol, identToString(ident)), rewrites) {
                        Some(renamed) => {
                            (__op_concat("".to_string(), renamed))
                        },
                        None => {
                            /*do*/ {
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
    /*do*/ {
        let condition_q = lift(lift(mapM((interpretExpr(true)), condition)));

        let next_q = interpretStatement(body, next);

        let label = match next_q {
                ([], Branch(to)) => {
                    to
                },
                (rest, end) => {
                    /*do*/ {
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
    /*do*/ {
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
    lift(/*do*/ {
        modify(|st| { st {
                symbolEnvironment: __op_concat((ident, action), symbolEnvironment(st))
            } })
    })
}

pub fn addTagIdent(ident: Ident, ty: EnvMonad<s, CType>) -> EnvMonad<s, ()> {
    lift(/*do*/ {
        modify(|st| { st {
                tagEnvironment: __op_concat((ident, ty), tagEnvironment(st))
            } })
    })
}

pub fn addTypedefIdent(ident: Ident, ty: EnvMonad<s, IntermediateType>) -> EnvMonad<s, ()> {
    lift(/*do*/ {
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

pub fn badSource<a>(node: node, msg: String) -> EnvMonad<s, a> {
    noTranslation(node, (__op_addadd("illegal ".to_string(), __op_addadd(msg, "; check whether a real C compiler accepts this".to_string()))))
}

pub fn baseTypeOf(specs: Vec<CDeclSpec>) -> EnvMonad<s, (Option<CStorageSpec>, EnvMonad<s, IntermediateType>)> {
    /*do*/ {
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
                    /*do*/ {
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

pub fn bitWidth(_0: isize, _1: IntWidth) -> isize {
    match (_0, _1) {
        (wordWidth, WordWidth) => {
            wordWidth
        },
        (_, BitWidth(w)) => {
            wordWidth
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

pub fn cfgToRust(_node: node, build: CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)>) -> EnvMonad<s, Vec<Rust::Stmt>> {
    /*do*/ {
        let builder = buildCFG(/*do*/ {
                let (early, term) = build;

                let entry = newLabel;

                addBlock(entry, early, term);
                entry
            });

        let (rawCFG, _) = evalRWST(builder, (OuterLabels(None, None, None)), Map::empty);

        let cfg = depthFirstOrder((removeEmptyBlocks(rawCFG)));

        let (hasGoto, structured) = structureCFG(mkBreak, mkContinue, mkLoop, mkIf, mkGoto, mkMatch, cfg);

        return(__op_concat(if hasGoto { declCurrent }, structured(else, structured)))
    }
}

pub fn charType() -> CType {
    IsInt(Unsigned, (BitWidth(8)))
}

pub fn compatibleInitializer(_0: CType, _1: CType) -> bool {
    match (_0, _1) {
        (IsStruct(name1, _), IsStruct(name2, _)) => {
            (name1 == name2)
        },
        (IsStruct {

        }, _) => {
            (name1 == name2)
        },
        (_, IsStruct {

        }) => {
            (name1 == name2)
        },
        (_, _) => {
            (name1 == name2)
        },
    }
}

pub fn compatiblePtr(_0: CType, _1: CType) -> CType {
    match (_0, _1) {
        (IsPtr(_, IsVoid), b) => {
            b
        },
        (IsArray(__mut, _, el), b) => {
            b
        },
        (a, IsPtr(_, IsVoid)) => {
            b
        },
        (a, IsArray(__mut, _, el)) => {
            b
        },
        (IsPtr(m1, a), IsPtr(m2, b)) => {
            b
        },
        (a, b) => {
            b
        },
        (_, _) => {
            b
        },
    }
}

pub fn completeType(_0: CType, _1: EnvMonad<s, CType>) -> EnvMonad<s, CType> {
    match (_0, _1, _2) {
        (orig, __OP__, IsIncomplete(ident)) => {
            /*do*/ {
                let mty = getTagIdent(ident);

                fromMaybe((orig), mty)
            }
        },
        ty => {
            /*do*/ {
                let mty = getTagIdent(ident);

                fromMaybe((orig), mty)
            }
        },
    }
}

pub fn compound(expr: CExpr, returnOld: bool, demand: bool, op: CAssignOp, lhs: Result, rhs: Result) -> EnvMonad<s, Result> {
    /*do*/ {
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

        let (bindings1, dereflhs, boundrhs) = (if not(duplicateLHS) { () } || hasNoSideEffects((result(lhs)), then, (vec![], lhs, rhs), else, {
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

        let assignment = Rust::Assign((result(dereflhs)), Rust::__id_3a3d(), (castTo((resultType(lhs)), rhs_q)));

        let (bindings2, ret) = if not(demand) {             
(vec![], None)} else {
if not(returnOld) {             
(vec![], Some((result(dereflhs))))} else {
{
                let oldvar = Rust::VarName("_old".to_string());

            (vec![Rust::Let(Rust::Immutable, oldvar, None, (Some((result(dereflhs)))))], Some((Rust::Var(oldvar))))            }
            }
            };

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
    /*do*/ {
        let derived_q = mapM(derive, derived);

        return(/*do*/ {
            let basetype = deferred;

            foldrM((__op_dollar), basetype, derived_q)
        })
    }
}

pub fn derivedTypeOf(deferred: EnvMonad<s, IntermediateType>, declr: CDeclr) -> EnvMonad<s, IntermediateType> {
    join((derivedDeferredTypeOf(deferred, declr, vec![])))
}

pub fn designatorType(_0: Designator) -> CType {
    match (_0) {
        Base(ty) => {
            ty
        },
        From(ty, _, _, _) => {
            ty
        },
    }
}

pub fn emitIncomplete(kind: ItemKind, ident: Ident) -> EnvMonad<s, CType> {
    /*do*/ {
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

pub fn exprToStatements(_0: Rust::Expr) -> Vec<Rust::Stmt> {
    match (_0) {
        Rust::IfThenElse(c, t, f) => {
            vec![Rust::Stmt((Rust::IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))]
        },
        Rust::BlockExpr(b) => {
            vec![Rust::Stmt((Rust::IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))]
        },
        e => {
            vec![Rust::Stmt((Rust::IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))]
        },
    }
}

pub fn getSwitchCases<a>(expr: CExpr) -> CSourceBuildCFGT<s, (a, SwitchCases)> {
    mapBuildCFGT(wrap)
}

pub fn getSwitchExpression(stmt: CStat) -> CSourceBuildCFGT<s, CExpr> {
    /*do*/ {
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
    /*do*/ {
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
    lift(/*do*/ {
        let env = gets(tagEnvironment);

        return(lookup(ident, env))
    })
}

pub fn getTypedefIdent(ident: Ident) -> EnvMonad<s, (String, Option<EnvMonad<s, IntermediateType>>)> {
    lift(/*do*/ {
        let env = gets(typedefEnvironment);

        (identToString(ident), lookup(ident, env))
    })
}

pub fn gotoLabel(ident: Ident) -> CSourceBuildCFGT<s, Label> {
    /*do*/ {
        let labels = lift(get);

        match Map::lookup(ident, labels) {
            None => {
                /*do*/ {
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

pub fn intPromote(_0: CType) -> CType {
    match (_0) {
        IsBool => {
            IsInt(Signed, (BitWidth(32)))
        },
        IsEnum(_) => {
            IsInt(Signed, (BitWidth(32)))
        },
        IsInt(_, BitWidth(w)) => {
            IsInt(Signed, (BitWidth(32)))
        },
        x => {
            IsInt(Signed, (BitWidth(32)))
        },
    }
}

pub fn integerConversionRank(_0: IntWidth, _1: IntWidth) -> Option<Ordering> {
    match (_0, _1) {
        (BitWidth(a), BitWidth(b)) => {
            Some((compare(a, b)))
        },
        (WordWidth, WordWidth) => {
            Some((compare(a, b)))
        },
        (BitWidth(a), WordWidth) => {
            Some((compare(a, b)))
        },
        (WordWidth, BitWidth(b)) => {
            Some((compare(a, b)))
        },
        (_, _) => {
            Some((compare(a, b)))
        },
    }
}

pub fn interpretBlockItem(_0: CBlockItem, _1: CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)>) -> CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)> {
    match (_0, _1) {
        (CBlockStmt(stmt), next) => {
            interpretStatement(stmt, next)
        },
        (CBlockDecl(decl), next) => {
            interpretStatement(stmt, next)
        },
        (item, _) => {
            interpretStatement(stmt, next)
        },
    }
}

pub fn interpretConstExpr(_0: CExpr) -> EnvMonad<s, Integer> {
    match (_0) {
        CConst(CIntConst(CInteger(v, _, _), _)) => {
            v
        },
        expr => {
            v
        },
    }
}

pub fn interpretDeclarations<b>(_0: MakeBinding<s, b>, _1: CDecl, _2: EnvMonad<s, Vec<b>>) -> EnvMonad<s, Vec<b>> {
    match (_0, _1, _2, _3) {
        ((fromItem, makeBinding), declaration, __OP__, CDecl(specs, decls, _)) => {
            /*do*/ {
                let (storagespecs, baseTy) = baseTypeOf(specs);

                let mbinds = forM(decls)(|declarator| { /*do*/ {
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
                                    /*do*/ {
                                        when((isJust(minit)), (badSource(decl, "initializer on typedef".to_string())));
                                        addTypedefIdent(ident, deferred);
                                        None
                                    }
                                },
                                (Some(CStatic(_)), [CFunDeclr {

                                                }, _]) => {
                                    /*do*/ {
                                        addSymbolIdentAction(ident)(/*do*/ {
                                            let itype = deferred;

                                            useForwardRef(ident);
                                            (typeToResult(itype, (Rust::Path((Rust::PathSegments(vec![applyRenames(ident)]))))))
                                        });
                                        None
                                    }
                                },
                                (_, [CFunDeclr {

                                                }, _]) => {
                                    /*do*/ {
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
                                    /*do*/ {
                                        addExternIdent(ident, deferred)(|name, (__mut, ty)| { Rust::ExternStatic(__mut, (Rust::VarName(name)), (toRustType(ty))) });
                                        None
                                    }
                                },
                                (Some(CStatic(_)), _) => {
                                    /*do*/ {
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
                                    /*do*/ {
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
            /*do*/ {
                let (storagespecs, baseTy) = baseTypeOf(specs);

                let mbinds = forM(decls)(|declarator| { /*do*/ {
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
                                    /*do*/ {
                                        when((isJust(minit)), (badSource(decl, "initializer on typedef".to_string())));
                                        addTypedefIdent(ident, deferred);
                                        None
                                    }
                                },
                                (Some(CStatic(_)), [CFunDeclr {

                                                }, _]) => {
                                    /*do*/ {
                                        addSymbolIdentAction(ident)(/*do*/ {
                                            let itype = deferred;

                                            useForwardRef(ident);
                                            (typeToResult(itype, (Rust::Path((Rust::PathSegments(vec![applyRenames(ident)]))))))
                                        });
                                        None
                                    }
                                },
                                (_, [CFunDeclr {

                                                }, _]) => {
                                    /*do*/ {
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
                                    /*do*/ {
                                        addExternIdent(ident, deferred)(|name, (__mut, ty)| { Rust::ExternStatic(__mut, (Rust::VarName(name)), (toRustType(ty))) });
                                        None
                                    }
                                },
                                (Some(CStatic(_)), _) => {
                                    /*do*/ {
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
                                    /*do*/ {
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
    }
}

pub fn interpretExpr(_0: bool, _1: CExpr) -> EnvMonad<s, Result> {
    match (_0, _1) {
        (demand, CComma(exprs, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

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
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (demand, expr, __OP__, CCond(c, Some(t), f, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, expr, __OP__, CBinary(op, lhs, rhs, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, CCast(decl, expr, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (demand, node, __OP__, CUnary(op, expr, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, CSizeofExpr(e, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, CSizeofType(decl, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, CAlignofExpr(e, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, CAlignofType(decl, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, expr, __OP__, CIndex(lhs, rhs, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, expr, __OP__, CCall(func, args, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, expr, __OP__, CMember(obj, ident, deref, node)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, expr, __OP__, CVar(ident, _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, expr, __OP__, CConst(c)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, CCompoundLit(decl, initials, info)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (demand, stat, __OP__, CStatExpr(CCompound([], stmts, _), _)) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
        (_, expr) => {
            /*do*/ {
                let (effects, mfinal) = if demand {                     
(init(exprs), Some((last(exprs))))} else {
(exprs, None)
                    };

                let effects_q = mapM((fmap(resultToStatements, interpretExpr(false))), effects);

                let mfinal_q = mapM((interpretExpr(true)), mfinal);

                Result {
                    resultType: maybe(IsVoid, resultType, mfinal_q),
                    resultMutable: maybe(Rust::Immutable, resultMutable, mfinal_q),
                    result: Rust::BlockExpr((Rust::Block((concat(effects_q)), (fmap(result, mfinal_q)))))
                }
            }
        },
    }
}

pub fn interpretFunction(CFunDef(specs, declr, __OP__, CDeclr(mident, _, _, _, _), argtypes, body, _): CFunDef) -> EnvMonad<s, ()> {
    /*do*/ {
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
            /*do*/ {
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

                when(((name == "_c_main".to_string())), (wrapMain(declr, name, (__map!(snd, args)))));
                let setRetTy = |flow| {
                    flow {
                        functionReturnType: Some(retTy),
                        functionName: Some(name)
                    }
                };

                let f_q = mapExceptT((local(setRetTy)))(scope(/*do*/ {
                        let formals = sequence(/* Expr::Generator */ Generator);

                        let returnValue = (if name { () } == "_c_main".to_string()(then, Some, 0, else, None));

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
            Rust::Private if Set::notMember(ident, alreadyUsed) => { /*do*/ {
                let action = runOnce(/*do*/ {
                        let ty = deferred;

                        go(name, (resultType(ty)));
                        ty
                    });

                addSymbolIdentAction(ident, action)
            } }
            _ => {
                /*do*/ {
                    let ty = deferred;

                    addSymbolIdentAction(ident)(ty);
                    go(name, (resultType(ty)))
                }
            },
        }
    }
}

pub fn interpretInitializer(ty: CType, initial: CInit) -> EnvMonad<s, Rust::Expr> {
    /*do*/ {
        let initial_q = match initial {
                CInitExpr(expr, _) => {
                    /*do*/ {
                        let expr_q = interpretExpr(true, expr);

                        compatibleInitializer(if resultType(expr_q) { () }, ty(then, __pure)(scalar((castTo(ty, expr_q)), else, badSource, initial, "initializer for incompatible type".to_string())))
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

pub fn interpretStatement(_0: CStat, _1: CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)>) -> CSourceBuildCFGT<s, (Vec<Rust::Stmt>, Terminator<Result>)> {
    match (_0, _1) {
        (CLabel(ident, body, _, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (stmt, __OP__, CCase(expr, body, node), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (stmt, __OP__, CCases(lower, upper, body, node), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (CDefault(body, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (CExpr(None, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (CExpr(Some(expr), _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (CCompound([], items, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (CIf(c, t, mf, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (stmt, __OP__, CSwitch(expr, body, node), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (CWhile(c, body, doWhile, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (CFor(initial, mcond, mincr, body, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (CGoto(ident, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (stmt, __OP__, CCont(_), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (stmt, __OP__, CBreak(_), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (stmt, __OP__, CReturn(expr, _), next) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
        },
        (stmt, _) => {
            /*do*/ {
                let label = gotoLabel(ident);

                let (rest, end) = interpretStatement(body, next);

                addBlock(label, rest, end);
                (vec![], Branch(label))
            }
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

pub fn modifyGlobal<a>(f: fn(GlobalState) -> (GlobalState, a)) -> EnvMonad<s, a> {
    lift(/*do*/ {
        let st = get;

        let (global_q, a) = f((globalState(st)));

        put(st {
                globalState: global_q
            });
        a
    })
}

pub fn mutable<a>(quals: Vec<CTypeQualifier<a>>) -> Rust::Mutable {
    if any((|q| { match q {
                CConstQual(_) => {
                    true
                },
                _ => {
                    false
                },
            } }), quals) {     
Rust::Immutable} else {
Rust::Mutable
    }
}

pub fn nestedObject(ty: CType, desig: Designator) -> Option<Designator> {
    match designatorType(desig) {
        IsArray(_, size, el) => {
            Some((From(el, 0, (replicate(((size - 1)), el)), desig)))
        },
        ty_q if compatibleInitializer(ty, ty_q) => { Some(desig) }
        IsStruct(_, [(_, ty_q), fields]) => {
            nestedObject(ty, (From(ty_q, 0, (__map!(snd, fields)), desig)))
        },
        _ => {
            None
        },
    }
}

pub fn nextObject(_0: Designator) -> CurrentObject {
    match (_0) {
        Base {

        } => {
            None
        },
        From(_, i, [ty, remaining], base) => {
            None
        },
        From(_, _, [], base) => {
            None
        },
    }
}

pub fn noTranslation<a>(node: node, msg: String) -> EnvMonad<s, a> {
    throwE(concat(vec![
            show((posOf(node))),
            ": ".to_string(),
            msg,
            ":\n".to_string(),
            render((nest(4, (pretty(node))))),
        ]))
}

pub fn objectFromDesignators(_0: CType, _1: Vec<CDesignator>) -> EnvMonad<s, CurrentObject> {
    match (_0, _1) {
        (_, []) => {
            __pure(None)
        },
        (ty, desigs) => {
            __pure(None)
        },
    }
}

pub fn promote<a, b>(node: node, op: fn(Rust::Expr) -> fn(Rust::Expr) -> Rust::Expr, a: Result, b: Result) -> EnvMonad<s, Result> {
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

pub fn promotePtr<a, b>(node: node, op: fn(Rust::Expr) -> fn(Rust::Expr) -> Rust::Expr, a: Result, b: Result) -> EnvMonad<s, Result> {
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
            /*do*/ {
                let (obj_q, initial) = match cinitial {
                        CInitList(list_q, _) => {
                            /*do*/ {
                                let initial = translateInitList((designatorType(obj)), list_q);

                                (obj, initial)
                            }
                        },
                        CInitExpr(expr, _) => {
                            /*do*/ {
                                let expr_q = interpretExpr(true, expr);

                                match nestedObject((resultType(expr_q)), obj) {
                                    None => {
                                        badSource(cinitial, "type in initializer".to_string())
                                    },
                                    Some(obj_q) => {
                                        /*do*/ {
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

pub fn runOnce<a>(action: EnvMonad<s, a>) -> EnvMonad<s, EnvMonad<s, a>> {
    /*do*/ {
        let cacheRef = lift(lift(newSTRef((Left(action)))));

        return(/*do*/ {
            let cache = lift(lift(readSTRef(cacheRef)));

            match cache {
                Left(todo) => {
                    /*do*/ {
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

pub fn scope<a>(m: EnvMonad<s, a>) -> EnvMonad<s, a> {
    /*do*/ {
        let old = lift(get);

        let a = m;

        lift((modify((|st| { old {
                        globalState: globalState(st)
                    } }))));
        a
    }
}

pub fn setBreak<a>(label: Label) -> CSourceBuildCFGT<s, a> {
    mapBuildCFGT((local((|flow| { flow {
                    onBreak: Some(label)
                } }))))
}

pub fn setContinue<a>(label: Label) -> CSourceBuildCFGT<s, a> {
    mapBuildCFGT((local((|flow| { flow {
                    onContinue: Some(label)
                } }))))
}

pub fn statementsToBlock(_0: Vec<Rust::Stmt>) -> Rust::Block {
    match (_0) {
        [Rust::Stmt(Rust::BlockExpr(stmts))] => {
            stmts
        },
        stmts => {
            stmts
        },
    }
}

pub fn toBool(_0: Result) -> Rust::Expr {
    match (_0) {
        Result {

            } => {
            Rust::Lit((Rust::LitBool(false)))
        },
        Result {

            } => {
            Rust::Lit((Rust::LitBool(false)))
        },
        Result {

            } => {
            Rust::Lit((Rust::LitBool(false)))
        },
    }
}

pub fn toNotBool(_0: Result) -> Rust::Expr {
    match (_0) {
        Result {

            } => {
            Rust::Lit((Rust::LitBool(true)))
        },
        Result {

            } => {
            Rust::Lit((Rust::LitBool(true)))
        },
        Result {

            } => {
            Rust::Lit((Rust::LitBool(true)))
        },
    }
}

pub fn toPtr(_0: Result, _1: Option<Result>) -> Option<Result> {
    match (_0, _1, _2) {
        (ptr, __OP__, Result {

            }) => {
            Some(ptr {
                    resultType: IsPtr(__mut, el),
                    result: castTo((IsPtr(__mut, el)), ptr)
                })
        },
        (ptr, __OP__, Result {

            }) => {
            Some(ptr {
                    resultType: IsPtr(__mut, el),
                    result: castTo((IsPtr(__mut, el)), ptr)
                })
        },
        _ => {
            Some(ptr {
                    resultType: IsPtr(__mut, el),
                    result: castTo((IsPtr(__mut, el)), ptr)
                })
        },
    }
}

pub fn toRustRetType(_0: CType) -> Rust::Type {
    match (_0) {
        IsVoid => {
            Rust::TypeName("()".to_string())
        },
        ty => {
            Rust::TypeName("()".to_string())
        },
    }
}

pub fn toRustType(_0: CType) -> Rust::Type {
    match (_0) {
        IsBool => {
            Rust::TypeName("bool".to_string())
        },
        IsInt(s, w) => {
            Rust::TypeName("bool".to_string())
        },
        IsFloat(w) => {
            Rust::TypeName("bool".to_string())
        },
        IsVoid => {
            Rust::TypeName("bool".to_string())
        },
        IsFunc(retTy, args, variadic) => {
            Rust::TypeName("bool".to_string())
        },
        IsPtr(__mut, to) => {
            Rust::TypeName("bool".to_string())
        },
        IsArray(_, size, el) => {
            Rust::TypeName("bool".to_string())
        },
        IsStruct(name, _fields) => {
            Rust::TypeName("bool".to_string())
        },
        IsEnum(name) => {
            Rust::TypeName("bool".to_string())
        },
        IsIncomplete(ident) => {
            Rust::TypeName("bool".to_string())
        },
    }
}

pub fn translateInitList(ty: CType, list: CInitList) -> EnvMonad<s, Initializer> {
    /*do*/ {
        let objectsAndInitializers = forM(list)(|(desigs, initial)| { /*do*/ {
                    let currObj = objectFromDesignators(ty, desigs);

                    __pure((currObj, initial))
                } });

        let base = match ty {
                IsArray(_, size, el) => {
                    From(el, 0, (replicate(((size - 1)), el)), (Base(ty)))
                },
                IsStruct(_, [(_, ty_q), fields]) => {
                    From(ty_q, 0, (__map!(snd, fields)), (Base(ty)))
                },
                _ => {
                    Base(ty)
                },
            };

        let (_, initializer) = foldM(resolveCurrentObject, (Some(base), mempty), objectsAndInitializers);

        initializer
    }
}

pub fn typeName(_0: CDecl, _1: EnvMonad<s, (Rust::Mutable, CType)>) -> EnvMonad<s, (Rust::Mutable, CType)> {
    match (_0, _1, _2) {
        (decl, __OP__, CStaticAssert {

            }) => {
            badSource(decl, "static assert in type name ".to_string())
        },
        (decl, __OP__, CDecl(spec, declarators, _)) => {
            badSource(decl, "static assert in type name ".to_string())
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

pub fn unimplemented<a>(node: node) -> EnvMonad<s, a> {
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

pub fn usual(_0: CType, _1: CType) -> Option<CType> {
    match (_0, _1) {
        (IsFloat(aw), IsFloat(bw)) => {
            Some((IsFloat((max(aw, bw)))))
        },
        (a, __OP__, IsFloat(_), _) => {
            Some((IsFloat((max(aw, bw)))))
        },
        (_, b, __OP__, IsFloat(_)) => {
            Some((IsFloat((max(aw, bw)))))
        },
        (origA, origB) => {
            Some((IsFloat((max(aw, bw)))))
        },
    }
}

pub fn wrapMain(declr: CDeclr, realName: String, argTypes: Vec<CType>) -> EnvMonad<s, ()> {
    /*do*/ {
        let (setup, args) = wrapArgv(argTypes);

        let ret = Rust::VarName("ret".to_string());

        emitItems(vec![
                Rust::Item(vec![], Rust::Private, (Rust::Function(vec![], "main".to_string(), vec![], (Rust::TypeName("()".to_string())), (statementsToBlock((__op_addadd(setup, __op_addadd(vec![
                            bind(Rust::Immutable, ret)(Rust::UnsafeExpr(Rust::Block(vec![])(Some(call(realName, args))))),
                        ], exprToStatements((call("::std::process::exit".to_string(), vec![Rust::Var(ret)]))))))))))),
            ])
    }
}

pub fn wrapping(_0: Result, _1: Result) -> Result {
    match (_0, _1, _2) {
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
    }
}



