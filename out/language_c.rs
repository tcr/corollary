mod Language_C_Analysis_AstAnalysis {
    struct StmtCtx(FunCtx, VarDecl, LoopCtx, SwitchCtx);

    #[derive(Debug, Eq)]
    struct ExprSide(LValue, RValue);

    fn advanceDesigList(ds: Vec<CDesignator>, d: CDesignator) -> Vec<CDesignator> {
        drop(1)(dropWhile(((not . matchDesignator(d))), ds))
    }

    fn analyseAST(CTranslUnit(decls, _file_node): CTranslUnit) -> m(GlobalDecls) {
        {

        }
    }

    fn analyseExt(__0: CExtDecl) -> m(()) {
        match (__0) {
            CAsmExt(asm, _) => { handleAsmBlock(asm) },
            CFDefExt(fundef) => { analyseFunDef(fundef) },
            CDeclExt(decl) => { analyseDecl(False, decl) },
        }
    }

    fn analyseFunDef(CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info): CFunDef) -> m(()) {
        {

        }
    }

    fn analyseFunctionBody(__0: NodeInfo, __1: VarDecl, __2: CStat, __3: m(Stmt)) -> m(Stmt) {
        match (__0, __1, __2, __3, __4) {
            node_info, decl, s, <todo>, CCompound(localLabels, items, _) => { {

            } },
            _, _, s => { astError((nodeInfo(s)), "Function body is no compound statement".to_string()) },
        }
    }

    fn analyseTypeDef(handle_sue_def: Bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, node_info: NodeInfo) -> m(()) {
        {

        }
    }

    fn builtinType(__0: CBuiltin) -> m(Type) {
        match (__0) {
            CBuiltinVaArg(_, d, _) => { analyseTypeDecl(d) },
            CBuiltinOffsetOf(_, _, _) => { return(size_tType) },
            CBuiltinTypesCompatible(_, _, _) => { return(boolType) },
        }
    }

    fn checkGuard(c: Vec<StmtCtx>, e: CExpr) -> m(()) {
        >>=(tExpr(c, RValue, e), checkScalar'((nodeInfo(e))))
    }

    fn checkInits(__0: Type, __1: Vec<CDesignator>, __2: CInitList) -> m(()) {
        match (__0, __1, __2) {
            _, _, [] => { return(()) },
            t, dds, (ds, i)(:, is) => { {

            } },
        }
    }

    fn complexBaseType(ni: NodeInfo, c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m(Type) {
        {

        }
    }

    fn computeFunDefStorage(__0: Ident, __1: StorageSpec) -> m(Storage) {
        match (__0, __1) {
            _, StaticSpec(b) => { return(FunLinkage(InternalLinkage)) },
            ident, other_spec => { {

            } },
        }
    }

    fn defaultMD() -> MachineDesc {
        MachineDesc {
            iSize: Lambda,
            fSize: Lambda,
            builtinSize: Lambda,
            ptrSize: 4,
            voidSize: 1,
            iAlign: Lambda,
            fAlign: Lambda,
            builtinAlign: Lambda,
            ptrAlign: 4,
            voidAlign: 1
            }
    }

    fn defineParams(ni: NodeInfo, decl: VarDecl) -> m(()) {
        match (getParams(declType(decl))) {
                Nothing => { astError(ni, "expecting complete function type in function definition".to_string()) },
                Just, params => { mapM_(handleParamDecl, params) },
            }
    }

    fn enclosingFunctionType(__0: Vec<StmtCtx>) -> Maybe(Type) {
        match (__0) {
            [] => { Nothing },
            FunCtx(vd, :, _) => { Just(declType(vd)) },
            _(:, cs) => { enclosingFunctionType(cs) },
        }
    }

    fn extFunProto(VarDeclInfo(var_name, is_inline, storage_spec, attrs, ty, node_info): VarDeclInfo) -> m(()) {
        {

        }
    }

    fn extVarDecl(VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Maybe(Initializer)) -> m(()) {
        {

        }
    }

    fn getParams(__0: Type) -> Maybe(Vec<ParamDecl>) {
        match (__0) {
            FunctionType(FunType(_, params, _), _) => { Just(params) },
            _ => { Nothing },
        }
    }

    fn hasTypeDef(declspecs: Vec<CDeclSpec>) -> Maybe(Vec<CDeclSpec>) {
        match foldr(hasTypeDefSpec, (False, vec![]), declspecs) {
                (True, specs') => { Just(specs') },
                (False, _) => { Nothing },
            }
    }

    fn inLoop(c: Vec<StmtCtx>) -> Bool {
        any(isLoop, c)
    }

    fn inSwitch(c: Vec<StmtCtx>) -> Bool {
        any(isSwitch, c)
    }

    fn localVarDecl(VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Maybe(Initializer)) -> m(()) {
        {

        }
    }

    fn matchDesignator(__0: CDesignator, __1: CDesignator) -> Bool {
        match (__0, __1) {
            CMemberDesig(m1, _), CMemberDesig(m2, _) => { ==(m1, m2) },
            _, _ => { True },
        }
    }

    fn tBlockItem(__0: Vec<StmtCtx>, __1: CBlockItem) -> m(Type) {
        match (__0, __1) {
            c, CBlockStmt(s) => { tStmt(c, s) },
            _, CBlockDecl(d) => { >>(analyseDecl(True, d), return(voidType)) },
            _, CNestedFunDef(fd) => { >>(analyseFunDef(fd), return(voidType)) },
        }
    }

    fn tDesignator(__0: Type, __1: Vec<CDesignator>) -> m(Type) {
        match (__0, __1) {
            ArrayType(bt, _, _, _), CArrDesig(e, ni, :, ds) => { {

            } },
            ArrayType(bt, _, _, _), CRangeDesig(e1, e2, ni, :, ds) => { {

            } },
            ArrayType(_, _, _, _), d(:, ds) => { typeError((nodeInfo(d)), "member designator in array initializer".to_string()) },
            t, <todo>, DirectType(TyComp(_), _, _), CMemberDesig(m, ni, :, ds) => { {

            } },
            t, <todo>, DirectType(TyComp(_), _, _), d(:, _) => { typeError((nodeInfo(d)), "array designator in compound initializer".to_string()) },
            t, [] => { return(t) },
        }
    }

    fn tExpr(c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m(Type) {
        match nameOfNode((nodeInfo(e))) {
                Just, n => { {

                } },
                Nothing => { tExpr'(c, side, e) },
            }
    }

    fn tExpr'(__0: Vec<StmtCtx>, __1: ExprSide, __2: CExpr) -> m(Type) {
        match (__0, __1, __2) {
            c, side, CBinary(op, le, re, ni) => { {

            } },
            c, side, CUnary(CAdrOp, e, ni) => { {

            } },
            c, _, CUnary(CIndOp, e, ni) => { >>=(tExpr(c, RValue, e), ((typeErrorOnLeft(ni) . derefType))) },
            c, _, CUnary(CCompOp, e, ni) => { {

            } },
            c, side, CUnary(CNegOp, e, ni) => { {

            } },
            c, side, CUnary(op, e, _) => { tExpr(c, (if(isEffectfulOp, op, then, LValue, else, side)), e) },
            c, _, CIndex(b, i, ni) => { {

            } },
            c, side, CCond(e1, me2, e3, ni) => { {

            } },
            c, side, CMember(e, m, deref, ni) => { {

            } },
            c, side, CComma(es, _) => { >>=(mapM((tExpr(c, side)), es), (return . last)) },
            c, side, CCast(d, e, ni) => { {

            } },
            c, side, CSizeofExpr(e, ni) => { {

            } },
            c, side, CAlignofExpr(e, ni) => { {

            } },
            c, side, CComplexReal(e, ni) => { complexBaseType(ni, c, side, e) },
            c, side, CComplexImag(e, ni) => { complexBaseType(ni, c, side, e) },
            _, side, CLabAddrExpr(_, ni) => { {

            } },
            _, side, CCompoundLit(d, initList, ni) => { {

            } },
            _, RValue, CAlignofType(_, _) => { return(size_tType) },
            _, RValue, CSizeofType(_, _) => { return(size_tType) },
            _, LValue, CAlignofType(_, ni) => { typeError(ni, "alignoftype as lvalue".to_string()) },
            _, LValue, CSizeofType(_, ni) => { typeError(ni, "sizeoftype as lvalue".to_string()) },
            _, side, CVar(i, ni) => { >>=(lookupObject(i), maybe((typeErrorOnLeft(ni)(notFound(i))), ((return . declType)))) },
            _, _, CConst(c) => { constType(c) },
            _, _, CBuiltinExpr(b) => { builtinType(b) },
            c, _, CCall(fe, args, ni) => { {

            } },
            c, _, CAssign(op, le, re, ni) => { {

            } },
            c, _, CStatExpr(s, _) => { {

            } },
        }
    }

    fn tInit(__0: Type, __1: CInit, __2: m(Initializer)) -> m(Initializer) {
        match (__0, __1, __2, __3) {
            t, i, <todo>, CInitExpr(e, ni) => { {

            } },
            t, i, <todo>, CInitList(initList, ni) => { >>(tInitList(ni, (canonicalType(t)), initList), return(i)) },
        }
    }

    fn tInitList(__0: NodeInfo, __1: Type, __2: CInitList, __3: m(())) -> m(()) {
        match (__0, __1, __2, __3, __4) {
            ni, t, <todo>, ArrayType(DirectType(TyIntegral(TyChar), _, _), _, _, _), [([], CInitExpr(e, <todo>, CConst(CStrConst(_, _)), _))] => { >>(tExpr(vec![], RValue, e), return(())) },
            ni, t, <todo>, ArrayType(_, _, _, _), initList => { {

            } },
            ni, t, <todo>, DirectType(TyComp(ctr), _, _), initList => { {

            } },
            ni, PtrType(DirectType(TyVoid, _, _), _, _), _ => { return(()) },
            _, t, [([], i)] => { >>(tInit(t, i), return(())) },
            ni, t, _ => { typeError(ni)(++("initializer list for type: ".to_string(), pType(t))) },
        }
    }

    fn tStmt(__0: Vec<StmtCtx>, __1: CStat) -> m(Type) {
        match (__0, __1) {
            c, CLabel(_, s, _, _) => { tStmt(c, s) },
            c, CExpr(e, _) => { maybe((return(voidType)), (tExpr(c, RValue)), e) },
            c, CCompound(ls, body, _) => { {

            } },
            c, CIf(e, sthen, selse, _) => { >>(checkGuard(c, e), >>(tStmt(c, sthen), >>(maybe((return(())), (>>(Lambda(c, s), return(()))), selse), return(voidType)))) },
            c, CSwitch(e, s, ni) => { >>=(tExpr(c, RValue, e), >>(checkIntegral'(ni), tStmt((:(SwitchCtx, c)), s))) },
            c, CWhile(e, s, _, _) => { >>(checkGuard(c, e), tStmt((:(LoopCtx, c)), s)) },
            _, CGoto(l, ni) => { {

            } },
            c, CCont(ni) => { {

            } },
            c, CBreak(ni) => { {

            } },
            c, CReturn(Just(e), ni) => { {

            } },
            _, CReturn(Nothing, _) => { return(voidType) },
            _, CAsm(_, _) => { return(voidType) },
            c, CCase(e, s, ni) => { {

            } },
            c, CCases(e1, e2, s, ni) => { {

            } },
            c, CDefault(s, ni) => { {

            } },
            c, CFor(i, g, inc, s, _) => { {

            } },
            c, CGotoPtr(e, ni) => { {

            } },
        }
    }

}

mod Language_C_Analysis_Builtins {
    fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/ConstEval.hs"
Error: Unrecognized token `->`:
 21 | ;data MachineDesc =
 22 |   MachineDesc
 23 |   { iSize        :: IntType -> Integer
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Analysis_Debug {
    fn globalDeclStats(file_filter: fn(FilePath) -> Bool, gmap: GlobalDecls) -> Vec<(String, isize)> {
        vec![("Enumeration Constants".to_string(), Map.size(enumerators)), ("Total Object/Function Declarations".to_string(), Map.size(all_decls)), ("Object definitions".to_string(), Map.size(objDefs)), ("Function Definitions".to_string(), Map.size(funDefs)), ("Tag definitions".to_string(), Map.size(tagDefs)), ("TypeDefs".to_string(), Map.size(typeDefs))]
    }

    fn joinComma() -> Doc {
        (hsep . (punctuate(comma) . map(pretty)))
    }

    fn prettyAssocs(label: String) -> Doc {
        prettyAssocsWith(label, pretty, pretty)
    }

    fn prettyAssocsWith(label: String, prettyKey: fn(k) -> Doc, prettyVal: fn(v) -> Doc, theMap: Vec<(k, v)>) -> Doc {
        $$(text(label), (nest(8))((vcat(map(prettyEntry, theMap)))))
    }

    fn terminateSemi() -> Doc {
        (terminateSemi_ . map(pretty))
    }

    fn terminateSemi_() -> Doc {
        (hsep . map((Operator("<>")(semi))))
    }

}

mod Language_C_Analysis_DeclAnalysis {
    #[derive(Debug, Eq, Ord, Read)]
    struct StorageSpec(NoStorageSpec, AutoSpec, RegSpec, ThreadSpec, StaticSpec, Bool, ExternSpec, Bool);

    struct VarDeclInfo(VarDeclInfo, VarName, Bool, StorageSpec, Attributes, Type, NodeInfo);

    #[derive(Eq, Ord)]
    struct NumBaseType(NoBaseType, BaseChar, BaseInt, BaseFloat, BaseDouble);

    #[derive(Eq, Ord)]
    struct SignSpec(NoSignSpec, Signed, Unsigned);

    #[derive(Eq, Ord)]
    struct SizeMod(NoSizeMod, ShortMod, LongMod, LongLongMod);

    struct NumTypeSpec(NumTypeSpec, { /* struct def */ });

    struct TypeSpecAnalysis(TSNone, TSVoid, TSBool, TSNum, NumTypeSpec, TSTypeDef, TypeDefRef, TSType, Type, TSNonBasic, CTypeSpec);

    fn analyseVarDecl(handle_sue_def: Bool, storage_specs: Vec<CStorageSpec>, decl_attrs: Vec<CAttr>, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, inline: Bool, CDeclr(name_opt, derived_declrs, asmname_opt, declr_attrs, node): CDeclr, oldstyle_params: Vec<CDecl>, init_opt: Maybe(CInit)) -> m(VarDeclInfo) {
        {

        }
    }

    fn analyseVarDecl'(handle_sue_def: Bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, oldstyle: Vec<CDecl>, init_opt: Maybe(CInit)) -> m(VarDeclInfo) {
        {

        }
    }

    fn canonicalStorageSpec(storagespecs: Vec<CStorageSpec>) -> m(StorageSpec) {
        liftM(elideAuto)(foldrM(updStorage, NoStorageSpec, storagespecs))
    }

    fn canonicalTypeSpec() -> m(TypeSpecAnalysis) {
        foldrM(go, TSNone)
    }

    fn computeParamStorage(__0: NodeInfo, __1: StorageSpec) -> Either(BadSpecifierError, Storage) {
        match (__0, __1) {
            _, NoStorageSpec => { Right((Auto(False))) },
            _, RegSpec => { Right((Auto(True))) },
            node, spec => { (Left . badSpecifierError(node)(++("Bad storage specified for parameter: ".to_string(), show(spec)))) },
        }
    }

    fn emptyDeclr(node: NodeInfo) -> CDeclr {
        CDeclr(Nothing, vec![], Nothing, vec![], node)
    }

    fn emptyNumTypeSpec() -> NumTypeSpec {
        NumTypeSpec {
            base: NoBaseType,
            signSpec: NoSignSpec,
            sizeMod: NoSizeMod,
            isComplex: False
            }
    }

    fn getOnlyDeclr(__0: CDecl) -> m(CDeclr) {
        match (__0) {
            CDecl(_, [(Just(declr), _, _)], _) => { return(declr) },
            CDecl(_, _, node) => { internalErr("getOnlyDeclr: declaration doesn\'t have a unique declarator".to_string()) },
        }
    }

    fn hasThreadLocalSpec(__0: StorageSpec) -> Bool {
        match (__0) {
            ThreadSpec => { True },
            StaticSpec(b) => { b },
            ExternSpec(b) => { b },
            _ => { False },
        }
    }

    fn isTypeDef(declspecs: Vec<CDeclSpec>) -> Bool {
        not(null(Dummy))
    }

    fn mergeOldStyle(__0: NodeInfo, __1: Vec<CDecl>, __2: Vec<CDerivedDeclr>) -> m(Vec<CDerivedDeclr>) {
        match (__0, __1, __2) {
            _node, [], declrs => { return(declrs) },
            node, oldstyle_params, CFunDeclr(params, attrs, fdnode, :, dds) => { match params {
                    Left, list => { {

                    } },
                    Right, _newstyle => { astError(node, "oldstyle parameter list, but newstyle function declaration".to_string()) },
                } },
            node, _, _ => { astError(node, "oldstyle parameter list, but not function type".to_string()) },
        }
    }

    fn mergeTypeAttributes(node_info: NodeInfo, quals: TypeQuals, attrs: Vec<Attr>, typ: Type) -> m(Type) {
        match typ {
                DirectType, ty_name, quals', attrs' => { merge(quals', attrs')(mkDirect(ty_name)) },
                PtrType, ty, quals', attrs' => { merge(quals', attrs')(PtrType(ty)) },
                ArrayType, ty, array_sz, quals', attrs' => { merge(quals', attrs')(ArrayType(ty, array_sz)) },
                FunctionType, FunType(return_ty, params, inline), attrs' => { return(FunctionType((FunType(return_ty, params, inline)), (++(attrs', attrs)))) },
                TypeDefType, tdr, quals', attrs' => { merge(quals', attrs')(TypeDefType(tdr)) },
            }
    }

    fn mkVarName(__0: NodeInfo, __1: Maybe(Ident), __2: Maybe(AsmName)) -> m(VarName) {
        match (__0, __1, __2) {
            node, Nothing, _ => { return(NoName) },
            node, Just(n), asm => { return(VarName(n, asm)) },
        }
    }

    fn nameOfDecl(d: CDecl) -> m(Ident) {
        >>=(getOnlyDeclr(d), Lambda)
    }

    fn splitCDecl(decl: CDecl, <todo>: m(Vec<CDecl>)) -> m(Vec<CDecl>) {
        match declrs {
                [] => { internalErr("splitCDecl applied to empty declaration".to_string()) },
                [declr] => { return(vec![decl]) },
                d1:ds => { Let(return)(:((CDecl(declspecs, vec![d1], node)), Dummy)) },
            }
    }

    fn tArraySize(__0: CArrSize) -> m(ArraySize) {
        match (__0) {
            CNoArrSize(False) => { return((UnknownArraySize(False))) },
            CNoArrSize(True) => { return((UnknownArraySize(True))) },
            CArrSize(static, szexpr) => { liftM((ArraySize(static)), (return(szexpr))) },
        }
    }

    fn tAttr(CAttr(name, cexpr, node): CAttr) -> m(Attr) {
        return(Attr(name, cexpr, node))
    }

    fn tCompType(tag: SUERef, sue_ref: CompTyKind, member_decls: Vec<CDecl>, attrs: Attributes, node: NodeInfo) -> m(CompType) {
        ap(return((CompType(tag, sue_ref))), ap((concatMapM(tMemberDecls, member_decls)), ap((return(attrs)), (return(node)))))
    }

    fn tCompTypeDecl(handle_def: Bool, CStruct(tag, ident_opt, member_decls_opt, attrs, node_info): CStructUnion) -> m(CompTypeRef) {
        {

        }
    }

    fn tDirectType(handle_sue_def: Bool, node: NodeInfo, ty_quals: Vec<CTypeQual>, canonTySpec: TypeSpecAnalysis) -> m(Type) {
        {

        }
    }

    fn tEnumType(sue_ref: SUERef, enumerators: Vec<(Ident, Maybe(CExpr))>, attrs: Attributes, node: NodeInfo) -> m(EnumType) {
        {

        }
    }

    fn tMemberDecls(__0: CDecl) -> m(Vec<MemberDecl>) {
        match (__0) {
            CDecl(declspecs, [], node) => { {

            } },
            CDecl(declspecs, declrs, node) => { mapM((uncurry(tMemberDecl)), (zip((True:repeat(False)), declrs))) },
        }
    }

    fn tNumType(NumTypeSpec(basetype, sgn, sz, iscomplex): NumTypeSpec) -> m(Either((FloatType, Bool), IntType)) {
        match (basetype, sgn, sz) {
            (BaseChar, _, NoSizeMod) => if let Signed = sgn { intType(TySChar) }
let Unsigned = sgn { intType(TyUChar) }
otherwise { intType(TyChar) },
            (intbase, _, NoSizeMod) => if optBase(BaseInt, intbase) { intType(match sgn {
                        Unsigned => { TyUInt },
                        _ => { TyInt },
                    }) },
            (intbase, signed, sizemod) => if optBase(BaseInt, intbase) && optSign(Signed, signed) { intType(match sizemod {
                        ShortMod => { TyShort },
                        LongMod => { TyLong },
                        LongLongMod => { TyLLong },
                        _ => { internalErr("numTypeMapping: unexpected pattern matching error".to_string()) },
                    }) },
            (intbase, Unsigned, sizemod) => if optBase(BaseInt, intbase) { intType(match sizemod {
                        ShortMod => { TyUShort },
                        LongMod => { TyULong },
                        LongLongMod => { TyULLong },
                        _ => { internalErr("numTypeMapping: unexpected pattern matching error".to_string()) },
                    }) },
                (BaseFloat, NoSignSpec, NoSizeMod) => { floatType(TyFloat) },
                (BaseDouble, NoSignSpec, NoSizeMod) => { floatType(TyDouble) },
                (BaseDouble, NoSignSpec, LongMod) => { floatType(TyLDouble) },
                (_, _, _) => { error("Bad AST analysis".to_string()) },
            }
    }

    fn tParamDecl(CDecl(declspecs, declrs, node): CDecl) -> m(ParamDecl) {
        {

        }
    }

    fn tTag(__0: CStructTag) -> CompTyKind {
        match (__0) {
            CStructTag => { StructTag },
            CUnionTag => { UnionTag },
        }
    }

    fn tType(handle_sue_def: Bool, top_node: NodeInfo, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, derived_declrs: Vec<CDerivedDeclr>, oldstyle_params: Vec<CDecl>) -> m(Type) {
        >>=(mergeOldStyle(top_node, oldstyle_params, derived_declrs), buildType)
    }

    fn tTypeQuals() -> m((TypeQuals, Attributes)) {
        foldrM(go, (noTypeQuals, vec![]))
    }

    fn typeDefRef(t_node: NodeInfo, name: Ident) -> m(TypeDefRef) {
        >>=(lookupTypeDef(name), Lambda((TypeDefRef(name, (Just(ty)), t_node))))
    }

}

