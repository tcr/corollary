ERROR   - "./language-c/src/Language/C/Analysis/AstAnalysis.hs"



mod "./language-c/src/Language/C/Analysis/AstAnalysis.hs-boot" {

}



mod "./language-c/src/Language/C/Analysis/Builtins.hs" {
    fn builtins() -> DefTable {
        foldr(doIdent, foldr(doTypeDef, emptyDefTable, typedefs), idents)
    };

}



ERROR   - "./language-c/src/Language/C/Analysis/ConstEval.hs"



ERROR   - "./language-c/src/Language/C/Analysis/Debug.hs"



mod "./language-c/src/Language/C/Analysis/DeclAnalysis.hs" {
    fn analyseVarDecl(handle_sue_def: Bool, storage_specs: Vec<CStorageSpec>, decl_attrs: Vec<CAttr>, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, inline: Bool, $$$: CDeclr, oldstyle_params: Vec<CDecl>, init_opt: Maybe<CInit>) -> m<VarDeclInfo> {
        {
storage_spec(Operator("<-"), canonicalStorageSpec, storage_specs);
typ(Operator("<-"), tType, handle_sue_def, node, typequals, canonTySpecs, derived_declrs, oldstyle_params);
attrs'(Operator("<-"), mapM, tAttr, decl_attrs(Operator("++"), declr_attrs));
name(Operator("<-"), mkVarName, node, name_opt, asmname_opt);
return(Operator("$"), VarDeclInfo, name, inline, storage_spec, attrs', typ, node);
}
    };

    fn analyseVarDecl'(handle_sue_def: Bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, oldstyle: Vec<CDecl>, init_opt: Maybe<CInit>) -> m<VarDeclInfo> {
        {
Let;
canonTySpecs(Operator("<-"), canonicalTypeSpec, type_specs);
analyseVarDecl(handle_sue_def, storage_specs, attrs, type_quals, canonTySpecs, inline, declr, oldstyle, init_opt);
}
    };

    fn canonicalStorageSpec(storagespecs: Vec<CStorageSpec>) -> m<StorageSpec> {
        liftM(elideAuto, Operator("$"), foldrM, updStorage, NoStorageSpec, storagespecs)
    };

    fn canonicalTypeSpec() -> m<TypeSpecAnalysis> {
        foldrM(go, TSNone)
    };

    fn computeParamStorage(_: NodeInfo, NoStorageSpec: StorageSpec) -> Either<BadSpecifierError, Storage> {
        Right(Auto(False))
    };

    fn computeParamStorage(_: NodeInfo, RegSpec: StorageSpec) -> Either<BadSpecifierError, Storage> {
        Right(Auto(True))
    };

    fn computeParamStorage(node: NodeInfo, spec: StorageSpec) -> Either<BadSpecifierError, Storage> {
        Left(Operator("."), badSpecifierError, node, Operator("$"), "Bad storage specified for parameter: ".to_string(), Operator("++"), show, spec)
    };

    fn emptyDeclr(node: NodeInfo) -> CDeclr {
        CDeclr(Nothing, Vector([]), Nothing, Vector([]), node)
    };

    fn emptyNumTypeSpec() -> NumTypeSpec {
        NumTypeSpec(hashmap! {
"base" => NoBaseType, "signSpec" => NoSignSpec, "sizeMod" => NoSizeMod, "isComplex" => False
})
    };

    fn getOnlyDeclr($$$: CDecl) -> m<CDeclr> {
        return(declr)
    };

    fn getOnlyDeclr($$$: CDecl) -> m<CDeclr> {
        internalErr("getOnlyDeclr: declaration doesn\'t have a unique declarator".to_string())
    };

    fn hasThreadLocalSpec(ThreadSpec: StorageSpec) -> Bool {
        True
    };

    fn hasThreadLocalSpec($$$: StorageSpec) -> Bool {
        b
    };

    fn hasThreadLocalSpec($$$: StorageSpec) -> Bool {
        b
    };

    fn hasThreadLocalSpec(_: StorageSpec) -> Bool {
        False
    };

    fn isTypeDef(declspecs: Vec<CDeclSpec>) -> Bool {
        not(Operator("$"), null, Dummy)
    };

    fn mergeOldStyle(_node: NodeInfo, $$$: Vec<CDecl>, declrs: Vec<CDerivedDeclr>) -> m<Vec<CDerivedDeclr>> {
        return(declrs)
    };

    fn mergeOldStyle(node: NodeInfo, oldstyle_params: Vec<CDecl>, $$$: Vec<CDerivedDeclr>) -> m<Vec<CDerivedDeclr>> {
        Case
    };

    fn mergeOldStyle(node: NodeInfo, _: Vec<CDecl>, _: Vec<CDerivedDeclr>) -> m<Vec<CDerivedDeclr>> {
        astError(node, "oldstyle parameter list, but not function type".to_string())
    };

    fn mergeTypeAttributes(node_info: NodeInfo, quals: TypeQuals, attrs: Vec<Attr>, typ: Type) -> m<Type> {
        Case
    };

    fn mkVarName(node: NodeInfo, Nothing: Maybe<Ident>, _: Maybe<AsmName>) -> m<VarName> {
        return(NoName)
    };

    fn mkVarName(node: NodeInfo, $$$: Maybe<Ident>, asm: Maybe<AsmName>) -> m<VarName> {
        return(Operator("$"), VarName, n, asm)
    };

    fn nameOfDecl(d: CDecl) -> m<Ident> {
        getOnlyDeclr(d, Operator(">>="), Lambda)
    };

    fn splitCDecl(decl: CDecl, $$$: m<Vec<CDecl>>) -> m<Vec<CDecl>> {
        Case
    };

    fn tArraySize($$$: CArrSize) -> m<ArraySize> {
        return(UnknownArraySize(False))
    };

    fn tArraySize($$$: CArrSize) -> m<ArraySize> {
        return(UnknownArraySize(True))
    };

    fn tArraySize($$$: CArrSize) -> m<ArraySize> {
        liftM(ArraySize(static), return(szexpr))
    };

    fn tAttr($$$: CAttr) -> m<Attr> {
        return(Operator("$"), Attr, name, cexpr, node)
    };

    fn tCompType(tag: SUERef, sue_ref: CompTyKind, member_decls: Vec<CDecl>, attrs: Attributes, node: NodeInfo) -> m<CompType> {
        return(CompType(tag, sue_ref), Operator("ap"), concatMapM(tMemberDecls, member_decls), Operator("ap"), return(attrs), Operator("ap"), return(node))
    };

    fn tCompTypeDecl(handle_def: Bool, $$$: CStructUnion) -> m<CompTypeRef> {
        {
sue_ref(Operator("<-"), createSUERef, node_info, ident_opt);
Let;
attrs'(Operator("<-"), mapM, tAttr, attrs);
Let;
handleTagDecl(CompDecl(decl));
when(handle_def, Operator("$"), {
maybeM(member_decls_opt, Operator("$"), Lambda, sue_ref, tag', decls, attrs', node_info, Operator(">>="), handleTagDef.CompDef);
});
return(decl);
}
    };

    fn tDirectType(handle_sue_def: Bool, node: NodeInfo, ty_quals: Vec<CTypeQual>, canonTySpec: TypeSpecAnalysis) -> m<Type> {
        {
quals, attrs(Operator("<-"), tTypeQuals, ty_quals);
Let;
Case;
}
    };

    fn tEnumType(sue_ref: SUERef, enumerators: Vec<(Ident, Maybe<CExpr>)>, attrs: Attributes, node: NodeInfo) -> m<EnumType> {
        {
    let $$$ = || {
        mapAccumL(nextEnumerator, Left(0), enumerators)
    };
    let intExpr = |i| {
        CConst(CIntConst(cInteger(i), undefNode))
    };
    fn nextEnrExpr($$$: Either<Integer, (Expr, Integer)>, Nothing: Maybe<CExpr>) -> (Either<Integer, (Expr, Integer)>, CExpr) {
        Left(succ(i)), intExpr(i)
    };

    fn nextEnrExpr($$$: Either<Integer, (Expr, Integer)>, Nothing: Maybe<CExpr>) -> (Either<Integer, (Expr, Integer)>, CExpr) {
        Right(e, succ(offs)), offsExpr(e, offs)
    };

    fn nextEnrExpr(_: Either<Integer, (Expr, Integer)>, $$$: Maybe<CExpr>) -> (Either<Integer, (Expr, Integer)>, CExpr) {
        Right(e, 1), e
    };

    let nextEnumerator = |memo, $$$| {
        Let(memo', Enumerator(ident, expr, ty, nodeInfo(ident)))
    };
    let offsExpr = |e, offs| {
        CBinary(CAddOp, e, intExpr(offs), undefNode)
    };
    let ty = || {
        EnumType(sue_ref, enumerators', attrs, node)
    };
mapM_(handleEnumeratorDef, enumerators');
return(ty);
}
    };

    fn tMemberDecls($$$: CDecl) -> m<Vec<MemberDecl>> {
        {
Let;
when(is_inline, Operator("$"), astError, node, "member declaration with inline specifier".to_string());
canonTySpecs(Operator("<-"), canonicalTypeSpec, typespecs);
ty(Operator("<-"), tType, True, node, typequals, canonTySpecs, Vector([]), Vector([]));
Case;
}
    };

    fn tMemberDecls($$$: CDecl) -> m<Vec<MemberDecl>> {
        mapM(uncurry(tMemberDecl), zip(True(Operator(":"), repeat, False), declrs))
    };

    fn tNumType($$$: NumTypeSpec) -> m<Either<(FloatType, Bool), IntType>> {
        Case
    };

    fn tParamDecl($$$: CDecl) -> m<ParamDecl> {
        {
declr(Operator("<-"), getParamDeclr);
VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)(Operator("<-"), analyseVarDecl', True, declspecs, declr, Vector([]), Nothing);
when(is_inline, Operator("$"), throwTravError, badSpecifierError(node, "parameter declaration with inline specifier".to_string()));
storage(Operator("<-"), throwOnLeft, Operator("$"), computeParamStorage, node, storage_spec);
Let;
return(Operator("$"), paramDecl);
}
    };

    fn tTag(CStructTag: CStructTag) -> CompTyKind {
        StructTag
    };

    fn tTag(CUnionTag: CStructTag) -> CompTyKind {
        UnionTag
    };

    fn tType(handle_sue_def: Bool, top_node: NodeInfo, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, derived_declrs: Vec<CDerivedDeclr>, oldstyle_params: Vec<CDecl>) -> m<Type> {
        mergeOldStyle(top_node, oldstyle_params, derived_declrs, Operator(">>="), buildType)
    };

    fn tTypeQuals() -> m<(TypeQuals, Attributes)> {
        foldrM(go, noTypeQuals, Vector([]))
    };

    fn typeDefRef(t_node: NodeInfo, name: Ident) -> m<TypeDefRef> {
        lookupTypeDef(name, Operator(">>="), Lambda, TypeDefRef(name, Just(ty), t_node))
    };

}



ERROR   - "./language-c/src/Language/C/Analysis/DefTable.hs"



mod "./language-c/src/Language/C/Analysis/Export.hs" {
    fn exportArraySize($$$: ArraySize) -> CArrSize {
        CArrSize(static, e)
    };

    fn exportArraySize($$$: ArraySize) -> CArrSize {
        CNoArrSize(complete)
    };

    fn exportAttrs() -> Vec<CAttr> {
        map(exportAttr)
    };

    fn exportCompType($$$: CompType) -> Vec<CTypeSpec> {
        Vector([Span([Ref(Ident("CSUType")), Ref(Ident("comp")), Ref(Ident("ni"))])])
    };

    fn exportCompTypeDecl(ty: CompTypeRef) -> Vec<CTypeSpec> {
        Vector([Span([Ref(Ident("CSUType")), Parens([Span([Ref(Ident("exportComp")), Ref(Ident("ty"))])]), Ref(Ident("ni"))])])
    };

    fn exportCompTypeRef($$$: CompType) -> Vec<CTypeSpec> {
        exportCompTypeDecl(CompTypeRef(sue_ref, com_tag, node_info))
    };

    fn exportComplexType(ty: FloatType) -> Vec<CTypeSpec> {
        CComplexType(ni)(Operator(":"), exportFloatType, ty)
    };

    fn exportDeclAttrs($$$: DeclAttrs) -> Vec<CDeclSpec> {
        if(inline, then, Vector([Span([Ref(Ident("CTypeQual")), Parens([Span([Ref(Ident("CInlineQual")), Ref(Ident("ni"))])])])]), else, Vector([]))(Operator("++"), map, CStorageSpec, exportStorage(storage), Operator("++"), map, CTypeQual(Operator("."), CAttrQual), exportAttrs(attrs))
    };

    fn exportDeclr(other_specs: Vec<CDeclSpec>, ty: Type, attrs: Attributes, name: VarName) -> (Vec<CDeclSpec>, CDeclr) {
        other_specs(Operator("++"), specs), CDeclr(ident, derived, asmname, exportAttrs(attrs), ni)
    };

    fn exportEnumType($$$: EnumType) -> Vec<CTypeSpec> {
        Vector([Span([Ref(Ident("CEnumType")), Ref(Ident("enum")), Ref(Ident("ni"))])])
    };

    fn exportEnumTypeDecl(ty: EnumTypeRef) -> Vec<CTypeSpec> {
        Vector([Span([Ref(Ident("CEnumType")), Parens([Span([Ref(Ident("exportEnum")), Ref(Ident("ty"))])]), Ref(Ident("ni"))])])
    };

    fn exportEnumTypeRef($$$: EnumType) -> Vec<CTypeSpec> {
        exportEnumTypeDecl(EnumTypeRef(sue_ref, node_info))
    };

    fn exportFloatType(ty: FloatType) -> Vec<CTypeSpec> {
        Case
    };

    fn exportIntType(ty: IntType) -> Vec<CTypeSpec> {
        Case
    };

    fn exportMemberDecl($$$: MemberDecl) -> CDecl {
        CDecl(map(CTypeSpec, Operator("$"), exportTypeSpec, Operator("$"), fromDirectType, ty), Vector([Span([Parens([Span([Ref(Ident("Nothing"))]), Span([Ref(Ident("Nothing"))]), Span([Ref(Ident("Just")), Ref(Ident("expr"))])])])]), node_info)
    };

    fn exportMemberDecl($$$: MemberDecl) -> CDecl {
        Let(in, CDecl, specs, Vector([Span([Parens([Span([Ref(Ident("Just")), Ref(Ident("declarator"))]), Span([Ref(Ident("Nothing"))]), Span([Ref(Ident("bitfieldsz"))])])])]), node_info)
    };

    fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
        Let(in, CDecl, specs, Vector([Span([Parens([Span([Ref(Ident("Just")), Ref(Ident("declr"))]), Span([Ref(Ident("Nothing"))]), Span([Ref(Ident("Nothing"))])])])]), nodeInfo(paramdecl))
    };

    fn exportSUERef() -> Maybe<Ident> {
        Just(Operator("."), internalIdent, Operator("."), show)
    };

    fn exportStorage(NoStorage: Storage) -> Vec<CStorageSpec> {
        Vector([])
    };

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        if(reg, then, Vector([Span([Ref(Ident("CRegister")), Ref(Ident("ni"))])]), else, Vector([]))
    };

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        threadLocal(thread_local, Vector([Span([Ref(Ident("CStatic")), Ref(Ident("ni"))])]))
    };

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        threadLocal(thread_local, Vector([Span([Ref(Ident("CExtern")), Ref(Ident("ni"))])]))
    };

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        error("impossible storage: static without linkage".to_string())
    };

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        Vector([Span([Ref(Ident("CStatic")), Ref(Ident("ni"))])])
    };

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        Vector([])
    };

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        error("impossible storage: function without linkage".to_string())
    };

    fn exportType(ty: Type) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {
        exportTy(Vector([]), ty)
    };

    fn exportTypeDecl(ty: Type) -> CDecl {
        CDecl(declspecs, declrs, ni)
    };

    fn exportTypeDef($$$: TypeDef) -> CDecl {
        CDecl(CStorageSpec(CTypedef(ni), Operator(":"), declspecs), Vector([Span([Ref(Ident("declr"))])]), node_info)
    };

    fn exportTypeQuals(quals: TypeQuals) -> Vec<CTypeQual> {
        mapMaybe(select, Vector([Span([Parens([Span([Ref(Ident("constant"))]), Span([Ref(Ident("CConstQual")), Ref(Ident("ni"))])])]), Span([Parens([Span([Ref(Ident("volatile"))]), Span([Ref(Ident("CVolatQual")), Ref(Ident("ni"))])])]), Span([Parens([Span([Ref(Ident("restrict"))]), Span([Ref(Ident("CRestrQual")), Ref(Ident("ni"))])])])]))
    };

    fn exportTypeQualsAttrs(tyqs: TypeQuals, attrs: Attributes) -> Vec<CTypeQual> {
        exportTypeQuals(tyqs, Operator("++"), map, CAttrQual, exportAttrs(attrs))
    };

    fn exportTypeSpec(tyname: TypeName) -> Vec<CTypeSpec> {
        Case
    };

    fn exportVarDecl($$$: VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr(exportDeclAttrs(attrs), ty, Vector([]), name)
    };

    fn fromDirectType($$$: Type) -> TypeName {
        ty
    };

    fn fromDirectType($$$: Type) -> TypeName {
        maybe(error("undefined typeDef".to_string()), fromDirectType, ref)
    };

    fn fromDirectType(_: Type) -> TypeName {
        error("fromDirectType".to_string())
    };

    fn ni() -> NodeInfo {
        undefNode
    };

    fn threadLocal(False: Bool) -> Vec<CStorageSpec> {
        id
    };

    fn threadLocal(True: Bool) -> Vec<CStorageSpec> {
        CThread(ni)(Operator(":"))
    };

}



