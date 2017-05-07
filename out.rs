ERROR   - "./language-c/src/Language/C/Analysis/AstAnalysis.hs"



mod "./language-c/src/Language/C/Analysis/AstAnalysis.hs-boot" {
}



mod "./language-c/src/Language/C/Analysis/Builtins.hs" {
    fn builtins() -> DefTable {
        foldr
        doIdent
        foldr, doTypeDef, emptyDefTable, typedefs, 
        idents
    }

}



ERROR   - "./language-c/src/Language/C/Analysis/ConstEval.hs"



ERROR   - "./language-c/src/Language/C/Analysis/Debug.hs"



mod "./language-c/src/Language/C/Analysis/DeclAnalysis.hs" {
    fn analyseVarDecl(handle_sue_def: Bool, storage_specs: [CStorageSpec], decl_attrs: [CAttr], typequals: [CTypeQual], canonTySpecs: TypeSpecAnalysis, inline: Bool, $$$: CDeclr, oldstyle_params: [CDecl], init_opt: <Maybe CInit>, ) -> m VarDeclInfo {
        { storage_spec, Dummy, canonicalStorageSpec, storage_specs, ; typ, Dummy, tType, handle_sue_def, node, typequals, canonTySpecs, derived_declrs, oldstyle_params, ; attrs', Dummy, mapM, tAttr, decl_attrs, Dummy, declr_attrs, , ; name, Dummy, mkVarName, node, name_opt, asmname_opt, ; return, Dummy, VarDeclInfo, name, inline, storage_spec, attrs', typ, node, ;  }
    }

    fn analyseVarDecl'(handle_sue_def: Bool, declspecs: [CDeclSpec], declr: CDeclr, oldstyle: [CDecl], init_opt: <Maybe CInit>, ) -> m VarDeclInfo {
        { Dummy, ; canonTySpecs, Dummy, canonicalTypeSpec, type_specs, ; analyseVarDecl, handle_sue_def, storage_specs, attrs, type_quals, canonTySpecs, inline, declr, oldstyle, init_opt, ;  }
    }

    fn canonicalStorageSpec(storagespecs: [CStorageSpec], ) -> m StorageSpec {
        liftM
        elideAuto
        Dummy
        foldrM
        updStorage
        NoStorageSpec
        storagespecs
    }

    fn canonicalTypeSpec() -> m TypeSpecAnalysis {
        foldrM
        go
        TSNone
    }

    fn computeParamStorage(_: NodeInfo, NoStorageSpec: StorageSpec, ) -> Either BadSpecifierError Storage {
        Right
        Auto, False, 
    }

    fn computeParamStorage(_: NodeInfo, RegSpec: StorageSpec, ) -> Either BadSpecifierError Storage {
        Right
        Auto, True, 
    }

    fn computeParamStorage(node: NodeInfo, spec: StorageSpec, ) -> Either BadSpecifierError Storage {
        Left
        Dummy
        badSpecifierError
        node
        Dummy
        Dummy
        Dummy
        show
        spec
    }

    fn emptyDeclr(node: NodeInfo, ) -> CDeclr {
        CDeclr
        Nothing
        
        Nothing
        
        node
    }

    fn emptyNumTypeSpec() -> NumTypeSpec {
        NumTypeSpec
        Dummy
    }

    fn getOnlyDeclr($$$: CDecl, ) -> m CDeclr {
        return
        declr
    }

    fn getOnlyDeclr($$$: CDecl, ) -> m CDeclr {
        internalErr
        Dummy
    }

    fn hasThreadLocalSpec(ThreadSpec: StorageSpec, ) -> Bool {
        True
    }

    fn hasThreadLocalSpec($$$: StorageSpec, ) -> Bool {
        b
    }

    fn hasThreadLocalSpec($$$: StorageSpec, ) -> Bool {
        b
    }

    fn hasThreadLocalSpec(_: StorageSpec, ) -> Bool {
        False
    }

    fn isTypeDef(declspecs: [CDeclSpec], ) -> Bool {
        not
        Dummy
        null
        Dummy
    }

    fn mergeOldStyle(_node: NodeInfo, $$$: [CDecl], declrs: [CDerivedDeclr], ) -> m [CDerivedDeclr] {
        return
        declrs
    }

    fn mergeOldStyle(node: NodeInfo, oldstyle_params: [CDecl], $$$: [CDerivedDeclr], ) -> m [CDerivedDeclr] {
        Dummy
    }

    fn mergeOldStyle(node: NodeInfo, _: [CDecl], _: [CDerivedDeclr], ) -> m [CDerivedDeclr] {
        astError
        node
        Dummy
    }

    fn mergeTypeAttributes(node_info: NodeInfo, quals: TypeQuals, attrs: [Attr], typ: Type, ) -> m Type {
        Dummy
    }

    fn mkVarName(node: NodeInfo, Nothing: Maybe Ident, _: Maybe AsmName, ) -> m VarName {
        return
        NoName
    }

    fn mkVarName(node: NodeInfo, $$$: Maybe Ident, asm: Maybe AsmName, ) -> m VarName {
        return
        Dummy
        VarName
        n
        asm
    }

    fn nameOfDecl(d: CDecl, ) -> m Ident {
        getOnlyDeclr
        d
        Dummy
        Dummy
    }

    fn splitCDecl(decl: CDecl, $$$: m [CDecl], ) -> m [CDecl] {
        Dummy
    }

    fn tArraySize($$$: CArrSize, ) -> m ArraySize {
        return
        UnknownArraySize, False, 
    }

    fn tArraySize($$$: CArrSize, ) -> m ArraySize {
        return
        UnknownArraySize, True, 
    }

    fn tArraySize($$$: CArrSize, ) -> m ArraySize {
        liftM
        ArraySize, static, 
        return, szexpr, 
    }

    fn tAttr($$$: CAttr, ) -> m Attr {
        return
        Dummy
        Attr
        name
        cexpr
        node
    }

    fn tCompType(tag: SUERef, sue_ref: CompTyKind, member_decls: [CDecl], attrs: Attributes, node: NodeInfo, ) -> m CompType {
        return
        CompType, tag, sue_ref, 
        Dummy
        concatMapM, tMemberDecls, member_decls, 
        Dummy
        return, attrs, 
        Dummy
        return, node, 
    }

    fn tCompTypeDecl(handle_def: Bool, $$$: CStructUnion, ) -> m CompTypeRef {
        { sue_ref, Dummy, createSUERef, node_info, ident_opt, ; Dummy, ; attrs', Dummy, mapM, tAttr, attrs, ; Dummy, ; handleTagDecl, CompDecl, decl, , ; when, handle_def, , Dummy, { maybeM, member_decls_opt, Dummy, Dummy, sue_ref, tag', decls, attrs', , node_info, Dummy, handleTagDef.CompDef, , ;  }, ; return, decl, ;  }
    }

    fn tDirectType(handle_sue_def: Bool, node: NodeInfo, ty_quals: [CTypeQual], canonTySpec: TypeSpecAnalysis, ) -> m Type {
        { quals, attrs, , Dummy, tTypeQuals, ty_quals, ; Dummy, ; Dummy, ;  }
    }

    fn tEnumType(sue_ref: SUERef, enumerators: [<Ident, Maybe CExpr>], attrs: Attributes, node: NodeInfo, ) -> m EnumType {
        { {
        let ty = EnumType, sue_ref, enumerators', attrs, node, 
        let $$$ = mapAccumL, nextEnumerator, Left, Dummy, , enumerators, 
        let nextEnumerator = Dummy, memo', Enumerator, ident, expr, ty, nodeInfo, ident, , , 
        let nextEnrExpr = Left, succ, i, , intExpr, i, , 
        let nextEnrExpr = Right, e, succ, offs, , offsExpr, e, offs, , 
        let nextEnrExpr = Right, e, Dummy, , e, , 
        let intExpr = CConst, CIntConst, cInteger, i, , undefNode, , 
        let offsExpr = CBinary, CAddOp, e, intExpr, offs, , undefNode, 
    }
mapM_, handleEnumeratorDef, enumerators', ; return, ty, ;  }
    }

    fn tMemberDecls($$$: CDecl, ) -> m [MemberDecl] {
        { Dummy, ; when, is_inline, Dummy, astError, node, Dummy, ; canonTySpecs, Dummy, canonicalTypeSpec, typespecs, ; ty, Dummy, tType, True, node, typequals, canonTySpecs, , , ; Dummy, ;  }
    }

    fn tMemberDecls($$$: CDecl, ) -> m [MemberDecl] {
        mapM
        uncurry, tMemberDecl, 
        zip, True, Dummy, repeat, False, , declrs, 
    }

    fn tNumType($$$: NumTypeSpec, ) -> m <Either <FloatType, Bool> IntType> {
        Dummy
    }

    fn tParamDecl($$$: CDecl, ) -> m ParamDecl {
        { declr, Dummy, getParamDeclr, ; VarDeclInfo, name, is_inline, storage_spec, attrs, ty, declr_node, , Dummy, analyseVarDecl', True, declspecs, declr, , Nothing, ; when, is_inline, , Dummy, throwTravError, badSpecifierError, node, Dummy, , ; storage, Dummy, throwOnLeft, Dummy, computeParamStorage, node, storage_spec, ; Dummy, ; return, Dummy, paramDecl, ;  }
    }

    fn tTag(CStructTag: CStructTag, ) -> CompTyKind {
        StructTag
    }

    fn tTag(CUnionTag: CStructTag, ) -> CompTyKind {
        UnionTag
    }

    fn tType(handle_sue_def: Bool, top_node: NodeInfo, typequals: [CTypeQual], canonTySpecs: TypeSpecAnalysis, derived_declrs: [CDerivedDeclr], oldstyle_params: [CDecl], ) -> m Type {
        mergeOldStyle
        top_node
        oldstyle_params
        derived_declrs
        Dummy
        buildType
    }

    fn tTypeQuals() -> m <TypeQuals, Attributes> {
        foldrM
        go
        noTypeQuals, , 
    }

    fn typeDefRef(t_node: NodeInfo, name: Ident, ) -> m TypeDefRef {
        lookupTypeDef
        name
        Dummy
        Dummy
        TypeDefRef, name, Just, ty, , t_node, 
    }

}



ERROR   - "./language-c/src/Language/C/Analysis/DefTable.hs"



mod "./language-c/src/Language/C/Analysis/Export.hs" {
    fn exportArraySize($$$: ArraySize, ) -> CArrSize {
        CArrSize
        static
        e
    }

    fn exportArraySize($$$: ArraySize, ) -> CArrSize {
        CNoArrSize
        complete
    }

    fn exportAttrs() -> [CAttr] {
        map
        exportAttr
    }

    fn exportCompType($$$: CompType, ) -> [CTypeSpec] {
        CSUType, comp, ni, 
    }

    fn exportCompTypeDecl(ty: CompTypeRef, ) -> [CTypeSpec] {
        CSUType, exportComp, ty, , ni, 
    }

    fn exportCompTypeRef($$$: CompType, ) -> [CTypeSpec] {
        exportCompTypeDecl
        CompTypeRef, sue_ref, com_tag, node_info, 
    }

    fn exportComplexType(ty: FloatType, ) -> [CTypeSpec] {
        CComplexType, ni, 
        Dummy
        exportFloatType
        ty
    }

    fn exportDeclAttrs($$$: DeclAttrs, ) -> [CDeclSpec] {
        if, inline, then, CTypeQual, CInlineQual, ni, , , else, , 
        Dummy
        map
        CStorageSpec, 
        exportStorage, storage, 
        Dummy
        map
        CTypeQual, Dummy, CAttrQual, 
        exportAttrs, attrs, 
    }

    fn exportDeclr(other_specs: [CDeclSpec], ty: Type, attrs: Attributes, name: VarName, ) -> <[CDeclSpec], CDeclr> {
        other_specs, Dummy, specs, CDeclr, ident, derived, asmname, exportAttrs, attrs, , ni, 
    }

    fn exportEnumType($$$: EnumType, ) -> [CTypeSpec] {
        CEnumType, enum, ni, 
    }

    fn exportEnumTypeDecl(ty: EnumTypeRef, ) -> [CTypeSpec] {
        CEnumType, exportEnum, ty, , ni, 
    }

    fn exportEnumTypeRef($$$: EnumType, ) -> [CTypeSpec] {
        exportEnumTypeDecl
        EnumTypeRef, sue_ref, node_info, 
    }

    fn exportFloatType(ty: FloatType, ) -> [CTypeSpec] {
        Dummy
    }

    fn exportIntType(ty: IntType, ) -> [CTypeSpec] {
        Dummy
    }

    fn exportMemberDecl($$$: MemberDecl, ) -> CDecl {
        CDecl
        map, CTypeSpec, Dummy, exportTypeSpec, Dummy, fromDirectType, ty, 
        Nothing, Nothing, Just, expr, , 
        node_info
    }

    fn exportMemberDecl($$$: MemberDecl, ) -> CDecl {
        Dummy
        in
        CDecl
        specs
        Just, declarator, Nothing, bitfieldsz, , 
        node_info
    }

    fn exportParamDecl(paramdecl: ParamDecl, ) -> CDecl {
        Dummy
        in
        CDecl
        specs
        Just, declr, Nothing, Nothing, , 
        nodeInfo, paramdecl, 
    }

    fn exportSUERef() -> Maybe Ident {
        Just
        Dummy
        internalIdent
        Dummy
        show
    }

    fn exportStorage(NoStorage: Storage, ) -> [CStorageSpec] {
        
    }

    fn exportStorage($$$: Storage, ) -> [CStorageSpec] {
        if
        reg
        then
        CRegister, ni, 
        else
        
    }

    fn exportStorage($$$: Storage, ) -> [CStorageSpec] {
        threadLocal
        thread_local
        CStatic, ni, 
    }

    fn exportStorage($$$: Storage, ) -> [CStorageSpec] {
        threadLocal
        thread_local
        CExtern, ni, 
    }

    fn exportStorage($$$: Storage, ) -> [CStorageSpec] {
        error
        Dummy
    }

    fn exportStorage($$$: Storage, ) -> [CStorageSpec] {
        CStatic, ni, 
    }

    fn exportStorage($$$: Storage, ) -> [CStorageSpec] {
        
    }

    fn exportStorage($$$: Storage, ) -> [CStorageSpec] {
        error
        Dummy
    }

    fn exportType(ty: Type, ) -> <[CDeclSpec], [CDerivedDeclr]> {
        exportTy
        
        ty
    }

    fn exportTypeDecl(ty: Type, ) -> CDecl {
        CDecl
        declspecs
        declrs
        ni
    }

    fn exportTypeDef($$$: TypeDef, ) -> CDecl {
        CDecl
        CStorageSpec, CTypedef, ni, , Dummy, declspecs, 
        declr, 
        node_info
    }

    fn exportTypeQuals(quals: TypeQuals, ) -> [CTypeQual] {
        mapMaybe
        select
        constant, CConstQual, ni, , volatile, CVolatQual, ni, , restrict, CRestrQual, ni, , 
    }

    fn exportTypeQualsAttrs(tyqs: TypeQuals, attrs: Attributes, ) -> [CTypeQual] {
        exportTypeQuals, tyqs, Dummy, map, CAttrQual, exportAttrs, attrs, , 
    }

    fn exportTypeSpec(tyname: TypeName, ) -> [CTypeSpec] {
        Dummy
    }

    fn exportVarDecl($$$: VarDecl, ) -> <[CDeclSpec], CDeclr> {
        exportDeclr
        exportDeclAttrs, attrs, 
        ty
        
        name
    }

    fn fromDirectType($$$: Type, ) -> TypeName {
        ty
    }

    fn fromDirectType($$$: Type, ) -> TypeName {
        maybe
        error, Dummy, 
        fromDirectType
        ref
    }

    fn fromDirectType(_: Type, ) -> TypeName {
        error
        Dummy
    }

    fn ni() -> NodeInfo {
        undefNode
    }

    fn threadLocal(False: Bool, ) -> [CStorageSpec] {
        id
    }

    fn threadLocal(True: Bool, ) -> [CStorageSpec] {
        CThread, ni, , Dummy, 
    }

}



mod "./language-c/src/Language/C/Analysis/NameSpaceMap.hs" {
    fn defGlobal($$$: NameSpaceMap k a, ident: k, def: a, ) -> <NameSpaceMap k a, Maybe a> {
        NsMap, Map.insert, ident, def, gs, , lss, Map.lookup, ident, gs, 
    }

    fn defLocal(ns: NameSpaceMap k a, $$$: k, $$$: a, ident: <NameSpaceMap k a, Maybe a>, ) -> <NameSpaceMap k a, Maybe a> {
        defGlobal
        ns
        ident
        def
    }

    fn defLocal($$$: NameSpaceMap k a, ident: k, def: a, ) -> <NameSpaceMap k a, Maybe a> {
        NsMap, gs, ident, def, , Dummy, ls, , Dummy, lss, , Prelude.lookup, ident, ls, 
    }

    fn enterNewScope($$$: NameSpaceMap k a, ) -> NameSpaceMap k a {
        NsMap
        gs
        , Dummy, lss, 
    }

    fn globalNames($$$: NameSpaceMap k v, ) -> Map k v {
        g
    }

    fn hasLocalNames($$$: NameSpaceMap k v, ) -> Bool {
        not
        null, l, 
    }

    fn leaveScope($$$: NameSpaceMap k a, ) -> <NameSpaceMap k a, [<k, a>]> {
        error
        Dummy
    }

    fn leaveScope($$$: NameSpaceMap k a, ) -> <NameSpaceMap k a, [<k, a>]> {
        NsMap, gs, lss, ls, 
    }

    fn localNames($$$: NameSpaceMap k v, ) -> [[<k, v>]] {
        l
    }

    fn lookupGlobal($$$: NameSpaceMap k a, ident: k, ) -> Maybe a {
        Map.lookup
        ident
        gs
    }

    fn lookupInnermostScope(nsm: NameSpaceMap k a, $$$: k, $$$: Maybe a, ) -> Maybe a {
        Dummy
    }

    fn lookupName(ns: NameSpaceMap k a, $$$: k, $$$: Maybe a, ) -> Maybe a {
        Dummy
    }

    fn mergeNameSpace($$$: NameSpaceMap k a, $$$: NameSpaceMap k a, ) -> NameSpaceMap k a {
        NsMap
        Map.union, global1, global2, 
        localUnion, local1, local2, 
    }

    fn nameSpaceMap() -> NameSpaceMap k v {
        NsMap
        Map.empty
        
    }

    fn nsMapToList($$$: NameSpaceMap k a, ) -> [<k, a>] {
        concat
        lss
        Dummy
        Map.toList
        gs
    }

}



mod "./language-c/src/Language/C/Analysis/SemError.hs" {
    fn badSpecifierError(node_info: NodeInfo, msg: String, ) -> BadSpecifierError {
        BadSpecifierError
        mkErrorInfo, LevelError, msg, node_info, 
    }

    fn invalidAST(node_info: NodeInfo, msg: String, ) -> InvalidASTError {
        InvalidAST
        mkErrorInfo, LevelError, msg, node_info, 
    }

    fn prevDeclMsg(old_node: NodeInfo, ) -> [String] {
        Dummy, show, posOfNode, old_node, , 
    }

    fn redefErrLabel($$$: RedefInfo, ) -> String {
        ident
        Dummy
        Dummy
    }

    fn redefErrReason($$$: RedefInfo, ) -> String {
        Dummy
        Dummy
        ident
    }

    fn redefErrReason($$$: RedefInfo, ) -> String {
        Dummy
        Dummy
        ident
        Dummy
        Dummy
    }

    fn redefErrReason($$$: RedefInfo, ) -> String {
        ident
        Dummy
        Dummy
    }

    fn redefErrReason($$$: RedefInfo, ) -> String {
        ident
        Dummy
        Dummy
    }

    fn redefErrReason($$$: RedefInfo, ) -> String {
        ident
        Dummy
        Dummy
    }

    fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, $$$: ErrorInfo, ) -> ErrorInfo {
        ErrorInfo
        lvl
        posOfNode, node, 
        redefErrReason, info, , Dummy, prevDeclMsg, old_node, 
    }

    fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo, ) -> RedefError {
        RedefError
        lvl
        RedefInfo, ctx, kind, new, old, 
    }

    fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    fn typeMismatchInfo($$$: TypeMismatch, ) -> ErrorInfo {
        ErrorInfo
        LevelError
        posOfNode, node1, 
        reason, 
    }

}



