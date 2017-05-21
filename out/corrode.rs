mod Language_Rust_AST {
    #[derive(Debug, Eq)]
    enum LitIntRepr{
        DecRepr,
        OctalRepr,
        HexRepr
    };

    #[derive(Debug, Eq)]
    enum Lit{
        LitByteStr(String),
        LitByteChar(Char),
        LitBool(Bool),
        LitInt(Integer, LitIntRepr, Type),
        LitFloat(String, Type)
    };

    #[derive(Debug, Eq)]
    enum Visibility{
        Public,
        Private
    };

    #[derive(Debug, Eq)]
    enum Mutable{
        Immutable,
        Mutable
    };

    #[derive(Debug)]
    enum Stmt{
        Stmt(Expr),
        Let(Mutable, Var, Maybe(Type), Maybe(Expr)),
        StmtItem(Vec<Attribute>, ItemKind)
    };

    #[derive(Debug)]
    struct Block(Block, Vec<Stmt>, Maybe(Expr));

    #[derive(Debug)]
    struct Attribute(Attribute, String);

    #[derive(Debug)]
    struct Item(Item, Vec<Attribute>, Visibility, ItemKind);

    #[derive(Debug)]
    enum FunctionAttribute{
        UnsafeFn,
        ExternABI(Maybe(String))
    };

    #[derive(Debug)]
    enum ItemKind{
        Function(Vec<FunctionAttribute>, String, Vec<(Mutable, Var, Type)>, Type, Block),
        Static(Mutable, Var, Type, Expr),
        Struct(String, Vec<(String, Type)>),
        Extern(Vec<ExternItem>),
        Use(String),
        Enum(String, Vec<Enumerator>),
        CloneImpl(Type)
    };

    #[derive(Debug)]
    enum ExternItem{
        ExternFn(String, Vec<(Var, Type)>, Bool, Type),
        ExternStatic(Mutable, Var, Type)
    };

    #[derive(Debug)]
    enum Enumerator{
        EnumeratorAuto(String),
        EnumeratorExpr(String, Expr)
    };

    #[derive(Debug)]
    enum Expr{
        Lit(Lit),
        Var(Var),
        Path(Path),
        Index(Expr, Expr),
        ArrayExpr(Vec<Expr>),
        RepeatArray(Expr, Expr),
        StructExpr(String, Vec<(String, Expr)>, Maybe(Expr)),
        Call(Expr, Vec<Expr>),
        MethodCall(Expr, Var, Vec<Expr>),
        Lambda(Vec<Var>, Expr),
        Member(Expr, Var),
        BlockExpr(Block),
        UnsafeExpr(Block),
        IfThenElse(Expr, Block, Block),
        Loop(Maybe(Lifetime), Block),
        While(Maybe(Lifetime), Expr, Block),
        For(Maybe(Lifetime), Var, Expr, Block),
        Break(Maybe(Lifetime)),
        Continue(Maybe(Lifetime)),
        Return(Maybe(Expr)),
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
    };

    #[derive(Debug)]
    enum AssignOp{
        :=,
        :+=,
        :-=,
        :*=,
        :/=,
        :%=,
        :&=,
        :|=,
        :^=,
        :<<=,
        :>>=
    };

    #[derive(Debug)]
    enum ExprPosition{
        TopExpr,
        LeftExpr,
        RightExpr
    };

        fn pPrintBlock(__0: Doc) -> Doc {
        match (__0, __1) {
            <todo> => { sep(vec![<+>(pre, text("{".to_string())), nest(4, (maybe(empty, pPrint, e))), text("}".to_string())]) },
            <todo> => { <+>(pre, $+$(text("{".to_string()), $+$(nest(4, (vcat((++(map(pPrint, ss), vec![maybe(empty, pPrint, e)]))))), text("}".to_string())))) },
        }
    }

}

mod Language_Rust_Corrode_C {
    struct FunctionContext(FunctionContext, { /* struct def */ });

    struct Output(Output, { /* struct def */ });

    struct GlobalState(GlobalState, { /* struct def */ });

    struct EnvState(EnvState, { /* struct def */ });

    struct Initializer(Initializer, Maybe(Rust.Expr), IntMap.IntMap(Initializer));

    #[derive(Debug)]
    enum Designator{
        Base(CType),
        From(CType, isize, Vec<CType>, Designator)
    };

    struct OuterLabels(OuterLabels, { /* struct def */ });

    struct Result(Result, { /* struct def */ });

    #[derive(Debug, Eq)]
    enum Signed{
        Signed,
        Unsigned
    };

    #[derive(Debug, Eq)]
    enum IntWidth{
        BitWidth(isize),
        WordWidth
    };

    #[derive(Debug)]
    enum CType{
        IsBool,
        IsInt(Signed, IntWidth),
        IsFloat(isize),
        IsVoid,
        IsFunc(CType, Vec<(Maybe((Rust.Mutable, Ident)), CType)>, Bool),
        IsPtr(Rust.Mutable, CType),
        IsArray(Rust.Mutable, isize, CType),
        IsStruct(String, Vec<(String, CType)>),
        IsEnum(String),
        IsIncomplete(Ident)
    };

    struct IntermediateType(IntermediateType, { /* struct def */ });

        fn addExternIdent(ident: EnvMonad) -> EnvMonad {
        {

                let action = runOnce({

                let itype = deferred;
                let rewrites = lift(asks(itemRewrites));
                let path = match Map_lookup((Symbol, identToString(ident)), rewrites) {
            Just, renamed => { return((:("".to_string(), renamed))) },
            Nothing => { {

                    Let([Assign([Span([Ref(Ident("name"))])], Span([Ref(Ident("applyRenames")), Ref(Ident("ident"))]))], []);
                    Let([Assign([Span([Ref(Ident("ty"))])], Span([Parens([Span([Ref(Ident("typeMutable")), Ref(Ident("itype"))]), Span([Ref(Ident("typeRep")), Ref(Ident("itype"))])])]))], []);
                    lift(tell(mempty, {
        outputExterns: Map_singleton(name, (mkItem(name, ty)))
        }));
                    return(vec![name])
            } },
        };
                return((typeToResult(itype, (Rust_Path((Rust_PathSegments(path)))))))
        });
                addSymbolIdentAction(ident, action)
        }
    }

    fn addSwitchCase(condition: Maybe) -> Maybe {
        {

                let condition_q = lift(lift(mapM((interpretExpr(True)), condition)));
                let next_q = interpretStatement(body, next);
                let label = match next_q {
            ([], Branch(to)) => { return(to) },
            (rest, end) => { {

                    let label = newLabel;
                    addBlock(label, rest, end);
                    return(label)
            } },
        };
                lift(tell(SwitchCases(IntMap_singleton(label, condition_q))));
                return((vec![], Branch(label)))
        }
    }

    fn addSymbolIdent(ident: EnvMonad) -> EnvMonad {
        {

                Let([Assign([Span([Ref(Ident("name"))])], Span([Ref(Ident("applyRenames")), Ref(Ident("ident"))]))], []);
                addSymbolIdentAction(ident)(return(Result, {
        resultType: ty,
        resultMutable: mut,
        result: Rust_Path((Rust_PathSegments(vec![name])))
        }));
                return(name)
        }
    }

    fn addSymbolIdentAction(ident: EnvMonad) -> EnvMonad {
        lift({

                    modify(Lambda({
        symbolEnvironment: :((ident, action), symbolEnvironment(st))
        }))
            })
    }

    fn addTagIdent(ident: EnvMonad) -> EnvMonad {
        lift({

                    modify(Lambda({
        tagEnvironment: :((ident, ty), tagEnvironment(st))
        }))
            })
    }

    fn addTypedefIdent(ident: EnvMonad) -> EnvMonad {
        lift({

                    modify(Lambda({
        typedefEnvironment: :((ident, ty), typedefEnvironment(st))
        }))
            })
    }

    fn applyRenames(ident: String) -> String {
        match identToString(ident) {
                "final" => { "final_".to_string() },
                "fn" => { "fn_".to_string() },
                "in" => { "in_".to_string() },
                "let" => { "let_".to_string() },
                "main" => { "_c_main".to_string() },
                "match" => { "match_".to_string() },
                "mod" => { "mod_".to_string() },
                "proc" => { "proc_".to_string() },
                "type" => { "type_".to_string() },
                "where" => { "where_".to_string() },
                name => { name },
            }
    }

    fn badSource(node: EnvMonad) -> EnvMonad {
        noTranslation(node, (++("illegal ".to_string(), ++(msg, "; check whether a real C compiler accepts this".to_string()))))
    }

    fn baseTypeOf(specs: EnvMonad) -> EnvMonad {
        {

                Let([Assign([Span([Parens([Span([Ref(Ident("storage"))]), Span([Ref(Ident("_attributes"))]), Span([Ref(Ident("basequals"))]), Span([Ref(Ident("basespecs"))]), Span([Ref(Ident("_inlineNoReturn"))]), Span([Ref(Ident("_align"))])])])], Span([Ref(Ident("partitionDeclSpecs")), Ref(Ident("specs"))]))], []);
                let mstorage = match storage {
            [] => { return(Nothing) },
            [spec] => { return((Just(spec))) },
            _, :, excess, :, _ => { badSource(excess, "extra storage class specifier".to_string()) },
        };
                let base = typedef((mutable(basequals)), basespecs);
                return((mstorage, base));
            
        }
    }