mod "./language-c/src/Language/C/Analysis/NameSpaceMap.hs" {
    fn defGlobal($$$: NameSpaceMap<k, a>, ident: k, def: a) -> (NameSpaceMap<k, a>, Maybe<a>) {
        NsMap(Map.insert(ident, def, gs), lss), Map.lookup(ident, gs)
    };

    fn defLocal(ns: NameSpaceMap<k, a>, $$$: k, $$$: a, ident: (NameSpaceMap<k, a>, Maybe<a>)) -> (NameSpaceMap<k, a>, Maybe<a>) {
        defGlobal(ns, ident, def)
    };

    fn defLocal($$$: NameSpaceMap<k, a>, ident: k, def: a) -> (NameSpaceMap<k, a>, Maybe<a>) {
        NsMap(gs, ident, def(Operator(":"), ls)(Operator(":"), lss)), Prelude.lookup(ident, ls)
    };

    fn enterNewScope($$$: NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
        NsMap(gs, Vector([])(Operator(":"), lss))
    };

    fn globalNames($$$: NameSpaceMap<k, v>) -> Map<k, v> {
        g
    };

    fn hasLocalNames($$$: NameSpaceMap<k, v>) -> Bool {
        not(null(l))
    };

    fn leaveScope($$$: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
        error("NsMaps.leaveScope: No local scope!".to_string())
    };

    fn leaveScope($$$: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
        NsMap(gs, lss), ls
    };

    fn localNames($$$: NameSpaceMap<k, v>) -> Vec<Vec<(k, v)>> {
        l
    };

    fn lookupGlobal($$$: NameSpaceMap<k, a>, ident: k) -> Maybe<a> {
        Map.lookup(ident, gs)
    };

    fn lookupInnermostScope(nsm: NameSpaceMap<k, a>, $$$: k, $$$: Maybe<a>) -> Maybe<a> {
        Case
    };

    fn lookupName(ns: NameSpaceMap<k, a>, $$$: k, $$$: Maybe<a>) -> Maybe<a> {
        Case
    };

    fn mergeNameSpace($$$: NameSpaceMap<k, a>, $$$: NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
        NsMap(Map.union(global1, global2), localUnion(local1, local2))
    };

    fn nameSpaceMap() -> NameSpaceMap<k, v> {
        NsMap(Map.empty, Vector([]))
    };

    fn nsMapToList($$$: NameSpaceMap<k, a>) -> Vec<(k, a)> {
        concat(lss, Operator("++"), Map.toList, gs)
    };

}