ERROR   - "./language-c/src/Language/C/Analysis/SemRep.hs"



ERROR   - "./language-c/src/Language/C/Analysis/TravMonad.hs"



ERROR   - "./language-c/src/Language/C/Analysis/TypeCheck.hs"



mod "./language-c/src/Language/C/Analysis/TypeConversions.hs" {
    fn arithmeticConversion($$$: TypeName, $$$: TypeName, ) -> Maybe TypeName {
        Just
        Dummy
        TyComplex
        Dummy
        floatConversion
        t1
        t2
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName, ) -> Maybe TypeName {
        Just
        Dummy
        TyComplex
        Dummy
        floatConversion
        t1
        t2
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName, ) -> Maybe TypeName {
        Just
        Dummy
        TyComplex
        Dummy
        floatConversion
        t1
        t2
    }

    fn arithmeticConversion(t1: TypeName, $$$: TypeName, $$$: Maybe TypeName, ) -> Maybe TypeName {
        Just
        t1
    }

    fn arithmeticConversion($$$: TypeName, t2: TypeName, $$$: Maybe TypeName, ) -> Maybe TypeName {
        Just
        t2
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName, ) -> Maybe TypeName {
        Just
        Dummy
        TyFloating
        Dummy
        floatConversion
        t1
        t2
    }

    fn arithmeticConversion(t1: TypeName, $$$: TypeName, $$$: Maybe TypeName, ) -> Maybe TypeName {
        Just
        t1
    }

    fn arithmeticConversion($$$: TypeName, t2: TypeName, $$$: Maybe TypeName, ) -> Maybe TypeName {
        Just
        t2
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName, ) -> Maybe TypeName {
        Just
        Dummy
        TyIntegral
        Dummy
        intConversion
        t1
        t2
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName, ) -> Maybe TypeName {
        Just
        Dummy
        TyIntegral
        TyInt
    }

    fn arithmeticConversion($$$: TypeName, t2: TypeName, ) -> Maybe TypeName {
        Just
        Dummy
        t2
    }

    fn arithmeticConversion(t1: TypeName, $$$: TypeName, ) -> Maybe TypeName {
        Just
        Dummy
        t1
    }

    fn arithmeticConversion(_: TypeName, _: TypeName, ) -> Maybe TypeName {
        Nothing
    }

    fn floatConversion() -> FloatType {
        max
    }

    fn intConversion(t1: IntType, t2: IntType, ) -> IntType {
        max
        TyInt
        max, t1, t2, 
    }

}



