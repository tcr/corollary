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
        Let(Mutable, Var, Maybe<Type>, Maybe<Expr>),
        StmtItem(Vec<Attribute>, ItemKind)
    }

    #[derive(Debug)]
    struct Block(Block, Vec<Stmt>, Maybe<Expr>);

    #[derive(Debug)]
    struct Attribute(Attribute, String);

    #[derive(Debug)]
    struct Item(Item, Vec<Attribute>, Visibility, ItemKind);

    #[derive(Debug)]
    enum FunctionAttribute {
        UnsafeFn,
        ExternABI(Maybe<String>)
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
        StructExpr(String, Vec<(String, Expr)>, Maybe<Expr>),
        Call(Expr, Vec<Expr>),
        MethodCall(Expr, Var, Vec<Expr>),
        Lambda(Vec<Var>, Expr),
        Member(Expr, Var),
        BlockExpr(Block),
        UnsafeExpr(Block),
        IfThenElse(Expr, Block, Block),
        Loop(Maybe<Lifetime>, Block),
        While(Maybe<Lifetime>, Expr, Block),
        For(Maybe<Lifetime>, Var, Expr, Block),
        Break(Maybe<Lifetime>),
        Continue(Maybe<Lifetime>),
        Return(Maybe<Expr>),
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
    }

    #[derive(Debug)]
    enum ExprPosition {
        TopExpr,
        LeftExpr,
        RightExpr
    }

    fn pPrintBlock(__0: Doc) -> Doc {
        match (__0, __1) {
            pre, Block([], e) => { sep(vec![<+>(pre, text("{".to_string())), nest(4, (maybe(empty, pPrint, e))), text("}".to_string())]) },
            pre, Block(ss, e) => { <+>(pre, $+$(text("{".to_string()), $+$(nest(4, (vcat((__op_concat(map(pPrint, ss), vec![maybe(empty, pPrint, e)]))))), text("}".to_string())))) },
        }
    }

}

mod Language_Rust_Corrode_C {
    struct FunctionContext(FunctionContext, { /* struct def */ });

    struct Output(Output, { /* struct def */ });

    struct GlobalState(GlobalState, { /* struct def */ });

    struct EnvState(EnvState, { /* struct def */ });