mod "./language-c/src/Language/C/Analysis/SemError.hs" {
    fn badSpecifierError(node_info: NodeInfo, msg: String) -> BadSpecifierError {
        BadSpecifierError(mkErrorInfo(LevelError, msg, node_info))
    };

    fn invalidAST(node_info: NodeInfo, msg: String) -> InvalidASTError {
        InvalidAST(mkErrorInfo(LevelError, msg, node_info))
    };

    fn prevDeclMsg(old_node: NodeInfo) -> Vec<String> {
        Vector([Span([Str("VGhlIHByZXZpb3VzIGRlY2xhcmF0aW9uIHdhcyBoZXJlOiA=")]), Span([Ref(Ident("show")), Parens([Span([Ref(Ident("posOfNode")), Ref(Ident("old_node"))])])])])
    };

    fn redefErrLabel($$$: RedefInfo) -> String {
        ident(Operator("++"), " redefined".to_string())
    };

    fn redefErrReason($$$: RedefInfo) -> String {
        "duplicate definition of ".to_string()(Operator("++"), ident)
    };

    fn redefErrReason($$$: RedefInfo) -> String {
        "this declaration of ".to_string()(Operator("++"), ident, Operator("++"), " shadows a previous one".to_string())
    };

    fn redefErrReason($$$: RedefInfo) -> String {
        ident(Operator("++"), " previously declared as a different kind of symbol".to_string())
    };

    fn redefErrReason($$$: RedefInfo) -> String {
        ident(Operator("++"), " previously declared with different linkage".to_string())
    };

    fn redefErrReason($$$: RedefInfo) -> String {
        ident(Operator("++"), " previously declared without linkage".to_string())
    };

    fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, $$$: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, posOfNode(node), Vector([Span([Ref(Ident("redefErrReason")), Ref(Ident("info"))])])(Operator("++"), prevDeclMsg, old_node))
    };

    fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
        RedefError(lvl, RedefInfo(ctx, kind, new, old))
    };

    fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    };

    fn typeMismatchInfo($$$: TypeMismatch) -> ErrorInfo {
        ErrorInfo(LevelError, posOfNode(node1), Vector([Span([Ref(Ident("reason"))])]))
    };

}