ERROR   - "./language-c/src/Language/C/Analysis/TypeUtils.hs"



mod "./language-c/src/Language/C/Analysis.hs" {
}



ERROR   - "./language-c/src/Language/C/Data/Error.hs"



mod "./language-c/src/Language/C/Data/Ident.hs" {
    fn bits14() -> Int {
        Dummy
        Dummy
        Dummy, Dummy, Int, 
    }

    fn bits21() -> Int {
        Dummy
        Dummy
        Dummy, Dummy, Int, 
    }

    fn bits28() -> Int {
        Dummy
        Dummy
        Dummy, Dummy, Int, 
    }

    fn bits7() -> Int {
        Dummy
        Dummy
        Dummy, Dummy, Int, 
    }

    fn builtinIdent(s: String, ) -> Ident {
        Ident
        s
        quad, s, 
        mkNodeInfoOnlyPos, builtinPos, 
    }

    fn dumpIdent(ide: Ident, ) -> String {
        identToString
        ide
        Dummy
        Dummy
        Dummy
        show
        nodeInfo, ide, 
    }

    fn identToString($$$: Ident, ) -> String {
        s
    }

    fn internalIdent(s: String, ) -> Ident {
        Ident
        s
        quad, s, 
        mkNodeInfoOnlyPos, internalPos, 
    }

    fn internalIdentAt(pos: Position, s: String, ) -> Ident {
        Ident
        s
        quad, s, 
        mkNodeInfoPosLen, pos, pos, length, s, , 
    }

    fn isAnonymousRef($$$: SUERef, ) -> Bool {
        True
    }

    fn isAnonymousRef(_: SUERef, ) -> Bool {
        False
    }

    fn isInternalIdent($$$: Ident, ) -> Bool {
        isInternalPos
        posOfNode, nodeinfo, 
    }

    fn mkIdent(pos: Position, s: String, name: Name, ) -> Ident {
        Ident
        s
        quad, s, 
        mkNodeInfo', pos, pos, length, s, , name, 
    }

    fn quad($$$: String, ) -> Int {
        ord, c4, Dummy, bits21, Dummy, ord, c3, Dummy, bits14, Dummy, ord, c2, Dummy, bits7, Dummy, ord, c1, , Dummy, bits28, 
        Dummy
        quad, s, Dummy, bits28, 
    }

    fn quad($$$: String, ) -> Int {
        ord
        c3
        Dummy
        bits14
        Dummy
        ord
        c2
        Dummy
        bits7
        Dummy
        ord
        c1
    }

    fn quad($$$: String, ) -> Int {
        ord
        c2
        Dummy
        bits7
        Dummy
        ord
        c1
    }

    fn quad($$$: String, ) -> Int {
        ord
        c1
    }

    fn quad($$$: String, ) -> Int {
        Dummy
    }

}



