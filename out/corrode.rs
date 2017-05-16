mod Language_Rust_AST {
    #[derive(Debug, Eq)]
    struct LitIntRepr(DecRepr, OctalRepr, HexRepr);

    #[derive(Debug, Eq)]
    struct Lit(LitByteStr, String, LitByteChar, Char, LitBool, Bool, LitInt, Integer, LitIntRepr, Type, LitFloat, String, Type);

    #[derive(Debug, Eq)]
    struct Visibility(Public, Private);

    #[derive(Debug, Eq)]
    struct Mutable(Immutable, Mutable);

    #[derive(Debug)]
    struct Stmt(Stmt, Expr, Let, Mutable, Var, Maybe(Type), Maybe(Expr), StmtItem, Vec<Attribute>, ItemKind);

    #[derive(Debug)]
    struct Block(Block, Vec<Stmt>, Maybe(Expr));

    #[derive(Debug)]
    struct Attribute(Attribute, String);

    #[derive(Debug)]
    struct Item(Item, Vec<Attribute>, Visibility, ItemKind);

    #[derive(Debug)]
    struct FunctionAttribute(UnsafeFn, ExternABI, Maybe(String));

    #[derive(Debug)]
    struct ItemKind(Function, Vec<FunctionAttribute>, String, Vec<(Mutable, Var, Type)>, Type, Block, Static, Mutable, Var, Type, Expr, Struct, String, Vec<(String, Type)>, Extern, Vec<ExternItem>, Use, String, Enum, String, Vec<Enumerator>, CloneImpl, Type);

    #[derive(Debug)]
    struct ExternItem(ExternFn, String, Vec<(Var, Type)>, Bool, Type, ExternStatic, Mutable, Var, Type);

    #[derive(Debug)]
    struct Enumerator(EnumeratorAuto, String, EnumeratorExpr, String, Expr);

    #[derive(Debug)]
    struct Expr(Lit, Lit, Var, Var, Path, Path, Index, Expr, Expr, ArrayExpr, Vec<Expr>, RepeatArray, Expr, Expr, StructExpr, String, Vec<(String, Expr)>, Maybe(Expr), Call, Expr, Vec<Expr>, MethodCall, Expr, Var, Vec<Expr>, Lambda, Vec<Var>, Expr, Member, Expr, Var, BlockExpr, Block, UnsafeExpr, Block, IfThenElse, Expr, Block, Block, Loop, Maybe(Lifetime), Block, While, Maybe(Lifetime), Expr, Block, For, Maybe(Lifetime), Var, Expr, Block, Break, Maybe(Lifetime), Continue, Maybe(Lifetime), Return, Maybe(Expr), Neg, Expr, Deref, Expr, Not, Expr, Borrow, Mutable, Expr, Cast, Expr, Type, Mul, Expr, Expr, Div, Expr, Expr, Mod, Expr, Expr, Add, Expr, Expr, Sub, Expr, Expr, ShiftL, Expr, Expr, ShiftR, Expr, Expr, And, Expr, Expr, Xor, Expr, Expr, Or, Expr, Expr, CmpLT, Expr, Expr, CmpGT, Expr, Expr, CmpLE, Expr, Expr, CmpGE, Expr, Expr, CmpEQ, Expr, Expr, CmpNE, Expr, Expr, LAnd, Expr, Expr, LOr, Expr, Expr, Range, Expr, Expr, Assign, Expr, AssignOp, Expr);

    #[derive(Debug)]
    struct AssignOp(:=, :+=, :-=, :*=, :/=, :%=, :&=, :|=, :^=, :<<=, :>>=);

    #[derive(Debug)]
    struct ExprPosition(TopExpr, LeftExpr, RightExpr);

    fn pPrintBlock(__0: Doc, __1: Block) -> Doc {
        match (__0, __1) {
            pre, Block([], e) => { sep(vec![<+>(pre, text("{".to_string())), nest(4, (maybe(empty, pPrint, e))), text("}".to_string())]) },
            pre, Block(ss, e) => { <+>(pre, $+$(text("{".to_string()), $+$(nest(4, (vcat((++(map(pPrint, ss), vec![maybe(empty, pPrint, e)]))))), text("}".to_string())))) },
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
    struct Designator(Base, CType, From, CType, isize, Vec<CType>, Designator);

    struct OuterLabels(OuterLabels, { /* struct def */ });

    struct Result(Result, { /* struct def */ });

    #[derive(Debug, Eq)]
    struct Signed(Signed, Unsigned);

    #[derive(Debug, Eq)]
    struct IntWidth(BitWidth, isize, WordWidth);

    #[derive(Debug)]
    struct CType(IsBool, IsInt, Signed, IntWidth, IsFloat, isize, IsVoid, IsFunc, CType, Vec<(Maybe((Rust.Mutable, Ident)), CType)>, Bool, IsPtr, Rust.Mutable, CType, IsArray, Rust.Mutable, isize, CType, IsStruct, String, Vec<(String, CType)>, IsEnum, String, IsIncomplete, Ident);

    struct IntermediateType(IntermediateType, { /* struct def */ });

    fn addExternIdent(ident: Ident, deferred: EnvMonad(s, IntermediateType), mkItem: fn(String) -> fn((Rust.Mutable, CType)) -> Rust.ExternItem) -> EnvMonad(s, ()) {
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

    fn addSymbolIdentAction(ident: Ident, action: EnvMonad(s, Result)) -> EnvMonad(s, ()) {
        lift({

            })
    }

    fn addTagIdent(ident: Ident, ty: EnvMonad(s, CType)) -> EnvMonad(s, ()) {
        lift({

            })
    }

    fn addTypedefIdent(ident: Ident, ty: EnvMonad(s, IntermediateType)) -> EnvMonad(s, ()) {
        lift({

            })
    }

    fn applyRenames(ident: Ident) -> String {
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

    fn badSource(node: node, msg: String) -> EnvMonad(s, a) {
        noTranslation(node, (++("illegal ".to_string(), ++(msg, "; check whether a real C compiler accepts this".to_string()))))
    }

    fn baseTypeOf(specs: Vec<CDeclSpec>) -> EnvMonad(s, (Maybe(CStorageSpec), EnvMonad(s, IntermediateType))) {
        {

        }
    }

    fn binop(expr: CExpr, op: CBinaryOp, lhs: Result, rhs: Result) -> EnvMonad(s, Result) {
        fmap(wrapping)(match op {
                    CMulOp => { promote(expr, Rust.Mul, lhs, rhs) },
                    CDivOp => { promote(expr, Rust.Div, lhs, rhs) },
                    CRmdOp => { promote(expr, Rust.Mod, lhs, rhs) },
                    CAddOp => { match (toPtr(lhs), toPtr(rhs)) {
                            (Just(ptr), _) => { return((offset(ptr, rhs))) },
                            (_, Just(ptr)) => { return((offset(ptr, lhs))) },
                            _ => { promote(expr, Rust.Add, lhs, rhs) },
                        } },
                    CSubOp => { match (toPtr(lhs), toPtr(rhs)) {
                            (Just(lhs'), Just(rhs')) => { {

                            } },
                            (Just(ptr), _) => { return(ptr, hashmap! {
                                "result" => Rust.MethodCall((result(ptr)), (Rust.VarName("offset".to_string())), vec![Rust.Neg((castTo((IsInt(Signed, WordWidth)), rhs)))])
                                }) },
                            _ => { promote(expr, Rust.Sub, lhs, rhs) },
                        } },
                    CShlOp => { shift(Rust.ShiftL) },
                    CShrOp => { shift(Rust.ShiftR) },
                    CLeOp => { comparison(Rust.CmpLT) },
                    CGrOp => { comparison(Rust.CmpGT) },
                    CLeqOp => { comparison(Rust.CmpLE) },
                    CGeqOp => { comparison(Rust.CmpGE) },
                    CEqOp => { comparison(Rust.CmpEQ) },
                    CNeqOp => { comparison(Rust.CmpNE) },
                    CAndOp => { promote(expr, Rust.And, lhs, rhs) },
                    CXorOp => { promote(expr, Rust.Xor, lhs, rhs) },
                    COrOp => { promote(expr, Rust.Or, lhs, rhs) },
                    CLndOp => { return(Result, hashmap! {
                        "resultType" => IsBool,
                        "resultMutable" => Rust.Immutable,
                        "result" => Rust.LAnd((toBool(lhs)), (toBool(rhs)))
                        }) },
                    CLorOp => { return(Result, hashmap! {
                        "resultType" => IsBool,
                        "resultMutable" => Rust.Immutable,
                        "result" => Rust.LOr((toBool(lhs)), (toBool(rhs)))
                        }) },
                })
    }

    fn bitWidth(__0: isize, __1: IntWidth) -> isize {
        match (__0, __1) {
            wordWidth, WordWidth => { wordWidth },
            _, BitWidth(w) => { w },
        }
    }

    fn blockToStatements(Rust.Block(stmts, mexpr): Rust.Block) -> Vec<Rust.Stmt> {
        match mexpr {
                Just, expr => { ++(stmts, exprToStatements(expr)) },
                Nothing => { stmts },
            }
    }

    fn castTo(__0: CType, __1: Result) -> Rust.Expr {
        match (__0, __1) {
            target, Result({ .. }) => { castTo(target, Result, hashmap! {
                "resultType" => IsPtr(mut, el),
                "resultMutable" => Rust.Immutable,
                "result" => Rust.MethodCall(source, (Rust.VarName(method)), vec![])
                }) },
            IsBool, source => { toBool(source) },
            target, <todo>, IsInt({ .. }), Result({ .. }) => { Rust.Lit((Rust.LitInt(n, repr, (toRustType(target))))) },
            IsInt(Signed, w), Result({ .. }) => { Rust.Neg((Rust.Lit((Rust.LitInt(n, repr, (toRustType((IsInt(Signed, w))))))))) },
            target, source => { Rust.Cast((result(source)), (toRustType(target))) },
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
            IsStruct(name1, _), IsStruct(name2, _) => { ==(name1, name2) },
            IsStruct, { .. }, _ => { False },
            _, IsStruct, { .. } => { False },
            _, _ => { True },
        }
    }

    fn compatiblePtr(__0: CType, __1: CType) -> CType {
        match (__0, __1) {
            IsPtr(_, IsVoid), b => { b },
            IsArray(mut, _, el), b => { compatiblePtr((IsPtr(mut, el)), b) },
            a, IsPtr(_, IsVoid) => { a },
            a, IsArray(mut, _, el) => { compatiblePtr(a, (IsPtr(mut, el))) },
            IsPtr(m1, a), IsPtr(m2, b) => { IsPtr((leastMutable(m1, m2)), (compatiblePtr(a, b))) },
            _, _ => { IsVoid },
        }
    }

    fn completeType(__0: CType, __1: EnvMonad(s, CType)) -> EnvMonad(s, CType) {
        match (__0, __1, __2) {
            orig, <todo>, IsIncomplete(ident) => { {

            } },
            ty => { return(ty) },
        }
    }

    fn compound(expr: CExpr, returnOld: Bool, demand: Bool, op: CAssignOp, lhs: Result, rhs: Result) -> EnvMonad(s, Result) {
        {

        }
    }

    fn derivedDeferredTypeOf(deferred: EnvMonad(s, IntermediateType), declr: CDeclr, <todo>: Vec<CDecl>, CDeclr(_, derived, _, _, _): EnvMonad(s, EnvMonad(s, IntermediateType))) -> EnvMonad(s, EnvMonad(s, IntermediateType)) {
        {

        }
    }

    fn derivedTypeOf(deferred: EnvMonad(s, IntermediateType), declr: CDeclr) -> EnvMonad(s, IntermediateType) {
        join((derivedDeferredTypeOf(deferred, declr, vec![])))
    }

    fn designatorType(__0: Designator) -> CType {
        match (__0) {
            Base(ty) => { ty },
            From(ty, _, _, _) => { ty },
        }
    }

    fn emitIncomplete(kind: ItemKind, ident: Ident) -> EnvMonad(s, CType) {
        {

        }
    }

    fn emitItems(items: Vec<Rust.Item>) -> EnvMonad(s, ()) {
        lift(tell(mempty, hashmap! {
                "outputItems" => items
                }))
    }

    fn enumReprType() -> CType {
        IsInt(Signed, (BitWidth(32)))
    }

    fn exprToStatements(__0: Rust.Expr) -> Vec<Rust.Stmt> {
        match (__0) {
            Rust.IfThenElse(c, t, f) => { vec![Rust.Stmt((Rust.IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))] },
            Rust.BlockExpr(b) => { blockToStatements(b) },
            e => { vec![Rust.Stmt(e)] },
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
            IsBool => { IsInt(Signed, (BitWidth(32))) },
            IsEnum(_) => { enumReprType },
            x => { x },
        }
    }

    fn integerConversionRank(__0: IntWidth, __1: IntWidth) -> Maybe(Ordering) {
        match (__0, __1) {
            BitWidth(a), BitWidth(b) => { Just((compare(a, b))) },
            WordWidth, WordWidth => { Just(EQ) },
            _, _ => { Nothing },
        }
    }

    fn interpretBlockItem(__0: CBlockItem, __1: CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result)))) -> CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result))) {
        match (__0, __1) {
            CBlockStmt(stmt), next => { interpretStatement(stmt, next) },
            CBlockDecl(decl), next => { {

            } },
            item, _ => { lift(lift((unimplemented(item)))) },
        }
    }

    fn interpretConstExpr(__0: CExpr) -> EnvMonad(s, Integer) {
        match (__0) {
            CConst(CIntConst(CInteger(v, _, _), _)) => { return(v) },
            expr => { unimplemented(expr) },
        }
    }

    fn interpretDeclarations(__0: MakeBinding(s, b), __1: CDecl, __2: EnvMonad(s, Vec<b>)) -> EnvMonad(s, Vec<b>) {
        match (__0, __1, __2, __3) {
            (fromItem, makeBinding), declaration, <todo>, CDecl(specs, decls, _) => { {

            } },
            _, node, <todo>, CStaticAssert({ .. }) => { unimplemented(node) },
        }
    }

    fn interpretExpr(__0: Bool, __1: CExpr) -> EnvMonad(s, Result) {
        match (__0, __1) {
            demand, CComma(exprs, _) => { {

            } },
            demand, expr, <todo>, CAssign(op, lhs, rhs, _) => { {

            } },
            demand, expr, <todo>, CCond(c, Just(t), f, _) => { {

            } },
            _, expr, <todo>, CBinary(op, lhs, rhs, _) => { {

            } },
            _, CCast(decl, expr, _) => { {

            } },
            demand, node, <todo>, CUnary(op, expr, _) => { match op {
                    CPreIncOp => { incdec(False, CAddAssOp) },
                    CPreDecOp => { incdec(False, CSubAssOp) },
                    CPostIncOp => { incdec(True, CAddAssOp) },
                    CPostDecOp => { incdec(True, CSubAssOp) },
                    CAdrOp => { {

                    } },
                    CIndOp => { {

                    } },
                    CPlusOp => { {

                    } },
                    CMinOp => { fmap(wrapping)(simple(Rust.Neg)) },
                    CCompOp => { simple(Rust.Not) },
                    CNegOp => { {

                    } },
                } },
            _, CSizeofExpr(e, _) => { {

            } },
            _, CSizeofType(decl, _) => { {

            } },
            _, CAlignofExpr(e, _) => { {

            } },
            _, CAlignofType(decl, _) => { {

            } },
            _, expr, <todo>, CIndex(lhs, rhs, _) => { {

            } },
            _, expr, <todo>, CCall(func, args, _) => { {

            } },
            _, expr, <todo>, CMember(obj, ident, deref, node) => { {

            } },
            _, expr, <todo>, CVar(ident, _) => { {

            } },
            _, expr, <todo>, CConst(c) => { match c {
                    CIntConst, CInteger(v, repr, flags), _ => { Let(in, match allowed_types {
                            [] => { badSource(expr, "integer (too big)".to_string()) },
                            ty, :, _ => { return((literalNumber(ty, (Rust.LitInt(v, repr'))))) },
                        }) },
                    CFloatConst, CFloat(str), _ => { match span((Operator("notElem")("fF".to_string())), str) {
                            (v, "") => { return((literalNumber((IsFloat(64)), (Rust.LitFloat(v))))) },
                            (v, [_]) => { return((literalNumber((IsFloat(32)), (Rust.LitFloat(v))))) },
                            _ => { badSource(expr, "float".to_string()) },
                        } },
                    CCharConst, CChar(ch, False), _ => { return(Result, hashmap! {
                        "resultType" => charType,
                        "resultMutable" => Rust.Immutable,
                        "result" => Rust.Lit((Rust.LitByteChar(ch)))
                        }) },
                    CStrConst, CString(str, False), _ => { return(Result, hashmap! {
                        "resultType" => IsArray(Rust.Immutable, (+(length(str), 1)), charType),
                        "resultMutable" => Rust.Immutable,
                        "result" => Rust.Deref((Rust.Lit((Rust.LitByteStr((++(str, "N".to_string())))))))
                        }) },
                    _ => { unimplemented(expr) },
                } },
            _, CCompoundLit(decl, initials, info) => { {

            } },
            demand, stat, <todo>, CStatExpr(CCompound([], stmts, _), _) => { scope({

                }) },
            _, expr => { unimplemented(expr) },
        }
    }

    fn interpretFunction(CFunDef(specs, declr, <todo>, CDeclr(mident, _, _, _, _), argtypes, body, _): CFunDef) -> EnvMonad(s, ()) {
        {

        }
    }

    fn interpretInitializer(ty: CType, initial: CInit) -> EnvMonad(s, Rust.Expr) {
        {

        }
    }

    fn interpretStatement(__0: CStat, __1: CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result)))) -> CSourceBuildCFGT(s, (Vec<Rust.Stmt>, Terminator(Result))) {
        match (__0, __1) {
            CLabel(ident, body, _, _), next => { {

            } },
            stmt, <todo>, CCase(expr, body, node), next => { {

            } },
            stmt, <todo>, CCases(lower, upper, body, node), next => { {

            } },
            CDefault(body, _), next => { addSwitchCase(Nothing, body, next) },
            CExpr(Nothing, _), next => { next },
            CExpr(Just(expr), _), next => { {

            } },
            CCompound([], items, _), next => { mapBuildCFGT((mapRWST(scope)))({

                }) },
            CIf(c, t, mf, _), next => { {

            } },
            stmt, <todo>, CSwitch(expr, body, node), next => { {

            } },
            CWhile(c, body, doWhile, _), next => { {

            } },
            CFor(initial, mcond, mincr, body, _), next => { {

            } },
            CGoto(ident, _), next => { {

            } },
            stmt, <todo>, CCont(_), next => { {

            } },
            stmt, <todo>, CBreak(_), next => { {

            } },
            stmt, <todo>, CReturn(expr, _), next => { {

            } },
            stmt, _ => { lift(lift(unimplemented(stmt))) },
        }
    }

    fn interpretTranslationUnit(_thisModule: ModuleMap, rewrites: ItemRewrites, CTranslUnit(decls, _): CTranslUnit) -> Either(String, Vec<Rust.Item>) {
        match err {
                Left, msg => { Left(msg) },
                Right, _ => { Right(items') },
            }
    }

    fn makeLetBinding() -> MakeBinding(s, Rust.Stmt) {
        (Rust.StmtItem(vec![]), makeBinding)
    }

    fn makeStaticBinding() -> MakeBinding(s, Rust.Item) {
        (Rust.Item(vec![], Rust.Private), makeBinding)
    }

    fn modifyGlobal(f: fn(GlobalState) -> (GlobalState, a)) -> EnvMonad(s, a) {
        lift({

            })
    }

    fn mutable(quals: Vec<CTypeQualifier(a)>) -> Rust.Mutable {
        if(any, (Lambda), quals, then, Rust.Immutable, else, Rust.Mutable)
    }

    fn nestedObject(ty: CType, desig: Designator) -> Maybe(Designator) {
        match designatorType(desig) {
                IsArray, _, size, el => { Just((From(el, 0, (replicate((-(size, 1)), el)), desig))) },
            ty' => if compatibleInitializer(ty, ty') { Just(desig) },
                IsStruct, _, (_, ty')(:, fields) => { nestedObject(ty, (From(ty', 0, (map(snd, fields)), desig))) },
                _ => { Nothing },
            }
    }

    fn nextObject(__0: Designator, __1: CurrentObject) -> CurrentObject {
        match (__0, __1) {
            Base, { .. } => { Nothing },
            From(_, i, ty(:, remaining), base) => { Just((From(ty, (+(i, 1)), remaining, base))) },
            From(_, _, [], base) => { nextObject(base) },
        }
    }

    fn noTranslation(node: node, msg: String) -> EnvMonad(s, a) {
        throwE(concat(vec![show((posOf(node))), ": ".to_string(), msg, ":\\n".to_string(), render((nest(4, (pretty(node)))))]))
    }

    fn objectFromDesignators(__0: CType, __1: Vec<CDesignator>) -> EnvMonad(s, CurrentObject) {
        match (__0, __1) {
            _, [] => { pure(Nothing) },
            ty, desigs => { <$>(Just, go(ty, desigs, (Base(ty)))) },
        }
    }

    fn promote(node: node, op: fn(Rust.Expr) -> fn(Rust.Expr) -> Rust.Expr, a: Result, b: Result) -> EnvMonad(s, Result) {
        match usual((resultType(a)), (resultType(b))) {
                Just, rt => { return(Result, hashmap! {
                    "resultType" => rt,
                    "resultMutable" => Rust.Immutable,
                    "result" => op((castTo(rt, a)), (castTo(rt, b)))
                    }) },
                Nothing => { badSource(node)(concat(vec!["arithmetic combination for ".to_string(), show((resultType(a))), " and ".to_string(), show((resultType(b)))])) },
            }
    }

    fn promotePtr(node: node, op: fn(Rust.Expr) -> fn(Rust.Expr) -> Rust.Expr, a: Result, b: Result) -> EnvMonad(s, Result) {
        match (resultType(a), resultType(b)) {
                (IsArray(_, _, _), _) => { ptrs },
                (IsPtr(_, _), _) => { ptrs },
                (_, IsArray(_, _, _)) => { ptrs },
                (_, IsPtr(_, _)) => { ptrs },
                _ => { promote(node, op, a, b) },
            }
    }

    fn resolveCurrentObject((obj0, prior): (CurrentObject, Initializer), (obj1, cinitial): (CurrentObject, CInit)) -> EnvMonad(s, (CurrentObject, Initializer)) {
        match mplus(obj1, obj0) {
                Nothing => { return((Nothing, prior)) },
                Just, obj => { {

                } },
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
            [Rust.Stmt(Rust.BlockExpr(stmts))] => { stmts },
            stmts => { Rust.Block(stmts, Nothing) },
        }
    }

    fn toBool(__0: Result) -> Rust.Expr {
        match (__0) {
            Result({ .. }) => { Rust.Lit((Rust.LitBool(False))) },
            Result({ .. }) => { Rust.Lit((Rust.LitBool(True))) },
            Result({ .. }) => { match t {
                    IsBool => { v },
                    IsPtr, _, _ => { Rust.Not((Rust.MethodCall(v, (Rust.VarName("is_null".to_string())), vec![]))) },
                    _ => { Rust.CmpNE(v, 0) },
                } },
        }
    }

    fn toNotBool(__0: Result) -> Rust.Expr {
        match (__0) {
            Result({ .. }) => { Rust.Lit((Rust.LitBool(True))) },
            Result({ .. }) => { Rust.Lit((Rust.LitBool(False))) },
            Result({ .. }) => { match t {
                    IsBool => { Rust.Not(v) },
                    IsPtr, _, _ => { Rust.MethodCall(v, (Rust.VarName("is_null".to_string())), vec![]) },
                    _ => { Rust.CmpEQ(v, 0) },
                } },
        }
    }

    fn toPtr(__0: Result, __1: Maybe(Result)) -> Maybe(Result) {
        match (__0, __1, __2) {
            ptr, <todo>, Result({ .. }) => { Just(ptr, hashmap! {
                "resultType" => IsPtr(mut, el),
                "result" => castTo((IsPtr(mut, el)), ptr)
                }) },
            ptr, <todo>, Result({ .. }) => { Just(ptr) },
            _ => { Nothing },
        }
    }

    fn toRustRetType(__0: CType) -> Rust.Type {
        match (__0) {
            IsVoid => { Rust.TypeName("()".to_string()) },
            ty => { toRustType(ty) },
        }
    }

    fn toRustType(__0: CType) -> Rust.Type {
        match (__0) {
            IsBool => { Rust.TypeName("bool".to_string()) },
            IsInt(s, w) => { Rust.TypeName((:((match s {
                                Signed => { "\"\"".to_string() },
                                Unsigned => { "\"\"".to_string() },
                            }), (match w {
                                BitWidth, b => { show(b) },
                                WordWidth => { "size".to_string() },
                            })))) },
            IsFloat(w) => { Rust.TypeName((:("\"\"".to_string(), show(w)))) },
            IsVoid => { Rust.TypeName("::std::os::raw::c_void".to_string()) },
            IsFunc(retTy, args, variadic) => { Rust.TypeName(concat(vec!["unsafe extern fn(".to_string(), args', ")".to_string(), /=(if(retTy), ++(IsVoid(then, " -> ".to_string()), typename(retTy, else, "".to_string())))])) },
            IsPtr(mut, to) => { Let },
            IsArray(_, size, el) => { Rust.TypeName((++("[".to_string(), ++(typename(el), ++("; ".to_string(), ++(show(size), "]".to_string())))))) },
            IsStruct(name, _fields) => { Rust.TypeName(name) },
            IsEnum(name) => { Rust.TypeName(name) },
            IsIncomplete(ident) => { Rust.TypeName((identToString(ident))) },
        }
    }

    fn translateInitList(ty: CType, list: CInitList) -> EnvMonad(s, Initializer) {
        {

        }
    }

    fn typeName(__0: CDecl, __1: EnvMonad(s, (Rust.Mutable, CType))) -> EnvMonad(s, (Rust.Mutable, CType)) {
        match (__0, __1, __2) {
            decl, <todo>, CStaticAssert({ .. }) => { badSource(decl, "static assert in type name ".to_string()) },
            decl, <todo>, CDecl(spec, declarators, _) => { {

            } },
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

    fn useForwardRef(ident: Ident) -> EnvMonad(s, ()) {
        modifyGlobal(Lambda)
    }

    fn usual(__0: CType, __1: CType) -> Maybe(CType) {
        match (__0, __1) {
            IsFloat(aw), IsFloat(bw) => { Just((IsFloat((max(aw, bw))))) },
            a, <todo>, IsFloat(_), _ => { Just(a) },
            _, b, <todo>, IsFloat(_) => { Just(b) },
            origA, origB => { match (intPromote(origA), intPromote(origB)) {
                (a, b) => if ==(a, b) { Just(a) },
                    (IsInt(Signed, sw), IsInt(Unsigned, uw)) => { mixedSign(sw, uw) },
                    (IsInt(Unsigned, uw), IsInt(Signed, sw)) => { mixedSign(sw, uw) },
                    (IsInt(as, aw), IsInt(_bs, bw)) => { {

                    } },
                    _ => { Nothing },
                } },
        }
    }

    fn wrapMain(declr: CDeclr, realName: String, argTypes: Vec<CType>) -> EnvMonad(s, ()) {
        {

        }
    }

    fn wrapping(__0: Result, __1: Result) -> Result {
        match (__0, __1, __2) {
            r, <todo>, Result({ .. }) => { match result(r) {
                    Rust.Add, lhs, rhs => { r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_add".to_string())), vec![rhs])
                        }) },
                    Rust.Sub, lhs, rhs => { r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_sub".to_string())), vec![rhs])
                        }) },
                    Rust.Mul, lhs, rhs => { r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_mul".to_string())), vec![rhs])
                        }) },
                    Rust.Div, lhs, rhs => { r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_div".to_string())), vec![rhs])
                        }) },
                    Rust.Mod, lhs, rhs => { r(hashmap! {
                        "result" => Rust.MethodCall(lhs, (Rust.VarName("wrapping_rem".to_string())), vec![rhs])
                        }) },
                    Rust.Neg, e => { r(hashmap! {
                        "result" => Rust.MethodCall(e, (Rust.VarName("wrapping_neg".to_string())), vec![])
                        }) },
                    _ => { r },
                } },
            r => { r },
        }
    }

}

