// ERROR: cannot yet convert file "./corrode/src/Language/Rust/AST.hs"

mod Language_Rust_Corrode_C {
    struct FunctionContext(FunctionContext, RecordTODO);

    struct Output(Output, RecordTODO);

    struct GlobalState(GlobalState, RecordTODO);

    struct EnvState(EnvState, RecordTODO);

    struct Initializer(Initializer, Maybe(Rust.Expr), IntMap.IntMap(Initializer));

    #[derive(Show)]
    struct Designator(Base, CType, From, CType, isize, Vec<CType>, Designator);

    struct OuterLabels(OuterLabels, RecordTODO);

    struct Result(Result, RecordTODO);

    #[derive(Show, Eq)]
    struct Signed(Signed, Unsigned);

    #[derive(Show, Eq)]
    struct IntWidth(BitWidth, isize, WordWidth);

    #[derive(Show)]
    struct CType(IsBool, IsInt, Signed, IntWidth, IsFloat, isize, IsVoid, IsFunc, CType, Vec<(Maybe((Rust.Mutable, Ident)), CType)>, Bool, IsPtr, Rust.Mutable, CType, IsArray, Rust.Mutable, isize, CType, IsStruct, String, Vec<(String, CType)>, IsEnum, String, IsIncomplete, Ident);

    struct IntermediateType(IntermediateType, RecordTODO);

    fn addExternIdent(ident: Ident, deferred: EnvMonad(s, IntermediateType), mkItem: Pair(Span([Ref(Ident("String"))]), Pair(Span([Tuple([Span([Ref(Ident("Rust.Mutable"))]), Span([Ref(Ident("CType"))])])]), Span([Ref(Ident("Rust.ExternItem"))])))) -> EnvMonad(s, EmptyParen) {
        {

        }
    }