mod "./language-c/src/Language/C/Data/InputStream.hs" {
    fn countLines() -> Int {
        length
        Dummy
        BSC.lines
    }

    fn countLines() -> Int {
        length
        Dummy
        lines
    }

    fn inputStreamEmpty() -> Bool {
        BSW.null
    }

    fn inputStreamEmpty() -> Bool {
        null
    }

    fn inputStreamFromString() -> InputStream {
        BSC.pack
    }

    fn inputStreamFromString() -> InputStream {
        id
    }

    fn inputStreamToString() -> String {
        BSC.unpack
    }

    fn inputStreamToString() -> String {
        id
    }

    fn readInputStream() -> IO InputStream {
        BSW.readFile
    }

    fn readInputStream() -> IO InputStream {
        readFile
    }

    fn takeByte(bs: InputStream, ) -> <Word8, InputStream> {
        BSW.head
        bs
        Dummy
        BSW.head, bs, BSW.tail, bs, 
    }

    fn takeChar(bs: InputStream, ) -> <Char, InputStream> {
        BSC.head
        bs
        Dummy
        BSC.head, bs, BSC.tail, bs, 
    }

    fn takeChar(bs: InputStream, ) -> <Char, InputStream> {
        head, bs, tail, bs, 
    }

    fn takeChars($$$: Int, bstr: InputStream, ) -> [Char] {
        BSC.unpack
        Dummy
        BSC.take
        n
        bstr
    }

    fn takeChars(n: Int, str: InputStream, ) -> [Char] {
        take
        n
        str
    }

}