mod Language_Rust_Corrode_CFG {
    struct BasicBlock(BasicBlock, s, Terminator(c));

    #[derive(Debug)]
    struct Terminator'(Unreachable, Branch, l, CondBranch, c, l, l);

    struct CFG(CFG, Label, IntMap.IntMap(BasicBlock(s, c)));

    struct BuildState(BuildState, { /* struct def */ });

    #[derive(Debug)]
    struct StructureLabel(GoTo, { /* struct def */ }, ExitTo, { /* struct def */ }, Nested, Vec<Structure(s, c)>);

    #[derive(Debug)]
    struct Structure'(Simple, s, StructureTerminator(s, c), Loop, a, Multiple, IntMap.IntMap(a), a);

    #[derive(Debug)]
    struct Structure(Structure, { /* struct def */ });

    fn addBlock(label: Label, stmt: s, terminator: Terminator(c)) -> BuildCFGT(m, s, c, ()) {
        {

        }
    }

    fn buildCFG(root: BuildCFGT(m, s, c, Label)) -> m(CFG(Unordered, s, c)) {
        {

        }
    }

    fn depthFirstOrder(CFG(start, blocks): CFG(k, s, c)) -> CFG(DepthFirst, s, c) {
        CFG(start', blocks')
    }

    fn flipEdges(edges: IntMap.IntMap(IntSet.IntSet)) -> IntMap.IntMap(IntSet.IntSet) {
        IntMap.unionsWith(IntSet.union, Dummy)
    }