mod Language_C_Analysis_DefTable {
    struct TagFwdDecl(CompDecl, CompTypeRef, EnumDecl, EnumTypeRef);

    struct DefTable(DefTable, { /* struct def */ });

    #[derive(Clone, Debug)]
    struct DeclarationStatus(NewDecl, Redeclared, t, KeepDef, t, Shadowed, t, KindMismatch, t);

    #[derive(Eq, Ord)]
    struct TagEntryKind(CompKind, CompTyKind, EnumKind);

    fn compatIdentEntry(__0: IdentEntry) -> Bool {
        match (__0) {
            Left(_tydef) => { either((const(True)), (const(False))) },
            Right(def) => { either((const(False)))(Lambda) },
        }
    }

    fn compatTagEntry(te1: TagEntry, te2: TagEntry) -> Bool {
        ==(tagKind(te1), tagKind(te2))
    }

    fn declStatusDescr(__0: DeclarationStatus(t)) -> String {
        match (__0) {
            NewDecl => { "new".to_string() },
            Redeclared(_) => { "redeclared".to_string() },
            KeepDef(_) => { "keep old".to_string() },
            Shadowed(_) => { "shadowed".to_string() },
            KindMismatch(_) => { "kind mismatch".to_string() },
        }
    }

    fn declareTag(sueref: SUERef, decl: TagFwdDecl, deftbl: DefTable) -> (DeclarationStatus(TagEntry), DefTable) {
        match lookupTag(sueref, deftbl) {
                Nothing => { (NewDecl, deftbl {
                        tagDecls: fst(defLocal((tagDecls(deftbl)), sueref, (Left(decl))))
                        }) },
            Just, old_def => if ==(tagKind(old_def), tagKind((Left(decl)))) { (KeepDef(old_def), deftbl) }
otherwise { (KindMismatch(old_def), deftbl) },
            }
    }