mod "./language-c/src/Language/C/Data/Name.hs" {
    fn namesStartingFrom(k: Int, ) -> [Name] {
        Name, k.., 
    }

    fn newNameSupply() -> [Name] {
        namesStartingFrom
        Dummy
    }

}



ERROR   - "./language-c/src/Language/C/Data/Node.hs"



mod "./language-c/src/Language/C/Data/Position.hs" {
    fn adjustPos(fname: FilePath, row: Int, $$$: Position, ) -> Position {
        Position
        offs
        fname
        row
        Dummy
    }

    fn adjustPos(_: FilePath, _: Int, p: Position, ) -> Position {
        p
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset($$$: Position, n: Int, ) -> Position {
        Position
        o, Dummy, n, 
        f
        r
        c
    }

    fn incOffset(p: Position, n: Int, ) -> Position {
        p
    }

    fn incPos($$$: Position, n: Int, ) -> Position {
        Position
        offs, Dummy, n, 
        fname
        row
        col, Dummy, n, 
    }

    fn incPos(p: Position, _: Int, ) -> Position {
        p
    }

    fn initPos(file: FilePath, ) -> Position {
        Position
        Dummy
        file
        Dummy
        Dummy
    }

    fn internalPos() -> Position {
        InternalPosition
    }

    fn isBuiltinPos(BuiltinPosition: Position, ) -> Bool {
        True
    }

    fn isBuiltinPos(_: Position, ) -> Bool {
        False
    }

    fn isInternalPos(InternalPosition: Position, ) -> Bool {
        True
    }

    fn isInternalPos(_: Position, ) -> Bool {
        False
    }

    fn isNoPos(NoPosition: Position, ) -> Bool {
        True
    }

    fn isNoPos(_: Position, ) -> Bool {
        False
    }

    fn isSourcePos($$$: Position, ) -> Bool {
        True
    }

    fn isSourcePos(_: Position, ) -> Bool {
        False
    }

    fn nopos() -> Position {
        NoPosition
    }

    fn position() -> Position {
        Position
    }

    fn retPos($$$: Position, ) -> Position {
        Position
        offs, Dummy, Dummy, 
        fname
        row, Dummy, Dummy, 
        Dummy
    }

    fn retPos(p: Position, ) -> Position {
        p
    }

}



ERROR   - "./language-c/src/Language/C/Data/RList.hs"



mod "./language-c/src/Language/C/Data.hs" {
}



mod "./language-c/src/Language/C/Parser/Builtin.hs" {
    fn builtinTypeNames() -> [Ident] {
        builtinIdent, Dummy, 
    }

}



ERROR   - "./language-c/src/Language/C/Parser/Lexer.x"



ERROR   - "./language-c/src/Language/C/Parser/Parser.y"



ERROR   - "./language-c/src/Language/C/Parser/ParserMonad.hs"



ERROR   - "./language-c/src/Language/C/Parser/Tokens.hs"



mod "./language-c/src/Language/C/Parser.hs" {
    fn execParser_(parser: P a, input: InputStream, pos: Position, ) -> Either ParseError a {
        fmap
        fst
        Dummy
        execParser
        parser
        input
        pos
        builtinTypeNames
        newNameSupply
    }

}



ERROR   - "./language-c/src/Language/C/Pretty.hs"



ERROR   - "./language-c/src/Language/C/Syntax/AST.hs"



mod "./language-c/src/Language/C/Syntax/Constants.hs" {
    fn _showWideFlag(flag: Bool, ) -> ShowS {
        if
        flag
        then
        showString
        Dummy
        else
        id
    }

    fn cChar(c: Char, ) -> CChar {
        CChar
        c
        False
    }

    fn cChar_w(c: Char, ) -> CChar {
        CChar
        c
        True
    }

    fn cChars() -> CChar {
        CChars
    }

    fn cFloat() -> CFloat {
        CFloat
        Dummy
        show
    }

    fn cInteger(i: Integer, ) -> CInteger {
        CInteger
        i
        DecRepr
        noFlags
    }

    fn cString(str: String, ) -> CString {
        CString
        str
        False
    }

    fn cString_w(str: String, ) -> CString {
        CString
        str
        True
    }

    fn clearFlag(flag: f, $$$: Flags f, ) -> Flags f {
        Flags
        Dummy
        k
        Dummy
        fromEnum
        flag
    }

    fn concatCStrings(cs: [CString], ) -> CString {
        CString
        concatMap, getCString, cs, 
        any, isWideString, cs, 
    }

    fn dQuote(s: String, t: ShowS, ) -> ShowS {
        Dummy, Dummy, s, 
        Dummy
        Dummy
        Dummy
        t
    }

    fn escapeCChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn escapeChar($$$: Char, ) -> String {
        Dummy
    }

    fn getCChar($$$: CChar, ) -> [Char] {
        c, 
    }

    fn getCChar($$$: CChar, ) -> [Char] {
        cs
    }

    fn getCCharAsInt($$$: CChar, ) -> Integer {
        fromIntegral
        fromEnum, c, 
    }

    fn getCCharAsInt($$$: CChar, ) -> Integer {
        error
        Dummy
    }

    fn getCInteger($$$: CInteger, ) -> Integer {
        i
    }

    fn getCString($$$: CString, ) -> String {
        str
    }

    fn head'(err: String, $$$: [a], ) -> a {
        error
        err
    }

    fn head'(_: String, $$$: [a], ) -> a {
        x
    }

    fn isAsciiSourceChar(c: Char, ) -> Bool {
        isAscii
        c
        Dummy
        isPrint
        c
    }

    fn isCChar($$$: Char, ) -> Bool {
        False
    }

    fn isCChar($$$: Char, ) -> Bool {
        False
    }

    fn isCChar($$$: Char, ) -> Bool {
        False
    }

    fn isCChar(c: Char, ) -> Bool {
        isAsciiSourceChar
        c
    }

    fn isSChar($$$: Char, ) -> Bool {
        False
    }

    fn isSChar($$$: Char, ) -> Bool {
        False
    }

    fn isSChar($$$: Char, ) -> Bool {
        False
    }

    fn isSChar(c: Char, ) -> Bool {
        isAsciiSourceChar
        c
    }

    fn isWideChar($$$: CChar, ) -> Bool {
        wideFlag
    }

    fn isWideChar($$$: CChar, ) -> Bool {
        wideFlag
    }

    fn isWideString($$$: CString, ) -> Bool {
        wideflag
    }

    fn noFlags() -> Flags f {
        Flags
        Dummy
    }

    fn readCFloat() -> CFloat {
        CFloat
    }

    fn readCInteger(repr: CIntRepr, str: String, ) -> Either String CInteger {
        Dummy
    }

    fn sQuote(s: String, t: ShowS, ) -> ShowS {
        Dummy
        Dummy
        s
        Dummy
        Dummy
        Dummy
        t
    }

    fn setFlag(flag: f, $$$: Flags f, ) -> Flags f {
        Flags
        Dummy
        k
        Dummy
        fromEnum
        flag
    }

    fn showCharConst(c: Char, ) -> ShowS {
        sQuote
        Dummy
        escapeCChar
        c
    }

    fn showStringLit() -> ShowS {
        dQuote
        Dummy
        concatMap
        showStringChar
    }

    fn testFlag(flag: f, $$$: Flags f, ) -> Bool {
        k
        Dummy
        fromEnum
        flag
    }

    fn unescapeChar($$$: String, ) -> <Char, String> {
        Dummy
    }

    fn unescapeChar($$$: String, ) -> <Char, String> {
        c, cs, 
    }

    fn unescapeChar($$$: String, ) -> <Char, String> {
        error
        Dummy
        Dummy
    }

    fn unescapeString($$$: String, ) -> String {
        
    }

    fn unescapeString(cs: String, ) -> String {
        Dummy
    }

}