    struct Initializer(Initializer, Maybe<Rust.Expr>, IntMap.IntMap<Initializer>);

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
        IsFunc(CType, Vec<(Maybe<(Rust.Mutable, Ident)>, CType)>, Bool),
        IsPtr(Rust.Mutable, CType),
        IsArray(Rust.Mutable, isize, CType),
        IsStruct(String, Vec<(String, CType)>),
        IsEnum(String),
        IsIncomplete(Ident)
    }

    struct IntermediateType(IntermediateType, { /* struct def */ });

    fn addExternIdent(ident: EnvMonad) -> EnvMonad {
        /* do */ {

            let action = runOnce(/* do */ {

            let itype = deferred;
            let rewrites = lift(asks(itemRewrites));
            let path = match Map_lookup((Symbol, identToString(ident)), rewrites) {
            Just, renamed => { return((:("".to_string(), renamed))) },
            Nothing => { /* do */ {

                {

            let name = || {
            applyRenames(ident)
        };

};
                {

            let ty = || {
            (typeMutable(itype), typeRep(itype))
        };

};
                lift(tell(mempty, {
        outputExterns: Map_singleton(name, (mkItem(name, ty)))
        }));
                return(vec![name])} },
        };
            return((typeToResult(itype, (Rust_Path((Rust_PathSegments(path)))))))});
            addSymbolIdentAction(ident, action)}
    }

    fn addSwitchCase(condition: Maybe) -> Maybe {
        /* do */ {

            let condition_q = lift(lift(mapM((interpretExpr(True)), condition)));
            let next_q = interpretStatement(body, next);
            let label = match next_q {
            ([], Branch(to)) => { return(to) },
            (rest, end) => { /* do */ {

                let label = newLabel;
                addBlock(label, rest, end);
                return(label)} },
        };
            lift(tell(SwitchCases(IntMap_singleton(label, condition_q))));
            return((vec![], Branch(label)))}
    }

    fn addSymbolIdent(ident: EnvMonad) -> EnvMonad {
        /* do */ {

            {

            let name = || {
            applyRenames(ident)
        };

};
            addSymbolIdentAction(ident)(return(Result, {
        resultType: ty,
        resultMutable: mut,
        result: Rust_Path((Rust_PathSegments(vec![name])))
        }));
            return(name)}
    }

    fn addSymbolIdentAction(ident: EnvMonad) -> EnvMonad {
        lift(/* do */ {

                modify(Lambda({
        symbolEnvironment: :((ident, action), symbolEnvironment(st))
        }))})
    }

    fn addTagIdent(ident: EnvMonad) -> EnvMonad {
        lift(/* do */ {

                modify(Lambda({
        tagEnvironment: :((ident, ty), tagEnvironment(st))
        }))})
    }

    fn addTypedefIdent(ident: EnvMonad) -> EnvMonad {
        lift(/* do */ {

                modify(Lambda({
        typedefEnvironment: :((ident, ty), typedefEnvironment(st))
        }))})
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
        noTranslation(node, (__op_concat("illegal ".to_string(), __op_concat(msg, "; check whether a real C compiler accepts this".to_string()))))
    }

    fn baseTypeOf(specs: EnvMonad) -> EnvMonad {
        /* do */ {

            {

            let (storage, _attributes, basequals, basespecs, _inlineNoReturn, _align) = || {
            partitionDeclSpecs(specs)
        };

};
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
                            (Just(lhs_q), Just(rhs_q)) => { /* do */ {

                                let ptrTo = match compatiblePtr((resultType(lhs_q)), (resultType(rhs_q))) {
            IsPtr, _, ptrTo => { return(ptrTo) },
            _ => { badSource(expr, "pointer subtraction of incompatible pointers".to_string()) },
        };
                                {

            let ty = || {
            IsInt(Signed, WordWidth)
        };

};
                                {

            let size = || {
            rustSizeOfType((toRustType(ptrTo)))
        };

};
                                return(Result, {
    resultType: ty,
    resultMutable: Rust_Immutable,
    result: /((Rust_MethodCall((castTo(ty, lhs_q)), (Rust_VarName("wrapping_sub".to_string())), vec![castTo(ty, rhs_q)])), castTo(ty, size))
    })} },
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
            wordWidth, WordWidth => { wordWidth },
            _, BitWidth(w) => { w },
        }
    }

    fn blockToStatements((Rust_Block(stmts, mexpr)): Vec<Rust.Stmt>) -> Vec<Rust.Stmt> {
        match mexpr {
                Just, expr => { __op_concat(stmts, exprToStatements(expr)) },
                Nothing => { stmts },
            }
    }

    fn castTo(__0: Rust.Expr) -> Rust.Expr {
        match (__0, __1) {
            target, Result(<todo>) => { castTo(target, Result, {
                resultType: IsPtr(mut, el),
                resultMutable: Rust_Immutable,
                result: Rust_MethodCall(source, (Rust_VarName(method)), vec![])
                }) },
            IsBool, source => { toBool(source) },
            target, <todo>, IsInt(<todo>), Result(<todo>) => { Rust_Lit((Rust_LitInt(n, repr, (toRustType(target))))) },
            IsInt(Signed, w), Result(<todo>) => { Rust_Neg((Rust_Lit((Rust_LitInt(n, repr, (toRustType((IsInt(Signed, w))))))))) },
            target, source => { Rust_Cast((result(source)), (toRustType(target))) },
        }
    }

    fn cfgToRust(_node: CSourceBuildCFGT) -> CSourceBuildCFGT {
        /* do */ {

            {

            let builder = || {
            buildCFG(/* do */ {

                    let (early, term) = build;
                    let entry = newLabel;
                    addBlock(entry, early, term);
                    return(entry)})
        };

};
            let (rawCFG, _) = evalRWST(builder, (OuterLabels(Nothing, Nothing, Nothing)), Map_empty);
            {

            let cfg = || {
            depthFirstOrder((removeEmptyBlocks(rawCFG)))
        };

};
            {

            let (hasGoto, structured) = || {
            structureCFG(mkBreak, mkContinue, mkLoop, mkIf, mkGoto, mkMatch, cfg)
        };

};
            return(:(if(hasGoto, then, declCurrent), structured(else, structured)));
        }
    }

    fn charType() -> CType {
        IsInt(Unsigned, (BitWidth(8)))
    }

    fn compatibleInitializer(__0: Bool) -> Bool {
        match (__0, __1) {
            IsStruct(name1, _), IsStruct(name2, _) => { (name1 == name2) },
            IsStruct, <todo>, _ => { False },
            _, IsStruct, <todo> => { False },
            _, _ => { True },
        }
    }

    fn compatiblePtr(__0: CType) -> CType {
        match (__0, __1) {
            IsPtr(_, IsVoid), b => { b },
            IsArray(mut, _, el), b => { compatiblePtr((IsPtr(mut, el)), b) },
            a, IsPtr(_, IsVoid) => { a },
            a, IsArray(mut, _, el) => { compatiblePtr(a, (IsPtr(mut, el))) },
            IsPtr(m1, a), IsPtr(m2, b) => { IsPtr((leastMutable(m1, m2)), (compatiblePtr(a, b))) },
            _, _ => { IsVoid },
        }
    }

    fn completeType(__0: EnvMonad) -> EnvMonad {
        match (__0, __1, __2) {
            orig, <todo>, IsIncomplete(ident) => { /* do */ {

                let mty = getTagIdent(ident);
                fromMaybe((return(orig)), mty)} },
            ty => { return(ty) },
        }
    }

    fn compound(expr: EnvMonad) -> EnvMonad {
        /* do */ {

            {

            let op_q = || {
            match op {
                    CAssignOp => { Nothing },
                    CMulAssOp => { Just(CMulOp) },
                    CDivAssOp => { Just(CDivOp) },
                    CRmdAssOp => { Just(CRmdOp) },
                    CAddAssOp => { Just(CAddOp) },
                    CSubAssOp => { Just(CSubOp) },
                    CShlAssOp => { Just(CShlOp) },
                    CShrAssOp => { Just(CShrOp) },
                    CAndAssOp => { Just(CAndOp) },
                    CXorAssOp => { Just(CXorOp) },
                    COrAssOp => { Just(COrOp) },
                }
        };

};
            {

            let duplicateLHS = || {
            ||(isJust(op_q), demand)
        };

};
            {

            let (bindings1, dereflhs, boundrhs) = || {
            ||(if(not, duplicateLHS), hasNoSideEffects((result(lhs)), then, (vec![], lhs, rhs), else, {

                                            let lhsvar = || {
                            Rust_VarName("_lhs".to_string())
                        };
;
                                            let rhsvar = || {
                            Rust_VarName("_rhs".to_string())
                        };

                }, in, (vec![Rust_Let(Rust_Immutable, rhsvar, Nothing, (Just((result(rhs))))), Rust_Let(Rust_Immutable, lhsvar, Nothing, (Just((Rust_Borrow(Rust_Mutable, (result(lhs)))))))], lhs({
                        result: Rust_Deref((Rust_Var(lhsvar)))
                        }), rhs({
                        result: Rust_Var(rhsvar)
                        }))))
        };

};
            let rhs_q = match op_q {
            Just, o => { binop(expr, o, dereflhs, boundrhs) },
            Nothing => { return(boundrhs) },
        };
            {

            let assignment = || {
            Rust_Assign((result(dereflhs)), (Rust_:=), (castTo((resultType(lhs)), rhs_q)))
        };

};
            {

            let (bindings2, ret) = || {
            if(not, demand, then, (vec![], Nothing), else, if, not, returnOld, then, (vec![], Just((result(dereflhs)))), else, {

                                    let oldvar = || {
                        Rust_VarName("_old".to_string())
                    };

            }, in, (vec![Rust_Let(Rust_Immutable, oldvar, Nothing, (Just((result(dereflhs)))))], Just((Rust_Var(oldvar)))))
        };

};
            return(match Rust_Block((__op_concat(bindings1, __op_concat(bindings2, exprToStatements(assignment)))), ret) {
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
        /* do */ {

            let derived_q = mapM(derive, derived);
            return(/* do */ {

        let basetype = deferred;
        foldrM((Operator("$")), basetype, derived_q)});
        }
    }

    fn derivedTypeOf(deferred: EnvMonad) -> EnvMonad {
        join((derivedDeferredTypeOf(deferred, declr, vec![])))
    }

    fn designatorType(__0: CType) -> CType {
        match (__0) {
            Base(ty) => { ty },
            From(ty, _, _, _) => { ty },
        }
    }

    fn emitIncomplete(kind: EnvMonad) -> EnvMonad {
        /* do */ {

            let rewrites = lift((asks(itemRewrites)));
            unless((Map_member((kind, identToString(ident)), rewrites)))(lift(tell(mempty, {
            outputIncomplete: Set_singleton((identToString(ident)))
            })));
            return((IsIncomplete(ident)))}
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
            Rust_IfThenElse(c, t, f) => { vec![Rust_Stmt((Rust_IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))] },
            Rust_BlockExpr(b) => { blockToStatements(b) },
            e => { vec![Rust_Stmt(e)] },
        }
    }

    fn getSwitchCases(expr: CSourceBuildCFGT) -> CSourceBuildCFGT {
        mapBuildCFGT(wrap)
    }

    fn getSwitchExpression(stmt: CSourceBuildCFGT) -> CSourceBuildCFGT {
        /* do */ {

            let mexpr = lift(asks(switchExpression));
            match mexpr {
        Nothing => { lift(lift(badSource(stmt, "case outside switch".to_string()))) },
        Just, expr => { return(expr) },
    }}
    }

    fn getSymbolIdent(ident: EnvMonad) -> EnvMonad {
        /* do */ {

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
        lift(/* do */ {

                let env = gets(tagEnvironment);
                return(lookup(ident, env))})
    }

    fn getTypedefIdent(ident: EnvMonad) -> EnvMonad {
        lift(/* do */ {

                let env = gets(typedefEnvironment);
                return((identToString(ident), lookup(ident, env)))})
    }

    fn gotoLabel(ident: CSourceBuildCFGT) -> CSourceBuildCFGT {
        /* do */ {

            let labels = lift(get);
            match Map_lookup(ident, labels) {
        Nothing => { /* do */ {

            let label = newLabel;
            lift((put((Map_insert(ident, label, labels)))));
            return(label)} },
        Just, label => { return(label) },
    }}
    }

    fn intPromote(__0: CType) -> CType {
        match (__0) {
            IsBool => { IsInt(Signed, (BitWidth(32))) },
            IsEnum(_) => { enumReprType },
            x => { x },
        }
    }

    fn integerConversionRank(__0: Maybe) -> Maybe {
        match (__0, __1) {
            BitWidth(a), BitWidth(b) => { Just((compare(a, b))) },
            WordWidth, WordWidth => { Just(EQ) },
            _, _ => { Nothing },
        }
    }

    fn interpretBlockItem(__0: CSourceBuildCFGT) -> CSourceBuildCFGT {
        match (__0, __1) {
            CBlockStmt(stmt), next => { interpretStatement(stmt, next) },
            CBlockDecl(decl), next => { /* do */ {

                let decl_q = lift(lift((interpretDeclarations(makeLetBinding, decl))));
                let (rest, end) = next;
                return((__op_concat(decl_q, rest), end))} },
            item, _ => { lift(lift((unimplemented(item)))) },
        }
    }

    fn interpretConstExpr(__0: EnvMonad) -> EnvMonad {
        match (__0) {
            CConst(CIntConst(CInteger(v, _, _), _)) => { return(v) },
            expr => { unimplemented(expr) },
        }
    }

    fn interpretDeclarations(__0: MakeBinding) -> MakeBinding {
        match (__0, __1, __2, __3) {
            fromItem(makeBinding), declaration, <todo>, CDecl(specs, decls, _) => { /* do */ {

                let (storagespecs, baseTy) = baseTypeOf(specs);
                let mbinds = forM(decls)(Lambda);
                return((catMaybes(mbinds)))} },
            _, node, <todo>, CStaticAssert(<todo>) => { unimplemented(node) },
        }
    }

    fn interpretExpr(__0: EnvMonad) -> EnvMonad {
        match (__0, __1) {
            demand, CComma(exprs, _) => { /* do */ {

                {

            let (effects, mfinal) = || {
            if(demand, then, (init(exprs), Just((last(exprs)))), else, (exprs, Nothing))
        };

};
                let effects_q = mapM(((fmap(resultToStatements) . interpretExpr(False))), effects);
                let mfinal_q = mapM((interpretExpr(True)), mfinal);
                return(Result, {
    resultType: maybe(IsVoid, resultType, mfinal_q),
    resultMutable: maybe(Rust_Immutable, resultMutable, mfinal_q),
    result: Rust_BlockExpr((Rust_Block((concat(effects_q)), (fmap(result, mfinal_q)))))
    })} },
            demand, expr, <todo>, CAssign(op, lhs, rhs, _) => { /* do */ {

                let lhs_q = interpretExpr(True, lhs);
                let rhs_q = interpretExpr(True, rhs);
                compound(expr, False, demand, op, lhs_q, rhs_q)} },
            demand, expr, <todo>, CCond(c, Just(t), f, _) => { /* do */ {

                let c_q = fmap(toBool, (interpretExpr(True, c)));
                let t_q = interpretExpr(demand, t);
                let f_q = interpretExpr(demand, f);
                if(demand, then, promotePtr, expr, (mkIf(c_q)), t_q, f_q, else, return, Result, {
    resultType: IsVoid,
    resultMutable: Rust_Immutable,
    result: mkIf(c_q, (result(t_q)), (result(f_q)))
    });
            } },
            _, expr, <todo>, CBinary(op, lhs, rhs, _) => { /* do */ {

                let lhs_q = interpretExpr(True, lhs);
                let rhs_q = interpretExpr(True, rhs);
                binop(expr, op, lhs_q, rhs_q)} },
            _, CCast(decl, expr, _) => { /* do */ {

                let (_mut, ty) = typeName(decl);
                let expr_q = interpretExpr((/=(ty, IsVoid)), expr);
                return(Result, {
    resultType: ty,
    resultMutable: Rust_Immutable,
    result: ((if(ty) == IsVoid(then, result, else, castTo, ty)))(expr_q)
    })} },
            demand, node, <todo>, CUnary(op, expr, _) => { match op {
                    CPreIncOp => { incdec(False, CAddAssOp) },
                    CPreDecOp => { incdec(False, CSubAssOp) },
                    CPostIncOp => { incdec(True, CAddAssOp) },
                    CPostDecOp => { incdec(True, CSubAssOp) },
                    CAdrOp => { /* do */ {

                        let expr_q = interpretExpr(True, expr);
                        {

            let ty_q = || {
            IsPtr((resultMutable(expr_q)), (resultType(expr_q)))
        };

};
                        return(Result, {
    resultType: ty_q,
    resultMutable: Rust_Immutable,
    result: Rust_Cast((Rust_Borrow((resultMutable(expr_q)), (result(expr_q)))), (toRustType(ty_q)))
    })} },
                    CIndOp => { /* do */ {

                        let expr_q = interpretExpr(True, expr);
                        match resultType(expr_q) {
        IsPtr, mut_q, ty_q => { return(Result, {
            resultType: ty_q,
            resultMutable: mut_q,
            result: Rust_Deref((result(expr_q)))
            }) },
        IsFunc, { .. } => { return(expr_q) },
        _ => { badSource(node, "dereference of non-pointer".to_string()) },
    }} },
                    CPlusOp => { /* do */ {

                        let expr_q = interpretExpr(demand, expr);
                        {

            let ty_q = || {
            intPromote((resultType(expr_q)))
        };

};
                        return(Result, {
    resultType: ty_q,
    resultMutable: Rust_Immutable,
    result: castTo(ty_q, expr_q)
    })} },
                    CMinOp => { fmap(wrapping)(simple(Rust_Neg)) },
                    CCompOp => { simple(Rust_Not) },
                    CNegOp => { /* do */ {

                        let expr_q = interpretExpr(True, expr);
                        return(Result, {
    resultType: IsBool,
    resultMutable: Rust_Immutable,
    result: toNotBool(expr_q)
    })} },
                } },
            _, CSizeofExpr(e, _) => { /* do */ {

                let e_q = interpretExpr(True, e);
                return((rustSizeOfType((toRustType((resultType(e_q)))))))} },
            _, CSizeofType(decl, _) => { /* do */ {

                let (_mut, ty) = typeName(decl);
                return((rustSizeOfType((toRustType(ty)))))} },
            _, CAlignofExpr(e, _) => { /* do */ {

                let e_q = interpretExpr(True, e);
                return((rustAlignOfType((toRustType((resultType(e_q)))))))} },
            _, CAlignofType(decl, _) => { /* do */ {

                let (_mut, ty) = typeName(decl);
                return((rustAlignOfType((toRustType(ty)))))} },
            _, expr, <todo>, CIndex(lhs, rhs, _) => { /* do */ {

                let lhs_q = interpretExpr(True, lhs);
                let rhs_q = interpretExpr(True, rhs);
                match (resultType(lhs_q), resultType(rhs_q)) {
        (IsArray(mut, _, el), _) => { return((subscript(mut, el, (result(lhs_q)), rhs_q))) },
        (_, IsArray(mut, _, el)) => { return((subscript(mut, el, (result(rhs_q)), lhs_q))) },
        _ => { /* do */ {

            let ptr = binop(expr, CAddOp, lhs_q, rhs_q);
            match resultType(ptr) {
        IsPtr, mut, ty => { return(Result, {
            resultType: ty,
            resultMutable: mut,
            result: Rust_Deref((result(ptr)))
            }) },
        _ => { badSource(expr, "array subscript of non-pointer".to_string()) },
    }} },
    };
            } },
            _, expr, <todo>, CCall(func, args, _) => { /* do */ {

                let func_q = interpretExpr(True, func);
                match resultType(func_q) {
        IsFunc, retTy, argTys, variadic => { /* do */ {

            let args_q = castArgs(variadic, (map(snd, argTys)), args);
            return(Result, {
    resultType: retTy,
    resultMutable: Rust_Immutable,
    result: Rust_Call((result(func_q)), args_q)
    })} },
        _ => { badSource(expr, "function call to non-function".to_string()) },
    };
            } },
            _, expr, <todo>, CMember(obj, ident, deref, node) => { /* do */ {

                let obj_q = interpretExpr(True)(if(deref, then, CUnary, CIndOp, obj, node, else, obj));
                let objTy = completeType((resultType(obj_q)));
                let fields = match objTy {
            IsStruct, _, fields => { return(fields) },
            _ => { badSource(expr, "member access of non-struct".to_string()) },
        };
                {

            let field = || {
            applyRenames(ident)
        };

};
                let ty = match lookup(field, fields) {
            Just, ty => { return(ty) },
            Nothing => { badSource(expr, "request for non-existent field".to_string()) },
        };
                return(Result, {
    resultType: ty,
    resultMutable: resultMutable(obj_q),
    result: Rust_Member((result(obj_q)), (Rust_VarName(field)))
    })} },
            _, expr, <todo>, CVar(ident, _) => { /* do */ {

                let sym = getSymbolIdent(ident);
                maybe((badSource(expr, "undefined variable".to_string())), return, sym)} },
            _, expr, <todo>, CConst(c) => { match c {
                    CIntConst, CInteger(v, repr, flags), _ => { {

                                            let allow_signed = || {
                            not((testFlag(FlagUnsigned, flags)))
                        };
;
                                            let allow_unsigned = || {
                            ||(not(allow_signed), /=(repr, DecRepr))
                        };
;
                                            let widths = || {
                            vec![(32, if(any, (Operator("testFlag")(flags)), vec![FlagLongLong, FlagLong], then, WordWidth, else, BitWidth, 32)), (64, BitWidth(64))]
                        };
;
                                            let allowed_types = || {
                            Dummy
                        };
;
                                            let repr_q = || {
                            match repr {
                                    DecRepr => { Rust_DecRepr },
                                    OctalRepr => { Rust_OctalRepr },
                                    HexRepr => { Rust_HexRepr },
                                }
                        };

                }(in, match allowed_types {
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
                        result: Rust_Deref((Rust_Lit((Rust_LitByteStr((__op_concat(str, "\u{0}".to_string())))))))
                        }) },
                    _ => { unimplemented(expr) },
                } },
            _, CCompoundLit(decl, initials, info) => { /* do */ {

                let (mut, ty) = typeName(decl);
                let final = interpretInitializer(ty, (CInitList(initials, info)));
                return(Result, {
    resultType: ty,
    resultMutable: mut,
    result: final
    })} },
            demand, stat, <todo>, CStatExpr(CCompound([], stmts, _), _) => { scope(/* do */ {

                    {

            let (effects, final) = || {
            match last(stmts) {
                CBlockStmt, CExpr(expr, _) => if demand { (init(stmts), expr) },
                    _ => { (stmts, Nothing) },
                }
        };

};
                    let effects_q = cfgToRust(stat, (foldr(interpretBlockItem, (return((vec![], Unreachable))), effects)));
                    let final_q = mapM((interpretExpr(True)), final);
                    return(Result, {
    resultType: maybe(IsVoid, resultType, final_q),
    resultMutable: maybe(Rust_Immutable, resultMutable, final_q),
    result: Rust_BlockExpr((Rust_Block(effects_q, (fmap(result, final_q)))))
    })}) },
            _, expr => { unimplemented(expr) },
        }
    }

    fn interpretFunction((@(CFunDef(specs, declr), (CDeclr(mident, _, _, _, _))(argtypes, body, _))): EnvMonad) -> EnvMonad {
        /* do */ {

            let (storage, baseTy) = baseTypeOf(specs);
            let (attrs, vis) = match storage {
            Nothing => { return((vec![Rust_Attribute("no_mangle".to_string())], Rust_Public)) },
            Just, CStatic(_) => { return((vec![], Rust_Private)) },
            Just, s => { badSource(s, "storage class specifier for function".to_string()) },
        };
            {

            let go = |name, funTy| {
            /* do */ {

                let (retTy, args) = match funTy {
            IsFunc, _, _, True => { unimplemented(declr) },
            IsFunc, retTy, args, False => { return((retTy, args)) },
            _ => { badSource(declr, "function definition".to_string()) },
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

            let returnValue = || {
            (if(name) == "_c_main".to_string()(then, Just, 0, else, Nothing))
        };
;
            let returnStatement = || {
            Rust_Stmt((Rust_Return(returnValue)))
        };

};
                let body_q = cfgToRust(declr, (interpretStatement(body, (return((vec![returnStatement], Unreachable))))));
                return((Rust_Item(attrs, vis, (Rust_Function(vec![Rust_UnsafeFn, Rust_ExternABI(Nothing)], name, formals, (toRustRetType(retTy)), (statementsToBlock(body_q)))))))}));
                emitItems(vec![f_q])}
        };

};
            let ident = match mident {
            Nothing => { badSource(declr, "anonymous function definition".to_string()) },
            Just, ident => { return(ident) },
        };
            {

            let name = || {
            applyRenames(ident)
        };

};
            {

            let funTy = |itype| {
            typeToResult(itype, (Rust_Path((Rust_PathSegments(vec![name])))))
        };

};
            let deferred = fmap((fmap(funTy)), (derivedDeferredTypeOf(baseTy, declr, argtypes)));
            let alreadyUsed = lift(gets(((usedForwardRefs . globalState))));
            match vis {
    Rust_Private => if Set.notMember(ident, alreadyUsed) { /* do */ {

        let action = runOnce(/* do */ {

            let ty = deferred;
            go(name, (resultType(ty)));
            return(ty)});
        addSymbolIdentAction(ident, action)} },
        _ => { /* do */ {

            let ty = deferred;
            addSymbolIdentAction(ident)(return(ty));
            go(name, (resultType(ty)))} },
    }}
    }

    fn interpretInitializer(ty: EnvMonad) -> EnvMonad {
        /* do */ {

            let initial_q = match initial {
            CInitExpr, expr, _ => { /* do */ {

                let expr_q = interpretExpr(True, expr);
                compatibleInitializer(if(resultType, expr_q), ty(then, pure)(scalar((castTo(ty, expr_q)), else, badSource, initial, "initializer for incompatible type".to_string())))} },
            CInitList, list, _ => { translateInitList(ty, list) },
        };
            let zeroed = zeroInitialize(initial_q, ty);
            helper(ty, zeroed);
        }
    }

    fn interpretStatement(__0: CSourceBuildCFGT) -> CSourceBuildCFGT {
        match (__0, __1) {
            CLabel(ident, body, _, _), next => { /* do */ {

                let label = gotoLabel(ident);
                let (rest, end) = interpretStatement(body, next);
                addBlock(label, rest, end);
                return((vec![], Branch(label)))} },
            stmt, <todo>, CCase(expr, body, node), next => { /* do */ {

                let selector = getSwitchExpression(stmt);
                {

            let condition = || {
            CBinary(CEqOp, selector, expr, node)
        };

};
                addSwitchCase((Just(condition)), body, next)} },
            stmt, <todo>, CCases(lower, upper, body, node), next => { /* do */ {

                let selector = getSwitchExpression(stmt);
                {

            let condition = || {
            CBinary(CLndOp, (CBinary(CGeqOp, selector, lower, node)), (CBinary(CLeqOp, selector, upper, node)), node)
        };

};
                addSwitchCase((Just(condition)), body, next)} },
            CDefault(body, _), next => { addSwitchCase(Nothing, body, next) },
            CExpr(Nothing, _), next => { next },
            CExpr(Just(expr), _), next => { /* do */ {

                let expr_q = lift(lift(interpretExpr(False, expr)));
                let (rest, end) = next;
                return((__op_concat(resultToStatements(expr_q), rest), end))} },
            CCompound([], items, _), next => { mapBuildCFGT((mapRWST(scope)))(/* do */ {

                    foldr(interpretBlockItem, next, items)}) },
            CIf(c, t, mf, _), next => { /* do */ {

                let c_q = lift(lift(interpretExpr(True, c)));
                let after = newLabel;
                let falseLabel = match mf {
            Nothing => { return(after) },
            Just, f => { /* do */ {

                let (falseEntry, falseTerm) = interpretStatement(f, (return((vec![], Branch(after)))));
                let falseLabel = newLabel;
                addBlock(falseLabel, falseEntry, falseTerm);
                return(falseLabel)} },
        };
                let (trueEntry, trueTerm) = interpretStatement(t, (return((vec![], Branch(after)))));
                let trueLabel = newLabel;
                addBlock(trueLabel, trueEntry, trueTerm);
                let (rest, end) = next;
                addBlock(after, rest, end);
                return((vec![], CondBranch(c_q, trueLabel, falseLabel)))} },
            stmt, <todo>, CSwitch(expr, body, node), next => { /* do */ {

                let (bindings, expr_q) = match expr {
            CVar, { .. } => { return((vec![], expr)) },
            _ => { lift(lift(/* do */ {

                        let ident = fmap(internalIdent, (uniqueName("switch".to_string())));
                        let rhs = interpretExpr(True, expr);
                        let var = addSymbolIdent(ident, (Rust_Immutable, resultType(rhs)));
                        return((vec![Rust_Let(Rust_Immutable, (Rust_VarName(var)), Nothing, (Just((result(rhs)))))], CVar(ident, node)))})) },
        };
                let after = newLabel;
                let (_, SwitchCases(cases)) = getSwitchCases(expr_q)(setBreak(after)(interpretStatement(body, (return((vec![], Branch(after)))))));
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

            let (conditions, defaults) = || {
            IntMap_mapEither(isDefault, cases)
        };

};
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
            CWhile(c, body, doWhile, _), next => { /* do */ {

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
                return((vec![], Branch((if(doWhile, then, bodyLabel, else, headerLabel)))))} },
            CFor(initial, mcond, mincr, body, _), next => { /* do */ {

                let after = newLabel;
                let ret = mapBuildCFGT((mapRWST(scope)))(/* do */ {

            let prefix = match initial {
            Left, Nothing => { return(vec![]) },
            Left, Just(expr) => { /* do */ {

                let expr_q = lift(lift(interpretExpr(False, expr)));
                return((resultToStatements(expr_q)))} },
            Right, decls => { lift(lift(interpretDeclarations(makeLetBinding, decls))) },
        };
            let headerLabel = newLabel;
            let incrLabel = match mincr {
            Nothing => { return(headerLabel) },
            Just, incr => { /* do */ {

                let incr_q = lift(lift(interpretExpr(False, incr)));
                let incrLabel = newLabel;
                addBlock(incrLabel, (resultToStatements(incr_q)), (Branch(headerLabel)));
                return(incrLabel)} },
        };
            let (bodyEntry, bodyTerm) = setBreak(after)(setContinue(incrLabel)(interpretStatement(body, (return((vec![], Branch(incrLabel)))))));
            let bodyLabel = newLabel;
            addBlock(bodyLabel, bodyEntry, bodyTerm);
            let cond = match mcond {
            Just, cond => { /* do */ {

                let cond_q = lift(lift(interpretExpr(True, cond)));
                return((CondBranch(cond_q, bodyLabel, after)))} },
            Nothing => { return((Branch(bodyLabel))) },
        };
            addBlock(headerLabel, vec![], cond);
            return((prefix, Branch(headerLabel)))});
                let (rest, end) = next;
                addBlock(after, rest, end);
                return(ret)} },
            CGoto(ident, _), next => { /* do */ {

                let _ = next;
                let label = gotoLabel(ident);
                return((vec![], Branch(label)))} },
            stmt, <todo>, CCont(_), next => { /* do */ {

                let _ = next;
                let val = lift((asks(onContinue)));
                match val {
        Just, label => { return((vec![], Branch(label))) },
        Nothing => { lift(lift(badSource(stmt, "continue outside loop".to_string()))) },
    }} },
            stmt, <todo>, CBreak(_), next => { /* do */ {

                let _ = next;
                let val = lift((asks(onBreak)));
                match val {
        Just, label => { return((vec![], Branch(label))) },
        Nothing => { lift(lift(badSource(stmt, "break outside loop".to_string()))) },
    }} },
            stmt, <todo>, CReturn(expr, _), next => { /* do */ {

                let _ = next;
                lift(lift(/* do */ {

            let val = lift((asks(functionReturnType)));
            match val {
        Nothing => { badSource(stmt, "return statement outside function".to_string()) },
        Just, retTy => { /* do */ {

            let expr_q = mapM(((fmap((castTo(retTy))) . interpretExpr(True))), expr);
            return((exprToStatements((Rust_Return(expr_q))), Unreachable))} },
    }}))} },
            stmt, _ => { lift(lift(unimplemented(stmt))) },
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
        lift(/* do */ {

                let st = get;
                {

            let (global_q, a) = || {
            f((globalState(st)))
        };

};
                put(st, {
    globalState: global_q
    });
                return(a)})
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
            Base, <todo> => { Nothing },
            From(_, i, ty(<todo>, remaining), base) => { Just((From(ty, (+(i, 1)), remaining, base))) },
            From(_, _, [], base) => { nextObject(base) },
        }
    }

    fn noTranslation(node: EnvMonad) -> EnvMonad {
        throwE(concat(vec![show((posOf(node))), ": ".to_string(), msg, ":\n".to_string(), render((nest(4, (pretty(node)))))]))
    }

    fn objectFromDesignators(__0: EnvMonad) -> EnvMonad {
        match (__0, __1) {
            _, [] => { pure(Nothing) },
            ty, desigs => { <$>(Just, go(ty, desigs, (Base(ty)))) },
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
                Just, obj => { /* do */ {

                    let (obj_q, initial) = match cinitial {
            CInitList, list_q, _ => { /* do */ {

                let initial = translateInitList((designatorType(obj)), list_q);
                return((obj, initial))} },
            CInitExpr, expr, _ => { /* do */ {

                let expr_q = interpretExpr(True, expr);
                match nestedObject((resultType(expr_q)), obj) {
        Nothing => { badSource(cinitial, "type in initializer".to_string()) },
        Just, obj_q => { /* do */ {

            {

            let s = || {
            castTo((designatorType(obj_q)), expr_q)
        };

};
            return((obj_q, scalar(s)))} },
    }} },
        };
                    {

            let indices = || {
            unfoldr((Lambda), obj_q)
        };

};
                    {

            let initializer = || {
            foldl((Lambda(Nothing, (IntMap_singleton(j, a)))), initial, indices)
        };

};
                    return((nextObject(obj_q), mappend(prior, initializer)))} },
            }
    }

    fn resultToStatements() -> Vec<Rust.Stmt> {
        (exprToStatements . result)
    }

    fn runOnce(action: EnvMonad) -> EnvMonad {
        /* do */ {

            let cacheRef = lift(lift(newSTRef((Left(action)))));
            return(/* do */ {

        let cache = lift(lift(readSTRef(cacheRef)));
        match cache {
        Left, todo => { /* do */ {

            lift(lift(writeSTRef(cacheRef)(Left(fail("internal error: runOnce action depends on itself, leading to an infinite loop".to_string())))));
            let val = todo;
            lift(lift(writeSTRef(cacheRef, (Right(val)))));
            return(val)} },
        Right, val => { return(val) },
    }})}
    }

    fn rustAlignOfType((Rust_TypeName(ty)): Result) -> Result {
        Result({
            resultType: IsInt(Unsigned, WordWidth),
            resultMutable: Rust_Immutable,
            result: Rust_Call((Rust_Var((Rust_VarName((__op_concat("::std::mem::align_of::<".to_string(), __op_concat(ty, ">".to_string()))))))), vec![])
            })
    }

    fn rustSizeOfType((Rust_TypeName(ty)): Result) -> Result {
        Result({
            resultType: IsInt(Unsigned, WordWidth),
            resultMutable: Rust_Immutable,
            result: Rust_Call((Rust_Var((Rust_VarName((__op_concat("::std::mem::size_of::<".to_string(), __op_concat(ty, ">".to_string()))))))), vec![])
            })
    }

    fn scalar(expr: Initializer) -> Initializer {
        Initializer((Just(expr)), IntMap_empty)
    }

    fn scope(m: EnvMonad) -> EnvMonad {
        /* do */ {

            let old = lift(get);
            let a = m;
            lift((modify((Lambda({
            globalState: globalState(st)
            })))));
            return(a)}
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
            [Rust_Stmt(Rust_BlockExpr(stmts))] => { stmts },
            stmts => { Rust_Block(stmts, Nothing) },
        }
    }

    fn toBool(__0: Rust.Expr) -> Rust.Expr {
        match (__0) {
            Result(<todo>) => { Rust_Lit((Rust_LitBool(False))) },
            Result(<todo>) => { Rust_Lit((Rust_LitBool(True))) },
            Result(<todo>) => { match t {
                    IsBool => { v },
                    IsPtr, _, _ => { Rust_Not((Rust_MethodCall(v, (Rust_VarName("is_null".to_string())), vec![]))) },
                    _ => { Rust_CmpNE(v, 0) },
                } },
        }
    }

    fn toNotBool(__0: Rust.Expr) -> Rust.Expr {
        match (__0) {
            Result(<todo>) => { Rust_Lit((Rust_LitBool(True))) },
            Result(<todo>) => { Rust_Lit((Rust_LitBool(False))) },
            Result(<todo>) => { match t {
                    IsBool => { Rust_Not(v) },
                    IsPtr, _, _ => { Rust_MethodCall(v, (Rust_VarName("is_null".to_string())), vec![]) },
                    _ => { Rust_CmpEQ(v, 0) },
                } },
        }
    }

    fn toPtr(__0: Maybe) -> Maybe {
        match (__0, __1, __2) {
            ptr, <todo>, Result(<todo>) => { Just(ptr, {
                resultType: IsPtr(mut, el),
                result: castTo((IsPtr(mut, el)), ptr)
                }) },
            ptr, <todo>, Result(<todo>) => { Just(ptr) },
            _ => { Nothing },
        }
    }

    fn toRustRetType(__0: Rust.Type) -> Rust.Type {
        match (__0) {
            IsVoid => { Rust_TypeName("()".to_string()) },
            ty => { toRustType(ty) },
        }
    }

    fn toRustType(__0: Rust.Type) -> Rust.Type {
        match (__0) {
            IsBool => { Rust_TypeName("bool".to_string()) },
            IsInt(s, w) => { Rust_TypeName((:((match s {
                                Signed => { 'i' },
                                Unsigned => { 'u' },
                            }), (match w {
                                BitWidth, b => { show(b) },
                                WordWidth => { "size".to_string() },
                            })))) },
            IsFloat(w) => { Rust_TypeName((:('f', show(w)))) },
            IsVoid => { Rust_TypeName("::std::os::raw::c_void".to_string()) },
            IsFunc(retTy, args, variadic) => { Rust_TypeName(concat(vec!["unsafe extern fn(".to_string(), args_q, ")".to_string(), /=(if(retTy), __op_concat(IsVoid(then, " -> ".to_string()), typename(retTy, else, "".to_string())))])) },
            IsPtr(mut, to) => { {

                                    let Rust_TypeName = |to_q| {
                        toRustType(to, in, Rust_TypeName, (__op_concat(rustMut(mut), to_q)))
                    };

            } },
            IsArray(_, size, el) => { Rust_TypeName((__op_concat("[".to_string(), __op_concat(typename(el), __op_concat("; ".to_string(), __op_concat(show(size), "]".to_string())))))) },
            IsStruct(name, _fields) => { Rust_TypeName(name) },
            IsEnum(name) => { Rust_TypeName(name) },
            IsIncomplete(ident) => { Rust_TypeName((identToString(ident))) },
        }
    }

    fn translateInitList(ty: EnvMonad) -> EnvMonad {
        /* do */ {

            let objectsAndInitializers = forM(list)(Lambda);
            {

            let base = || {
            match ty {
                    IsArray, _, size, el => { From(el, 0, (replicate((-(size, 1)), el)), (Base(ty))) },
                    IsStruct, _, (_, ty_q)(:, fields) => { From(ty_q, 0, (map(snd, fields)), (Base(ty))) },
                    _ => { Base(ty) },
                }
        };

};
            let (_, initializer) = foldM(resolveCurrentObject, (Just(base), mempty), objectsAndInitializers);
            return(initializer)}
    }

    fn typeName(__0: EnvMonad) -> EnvMonad {
        match (__0, __1, __2) {
            decl, <todo>, CStaticAssert(<todo>) => { badSource(decl, "static assert in type name ".to_string()) },
            decl, <todo>, CDecl(spec, declarators, _) => { /* do */ {

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
                return((typeMutable(itype), typeRep(itype)))} },
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
            IsFloat(aw), IsFloat(bw) => { Just((IsFloat((max(aw, bw))))) },
            a, <todo>, IsFloat(_), _ => { Just(a) },
            _, b, <todo>, IsFloat(_) => { Just(b) },
            origA, origB => { match (intPromote(origA), intPromote(origB)) {
                (a, b) => if (a == b) { Just(a) },
                    (IsInt(Signed, sw), IsInt(Unsigned, uw)) => { mixedSign(sw, uw) },
                    (IsInt(Unsigned, uw), IsInt(Signed, sw)) => { mixedSign(sw, uw) },
                    (IsInt(as, aw), IsInt(_bs, bw)) => { /* do */ {

                        let rank = integerConversionRank(aw, bw);
                        Just((IsInt(as, ((if(rank) == GT(then, aw, else, bw))))))} },
                    _ => { Nothing },
                } },
        }
    }

    fn wrapMain(declr: EnvMonad) -> EnvMonad {
        /* do */ {

            let (setup, args) = wrapArgv(argTypes);
            {

            let ret = || {
            Rust_VarName("ret".to_string())
        };

};
            emitItems(vec![Rust_Item(vec![], Rust_Private, (Rust_Function(vec![], "main".to_string(), vec![], (Rust_TypeName("()".to_string())), (statementsToBlock((__op_concat(setup, __op_concat(vec![bind(Rust_Immutable, ret)(Rust_UnsafeExpr(Rust_Block(vec![])(Just(call(realName, args)))))], exprToStatements((call("::std::process::exit".to_string(), vec![Rust_Var(ret)])))))))))))]);
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
            r, <todo>, Result(<todo>) => { match result(r) {
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
            r => { r },
        }
    }

}

mod Language_Rust_Corrode_CFG {
    struct BasicBlock(BasicBlock, s, Terminator<c>);

    #[derive(Debug)]
    enum Terminator' {
        Unreachable,
        Branch(l),
        CondBranch(c, l, l)
    }

    struct Unordered;

    struct DepthFirst;

    struct CFG(CFG, Label, IntMap.IntMap<BasicBlock<s, c>>);

    struct BuildState(BuildState, { /* struct def */ });

    #[derive(Debug)]
    enum StructureLabel {
        GoTo({ /* struct def */ }),
        ExitTo({ /* struct def */ }),
        Nested(Vec<Structure<s, c>>)
    }

    #[derive(Debug)]
    enum Structure' {
        Simple(s, StructureTerminator<s, c>),
        Loop(a),
        Multiple(IntMap.IntMap<a>, a)
    }

    #[derive(Debug)]
    struct Structure(Structure, { /* struct def */ });

    fn addBlock(label: Monad) -> Monad {
        /* do */ {

            modify(Lambda({
        buildBlocks: IntMap_insert(label, (BasicBlock(stmt, terminator)), (buildBlocks(st)))
        }))}
    }

    fn buildCFG(root: Monad) -> Monad {
        /* do */ {

            let (label, final) = runStateT(root, (BuildState(0, IntMap_empty)));
            return((CFG(label, (buildBlocks(final)))))}
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
        /* do */ {

            let old = get;
            put(old, {
    buildLabel: +(buildLabel(old), 1)
    });
            return((buildLabel(old)))}
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
        {

                    let (returns, noreturns) = || {
                partitionMembers(entries)(IntSet_unions(map(successors)(IntMap_elems(blocks))))
            };
;
                    let (present, absent) = || {
                partitionMembers(entries, (IntMap_keysSet(blocks)))
            };

    }(in, match (IntSet_toList(noreturns), IntSet_toList(returns)) {
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
    enum ItemKind {
        Enum,
        Struct,
        Union,
        Type,
        Symbol
    }

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
        fromMaybe((vec![], crates))(/* do */ {

                let thisCrate = Map_lookup("".to_string(), crates);
                let thisModule = Map_lookup(modName, thisCrate);
                {

            let thisCrate_q = || {
            Map_delete(modName, thisCrate)
        };

};
                {

            let crates_q = || {
            Map_insert("".to_string(), thisCrate_q, crates)
        };

};
                return((thisModule, crates_q))})
    }

}

mod Language_Rust_Idiomatic {
    fn itemIdioms(__0: Rust.Item) -> Rust.Item {
        match (__0) {
            Rust_Item(attrs, vis, Rust_Function(fattrs, name, formals, ret, b)) => { Rust_Item(attrs, vis, (Rust_Function(fattrs, name, formals, ret, (tailBlock(b))))) },
            i => { i },
        }
    }

    fn tailBlock(__0: Rust.Block) -> Rust.Block {
        match (__0) {
            Rust_Block(b, Just(<todo>)) => { Rust_Block(b, e) },
            Rust_Block(<todo>, Nothing) => { Rust_Block(b, e) },
            b => { b },
        }
    }

    fn tailExpr(__0: Maybe) -> Maybe {
        match (__0) {
            Rust_Return(e) => { Just(e) },
            Rust_BlockExpr(b) => { Just((Just((Rust_BlockExpr((tailBlock(b))))))) },
            Rust_IfThenElse(c, t, f) => { Just((Just((Rust_IfThenElse(c, (tailBlock(t)), (tailBlock(f))))))) },
            _ => { Nothing },
        }
    }

    fn unsnoc(__0: Maybe) -> Maybe {
        match (__0) {
            [] => { Nothing },
            x:xs => { match unsnoc(xs) {
                    Just, (a, b) => { Just((x:a, b)) },
                    Nothing => { Just((vec![], x)) },
                } },
        }
    }

}

mod Language_Rust {

}



fn main() { /* demo */ }