    fn defRedeclStatus(sameKind: fn(t) -> fn(t) -> Bool, def: t, oldDecl: Maybe(t)) -> DeclarationStatus(t) {
        match oldDecl {
            Just, def' => if sameKind(def, def') { Redeclared(def') }
otherwise { KindMismatch(def') },
                Nothing => { NewDecl },
            }
    }

    fn defRedeclStatusLocal(sameKind: fn(t) -> fn(t) -> Bool, ident: k, def: t, oldDecl: Maybe(t), nsm: NameSpaceMap(k, t)) -> DeclarationStatus(t) {
        match defRedeclStatus(sameKind, def, oldDecl) {
                NewDecl => { match lookupName(nsm, ident) {
                        Just, shadowed => { Shadowed(shadowed) },
                        Nothing => { NewDecl },
                    } },
                redecl => { redecl },
            }
    }

    fn defineGlobalIdent(ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus(IdentEntry), DefTable) {
        (defRedeclStatus(compatIdentEntry, (Right(def)), oldDecl), deftbl {
                identDecls: decls'
                })
    }

    fn defineLabel(ident: Ident, deftbl: DefTable) -> (DeclarationStatus(Ident), DefTable) {
        Let(in, (maybe(NewDecl, Redeclared, old_label), deftbl {
                labelDefs: labels'
                }))
    }

    fn defineScopedIdent() -> (DeclarationStatus(IdentEntry), DefTable) {
        defineScopedIdentWhen((const(True)))
    }

    fn defineScopedIdentWhen(override_def: fn(IdentDecl) -> Bool, ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus(IdentEntry), DefTable) {
        (redecl_status, deftbl {
                identDecls: decls'
                })
    }

    fn defineTag(sueref: SUERef, def: TagDef, deftbl: DefTable) -> (DeclarationStatus(TagEntry), DefTable) {
        (redeclStatus, deftbl {
                tagDecls: decls'
                })
    }

    fn defineTypeDef(ident: Ident, tydef: TypeDef, deftbl: DefTable) -> (DeclarationStatus(IdentEntry), DefTable) {
        (defRedeclStatus(compatIdentEntry, (Left(tydef)), oldDecl), deftbl {
                identDecls: decls'
                })
    }

    fn emptyDefTable() -> DefTable {
        DefTable(nameSpaceMap, nameSpaceMap, nameSpaceMap, nameSpaceMap, IntMap.empty, IntMap.empty)
    }

    fn enterBlockScope(deftbl: DefTable) -> DefTable {
        enterLocalScope(deftbl {
                labelDefs: enterNewScope((labelDefs(deftbl)))
                })
    }

    fn enterFunctionScope(deftbl: DefTable) -> DefTable {
        enterLocalScope(deftbl {
                labelDefs: enterNewScope((labelDefs(deftbl)))
                })
    }

    fn enterLocalScope(deftbl: DefTable) -> DefTable {
        deftbl {
            identDecls: enterNewScope((identDecls(deftbl))),
            tagDecls: enterNewScope((tagDecls(deftbl)))
            }
    }

    fn enterMemberDecl(deftbl: DefTable) -> DefTable {
        deftbl {
            memberDecls: enterNewScope((memberDecls(deftbl)))
            }
    }

    fn globalDefs(deftbl: DefTable) -> GlobalDecls {
        Map.foldWithKey(insertDecl, (GlobalDecls(e, gtags, e)), (globalNames(identDecls(deftbl))))
    }

    fn identOfTyDecl() -> Ident {
        either(identOfTypeDef, declIdent)
    }

    fn inFileScope(dt: DefTable) -> Bool {
        not((||(hasLocalNames((identDecls(dt))), hasLocalNames((labelDefs(dt))))))
    }

    fn insertType(dt: DefTable, n: Name, t: Type) -> DefTable {
        dt {
            typeTable: IntMap.insert((nameId(n)), t, (typeTable(dt)))
            }
    }

    fn leaveBlockScope(deftbl: DefTable) -> DefTable {
        leaveLocalScope(deftbl {
                labelDefs: leaveScope_((labelDefs(deftbl)))
                })
    }

    fn leaveFunctionScope(deftbl: DefTable) -> DefTable {
        leaveLocalScope(deftbl {
                labelDefs: leaveScope_((labelDefs(deftbl)))
                })
    }

    fn leaveLocalScope(deftbl: DefTable) -> DefTable {
        deftbl {
            identDecls: leaveScope_((identDecls(deftbl))),
            tagDecls: leaveScope_((tagDecls(deftbl)))
            }
    }

    fn leaveMemberDecl(deftbl: DefTable) -> (Vec<MemberDecl>, DefTable) {
        Let(in, Dummy, (map(snd, members)), (deftbl {
                memberDecls: decls'
                }))
    }

    fn leaveScope_() -> NameSpaceMap(k, a) {
        (fst . leaveScope)
    }

    fn lookupIdent(ident: Ident, deftbl: DefTable) -> Maybe(IdentEntry) {
        lookupName((identDecls(deftbl)), ident)
    }

    fn lookupIdentInner(ident: Ident, deftbl: DefTable) -> Maybe(IdentEntry) {
        lookupInnermostScope((identDecls(deftbl)), ident)
    }

    fn lookupLabel(ident: Ident, deftbl: DefTable) -> Maybe(Ident) {
        lookupName((labelDefs(deftbl)), ident)
    }

    fn lookupTag(sue_ref: SUERef, deftbl: DefTable) -> Maybe(TagEntry) {
        lookupName((tagDecls(deftbl)), sue_ref)
    }

    fn lookupTagInner(sue_ref: SUERef, deftbl: DefTable) -> Maybe(TagEntry) {
        lookupInnermostScope((tagDecls(deftbl)), sue_ref)
    }

    fn lookupType(dt: DefTable, n: Name) -> Maybe(Type) {
        IntMap.lookup((nameId(n)), (typeTable(dt)))
    }

    fn mergeDefTable(DefTable(i1, t1, l1, m1, r1, tt1): DefTable, DefTable(i2, t2, l2, m2, r2, tt2): DefTable) -> DefTable {
        DefTable((mergeNameSpace(i1, i2)), (mergeNameSpace(t1, t2)), (mergeNameSpace(l1, l2)), (mergeNameSpace(m1, m2)), (union(r1, r2)), (union(tt1, tt2)))
    }

    fn tagKind(__0: TagEntry) -> TagEntryKind {
        match (__0) {
            Left(CompDecl(cd)) => { CompKind((compTag(cd))) },
            Left(EnumDecl(_)) => { EnumKind },
            Right(CompDef(cd)) => { CompKind((compTag(cd))) },
            Right(EnumDef(_)) => { EnumKind },
        }
    }

}

mod Language_C_Analysis_Export {
    fn exportArraySize(__0: ArraySize) -> CArrSize {
        match (__0) {
            ArraySize(static, e) => { CArrSize(static, e) },
            UnknownArraySize(complete) => { CNoArrSize(complete) },
        }
    }

    fn exportAttrs() -> Vec<CAttr> {
        map(exportAttr)
    }

    fn exportCompType(CompType(sue_ref, comp_tag, members, attrs, node_info): CompType) -> Vec<CTypeSpec> {
        vec![CSUType(comp, ni)]
    }

    fn exportCompTypeDecl(ty: CompTypeRef) -> Vec<CTypeSpec> {
        vec![CSUType((exportComp(ty)), ni)]
    }

    fn exportCompTypeRef(CompType(sue_ref, com_tag, _, _, node_info): CompType) -> Vec<CTypeSpec> {
        exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
    }

    fn exportComplexType(ty: FloatType) -> Vec<CTypeSpec> {
        :((CComplexType(ni)), exportFloatType(ty))
    }

    fn exportDeclAttrs(DeclAttrs(inline, storage, attrs): DeclAttrs) -> Vec<CDeclSpec> {
        ++((if(inline, then, vec![CTypeQual((CInlineQual(ni)))], else, vec![])), ++(map((CStorageSpec), (exportStorage(storage))), map(((CTypeQual . CAttrQual)), (exportAttrs(attrs)))))
    }

    fn exportDeclr(other_specs: Vec<CDeclSpec>, ty: Type, attrs: Attributes, name: VarName) -> (Vec<CDeclSpec>, CDeclr) {
        (++(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
    }

    fn exportEnumType(EnumType(sue_ref, enumerators, attrs, node_info): EnumType) -> Vec<CTypeSpec> {
        vec![CEnumType(enum, ni)]
    }

    fn exportEnumTypeDecl(ty: EnumTypeRef) -> Vec<CTypeSpec> {
        vec![CEnumType((exportEnum(ty)), ni)]
    }

    fn exportEnumTypeRef(EnumType(sue_ref, _, _, node_info): EnumType) -> Vec<CTypeSpec> {
        exportEnumTypeDecl((EnumTypeRef(sue_ref, node_info)))
    }

    fn exportFloatType(ty: FloatType) -> Vec<CTypeSpec> {
        match ty {
                TyFloat => { vec![CFloatType(ni)] },
                TyDouble => { vec![CDoubleType(ni)] },
                TyLDouble => { vec![CLongType(ni), CDoubleType(ni)] },
            }
    }

    fn exportIntType(ty: IntType) -> Vec<CTypeSpec> {
        match ty {
                TyBool => { vec![CBoolType(ni)] },
                TyChar => { vec![CCharType(ni)] },
                TySChar => { vec![CSignedType(ni), CCharType(ni)] },
                TyUChar => { vec![CUnsigType(ni), CCharType(ni)] },
                TyShort => { vec![CShortType(ni)] },
                TyUShort => { vec![CUnsigType(ni), CShortType(ni)] },
                TyInt => { vec![CIntType(ni)] },
                TyUInt => { vec![CUnsigType(ni), CIntType(ni)] },
                TyLong => { vec![CLongType(ni)] },
                TyULong => { vec![CUnsigType(ni), CLongType(ni)] },
                TyLLong => { vec![CLongType(ni), CLongType(ni)] },
                TyULLong => { vec![CUnsigType(ni), CLongType(ni), CLongType(ni)] },
            }
    }

    fn exportMemberDecl(__0: MemberDecl) -> CDecl {
        match (__0) {
            AnonBitField(ty, expr, node_info) => { CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info) },
            MemberDecl(vardecl, bitfieldsz, node_info) => { Let(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info) },
        }
    }

    fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
        Let(in, CDecl, specs, vec![(Just(declr), Nothing, Nothing)], (nodeInfo(paramdecl)))
    }

    fn exportSUERef() -> Maybe(Ident) {
        (Just . (internalIdent . show))
    }

    fn exportStorage(__0: Storage) -> Vec<CStorageSpec> {
        match (__0) {
            NoStorage => { vec![] },
            Auto(reg) => { if(reg, then, vec![CRegister(ni)], else, vec![]) },
            Static(InternalLinkage, thread_local) => { threadLocal(thread_local, vec![CStatic(ni)]) },
            Static(ExternalLinkage, thread_local) => { threadLocal(thread_local, vec![CExtern(ni)]) },
            Static(NoLinkage, _) => { error("impossible storage: static without linkage".to_string()) },
            FunLinkage(InternalLinkage) => { vec![CStatic(ni)] },
            FunLinkage(ExternalLinkage) => { vec![] },
            FunLinkage(NoLinkage) => { error("impossible storage: function without linkage".to_string()) },
        }
    }

    fn exportType(ty: Type) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {
        exportTy(vec![], ty)
    }

    fn exportTypeDecl(ty: Type) -> CDecl {
        CDecl(declspecs, declrs, ni)
    }

    fn exportTypeDef(TypeDef(ident, ty, attrs, node_info): TypeDef) -> CDecl {
        CDecl((:(CStorageSpec((CTypedef(ni))), declspecs)), vec![declr], node_info)
    }

    fn exportTypeQuals(quals: TypeQuals) -> Vec<CTypeQual> {
        mapMaybe(select, vec![(constant, CConstQual(ni)), (volatile, CVolatQual(ni)), (restrict, CRestrQual(ni))])
    }

    fn exportTypeQualsAttrs(tyqs: TypeQuals, attrs: Attributes) -> Vec<CTypeQual> {
        (++(exportTypeQuals(tyqs), map(CAttrQual, (exportAttrs(attrs)))))
    }

    fn exportTypeSpec(tyname: TypeName) -> Vec<CTypeSpec> {
        match tyname {
                TyVoid => { vec![CVoidType(ni)] },
                TyIntegral, ity => { exportIntType(ity) },
                TyFloating, fty => { exportFloatType(fty) },
                TyComplex, fty => { exportComplexType(fty) },
                TyComp, comp => { exportCompTypeDecl(comp) },
                TyEnum, enum => { exportEnumTypeDecl(enum) },
                TyBuiltin, TyVaList => { vec![CTypeDef((internalIdent("va_list".to_string())), ni)] },
                TyBuiltin, TyAny => { vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)] },
            }
    }

    fn exportVarDecl(VarDecl(name, attrs, ty): VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    }

    fn fromDirectType(__0: Type) -> TypeName {
        match (__0) {
            DirectType(ty, _, _) => { ty },
            TypeDefType(TypeDefRef(_, ref, _), _, _) => { maybe((error("undefined typeDef".to_string())), fromDirectType, ref) },
            _ => { error("fromDirectType".to_string()) },
        }
    }

    fn ni() -> NodeInfo {
        undefNode
    }

    fn threadLocal(__0: Bool) -> Vec<CStorageSpec> {
        match (__0) {
            False => { id },
            True => { ((CThread(ni))(Operator(":"))) },
        }
    }

}

mod Language_C_Analysis_NameSpaceMap {
    struct NameSpaceMap(NsMap, Map(k, v), Vec<Vec<(k, v)>>);