mod "./language-c/src/Language/C/Syntax/Ops.hs" {
    fn assignBinop(CAssignOp: CAssignOp, ) -> CBinaryOp {
        error
        Dummy
    }

    fn assignBinop(CMulAssOp: CAssignOp, ) -> CBinaryOp {
        CMulOp
    }

    fn assignBinop(CDivAssOp: CAssignOp, ) -> CBinaryOp {
        CDivOp
    }

    fn assignBinop(CRmdAssOp: CAssignOp, ) -> CBinaryOp {
        CRmdOp
    }

    fn assignBinop(CAddAssOp: CAssignOp, ) -> CBinaryOp {
        CAddOp
    }

    fn assignBinop(CSubAssOp: CAssignOp, ) -> CBinaryOp {
        CSubOp
    }

    fn assignBinop(CShlAssOp: CAssignOp, ) -> CBinaryOp {
        CShlOp
    }

    fn assignBinop(CShrAssOp: CAssignOp, ) -> CBinaryOp {
        CShrOp
    }

    fn assignBinop(CAndAssOp: CAssignOp, ) -> CBinaryOp {
        CAndOp
    }

    fn assignBinop(CXorAssOp: CAssignOp, ) -> CBinaryOp {
        CXorOp
    }

    fn assignBinop(COrAssOp: CAssignOp, ) -> CBinaryOp {
        COrOp
    }

    fn isBitOp(op: CBinaryOp, ) -> Bool {
        op
        Dummy
        CShlOp, CShrOp, CAndOp, COrOp, CXorOp, 
    }

    fn isCmpOp(op: CBinaryOp, ) -> Bool {
        op
        Dummy
        CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp, 
    }

    fn isEffectfulOp(op: CUnaryOp, ) -> Bool {
        op
        Dummy
        CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp, 
    }

    fn isLogicOp(op: CBinaryOp, ) -> Bool {
        op
        Dummy
        CLndOp, CLorOp, 
    }

    fn isPtrOp(op: CBinaryOp, ) -> Bool {
        op
        Dummy
        CAddOp, CSubOp, 
    }

}