ERROR   - "./language-c/src/Language/C/Analysis/SemRep.hs"



ERROR   - "./language-c/src/Language/C/Analysis/TravMonad.hs"



ERROR   - "./language-c/src/Language/C/Analysis/TypeCheck.hs"



mod "./language-c/src/Language/C/Analysis/TypeConversions.hs" {
    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(Operator("$"), TyComplex, Operator("$"), floatConversion, t1, t2)
    };

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(Operator("$"), TyComplex, Operator("$"), floatConversion, t1, t2)
    };

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(Operator("$"), TyComplex, Operator("$"), floatConversion, t1, t2)
    };

    fn arithmeticConversion(t1: TypeName, $$$: TypeName, $$$: Maybe<TypeName>) -> Maybe<TypeName> {
        Just(t1)
    };

    fn arithmeticConversion($$$: TypeName, t2: TypeName, $$$: Maybe<TypeName>) -> Maybe<TypeName> {
        Just(t2)
    };

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(Operator("$"), TyFloating, Operator("$"), floatConversion, t1, t2)
    };

    fn arithmeticConversion(t1: TypeName, $$$: TypeName, $$$: Maybe<TypeName>) -> Maybe<TypeName> {
        Just(t1)
    };

    fn arithmeticConversion($$$: TypeName, t2: TypeName, $$$: Maybe<TypeName>) -> Maybe<TypeName> {
        Just(t2)
    };

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(Operator("$"), TyIntegral, Operator("$"), intConversion, t1, t2)
    };

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(Operator("$"), TyIntegral, TyInt)
    };

    fn arithmeticConversion($$$: TypeName, t2: TypeName) -> Maybe<TypeName> {
        Just(Operator("$"), t2)
    };

    fn arithmeticConversion(t1: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(Operator("$"), t1)
    };

    fn arithmeticConversion(_: TypeName, _: TypeName) -> Maybe<TypeName> {
        Nothing
    };

    fn floatConversion() -> FloatType {
        max
    };

    fn intConversion(t1: IntType, t2: IntType) -> IntType {
        max(TyInt, max(t1, t2))
    };

}