    fn binop(expr: EnvMonad) -> EnvMonad {
        fmap(wrapping)(match op {
                    CMulOp => { promote(expr, Rust_Mul, lhs, rhs) },
                    CDivOp => { promote(expr, Rust_Div, lhs, rhs) },
                    CRmdOp => { promote(expr, Rust_Mod, lhs, rhs) },
                    CAddOp => { match (toPtr(lhs), toPtr(rhs)) {
                            (Just(ptr), _) => { return((offset(ptr, rhs))) },
                            (_, Just(ptr)) => { return((offset(ptr, lhs))) },
                            _ => { promote(expr, Rust_Add, lhs, rhs) },
                        } },
                    CSubOp => { match (toPtr(lhs), toPtr(rhs)) {
                            (Just(lhs_q), Just(rhs_q)) => { {

                                    let ptrTo = match compatiblePtr((resultType(lhs_q)), (resultType(rhs_q))) {
            IsPtr, _, ptrTo => { return(ptrTo) },
            _ => { badSource(expr, "pointer subtraction of incompatible pointers".to_string()) },
        };
                                    Let([Assign([Span([Ref(Ident("ty"))])], Span([Ref(Ident("IsInt")), Ref(Ident("Signed")), Ref(Ident("WordWidth"))]))], []);
                                    Let([Assign([Span([Ref(Ident("size"))])], Span([Ref(Ident("rustSizeOfType")), Parens([Span([Ref(Ident("toRustType")), Ref(Ident("ptrTo"))])])]))], []);
                                    return(Result, {
    resultType: ty,
    resultMutable: Rust_Immutable,
    result: /((Rust_MethodCall((castTo(ty, lhs_q)), (Rust_VarName("wrapping_sub".to_string())), vec![castTo(ty, rhs_q)])), castTo(ty, size))
    })
                            } },
                            (Just(ptr), _) => { return(ptr, {
                                result: Rust_MethodCall((result(ptr)), (Rust_VarName("offset".to_string())), vec![Rust_Neg((castTo((IsInt(Signed, WordWidth)), rhs)))])
                                }) },
                            _ => { promote(expr, Rust_Sub, lhs, rhs) },
                        } },
                    CShlOp => { shift(Rust_ShiftL) },
                    CShrOp => { shift(Rust_ShiftR) },
                    CLeOp => { comparison(Rust_CmpLT) },
                    CGrOp => { comparison(Rust_CmpGT) },
                    CLeqOp => { comparison(Rust_CmpLE) },
                    CGeqOp => { comparison(Rust_CmpGE) },
                    CEqOp => { comparison(Rust_CmpEQ) },
                    CNeqOp => { comparison(Rust_CmpNE) },
                    CAndOp => { promote(expr, Rust_And, lhs, rhs) },
                    CXorOp => { promote(expr, Rust_Xor, lhs, rhs) },
                    COrOp => { promote(expr, Rust_Or, lhs, rhs) },
                    CLndOp => { return(Result, {
                        resultType: IsBool,
                        resultMutable: Rust_Immutable,
                        result: Rust_LAnd((toBool(lhs)), (toBool(rhs)))
                        }) },
                    CLorOp => { return(Result, {
                        resultType: IsBool,
                        resultMutable: Rust_Immutable,
                        result: Rust_LOr((toBool(lhs)), (toBool(rhs)))
                        }) },
                })
    }

    fn bitWidth(__0: isize) -> isize {
        match (__0, __1) {
            <todo> => { wordWidth },
            <todo> => { w },
        }
    }

    fn blockToStatements((Rust_Block(stmts, mexpr)): Vec<Rust.Stmt>) -> Vec<Rust.Stmt> {
        match mexpr {
                Just, expr => { ++(stmts, exprToStatements(expr)) },
                Nothing => { stmts },
            }
    }

    fn castTo(__0: Rust.Expr) -> Rust.Expr {
        match (__0, __1) {
            <todo> => { castTo(target, Result, {
                resultType: IsPtr(mut, el),
                resultMutable: Rust_Immutable,
                result: Rust_MethodCall(source, (Rust_VarName(method)), vec![])
                }) },
            <todo> => { toBool(source) },
            <todo> => { Rust_Lit((Rust_LitInt(n, repr, (toRustType(target))))) },
            <todo> => { Rust_Neg((Rust_Lit((Rust_LitInt(n, repr, (toRustType((IsInt(Signed, w))))))))) },
            <todo> => { Rust_Cast((result(source)), (toRustType(target))) },
        }
    }

    fn cfgToRust(_node: CSourceBuildCFGT) -> CSourceBuildCFGT {
        {

                Let([Assign([Span([Ref(Ident("builder"))])], Span([Ref(Ident("buildCFG")), Operator("$"), Do([Expression(Span([Parens([Span([Ref(Ident("early"))]), Span([Ref(Ident("term"))])]), Operator("<-"), Ref(Ident("build"))]), []), Expression(Span([Ref(Ident("entry")), Operator("<-"), Ref(Ident("newLabel"))]), []), Expression(Span([Ref(Ident("addBlock")), Ref(Ident("entry")), Ref(Ident("early")), Ref(Ident("term"))]), []), Expression(Span([Ref(Ident("return")), Ref(Ident("entry"))]), [])], [])]))], []);
                let (rawCFG, _) = evalRWST(builder, (OuterLabels(Nothing, Nothing, Nothing)), Map_empty);
                Let([Assign([Span([Ref(Ident("cfg"))])], Span([Ref(Ident("depthFirstOrder")), Parens([Span([Ref(Ident("removeEmptyBlocks")), Ref(Ident("rawCFG"))])])]))], []);
                Let([Assign([Span([Parens([Span([Ref(Ident("hasGoto"))]), Span([Ref(Ident("structured"))])])])], Span([Ref(Ident("structureCFG")), Ref(Ident("mkBreak")), Ref(Ident("mkContinue")), Ref(Ident("mkLoop")), Ref(Ident("mkIf")), Ref(Ident("mkGoto")), Ref(Ident("mkMatch")), Ref(Ident("cfg"))]))], []);
                return(:(if(hasGoto, then, declCurrent), structured(else, structured)));
            
        }
    }

    fn charType() -> CType {
        IsInt(Unsigned, (BitWidth(8)))
    }

    fn compatibleInitializer(__0: Bool) -> Bool {
        match (__0, __1) {
            <todo> => { (name1 == name2) },
            <todo> => { False },
            <todo> => { False },
            <todo> => { True },
        }
    }

    fn compatiblePtr(__0: CType) -> CType {
        match (__0, __1) {
            <todo> => { b },
            <todo> => { compatiblePtr((IsPtr(mut, el)), b) },
            <todo> => { a },
            <todo> => { compatiblePtr(a, (IsPtr(mut, el))) },
            <todo> => { IsPtr((leastMutable(m1, m2)), (compatiblePtr(a, b))) },
            <todo> => { IsVoid },
        }
    }

    fn completeType(__0: EnvMonad) -> EnvMonad {
        match (__0, __1, __2) {
            <todo> => { {

                    let mty = getTagIdent(ident);
                    fromMaybe((return(orig)), mty)
            } },
            <todo> => { return(ty) },
        }
    }