    fn addSwitchCase(condition: Maybe(CExpr), body: CStat, next: CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result)))) -> CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result))) {
        {

        }
    }

    fn addSymbolIdent(ident: Ident, (mut, ty): (Rust.Mutable, CType)) -> EnvMonad(s, String) {
        {

        }
    }

    fn addSymbolIdentAction(ident: Ident, action: EnvMonad(s, Result)) -> EnvMonad(s, EmptyParen) {
        lift({

            })
    }

    fn addTagIdent(ident: Ident, ty: EnvMonad(s, CType)) -> EnvMonad(s, EmptyParen) {
        lift({

            })
    }

    fn addTypedefIdent(ident: Ident, ty: EnvMonad(s, IntermediateType)) -> EnvMonad(s, EmptyParen) {
        lift({

            })
    }

    fn applyRenames(ident: Ident) -> String {
        match identToString(ident) {
                "final" => "final_".to_string(),
                "fn" => "fn_".to_string(),
                "in" => "in_".to_string(),
                "let" => "let_".to_string(),
                "main" => "_c_main".to_string(),
                "match" => "match_".to_string(),
                "mod" => "mod_".to_string(),
                "proc" => "proc_".to_string(),
                "type" => "type_".to_string(),
                "where" => "where_".to_string(),
                name => name,
            }
    }

    fn badSource(node: node, msg: String) -> EnvMonad(s, a) {
        noTranslation(node, (++("illegal ".to_string(), ++(msg, "; check whether a real C compiler accepts this".to_string()))))
    }

    fn baseTypeOf(specs: Vec<CDeclSpec>) -> EnvMonad(s, (Maybe(CStorageSpec), EnvMonad(s, IntermediateType))) {
        {

        }
    }

    fn binop(expr: CExpr, op: CBinaryOp, lhs: Result, rhs: Result) -> EnvMonad(s, Result) {
        fmap(wrapping)(match op {
                    CMulOp => promote(expr, Rust.Mul, lhs, rhs),
                    CDivOp => promote(expr, Rust.Div, lhs, rhs),
                    CRmdOp => promote(expr, Rust.Mod, lhs, rhs),
                    CAddOp => match (toPtr(lhs), toPtr(rhs)) {
                            (Just(ptr), _) => return((offset(ptr, rhs))),
                            (_, Just(ptr)) => return((offset(ptr, lhs))),
                            _ => promote(expr, Rust.Add, lhs, rhs),
                        },
                    CSubOp => match (toPtr(lhs), toPtr(rhs)) {
                            (Just(lhs'), Just(rhs')) => {

                            },
                            (Just(ptr), _) => return(ptr, hashmap! {
                                "result" => Rust.MethodCall((result(ptr)), (Rust.VarName("offset".to_string())), vec![Rust.Neg((castTo((IsInt(Signed, WordWidth)), rhs)))])
                                }),
                            _ => promote(expr, Rust.Sub, lhs, rhs),
                        },
                    CShlOp => shift(Rust.ShiftL),
                    CShrOp => shift(Rust.ShiftR),
                    CLeOp => comparison(Rust.CmpLT),
                    CGrOp => comparison(Rust.CmpGT),
                    CLeqOp => comparison(Rust.CmpLE),
                    CGeqOp => comparison(Rust.CmpGE),
                    CEqOp => comparison(Rust.CmpEQ),
                    CNeqOp => comparison(Rust.CmpNE),
                    CAndOp => promote(expr, Rust.And, lhs, rhs),
                    CXorOp => promote(expr, Rust.Xor, lhs, rhs),
                    COrOp => promote(expr, Rust.Or, lhs, rhs),
                    CLndOp => return(Result, hashmap! {
                        "resultType" => IsBool,
                        "resultMutable" => Rust.Immutable,
                        "result" => Rust.LAnd((toBool(lhs)), (toBool(rhs)))
                        }),
                    CLorOp => return(Result, hashmap! {
                        "resultType" => IsBool,
                        "resultMutable" => Rust.Immutable,
                        "result" => Rust.LOr((toBool(lhs)), (toBool(rhs)))
                        }),
                })
    }

    fn bitWidth(__0: isize, __1: IntWidth) -> isize {
        match (__0, __1) {
            wordWidth WordWidth => wordWidth,
            _ BitWidth(w) => w,
        }
    }

    fn blockToStatements(Rust.Block(stmts, mexpr): Rust.Block) -> Vec<Rust.Stmt> {
        match mexpr {
                Just expr => ++(stmts, exprToStatements(expr)),
                Nothing => stmts,
            }
    }

    fn castTo(__0: CType, __1: Result) -> Rust.Expr {
        match (__0, __1) {
            target Result(RecordTODO) => castTo(target, Result, hashmap! {
                "resultType" => IsPtr(mut, el),
                "resultMutable" => Rust.Immutable,
                "result" => Rust.MethodCall(source, (Rust.VarName(method)), vec![])
                }),
            IsBool source => toBool(source),
            target EmptyParen IsInt(RecordTODO) Result(RecordTODO) => Rust.Lit((Rust.LitInt(n, repr, (toRustType(target))))),
            IsInt(Signed, w) Result(RecordTODO) => Rust.Neg((Rust.Lit((Rust.LitInt(n, repr, (toRustType((IsInt(Signed, w))))))))),
            target source => Rust.Cast((result(source)), (toRustType(target))),
        }
    }

    fn cfgToRust(_node: node, build: CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result)))) -> EnvMonad(s, Vec<Rust.Stmt>) {
        {

        }
    }

    fn charType() -> CType {
        IsInt(Unsigned, (BitWidth(8)))
    }

    fn compatibleInitializer(__0: CType, __1: CType) -> Bool {
        match (__0, __1) {
            IsStruct(name1, _) IsStruct(name2, _) => ==(name1, name2),
            IsStruct RecordTODO _ => False,
            _ IsStruct RecordTODO => False,
            _ _ => True,
        }
    }

    fn compatiblePtr(__0: CType, __1: CType) -> CType {
        match (__0, __1) {
            IsPtr(_, IsVoid) b => b,
            IsArray(mut, _, el) b => compatiblePtr((IsPtr(mut, el)), b),
            a IsPtr(_, IsVoid) => a,
            a IsArray(mut, _, el) => compatiblePtr(a, (IsPtr(mut, el))),
            IsPtr(m1, a) IsPtr(m2, b) => IsPtr((leastMutable(m1, m2)), (compatiblePtr(a, b))),
            _ _ => IsVoid,
        }
    }

    fn completeType(__0: CType, __1: EnvMonad(s, CType)) -> EnvMonad(s, CType) {
        match (__0, __1, __2) {
            orig EmptyParen IsIncomplete(ident) => {

            },
            ty => return(ty),
        }
    }

    fn compound(expr: CExpr, returnOld: Bool, demand: Bool, op: CAssignOp, lhs: Result, rhs: Result) -> EnvMonad(s, Result) {
        {

        }
    }

    fn derivedDeferredTypeOf(deferred: EnvMonad(s, IntermediateType), declr: CDeclr, EmptyParen: Vec<CDecl>, CDeclr(_, derived, _, _, _): EnvMonad(s, EnvMonad(s, IntermediateType))) -> EnvMonad(s, EnvMonad(s, IntermediateType)) {
        {

        }
    }

    fn derivedTypeOf(deferred: EnvMonad(s, IntermediateType), declr: CDeclr) -> EnvMonad(s, IntermediateType) {
        join((derivedDeferredTypeOf(deferred, declr, vec![])))
    }

    fn designatorType(__0: Designator) -> CType {
        match (__0) {
            Base(ty) => ty,
            From(ty, _, _, _) => ty,
        }
    }

    fn emitIncomplete(kind: ItemKind, ident: Ident) -> EnvMonad(s, CType) {
        {

        }
    }

    fn emitItems(items: Vec<Rust.Item>) -> EnvMonad(s, EmptyParen) {
        lift(tell(mempty, hashmap! {
                "outputItems" => items
                }))
    }

    fn enumReprType() -> CType {
        IsInt(Signed, (BitWidth(32)))
    }

    fn exprToStatements(__0: Rust.Expr) -> Vec<Rust.Stmt> {
        match (__0) {
            Rust.IfThenElse(c, t, f) => vec![Rust.Stmt((Rust.IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))],
            Rust.BlockExpr(b) => blockToStatements(b),
            e => vec![Rust.Stmt(e)],
        }
    }

    fn getSwitchCases(expr: CExpr) -> CSourceBuildCFGT(s, (a, SwitchCases)) {
        mapBuildCFGT(wrap)
    }

    fn getSwitchExpression(stmt: CStat) -> CSourceBuildCFGT(s, CExpr) {
        {

        }
    }

    fn getSymbolIdent(ident: Ident) -> EnvMonad(s, Maybe(Result)) {
        {

        }
    }

    fn getTagIdent(ident: Ident) -> EnvMonad(s, Maybe(EnvMonad(s, CType))) {
        lift({

            })
    }

    fn getTypedefIdent(ident: Ident) -> EnvMonad(s, (String, Maybe(EnvMonad(s, IntermediateType)))) {
        lift({

            })
    }

    fn gotoLabel(ident: Ident) -> CSourceBuildCFGT(s, Label) {
        {

        }
    }

    fn intPromote(__0: CType) -> CType {
        match (__0) {
            IsBool => IsInt(Signed, (BitWidth(32))),
            IsEnum(_) => enumReprType,
            x => x,
        }
    }

    fn integerConversionRank(__0: IntWidth, __1: IntWidth) -> Maybe(Ordering) {
        match (__0, __1) {
            BitWidth(a) BitWidth(b) => Just((compare(a, b))),
            WordWidth WordWidth => Just(EQ),
            _ _ => Nothing,
        }
    }

    fn interpretBlockItem(__0: CBlockItem, __1: CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result)))) -> CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result))) {
        match (__0, __1) {
            CBlockStmt(stmt) next => interpretStatement(stmt, next),
            CBlockDecl(decl) next => {

            },
            item _ => lift(lift((unimplemented(item)))),
        }
    }

    fn interpretConstExpr(__0: CExpr) -> EnvMonad(s, Integer) {
        match (__0) {
            CConst(CIntConst(CInteger(v, _, _), _)) => return(v),
            expr => unimplemented(expr),
        }
    }

    fn interpretDeclarations(__0: MakeBinding(s, b), __1: CDecl, __2: EnvMonad(s, Vec<b>)) -> EnvMonad(s, Vec<b>) {
        match (__0, __1, __2, __3) {
            (fromItem, makeBinding) declaration EmptyParen CDecl(specs, decls, _) => {

            },
            _ node EmptyParen CStaticAssert(RecordTODO) => unimplemented(node),
        }
    }

    fn interpretExpr(__0: Bool, __1: CExpr) -> EnvMonad(s, Result) {
        match (__0, __1) {
            demand CComma(exprs, _) => {

            },
            demand expr EmptyParen CAssign(op, lhs, rhs, _) => {

            },
            demand expr EmptyParen CCond(c, Just(t), f, _) => {

            },
            _ expr EmptyParen CBinary(op, lhs, rhs, _) => {

            },
            _ CCast(decl, expr, _) => {

            },
            demand node EmptyParen CUnary(op, expr, _) => match op {
                    CPreIncOp => incdec(False, CAddAssOp),
                    CPreDecOp => incdec(False, CSubAssOp),
                    CPostIncOp => incdec(True, CAddAssOp),
                    CPostDecOp => incdec(True, CSubAssOp),
                    CAdrOp => {

                    },
                    CIndOp => {

                    },
                    CPlusOp => {

                    },
                    CMinOp => fmap(wrapping)(simple(Rust.Neg)),
                    CCompOp => simple(Rust.Not),
                    CNegOp => {

                    },
                },
            _ CSizeofExpr(e, _) => {

            },
            _ CSizeofType(decl, _) => {

            },
            _ CAlignofExpr(e, _) => {

            },
            _ CAlignofType(decl, _) => {

            },
            _ expr EmptyParen CIndex(lhs, rhs, _) => {

            },
            _ expr EmptyParen CCall(func, args, _) => {

            },
            _ expr EmptyParen CMember(obj, ident, deref, node) => {

            },
            _ expr EmptyParen CVar(ident, _) => {

            },
            _ expr EmptyParen CConst(c) => match c {
                    CIntConst CInteger(v, repr, flags) _ => Let(in, match allowed_types {
                            Vec<> => badSource(expr, "integer (too big)".to_string()),
                            ty EmptyParen _ => return((literalNumber(ty, (Rust.LitInt(v, repr'))))),
                        }),
                    CFloatConst CFloat(str) _ => match span((Operator("notElem")("fF".to_string())), str) {
                            (v, "") => return((literalNumber((IsFloat(64)), (Rust.LitFloat(v))))),
                            (v, Vec<_>) => return((literalNumber((IsFloat(32)), (Rust.LitFloat(v))))),
                            _ => badSource(expr, "float".to_string()),
                        },
                    CCharConst CChar(ch, False) _ => return(Result, hashmap! {
                        "resultType" => charType,
                        "resultMutable" => Rust.Immutable,
                        "result" => Rust.Lit((Rust.LitByteChar(ch)))
                        }),
                    CStrConst CString(str, False) _ => return(Result, hashmap! {
                        "resultType" => IsArray(Rust.Immutable, (+(length(str), 1)), charType),
                        "resultMutable" => Rust.Immutable,
                        "result" => Rust.Deref((Rust.Lit((Rust.LitByteStr((++(str, "N".to_string())))))))
                        }),
                    _ => unimplemented(expr),
                },
            _ CCompoundLit(decl, initials, info) => {

            },
            demand stat EmptyParen CStatExpr(CCompound(Vec<>, stmts, _), _) => scope({

                }),
            _ expr => unimplemented(expr),
        }
    }

    fn interpretFunction(CFunDef(specs, declr, EmptyParen, CDeclr(mident, _, _, _, _), argtypes, body, _): CFunDef) -> EnvMonad(s, EmptyParen) {
        {

        }
    }

    fn interpretInitializer(ty: CType, initial: CInit) -> EnvMonad(s, Rust.Expr) {
        {

        }
    }

    fn interpretStatement(__0: CStat, __1: CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result)))) -> CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result))) {
        match (__0, __1) {
            CLabel(ident, body, _, _) next => {

            },
            stmt EmptyParen CCase(expr, body, node) next => {

            },
            stmt EmptyParen CCases(lower, upper, body, node) next => {

            },
            CDefault(body, _) next => addSwitchCase(Nothing, body, next),
            CExpr(Nothing, _) next => next,
            CExpr(Just(expr), _) next => {

            },
            CCompound(Vec<>, items, _) next => mapBuildCFGT((mapRWST(scope)))({

                }),
            CIf(c, t, mf, _) next => {

            },
            stmt EmptyParen CSwitch(expr, body, node) next => {

            },
            CWhile(c, body, doWhile, _) next => {

            },
            CFor(initial, mcond, mincr, body, _) next => {

            },
            CGoto(ident, _) next => {

            },
            stmt EmptyParen CCont(_) next => {

            },
            stmt EmptyParen CBreak(_) next => {

            },
            stmt EmptyParen CReturn(expr, _) next => {

            },
            stmt _ => lift(lift(unimplemented(stmt))),
        }
    }

    fn interpretTranslationUnit(_thisModule: ModuleMap, rewrites: ItemRewrites, CTranslUnit(decls, _): CTranslUnit) -> Either(String, Vec<Rust.Item>) {
        match err {
                Left msg => Left(msg),
                Right _ => Right(items'),
            }
    }

    fn makeLetBinding() -> MakeBinding(s, Rust.Stmt) {
        (Rust.StmtItem(vec![]), makeBinding)
    }

    fn makeStaticBinding() -> MakeBinding(s, Rust.Item) {
        (Rust.Item(vec![], Rust.Private), makeBinding)
    }

    fn modifyGlobal(f: Pair(Span([Ref(Ident("GlobalState"))]), Span([Tuple([Span([Ref(Ident("GlobalState"))]), Span([Ref(Ident("a"))])])]))) -> EnvMonad(s, a) {
        lift({

            })
    }

    fn mutable(quals: Vec<CTypeQualifier(a)>) -> Rust.Mutable {
        if(any, (Lambda), quals, then, Rust.Immutable, else, Rust.Mutable)
    }

    fn nestedObject(ty: CType, desig: Designator) -> Maybe(Designator) {
        match designatorType(desig) {
                IsArray _ size el => Just((From(el, 0, (replicate((-(size, 1)), el)), desig))),
            ty' => if compatibleInitializer(ty, ty') { Just(desig) },
                IsStruct _ (_, ty')(EmptyParen, fields) => nestedObject(ty, (From(ty', 0, (map(snd, fields)), desig))),
                _ => Nothing,
            }
    }

    fn nextObject(__0: Designator, __1: CurrentObject) -> CurrentObject {
        match (__0, __1) {
            Base RecordTODO => Nothing,
            From(_, i, ty(EmptyParen, remaining), base) => Just((From(ty, (+(i, 1)), remaining, base))),
            From(_, _, Vec<>, base) => nextObject(base),
        }
    }

    fn noTranslation(node: node, msg: String) -> EnvMonad(s, a) {
        throwE(concat(vec![show((posOf(node))), ": ".to_string(), msg, ":\\n".to_string(), render((nest(4, (pretty(node)))))]))
    }

    fn objectFromDesignators(__0: CType, __1: Vec<CDesignator>) -> EnvMonad(s, CurrentObject) {
        match (__0, __1) {
            _ Vec<> => pure(Nothing),
            ty desigs => <(Just, ()(>((), go(ty, desigs, (Base(ty)))))),
        }
    }

    fn promote(node: node, op: Pair(Span([Ref(Ident("Rust.Expr"))]), Pair(Span([Ref(Ident("Rust.Expr"))]), Span([Ref(Ident("Rust.Expr"))]))), a: Result, b: Result) -> EnvMonad(s, Result) {
        match usual((resultType(a)), (resultType(b))) {
                Just rt => return(Result, hashmap! {
                    "resultType" => rt,
                    "resultMutable" => Rust.Immutable,
                    "result" => op((castTo(rt, a)), (castTo(rt, b)))
                    }),
                Nothing => badSource(node)(concat(vec!["arithmetic combination for ".to_string(), show((resultType(a))), " and ".to_string(), show((resultType(b)))])),
            }
    }

    fn promotePtr(node: node, op: Pair(Span([Ref(Ident("Rust.Expr"))]), Pair(Span([Ref(Ident("Rust.Expr"))]), Span([Ref(Ident("Rust.Expr"))]))), a: Result, b: Result) -> EnvMonad(s, Result) {
        match (resultType(a), resultType(b)) {
                (IsArray(_, _, _), _) => ptrs,
                (IsPtr(_, _), _) => ptrs,
                (_, IsArray(_, _, _)) => ptrs,
                (_, IsPtr(_, _)) => ptrs,
                _ => promote(node, op, a, b),
            }
    }

    fn resolveCurrentObject((obj0, prior): (CurrentObject, Initializer), (obj1, cinitial): (CurrentObject, CInit)) -> EnvMonad(s, (CurrentObject, Initializer)) {
        match mplus(obj1, obj0) {
                Nothing => return((Nothing, prior)),
                Just obj => {

                },
            }
    }

    fn resultToStatements() -> Vec<Rust.Stmt> {
        (exprToStatements . result)
    }

    fn runOnce(action: EnvMonad(s, a)) -> EnvMonad(s, EnvMonad(s, a)) {
        {

        }
    }

    fn rustAlignOfType(Rust.TypeName(ty): Rust.Type) -> Result {
        Result(hashmap! {
            "resultType" => IsInt(Unsigned, WordWidth),
            "resultMutable" => Rust.Immutable,
            "result" => Rust.Call((Rust.Var((Rust.VarName((++("::std::mem::align_of::<".to_string(), ++(ty, ">".to_string()))))))), vec![])
            })
    }

    fn rustSizeOfType(Rust.TypeName(ty): Rust.Type) -> Result {
        Result(hashmap! {
            "resultType" => IsInt(Unsigned, WordWidth),
            "resultMutable" => Rust.Immutable,
            "result" => Rust.Call((Rust.Var((Rust.VarName((++("::std::mem::size_of::<".to_string(), ++(ty, ">".to_string()))))))), vec![])
            })
    }

    fn scalar(expr: Rust.Expr) -> Initializer {
        Initializer((Just(expr)), IntMap.empty)
    }

    fn scope(m: EnvMonad(s, a)) -> EnvMonad(s, a) {
        {

        }
    }

    fn setBreak(label: Label) -> CSourceBuildCFGT(s, a) {
        mapBuildCFGT((local((Lambda(hashmap! {
                    "onBreak" => Just(label)
                    })))))
    }

    fn setContinue(label: Label) -> CSourceBuildCFGT(s, a) {
        mapBuildCFGT((local((Lambda(hashmap! {
                    "onContinue" => Just(label)
                    })))))
    }

    fn statementsToBlock(__0: Vec<Rust.Stmt>) -> Rust.Block {
        match (__0) {
            Vec<Rust.Stmt(Rust.BlockExpr(stmts))> => stmts,
            stmts => Rust.Block(stmts, Nothing),
        }
    }

    fn toBool(__0: Result) -> Rust.Expr {
        match (__0) {
            Result(RecordTODO) => Rust.Lit((Rust.LitBool(False))),
            Result(RecordTODO) => Rust.Lit((Rust.LitBool(True))),
            Result(RecordTODO) => match t {
                    IsBool => v,
                    IsPtr _ _ => Rust.Not((Rust.MethodCall(v, (Rust.VarName("is_null".to_string())), vec![]))),
                    _ => Rust.CmpNE(v, 0),
                },
        }
    }

    fn toNotBool(__0: Result) -> Rust.Expr {
        match (__0) {
            Result(RecordTODO) => Rust.Lit((Rust.LitBool(True))),
            Result(RecordTODO) => Rust.Lit((Rust.LitBool(False))),
            Result(RecordTODO) => match t {
                    IsBool => Rust.Not(v),
                    IsPtr _ _ => Rust.MethodCall(v, (Rust.VarName("is_null".to_string())), vec![]),
                    _ => Rust.CmpEQ(v, 0),
                },
        }
    }

    fn toPtr(__0: Result, __1: Maybe(Result)) -> Maybe(Result) {
        match (__0, __1, __2) {
            ptr EmptyParen Result(RecordTODO) => Just(ptr, hashmap! {
                "resultType" => IsPtr(mut, el),
                "result" => castTo((IsPtr(mut, el)), ptr)
                }),
            ptr EmptyParen Result(RecordTODO) => Just(ptr),
            _ => Nothing,
        }
    }

    fn toRustRetType(__0: CType) -> Rust.Type {
        match (__0) {
            IsVoid => Rust.TypeName("()".to_string()),
            ty => toRustType(ty),
        }
    }

    fn toRustType(__0: CType) -> Rust.Type {
        match (__0) {
            IsBool => Rust.TypeName("bool".to_string()),
            IsInt(s, w) => Rust.TypeName((:((match s {
                                Signed => "\"\"".to_string(),
                                Unsigned => "\"\"".to_string(),
                            }), (match w {
                                BitWidth b => show(b),
                                WordWidth => "size".to_string(),
                            })))),
            IsFloat(w) => Rust.TypeName((:("\"\"".to_string(), show(w)))),
            IsVoid => Rust.TypeName("::std::os::raw::c_void".to_string()),
            IsFunc(retTy, args, variadic) => Rust.TypeName(concat(vec!["unsafe extern fn(".to_string(), args', ")".to_string(), /=(if(retTy), ++(IsVoid(then, " -> ".to_string()), typename(retTy, else, "".to_string())))])),
            IsPtr(mut, to) => Let,
            IsArray(_, size, el) => Rust.TypeName((++("[".to_string(), ++(typename(el), ++("; ".to_string(), ++(show(size), "]".to_string())))))),
            IsStruct(name, _fields) => Rust.TypeName(name),
            IsEnum(name) => Rust.TypeName(name),
            IsIncomplete(ident) => Rust.TypeName((identToString(ident))),
        }
    }

    fn translateInitList(ty: CType, list: CInitList) -> EnvMonad(s, Initializer) {
        {

        }
    }

    fn typeName(__0: CDecl, __1: EnvMonad(s, (Rust.Mutable, CType))) -> EnvMonad(s, (Rust.Mutable, CType)) {
        match (__0, __1, __2) {
            decl EmptyParen CStaticAssert(RecordTODO) => badSource(decl, "static assert in type name ".to_string()),
            decl EmptyParen CDecl(spec, declarators, _) => {

            },
        }
    }

    fn typeToResult(itype: IntermediateType, expr: Rust.Expr) -> Result {
        Result(hashmap! {
            "resultType" => typeRep(itype),
            "resultMutable" => typeMutable(itype),
            "result" => expr
            })
    }

    fn unimplemented(node: node) -> EnvMonad(s, a) {
        noTranslation(node, "Corrode doesn\'t handle this yet".to_string())
    }

    fn uniqueName(base: String) -> EnvMonad(s, String) {
        modifyGlobal(Lambda)
    }

    fn useForwardRef(ident: Ident) -> EnvMonad(s, EmptyParen) {
        modifyGlobal(Lambda)
    }

    fn usual(__0: CType, __1: CType) -> Maybe(CType) {
        match (__0, __1) {
            IsFloat(aw) IsFloat(bw) => Just((IsFloat((max(aw, bw))))),
            a EmptyParen IsFloat(_) _ => Just(a),
            _ b EmptyParen IsFloat(_) => Just(b),
            origA origB => match (intPromote(origA), intPromote(origB)) {
                (a, b) => if ==(a, b) { Just(a) },
                    (IsInt(Signed, sw), IsInt(Unsigned, uw)) => mixedSign(sw, uw),
                    (IsInt(Unsigned, uw), IsInt(Signed, sw)) => mixedSign(sw, uw),
                    (IsInt(as, aw), IsInt(_bs, bw)) => {

                    },
                    _ => Nothing,
                },
        }
    }

    fn wrapMain(declr: CDeclr, realName: String, argTypes: Vec<CType>) -> EnvMonad(s, EmptyParen) {
        {

        }
    }

    fn wrapping(__0: Result, __1: Result) -> Result {
        match (__0, __1, __2) {
            r EmptyParen Result(RecordTODO) => match result(r) {
                    Rust.Add lhs rhs => r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_add".to_string())), vec![rhs])
                        }),
                    Rust.Sub lhs rhs => r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_sub".to_string())), vec![rhs])
                        }),
                    Rust.Mul lhs rhs => r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_mul".to_string())), vec![rhs])
                        }),
                    Rust.Div lhs rhs => r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_div".to_string())), vec![rhs])
                        }),
                    Rust.Mod lhs rhs => r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_rem".to_string())), vec![rhs])
                        }),
                    Rust.Neg e => r(hashmap! {
                        "result" => Rust.MethodCall(e, (Rust.VarName("wrapping_neg".to_string())), vec![])
                        }),
                    _ => r,
                },
            r => r,
        }
    }

}