ERROR   - "./language-c/src/Language/C/Analysis/TypeUtils.hs"



mod "./language-c/src/Language/C/Analysis.hs" {

}



ERROR   - "./language-c/src/Language/C/Data/Error.hs"



mod "./language-c/src/Language/C/Data/Ident.hs" {
    fn bits14() -> Int {
        2(Operator("^"), 14(Operator("::"), Int))
    };

    fn bits21() -> Int {
        2(Operator("^"), 21(Operator("::"), Int))
    };

    fn bits28() -> Int {
        2(Operator("^"), 28(Operator("::"), Int))
    };

    fn bits7() -> Int {
        2(Operator("^"), 7(Operator("::"), Int))
    };

    fn builtinIdent(s: String) -> Ident {
        Ident(s, quad(s), mkNodeInfoOnlyPos(builtinPos))
    };

    fn dumpIdent(ide: Ident) -> String {
        identToString(ide, Operator("++"), " at ".to_string(), Operator("++"), show, nodeInfo(ide))
    };

    fn identToString($$$: Ident) -> String {
        s
    };

    fn internalIdent(s: String) -> Ident {
        Ident(s, quad(s), mkNodeInfoOnlyPos(internalPos))
    };

    fn internalIdentAt(pos: Position, s: String) -> Ident {
        Ident(s, quad(s), mkNodeInfoPosLen(pos, pos, length(s)))
    };

    fn isAnonymousRef($$$: SUERef) -> Bool {
        True
    };

    fn isAnonymousRef(_: SUERef) -> Bool {
        False
    };

    fn isInternalIdent($$$: Ident) -> Bool {
        isInternalPos(posOfNode(nodeinfo))
    };

    fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, quad(s), mkNodeInfo'(pos, pos, length(s), name))
    };

    fn quad($$$: String) -> Int {
        ord(c4, Operator("*"), bits21, Operator("+"), ord, c3, Operator("*"), bits14, Operator("+"), ord, c2, Operator("*"), bits7, Operator("+"), ord, c1)(Operator("mod"), bits28)(Operator("+"), quad(s, Operator("mod"), bits28))
    };

    fn quad($$$: String) -> Int {
        ord(c3, Operator("*"), bits14, Operator("+"), ord, c2, Operator("*"), bits7, Operator("+"), ord, c1)
    };

    fn quad($$$: String) -> Int {
        ord(c2, Operator("*"), bits7, Operator("+"), ord, c1)
    };

    fn quad($$$: String) -> Int {
        ord(c1)
    };

    fn quad($$$: String) -> Int {
        0
    };

}