    fn compound(expr: EnvMonad) -> EnvMonad {
        {

                Let([Assign([Span([Ref(Ident("op\'"))])], Span([Case(Span([Ref(Ident("op"))]), [Direct([Ref(Ident("CAssignOp"))], [Span([Ref(Ident("Nothing"))])]), Direct([Ref(Ident("CMulAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CMulOp"))])]), Direct([Ref(Ident("CDivAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CDivOp"))])]), Direct([Ref(Ident("CRmdAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CRmdOp"))])]), Direct([Ref(Ident("CAddAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CAddOp"))])]), Direct([Ref(Ident("CSubAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CSubOp"))])]), Direct([Ref(Ident("CShlAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CShlOp"))])]), Direct([Ref(Ident("CShrAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CShrOp"))])]), Direct([Ref(Ident("CAndAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CAndOp"))])]), Direct([Ref(Ident("CXorAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CXorOp"))])]), Direct([Ref(Ident("COrAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("COrOp"))])])])]))], []);
                Let([Assign([Span([Ref(Ident("duplicateLHS"))])], Span([Ref(Ident("isJust")), Ref(Ident("op\'")), Operator("||"), Ref(Ident("demand"))]))], []);
                Let([Assign([Span([Parens([Span([Ref(Ident("bindings1"))]), Span([Ref(Ident("dereflhs"))]), Span([Ref(Ident("boundrhs"))])])])], Span([Ref(Ident("if")), Ref(Ident("not")), Ref(Ident("duplicateLHS")), Operator("||"), Ref(Ident("hasNoSideEffects")), Parens([Span([Ref(Ident("result")), Ref(Ident("lhs"))])]), Ref(Ident("then")), Parens([Span([Vector([])]), Span([Ref(Ident("lhs"))]), Span([Ref(Ident("rhs"))])]), Ref(Ident("else")), Let([Assign([Span([Ref(Ident("lhsvar"))])], Span([Ref(Ident("Rust.VarName")), Str("_lhs")])), Assign([Span([Ref(Ident("rhsvar"))])], Span([Ref(Ident("Rust.VarName")), Str("_rhs")]))], []), Ref(Ident("in")), Parens([Span([Vector([Span([Ref(Ident("Rust.Let")), Ref(Ident("Rust.Immutable")), Ref(Ident("rhsvar")), Ref(Ident("Nothing")), Parens([Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("result")), Ref(Ident("rhs"))])])])])]), Span([Ref(Ident("Rust.Let")), Ref(Ident("Rust.Immutable")), Ref(Ident("lhsvar")), Ref(Ident("Nothing")), Parens([Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("Rust.Borrow")), Ref(Ident("Rust.Mutable")), Parens([Span([Ref(Ident("result")), Ref(Ident("lhs"))])])])])])])])])]), Span([Ref(Ident("lhs")), Record([(Ident("result"), Span([Ref(Ident("Rust.Deref")), Parens([Span([Ref(Ident("Rust.Var")), Ref(Ident("lhsvar"))])])]))])]), Span([Ref(Ident("rhs")), Record([(Ident("result"), Span([Ref(Ident("Rust.Var")), Ref(Ident("rhsvar"))]))])])])]))], []);
                let rhs_q = match op_q {
            Just, o => { binop(expr, o, dereflhs, boundrhs) },
            Nothing => { return(boundrhs) },
        };
                Let([Assign([Span([Ref(Ident("assignment"))])], Span([Ref(Ident("Rust.Assign")), Parens([Span([Ref(Ident("result")), Ref(Ident("dereflhs"))])]), Parens([Span([Ref(Ident("Rust.:="))])]), Parens([Span([Ref(Ident("castTo")), Parens([Span([Ref(Ident("resultType")), Ref(Ident("lhs"))])]), Ref(Ident("rhs\'"))])])]))], []);
                Let([Assign([Span([Parens([Span([Ref(Ident("bindings2"))]), Span([Ref(Ident("ret"))])])])], Span([Ref(Ident("if")), Ref(Ident("not")), Ref(Ident("demand")), Ref(Ident("then")), Parens([Span([Vector([])]), Span([Ref(Ident("Nothing"))])]), Ref(Ident("else")), Ref(Ident("if")), Ref(Ident("not")), Ref(Ident("returnOld")), Ref(Ident("then")), Parens([Span([Vector([])]), Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("result")), Ref(Ident("dereflhs"))])])])]), Ref(Ident("else")), Let([Assign([Span([Ref(Ident("oldvar"))])], Span([Ref(Ident("Rust.VarName")), Str("_old")]))], []), Ref(Ident("in")), Parens([Span([Vector([Span([Ref(Ident("Rust.Let")), Ref(Ident("Rust.Immutable")), Ref(Ident("oldvar")), Ref(Ident("Nothing")), Parens([Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("result")), Ref(Ident("dereflhs"))])])])])])])]), Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("Rust.Var")), Ref(Ident("oldvar"))])])])])]))], []);
                return(match Rust_Block((++(bindings1, ++(bindings2, exprToStatements(assignment)))), ret) {
            b, @, Rust_Block(body, Nothing) => { Result({
                resultType: IsVoid,
                resultMutable: Rust_Immutable,
                result: match body {
                                [Rust_Stmt(e)] => { e },
                                _ => { Rust_BlockExpr(b) },
                            }
                }) },
            b => { lhs({
                result: Rust_BlockExpr(b)
                }) },
        });
            
        }
    }

    fn derivedDeferredTypeOf(deferred: EnvMonad) -> EnvMonad {
        {

                let derived_q = mapM(derive, derived);
                return({

            let basetype = deferred;
            foldrM((Operator("$")), basetype, derived_q)
    });
            
        }
    }

    fn derivedTypeOf(deferred: EnvMonad) -> EnvMonad {
        join((derivedDeferredTypeOf(deferred, declr, vec![])))
    }

    fn designatorType(__0: CType) -> CType {
        match (__0) {
            <todo> => { ty },
            <todo> => { ty },
        }
    }

    fn emitIncomplete(kind: EnvMonad) -> EnvMonad {
        {

                let rewrites = lift((asks(itemRewrites)));
                unless((Map_member((kind, identToString(ident)), rewrites)))(lift(tell(mempty, {
            outputIncomplete: Set_singleton((identToString(ident)))
            })));
                return((IsIncomplete(ident)))
        }
    }

    fn emitItems(items: EnvMonad) -> EnvMonad {
        lift(tell(mempty, {
                outputItems: items
                }))
    }

    fn enumReprType() -> CType {
        IsInt(Signed, (BitWidth(32)))
    }

    fn exprToStatements(__0: Vec<Rust.Stmt>) -> Vec<Rust.Stmt> {
        match (__0) {
            <todo> => { vec![Rust_Stmt((Rust_IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))] },
            <todo> => { blockToStatements(b) },
            <todo> => { vec![Rust_Stmt(e)] },
        }
    }

    fn getSwitchCases(expr: CSourceBuildCFGT) -> CSourceBuildCFGT {
        mapBuildCFGT(wrap)
    }

    fn getSwitchExpression(stmt: CSourceBuildCFGT) -> CSourceBuildCFGT {
        {

                let mexpr = lift(asks(switchExpression));
                match mexpr {
        Nothing => { lift(lift(badSource(stmt, "case outside switch".to_string()))) },
        Just, expr => { return(expr) },
    }
        }
    }

    fn getSymbolIdent(ident: EnvMonad) -> EnvMonad {
        {

                let env = lift(get);
                match lookup(ident, (symbolEnvironment(env))) {
        Just, symbol => { fmap(Just, symbol) },
        Nothing => { match identToString(ident) {
                "__func__" => { getFunctionName("".to_string()) },
                "__FUNCTION__" => { getFunctionName("".to_string()) },
                "__PRETTY_FUNCTION__" => { getFunctionName("top level".to_string()) },
                name => { return(lookup(name, builtinSymbols)) },
            } },
    };
            
        }
    }

    fn getTagIdent(ident: EnvMonad) -> EnvMonad {
        lift({

                    let env = gets(tagEnvironment);
                    return(lookup(ident, env))
            })
    }

    fn getTypedefIdent(ident: EnvMonad) -> EnvMonad {
        lift({

                    let env = gets(typedefEnvironment);
                    return((identToString(ident), lookup(ident, env)))
            })
    }

    fn gotoLabel(ident: CSourceBuildCFGT) -> CSourceBuildCFGT {
        {

                let labels = lift(get);
                match Map_lookup(ident, labels) {
        Nothing => { {

                let label = newLabel;
                lift((put((Map_insert(ident, label, labels)))));
                return(label)
        } },
        Just, label => { return(label) },
    }
        }
    }

    fn intPromote(__0: CType) -> CType {
        match (__0) {
            <todo> => { IsInt(Signed, (BitWidth(32))) },
            <todo> => { enumReprType },
            <todo> => { x },
        }
    }

    fn integerConversionRank(__0: Maybe) -> Maybe {
        match (__0, __1) {
            <todo> => { Just((compare(a, b))) },
            <todo> => { Just(EQ) },
            <todo> => { Nothing },
        }
    }

    fn interpretBlockItem(__0: CSourceBuildCFGT) -> CSourceBuildCFGT {
        match (__0, __1) {
            <todo> => { interpretStatement(stmt, next) },
            <todo> => { {

                    let decl_q = lift(lift((interpretDeclarations(makeLetBinding, decl))));
                    let (rest, end) = next;
                    return((++(decl_q, rest), end))
            } },
            <todo> => { lift(lift((unimplemented(item)))) },
        }
    }

    fn interpretConstExpr(__0: EnvMonad) -> EnvMonad {
        match (__0) {
            <todo> => { return(v) },
            <todo> => { unimplemented(expr) },
        }
    }

    fn interpretDeclarations(__0: MakeBinding) -> MakeBinding {
        match (__0, __1, __2, __3) {
            <todo> => { {

                    let (storagespecs, baseTy) = baseTypeOf(specs);
                    let mbinds = forM(decls)(Lambda);
                    return((catMaybes(mbinds)))
            } },
            <todo> => { unimplemented(node) },
        }
    }

    fn interpretExpr(__0: EnvMonad) -> EnvMonad {
        match (__0, __1) {
            <todo> => { {

                    Let([Assign([Span([Parens([Span([Ref(Ident("effects"))]), Span([Ref(Ident("mfinal"))])])])], Span([Ref(Ident("if")), Ref(Ident("demand")), Ref(Ident("then")), Parens([Span([Ref(Ident("init")), Ref(Ident("exprs"))]), Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("last")), Ref(Ident("exprs"))])])])]), Ref(Ident("else")), Parens([Span([Ref(Ident("exprs"))]), Span([Ref(Ident("Nothing"))])])]))], []);
                    let effects_q = mapM(((fmap(resultToStatements) . interpretExpr(False))), effects);
                    let mfinal_q = mapM((interpretExpr(True)), mfinal);
                    return(Result, {
    resultType: maybe(IsVoid, resultType, mfinal_q),
    resultMutable: maybe(Rust_Immutable, resultMutable, mfinal_q),
    result: Rust_BlockExpr((Rust_Block((concat(effects_q)), (fmap(result, mfinal_q)))))
    })
            } },
            <todo> => { {

                    let lhs_q = interpretExpr(True, lhs);
                    let rhs_q = interpretExpr(True, rhs);
                    compound(expr, False, demand, op, lhs_q, rhs_q)
            } },
            <todo> => { {

                    let c_q = fmap(toBool, (interpretExpr(True, c)));
                    let t_q = interpretExpr(demand, t);
                    let f_q = interpretExpr(demand, f);
                    if(demand, then, promotePtr, expr, (mkIf(c_q)), t_q, f_q, else, return, Result, {
    resultType: IsVoid,
    resultMutable: Rust_Immutable,
    result: mkIf(c_q, (result(t_q)), (result(f_q)))
    });
                
            } },
            <todo> => { {

                    let lhs_q = interpretExpr(True, lhs);
                    let rhs_q = interpretExpr(True, rhs);
                    binop(expr, op, lhs_q, rhs_q)
            } },
            <todo> => { {

                    let (_mut, ty) = typeName(decl);
                    let expr_q = interpretExpr((/=(ty, IsVoid)), expr);
                    return(Result, {
    resultType: ty,
    resultMutable: Rust_Immutable,
    result: ((if(ty) == IsVoid(then, result, else, castTo, ty)))(expr_q)
    })
            } },
            <todo> => { match op {
                    CPreIncOp => { incdec(False, CAddAssOp) },
                    CPreDecOp => { incdec(False, CSubAssOp) },
                    CPostIncOp => { incdec(True, CAddAssOp) },
                    CPostDecOp => { incdec(True, CSubAssOp) },
                    CAdrOp => { {

                            let expr_q = interpretExpr(True, expr);
                            Let([Assign([Span([Ref(Ident("ty\'"))])], Span([Ref(Ident("IsPtr")), Parens([Span([Ref(Ident("resultMutable")), Ref(Ident("expr\'"))])]), Parens([Span([Ref(Ident("resultType")), Ref(Ident("expr\'"))])])]))], []);
                            return(Result, {
    resultType: ty_q,
    resultMutable: Rust_Immutable,
    result: Rust_Cast((Rust_Borrow((resultMutable(expr_q)), (result(expr_q)))), (toRustType(ty_q)))
    })
                    } },
                    CIndOp => { {

                            let expr_q = interpretExpr(True, expr);
                            match resultType(expr_q) {
        IsPtr, mut_q, ty_q => { return(Result, {
            resultType: ty_q,
            resultMutable: mut_q,
            result: Rust_Deref((result(expr_q)))
            }) },
        IsFunc, { .. } => { return(expr_q) },
        _ => { badSource(node, "dereference of non-pointer".to_string()) },
    }
                    } },
                    CPlusOp => { {

                            let expr_q = interpretExpr(demand, expr);
                            Let([Assign([Span([Ref(Ident("ty\'"))])], Span([Ref(Ident("intPromote")), Parens([Span([Ref(Ident("resultType")), Ref(Ident("expr\'"))])])]))], []);
                            return(Result, {
    resultType: ty_q,
    resultMutable: Rust_Immutable,
    result: castTo(ty_q, expr_q)
    })
                    } },
                    CMinOp => { fmap(wrapping)(simple(Rust_Neg)) },
                    CCompOp => { simple(Rust_Not) },
                    CNegOp => { {

                            let expr_q = interpretExpr(True, expr);
                            return(Result, {
    resultType: IsBool,
    resultMutable: Rust_Immutable,
    result: toNotBool(expr_q)
    })
                    } },
                } },
            <todo> => { {

                    let e_q = interpretExpr(True, e);
                    return((rustSizeOfType((toRustType((resultType(e_q)))))))
            } },
            <todo> => { {

                    let (_mut, ty) = typeName(decl);
                    return((rustSizeOfType((toRustType(ty)))))
            } },
            <todo> => { {

                    let e_q = interpretExpr(True, e);
                    return((rustAlignOfType((toRustType((resultType(e_q)))))))
            } },
            <todo> => { {

                    let (_mut, ty) = typeName(decl);
                    return((rustAlignOfType((toRustType(ty)))))
            } },
            <todo> => { {

                    let lhs_q = interpretExpr(True, lhs);
                    let rhs_q = interpretExpr(True, rhs);
                    match (resultType(lhs_q), resultType(rhs_q)) {
        (IsArray(mut, _, el), _) => { return((subscript(mut, el, (result(lhs_q)), rhs_q))) },
        (_, IsArray(mut, _, el)) => { return((subscript(mut, el, (result(rhs_q)), lhs_q))) },
        _ => { {

                let ptr = binop(expr, CAddOp, lhs_q, rhs_q);
                match resultType(ptr) {
        IsPtr, mut, ty => { return(Result, {
            resultType: ty,
            resultMutable: mut,
            result: Rust_Deref((result(ptr)))
            }) },
        _ => { badSource(expr, "array subscript of non-pointer".to_string()) },
    }
        } },
    };
                
            } },
            <todo> => { {

                    let func_q = interpretExpr(True, func);
                    match resultType(func_q) {
        IsFunc, retTy, argTys, variadic => { {

                let args_q = castArgs(variadic, (map(snd, argTys)), args);
                return(Result, {
    resultType: retTy,
    resultMutable: Rust_Immutable,
    result: Rust_Call((result(func_q)), args_q)
    })
        } },
        _ => { badSource(expr, "function call to non-function".to_string()) },
    };
                
            } },
            <todo> => { {

                    let obj_q = interpretExpr(True)(if(deref, then, CUnary, CIndOp, obj, node, else, obj));
                    let objTy = completeType((resultType(obj_q)));
                    let fields = match objTy {
            IsStruct, _, fields => { return(fields) },
            _ => { badSource(expr, "member access of non-struct".to_string()) },
        };
                    Let([Assign([Span([Ref(Ident("field"))])], Span([Ref(Ident("applyRenames")), Ref(Ident("ident"))]))], []);
                    let ty = match lookup(field, fields) {
            Just, ty => { return(ty) },
            Nothing => { badSource(expr, "request for non-existent field".to_string()) },
        };
                    return(Result, {
    resultType: ty,
    resultMutable: resultMutable(obj_q),
    result: Rust_Member((result(obj_q)), (Rust_VarName(field)))
    })
            } },
            <todo> => { {

                    let sym = getSymbolIdent(ident);
                    maybe((badSource(expr, "undefined variable".to_string())), return, sym)
            } },
            <todo> => { match c {
                    CIntConst, CInteger(v, repr, flags), _ => { Let([Assign([Span([Ref(Ident("allow_signed"))])], Span([Ref(Ident("not")), Parens([Span([Ref(Ident("testFlag")), Ref(Ident("FlagUnsigned")), Ref(Ident("flags"))])])])), Assign([Span([Ref(Ident("allow_unsigned"))])], Span([Ref(Ident("not")), Ref(Ident("allow_signed")), Operator("||"), Ref(Ident("repr")), Operator("/="), Ref(Ident("DecRepr"))])), Assign([Span([Ref(Ident("widths"))])], Span([Vector([Span([Parens([Span([Number(32)]), Span([Ref(Ident("if")), Ref(Ident("any")), Parens([Span([Operator("testFlag"), Ref(Ident("flags"))])]), Vector([Span([Ref(Ident("FlagLongLong"))]), Span([Ref(Ident("FlagLong"))])]), Ref(Ident("then")), Ref(Ident("WordWidth")), Ref(Ident("else")), Ref(Ident("BitWidth")), Number(32)])])]), Span([Parens([Span([Number(64)]), Span([Ref(Ident("BitWidth")), Number(64)])])])])])), Assign([Span([Ref(Ident("allowed_types"))])], Span([Dummy])), Assign([Span([Ref(Ident("repr\'"))])], Span([Case(Span([Ref(Ident("repr"))]), [Direct([Ref(Ident("DecRepr"))], [Span([Ref(Ident("Rust.DecRepr"))])]), Direct([Ref(Ident("OctalRepr"))], [Span([Ref(Ident("Rust.OctalRepr"))])]), Direct([Ref(Ident("HexRepr"))], [Span([Ref(Ident("Rust.HexRepr"))])])])]))], [])(in, match allowed_types {
                            [] => { badSource(expr, "integer (too big)".to_string()) },
                            ty, :, _ => { return((literalNumber(ty, (Rust_LitInt(v, repr_q))))) },
                        }) },
                    CFloatConst, CFloat(str), _ => { match span((Operator("notElem")("fF".to_string())), str) {
                            (v, "") => { return((literalNumber((IsFloat(64)), (Rust_LitFloat(v))))) },
                            (v, [_]) => { return((literalNumber((IsFloat(32)), (Rust_LitFloat(v))))) },
                            _ => { badSource(expr, "float".to_string()) },
                        } },
                    CCharConst, CChar(ch, False), _ => { return(Result, {
                        resultType: charType,
                        resultMutable: Rust_Immutable,
                        result: Rust_Lit((Rust_LitByteChar(ch)))
                        }) },
                    CStrConst, CString(str, False), _ => { return(Result, {
                        resultType: IsArray(Rust_Immutable, (+(length(str), 1)), charType),
                        resultMutable: Rust_Immutable,
                        result: Rust_Deref((Rust_Lit((Rust_LitByteStr((++(str, "\u{0}".to_string())))))))
                        }) },
                    _ => { unimplemented(expr) },
                } },
            <todo> => { {

                    let (mut, ty) = typeName(decl);
                    let final = interpretInitializer(ty, (CInitList(initials, info)));
                    return(Result, {
    resultType: ty,
    resultMutable: mut,
    result: final
    })
            } },
            <todo> => { scope({

                        Let([Assign([Span([Parens([Span([Ref(Ident("effects"))]), Span([Ref(Ident("final"))])])])], Span([Case(Span([Ref(Ident("last")), Ref(Ident("stmts"))]), [Matching([Ref(Ident("CBlockStmt")), Span([Ref(Ident("CExpr")), Ref(Ident("expr")), Ref(Ident("_"))])], [([Span([Ref(Ident("demand"))])], Span([Parens([Span([Ref(Ident("init")), Ref(Ident("stmts"))]), Span([Ref(Ident("expr"))])])]))]), Direct([Ref(Ident("_"))], [Span([Parens([Span([Ref(Ident("stmts"))]), Span([Ref(Ident("Nothing"))])])])])])]))], []);
                        let effects_q = cfgToRust(stat, (foldr(interpretBlockItem, (return((vec![], Unreachable))), effects)));
                        let final_q = mapM((interpretExpr(True)), final);
                        return(Result, {
    resultType: maybe(IsVoid, resultType, final_q),
    resultMutable: maybe(Rust_Immutable, resultMutable, final_q),
    result: Rust_BlockExpr((Rust_Block(effects_q, (fmap(result, final_q)))))
    })
                }) },
            <todo> => { unimplemented(expr) },
        }
    }

    fn interpretFunction((@(CFunDef(specs, declr), (CDeclr(mident, _, _, _, _))(argtypes, body, _))): EnvMonad) -> EnvMonad {
        {

                let (storage, baseTy) = baseTypeOf(specs);
                let (attrs, vis) = match storage {
            Nothing => { return((vec![Rust_Attribute("no_mangle".to_string())], Rust_Public)) },
            Just, CStatic(_) => { return((vec![], Rust_Private)) },
            Just, s => { badSource(s, "storage class specifier for function".to_string()) },
        };
                Let([Assign([Span([Ref(Ident("go")), Ref(Ident("name")), Ref(Ident("funTy"))])], Span([Do([Expression(Span([Parens([Span([Ref(Ident("retTy"))]), Span([Ref(Ident("args"))])]), Operator("<-"), Case(Span([Ref(Ident("funTy"))]), [Direct([Ref(Ident("IsFunc")), Ref(Ident("_")), Ref(Ident("_")), Ref(Ident("True"))], [Span([Ref(Ident("unimplemented")), Ref(Ident("declr"))])]), Direct([Ref(Ident("IsFunc")), Ref(Ident("retTy")), Ref(Ident("args")), Ref(Ident("False"))], [Span([Ref(Ident("return")), Parens([Span([Ref(Ident("retTy"))]), Span([Ref(Ident("args"))])])])]), Direct([Ref(Ident("_"))], [Span([Ref(Ident("badSource")), Ref(Ident("declr")), Str("function definition")])])])]), []), Expression(Span([Ref(Ident("when")), Parens([Span([Ref(Ident("name")), Operator("=="), Str("_c_main")])]), Parens([Span([Ref(Ident("wrapMain")), Ref(Ident("declr")), Ref(Ident("name")), Parens([Span([Ref(Ident("map")), Ref(Ident("snd")), Ref(Ident("args"))])])])])]), []), Expression(Span([Let([Assign([Span([Ref(Ident("setRetTy")), Ref(Ident("flow"))])], Span([Ref(Ident("flow")), Record([(Ident("functionReturnType"), Span([Ref(Ident("Just")), Ref(Ident("retTy"))])), (Ident("functionName"), Span([Ref(Ident("Just")), Ref(Ident("name"))]))])]))], [])]), []), Expression(Span([Ref(Ident("f\'")), Operator("<-"), Ref(Ident("mapExceptT")), Parens([Span([Ref(Ident("local")), Ref(Ident("setRetTy"))])]), Operator("$"), Ref(Ident("scope")), Operator("$"), Do([Expression(Span([Ref(Ident("formals")), Operator("<-"), Ref(Ident("sequence")), Dummy]), []), Expression(Span([Let([Assign([Span([Ref(Ident("returnValue"))])], Span([Ref(Ident("if")), Ref(Ident("name")), Operator("=="), Str("_c_main"), Ref(Ident("then")), Ref(Ident("Just")), Number(0), Ref(Ident("else")), Ref(Ident("Nothing"))])), Assign([Span([Ref(Ident("returnStatement"))])], Span([Ref(Ident("Rust.Stmt")), Parens([Span([Ref(Ident("Rust.Return")), Ref(Ident("returnValue"))])])]))], [])]), []), Expression(Span([Ref(Ident("body\'")), Operator("<-"), Ref(Ident("cfgToRust")), Ref(Ident("declr")), Parens([Span([Ref(Ident("interpretStatement")), Ref(Ident("body")), Parens([Span([Ref(Ident("return")), Parens([Span([Vector([Span([Ref(Ident("returnStatement"))])])]), Span([Ref(Ident("Unreachable"))])])])])])])]), []), Expression(Span([Ref(Ident("return")), Parens([Span([Ref(Ident("Rust.Item")), Ref(Ident("attrs")), Ref(Ident("vis")), Parens([Span([Ref(Ident("Rust.Function")), Vector([Span([Ref(Ident("Rust.UnsafeFn"))]), Span([Ref(Ident("Rust.ExternABI")), Ref(Ident("Nothing"))])]), Ref(Ident("name")), Ref(Ident("formals")), Parens([Span([Ref(Ident("toRustRetType")), Ref(Ident("retTy"))])]), Parens([Span([Ref(Ident("statementsToBlock")), Ref(Ident("body\'"))])])])])])])]), [])], [])]), []), Expression(Span([Ref(Ident("emitItems")), Vector([Span([Ref(Ident("f\'"))])])]), [])], [])]))], []);
                let ident = match mident {
            Nothing => { badSource(declr, "anonymous function definition".to_string()) },
            Just, ident => { return(ident) },
        };
                Let([Assign([Span([Ref(Ident("name"))])], Span([Ref(Ident("applyRenames")), Ref(Ident("ident"))]))], []);
                Let([Assign([Span([Ref(Ident("funTy")), Ref(Ident("itype"))])], Span([Ref(Ident("typeToResult")), Ref(Ident("itype")), Parens([Span([Ref(Ident("Rust.Path")), Parens([Span([Ref(Ident("Rust.PathSegments")), Vector([Span([Ref(Ident("name"))])])])])])])]))], []);
                let deferred = fmap((fmap(funTy)), (derivedDeferredTypeOf(baseTy, declr, argtypes)));
                let alreadyUsed = lift(gets(((usedForwardRefs . globalState))));
                match vis {
    Rust_Private => if Set.notMember(ident, alreadyUsed) { {

            let action = runOnce({

                let ty = deferred;
                go(name, (resultType(ty)));
                return(ty)
        });
            addSymbolIdentAction(ident, action)
    } },
        _ => { {

                let ty = deferred;
                addSymbolIdentAction(ident)(return(ty));
                go(name, (resultType(ty)))
        } },
    }
        }
    }

    fn interpretInitializer(ty: EnvMonad) -> EnvMonad {
        {

                let initial_q = match initial {
            CInitExpr, expr, _ => { {

                    let expr_q = interpretExpr(True, expr);
                    compatibleInitializer(if(resultType, expr_q), ty(then, pure)(scalar((castTo(ty, expr_q)), else, badSource, initial, "initializer for incompatible type".to_string())))
            } },
            CInitList, list, _ => { translateInitList(ty, list) },
        };
                let zeroed = zeroInitialize(initial_q, ty);
                helper(ty, zeroed);
            
        }
    }

    fn interpretStatement(__0: CSourceBuildCFGT) -> CSourceBuildCFGT {
        match (__0, __1) {
            <todo> => { {

                    let label = gotoLabel(ident);
                    let (rest, end) = interpretStatement(body, next);
                    addBlock(label, rest, end);
                    return((vec![], Branch(label)))
            } },
            <todo> => { {

                    let selector = getSwitchExpression(stmt);
                    Let([Assign([Span([Ref(Ident("condition"))])], Span([Ref(Ident("CBinary")), Ref(Ident("CEqOp")), Ref(Ident("selector")), Ref(Ident("expr")), Ref(Ident("node"))]))], []);
                    addSwitchCase((Just(condition)), body, next)
            } },
            <todo> => { {

                    let selector = getSwitchExpression(stmt);
                    Let([Assign([Span([Ref(Ident("condition"))])], Span([Ref(Ident("CBinary")), Ref(Ident("CLndOp")), Parens([Span([Ref(Ident("CBinary")), Ref(Ident("CGeqOp")), Ref(Ident("selector")), Ref(Ident("lower")), Ref(Ident("node"))])]), Parens([Span([Ref(Ident("CBinary")), Ref(Ident("CLeqOp")), Ref(Ident("selector")), Ref(Ident("upper")), Ref(Ident("node"))])]), Ref(Ident("node"))]))], []);
                    addSwitchCase((Just(condition)), body, next)
            } },
            <todo> => { addSwitchCase(Nothing, body, next) },
            <todo> => { next },
            <todo> => { {

                    let expr_q = lift(lift(interpretExpr(False, expr)));
                    let (rest, end) = next;
                    return((++(resultToStatements(expr_q), rest), end))
            } },
            <todo> => { mapBuildCFGT((mapRWST(scope)))({

                        foldr(interpretBlockItem, next, items)
                }) },
            <todo> => { {

                    let c_q = lift(lift(interpretExpr(True, c)));
                    let after = newLabel;
                    let falseLabel = match mf {
            Nothing => { return(after) },
            Just, f => { {

                    let (falseEntry, falseTerm) = interpretStatement(f, (return((vec![], Branch(after)))));
                    let falseLabel = newLabel;
                    addBlock(falseLabel, falseEntry, falseTerm);
                    return(falseLabel)
            } },
        };
                    let (trueEntry, trueTerm) = interpretStatement(t, (return((vec![], Branch(after)))));
                    let trueLabel = newLabel;
                    addBlock(trueLabel, trueEntry, trueTerm);
                    let (rest, end) = next;
                    addBlock(after, rest, end);
                    return((vec![], CondBranch(c_q, trueLabel, falseLabel)))
            } },
            <todo> => { {

                    let (bindings, expr_q) = match expr {
            CVar, { .. } => { return((vec![], expr)) },
            _ => { lift(lift({

                            let ident = fmap(internalIdent, (uniqueName("switch".to_string())));
                            let rhs = interpretExpr(True, expr);
                            let var = addSymbolIdent(ident, (Rust_Immutable, resultType(rhs)));
                            return((vec![Rust_Let(Rust_Immutable, (Rust_VarName(var)), Nothing, (Just((result(rhs)))))], CVar(ident, node)))
                    })) },
        };
                    let after = newLabel;
                    let (_, SwitchCases(cases)) = getSwitchCases(expr_q)(setBreak(after)(interpretStatement(body, (return((vec![], Branch(after)))))));
                    Let([Assign([Span([Ref(Ident("isDefault")), Parens([Span([Ref(Ident("Just")), Ref(Ident("condition"))])])])], Span([Ref(Ident("Left")), Ref(Ident("condition"))])), Assign([Span([Ref(Ident("isDefault")), Ref(Ident("Nothing"))])], Span([Ref(Ident("Right")), Parens([])]))], []);
                    Let([Assign([Span([Parens([Span([Ref(Ident("conditions"))]), Span([Ref(Ident("defaults"))])])])], Span([Ref(Ident("IntMap.mapEither")), Ref(Ident("isDefault")), Ref(Ident("cases"))]))], []);
                    let defaultCase = match IntMap_keys(defaults) {
            [] => { return(after) },
            [defaultCase] => { return(defaultCase) },
            _ => { lift(lift(badSource(stmt, "duplicate default cases".to_string()))) },
        };
                    let entry = foldrM(conditionBlock, defaultCase, (IntMap_toList(conditions)));
                    let (rest, end) = next;
                    addBlock(after, rest, end);
                    return((bindings, Branch(entry)));
                
            } },
            <todo> => { {

                    let c_q = lift(lift(interpretExpr(True, c)));
                    let after = newLabel;
                    let headerLabel = newLabel;
                    let (bodyEntry, bodyTerm) = setBreak(after)(setContinue(headerLabel)(interpretStatement(body, (return((vec![], Branch(headerLabel)))))));
                    let bodyLabel = newLabel;
                    addBlock(bodyLabel, bodyEntry, bodyTerm);
                    addBlock(headerLabel, vec![])(match toBool(c_q) {
        Rust_Lit, Rust_LitBool(cont) => if /=(cont, doWhile) { Branch((if(cont, then, bodyLabel, else, after))) },
            _ => { CondBranch(c_q, bodyLabel, after) },
        });
                    let (rest, end) = next;
                    addBlock(after, rest, end);
                    return((vec![], Branch((if(doWhile, then, bodyLabel, else, headerLabel)))))
            } },
            <todo> => { {

                    let after = newLabel;
                    let ret = mapBuildCFGT((mapRWST(scope)))({

                let prefix = match initial {
            Left, Nothing => { return(vec![]) },
            Left, Just(expr) => { {

                    let expr_q = lift(lift(interpretExpr(False, expr)));
                    return((resultToStatements(expr_q)))
            } },
            Right, decls => { lift(lift(interpretDeclarations(makeLetBinding, decls))) },
        };
                let headerLabel = newLabel;
                let incrLabel = match mincr {
            Nothing => { return(headerLabel) },
            Just, incr => { {

                    let incr_q = lift(lift(interpretExpr(False, incr)));
                    let incrLabel = newLabel;
                    addBlock(incrLabel, (resultToStatements(incr_q)), (Branch(headerLabel)));
                    return(incrLabel)
            } },
        };
                let (bodyEntry, bodyTerm) = setBreak(after)(setContinue(incrLabel)(interpretStatement(body, (return((vec![], Branch(incrLabel)))))));
                let bodyLabel = newLabel;
                addBlock(bodyLabel, bodyEntry, bodyTerm);
                let cond = match mcond {
            Just, cond => { {

                    let cond_q = lift(lift(interpretExpr(True, cond)));
                    return((CondBranch(cond_q, bodyLabel, after)))
            } },
            Nothing => { return((Branch(bodyLabel))) },
        };
                addBlock(headerLabel, vec![], cond);
                return((prefix, Branch(headerLabel)))
        });
                    let (rest, end) = next;
                    addBlock(after, rest, end);
                    return(ret)
            } },
            <todo> => { {

                    let _ = next;
                    let label = gotoLabel(ident);
                    return((vec![], Branch(label)))
            } },
            <todo> => { {

                    let _ = next;
                    let val = lift((asks(onContinue)));
                    match val {
        Just, label => { return((vec![], Branch(label))) },
        Nothing => { lift(lift(badSource(stmt, "continue outside loop".to_string()))) },
    }
            } },
            <todo> => { {

                    let _ = next;
                    let val = lift((asks(onBreak)));
                    match val {
        Just, label => { return((vec![], Branch(label))) },
        Nothing => { lift(lift(badSource(stmt, "break outside loop".to_string()))) },
    }
            } },
            <todo> => { {

                    let _ = next;
                    lift(lift({

                let val = lift((asks(functionReturnType)));
                match val {
        Nothing => { badSource(stmt, "return statement outside function".to_string()) },
        Just, retTy => { {

                let expr_q = mapM(((fmap((castTo(retTy))) . interpretExpr(True))), expr);
                return((exprToStatements((Rust_Return(expr_q))), Unreachable))
        } },
    }
        }))
            } },
            <todo> => { lift(lift(unimplemented(stmt))) },
        }
    }

    fn interpretTranslationUnit(_thisModule: Either) -> Either {
        match err {
                Left, msg => { Left(msg) },
                Right, _ => { Right(items_q) },
            }
    }

    fn makeLetBinding() -> MakeBinding {
        (Rust_StmtItem(vec![]), makeBinding)
    }

    fn makeStaticBinding() -> MakeBinding {
        (Rust_Item(vec![], Rust_Private), makeBinding)
    }

    fn modifyGlobal(f: EnvMonad) -> EnvMonad {
        lift({

                    let st = get;
                    Let([Assign([Span([Parens([Span([Ref(Ident("global\'"))]), Span([Ref(Ident("a"))])])])], Span([Ref(Ident("f")), Parens([Span([Ref(Ident("globalState")), Ref(Ident("st"))])])]))], []);
                    put(st, {
    globalState: global_q
    });
                    return(a)
            })
    }

    fn mutable(quals: Rust.Mutable) -> Rust.Mutable {
        if(any, (Lambda), quals, then, Rust_Immutable, else, Rust_Mutable)
    }

    fn nestedObject(ty: Maybe) -> Maybe {
        match designatorType(desig) {
                IsArray, _, size, el => { Just((From(el, 0, (replicate((-(size, 1)), el)), desig))) },
            ty_q => if compatibleInitializer(ty, ty_q) { Just(desig) },
                IsStruct, _, (_, ty_q)(:, fields) => { nestedObject(ty, (From(ty_q, 0, (map(snd, fields)), desig))) },
                _ => { Nothing },
            }
    }

    fn nextObject(__0: CurrentObject) -> CurrentObject {
        match (__0, __1) {
            <todo> => { Nothing },
            <todo> => { Just((From(ty, (+(i, 1)), remaining, base))) },
            <todo> => { nextObject(base) },
        }
    }

    fn noTranslation(node: EnvMonad) -> EnvMonad {
        throwE(concat(vec![show((posOf(node))), ": ".to_string(), msg, ":\n".to_string(), render((nest(4, (pretty(node)))))]))
    }

    fn objectFromDesignators(__0: EnvMonad) -> EnvMonad {
        match (__0, __1) {
            <todo> => { pure(Nothing) },
            <todo> => { <$>(Just, go(ty, desigs, (Base(ty)))) },
        }
    }

    fn promote(node: EnvMonad) -> EnvMonad {
        match usual((resultType(a)), (resultType(b))) {
                Just, rt => { return(Result, {
                    resultType: rt,
                    resultMutable: Rust_Immutable,
                    result: op((castTo(rt, a)), (castTo(rt, b)))
                    }) },
                Nothing => { badSource(node)(concat(vec!["arithmetic combination for ".to_string(), show((resultType(a))), " and ".to_string(), show((resultType(b)))])) },
            }
    }

    fn promotePtr(node: EnvMonad) -> EnvMonad {
        match (resultType(a), resultType(b)) {
                (IsArray(_, _, _), _) => { ptrs },
                (IsPtr(_, _), _) => { ptrs },
                (_, IsArray(_, _, _)) => { ptrs },
                (_, IsPtr(_, _)) => { ptrs },
                _ => { promote(node, op, a, b) },
            }
    }

    fn resolveCurrentObject((obj0, prior): EnvMonad) -> EnvMonad {
        match mplus(obj1, obj0) {
                Nothing => { return((Nothing, prior)) },
                Just, obj => { {

                        let (obj_q, initial) = match cinitial {
            CInitList, list_q, _ => { {

                    let initial = translateInitList((designatorType(obj)), list_q);
                    return((obj, initial))
            } },
            CInitExpr, expr, _ => { {

                    let expr_q = interpretExpr(True, expr);
                    match nestedObject((resultType(expr_q)), obj) {
        Nothing => { badSource(cinitial, "type in initializer".to_string()) },
        Just, obj_q => { {

                Let([Assign([Span([Ref(Ident("s"))])], Span([Ref(Ident("castTo")), Parens([Span([Ref(Ident("designatorType")), Ref(Ident("obj\'"))])]), Ref(Ident("expr\'"))]))], []);
                return((obj_q, scalar(s)))
        } },
    }
            } },
        };
                        Let([Assign([Span([Ref(Ident("indices"))])], Span([Ref(Ident("unfoldr")), Parens([Span([Lambda])]), Ref(Ident("obj\'"))]))], []);
                        Let([Assign([Span([Ref(Ident("initializer"))])], Span([Ref(Ident("foldl")), Parens([Span([Lambda, Ref(Ident("Nothing")), Parens([Span([Ref(Ident("IntMap.singleton")), Ref(Ident("j")), Ref(Ident("a"))])])])]), Ref(Ident("initial")), Ref(Ident("indices"))]))], []);
                        return((nextObject(obj_q), mappend(prior, initializer)))
                } },
            }
    }

    fn resultToStatements() -> Vec<Rust.Stmt> {
        (exprToStatements . result)
    }

    fn runOnce(action: EnvMonad) -> EnvMonad {
        {

                let cacheRef = lift(lift(newSTRef((Left(action)))));
                return({

            let cache = lift(lift(readSTRef(cacheRef)));
            match cache {
        Left, todo => { {

                lift(lift(writeSTRef(cacheRef)(Left(fail("internal error: runOnce action depends on itself, leading to an infinite loop".to_string())))));
                let val = todo;
                lift(lift(writeSTRef(cacheRef, (Right(val)))));
                return(val)
        } },
        Right, val => { return(val) },
    }
    })
        }
    }

    fn rustAlignOfType((Rust_TypeName(ty)): Result) -> Result {
        Result({
            resultType: IsInt(Unsigned, WordWidth),
            resultMutable: Rust_Immutable,
            result: Rust_Call((Rust_Var((Rust_VarName((++("::std::mem::align_of::<".to_string(), ++(ty, ">".to_string()))))))), vec![])
            })
    }

    fn rustSizeOfType((Rust_TypeName(ty)): Result) -> Result {
        Result({
            resultType: IsInt(Unsigned, WordWidth),
            resultMutable: Rust_Immutable,
            result: Rust_Call((Rust_Var((Rust_VarName((++("::std::mem::size_of::<".to_string(), ++(ty, ">".to_string()))))))), vec![])
            })
    }

    fn scalar(expr: Initializer) -> Initializer {
        Initializer((Just(expr)), IntMap_empty)
    }

    fn scope(m: EnvMonad) -> EnvMonad {
        {

                let old = lift(get);
                let a = m;
                lift((modify((Lambda({
            globalState: globalState(st)
            })))));
                return(a)
        }
    }

    fn setBreak(label: CSourceBuildCFGT) -> CSourceBuildCFGT {
        mapBuildCFGT((local((Lambda({
                    onBreak: Just(label)
                    })))))
    }

    fn setContinue(label: CSourceBuildCFGT) -> CSourceBuildCFGT {
        mapBuildCFGT((local((Lambda({
                    onContinue: Just(label)
                    })))))
    }

    fn statementsToBlock(__0: Rust.Block) -> Rust.Block {
        match (__0) {
            <todo> => { stmts },
            <todo> => { Rust_Block(stmts, Nothing) },
        }
    }

    fn toBool(__0: Rust.Expr) -> Rust.Expr {
        match (__0) {
            <todo> => { Rust_Lit((Rust_LitBool(False))) },
            <todo> => { Rust_Lit((Rust_LitBool(True))) },
            <todo> => { match t {
                    IsBool => { v },
                    IsPtr, _, _ => { Rust_Not((Rust_MethodCall(v, (Rust_VarName("is_null".to_string())), vec![]))) },
                    _ => { Rust_CmpNE(v, 0) },
                } },
        }
    }

    fn toNotBool(__0: Rust.Expr) -> Rust.Expr {
        match (__0) {
            <todo> => { Rust_Lit((Rust_LitBool(True))) },
            <todo> => { Rust_Lit((Rust_LitBool(False))) },
            <todo> => { match t {
                    IsBool => { Rust_Not(v) },
                    IsPtr, _, _ => { Rust_MethodCall(v, (Rust_VarName("is_null".to_string())), vec![]) },
                    _ => { Rust_CmpEQ(v, 0) },
                } },
        }
    }

    fn toPtr(__0: Maybe) -> Maybe {
        match (__0, __1, __2) {
            <todo> => { Just(ptr, {
                resultType: IsPtr(mut, el),
                result: castTo((IsPtr(mut, el)), ptr)
                }) },
            <todo> => { Just(ptr) },
            <todo> => { Nothing },
        }
    }

    fn toRustRetType(__0: Rust.Type) -> Rust.Type {
        match (__0) {
            <todo> => { Rust_TypeName("()".to_string()) },
            <todo> => { toRustType(ty) },
        }
    }

    fn toRustType(__0: Rust.Type) -> Rust.Type {
        match (__0) {
            <todo> => { Rust_TypeName("bool".to_string()) },
            <todo> => { Rust_TypeName((:((match s {
                                Signed => { 'i' },
                                Unsigned => { 'u' },
                            }), (match w {
                                BitWidth, b => { show(b) },
                                WordWidth => { "size".to_string() },
                            })))) },
            <todo> => { Rust_TypeName((:('f', show(w)))) },
            <todo> => { Rust_TypeName("::std::os::raw::c_void".to_string()) },
            <todo> => { Rust_TypeName(concat(vec!["unsafe extern fn(".to_string(), args_q, ")".to_string(), /=(if(retTy), ++(IsVoid(then, " -> ".to_string()), typename(retTy, else, "".to_string())))])) },
            <todo> => { Let([Assign([Span([Ref(Ident("Rust.TypeName")), Ref(Ident("to\'"))])], Span([Ref(Ident("toRustType")), Ref(Ident("to")), Ref(Ident("in")), Ref(Ident("Rust.TypeName")), Parens([Span([Ref(Ident("rustMut")), Ref(Ident("mut")), Operator("++"), Ref(Ident("to\'"))])])]))], []) },
            <todo> => { Rust_TypeName((++("[".to_string(), ++(typename(el), ++("; ".to_string(), ++(show(size), "]".to_string())))))) },
            <todo> => { Rust_TypeName(name) },
            <todo> => { Rust_TypeName(name) },
            <todo> => { Rust_TypeName((identToString(ident))) },
        }
    }

    fn translateInitList(ty: EnvMonad) -> EnvMonad {
        {

                let objectsAndInitializers = forM(list)(Lambda);
                Let([Assign([Span([Ref(Ident("base"))])], Span([Case(Span([Ref(Ident("ty"))]), [Direct([Ref(Ident("IsArray")), Ref(Ident("_")), Ref(Ident("size")), Ref(Ident("el"))], [Span([Ref(Ident("From")), Ref(Ident("el")), Number(0), Parens([Span([Ref(Ident("replicate")), Parens([Span([Ref(Ident("size")), Operator("-"), Number(1)])]), Ref(Ident("el"))])]), Parens([Span([Ref(Ident("Base")), Ref(Ident("ty"))])])])]), Direct([Ref(Ident("IsStruct")), Ref(Ident("_")), Span([Tuple([Span([Ref(Ident("_"))]), Span([Ref(Ident("ty\'"))])]), Ref(Ident(":")), Ref(Ident("fields"))])], [Span([Ref(Ident("From")), Ref(Ident("ty\'")), Number(0), Parens([Span([Ref(Ident("map")), Ref(Ident("snd")), Ref(Ident("fields"))])]), Parens([Span([Ref(Ident("Base")), Ref(Ident("ty"))])])])]), Direct([Ref(Ident("_"))], [Span([Ref(Ident("Base")), Ref(Ident("ty"))])])])]))], []);
                let (_, initializer) = foldM(resolveCurrentObject, (Just(base), mempty), objectsAndInitializers);
                return(initializer)
        }
    }

    fn typeName(__0: EnvMonad) -> EnvMonad {
        match (__0, __1, __2) {
            <todo> => { badSource(decl, "static assert in type name ".to_string()) },
            <todo> => { {

                    let (storage, base) = baseTypeOf(spec);
                    match storage {
        Just, s => { badSource(s, "storage class specifier in type name".to_string()) },
        Nothing => { return(()) },
    };
                    let itype = match declarators {
            [] => { base },
            [(Just(declr, @, CDeclr(Nothing, _, _, _, _)), Nothing, Nothing)] => { derivedTypeOf(base, declr) },
            _ => { badSource(decl, "type name".to_string()) },
        };
                    when((typeIsFunc(itype)), (badSource(decl, "use of function type".to_string())));
                    return((typeMutable(itype), typeRep(itype)))
            } },
        }
    }

    fn typeToResult(itype: Result) -> Result {
        Result({
            resultType: typeRep(itype),
            resultMutable: typeMutable(itype),
            result: expr
            })
    }

    fn unimplemented(node: EnvMonad) -> EnvMonad {
        noTranslation(node, "Corrode doesn\'t handle this yet".to_string())
    }

    fn uniqueName(base: EnvMonad) -> EnvMonad {
        modifyGlobal(Lambda)
    }

    fn useForwardRef(ident: EnvMonad) -> EnvMonad {
        modifyGlobal(Lambda)
    }

    fn usual(__0: Maybe) -> Maybe {
        match (__0, __1) {
            <todo> => { Just((IsFloat((max(aw, bw))))) },
            <todo> => { Just(a) },
            <todo> => { Just(b) },
            <todo> => { match (intPromote(origA), intPromote(origB)) {
                (a, b) => if (a == b) { Just(a) },
                    (IsInt(Signed, sw), IsInt(Unsigned, uw)) => { mixedSign(sw, uw) },
                    (IsInt(Unsigned, uw), IsInt(Signed, sw)) => { mixedSign(sw, uw) },
                    (IsInt(as, aw), IsInt(_bs, bw)) => { {

                            let rank = integerConversionRank(aw, bw);
                            Just((IsInt(as, ((if(rank) == GT(then, aw, else, bw))))))
                    } },
                    _ => { Nothing },
                } },
        }
    }

    fn wrapMain(declr: EnvMonad) -> EnvMonad {
        {

                let (setup, args) = wrapArgv(argTypes);
                Let([Assign([Span([Ref(Ident("ret"))])], Span([Ref(Ident("Rust.VarName")), Str("ret")]))], []);
                emitItems(vec![Rust_Item(vec![], Rust_Private, (Rust_Function(vec![], "main".to_string(), vec![], (Rust_TypeName("()".to_string())), (statementsToBlock((++(setup, ++(vec![bind(Rust_Immutable, ret)(Rust_UnsafeExpr(Rust_Block(vec![])(Just(call(realName, args)))))], exprToStatements((call("::std::process::exit".to_string(), vec![Rust_Var(ret)])))))))))))]);
            ;
                            let wrapArgv = |vec![]| {
                    return((vec![], vec![]))
                };
;
            ;
                            let wrapArgv = |_| {
                    unimplemented(declr)
                };
;
                            let wrapEnvp = |vec![]| {
                    return((vec![], vec![]))
                };
;
            ;
                            let wrapEnvp = |_| {
                    unimplemented(declr)
                };

        }
    }

    fn wrapping(__0: Result) -> Result {
        match (__0, __1, __2) {
            <todo> => { match result(r) {
                    Rust_Add, lhs, rhs => { r({
                        result: Rust_MethodCall(lhs, (Rust_VarName("wrapping_add".to_string())), vec![rhs])
                        }) },
                    Rust_Sub, lhs, rhs => { r({
                        result: Rust_MethodCall(lhs, (Rust_VarName("wrapping_sub".to_string())), vec![rhs])
                        }) },
                    Rust_Mul, lhs, rhs => { r({
                        result: Rust_MethodCall(lhs, (Rust_VarName("wrapping_mul".to_string())), vec![rhs])
                        }) },
                    Rust_Div, lhs, rhs => { r({
                        result: Rust_MethodCall(lhs, (Rust_VarName("wrapping_div".to_string())), vec![rhs])
                        }) },
                    Rust_Mod, lhs, rhs => { r({
                        result: Rust_MethodCall(lhs, (Rust_VarName("wrapping_rem".to_string())), vec![rhs])
                        }) },
                    Rust_Neg, e => { r({
                        result: Rust_MethodCall(e, (Rust_VarName("wrapping_neg".to_string())), vec![])
                        }) },
                    _ => { r },
                } },
            <todo> => { r },
        }
    }

}