    fn defGlobal(NsMap(gs, lss): NameSpaceMap(k, a), ident: k, def: a) -> (NameSpaceMap(k, a), Maybe(a)) {
        (NsMap((Map.insert(ident, def, gs)), lss), Map.lookup(ident, gs))
    }

    fn defLocal(__0: NameSpaceMap(k, a), __1: k, __2: a, __3: (NameSpaceMap(k, a), Maybe(a))) -> (NameSpaceMap(k, a), Maybe(a)) {
        match (__0, __1, __2, __3, __4) {
            ns, <todo>, NsMap(_, []), ident, def => { defGlobal(ns, ident, def) },
            NsMap(gs, ls:lss), ident, def => { (NsMap(gs, (:((:((ident, def), ls)), lss))), Prelude.lookup(ident, ls)) },
        }
    }

    fn enterNewScope(NsMap(gs, lss): NameSpaceMap(k, a)) -> NameSpaceMap(k, a) {
        NsMap(gs, (:(vec![], lss)))
    }

    fn globalNames(NsMap(g, _): NameSpaceMap(k, v)) -> Map(k, v) {
        g
    }

    fn hasLocalNames(NsMap(_, l): NameSpaceMap(k, v)) -> Bool {
        not((null(l)))
    }

    fn leaveScope(__0: NameSpaceMap(k, a)) -> (NameSpaceMap(k, a), Vec<(k, a)>) {
        match (__0) {
            NsMap(_, []) => { error("NsMaps.leaveScope: No local scope!".to_string()) },
            NsMap(gs, ls:lss) => { (NsMap(gs, lss), ls) },
        }
    }

    fn localNames(NsMap(_, l): NameSpaceMap(k, v)) -> Vec<Vec<(k, v)>> {
        l
    }

    fn lookupGlobal(NsMap(gs, _): NameSpaceMap(k, a), ident: k) -> Maybe(a) {
        Map.lookup(ident, gs)
    }

    fn lookupInnermostScope(nsm: NameSpaceMap(k, a), <todo>: k, NsMap(_gs, localDefs): Maybe(a)) -> Maybe(a) {
        match localDefs {
                ls(:, _lss) => { Prelude.lookup(ident, ls) },
                [] => { lookupGlobal(nsm, ident) },
            }
    }

    fn lookupName(ns: NameSpaceMap(k, a), <todo>: k, NsMap(_, localDefs): Maybe(a)) -> Maybe(a) {
        match (lookupLocal(localDefs)) {
                Nothing => { lookupGlobal(ns, ident) },
                Just, def => { Just(def) },
            }
    }

    fn mergeNameSpace(NsMap(global1, local1): NameSpaceMap(k, a), NsMap(global2, local2): NameSpaceMap(k, a)) -> NameSpaceMap(k, a) {
        NsMap((Map.union(global1, global2)), (localUnion(local1, local2)))
    }

    fn nameSpaceMap() -> NameSpaceMap(k, v) {
        NsMap(Map.empty, vec![])
    }

    fn nsMapToList(NsMap(gs, lss): NameSpaceMap(k, a)) -> Vec<(k, a)> {
        ++(concat(lss), Map.toList(gs))
    }

}

mod Language_C_Analysis_SemError {
    #[derive(Debug)]
    struct RedefError(RedefError, ErrorLevel, RedefInfo);

    struct RedefInfo(RedefInfo, String, RedefKind, NodeInfo, NodeInfo);

    struct RedefKind(DuplicateDef, DiffKindRedecl, ShadowedDef, DisagreeLinkage, NoLinkageOld);

    #[derive(Debug)]
    struct TypeMismatch(TypeMismatch, String, (NodeInfo, Type), (NodeInfo, Type));

    fn badSpecifierError(node_info: NodeInfo, msg: String) -> BadSpecifierError {
        BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn invalidAST(node_info: NodeInfo, msg: String) -> InvalidASTError {
        InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn prevDeclMsg(old_node: NodeInfo) -> Vec<String> {
        vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
    }

    fn redefErrLabel(RedefInfo(ident, _, _, _): RedefInfo) -> String {
        ++(ident, " redefined".to_string())
    }

    fn redefErrReason(__0: RedefInfo) -> String {
        match (__0) {
            RedefInfo(ident, DuplicateDef, _, _) => { ++("duplicate definition of ".to_string(), ident) },
            RedefInfo(ident, ShadowedDef, _, _) => { ++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string())) },
            RedefInfo(ident, DiffKindRedecl, _, _) => { ++(ident, " previously declared as a different kind of symbol".to_string()) },
            RedefInfo(ident, DisagreeLinkage, _, _) => { ++(ident, " previously declared with different linkage".to_string()) },
            RedefInfo(ident, NoLinkageOld, _, _) => { ++(ident, " previously declared without linkage".to_string()) },
        }
    }

    fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, <todo>: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (++(vec![redefErrReason(info)], prevDeclMsg(old_node))))
    }

    fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
        RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
    }

    fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    fn typeMismatchInfo(TypeMismatch(reason, (node1, _ty2), _t2): TypeMismatch) -> ErrorInfo {
        ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
    }

}

mod Language_C_Analysis_SemRep {
    #[derive(Clone, Debug)]
    struct TagDef(CompDef, CompType, EnumDef, EnumType);

    #[derive(Clone, Debug)]
    struct IdentDecl(Declaration, Decl, ObjectDef, ObjDef, FunctionDef, FunDef, EnumeratorDef, Enumerator);

    struct GlobalDecls(GlobalDecls, { /* struct def */ });

    struct DeclEvent(TagEvent, TagDef, DeclEvent, IdentDecl, ParamEvent, ParamDecl, LocalEvent, IdentDecl, TypeDefEvent, TypeDef, AsmEvent, AsmBlock);

    #[derive(Clone, Debug)]
    struct Decl(Decl, VarDecl, NodeInfo);

    #[derive(Clone, Debug)]
    struct ObjDef(ObjDef, VarDecl, Maybe(Initializer), NodeInfo);

    #[derive(Clone, Debug)]
    struct FunDef(FunDef, VarDecl, Stmt, NodeInfo);

    #[derive(Clone, Debug)]
    struct ParamDecl(ParamDecl, VarDecl, NodeInfo, AbstractParamDecl, VarDecl, NodeInfo);

    #[derive(Clone, Debug)]
    struct MemberDecl(MemberDecl, VarDecl, Maybe(Expr), NodeInfo, AnonBitField, Type, Expr, NodeInfo);

    #[derive(Clone, Debug)]
    struct TypeDef(TypeDef, Ident, Type, Attributes, NodeInfo);

    #[derive(Clone, Debug)]
    struct VarDecl(VarDecl, VarName, DeclAttrs, Type);

    #[derive(Clone, Debug)]
    struct DeclAttrs(DeclAttrs, Bool, Storage, Attributes);

    #[derive(Clone, Debug, Eq, Ord)]
    struct Storage(NoStorage, Auto, Register, Static, Linkage, ThreadLocal, FunLinkage, Linkage);

    #[derive(Clone, Debug, Eq, Ord)]
    struct Linkage(NoLinkage, InternalLinkage, ExternalLinkage);

    #[derive(Clone, Debug)]
    struct Type(DirectType, TypeName, TypeQuals, Attributes, PtrType, Type, TypeQuals, Attributes, ArrayType, Type, ArraySize, TypeQuals, Attributes, FunctionType, FunType, Attributes, TypeDefType, TypeDefRef, TypeQuals, Attributes);

    #[derive(Clone, Debug)]
    struct FunType(FunType, Type, Vec<ParamDecl>, Bool, FunTypeIncomplete, Type);

    #[derive(Clone, Debug)]
    struct ArraySize(UnknownArraySize, Bool, ArraySize, Bool, Expr);

    #[derive(Clone, Debug)]
    struct TypeName(TyVoid, TyIntegral, IntType, TyFloating, FloatType, TyComplex, FloatType, TyComp, CompTypeRef, TyEnum, EnumTypeRef, TyBuiltin, BuiltinType);

    #[derive(Clone, Debug)]
    struct BuiltinType(TyVaList, TyAny);

    #[derive(Clone, Debug)]
    struct TypeDefRef(TypeDefRef, Ident, Maybe(Type), NodeInfo);

    #[derive(Clone, Debug, Eq, Ord)]
    struct IntType(TyBool, TyChar, TySChar, TyUChar, TyShort, TyUShort, TyInt, TyUInt, TyLong, TyULong, TyLLong, TyULLong);

    #[derive(Clone, Debug, Eq, Ord)]
    struct FloatType(TyFloat, TyDouble, TyLDouble);

    #[derive(Clone, Debug)]
    struct CompTypeRef(CompTypeRef, SUERef, CompTyKind, NodeInfo);

    #[derive(Clone, Debug)]
    struct EnumTypeRef(EnumTypeRef, SUERef, NodeInfo);

    #[derive(Clone, Debug)]
    struct CompType(CompType, SUERef, CompTyKind, Vec<MemberDecl>, Attributes, NodeInfo);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CompTyKind(StructTag, UnionTag);

    #[derive(Clone, Debug)]
    struct EnumType(EnumType, SUERef, Vec<Enumerator>, Attributes, NodeInfo);

