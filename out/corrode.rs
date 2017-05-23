mod Language_Rust_AST {
    #[derive(Debug, Eq)]
    enum LitIntRepr {
        DecRepr,
        OctalRepr,
        HexRepr
    }

    #[derive(Debug, Eq)]
    enum Lit {
        LitByteStr(String),
        LitByteChar(Char),
        LitBool(Bool),
        LitInt(Integer, LitIntRepr, Type),
        LitFloat(String, Type)
    }

    #[derive(Debug, Eq)]
    enum Visibility {
        Public,
        Private
    }

    #[derive(Debug, Eq)]
    enum Mutable {
        Immutable,
        Mutable
    }

    #[derive(Debug)]
    enum Stmt {
        Stmt(Expr),
        Let(Mutable, Var, Option<Type>, Option<Expr>),
        StmtItem(Vec<Attribute>, ItemKind)
    }

    #[derive(Debug)]
    struct Block(Block, Vec<Stmt>, Option<Expr>);

    #[derive(Debug)]
    struct Attribute(Attribute, String);

    #[derive(Debug)]
    struct Item(Item, Vec<Attribute>, Visibility, ItemKind);

    #[derive(Debug)]
    enum FunctionAttribute {
        UnsafeFn,
        ExternABI(Option<String>)
    }

    #[derive(Debug)]
    enum ItemKind {
        Function(Vec<FunctionAttribute>, String, Vec<(Mutable, Var, Type)>, Type, Block),
        Static(Mutable, Var, Type, Expr),
        Struct(String, Vec<(String, Type)>),
        Extern(Vec<ExternItem>),
        Use(String),
        Enum(String, Vec<Enumerator>),
        CloneImpl(Type)
    }

    #[derive(Debug)]
    enum ExternItem {
        ExternFn(String, Vec<(Var, Type)>, Bool, Type),
        ExternStatic(Mutable, Var, Type)
    }

    #[derive(Debug)]
    enum Enumerator {
        EnumeratorAuto(String),
        EnumeratorExpr(String, Expr)
    }

    #[derive(Debug)]
    enum Expr {
        Lit(Lit),
        Var(Var),
        Path(Path),
        Index(Expr, Expr),
        ArrayExpr(Vec<Expr>),
        RepeatArray(Expr, Expr),
        StructExpr(String, Vec<(String, Expr)>, Option<Expr>),
        Call(Expr, Vec<Expr>),
        MethodCall(Expr, Var, Vec<Expr>),
        Lambda(Vec<Var>, Expr),
        Member(Expr, Var),
        BlockExpr(Block),
        UnsafeExpr(Block),
        IfThenElse(Expr, Block, Block),
        Loop(Option<Lifetime>, Block),
        While(Option<Lifetime>, Expr, Block),
        For(Option<Lifetime>, Var, Expr, Block),
        Break(Option<Lifetime>),
        Continue(Option<Lifetime>),
        Return(Option<Expr>),
        Neg(Expr),
        Deref(Expr),
        Not(Expr),
        Borrow(Mutable, Expr),
        Cast(Expr, Type),
        Mul(Expr, Expr),
        Div(Expr, Expr),
        Mod(Expr, Expr),
        Add(Expr, Expr),
        Sub(Expr, Expr),
        ShiftL(Expr, Expr),
        ShiftR(Expr, Expr),
        And(Expr, Expr),
        Xor(Expr, Expr),
        Or(Expr, Expr),
        CmpLT(Expr, Expr),
        CmpGT(Expr, Expr),
        CmpLE(Expr, Expr),
        CmpGE(Expr, Expr),
        CmpEQ(Expr, Expr),
        CmpNE(Expr, Expr),
        LAnd(Expr, Expr),
        LOr(Expr, Expr),
        Range(Expr, Expr),
        Assign(Expr, AssignOp, Expr)
    }

    #[derive(Debug)]
    enum AssignOp {
        __id_3a3d,
        __id_3a2b3d,
        __id_3a2d3d,
        __id_3a2a3d,
        __id_3a2f3d,
        __id_3a253d,
        __id_3a263d,
        __id_3a7c3d,
        __id_3a5e3d,
        __id_3a3c3c3d,
        __id_3a3e3e3d
    }

    #[derive(Debug)]
    enum ExprPosition {
        TopExpr,
        LeftExpr,
        RightExpr
    }

    fn pPrintBlock(__0: Doc, __1: Block) -> Doc {
        match (__0, __1) {
            (pre, Block([], e)) => {
                sep(vec![__op_arrow_concat(pre, text("{".to_string())), nest(4, (maybe(empty, pPrint, e))), text("}".to_string())])
            },
            (pre, Block(ss, e)) => {
                __op_arrow_concat(pre, $+$(text("{".to_string()), $+$(nest(4, (vcat((__op_addadd(map(pPrint, ss), vec![maybe(empty, pPrint, e)]))))), text("}".to_string()))))
            },
        }
    }

}

mod Language_Rust_Corrode_C {
    struct FunctionContext(FunctionContext, { /* struct def */ });

    struct Output(Output, { /* struct def */ });

    struct GlobalState(GlobalState, { /* struct def */ });

    struct EnvState<s>(EnvState, { /* struct def */ });

    struct Initializer(Initializer, Option<Rust__id_3a3a45787072>, IntMap__id_3a3a496e744d6170<Initializer>);

    #[derive(Debug)]
    enum Designator {
        Base(CType),
        From(CType, isize, Vec<CType>, Designator)
    }

    struct OuterLabels(OuterLabels, { /* struct def */ });

    struct Result(Result, { /* struct def */ });

    #[derive(Debug, Eq)]
    enum Signed {
        Signed,
        Unsigned
    }

    #[derive(Debug, Eq)]
    enum IntWidth {
        BitWidth(isize),
        WordWidth
    }

    #[derive(Debug)]
    enum CType {
        IsBool,
        IsInt(Signed, IntWidth),
        IsFloat(isize),
        IsVoid,
        IsFunc(CType, Vec<(Option<(Rust__id_3a3a4d757461626c65, Ident)>, CType)>, Bool),
        IsPtr(Rust__id_3a3a4d757461626c65, CType),
        IsArray(Rust__id_3a3a4d757461626c65, isize, CType),
        IsStruct(String, Vec<(String, CType)>),
        IsEnum(String),
        IsIncomplete(Ident)
    }

    struct IntermediateType(IntermediateType, { /* struct def */ });

    fn addExternIdent(ident: Ident, deferred: EnvMonad) -> EnvMonad {
        /* do */ {
            let action = runOnce(/* do */ {
                let itype = deferred;
                let rewrites = lift(asks(itemRewrites));
                let path = match Map__id_3a3a6c6f6f6b7570((Symbol, identToString(ident)), rewrites) {
                    Some(renamed) => {
                        (__op_concat("".to_string(), renamed))
                    },
                    None => {
                        /* do */ {
                            {
                                let name = applyRenames(ident);
                            };
                            {
                                let ty = (typeMutable(itype), typeRep(itype));
                            };
                            lift(tell(mempty, {
                                outputExterns: Map__id_3a3a73696e676c65746f6e(name, (mkItem(name, ty)))
                            }));
                            vec![name]
                        }
                    },
                };
                (typeToResult(itype, (Rust__id_3a3a50617468((Rust__id_3a3a506174685365676d656e7473(path))))))
            });
            addSymbolIdentAction(ident, action)
        }
    }

    fn addSwitchCase(condition: Option) -> Option {
        /* do */ {
            let condition_q = lift(lift(mapM((interpretExpr(True)), condition)));
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
            lift(tell(SwitchCases(IntMap__id_3a3a73696e676c65746f6e(label, condition_q))));
            (vec![], Branch(label))
        }
    }

    fn addSymbolIdent(ident: Ident, (__mut, ty): (Rust__id_3a3a4d757461626c65, CType)) -> EnvMonad {
        /* do */ {
            {
                let name = applyRenames(ident);
            };
            addSymbolIdentAction(ident)(Result);
            name
        }
    }

    fn addSymbolIdentAction(ident: Ident, action: EnvMonad) -> EnvMonad {
        lift(/* do */ {
            modify(Lambda({
                symbolEnvironment: __op_concat((ident, action), symbolEnvironment(st))
            }))
        })
    }