    fn hasMultiple() -> Bool {
        any(((go . structureBody)))
    }

    fn mapBuildCFGT() -> BuildCFGT(n, s, c, b) {
        mapStateT
    }

    fn newLabel() -> BuildCFGT(m, s, c, Label) {
        {

        }
    }

    fn outEdges(blocks: IntMap.IntMap(StructureBlock(s, c))) -> IntSet.IntSet {
        IntSet.difference(IntSet.unions((map(successors)(IntMap.elems(blocks)))), IntMap.keysSet(blocks))
    }

    fn partitionMembers(a: IntSet.IntSet, b: IntSet.IntSet) -> (IntSet.IntSet, IntSet.IntSet) {
        (IntSet.intersection(a, b), IntSet.difference(a, b))
    }

    fn prettyCFG(fmtS: fn(s) -> Doc, fmtC: fn(c) -> Doc, CFG(entry, blocks): CFG(k, s, c)) -> Doc {
        vcat(:((<>(text("start @".to_string()), text((show(entry))))), blocks'))
    }

    fn prettyStructure() -> Doc {
        (vcat . map(go))
    }

    fn relooper(entries: IntSet.IntSet, blocks: IntMap.IntMap(StructureBlock(s, c))) -> Vec<Structure(s, c)> {
        Let(in, match (IntSet.toList(noreturns), IntSet.toList(returns)) {
                ([], []) => { vec![] },
                ([entry], []) => { match IntMap.updateLookupWithKey((Lambda), entry, blocks) {
                        (Just((s, term)), blocks') => { :(Structure(hashmap! {
                                "structureEntries" => entries,
                                "structureBody" => Simple(s, term)
                                }), relooper((successors((s, term))), blocks')) },
                        (Nothing, _) => { :(Structure(hashmap! {
                                "structureEntries" => entries,
                                "structureBody" => Simple(mempty, (Branch((GoTo(entry)))))
                                }), vec![]) },
                    } },
            _ => if not((IntSet.null(absent))) { :(if(IntSet.null, present, then, vec![], else, Structure, hashmap! {
                    "structureEntries" => entries,
                    "structureBody" => Multiple((IntMap.fromSet((const(vec![])), absent)), (relooper(present, blocks)))
                    }), vec![]) },
                ([], _) => { :(Structure(hashmap! {
                        "structureEntries" => entries,
                        "structureBody" => Loop((relooper(entries, blocks')))
                        }), relooper(followEntries, followBlocks)) },
                _ => { :(Structure(hashmap! {
                        "structureEntries" => entries,
                        "structureBody" => Multiple(handlers, unhandled)
                        }), relooper(followEntries, followBlocks)) },
            })
    }

    fn relooperRoot(CFG(entry, blocks): CFG(k, s, c)) -> Vec<Structure(s, c)> {
        relooper((IntSet.singleton(entry)))(IntMap.map((Lambda), blocks))
    }

    fn removeEmptyBlocks(CFG(start, blocks): CFG(k, f(s), c)) -> CFG(Unordered, f(s), c) {
        CFG((rewrite(start)), blocks')
    }

    fn restrictKeys(m: IntMap.IntMap(a), s: IntSet.IntSet) -> IntMap.IntMap(a) {
        IntMap.intersection(m, IntMap.fromSet((const(())), s))
    }

    fn simplifyStructure() -> Vec<Structure(s, c)> {
        (foldr(go, vec![]) . map(descend))
    }

    fn structureCFG(mkBreak: fn(Maybe(Label)) -> s, mkContinue: fn(Maybe(Label)) -> s, mkLoop: fn(Label) -> fn(s) -> s, mkIf: fn(c) -> fn(s) -> fn(s) -> s, mkGoto: fn(Label) -> s, mkMatch: fn(Vec<(Label, s)>) -> fn(s) -> s, cfg: CFG(DepthFirst, s, c)) -> (Bool, s) {
        (hasMultiple(root), foo(vec![], mempty, root))
    }

    fn successors((_, term): StructureBlock(s, c)) -> IntSet.IntSet {
        IntSet.fromList(Dummy)
    }

}

mod Language_Rust_Corrode_CrateMap {
    #[derive(Debug, Eq, Ord)]
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
            Rust.Item(attrs, vis, Rust.Function(fattrs, name, formals, ret, b)) => { Rust.Item(attrs, vis, (Rust.Function(fattrs, name, formals, ret, (tailBlock(b))))) },
            i => { i },
        }
    }

    fn tailBlock(__0: Rust.Block) -> Rust.Block {
        match (__0) {
            Rust.Block(b, Just((tailExpr -> Just(e)))) => { Rust.Block(b, e) },
            Rust.Block((unsnoc -> Just((b, Rust.Stmt((tailExpr -> Just(e)))))), Nothing) => { Rust.Block(b, e) },
            b => { b },
        }
    }

    fn tailExpr(__0: Rust.Expr) -> Maybe(Maybe(Rust.Expr)) {
        match (__0) {
            Rust.Return(e) => { Just(e) },
            Rust.BlockExpr(b) => { Just((Just((Rust.BlockExpr((tailBlock(b))))))) },
            Rust.IfThenElse(c, t, f) => { Just((Just((Rust.IfThenElse(c, (tailBlock(t)), (tailBlock(f))))))) },
            _ => { Nothing },
        }
    }

    fn unsnoc(__0: Vec<a>) -> Maybe((Vec<a>, a)) {
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