    #[derive(Clone, Debug)]
    struct Enumerator(Enumerator, Ident, Expr, EnumType, NodeInfo);

    #[derive(Clone, Debug)]
    struct TypeQuals(TypeQuals, { /* struct def */ });

    #[derive(Clone, Debug)]
    struct VarName(VarName, Ident, Maybe(AsmName), NoName);

    #[derive(Clone, Debug)]
    struct Attr(Attr, Ident, Vec<Expr>, NodeInfo);

    fn declAttrs() -> DeclAttrs {
        ((Lambda) . getVarDecl)
    }

    fn declIdent() -> Ident {
        (identOfVarName . declName)
    }

    fn declLinkage(decl: d) -> Linkage {
        match declStorage(decl) {
                NoStorage => { undefined },
                Auto, _ => { NoLinkage },
                Static, linkage, _ => { linkage },
                FunLinkage, linkage => { linkage },
            }
    }

    fn declName() -> VarName {
        ((Lambda) . getVarDecl)
    }

    fn declOfDef(def: n) -> Decl {
        Let
    }

    fn declStorage(d: d) -> Storage {
        match declAttrs(d) {
                DeclAttrs(_, st, _) => { st },
            }
    }

    fn declType() -> Type {
        ((Lambda) . getVarDecl)
    }

    fn emptyGlobalDecls() -> GlobalDecls {
        GlobalDecls(Map.empty, Map.empty, Map.empty)
    }

    fn filterGlobalDecls(decl_filter: fn(DeclEvent) -> Bool, gmap: GlobalDecls) -> GlobalDecls {
        GlobalDecls {
            gObjs: Map.filter(((decl_filter . DeclEvent)), (gObjs(gmap))),
            gTags: Map.filter(((decl_filter . TagEvent)), (gTags(gmap))),
            gTypeDefs: Map.filter(((decl_filter . TypeDefEvent)), (gTypeDefs(gmap)))
            }
    }

    fn hasLinkage(__0: Storage) -> Bool {
        match (__0) {
            Auto(_) => { False },
            Static(NoLinkage, _) => { False },
            _ => { True },
        }
    }

    fn identOfTypeDef(TypeDef(ide, _, _, _): TypeDef) -> Ident {
        ide
    }

    fn identOfVarName(__0: VarName) -> Ident {
        match (__0) {
            NoName => { error("identOfVarName: NoName".to_string()) },
            VarName(ident, _) => { ident },
        }
    }

    fn isExtDecl() -> Bool {
        (hasLinkage . declStorage)
    }

    fn isNoName(__0: VarName) -> Bool {
        match (__0) {
            NoName => { True },
            _ => { False },
        }
    }

    fn mergeAttributes() -> Attributes {
        (Operator("++"))
    }

    fn mergeGlobalDecls(gmap1: GlobalDecls, gmap2: GlobalDecls) -> GlobalDecls {
        GlobalDecls {
            gObjs: Map.union((gObjs(gmap1)), (gObjs(gmap2))),
            gTags: Map.union((gTags(gmap1)), (gTags(gmap2))),
            gTypeDefs: Map.union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
            }
    }

    fn mergeTypeQuals(TypeQuals(c1, v1, r1): TypeQuals, TypeQuals(c2, v2, r2): TypeQuals) -> TypeQuals {
        TypeQuals((&&(c1, c2)), (&&(v1, v2)), (&&(r1, r2)))
    }

    fn noAttributes() -> Attributes {
        vec![]
    }

    fn noTypeQuals() -> TypeQuals {
        TypeQuals(False, False, False)
    }

    fn objKindDescr(__0: IdentDecl) -> String {
        match (__0) {
            Declaration(_) => { "declaration".to_string() },
            ObjectDef(_) => { "object definition".to_string() },
            FunctionDef(_) => { "function definition".to_string() },
            EnumeratorDef(_) => { "enumerator definition".to_string() },
        }
    }

    fn splitIdentDecls(include_all: Bool) -> (Map(Ident, Decl), (Map(Ident, Enumerator), Map(Ident, ObjDef), Map(Ident, FunDef))) {
        Map.foldWithKey((if(include_all, then, deal, else, deal')), (Map.empty, (Map.empty, Map.empty, Map.empty)))
    }

    fn typeOfCompDef(CompType(ref, tag, _, _, _): CompType) -> TypeName {
        TyComp((CompTypeRef(ref, tag, undefNode)))
    }

    fn typeOfEnumDef(EnumType(ref, _, _, _): EnumType) -> TypeName {
        TyEnum((EnumTypeRef(ref, undefNode)))
    }

    fn typeOfTagDef(__0: TagDef) -> TypeName {
        match (__0) {
            CompDef(comptype) => { typeOfCompDef(comptype) },
            EnumDef(enumtype) => { typeOfEnumDef(enumtype) },
        }
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TravMonad.hs"
Error: Unrecognized token `->`:
372 |
373 |
374 | ;newtype Trav s a = Trav { unTrav :: TravState s -> Either CError (a,
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TypeCheck.hs"
Error: Unrecognized token `|`:
 95 |   do{ n <- genName
 96 |       ;let{ charType | wide      = TyInt
 97 |                     | otherwise = TyChar
~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Analysis_TypeConversions {
    fn arithmeticConversion(__0: TypeName, __1: TypeName) -> Maybe(TypeName) {
        match (__0, __1) {
            TyComplex(t1), TyComplex(t2) => { Just(TyComplex(floatConversion(t1, t2))) },
            TyComplex(t1), TyFloating(t2) => { Just(TyComplex(floatConversion(t1, t2))) },
            TyFloating(t1), TyComplex(t2) => { Just(TyComplex(floatConversion(t1, t2))) },
            t1, <todo>, TyComplex(_), TyIntegral(_) => { Just(t1) },
            TyIntegral(_), t2, <todo>, TyComplex(_) => { Just(t2) },
            TyFloating(t1), TyFloating(t2) => { Just(TyFloating(floatConversion(t1, t2))) },
            t1, <todo>, TyFloating(_), TyIntegral(_) => { Just(t1) },
            TyIntegral(_), t2, <todo>, TyFloating(_) => { Just(t2) },
            TyIntegral(t1), TyIntegral(t2) => { Just(TyIntegral(intConversion(t1, t2))) },
            TyEnum(_), TyEnum(_) => { Just(TyIntegral(TyInt)) },
            TyEnum(_), t2 => { Just(t2) },
            t1, TyEnum(_) => { Just(t1) },
            _, _ => { Nothing },
        }
    }

    fn floatConversion() -> FloatType {
        max
    }

    fn intConversion(t1: IntType, t2: IntType) -> IntType {
        max(TyInt, (max(t1, t2)))
    }

}

mod Language_C_Analysis_TypeUtils {
    fn baseType(__0: Type) -> Type {
        match (__0) {
            PtrType(t, _, _) => { t },
            ArrayType(t, _, _, _) => { t },
            _ => { error("base of non-pointer type".to_string()) },
        }
    }

    fn boolType() -> Type {
        integral(TyInt)
    }

    fn canonicalType(t: Type) -> Type {
        match deepDerefTypeDef(t) {
                FunctionType, ft, attrs => { simplePtr((FunctionType(ft, attrs))) },
                t' => { t' },
            }
    }

    fn charPtr() -> Type {
        simplePtr((integral(TyChar)))
    }

    fn constCharPtr() -> Type {
        constPtr((integral(TyChar)))
    }

    fn constPtr(t: Type) -> Type {
        PtrType(t, (TypeQuals(True, False, False)), vec![])
    }

    fn constVoidPtr() -> Type {
        constPtr(voidType)
    }

    fn deepDerefTypeDef(__0: Type) -> Type {
        match (__0) {
            PtrType(t, quals, attrs) => { PtrType((deepDerefTypeDef(t)), quals, attrs) },
            ArrayType(t, size, quals, attrs) => { ArrayType((deepDerefTypeDef(t)), size, quals, attrs) },
            FunctionType(FunType(rt, params, varargs), attrs) => { FunctionType((FunType((deepDerefTypeDef(rt)), params, varargs)), attrs) },
            FunctionType(FunTypeIncomplete(rt), attrs) => { FunctionType((FunTypeIncomplete((deepDerefTypeDef(rt)))), attrs) },
            TypeDefType(TypeDefRef(_, Just(t), _), q, a) => { ((typeAttrsUpd((mergeAttributes(a))) . typeQualsUpd((mergeTypeQuals(q)))))((deepDerefTypeDef(t))) },
            t => { t },
        }
    }

    fn derefTypeDef(__0: Type) -> Type {
        match (__0) {
            TypeDefType(TypeDefRef(_, Just(t), _), q, a) => { ((typeAttrsUpd((mergeAttributes(a))) . typeQualsUpd((mergeTypeQuals(q)))))((derefTypeDef(t))) },
            ty => { ty },
        }
    }

    fn floating(ty: FloatType) -> Type {
        DirectType((TyFloating(ty)), noTypeQuals, noAttributes)
    }

    fn integral(ty: IntType) -> Type {
        DirectType((TyIntegral(ty)), noTypeQuals, noAttributes)
    }

    fn isFloatingType(__0: Type) -> Bool {
        match (__0) {
            DirectType(TyFloating(_), _, _) => { True },
            _ => { False },
        }
    }

    fn isFunctionType(ty: Type) -> Bool {
        match ty {
                TypeDefType, TypeDefRef(_, Just(actual_ty), _), _, _ => { isFunctionType(actual_ty) },
                TypeDefType, _, _, _ => { error("isFunctionType: unresolved typeDef".to_string()) },
                FunctionType, _, _ => { True },
                _ => { False },
            }
    }

    fn isIntegralType(__0: Type) -> Bool {
        match (__0) {
            DirectType(TyIntegral(_), _, _) => { True },
            DirectType(TyEnum(_), _, _) => { True },
            _ => { False },
        }
    }

    fn isPointerType(__0: Type) -> Bool {
        match (__0) {
            PtrType(_, _, _) => { True },
            ArrayType(_, _, _, _) => { True },
            _ => { False },
        }
    }

    fn isScalarType(t: Type) -> Bool {
        ||(isIntegralType(t), ||(isPointerType(t), isFloatingType(t)))
    }

    fn ptrDiffType() -> Type {
        integral(TyInt)
    }

    fn simplePtr(t: Type) -> Type {
        PtrType(t, noTypeQuals, vec![])
    }

    fn size_tType() -> Type {
        integral(TyInt)
    }

    fn stringType() -> Type {
        ArrayType((DirectType((TyIntegral(TyChar)), (TypeQuals(True, False, False)), noAttributes)), (UnknownArraySize(False)), noTypeQuals, vec![])
    }

    fn testFlags(flags: Vec<f>, fi: Flags(f)) -> Bool {
        and(map(((flip(testFlag))(fi)), flags))
    }

    fn typeAttrs(__0: Type) -> Attributes {
        match (__0) {
            DirectType(_, _, a) => { a },
            PtrType(_, _, a) => { a },
            ArrayType(_, _, _, a) => { a },
            FunctionType(_, a) => { a },
            TypeDefType(TypeDefRef(_, Nothing, _), _, a) => { a },
            TypeDefType(TypeDefRef(_, Just(t), _), _, a) => { mergeAttributes(a, (typeAttrs(t))) },
        }
    }

    fn typeAttrsUpd(f: fn(Attributes) -> Attributes, ty: Type) -> Type {
        match ty {
                DirectType, ty_name, ty_quals, ty_attrs => { DirectType(ty_name, ty_quals, (f(ty_attrs))) },
                PtrType, ty_inner, ty_quals, ty_attrs => { PtrType(ty_inner, ty_quals, (f(ty_attrs))) },
                ArrayType, ty_inner, sz, ty_quals, ty_attrs => { ArrayType(ty_inner, sz, ty_quals, (f(ty_attrs))) },
                FunctionType, ty_inner, ty_attrs => { FunctionType(ty_inner, (f(ty_attrs))) },
                TypeDefType, ty_ref, ty_quals, ty_attrs => { TypeDefType(ty_ref, ty_quals, (f(ty_attrs))) },
            }
    }

    fn typeQuals(__0: Type) -> TypeQuals {
        match (__0) {
            DirectType(_, q, _) => { q },
            PtrType(_, q, _) => { q },
            ArrayType(_, _, q, _) => { q },
            FunctionType(_, _) => { noTypeQuals },
            TypeDefType(TypeDefRef(_, Nothing, _), q, _) => { q },
            TypeDefType(TypeDefRef(_, Just(t), _), q, _) => { mergeTypeQuals(q, (typeQuals(t))) },
        }
    }

    fn typeQualsUpd(f: fn(TypeQuals) -> TypeQuals, ty: Type) -> Type {
        match ty {
                DirectType, ty_name, ty_quals, ty_attrs => { DirectType(ty_name, (f(ty_quals)), ty_attrs) },
                PtrType, ty_inner, ty_quals, ty_attrs => { PtrType(ty_inner, (f(ty_quals)), ty_attrs) },
                ArrayType, ty_inner, sz, ty_quals, ty_attrs => { ArrayType(ty_inner, sz, (f(ty_quals)), ty_attrs) },
                FunctionType, ty_inner, ty_attrs => { FunctionType(ty_inner, ty_attrs) },
                TypeDefType, ty_ref, ty_quals, ty_attrs => { TypeDefType(ty_ref, (f(ty_quals)), ty_attrs) },
            }
    }

    fn valistType() -> Type {
        DirectType((TyBuiltin(TyVaList)), noTypeQuals, noAttributes)
    }

    fn voidPtr() -> Type {
        simplePtr(voidType)
    }

    fn voidType() -> Type {
        DirectType(TyVoid, noTypeQuals, noAttributes)
    }

}

mod Language_C_Analysis {

}

mod Language_C_Data_Error {
    #[derive(Eq, Ord)]
    struct ErrorLevel(LevelWarn, LevelError, LevelFatal);

    #[derive(Debug)]
    struct ErrorInfo(ErrorInfo, ErrorLevel, Position, Vec<String>);

    #[derive(Debug)]
    struct CError(forall, err., CError, err);

    #[derive(Debug)]
    struct UnsupportedFeature(UnsupportedFeature, String, Position);

    fn errorLevel() -> ErrorLevel {
        ((Lambda) . errorInfo)
    }

    fn errorMsgs() -> Vec<String> {
        ((Lambda) . errorInfo)
    }

    fn errorPos() -> Position {
        ((Lambda) . errorInfo)
    }

    fn indent() -> String {
        "  ".to_string()
    }

    fn indentLines() -> String {
        (unlines . (map((indent(Operator("++")))) . lines))
    }

    fn internalErr(msg: String) -> a {
        error((++(internalErrPrefix, ++("\n".to_string(), ++(indentLines(msg), "\n".to_string())))))
    }

    fn internalErrPrefix() -> String {
        unlines(vec!["Language.C : Internal Error".to_string(), ++("This is propably a bug, and should be reported at ".to_string(), "http://www.sivity.net/projects/language.c/newticket".to_string())])
    }

    fn isHardError() -> Bool {
        ((Operator(">")(LevelWarn)) . errorLevel)
    }

    fn mkErrorInfo(lvl: ErrorLevel, msg: String, node: NodeInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (lines(msg)))
    }

    fn showError(short_msg: String) -> String {
        (showErrorInfo(short_msg) . errorInfo)
    }

    fn showErrorInfo(short_msg: String, ErrorInfo(level, pos, msgs): ErrorInfo) -> String {
        ++(header, showMsgLines((if(null, short_msg, then, msgs, else, short_msg:msgs))))
    }

    fn unsupportedFeature(msg: String, a: a) -> UnsupportedFeature {
        UnsupportedFeature(msg, (posOf(a)))
    }

    fn unsupportedFeature_(msg: String) -> UnsupportedFeature {
        UnsupportedFeature(msg, internalPos)
    }

    fn userErr(msg: String) -> UserError {
        UserError((ErrorInfo(LevelError, internalPos, (lines(msg)))))
    }

}

mod Language_C_Data_Ident {
    #[derive(Clone, Debug, Eq, Ord)]
    struct SUERef(AnonymousRef, Name, NamedRef, Ident);

    #[derive(Clone, Debug)]
    struct Ident(Ident, String, isize, NodeInfo);

    fn bits14() -> isize {
        ^(2, (::(14, Int)))
    }

    fn bits21() -> isize {
        ^(2, (::(21, Int)))
    }

    fn bits28() -> isize {
        ^(2, (::(28, Int)))
    }

    fn bits7() -> isize {
        ^(2, (::(7, Int)))
    }

    fn builtinIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
    }

    fn dumpIdent(ide: Ident) -> String {
        ++(identToString(ide), ++(" at ".to_string(), show((nodeInfo(ide)))))
    }

    fn identToString(Ident(s, _, _): Ident) -> String {
        s
    }

    fn internalIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
    }

    fn internalIdentAt(pos: Position, s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
    }

    fn isAnonymousRef(__0: SUERef) -> Bool {
        match (__0) {
            AnonymousRef(_) => { True },
            _ => { False },
        }
    }

    fn isInternalIdent(Ident(_, _, nodeinfo): Ident) -> Bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo'(pos, (pos, length(s)), name)))
    }

    fn quad(__0: String) -> isize {
        match (__0) {
            c1:c2:c3:c4:s => { +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28))) },
            c1:c2:c3:([]) => { *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1))))) },
            c1:c2:([]) => { *(ord(c2), +(bits7, ord(c1))) },
            c1:([]) => { ord(c1) },
            [] => { 0 },
        }
    }

}