mod Language_Rust_Corrode_CFG {
    struct BasicBlock(BasicBlock, s, Terminator(c));

    #[derive(Debug)]
    enum Terminator'{
        Unreachable,
        Branch(l),
        CondBranch(c, l, l)
    };

    struct Unordered();

    struct DepthFirst();

    struct CFG(CFG, Label, IntMap.IntMap(BasicBlock(s, c)));

    struct BuildState(BuildState, { /* struct def */ });

    #[derive(Debug)]
    enum StructureLabel{
        GoTo({ /* struct def */ }),
        ExitTo({ /* struct def */ }),
        Nested(Vec<Structure(s, c)>)
    };

    #[derive(Debug)]
    enum Structure'{
        Simple(s, StructureTerminator(s, c)),
        Loop(a),
        Multiple(IntMap.IntMap(a), a)
    };

    #[derive(Debug)]
    struct Structure(Structure, { /* struct def */ });

        fn addBlock(label: Monad) -> Monad {
        {

                modify(Lambda({
        buildBlocks: IntMap_insert(label, (BasicBlock(stmt, terminator)), (buildBlocks(st)))
        }))
        }
    }

    fn buildCFG(root: Monad) -> Monad {
        {

                let (label, final) = runStateT(root, (BuildState(0, IntMap_empty)));
                return((CFG(label, (buildBlocks(final)))))
        }
    }

    fn depthFirstOrder((CFG(start, blocks)): CFG) -> CFG {
        CFG(start_q, blocks_q)
    }

    fn flipEdges(edges: IntMap.IntMap) -> IntMap.IntMap {
        IntMap_unionsWith(IntSet_union, Dummy)
    }

    fn hasMultiple() -> Bool {
        any(((go . structureBody)))
    }

    fn mapBuildCFGT() -> BuildCFGT {
        mapStateT
    }

    fn newLabel() -> Monad {
        {

                let old = get;
                put(old, {
    buildLabel: +(buildLabel(old), 1)
    });
                return((buildLabel(old)))
        }
    }

    fn outEdges(blocks: IntMap.IntMap) -> IntMap.IntMap {
        IntSet.difference(IntSet_unions((map(successors)(IntMap_elems(blocks)))), IntMap_keysSet(blocks))
    }

    fn partitionMembers(a: (IntSet.IntSet, IntSet.IntSet)) -> (IntSet.IntSet, IntSet.IntSet) {
        (IntSet.intersection(a, b), IntSet.difference(a, b))
    }

    fn prettyCFG(fmtS: CFG) -> CFG {
        vcat(:((<>(text("start @".to_string()), text((show(entry))))), blocks_q))
    }

    fn prettyStructure() -> Doc {
        (vcat . map(go))
    }

    fn relooper(entries: Monoid) -> Monoid {
        Let([Assign([Span([Parens([Span([Ref(Ident("returns"))]), Span([Ref(Ident("noreturns"))])])])], Span([Ref(Ident("partitionMembers")), Ref(Ident("entries")), Operator("$"), Ref(Ident("IntSet.unions")), Operator("$"), Ref(Ident("map")), Ref(Ident("successors")), Operator("$"), Ref(Ident("IntMap.elems")), Ref(Ident("blocks"))])), Assign([Span([Parens([Span([Ref(Ident("present"))]), Span([Ref(Ident("absent"))])])])], Span([Ref(Ident("partitionMembers")), Ref(Ident("entries")), Parens([Span([Ref(Ident("IntMap.keysSet")), Ref(Ident("blocks"))])])]))], [])(in, match (IntSet_toList(noreturns), IntSet_toList(returns)) {
                ([], []) => { vec![] },
                ([entry], []) => { match IntMap_updateLookupWithKey((Lambda), entry, blocks) {
                        (Just((s, term)), blocks_q) => { :(Structure({
                                structureEntries: entries,
                                structureBody: Simple(s, term)
                                }), relooper((successors((s, term))), blocks_q)) },
                        (Nothing, _) => { :(Structure({
                                structureEntries: entries,
                                structureBody: Simple(mempty, (Branch((GoTo(entry)))))
                                }), vec![]) },
                    } },
            _ => if not((IntSet_null(absent))) { :(if(IntSet_null, present, then, vec![], else, Structure, {
                    structureEntries: entries,
                    structureBody: Multiple((IntMap_fromSet((const(vec![])), absent)), (relooper(present, blocks)))
                    }), vec![]) },
                ([], _) => { :(Structure({
                        structureEntries: entries,
                        structureBody: Loop((relooper(entries, blocks_q)))
                        }), relooper(followEntries, followBlocks)) },
                _ => { :(Structure({
                        structureEntries: entries,
                        structureBody: Multiple(handlers, unhandled)
                        }), relooper(followEntries, followBlocks)) },
            })
    }

    fn relooperRoot((CFG(entry, blocks)): Monoid) -> Monoid {
        relooper((IntSet_singleton(entry)))(IntMap_map((Lambda), blocks))
    }

    fn removeEmptyBlocks((CFG(start, blocks)): Foldable) -> Foldable {
        CFG((rewrite(start)), blocks_q)
    }

    fn restrictKeys(m: IntMap.IntMap) -> IntMap.IntMap {
        IntMap.intersection(m, IntMap_fromSet((const(())), s))
    }

    fn simplifyStructure() -> Monoid {
        (foldr(go, vec![]) . map(descend))
    }

    fn structureCFG(mkBreak: Monoid) -> Monoid {
        (hasMultiple(root), foo(vec![], mempty, root))
    }

    fn successors((_, term): StructureBlock) -> StructureBlock {
        IntSet_fromList(Dummy)
    }

}