// ERROR: cannot yet convert file "./corrode/src/Language/Rust/Corrode/CFG.lhs"

mod Language_Rust_Corrode_CrateMap {
    #[derive(Eq, Ord, Show)]
    struct ItemKind(Enum, Struct, Union, Type, Symbol);

    fn mergeCrateMaps() -> Map.Map(String, CrateMap) {
        Map.fromListWith((Map.unionWith((Operator("++")))))
    }

    fn parseCrateMap() -> Either(String, CrateMap) {
        (fmap(root) . (foldrM(parseLine, (Map.empty, vec![])) . (filter(((not . null))) . (map(cleanLine) . lines))))
    }

    fn rewritesFromCratesMap(crates: CratesMap) -> ItemRewrites {
        Map.fromList(Dummy)
    }

    fn splitModuleMap(modName: String, crates: CratesMap) -> (ModuleMap, CratesMap) {
        fromMaybe((vec![], crates))({

            })
    }

}

mod Language_Rust_Idiomatic {
    fn itemIdioms(__0: Rust.Item) -> Rust.Item {
        match (__0) {
            Rust.Item(attrs, vis, Rust.Function(fattrs, name, formals, ret, b)) => Rust.Item(attrs, vis, (Rust.Function(fattrs, name, formals, ret, (tailBlock(b))))),
            i => i,
        }
    }