    fn addTagIdent(ident: Ident, ty: EnvMonad) -> EnvMonad {
        lift(/* do */ {
            modify(Lambda({
                tagEnvironment: __op_concat((ident, ty), tagEnvironment(st))
            }))
        })
    }

    fn addTypedefIdent(ident: Ident, ty: EnvMonad) -> EnvMonad {
        lift(/* do */ {
            modify(Lambda({
                typedefEnvironment: __op_concat((ident, ty), typedefEnvironment(st))
            }))
        })
    }

    fn applyRenames(ident: Ident) -> String {
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

    fn badSource(node: node, msg: String) -> EnvMonad {
        noTranslation(node, (__op_addadd("illegal ".to_string(), __op_addadd(msg, "; check whether a real C compiler accepts this".to_string()))))
    }

    fn baseTypeOf(specs: Vec<CDeclSpec>) -> EnvMonad {
        /* do */ {
            {
                let (storage, _attributes, basequals, basespecs, _inlineNoReturn, _align) = partitionDeclSpecs(specs);
            };
            let mstorage = match storage {
                [] => {
                    Nothing
                },
                [spec] => {
                    (Just(spec))
                },
                _(__id_3a, excess, __id_3a, _) => {
                    badSource(excess, "extra storage class specifier".to_string())
                },
            };
            let base = typedef((mutable(basequals)), basespecs);
            (mstorage, base);

        }
    }

    fn binop(expr: CExpr, op: CBinaryOp, lhs: Result, rhs: Result) -> EnvMonad {
        fmap(wrapping)(match op {
            CMulOp => {
                promote(expr, Rust__id_3a3a4d756c, lhs, rhs)
            },
            CDivOp => {
                promote(expr, Rust__id_3a3a446976, lhs, rhs)
            },
            CRmdOp => {
                promote(expr, Rust__id_3a3a4d6f64, lhs, rhs)
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
                        promote(expr, Rust__id_3a3a416464, lhs, rhs)
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
                            {
                                let ty = IsInt(Signed, WordWidth);
                            };
                            {
                                let size = rustSizeOfType((toRustType(ptrTo)));
                            };
                            Result
                        }
                    },
                    (Some(ptr), _) => {
                        ptr
                    },
                    _ => {
                        promote(expr, Rust__id_3a3a537562, lhs, rhs)
                    },
                }
            },
            CShlOp => {
                shift(Rust__id_3a3a53686966744c)
            },
            CShrOp => {
                shift(Rust__id_3a3a536869667452)
            },
            CLeOp => {
                comparison(Rust__id_3a3a436d704c54)
            },
            CGrOp => {
                comparison(Rust__id_3a3a436d704754)
            },
            CLeqOp => {
                comparison(Rust__id_3a3a436d704c45)
            },
            CGeqOp => {
                comparison(Rust__id_3a3a436d704745)
            },
            CEqOp => {
                comparison(Rust__id_3a3a436d704551)
            },
            CNeqOp => {
                comparison(Rust__id_3a3a436d704e45)
            },
            CAndOp => {
                promote(expr, Rust__id_3a3a416e64, lhs, rhs)
            },
            CXorOp => {
                promote(expr, Rust__id_3a3a586f72, lhs, rhs)
            },
            COrOp => {
                promote(expr, Rust__id_3a3a4f72, lhs, rhs)
            },
            CLndOp => {
                Result
            },
            CLorOp => {
                Result
            },
        })
    }

    fn bitWidth(__0: isize, __1: IntWidth) -> isize {
        match (__0, __1) {
            (wordWidth, WordWidth) => {
                wordWidth
            },
            (_, BitWidth(w)) => {
                w
            },
        }
    }

    fn blockToStatements((Rust__id_3a3a426c6f636b(stmts, mexpr)): Rust__id_3a3a426c6f636b) -> Vec<Rust__id_3a3a53746d74> {
        match mexpr {
            Some(expr) => {
                __op_addadd(stmts, exprToStatements(expr))
            },
            None => {
                stmts
            },
        }
    }

    fn castTo(__0: CType, __1: Result) -> Rust__id_3a3a45787072 {
        match (__0, __1) {
            (target, Result(<todo>)) => {
                castTo(target, Result, {
                    resultType: IsPtr(__mut, el),
                    resultMutable: Rust__id_3a3a496d6d757461626c65,
                    result: Rust__id_3a3a4d6574686f6443616c6c(source, (Rust__id_3a3a5661724e616d65(method)), vec![])
                })
            },
            (IsBool, source) => {
                toBool(source)
            },
            (target, <todo>, IsInt(<todo>), Result(<todo>)) => {
                Rust__id_3a3a4c6974((Rust__id_3a3a4c6974496e74(n, repr, (toRustType(target)))))
            },
            (IsInt(Signed, w), Result(<todo>)) => {
                Rust__id_3a3a4e6567((Rust__id_3a3a4c6974((Rust__id_3a3a4c6974496e74(n, repr, (toRustType((IsInt(Signed, w)))))))))
            },
            (target, source) => {
                Rust__id_3a3a43617374((result(source)), (toRustType(target)))
            },
        }
    }

    fn cfgToRust(_node: node, build: CSourceBuildCFGT) -> CSourceBuildCFGT {
        /* do */ {
            {
                let builder = buildCFG(/* do */ {
                        let (early, term) = build;
                        let entry = newLabel;
                        addBlock(entry, early, term);
                        entry
                    });
            };
            let (rawCFG, _) = evalRWST(builder, (OuterLabels(Nothing, Nothing, Nothing)), Map__id_3a3a656d707479);
            {
                let cfg = depthFirstOrder((removeEmptyBlocks(rawCFG)));
            };
            {
                let (hasGoto, structured) = structureCFG(mkBreak, mkContinue, mkLoop, mkIf, mkGoto, mkMatch, cfg);
            };
            return(__op_concat(if(hasGoto, then, declCurrent), structured(else, structured)));

        }
    }

    fn charType() -> CType {
        IsInt(Unsigned, (BitWidth(8)))
    }

    fn compatibleInitializer(__0: CType, __1: CType) -> Bool {
        match (__0, __1) {
            (IsStruct(name1, _), IsStruct(name2, _)) => {
                (name1 == name2)
            },
            (IsStruct, <todo>, _) => {
                False
            },
            (_, IsStruct, <todo>) => {
                False
            },
            (_, _) => {
                True
            },
        }
    }

    fn compatiblePtr(__0: CType, __1: CType) -> CType {
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
            (_, _) => {
                IsVoid
            },
        }
    }

    fn completeType(__0: CType, __1: EnvMonad) -> EnvMonad {
        match (__0, __1, __2) {
            (orig, <todo>, IsIncomplete(ident)) => {
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

    fn compound(expr: CExpr, returnOld: Bool, demand: Bool, op: CAssignOp, lhs: Result, rhs: Result) -> EnvMonad {
        /* do */ {
            {
                let op_q = match op {
                        CAssignOp => {
                            Nothing
                        },
                        CMulAssOp => {
                            Just(CMulOp)
                        },
                        CDivAssOp => {
                            Just(CDivOp)
                        },
                        CRmdAssOp => {
                            Just(CRmdOp)
                        },
                        CAddAssOp => {
                            Just(CAddOp)
                        },
                        CSubAssOp => {
                            Just(CSubOp)
                        },
                        CShlAssOp => {
                            Just(CShlOp)
                        },
                        CShrAssOp => {
                            Just(CShrOp)
                        },
                        CAndAssOp => {
                            Just(CAndOp)
                        },
                        CXorAssOp => {
                            Just(CXorOp)
                        },
                        COrAssOp => {
                            Just(COrOp)
                        },
                    };
            };
            {
                let duplicateLHS = ||(isJust(op_q), demand);
            };
            {
                let (bindings1, dereflhs, boundrhs) = ||(if(not, duplicateLHS), hasNoSideEffects((result(lhs)), then, (vec![], lhs, rhs), else, {
                            let lhsvar = Rust__id_3a3a5661724e616d65("_lhs".to_string());
;
                            let rhsvar = Rust__id_3a3a5661724e616d65("_rhs".to_string());
                        }, in, (vec![Rust__id_3a3a4c6574(Rust__id_3a3a496d6d757461626c65, rhsvar, Nothing, (Just((result(rhs))))), Rust__id_3a3a4c6574(Rust__id_3a3a496d6d757461626c65, lhsvar, Nothing, (Just((Rust__id_3a3a426f72726f77(Rust__id_3a3a4d757461626c65, (result(lhs)))))))], lhs({
                            result: Rust__id_3a3a4465726566((Rust__id_3a3a566172(lhsvar)))
                        }), rhs({
                            result: Rust__id_3a3a566172(rhsvar)
                        }))));
            };
            let rhs_q = match op_q {
                Some(o) => {
                    binop(expr, o, dereflhs, boundrhs)
                },
                None => {
                    boundrhs
                },
            };
            {
                let assignment = Rust__id_3a3a41737369676e((result(dereflhs)), (Rust__id_3a3a3a3d), (castTo((resultType(lhs)), rhs_q)));
            };
            {
                let (bindings2, ret) = if(not, demand, then, (vec![], Nothing), else, if, not, returnOld, then, (vec![], Just((result(dereflhs)))), else, {
                            let oldvar = Rust__id_3a3a5661724e616d65("_old".to_string());
                        }, in, (vec![Rust__id_3a3a4c6574(Rust__id_3a3a496d6d757461626c65, oldvar, Nothing, (Just((result(dereflhs)))))], Just((Rust__id_3a3a566172(oldvar)))));
            };
            return(match Rust__id_3a3a426c6f636b((__op_addadd(bindings1, __op_addadd(bindings2, exprToStatements(assignment)))), ret) {
                b(@, Rust__id_3a3a426c6f636b(body, None)) => {
                    Result({
                        resultType: IsVoid,
                        resultMutable: Rust__id_3a3a496d6d757461626c65,
                        result: match body {
                                    [Rust__id_3a3a53746d74(e)] => {
                                        e
                                    },
                                    _ => {
                                        Rust__id_3a3a426c6f636b45787072(b)
                                    },
                                }
                    })
                },
                b => {
                    lhs({
                        result: Rust__id_3a3a426c6f636b45787072(b)
                    })
                },
            });

        }
    }

    fn derivedDeferredTypeOf(deferred: EnvMonad) -> EnvMonad {
        /* do */ {
            let derived_q = mapM(derive, derived);
            return(/* do */ {
                let basetype = deferred;
                foldrM((Operator("$")), basetype, derived_q)
            });

        }
    }

    fn derivedTypeOf(deferred: EnvMonad) -> EnvMonad {
        join((derivedDeferredTypeOf(deferred, declr, vec![])))
    }

    fn designatorType(__0: Designator) -> CType {
        match (__0) {
            Base(ty) => {
                ty
            },
            From(ty, _, _, _) => {
                ty
            },
        }
    }

    fn emitIncomplete(kind: ItemKind, ident: Ident) -> EnvMonad {
        /* do */ {
            let rewrites = lift((asks(itemRewrites)));
            unless((Map__id_3a3a6d656d626572((kind, identToString(ident)), rewrites)))(lift(tell(mempty, {
                outputIncomplete: Set__id_3a3a73696e676c65746f6e((identToString(ident)))
            })));
            (IsIncomplete(ident))
        }
    }

    fn emitItems(items: Vec<Rust__id_3a3a4974656d>) -> EnvMonad {
        lift(tell(mempty, {
            outputItems: items
        }))
    }

    fn enumReprType() -> CType {
        IsInt(Signed, (BitWidth(32)))
    }

    fn exprToStatements(__0: Rust__id_3a3a45787072) -> Vec<Rust__id_3a3a53746d74> {
        match (__0) {
            Rust__id_3a3a49665468656e456c7365(c, t, f) => {
                vec![Rust__id_3a3a53746d74((Rust__id_3a3a49665468656e456c7365(c, (extractExpr(t)), (extractExpr(f)))))]
            },
            Rust__id_3a3a426c6f636b45787072(b) => {
                blockToStatements(b)
            },
            e => {
                vec![Rust__id_3a3a53746d74(e)]
            },
        }
    }

    fn getSwitchCases(expr: CExpr) -> CSourceBuildCFGT {
        mapBuildCFGT(wrap)
    }

    fn getSwitchExpression(stmt: CStat) -> CSourceBuildCFGT {
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

    fn getSymbolIdent(ident: Ident) -> EnvMonad {
        /* do */ {
            let env = lift(get);
            match lookup(ident, (symbolEnvironment(env))) {
                Some(symbol) => {
                    fmap(Just, symbol)
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
            };

        }
    }

    fn getTagIdent(ident: Ident) -> EnvMonad {
        lift(/* do */ {
            let env = gets(tagEnvironment);
            return(lookup(ident, env))
        })
    }

    fn getTypedefIdent(ident: Ident) -> EnvMonad {
        lift(/* do */ {
            let env = gets(typedefEnvironment);
            (identToString(ident), lookup(ident, env))
        })
    }

    fn gotoLabel(ident: Ident) -> CSourceBuildCFGT {
        /* do */ {
            let labels = lift(get);
            match Map__id_3a3a6c6f6f6b7570(ident, labels) {
                None => {
                    /* do */ {
                        let label = newLabel;
                        lift((put((Map__id_3a3a696e73657274(ident, label, labels)))));
                        label
                    }
                },
                Some(label) => {
                    label
                },
            }
        }
    }

    fn intPromote(__0: CType) -> CType {
        match (__0) {
            IsBool => {
                IsInt(Signed, (BitWidth(32)))
            },
            IsEnum(_) => {
                enumReprType
            },
            x => {
                x
            },
        }
    }

    fn integerConversionRank(__0: IntWidth, __1: IntWidth) -> Option {
        match (__0, __1) {
            (BitWidth(a), BitWidth(b)) => {
                Just((compare(a, b)))
            },
            (WordWidth, WordWidth) => {
                Just(EQ)
            },
            (_, _) => {
                Nothing
            },
        }
    }

    fn interpretBlockItem(__0: CBlockItem, __1: CSourceBuildCFGT) -> CSourceBuildCFGT {
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

    fn interpretConstExpr(__0: CExpr) -> EnvMonad {
        match (__0) {
            CConst(CIntConst(CInteger(v, _, _), _)) => {
                v
            },
            expr => {
                unimplemented(expr)
            },
        }
    }

    fn interpretDeclarations(__0: MakeBinding) -> MakeBinding {
        match (__0, __1, __2, __3) {
            (fromItem(makeBinding), declaration, <todo>, CDecl(specs, decls, _)) => {
                /* do */ {
                    let (storagespecs, baseTy) = baseTypeOf(specs);
                    let mbinds = forM(decls)(Lambda);
                    (catMaybes(mbinds))
                }
            },
            (_, node, <todo>, CStaticAssert(<todo>)) => {
                unimplemented(node)
            },
        }
    }

    fn interpretExpr(__0: Bool, __1: CExpr) -> EnvMonad {
        match (__0, __1) {
            (demand, CComma(exprs, _)) => {
                /* do */ {
                    {
                        let (effects, mfinal) = if(demand, then, (init(exprs), Just((last(exprs)))), else, (exprs, Nothing));
                    };
                    let effects_q = mapM((fmap(resultToStatements)interpretExpr(False)), effects);
                    let mfinal_q = mapM((interpretExpr(True)), mfinal);
                    Result
                }
            },
            (demand, expr, <todo>, CAssign(op, lhs, rhs, _)) => {
                /* do */ {
                    let lhs_q = interpretExpr(True, lhs);
                    let rhs_q = interpretExpr(True, rhs);
                    compound(expr, False, demand, op, lhs_q, rhs_q)
                }
            },
            (demand, expr, <todo>, CCond(c, Some(t), f, _)) => {
                /* do */ {
                    let c_q = fmap(toBool, (interpretExpr(True, c)));
                    let t_q = interpretExpr(demand, t);
                    let f_q = interpretExpr(demand, f);
                    if(demand, then, promotePtr, expr, (mkIf(c_q)), t_q, f_q, else, return, Result, {
                        resultType: IsVoid,
                        resultMutable: Rust__id_3a3a496d6d757461626c65,
                        result: mkIf(c_q, (result(t_q)), (result(f_q)))
                    });

                }
            },
            (_, expr, <todo>, CBinary(op, lhs, rhs, _)) => {
                /* do */ {
                    let lhs_q = interpretExpr(True, lhs);
                    let rhs_q = interpretExpr(True, rhs);
                    binop(expr, op, lhs_q, rhs_q)
                }
            },
            (_, CCast(decl, expr, _)) => {
                /* do */ {
                    let (_mut, ty) = typeName(decl);
                    let expr_q = interpretExpr((/=(ty, IsVoid)), expr);
                    Result
                }
            },
            (demand, node, <todo>, CUnary(op, expr, _)) => {
                match op {
                    CPreIncOp => {
                        incdec(False, CAddAssOp)
                    },
                    CPreDecOp => {
                        incdec(False, CSubAssOp)
                    },
                    CPostIncOp => {
                        incdec(True, CAddAssOp)
                    },
                    CPostDecOp => {
                        incdec(True, CSubAssOp)
                    },
                    CAdrOp => {
                        /* do */ {
                            let expr_q = interpretExpr(True, expr);
                            {
                                let ty_q = IsPtr((resultMutable(expr_q)), (resultType(expr_q)));
                            };
                            Result
                        }
                    },
                    CIndOp => {
                        /* do */ {
                            let expr_q = interpretExpr(True, expr);
                            match resultType(expr_q) {
                                IsPtr(mut_q, ty_q) => {
                                    Result
                                },
                                IsFunc({ .. }) => {
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
                            {
                                let ty_q = intPromote((resultType(expr_q)));
                            };
                            Result
                        }
                    },
                    CMinOp => {
                        fmap(wrapping)(simple(Rust__id_3a3a4e6567))
                    },
                    CCompOp => {
                        simple(Rust__id_3a3a4e6f74)
                    },
                    CNegOp => {
                        /* do */ {
                            let expr_q = interpretExpr(True, expr);
                            Result
                        }
                    },
                }
            },
            (_, CSizeofExpr(e, _)) => {
                /* do */ {
                    let e_q = interpretExpr(True, e);
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
                    let e_q = interpretExpr(True, e);
                    (rustAlignOfType((toRustType((resultType(e_q))))))
                }
            },
            (_, CAlignofType(decl, _)) => {
                /* do */ {
                    let (_mut, ty) = typeName(decl);
                    (rustAlignOfType((toRustType(ty))))
                }
            },
            (_, expr, <todo>, CIndex(lhs, rhs, _)) => {
                /* do */ {
                    let lhs_q = interpretExpr(True, lhs);
                    let rhs_q = interpretExpr(True, rhs);
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
                                        Result
                                    },
                                    _ => {
                                        badSource(expr, "array subscript of non-pointer".to_string())
                                    },
                                }
                            }
                        },
                    };

                }
            },
            (_, expr, <todo>, CCall(func, args, _)) => {
                /* do */ {
                    let func_q = interpretExpr(True, func);
                    match resultType(func_q) {
                        IsFunc(retTy, argTys, variadic) => {
                            /* do */ {
                                let args_q = castArgs(variadic, (map(snd, argTys)), args);
                                Result
                            }
                        },
                        _ => {
                            badSource(expr, "function call to non-function".to_string())
                        },
                    };

                }
            },
            (_, expr, <todo>, CMember(obj, ident, deref, node)) => {
                /* do */ {
                    let obj_q = interpretExpr(True)(if(deref, then, CUnary, CIndOp, obj, node, else, obj));
                    let objTy = completeType((resultType(obj_q)));
                    let fields = match objTy {
                        IsStruct(_, fields) => {
                            fields
                        },
                        _ => {
                            badSource(expr, "member access of non-struct".to_string())
                        },
                    };
                    {
                        let field = applyRenames(ident);
                    };
                    let ty = match lookup(field, fields) {
                        Some(ty) => {
                            ty
                        },
                        None => {
                            badSource(expr, "request for non-existent field".to_string())
                        },
                    };
                    Result
                }
            },
            (_, expr, <todo>, CVar(ident, _)) => {
                /* do */ {
                    let sym = getSymbolIdent(ident);
                    maybe((badSource(expr, "undefined variable".to_string())), return, sym)
                }
            },
            (_, expr, <todo>, CConst(c)) => {
                match c {
                    CIntConst(CInteger(v, repr, flags), _) => {
                        {
                            let allow_signed = not((testFlag(FlagUnsigned, flags)));
;
                            let allow_unsigned = ||(not(allow_signed), /=(repr, DecRepr));
;
                            let widths = vec![(32, if(any, (Operator("testFlag")(flags)), vec![FlagLongLong, FlagLong], then, WordWidth, else, BitWidth, 32)), (64, BitWidth(64))];
;
                            let allowed_types = Dummy;
;
                            let repr_q = match repr {
                                    DecRepr => {
                                        Rust__id_3a3a44656352657072
                                    },
                                    OctalRepr => {
                                        Rust__id_3a3a4f6374616c52657072
                                    },
                                    HexRepr => {
                                        Rust__id_3a3a48657852657072
                                    },
                                };
                        }(in, match allowed_types {
                                [] => {
                                    badSource(expr, "integer (too big)".to_string())
                                },
                                ty(__id_3a, _) => {
                                    (literalNumber(ty, (Rust__id_3a3a4c6974496e74(v, repr_q))))
                                },
                            })
                    },
                    CFloatConst(CFloat(__str), _) => {
                        match span((Operator("notElem")("fF".to_string())), __str) {
                            (v, "") => {
                                (literalNumber((IsFloat(64)), (Rust__id_3a3a4c6974466c6f6174(v))))
                            },
                            (v, [_]) => {
                                (literalNumber((IsFloat(32)), (Rust__id_3a3a4c6974466c6f6174(v))))
                            },
                            _ => {
                                badSource(expr, "float".to_string())
                            },
                        }
                    },
                    CCharConst(CChar(ch, False), _) => {
                        Result
                    },
                    CStrConst(CString(__str, False), _) => {
                        Result
                    },
                    _ => {
                        unimplemented(expr)
                    },
                }
            },
            (_, CCompoundLit(decl, initials, info)) => {
                /* do */ {
                    let (__mut, ty) = typeName(decl);
                    let final = interpretInitializer(ty, (CInitList(initials, info)));
                    Result
                }
            },
            (demand, stat, <todo>, CStatExpr(CCompound([], stmts, _), _)) => {
                scope(/* do */ {
                    {
                        let (effects, final) = match last(stmts) {
                                CBlockStmt, CExpr(expr, _) => if demand { (init(stmts), expr) },
                                _ => {
                                    (stmts, Nothing)
                                },
                            };
                    };
                    let effects_q = cfgToRust(stat, (foldr(interpretBlockItem, ((vec![], Unreachable)), effects)));
                    let final_q = mapM((interpretExpr(True)), final);
                    Result
                })
            },
            (_, expr) => {
                unimplemented(expr)
            },
        }
    }

    fn interpretFunction((@(CFunDef(specs, declr), (CDeclr(mident, _, _, _, _))(argtypes, body, _))): CFunDef) -> EnvMonad {
        /* do */ {
            let (storage, baseTy) = baseTypeOf(specs);
            let (attrs, vis) = match storage {
                None => {
                    (vec![Rust__id_3a3a417474726962757465("no_mangle".to_string())], Rust__id_3a3a5075626c6963)
                },
                Some(CStatic(_)) => {
                    (vec![], Rust__id_3a3a50726976617465)
                },
                Some(s) => {
                    badSource(s, "storage class specifier for function".to_string())
                },
            };
            {
                let go = |name, funTy| {
                    /* do */ {
                        let (retTy, args) = match funTy {
                            IsFunc(_, _, True) => {
                                unimplemented(declr)
                            },
                            IsFunc(retTy, args, False) => {
                                (retTy, args)
                            },
                            _ => {
                                badSource(declr, "function definition".to_string())
                            },
                        };
                        when(((name == "_c_main".to_string())), (wrapMain(declr, name, (map(snd, args)))));
                        {
                            let setRetTy = |flow| {
                                flow({
                                    functionReturnType: Just(retTy),
                                    functionName: Just(name)
                                })
                            };
                        };
                        let f_q = mapExceptT((local(setRetTy)))(scope(/* do */ {
                            let formals = sequence(Dummy);
                            {
                                let returnValue = (if(name) == "_c_main".to_string()(then, Just, 0, else, Nothing));
;
                                let returnStatement = Rust__id_3a3a53746d74((Rust__id_3a3a52657475726e(returnValue)));
                            };
                            let body_q = cfgToRust(declr, (interpretStatement(body, ((vec![returnStatement], Unreachable)))));
                            (Rust__id_3a3a4974656d(attrs, vis, (Rust__id_3a3a46756e6374696f6e(vec![Rust__id_3a3a556e73616665466e, Rust__id_3a3a45787465726e414249(Nothing)], name, formals, (toRustRetType(retTy)), (statementsToBlock(body_q))))))
                        }));
                        emitItems(vec![f_q])
                    }
                };
            };
            let ident = match mident {
                None => {
                    badSource(declr, "anonymous function definition".to_string())
                },
                Some(ident) => {
                    ident
                },
            };
            {
                let name = applyRenames(ident);
            };
            {
                let funTy = |itype| {
                    typeToResult(itype, (Rust__id_3a3a50617468((Rust__id_3a3a506174685365676d656e7473(vec![name])))))
                };
            };
            let deferred = fmap((fmap(funTy)), (derivedDeferredTypeOf(baseTy, declr, argtypes)));
            let alreadyUsed = lift(gets((usedForwardRefsglobalState)));
            match vis {
                Rust__id_3a3a50726976617465 => if Set.notMember(ident, alreadyUsed) { /* do */ {
                let action = runOnce(/* do */ {
                    let ty = deferred;
                    go(name, (resultType(ty)));
                    ty
                });
                addSymbolIdentAction(ident, action)
            } },
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

    fn interpretInitializer(ty: CType, initial: CInit) -> EnvMonad {
        /* do */ {
            let initial_q = match initial {
                CInitExpr(expr, _) => {
                    /* do */ {
                        let expr_q = interpretExpr(True, expr);
                        compatibleInitializer(if(resultType, expr_q), ty(then, pure)(scalar((castTo(ty, expr_q)), else, badSource, initial, "initializer for incompatible type".to_string())))
                    }
                },
                CInitList(list, _) => {
                    translateInitList(ty, list)
                },
            };
            let zeroed = zeroInitialize(initial_q, ty);
            helper(ty, zeroed);

        }
    }

    fn interpretStatement(__0: CStat, __1: CSourceBuildCFGT) -> CSourceBuildCFGT {
        match (__0, __1) {
            (CLabel(ident, body, _, _), next) => {
                /* do */ {
                    let label = gotoLabel(ident);
                    let (rest, end) = interpretStatement(body, next);
                    addBlock(label, rest, end);
                    (vec![], Branch(label))
                }
            },
            (stmt, <todo>, CCase(expr, body, node), next) => {
                /* do */ {
                    let selector = getSwitchExpression(stmt);
                    {
                        let condition = CBinary(CEqOp, selector, expr, node);
                    };
                    addSwitchCase((Just(condition)), body, next)
                }
            },
            (stmt, <todo>, CCases(lower, upper, body, node), next) => {
                /* do */ {
                    let selector = getSwitchExpression(stmt);
                    {
                        let condition = CBinary(CLndOp, (CBinary(CGeqOp, selector, lower, node)), (CBinary(CLeqOp, selector, upper, node)), node);
                    };
                    addSwitchCase((Just(condition)), body, next)
                }
            },
            (CDefault(body, _), next) => {
                addSwitchCase(Nothing, body, next)
            },
            (CExpr(None, _), next) => {
                next
            },
            (CExpr(Some(expr), _), next) => {
                /* do */ {
                    let expr_q = lift(lift(interpretExpr(False, expr)));
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
                    let c_q = lift(lift(interpretExpr(True, c)));
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
            (stmt, <todo>, CSwitch(expr, body, node), next) => {
                /* do */ {
                    let (bindings, expr_q) = match expr {
                        CVar({ .. }) => {
                            (vec![], expr)
                        },
                        _ => {
                            lift(lift(/* do */ {
                                let ident = fmap(internalIdent, (uniqueName("switch".to_string())));
                                let rhs = interpretExpr(True, expr);
                                let var = addSymbolIdent(ident, (Rust__id_3a3a496d6d757461626c65, resultType(rhs)));
                                (vec![Rust__id_3a3a4c6574(Rust__id_3a3a496d6d757461626c65, (Rust__id_3a3a5661724e616d65(var)), Nothing, (Just((result(rhs)))))], CVar(ident, node))
                            }))
                        },
                    };
                    let after = newLabel;
                    let (_, SwitchCases(cases)) = getSwitchCases(expr_q)(setBreak(after)(interpretStatement(body, ((vec![], Branch(after))))));
                    {
                        let isDefault = |(Just(condition))| {
                            Left(condition)
                        };
;
                        let isDefault = |Nothing| {
                            Right(())
                        };
                    };
                    {
                        let (conditions, defaults) = IntMap__id_3a3a6d6170456974686572(isDefault, cases);
                    };
                    let defaultCase = match IntMap__id_3a3a6b657973(defaults) {
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
                    let entry = foldrM(conditionBlock, defaultCase, (IntMap__id_3a3a746f4c697374(conditions)));
                    let (rest, end) = next;
                    addBlock(after, rest, end);
                    (bindings, Branch(entry));

                }
            },
            (CWhile(c, body, doWhile, _), next) => {
                /* do */ {
                    let c_q = lift(lift(interpretExpr(True, c)));
                    let after = newLabel;
                    let headerLabel = newLabel;
                    let (bodyEntry, bodyTerm) = setBreak(after)(setContinue(headerLabel)(interpretStatement(body, ((vec![], Branch(headerLabel))))));
                    let bodyLabel = newLabel;
                    addBlock(bodyLabel, bodyEntry, bodyTerm);
                    addBlock(headerLabel, vec![])(match toBool(c_q) {
                        Rust__id_3a3a4c6974, Rust__id_3a3a4c6974426f6f6c(cont) => if /=(cont, doWhile) { Branch((if(cont, then, bodyLabel, else, after))) },
                        _ => {
                            CondBranch(c_q, bodyLabel, after)
                        },
                    });
                    let (rest, end) = next;
                    addBlock(after, rest, end);
                    (vec![], Branch((if(doWhile, then, bodyLabel, else, headerLabel))))
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
                                    let expr_q = lift(lift(interpretExpr(False, expr)));
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
                                    let incr_q = lift(lift(interpretExpr(False, incr)));
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
                                    let cond_q = lift(lift(interpretExpr(True, cond)));
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
            (stmt, <todo>, CCont(_), next) => {
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
            (stmt, <todo>, CBreak(_), next) => {
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
            (stmt, <todo>, CReturn(expr, _), next) => {
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
                                    let expr_q = mapM((fmap((castTo(retTy)))interpretExpr(True)), expr);
                                    (exprToStatements((Rust__id_3a3a52657475726e(expr_q))), Unreachable)
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

    fn interpretTranslationUnit(_thisModule: ModuleMap, rewrites: ItemRewrites, (CTranslUnit(decls, _)): CTranslUnit) -> Either {
        match err {
            Left(msg) => {
                Left(msg)
            },
            Right(_) => {
                Right(items_q)
            },
        }
    }

    fn makeLetBinding() -> MakeBinding {
        (Rust__id_3a3a53746d744974656d(vec![]), makeBinding)
    }

    fn makeStaticBinding() -> MakeBinding {
        (Rust__id_3a3a4974656d(vec![], Rust__id_3a3a50726976617465), makeBinding)
    }

    fn modifyGlobal(f: fn(GlobalState) -> (GlobalState, a)) -> EnvMonad {
        lift(/* do */ {
            let st = get;
            {
                let (global_q, a) = f((globalState(st)));
            };
            put(st, {
                globalState: global_q
            });
            a
        })
    }

    fn mutable(quals: Vec<CTypeQualifier<a>>) -> Rust__id_3a3a4d757461626c65 {
        if(any, (Lambda), quals, then, Rust__id_3a3a496d6d757461626c65, else, Rust__id_3a3a4d757461626c65)
    }

    fn nestedObject(ty: CType, desig: Designator) -> Option {
        match designatorType(desig) {
            IsArray(_, size, el) => {
                Just((From(el, 0, (replicate((-(size, 1)), el)), desig)))
            },
            ty_q => if compatibleInitializer(ty, ty_q) { Just(desig) },
            IsStruct(_, (_, ty_q)(__id_3a, fields)) => {
                nestedObject(ty, (From(ty_q, 0, (map(snd, fields)), desig)))
            },
            _ => {
                Nothing
            },
        }
    }

    fn nextObject(__0: Designator, __1: CurrentObject) -> CurrentObject {
        match (__0, __1) {
            (Base, <todo>) => {
                Nothing
            },
            From(_, i, ty(<todo>, remaining), base) => {
                Just((From(ty, (+(i, 1)), remaining, base)))
            },
            From(_, _, [], base) => {
                nextObject(base)
            },
        }
    }

    fn noTranslation(node: node, msg: String) -> EnvMonad {
        throwE(concat(vec![show((posOf(node))), ": ".to_string(), msg, ":\n".to_string(), render((nest(4, (pretty(node)))))]))
    }

    fn objectFromDesignators(__0: CType, __1: Vec<CDesignator>) -> EnvMonad {
        match (__0, __1) {
            (_, []) => {
                pure(Nothing)
            },
            (ty, desigs) => {
                <$>(Just, go(ty, desigs, (Base(ty))))
            },
        }
    }

    fn promote(node: node, op: fn(Rust__id_3a3a45787072) -> fn(Rust__id_3a3a45787072) -> Rust__id_3a3a45787072, a: Result, b: Result) -> EnvMonad {
        match usual((resultType(a)), (resultType(b))) {
            Some(rt) => {
                Result
            },
            None => {
                badSource(node)(concat(vec!["arithmetic combination for ".to_string(), show((resultType(a))), " and ".to_string(), show((resultType(b)))]))
            },
        }
    }

    fn promotePtr(node: node, op: fn(Rust__id_3a3a45787072) -> fn(Rust__id_3a3a45787072) -> Rust__id_3a3a45787072, a: Result, b: Result) -> EnvMonad {
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

    fn resolveCurrentObject((obj0, prior): (CurrentObject, Initializer), (obj1, cinitial): (CurrentObject, CInit)) -> EnvMonad {
        match mplus(obj1, obj0) {
            None => {
                (Nothing, prior)
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
                                let expr_q = interpretExpr(True, expr);
                                match nestedObject((resultType(expr_q)), obj) {
                                    None => {
                                        badSource(cinitial, "type in initializer".to_string())
                                    },
                                    Some(obj_q) => {
                                        /* do */ {
                                            {
                                                let s = castTo((designatorType(obj_q)), expr_q);
                                            };
                                            (obj_q, scalar(s))
                                        }
                                    },
                                }
                            }
                        },
                    };
                    {
                        let indices = unfoldr((Lambda), obj_q);
                    };
                    {
                        let initializer = foldl((Lambda(Nothing, (IntMap__id_3a3a73696e676c65746f6e(j, a)))), initial, indices);
                    };
                    (nextObject(obj_q), mappend(prior, initializer))
                }
            },
        }
    }

    fn resultToStatements() -> Vec<Rust__id_3a3a53746d74> {
        exprToStatementsresult
    }

    fn runOnce(action: EnvMonad) -> EnvMonad {
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

    fn rustAlignOfType((Rust__id_3a3a547970654e616d65(ty)): Rust__id_3a3a54797065) -> Result {
        Result({
            resultType: IsInt(Unsigned, WordWidth),
            resultMutable: Rust__id_3a3a496d6d757461626c65,
            result: Rust__id_3a3a43616c6c((Rust__id_3a3a566172((Rust__id_3a3a5661724e616d65((__op_addadd("::std::mem::align_of::<".to_string(), __op_addadd(ty, ">".to_string()))))))), vec![])
        })
    }

    fn rustSizeOfType((Rust__id_3a3a547970654e616d65(ty)): Rust__id_3a3a54797065) -> Result {
        Result({
            resultType: IsInt(Unsigned, WordWidth),
            resultMutable: Rust__id_3a3a496d6d757461626c65,
            result: Rust__id_3a3a43616c6c((Rust__id_3a3a566172((Rust__id_3a3a5661724e616d65((__op_addadd("::std::mem::size_of::<".to_string(), __op_addadd(ty, ">".to_string()))))))), vec![])
        })
    }

    fn scalar(expr: Rust__id_3a3a45787072) -> Initializer {
        Initializer((Just(expr)), IntMap__id_3a3a656d707479)
    }

    fn scope(m: EnvMonad) -> EnvMonad {
        /* do */ {
            let old = lift(get);
            let a = m;
            lift((modify((Lambda({
                        globalState: globalState(st)
                    })))));
            a
        }
    }

    fn setBreak(label: Label) -> CSourceBuildCFGT {
        mapBuildCFGT((local((Lambda({
                    onBreak: Just(label)
                })))))
    }

    fn setContinue(label: Label) -> CSourceBuildCFGT {
        mapBuildCFGT((local((Lambda({
                    onContinue: Just(label)
                })))))
    }

    fn statementsToBlock(__0: Vec<Rust__id_3a3a53746d74>) -> Rust__id_3a3a426c6f636b {
        match (__0) {
            [Rust__id_3a3a53746d74(Rust__id_3a3a426c6f636b45787072(stmts))] => {
                stmts
            },
            stmts => {
                Rust__id_3a3a426c6f636b(stmts, Nothing)
            },
        }
    }

    fn toBool(__0: Result) -> Rust__id_3a3a45787072 {
        match (__0) {
            Result(<todo>) => {
                Rust__id_3a3a4c6974((Rust__id_3a3a4c6974426f6f6c(False)))
            },
            Result(<todo>) => {
                Rust__id_3a3a4c6974((Rust__id_3a3a4c6974426f6f6c(True)))
            },
            Result(<todo>) => {
                match t {
                    IsBool => {
                        v
                    },
                    IsPtr(_, _) => {
                        Rust__id_3a3a4e6f74((Rust__id_3a3a4d6574686f6443616c6c(v, (Rust__id_3a3a5661724e616d65("is_null".to_string())), vec![])))
                    },
                    _ => {
                        Rust__id_3a3a436d704e45(v, 0)
                    },
                }
            },
        }
    }

    fn toNotBool(__0: Result) -> Rust__id_3a3a45787072 {
        match (__0) {
            Result(<todo>) => {
                Rust__id_3a3a4c6974((Rust__id_3a3a4c6974426f6f6c(True)))
            },
            Result(<todo>) => {
                Rust__id_3a3a4c6974((Rust__id_3a3a4c6974426f6f6c(False)))
            },
            Result(<todo>) => {
                match t {
                    IsBool => {
                        Rust__id_3a3a4e6f74(v)
                    },
                    IsPtr(_, _) => {
                        Rust__id_3a3a4d6574686f6443616c6c(v, (Rust__id_3a3a5661724e616d65("is_null".to_string())), vec![])
                    },
                    _ => {
                        Rust__id_3a3a436d704551(v, 0)
                    },
                }
            },
        }
    }

    fn toPtr(__0: Result, __1: Option) -> Option {
        match (__0, __1, __2) {
            (ptr, <todo>, Result(<todo>)) => {
                Just(ptr, {
                    resultType: IsPtr(__mut, el),
                    result: castTo((IsPtr(__mut, el)), ptr)
                })
            },
            (ptr, <todo>, Result(<todo>)) => {
                Just(ptr)
            },
            _ => {
                Nothing
            },
        }
    }

    fn toRustRetType(__0: CType) -> Rust__id_3a3a54797065 {
        match (__0) {
            IsVoid => {
                Rust__id_3a3a547970654e616d65("()".to_string())
            },
            ty => {
                toRustType(ty)
            },
        }
    }

    fn toRustType(__0: CType) -> Rust__id_3a3a54797065 {
        match (__0) {
            IsBool => {
                Rust__id_3a3a547970654e616d65("bool".to_string())
            },
            IsInt(s, w) => {
                Rust__id_3a3a547970654e616d65((__op_concat((match s {
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
                Rust__id_3a3a547970654e616d65((__op_concat('f', show(w))))
            },
            IsVoid => {
                Rust__id_3a3a547970654e616d65("::std::os::raw::c_void".to_string())
            },
            IsFunc(retTy, args, variadic) => {
                Rust__id_3a3a547970654e616d65(concat(vec!["unsafe extern fn(".to_string(), args_q, ")".to_string(), /=(if(retTy), __op_addadd(IsVoid(then, " -> ".to_string()), typename(retTy, else, "".to_string())))]))
            },
            IsPtr(__mut, to) => {
                {
                    let Rust__id_3a3a547970654e616d65 = |to_q| {
                        toRustType(to, in, Rust__id_3a3a547970654e616d65, (__op_addadd(rustMut(__mut), to_q)))
                    };
                }
            },
            IsArray(_, size, el) => {
                Rust__id_3a3a547970654e616d65((__op_addadd("[".to_string(), __op_addadd(typename(el), __op_addadd("; ".to_string(), __op_addadd(show(size), "]".to_string()))))))
            },
            IsStruct(name, _fields) => {
                Rust__id_3a3a547970654e616d65(name)
            },
            IsEnum(name) => {
                Rust__id_3a3a547970654e616d65(name)
            },
            IsIncomplete(ident) => {
                Rust__id_3a3a547970654e616d65((identToString(ident)))
            },
        }
    }

    fn translateInitList(ty: CType, list: CInitList) -> EnvMonad {
        /* do */ {
            let objectsAndInitializers = forM(list)(Lambda);
            {
                let base = match ty {
                        IsArray(_, size, el) => {
                            From(el, 0, (replicate((-(size, 1)), el)), (Base(ty)))
                        },
                        IsStruct(_, (_, ty_q)(__id_3a, fields)) => {
                            From(ty_q, 0, (map(snd, fields)), (Base(ty)))
                        },
                        _ => {
                            Base(ty)
                        },
                    };
            };
            let (_, initializer) = foldM(resolveCurrentObject, (Just(base), mempty), objectsAndInitializers);
            initializer
        }
    }

    fn typeName(__0: CDecl, __1: EnvMonad) -> EnvMonad {
        match (__0, __1, __2) {
            (decl, <todo>, CStaticAssert(<todo>)) => {
                badSource(decl, "static assert in type name ".to_string())
            },
            (decl, <todo>, CDecl(spec, declarators, _)) => {
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
                        [(Some(declr, @, CDeclr(None, _, _, _, _)), None, None)] => {
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

    fn typeToResult(itype: IntermediateType, expr: Rust__id_3a3a45787072) -> Result {
        Result({
            resultType: typeRep(itype),
            resultMutable: typeMutable(itype),
            result: expr
        })
    }

    fn unimplemented(node: node) -> EnvMonad {
        noTranslation(node, "Corrode doesn\'t handle this yet".to_string())
    }

    fn uniqueName(base: String) -> EnvMonad {
        modifyGlobal(Lambda)
    }

    fn useForwardRef(ident: Ident) -> EnvMonad {
        modifyGlobal(Lambda)
    }

    fn usual(__0: CType, __1: CType) -> Option {
        match (__0, __1) {
            (IsFloat(aw), IsFloat(bw)) => {
                Just((IsFloat((max(aw, bw)))))
            },
            (a, <todo>, IsFloat(_), _) => {
                Just(a)
            },
            (_, b, <todo>, IsFloat(_)) => {
                Just(b)
            },
            (origA, origB) => {
                match (intPromote(origA), intPromote(origB)) {
                    (a, b) => if (a == b) { Just(a) },
                    (IsInt(Signed, sw), IsInt(Unsigned, uw)) => {
                        mixedSign(sw, uw)
                    },
                    (IsInt(Unsigned, uw), IsInt(Signed, sw)) => {
                        mixedSign(sw, uw)
                    },
                    (IsInt(as, aw), IsInt(_bs, bw)) => {
                        /* do */ {
                            let rank = integerConversionRank(aw, bw);
                            Just((IsInt(as, ((if(rank) == GT(then, aw, else, bw))))))
                        }
                    },
                    _ => {
                        Nothing
                    },
                }
            },
        }
    }

    fn wrapMain(declr: CDeclr, realName: String, argTypes: Vec<CType>) -> EnvMonad {
        /* do */ {
            let (setup, args) = wrapArgv(argTypes);
            {
                let ret = Rust__id_3a3a5661724e616d65("ret".to_string());
            };
            emitItems(vec![Rust__id_3a3a4974656d(vec![], Rust__id_3a3a50726976617465, (Rust__id_3a3a46756e6374696f6e(vec![], "main".to_string(), vec![], (Rust__id_3a3a547970654e616d65("()".to_string())), (statementsToBlock((__op_addadd(setup, __op_addadd(vec![bind(Rust__id_3a3a496d6d757461626c65, ret)(Rust__id_3a3a556e7361666545787072(Rust__id_3a3a426c6f636b(vec![])(Just(call(realName, args)))))], exprToStatements((call("::std::process::exit".to_string(), vec![Rust__id_3a3a566172(ret)])))))))))))]);
;
            let wrapArgv = |vec![]| {
                (vec![], vec![])
            };
;
;
            let wrapArgv = |_| {
                unimplemented(declr)
            };
;
            let wrapEnvp = |vec![]| {
                (vec![], vec![])
            };
;
;
            let wrapEnvp = |_| {
                unimplemented(declr)
            };

        }
    }

    fn wrapping(__0: Result, __1: Result) -> Result {
        match (__0, __1, __2) {
            (r, <todo>, Result(<todo>)) => {
                match result(r) {
                    Rust__id_3a3a416464(lhs, rhs) => {
                        r({
                            result: Rust__id_3a3a4d6574686f6443616c6c(lhs, (Rust__id_3a3a5661724e616d65("wrapping_add".to_string())), vec![rhs])
                        })
                    },
                    Rust__id_3a3a537562(lhs, rhs) => {
                        r({
                            result: Rust__id_3a3a4d6574686f6443616c6c(lhs, (Rust__id_3a3a5661724e616d65("wrapping_sub".to_string())), vec![rhs])
                        })
                    },
                    Rust__id_3a3a4d756c(lhs, rhs) => {
                        r({
                            result: Rust__id_3a3a4d6574686f6443616c6c(lhs, (Rust__id_3a3a5661724e616d65("wrapping_mul".to_string())), vec![rhs])
                        })
                    },
                    Rust__id_3a3a446976(lhs, rhs) => {
                        r({
                            result: Rust__id_3a3a4d6574686f6443616c6c(lhs, (Rust__id_3a3a5661724e616d65("wrapping_div".to_string())), vec![rhs])
                        })
                    },
                    Rust__id_3a3a4d6f64(lhs, rhs) => {
                        r({
                            result: Rust__id_3a3a4d6574686f6443616c6c(lhs, (Rust__id_3a3a5661724e616d65("wrapping_rem".to_string())), vec![rhs])
                        })
                    },
                    Rust__id_3a3a4e6567(e) => {
                        r({
                            result: Rust__id_3a3a4d6574686f6443616c6c(e, (Rust__id_3a3a5661724e616d65("wrapping_neg".to_string())), vec![])
                        })
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

}

mod Language_Rust_Corrode_CFG {
    struct BasicBlock<s, c>(BasicBlock, s, Terminator<c>);

    #[derive(Debug)]
    enum Terminator_q<c, l> {
        Unreachable,
        Branch(l),
        CondBranch(c, l, l)
    }

    struct Unordered;

    struct DepthFirst;

    struct CFG<k, s, c>(CFG, Label, IntMap__id_3a3a496e744d6170<BasicBlock<s, c>>);

    struct BuildState<s, c>(BuildState, { /* struct def */ });

    #[derive(Debug)]
    enum StructureLabel<s, c> {
        GoTo({ /* struct def */ }),
        ExitTo({ /* struct def */ }),
        Nested(Vec<Structure<s, c>>)
    }

    #[derive(Debug)]
    enum Structure_q<s, c, a> {
        Simple(s, StructureTerminator<s, c>),
        Loop(a),
        Multiple(IntMap__id_3a3a496e744d6170<a>, a)
    }

    #[derive(Debug)]
    struct Structure<s, c>(Structure, { /* struct def */ });

    fn addBlock(label: Monad) -> Monad {
        /* do */ {
            modify(Lambda({
                buildBlocks: IntMap__id_3a3a696e73657274(label, (BasicBlock(stmt, terminator)), (buildBlocks(st)))
            }))
        }
    }

    fn buildCFG(root: Monad) -> Monad {
        /* do */ {
            let (label, final) = runStateT(root, (BuildState(0, IntMap__id_3a3a656d707479)));
            (CFG(label, (buildBlocks(final))))
        }
    }

    fn depthFirstOrder((CFG(start, blocks)): CFG) -> CFG {
        CFG(start_q, blocks_q)
    }

    fn flipEdges(edges: IntMap__id_3a3a496e744d6170) -> IntMap__id_3a3a496e744d6170 {
        IntMap__id_3a3a756e696f6e7357697468(IntSet__id_3a3a756e696f6e, Dummy)
    }

    fn hasMultiple() -> Bool {
        any((gostructureBody))
    }

    fn mapBuildCFGT() -> BuildCFGT {
        mapStateT
    }

    fn newLabel() -> Monad {
        /* do */ {
            let old = get;
            put(old, {
                buildLabel: +(buildLabel(old), 1)
            });
            (buildLabel(old))
        }
    }

    fn outEdges(blocks: IntMap__id_3a3a496e744d6170) -> IntMap__id_3a3a496e744d6170 {
        IntSet.difference(IntSet__id_3a3a756e696f6e73((map(successors)(IntMap__id_3a3a656c656d73(blocks)))), IntMap__id_3a3a6b657973536574(blocks))
    }

    fn partitionMembers(a: IntSet__id_3a3a496e74536574, b: IntSet__id_3a3a496e74536574) -> (IntSet__id_3a3a496e74536574, IntSet__id_3a3a496e74536574) {
        (IntSet.intersection(a, b), IntSet.difference(a, b))
    }

    fn prettyCFG(fmtS: fn(s) -> Doc, fmtC: fn(c) -> Doc, (CFG(entry, blocks)): CFG) -> CFG {
        vcat(__op_concat((<>(text("start @".to_string()), text((show(entry))))), blocks_q))
    }

    fn prettyStructure() -> Doc {
        vcatmap(go)
    }

    fn relooper(entries: Monoid) -> Monoid {
        {
            let (returns, noreturns) = partitionMembers(entries)(IntSet__id_3a3a756e696f6e73(map(successors)(IntMap__id_3a3a656c656d73(blocks))));
;
            let (present, absent) = partitionMembers(entries, (IntMap__id_3a3a6b657973536574(blocks)));
        }(in, match (IntSet__id_3a3a746f4c697374(noreturns), IntSet__id_3a3a746f4c697374(returns)) {
                ([], []) => {
                    vec![]
                },
                ([entry], []) => {
                    match IntMap__id_3a3a7570646174654c6f6f6b7570576974684b6579((Lambda), entry, blocks) {
                        (Some((s, term)), blocks_q) => {
                            __op_concat(Structure({
                                structureEntries: entries,
                                structureBody: Simple(s, term)
                            }), relooper((successors((s, term))), blocks_q))
                        },
                        (None, _) => {
                            __op_concat(Structure({
                                structureEntries: entries,
                                structureBody: Simple(mempty, (Branch((GoTo(entry)))))
                            }), vec![])
                        },
                    }
                },
                _ => if not((IntSet__id_3a3a6e756c6c(absent))) { __op_concat(if(IntSet__id_3a3a6e756c6c, present, then, vec![], else, Structure, {
                structureEntries: entries,
                structureBody: Multiple((IntMap__id_3a3a66726f6d536574((const(vec![])), absent)), (relooper(present, blocks)))
            }), vec![]) },
                ([], _) => {
                    __op_concat(Structure({
                        structureEntries: entries,
                        structureBody: Loop((relooper(entries, blocks_q)))
                    }), relooper(followEntries, followBlocks))
                },
                _ => {
                    __op_concat(Structure({
                        structureEntries: entries,
                        structureBody: Multiple(handlers, unhandled)
                    }), relooper(followEntries, followBlocks))
                },
            })
    }

    fn relooperRoot((CFG(entry, blocks)): Monoid) -> Monoid {
        relooper((IntSet__id_3a3a73696e676c65746f6e(entry)))(IntMap__id_3a3a6d6170((Lambda), blocks))
    }

    fn removeEmptyBlocks((CFG(start, blocks)): Foldable) -> Foldable {
        CFG((rewrite(start)), blocks_q)
    }

    fn restrictKeys(m: IntMap__id_3a3a496e744d6170) -> IntMap__id_3a3a496e744d6170 {
        IntMap.intersection(m, IntMap__id_3a3a66726f6d536574((const(())), s))
    }

    fn simplifyStructure() -> Monoid {
        foldr(go, vec![])map(descend)
    }

    fn structureCFG(mkBreak: Monoid) -> Monoid {
        (hasMultiple(root), foo(vec![], mempty, root))
    }

    fn successors((_, term): StructureBlock) -> StructureBlock {
        IntSet__id_3a3a66726f6d4c697374(Dummy)
    }

}

mod Language_Rust_Corrode_CrateMap {
    #[derive(Debug, Eq, Ord)]
    enum ItemKind {
        Enum,
        Struct,
        Union,
        Type,
        Symbol
    }

    fn mergeCrateMaps() -> Map__id_3a3a4d6170 {
        Map__id_3a3a66726f6d4c69737457697468((Map__id_3a3a756e696f6e57697468((Operator("++")))))
    }

    fn parseCrateMap() -> Either {
        fmap(root)foldrM(parseLine, (Map__id_3a3a656d707479, vec![]))filter((notnull))map(cleanLine)lines
    }

    fn rewritesFromCratesMap(crates: CratesMap) -> ItemRewrites {
        Map__id_3a3a66726f6d4c697374(Dummy)
    }

    fn splitModuleMap(modName: String, crates: CratesMap) -> (ModuleMap, CratesMap) {
        fromMaybe((vec![], crates))(/* do */ {
            let thisCrate = Map__id_3a3a6c6f6f6b7570("".to_string(), crates);
            let thisModule = Map__id_3a3a6c6f6f6b7570(modName, thisCrate);
            {
                let thisCrate_q = Map__id_3a3a64656c657465(modName, thisCrate);
            };
            {
                let crates_q = Map__id_3a3a696e73657274("".to_string(), thisCrate_q, crates);
            };
            (thisModule, crates_q)
        })
    }

}

mod Language_Rust_Idiomatic {
    fn itemIdioms(__0: Rust__id_3a3a4974656d) -> Rust__id_3a3a4974656d {
        match (__0) {
            Rust__id_3a3a4974656d(attrs, vis, Rust__id_3a3a46756e6374696f6e(fattrs, name, formals, ret, b)) => {
                Rust__id_3a3a4974656d(attrs, vis, (Rust__id_3a3a46756e6374696f6e(fattrs, name, formals, ret, (tailBlock(b)))))
            },
            i => {
                i
            },
        }
    }

    fn tailBlock(__0: Rust__id_3a3a426c6f636b) -> Rust__id_3a3a426c6f636b {
        match (__0) {
            Rust__id_3a3a426c6f636b(b, Some(<todo>)) => {
                Rust__id_3a3a426c6f636b(b, e)
            },
            Rust__id_3a3a426c6f636b(<todo>, None) => {
                Rust__id_3a3a426c6f636b(b, e)
            },
            b => {
                b
            },
        }
    }

    fn tailExpr(__0: Rust__id_3a3a45787072) -> Option {
        match (__0) {
            Rust__id_3a3a52657475726e(e) => {
                Just(e)
            },
            Rust__id_3a3a426c6f636b45787072(b) => {
                Just((Just((Rust__id_3a3a426c6f636b45787072((tailBlock(b)))))))
            },
            Rust__id_3a3a49665468656e456c7365(c, t, f) => {
                Just((Just((Rust__id_3a3a49665468656e456c7365(c, (tailBlock(t)), (tailBlock(f)))))))
            },
            _ => {
                Nothing
            },
        }
    }

    fn unsnoc(__0: Vec<a>) -> Option {
        match (__0) {
            [] => {
                Nothing
            },
            x__id_3a7873 => {
                match unsnoc(xs) {
                    Some((a, b)) => {
                        Just((x__id_3a61, b))
                    },
                    None => {
                        Just((vec![], x))
                    },
                }
            },
        }
    }

}

mod Language_Rust {

}



fn main() { /* demo */ }