mod Language_Rust_Corrode_CrateMap {
    #[derive(Debug, Eq, Ord)]
    enum ItemKind{
        Enum,
        Struct,
        Union,
        Type,
        Symbol
    };

        fn mergeCrateMaps() -> Map.Map {
        Map_fromListWith((Map_unionWith((Operator("++")))))
    }

    fn parseCrateMap() -> Either {
        (fmap(root) . (foldrM(parseLine, (Map_empty, vec![])) . (filter(((not . null))) . (map(cleanLine) . lines))))
    }

    fn rewritesFromCratesMap(crates: ItemRewrites) -> ItemRewrites {
        Map_fromList(Dummy)
    }

    fn splitModuleMap(modName: (ModuleMap, CratesMap)) -> (ModuleMap, CratesMap) {
        fromMaybe((vec![], crates))({

                    let thisCrate = Map_lookup("".to_string(), crates);
                    let thisModule = Map_lookup(modName, thisCrate);
                    Let([Assign([Span([Ref(Ident("thisCrate\'"))])], Span([Ref(Ident("Map.delete")), Ref(Ident("modName")), Ref(Ident("thisCrate"))]))], []);
                    Let([Assign([Span([Ref(Ident("crates\'"))])], Span([Ref(Ident("Map.insert")), Str(""), Ref(Ident("thisCrate\'")), Ref(Ident("crates"))]))], []);
                    return((thisModule, crates_q))
            })
    }

}