    fn tailBlock(__0: Rust.Block) -> Rust.Block {
        match (__0) {
            Rust.Block(b, Just(Pair(Span([Ref(Ident("tailExpr"))]), Span([Ref(Ident("Just")), Ref(Ident("e"))])))) => Rust.Block(b, e),
            Rust.Block(Pair(Span([Ref(Ident("unsnoc"))]), Span([Ref(Ident("Just")), Tuple([Span([Ref(Ident("b"))]), Span([Ref(Ident("Rust.Stmt")), Tuple([Pair(Span([Ref(Ident("tailExpr"))]), Span([Ref(Ident("Just")), Ref(Ident("e"))]))])])])])), Nothing) => Rust.Block(b, e),
            b => b,
        }
    }

    fn tailExpr(__0: Rust.Expr) -> Maybe(Maybe(Rust.Expr)) {
        match (__0) {
            Rust.Return(e) => Just(e),
            Rust.BlockExpr(b) => Just((Just((Rust.BlockExpr((tailBlock(b))))))),
            Rust.IfThenElse(c, t, f) => Just((Just((Rust.IfThenElse(c, (tailBlock(t)), (tailBlock(f))))))),
            _ => Nothing,
        }
    }

    fn unsnoc(__0: Vec<a>) -> Maybe((Vec<a>, a)) {
        match (__0) {
            Vec<> => Nothing,
            x:xs => match unsnoc(xs) {
                    Just (a, b) => Just((x:a, b)),
                    Nothing => Just((vec![], x)),
                },
        }
    }

}

mod Language_Rust {

}



fn main() { /* demo */ }