mod Language_C_Data_InputStream {
    fn countLines() -> isize {
        match () {
             => { (length . BSC.lines) },
             => { (length . lines) },
        }
    }

    fn inputStreamEmpty() -> Bool {
        match () {
             => { BSW.null },
             => { null },
        }
    }

    fn inputStreamFromString() -> InputStream {
        match () {
             => { BSC.pack },
             => { id },
        }
    }

    fn inputStreamToString() -> String {
        match () {
             => { BSC.unpack },
             => { id },
        }
    }

    fn readInputStream() -> IO(InputStream) {
        match () {
             => { BSW.readFile },
             => { readFile },
        }
    }

    fn takeByte(bs: InputStream) -> (Word8, InputStream) {
        seq(BSW.head(bs), (BSW.head(bs), BSW.tail(bs)))
    }

    fn takeChar(__0: InputStream) -> (Char, InputStream) {
        match (__0) {
            bs => { seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs))) },
            bs => { (head(bs), tail(bs)) },
        }
    }

    fn takeChars(__0: isize, __1: InputStream) -> Vec<Char> {
        match (__0, __1) {
            n, bstr => { BSC.unpack(BSC.take(n, bstr)) },
            n, str => { take(n, str) },
        }
    }

}

mod Language_C_Data_Name {
    fn namesStartingFrom(k: isize) -> Vec<Name> {
        vec![Name(k..)]
    }

    fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    }

}

mod Language_C_Data_Node {
    #[derive(Clone, Debug)]
    struct NodeInfo(OnlyPos, Position, PosLength, NodeInfo, Position, PosLength, Name);

    fn eqByName(obj1: a, obj2: a) -> Bool {
        ==((nodeInfo(obj1)), (nodeInfo(obj2)))
    }

    fn fileOfNode() -> Maybe(FilePath) {
        (fmap(posFile) . (justIf(isSourcePos) . (posOfNode . nodeInfo)))
    }

    fn getLastTokenPos(__0: NodeInfo) -> PosLength {
        match (__0) {
            NodeInfo(_, lastTok, _) => { lastTok },
            OnlyPos(_, lastTok) => { lastTok },
        }
    }

    fn internalNode() -> NodeInfo {
        undefNode
    }

    fn isUndefNode(_: NodeInfo) -> Bool {
        False
    }

    fn lengthOfNode(ni: NodeInfo) -> Maybe(isize) {
        len
    }

    fn mkNodeInfo(pos: Position, name: Name) -> NodeInfo {
        NodeInfo(pos, (nopos, Operator("-")(1)), name)
    }

    fn mkNodeInfo'(pos: Position, lasttok: PosLength, name: Name) -> NodeInfo {
        NodeInfo(pos, lasttok, name)
    }

    fn mkNodeInfoOnlyPos(pos: Position) -> NodeInfo {
        OnlyPos(pos, (nopos, Operator("-")(1)))
    }

    fn mkNodeInfoPosLen() -> NodeInfo {
        OnlyPos
    }

    fn nameOfNode(__0: NodeInfo) -> Maybe(Name) {
        match (__0) {
            OnlyPos(_, _) => { Nothing },
            NodeInfo(_, _, name) => { Just(name) },
        }
    }

    fn posOfNode(ni: NodeInfo) -> Position {
        match ni {
                OnlyPos(pos, _) => { pos },
                NodeInfo(pos, _, _) => { pos },
            }
    }

    fn undefNode() -> NodeInfo {
        OnlyPos(nopos, (nopos, Operator("-")(1)))
    }

}

mod Language_C_Data_Position {
    #[derive(Clone, Debug, Eq, Ord)]
    struct Position(Position, { /* struct def */ }, NoPosition, BuiltinPosition, InternalPosition);

    fn adjustPos(__0: FilePath, __1: isize, __2: Position) -> Position {
        match (__0, __1, __2) {
            fname, row, Position(offs, _, _, _) => { Position(offs, fname, row, 1) },
            _, _, p => { p },
        }
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset(__0: Position, __1: isize) -> Position {
        match (__0, __1) {
            Position(o, f, r, c), n => { Position((+(o, n)), f, r, c) },
            p, n => { p },
        }
    }

    fn incPos(__0: Position, __1: isize) -> Position {
        match (__0, __1) {
            Position(offs, fname, row, col), n => { Position((+(offs, n)), fname, row, (+(col, n))) },
            p, _ => { p },
        }
    }

    fn initPos(file: FilePath) -> Position {
        Position(0, file, 1, 1)
    }

    fn internalPos() -> Position {
        InternalPosition
    }

    fn isBuiltinPos(__0: Position) -> Bool {
        match (__0) {
            BuiltinPosition => { True },
            _ => { False },
        }
    }

    fn isInternalPos(__0: Position) -> Bool {
        match (__0) {
            InternalPosition => { True },
            _ => { False },
        }
    }

    fn isNoPos(__0: Position) -> Bool {
        match (__0) {
            NoPosition => { True },
            _ => { False },
        }
    }

    fn isSourcePos(__0: Position) -> Bool {
        match (__0) {
            Position(_, _, _, _) => { True },
            _ => { False },
        }
    }

    fn nopos() -> Position {
        NoPosition
    }

    fn position() -> Position {
        Position
    }

    fn retPos(__0: Position) -> Position {
        match (__0) {
            Position(offs, fname, row, _) => { Position((+(offs, 1)), fname, (+(row, 1)), 1) },
            p => { p },
        }
    }

}

mod Language_C_Data_RList {
    fn appendr(xs: Vec<a>, Reversed(ys): Reversed(Vec<a>)) -> Reversed(Vec<a>) {
        Reversed((++(ys, List.reverse(xs))))
    }

    fn empty() -> Reversed(Vec<a>) {
        Reversed(vec![])
    }

    fn rappend(Reversed(xs): Reversed(Vec<a>), ys: Vec<a>) -> Reversed(Vec<a>) {
        Reversed((++(List.reverse(ys), xs)))
    }

    fn rappendr(Reversed(xs): Reversed(Vec<a>), Reversed(ys): Reversed(Vec<a>)) -> Reversed(Vec<a>) {
        Reversed((++(ys, xs)))
    }

    fn reverse(Reversed(xs): Reversed(Vec<a>)) -> Vec<a> {
        List.reverse(xs)
    }

    fn rmap(f: fn(a) -> b, Reversed(xs): Reversed(Vec<a>)) -> Reversed(Vec<b>) {
        Reversed((map(f, xs)))
    }