ERROR   - "./language-c/src/Language/C/Syntax/Utils.hs"



mod "./language-c/src/Language/C/Syntax.hs" {
}



mod "./language-c/src/Language/C/System/GCC.hs" {
    fn buildCppArgs($$$: CppArgs, ) -> [String] {
        { concatMap, tOption, options, , ;  }
        Dummy
        outputFileOpt
        Dummy
        Dummy, input_file, 
        Dummy
        extra_args
    }

    fn gccParseCPPArgs(args: [String], ) -> Either String <CppArgs, [String]> {
        Dummy
    }

    fn newGCC() -> GCC {
        GCC
    }

}



mod "./language-c/src/Language/C/System/Preprocess.hs" {
    fn addCppOption(cpp_args: CppArgs, opt: CppOption, ) -> CppArgs {
        cpp_args
        Dummy
    }

    fn addExtraOption(cpp_args: CppArgs, extra: String, ) -> CppArgs {
        cpp_args
        Dummy
    }

    fn cppFile(input_file: FilePath, ) -> CppArgs {
        CppArgs
        Dummy
    }

    fn isPreprocessed() -> Bool {
        Dummy, Dummy, 
    }

    fn mkOutputFile(tmp_dir_opt: Maybe FilePath, input_file: FilePath, ) -> IO FilePath {
        { tmpDir, Dummy, getTempDir, tmp_dir_opt, ; mkTmpFile, tmpDir, getOutputFileName, input_file, , ;  }
    }

    fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath, ) -> IO FilePath {
        { path, file_handle, , Dummy, openTempFile, tmp_dir, file_templ, ; hClose, file_handle, ; return, path, ;  }
    }

    fn preprocessedExt() -> String {
        Dummy
    }

    fn rawCppArgs(opts: [String], input_file: FilePath, ) -> CppArgs {
        CppArgs
        Dummy
    }

    fn runPreprocessor(cpp: cpp, cpp_args: CppArgs, ) -> IO <Either ExitCode InputStream> {
        { {
        let getActualOutFile = maybe, mkOutputFile, cppTmpDir, cpp_args, , inputFile, cpp_args, , , return, outputFile, cpp_args, , 
        let invokeCpp = { exit_code, Dummy, runCPP, cpp, cpp_args, Dummy, , ; Dummy, ;  }, 
        let removeTmpOutFile = maybe, removeFile, out_file, , Dummy, , , outputFile, cpp_args, , 
    }
bracket, getActualOutFile, removeTmpOutFile, invokeCpp, ;  }
    }

}