mod "./language-c/src/Language/C/Data/InputStream.hs" {
    fn countLines() -> Int {
        length(Operator("."), BSC.lines)
    };

    fn countLines() -> Int {
        length(Operator("."), lines)
    };

    fn inputStreamEmpty() -> Bool {
        BSW.null
    };

    fn inputStreamEmpty() -> Bool {
        null
    };

    fn inputStreamFromString() -> InputStream {
        BSC.pack
    };

    fn inputStreamFromString() -> InputStream {
        id
    };

    fn inputStreamToString() -> String {
        BSC.unpack
    };

    fn inputStreamToString() -> String {
        id
    };

    fn readInputStream() -> IO<InputStream> {
        BSW.readFile
    };

    fn readInputStream() -> IO<InputStream> {
        readFile
    };

    fn takeByte(bs: InputStream) -> (Word8, InputStream) {
        BSW.head(bs, Operator("seq"), BSW.head(bs), BSW.tail(bs))
    };

    fn takeChar(bs: InputStream) -> (Char, InputStream) {
        BSC.head(bs, Operator("seq"), BSC.head(bs), BSC.tail(bs))
    };

    fn takeChar(bs: InputStream) -> (Char, InputStream) {
        head(bs), tail(bs)
    };

    fn takeChars($$$: Int, bstr: InputStream) -> Vec<Char> {
        BSC.unpack(Operator("$"), BSC.take, n, bstr)
    };

    fn takeChars(n: Int, str: InputStream) -> Vec<Char> {
        take(n, str)
    };

}



mod "./language-c/src/Language/C/Data/Name.hs" {
    fn namesStartingFrom(k: Int) -> Vec<Name> {
        Vector([Span([Ref(Ident("Name")), Ref(Ident("k.."))])])
    };

    fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    };

}



ERROR   - "./language-c/src/Language/C/Data/Node.hs"



mod "./language-c/src/Language/C/Data/Position.hs" {
    fn adjustPos(fname: FilePath, row: Int, $$$: Position) -> Position {
        Position(offs, fname, row, 1)
    };

    fn adjustPos(_: FilePath, _: Int, p: Position) -> Position {
        p
    };

    fn builtinPos() -> Position {
        BuiltinPosition
    };

    fn incOffset($$$: Position, n: Int) -> Position {
        Position(o(Operator("+"), n), f, r, c)
    };

    fn incOffset(p: Position, n: Int) -> Position {
        p
    };

    fn incPos($$$: Position, n: Int) -> Position {
        Position(offs(Operator("+"), n), fname, row, col(Operator("+"), n))
    };

    fn incPos(p: Position, _: Int) -> Position {
        p
    };

    fn initPos(file: FilePath) -> Position {
        Position(0, file, 1, 1)
    };

    fn internalPos() -> Position {
        InternalPosition
    };

    fn isBuiltinPos(BuiltinPosition: Position) -> Bool {
        True
    };

    fn isBuiltinPos(_: Position) -> Bool {
        False
    };

    fn isInternalPos(InternalPosition: Position) -> Bool {
        True
    };

    fn isInternalPos(_: Position) -> Bool {
        False
    };

    fn isNoPos(NoPosition: Position) -> Bool {
        True
    };

    fn isNoPos(_: Position) -> Bool {
        False
    };

    fn isSourcePos($$$: Position) -> Bool {
        True
    };

    fn isSourcePos(_: Position) -> Bool {
        False
    };

    fn nopos() -> Position {
        NoPosition
    };

    fn position() -> Position {
        Position
    };

    fn retPos($$$: Position) -> Position {
        Position(offs(Operator("+"), 1), fname, row(Operator("+"), 1), 1)
    };

    fn retPos(p: Position) -> Position {
        p
    };

}



ERROR   - "./language-c/src/Language/C/Data/RList.hs"



mod "./language-c/src/Language/C/Data.hs" {

}



mod "./language-c/src/Language/C/Parser/Builtin.hs" {
    fn builtinTypeNames() -> Vec<Ident> {
        Vector([Span([Ref(Ident("builtinIdent")), Str("X19idWlsdGluX3ZhX2xpc3Q=")])])
    };

}



ERROR   - "./language-c/src/Language/C/Parser/Lexer.x"



ERROR   - "./language-c/src/Language/C/Parser/Parser.y"



ERROR   - "./language-c/src/Language/C/Parser/ParserMonad.hs"



ERROR   - "./language-c/src/Language/C/Parser/Tokens.hs"



mod "./language-c/src/Language/C/Parser.hs" {
    fn execParser_(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
        fmap(fst, Operator("$"), execParser, parser, input, pos, builtinTypeNames, newNameSupply)
    };

}



ERROR   - "./language-c/src/Language/C/Pretty.hs"



ERROR   - "./language-c/src/Language/C/Syntax/AST.hs"