    fn singleton(x: a) -> Reversed(Vec<a>) {
        Reversed(vec![x])
    }

    fn snoc(Reversed(xs): Reversed(Vec<a>), x: a) -> Reversed(Vec<a>) {
        Reversed((:(x, xs)))
    }

    fn viewr(__0: Reversed(Vec<a>)) -> (Reversed(Vec<a>), a) {
        match (__0) {
            Reversed([]) => { error("viewr: empty RList".to_string()) },
            Reversed(x:xs) => { (Reversed(xs), x) },
        }
    }

}

mod Language_C_Data {

}

mod Language_C_Parser_Builtin {
    fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Parser/ParserMonad.hs"
Error: Unrecognized token `CTokEof`:
 45 | ;import Language.C.Data.Name    (Name)
 46 | ;import Language.C.Data.Ident    (Ident)
 47 | ;import Language.C.Parser.Tokens (CToken(CTokEof))
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./language-c/src/Language/C/Parser/Tokens.hs"
Error: Unrecognized token `|`:
255 |    ;showsPrec _ (CTokTilde    _  ) = showString "fg=="
256 |    ;showsPrec _ (CTokInc      _  ) = showString "Kys="
257 | QcmVjIF8gKENUb2tCYXIgICAgICBfICApID0gc2hvd1N0cmluZyA="|"CiAgc2hvd3NQcm
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Parser {
    fn execParser_(parser: P(a), input: InputStream, pos: Position) -> Either(ParseError, a) {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Pretty.hs"
Error: Unrecognized token `->`:
384 |          parenPrec p 26 $ prettyPrec 26 expr <> text "Kys="
385 |      ;prettyPrec p (CUnary CPostDecOp expr _) =
386 | gICAgICAgICAgICAgICA8PiB0ZXh0IChpZiBkZXJlZiB0aGVuIA=="->"IGVsc2Ug"."KS
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Syntax_AST {
    #[derive(Clone, Debug)]
    struct CTranslationUnit(CTranslUnit, Vec<CExternalDeclaration(a)>, a);

    #[derive(Clone, Debug)]
    struct CExternalDeclaration(CDeclExt, CDeclaration(a), CFDefExt, CFunctionDef(a), CAsmExt, CStringLiteral(a), a);

    #[derive(Clone, Debug)]
    struct CFunctionDef(CFunDef, Vec<CDeclarationSpecifier(a)>, CDeclarator(a), Vec<CDeclaration(a)>, CStatement(a), a);

    #[derive(Clone, Debug)]
    struct CDeclaration(CDecl, Vec<CDeclarationSpecifier(a)>, Vec<(Maybe(CDeclarator(a)), Maybe(CInitializer(a)), Maybe(CExpression(a)))>, a);

    #[derive(Clone, Debug)]
    struct CDeclarator(CDeclr, Maybe(Ident), Vec<CDerivedDeclarator(a)>, Maybe(CStringLiteral(a)), Vec<CAttribute(a)>, a);

    #[derive(Clone, Debug)]
    struct CDerivedDeclarator(CPtrDeclr, Vec<CTypeQualifier(a)>, a, CArrDeclr, Vec<CTypeQualifier(a)>, CArraySize(a), a, CFunDeclr, Either(Vec<Ident>, (Vec<CDeclaration(a)>, Bool)), Vec<CAttribute(a)>, a);

    #[derive(Clone, Debug)]
    struct CArraySize(CNoArrSize, Bool, CArrSize, Bool, CExpression(a));

    #[derive(Clone, Debug)]
    struct CStatement(CLabel, Ident, CStatement(a), Vec<CAttribute(a)>, a, CCase, CExpression(a), CStatement(a), a, CCases, CExpression(a), CExpression(a), CStatement(a), a, CDefault, CStatement(a), a, CExpr, Maybe(CExpression(a)), a, CCompound, Vec<Ident>, Vec<CCompoundBlockItem(a)>, a, CIf, CExpression(a), CStatement(a), Maybe(CStatement(a)), a, CSwitch, CExpression(a), CStatement(a), a, CWhile, CExpression(a), CStatement(a), Bool, a, CFor, Either(Maybe(CExpression(a)), CDeclaration(a)), Maybe(CExpression(a)), Maybe(CExpression(a)), CStatement(a), a, CGoto, Ident, a, CGotoPtr, CExpression(a), a, CCont, a, CBreak, a, CReturn, Maybe(CExpression(a)), a, CAsm, CAssemblyStatement(a), a);

    #[derive(Clone, Debug)]
    struct CAssemblyStatement(CAsmStmt, Maybe(CTypeQualifier(a)), CStringLiteral(a), Vec<CAssemblyOperand(a)>, Vec<CAssemblyOperand(a)>, Vec<CStringLiteral(a)>, a);

    #[derive(Clone, Debug)]
    struct CAssemblyOperand(CAsmOperand, Maybe(Ident), CStringLiteral(a), CExpression(a), a);

    #[derive(Clone, Debug)]
    struct CCompoundBlockItem(CBlockStmt, CStatement(a), CBlockDecl, CDeclaration(a), CNestedFunDef, CFunctionDef(a));

    #[derive(Clone, Debug)]
    struct CDeclarationSpecifier(CStorageSpec, CStorageSpecifier(a), CTypeSpec, CTypeSpecifier(a), CTypeQual, CTypeQualifier(a));

    #[derive(Clone, Debug, Eq, Ord)]
    struct CStorageSpecifier(CAuto, a, CRegister, a, CStatic, a, CExtern, a, CTypedef, a, CThread, a);

    #[derive(Clone, Debug)]
    struct CTypeSpecifier(CVoidType, a, CCharType, a, CShortType, a, CIntType, a, CLongType, a, CFloatType, a, CDoubleType, a, CSignedType, a, CUnsigType, a, CBoolType, a, CComplexType, a, CSUType, CStructureUnion(a), a, CEnumType, CEnumeration(a), a, CTypeDef, Ident, a, CTypeOfExpr, CExpression(a), a, CTypeOfType, CDeclaration(a), a);

    #[derive(Clone, Debug)]
    struct CTypeQualifier(CConstQual, a, CVolatQual, a, CRestrQual, a, CInlineQual, a, CAttrQual, CAttribute(a));

    #[derive(Clone, Debug)]
    struct CStructureUnion(CStruct, CStructTag, Maybe(Ident), Maybe(Vec<CDeclaration(a)>), Vec<CAttribute(a)>, a);

    #[derive(Clone, Debug, Eq)]
    struct CStructTag(CStructTag, CUnionTag);

    #[derive(Clone, Debug)]
    struct CEnumeration(CEnum, Maybe(Ident), Maybe(Vec<(Ident, Maybe(CExpression(a)))>), Vec<CAttribute(a)>, a);

    #[derive(Clone, Debug)]
    struct CInitializer(CInitExpr, CExpression(a), a, CInitList, CInitializerList(a), a);

    #[derive(Clone, Debug)]
    struct CPartDesignator(CArrDesig, CExpression(a), a, CMemberDesig, Ident, a, CRangeDesig, CExpression(a), CExpression(a), a);

    #[derive(Clone, Debug)]
    struct CAttribute(CAttr, Ident, Vec<CExpression(a)>, a);

    #[derive(Clone, Debug)]
    struct CExpression(CComma, Vec<CExpression(a)>, a, CAssign, CAssignOp, CExpression(a), CExpression(a), a, CCond, CExpression(a), Maybe(CExpression(a)), CExpression(a), a, CBinary, CBinaryOp, CExpression(a), CExpression(a), a, CCast, CDeclaration(a), CExpression(a), a, CUnary, CUnaryOp, CExpression(a), a, CSizeofExpr, CExpression(a), a, CSizeofType, CDeclaration(a), a, CAlignofExpr, CExpression(a), a, CAlignofType, CDeclaration(a), a, CComplexReal, CExpression(a), a, CComplexImag, CExpression(a), a, CIndex, CExpression(a), CExpression(a), a, CCall, CExpression(a), Vec<CExpression(a)>, a, CMember, CExpression(a), Ident, Bool, a, CVar, Ident, a, CConst, CConstant(a), CCompoundLit, CDeclaration(a), CInitializerList(a), a, CStatExpr, CStatement(a), a, CLabAddrExpr, Ident, a, CBuiltinExpr, CBuiltinThing(a));

    #[derive(Clone, Debug)]
    struct CBuiltinThing(CBuiltinVaArg, CExpression(a), CDeclaration(a), a, CBuiltinOffsetOf, CDeclaration(a), Vec<CPartDesignator(a)>, a, CBuiltinTypesCompatible, CDeclaration(a), CDeclaration(a), a);

    #[derive(Clone, Debug)]
    struct CConstant(CIntConst, CInteger, a, CCharConst, CChar, a, CFloatConst, CFloat, a, CStrConst, CString, a);

    #[derive(Clone, Debug)]
    struct CStringLiteral(CStrLit, CString, a);

    fn cstringOfLit(CStrLit(cstr, _): CStringLiteral(a)) -> CString {
        cstr
    }

    fn fmapInitList(_f: fn(a) -> b) -> CInitializerList(b) {
        map((Lambda))
    }

    fn isSUEDef(__0: CTypeSpecifier(a)) -> Bool {
        match (__0) {
            CSUType(CStruct(_, _, Just(_), _, _), _) => { True },
            CEnumType(CEnum(_, Just(_), _, _), _) => { True },
            _ => { False },
        }
    }

    fn liftStrLit(CStrLit(str, at): CStringLiteral(a)) -> CConstant(a) {
        CStrConst(str, at)
    }

    fn partitionDeclSpecs() -> (Vec<CStorageSpecifier(a)>, Vec<CAttribute(a)>, Vec<CTypeQualifier(a)>, Vec<CTypeSpecifier(a)>, Bool) {
        foldr(deals, (vec![], vec![], vec![], vec![], False))
    }

}

mod Language_C_Syntax_Constants {
    #[derive(Clone, Debug, Eq, Ord)]
    struct CChar(CChar, Char, Bool, CChars, Vec<Char>, Bool);

    #[derive(Bounded, Clone, Debug, Enum, Eq, Ord)]
    struct CIntRepr(DecRepr, HexRepr, OctalRepr);

    #[derive(Bounded, Clone, Debug, Enum, Eq, Ord)]
    struct CIntFlag(FlagUnsigned, FlagLong, FlagLongLong, FlagImag);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CInteger(CInteger, Integer, CIntRepr, Flags(CIntFlag));

    #[derive(Clone, Debug, Eq, Ord)]
    struct CFloat(CFloat, String);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CString(CString, Vec<Char>, Bool);

    fn _showWideFlag(flag: Bool) -> ShowS {
        if(flag, then, showString, "L".to_string(), else, id)
    }

    fn cChar(c: Char) -> CChar {
        CChar(c, False)
    }

    fn cChar_w(c: Char) -> CChar {
        CChar(c, True)
    }

    fn cChars() -> CChar {
        CChars
    }

    fn cFloat() -> CFloat {
        (CFloat . show)
    }

    fn cInteger(i: Integer) -> CInteger {
        CInteger(i, DecRepr, noFlags)
    }

    fn cString(str: String) -> CString {
        CString(str, False)
    }

    fn cString_w(str: String) -> CString {
        CString(str, True)
    }

    fn clearFlag(flag: f, Flags(k): Flags(f)) -> Flags(f) {
        Flags(clearBit(k, fromEnum(flag)))
    }

    fn concatCStrings(cs: Vec<CString>) -> CString {
        CString((concatMap(getCString, cs)), (any(isWideString, cs)))
    }

    fn dQuote(s: String, t: ShowS) -> ShowS {
        ++((:('\"', s)), ++("\"".to_string(), t))
    }

    fn escapeCChar('\'': Char) -> String {
        "\\\'".to_string()
    }

    fn escapeChar(__0: Char) -> String {
        match (__0) {
            '\\' => { "\\\\".to_string() },
            '\u{7}' => { "\\a".to_string() },
            '\u{8}' => { "\\b".to_string() },
            '\u{1b}' => { "\\e".to_string() },
            '\u{c}' => { "\\f".to_string() },
            '\n' => { "\\n".to_string() },
            '\r' => { "\\r".to_string() },
            '\t' => { "\\t".to_string() },
            '\u{b}' => { "\\v".to_string() },
        }
    }

    fn getCChar(__0: CChar) -> Vec<Char> {
        match (__0) {
            CChar(c, _) => { vec![c] },
            CChars(cs, _) => { cs },
        }
    }

    fn getCCharAsInt(__0: CChar) -> Integer {
        match (__0) {
            CChar(c, _) => { fromIntegral((fromEnum(c))) },
            CChars(_cs, _) => { error("integer value of multi-character character constants is implementation defined".to_string()) },
        }
    }

    fn getCInteger(CInteger(i, _, _): CInteger) -> Integer {
        i
    }

    fn getCString(CString(str, _): CString) -> String {
        str
    }

    fn head'(__0: String, __1: Vec<a>) -> a {
        match (__0, __1) {
            err, [] => { error(err) },
            _, x:_ => { x },
        }
    }

    fn isAsciiSourceChar(c: Char) -> Bool {
        &&(isAscii(c), isPrint(c))
    }

    fn isCChar(__0: Char) -> Bool {
        match (__0) {
            '\\' => { False },
            '\'' => { False },
            '\n' => { False },
            c => { isAsciiSourceChar(c) },
        }
    }

    fn isSChar(__0: Char) -> Bool {
        match (__0) {
            '\\' => { False },
            '\"' => { False },
            '\n' => { False },
            c => { isAsciiSourceChar(c) },
        }
    }

    fn isWideChar(__0: CChar) -> Bool {
        match (__0) {
            CChar(_, wideFlag) => { wideFlag },
            CChars(_, wideFlag) => { wideFlag },
        }
    }

    fn isWideString(CString(_, wideflag): CString) -> Bool {
        wideflag
    }

    fn noFlags() -> Flags(f) {
        Flags(0)
    }

    fn readCFloat() -> CFloat {
        CFloat
    }

    fn readCInteger(repr: CIntRepr, str: String) -> Either(String, CInteger) {
        match readNum(str) {
                [(n, suffix)] => { mkCInt(n, suffix) },
                parseFailed => { Left(++("Bad Integer literal: ".to_string(), show(parseFailed))) },
            }
    }

    fn sQuote(s: String, t: ShowS) -> ShowS {
        ++("\'".to_string(), ++(s, ++("\'".to_string(), t)))
    }

    fn setFlag(flag: f, Flags(k): Flags(f)) -> Flags(f) {
        Flags(setBit(k, fromEnum(flag)))
    }

    fn showCharConst(c: Char) -> ShowS {
        sQuote(escapeCChar(c))
    }

    fn showStringLit() -> ShowS {
        (dQuote . concatMap(showStringChar))
    }

    fn testFlag(flag: f, Flags(k): Flags(f)) -> Bool {
        testBit(k, fromEnum(flag))
    }

    fn unescapeChar(__0: String) -> (Char, String) {
        match (__0) {
            '\\'(:, c:cs) => { match c {
                    'n' => { ('\n', cs) },
                    't' => { ('\t', cs) },
                    'v' => { ('\u{b}', cs) },
                    'b' => { ('\u{8}', cs) },
                    'r' => { ('\r', cs) },
                    'f' => { ('\u{c}', cs) },
                    'a' => { ('\u{7}', cs) },
                    'e' => { ('\u{1b}', cs) },
                    'E' => { ('\u{1b}', cs) },
                    '\\' => { ('\\', cs) },
                    '?' => { ('?', cs) },
                    '\'' => { ('\'', cs) },
                    '\"' => { ('\"', cs) },
                    'x' => { match head'("bad escape sequence".to_string(), (readHex(cs))) {
                            (i, cs') => { (toEnum(i), cs') },
                        } },
                    _ => { match head'("bad escape sequence".to_string(), (readOct((c:cs)))) {
                            (i, cs') => { (toEnum(i), cs') },
                        } },
                } },
            c(:, cs) => { (c, cs) },
            [] => { error("unescape char: empty string".to_string()) },
        }
    }

    fn unescapeString(__0: String) -> String {
        match (__0) {
            [] => { vec![] },
            cs => { match unescapeChar(cs) {
                    (c, cs') => { :(c, unescapeString(cs')) },
                } },
        }
    }

}