mod Language_Rust_Idiomatic {
        fn itemIdioms(__0: Rust.Item) -> Rust.Item {
        match (__0) {
            <todo> => { Rust_Item(attrs, vis, (Rust_Function(fattrs, name, formals, ret, (tailBlock(b))))) },
            <todo> => { i },
        }
    }

    fn tailBlock(__0: Rust.Block) -> Rust.Block {
        match (__0) {
            <todo> => { Rust_Block(b, e) },
            <todo> => { Rust_Block(b, e) },
            <todo> => { b },
        }
    }

    fn tailExpr(__0: Maybe) -> Maybe {
        match (__0) {
            <todo> => { Just(e) },
            <todo> => { Just((Just((Rust_BlockExpr((tailBlock(b))))))) },
            <todo> => { Just((Just((Rust_IfThenElse(c, (tailBlock(t)), (tailBlock(f))))))) },
            <todo> => { Nothing },
        }
    }

    fn unsnoc(__0: Maybe) -> Maybe {
        match (__0) {
            <todo> => { Nothing },
            <todo> => { match unsnoc(xs) {
                    Just, (a, b) => { Just((x:a, b)) },
                    Nothing => { Just((vec![], x)) },
                } },
        }
    }

}

mod Language_Rust {
    
}



fn main() { /* demo */ }