mod "./language-c/src/Language/C/Syntax/Constants.hs" {
    fn _showWideFlag(flag: Bool) -> ShowS {
        if(flag, then, showString, "L".to_string(), else, id)
    };

    fn cChar(c: Char) -> CChar {
        CChar(c, False)
    };

    fn cChar_w(c: Char) -> CChar {
        CChar(c, True)
    };

    fn cChars() -> CChar {
        CChars
    };

    fn cFloat() -> CFloat {
        CFloat(Operator("."), show)
    };

    fn cInteger(i: Integer) -> CInteger {
        CInteger(i, DecRepr, noFlags)
    };

    fn cString(str: String) -> CString {
        CString(str, False)
    };

    fn cString_w(str: String) -> CString {
        CString(str, True)
    };

    fn clearFlag(flag: f, $$$: Flags<f>) -> Flags<f> {
        Flags(Operator("$"), k, Operator("clearBit"), fromEnum, flag)
    };

    fn concatCStrings(cs: Vec<CString>) -> CString {
        CString(concatMap(getCString, cs), any(isWideString, cs))
    };

    fn dQuote(s: String, t: ShowS) -> ShowS {
        "\"\"".to_string()(Operator(":"), s)(Operator("++"), "\\\"".to_string(), Operator("++"), t)
    };

    fn escapeCChar($$$: Char) -> String {
        "\\\\\'".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\\\\\".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\a".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\b".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\e".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\f".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\n".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\r".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\t".to_string()
    };

    fn escapeChar($$$: Char) -> String {
        "\\\\v".to_string()
    };

    fn getCChar($$$: CChar) -> Vec<Char> {
        Vector([Span([Ref(Ident("c"))])])
    };

    fn getCChar($$$: CChar) -> Vec<Char> {
        cs
    };

    fn getCCharAsInt($$$: CChar) -> Integer {
        fromIntegral(fromEnum(c))
    };

    fn getCCharAsInt($$$: CChar) -> Integer {
        error("integer value of multi-character character constants is implementation defined".to_string())
    };

    fn getCInteger($$$: CInteger) -> Integer {
        i
    };

    fn getCString($$$: CString) -> String {
        str
    };

    fn head'(err: String, $$$: Vec<a>) -> a {
        error(err)
    };

    fn head'(_: String, $$$: Vec<a>) -> a {
        x
    };

    fn isAsciiSourceChar(c: Char) -> Bool {
        isAscii(c, Operator("&&"), isPrint, c)
    };

    fn isCChar($$$: Char) -> Bool {
        False
    };

    fn isCChar($$$: Char) -> Bool {
        False
    };

    fn isCChar($$$: Char) -> Bool {
        False
    };

    fn isCChar(c: Char) -> Bool {
        isAsciiSourceChar(c)
    };

    fn isSChar($$$: Char) -> Bool {
        False
    };

    fn isSChar($$$: Char) -> Bool {
        False
    };

    fn isSChar($$$: Char) -> Bool {
        False
    };

    fn isSChar(c: Char) -> Bool {
        isAsciiSourceChar(c)
    };

    fn isWideChar($$$: CChar) -> Bool {
        wideFlag
    };

    fn isWideChar($$$: CChar) -> Bool {
        wideFlag
    };

    fn isWideString($$$: CString) -> Bool {
        wideflag
    };

    fn noFlags() -> Flags<f> {
        Flags(0)
    };

    fn readCFloat() -> CFloat {
        CFloat
    };

    fn readCInteger(repr: CIntRepr, str: String) -> Either<String, CInteger> {
        Case
    };

    fn sQuote(s: String, t: ShowS) -> ShowS {
        "\'".to_string()(Operator("++"), s, Operator("++"), "\'".to_string(), Operator("++"), t)
    };

    fn setFlag(flag: f, $$$: Flags<f>) -> Flags<f> {
        Flags(Operator("$"), k, Operator("setBit"), fromEnum, flag)
    };

    fn showCharConst(c: Char) -> ShowS {
        sQuote(Operator("$"), escapeCChar, c)
    };

    fn showStringLit() -> ShowS {
        dQuote(Operator("."), concatMap, showStringChar)
    };

    fn testFlag(flag: f, $$$: Flags<f>) -> Bool {
        k(Operator("testBit"), fromEnum, flag)
    };

    fn unescapeChar($$$: String) -> (Char, String) {
        Case
    };

    fn unescapeChar($$$: String) -> (Char, String) {
        c, cs
    };

    fn unescapeChar($$$: String) -> (Char, String) {
        error(Operator("$"), "unescape char: empty string".to_string())
    };

    fn unescapeString($$$: String) -> String {
        Vector([])
    };

    fn unescapeString(cs: String) -> String {
        Case
    };

}