mod Language_C_Syntax_Ops {
    #[derive(Clone, Debug, Eq, Ord)]
    struct CAssignOp(CAssignOp, CMulAssOp, CDivAssOp, CRmdAssOp, CAddAssOp, CSubAssOp, CShlAssOp, CShrAssOp, CAndAssOp, CXorAssOp, COrAssOp);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CBinaryOp(CMulOp, CDivOp, CRmdOp, CAddOp, CSubOp, CShlOp, CShrOp, CLeOp, CGrOp, CLeqOp, CGeqOp, CEqOp, CNeqOp, CAndOp, CXorOp, COrOp, CLndOp, CLorOp);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CUnaryOp(CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp, CAdrOp, CIndOp, CPlusOp, CMinOp, CCompOp, CNegOp);

    fn assignBinop(__0: CAssignOp) -> CBinaryOp {
        match (__0) {
            CAssignOp => { error("direct assignment has no binary operator".to_string()) },
            CMulAssOp => { CMulOp },
            CDivAssOp => { CDivOp },
            CRmdAssOp => { CRmdOp },
            CAddAssOp => { CAddOp },
            CSubAssOp => { CSubOp },
            CShlAssOp => { CShlOp },
            CShrAssOp => { CShrOp },
            CAndAssOp => { CAndOp },
            CXorAssOp => { CXorOp },
            COrAssOp => { COrOp },
        }
    }

    fn isBitOp(op: CBinaryOp) -> Bool {
        elem(op, vec![CShlOp, CShrOp, CAndOp, COrOp, CXorOp])
    }

    fn isCmpOp(op: CBinaryOp) -> Bool {
        elem(op, vec![CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp])
    }

    fn isEffectfulOp(op: CUnaryOp) -> Bool {
        elem(op, vec![CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp])
    }

    fn isLogicOp(op: CBinaryOp) -> Bool {
        elem(op, vec![CLndOp, CLorOp])
    }

    fn isPtrOp(op: CBinaryOp) -> Bool {
        elem(op, vec![CAddOp, CSubOp])
    }

}

mod Language_C_Syntax_Utils {
    fn compoundSubStmts(__0: CBlockItem) -> Vec<CStat> {
        match (__0) {
            CBlockStmt(s) => { vec![s] },
            CBlockDecl(_) => { vec![] },
            CNestedFunDef(_) => { vec![] },
        }
    }

    fn getLabels(__0: CStat) -> Vec<Ident> {
        match (__0) {
            CLabel(l, s, _, _) => { :(l, getLabels(s)) },
            CCompound(ls, body, _) => { \\(concatMap(((concatMap(getLabels) . compoundSubStmts)), body), ls) },
            stmt => { concatMap(getLabels, (getSubStmts(stmt))) },
        }
    }

    fn getSubStmts(__0: CStat) -> Vec<CStat> {
        match (__0) {
            CLabel(_, s, _, _) => { vec![s] },
            CCase(_, s, _) => { vec![s] },
            CCases(_, _, s, _) => { vec![s] },
            CDefault(s, _) => { vec![s] },
            CExpr(_, _) => { vec![] },
            CCompound(_, body, _) => { concatMap(compoundSubStmts, body) },
            CIf(_, sthen, selse, _) => { maybe(vec![sthen], (Lambda), selse) },
            CSwitch(_, s, _) => { vec![s] },
            CWhile(_, s, _, _) => { vec![s] },
            CFor(_, _, _, s, _) => { vec![s] },
            CGoto(_, _) => { vec![] },
            CGotoPtr(_, _) => { vec![] },
            CCont(_) => { vec![] },
            CBreak(_) => { vec![] },
            CReturn(_, _) => { vec![] },
            CAsm(_, _) => { vec![] },
        }
    }

    fn mapBlockItemStmts(__0: fn(CStat) -> Bool, __1: fn(CStat) -> CStat, __2: CBlockItem) -> CBlockItem {
        match (__0, __1, __2) {
            stop, f, CBlockStmt(s) => { CBlockStmt((mapSubStmts(stop, f, s))) },
            _, _, bi => { bi },
        }
    }

    fn mapSubStmts(__0: fn(CStat) -> Bool, __1: fn(CStat) -> CStat, __2: CStat) -> CStat {
        match (__0, __1, __2) {
            stop, f, CLabel(i, s, attrs, ni) => { f((CLabel(i, (mapSubStmts(stop, f, s)), attrs, ni))) },
            stop, f, CCase(e, s, ni) => { f((CCase(e, (mapSubStmts(stop, f, s)), ni))) },
            stop, f, CCases(e1, e2, s, ni) => { f((CCases(e1, e2, (mapSubStmts(stop, f, s)), ni))) },
            stop, f, CDefault(s, ni) => { f((CDefault((mapSubStmts(stop, f, s)), ni))) },
            stop, f, CCompound(ls, body, ni) => { f((CCompound(ls, (map((mapBlockItemStmts(stop, f)), body)), ni))) },
            stop, f, CIf(e, sthen, selse, ni) => { f((CIf(e, (mapSubStmts(stop, f, sthen)), (maybe(Nothing, ((Just . mapSubStmts(stop, f))), selse)), ni))) },
            stop, f, CSwitch(e, s, ni) => { f((CSwitch(e, (mapSubStmts(stop, f, s)), ni))) },
            stop, f, CWhile(e, s, isdo, ni) => { f((CWhile(e, (mapSubStmts(stop, f, s)), isdo, ni))) },
            stop, f, CFor(i, t, a, s, ni) => { f((CFor(i, t, a, (mapSubStmts(stop, f, s)), ni))) },
            _, f, s => { f(s) },
        }
    }

}

mod Language_C_Syntax {

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/System/GCC.hs"
Error: Unrecognized token `}`:
 38 | ce target = do{ copyFile source target
 39 |                 ;p <- getPermissions target
 40 |                 ;setPermissions target p{writable=True}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_System_Preprocess {
    struct CppOption(IncludeDir, FilePath, Define, String, String, Undefine, String, IncludeFile, FilePath);

    struct CppArgs(CppArgs, { /* struct def */ });

    fn addCppOption(cpp_args: CppArgs, opt: CppOption) -> CppArgs {
        cpp_args {
            cppOptions: :(opt, (cppOptions(cpp_args)))
            }
    }

    fn addExtraOption(cpp_args: CppArgs, extra: String) -> CppArgs {
        cpp_args {
            extraOptions: :(extra, (extraOptions(cpp_args)))
            }
    }

    fn cppFile(input_file: FilePath) -> CppArgs {
        CppArgs {
            cppOptions: vec![],
            extraOptions: vec![],
            cppTmpDir: Nothing,
            inputFile: input_file,
            outputFile: Nothing
            }
    }

    fn isPreprocessed() -> Bool {
        (".i".to_string()(Operator("isSuffixOf")))
    }

    fn mkOutputFile(tmp_dir_opt: Maybe(FilePath), input_file: FilePath) -> IO(FilePath) {
        {

        }
    }

    fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> IO(FilePath) {
        {

        }
    }

    fn preprocessedExt() -> String {
        ".i".to_string()
    }

    fn rawCppArgs(opts: Vec<String>, input_file: FilePath) -> CppArgs {
        CppArgs {
            inputFile: input_file,
            cppOptions: vec![],
            extraOptions: opts,
            outputFile: Nothing,
            cppTmpDir: Nothing
            }
    }

    fn runPreprocessor(cpp: cpp, cpp_args: CppArgs) -> IO(Either(ExitCode, InputStream)) {
        {

        }
    }

}



fn main() { /* demo */ }