mod "./language-c/src/Language/C/Syntax/Ops.hs" {
    fn assignBinop(CAssignOp: CAssignOp) -> CBinaryOp {
        error("direct assignment has no binary operator".to_string())
    };

    fn assignBinop(CMulAssOp: CAssignOp) -> CBinaryOp {
        CMulOp
    };

    fn assignBinop(CDivAssOp: CAssignOp) -> CBinaryOp {
        CDivOp
    };

    fn assignBinop(CRmdAssOp: CAssignOp) -> CBinaryOp {
        CRmdOp
    };

    fn assignBinop(CAddAssOp: CAssignOp) -> CBinaryOp {
        CAddOp
    };

    fn assignBinop(CSubAssOp: CAssignOp) -> CBinaryOp {
        CSubOp
    };

    fn assignBinop(CShlAssOp: CAssignOp) -> CBinaryOp {
        CShlOp
    };

    fn assignBinop(CShrAssOp: CAssignOp) -> CBinaryOp {
        CShrOp
    };

    fn assignBinop(CAndAssOp: CAssignOp) -> CBinaryOp {
        CAndOp
    };

    fn assignBinop(CXorAssOp: CAssignOp) -> CBinaryOp {
        CXorOp
    };

    fn assignBinop(COrAssOp: CAssignOp) -> CBinaryOp {
        COrOp
    };

    fn isBitOp(op: CBinaryOp) -> Bool {
        op(Operator("elem"), Vector([Span([Ref(Ident("CShlOp"))]), Span([Ref(Ident("CShrOp"))]), Span([Ref(Ident("CAndOp"))]), Span([Ref(Ident("COrOp"))]), Span([Ref(Ident("CXorOp"))])]))
    };

    fn isCmpOp(op: CBinaryOp) -> Bool {
        op(Operator("elem"), Vector([Span([Ref(Ident("CLeqOp"))]), Span([Ref(Ident("CGeqOp"))]), Span([Ref(Ident("CLeOp"))]), Span([Ref(Ident("CGrOp"))]), Span([Ref(Ident("CEqOp"))]), Span([Ref(Ident("CNeqOp"))])]))
    };

    fn isEffectfulOp(op: CUnaryOp) -> Bool {
        op(Operator("elem"), Vector([Span([Ref(Ident("CPreIncOp"))]), Span([Ref(Ident("CPreDecOp"))]), Span([Ref(Ident("CPostIncOp"))]), Span([Ref(Ident("CPostDecOp"))])]))
    };

    fn isLogicOp(op: CBinaryOp) -> Bool {
        op(Operator("elem"), Vector([Span([Ref(Ident("CLndOp"))]), Span([Ref(Ident("CLorOp"))])]))
    };

    fn isPtrOp(op: CBinaryOp) -> Bool {
        op(Operator("elem"), Vector([Span([Ref(Ident("CAddOp"))]), Span([Ref(Ident("CSubOp"))])]))
    };

}



ERROR   - "./language-c/src/Language/C/Syntax/Utils.hs"



mod "./language-c/src/Language/C/Syntax.hs" {

}



mod "./language-c/src/Language/C/System/GCC.hs" {
    fn buildCppArgs($$$: CppArgs) -> Vec<String> {
        {
concatMap(tOption, options);
}(Operator("++"), outputFileOpt, Operator("++"), Vector([Span([Str("LUU=")]), Span([Ref(Ident("input_file"))])]), Operator("++"), extra_args)
    };

    fn gccParseCPPArgs(args: Vec<String>) -> Either<String, (CppArgs, Vec<String>)> {
        Case
    };

    fn newGCC() -> GCC {
        GCC
    };

}



mod "./language-c/src/Language/C/System/Preprocess.hs" {
    fn addCppOption(cpp_args: CppArgs, opt: CppOption) -> CppArgs {
        cpp_args(hashmap! {
"cppOptions" => opt(Operator(":"), cppOptions(cpp_args))
})
    };

    fn addExtraOption(cpp_args: CppArgs, extra: String) -> CppArgs {
        cpp_args(hashmap! {
"extraOptions" => extra(Operator(":"), extraOptions(cpp_args))
})
    };

    fn cppFile(input_file: FilePath) -> CppArgs {
        CppArgs(hashmap! {
"cppOptions" => Vector([]), "extraOptions" => Vector([]), "cppTmpDir" => Nothing, "inputFile" => input_file, "outputFile" => Nothing
})
    };

    fn isPreprocessed() -> Bool {
        ".i".to_string()(Operator("isSuffixOf"))
    };

    fn mkOutputFile(tmp_dir_opt: Maybe<FilePath>, input_file: FilePath) -> IO<FilePath> {
        {
tmpDir(Operator("<-"), getTempDir, tmp_dir_opt);
mkTmpFile(tmpDir, getOutputFileName(input_file));
}
    };

    fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> IO<FilePath> {
        {
path, file_handle(Operator("<-"), openTempFile, tmp_dir, file_templ);
hClose(file_handle);
return(path);
}
    };

    fn preprocessedExt() -> String {
        ".i".to_string()
    };

    fn rawCppArgs(opts: Vec<String>, input_file: FilePath) -> CppArgs {
        CppArgs(hashmap! {
"inputFile" => input_file, "cppOptions" => Vector([]), "extraOptions" => opts, "outputFile" => Nothing, "cppTmpDir" => Nothing
})
    };

    fn runPreprocessor(cpp: cpp, cpp_args: CppArgs) -> IO<Either<ExitCode, InputStream>> {
        {
    fn getActualOutFile() -> IO<FilePath> {
        maybe(mkOutputFile(cppTmpDir(cpp_args), inputFile(cpp_args)), return, outputFile(cpp_args))
    };

    let invokeCpp = |actual_out_file| {
        {
exit_code(Operator("<-"), runCPP, cpp, cpp_args(hashmap! {
"outputFile" => Just(actual_out_file)
}));
Case;
}
    };
    let removeTmpOutFile = |out_file| {
        maybe(removeFile(out_file), Lambda(), outputFile(cpp_args))
    };
bracket(getActualOutFile, removeTmpOutFile, invokeCpp);
}
    };

}



