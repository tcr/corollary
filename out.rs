// ERROR: can't output "./language-c/src/Language/C/Analysis/AstAnalysis.hs"

mod Language_C_Analysis_AstAnalysis {

}

mod Language_C_Analysis_Builtins {
    fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}

// ERROR: can't output "./language-c/src/Language/C/Analysis/ConstEval.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/Debug.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/DeclAnalysis.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/DefTable.hs"

mod Language_C_Analysis_Export {
    fn exportArraySize($$$: ArraySize) -> CArrSize {
        CArrSize(static, e)
    }

    fn exportArraySize($$$: ArraySize) -> CArrSize {
        CNoArrSize(complete)
    }

    fn exportAttrs() -> Vec<CAttr> {
        map(exportAttr)
    }

    fn exportCompType($$$: CompType) -> Vec<CTypeSpec> {
        vec![CSUType(comp, ni)]
    }

    fn exportCompTypeDecl(ty: CompTypeRef) -> Vec<CTypeSpec> {
        vec![CSUType((exportComp(ty)), ni)]
    }

    fn exportCompTypeRef($$$: CompType) -> Vec<CTypeSpec> {
        exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
    }

    fn exportComplexType(ty: FloatType) -> Vec<CTypeSpec> {
        :((CComplexType(ni)), exportFloatType(ty))
    }

    fn exportDeclAttrs($$$: DeclAttrs) -> Vec<CDeclSpec> {
        ++((if(inline, then, vec![CTypeQual((CInlineQual(ni)))], else, vec![])), ++(map((CStorageSpec), (exportStorage(storage))), map(((CTypeQual . CAttrQual)), (exportAttrs(attrs)))))
    }

    fn exportDeclr(other_specs: Vec<CDeclSpec>, ty: Type, attrs: Attributes, name: VarName) -> (Vec<CDeclSpec>, CDeclr) {
        (++(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
    }

    fn exportEnumType($$$: EnumType) -> Vec<CTypeSpec> {
        vec![CEnumType(enum, ni)]
    }

    fn exportEnumTypeDecl(ty: EnumTypeRef) -> Vec<CTypeSpec> {
        vec![CEnumType((exportEnum(ty)), ni)]
    }

    fn exportEnumTypeRef($$$: EnumType) -> Vec<CTypeSpec> {
        exportEnumTypeDecl((EnumTypeRef(sue_ref, node_info)))
    }

    fn exportFloatType(ty: FloatType) -> Vec<CTypeSpec> {
        match ty {
            TyFloat => vec![CFloatType(ni)],
            TyDouble => vec![CDoubleType(ni)],
            TyLDouble => vec![CLongType(ni), CDoubleType(ni)],
        }
    }

    fn exportIntType(ty: IntType) -> Vec<CTypeSpec> {
        match ty {
            TyBool => vec![CBoolType(ni)],
            TyChar => vec![CCharType(ni)],
            TySChar => vec![CSignedType(ni), CCharType(ni)],
            TyUChar => vec![CUnsigType(ni), CCharType(ni)],
            TyShort => vec![CShortType(ni)],
            TyUShort => vec![CUnsigType(ni), CShortType(ni)],
            TyInt => vec![CIntType(ni)],
            TyUInt => vec![CUnsigType(ni), CIntType(ni)],
            TyLong => vec![CLongType(ni)],
            TyULong => vec![CUnsigType(ni), CLongType(ni)],
            TyLLong => vec![CLongType(ni), CLongType(ni)],
            TyULLong => vec![CUnsigType(ni), CLongType(ni), CLongType(ni)],
        }
    }

    fn exportMemberDecl($$$: MemberDecl) -> CDecl {
        CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info)
    }

    fn exportMemberDecl($$$: MemberDecl) -> CDecl {
        Let(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info)
    }

    fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
        Let(in, CDecl, specs, vec![(Just(declr), Nothing, Nothing)], (nodeInfo(paramdecl)))
    }

    fn exportSUERef() -> Maybe<Ident> {
        (Just . (internalIdent . show))
    }

    fn exportStorage(NoStorage: Storage) -> Vec<CStorageSpec> {
        vec![]
    }

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        if(reg, then, vec![CRegister(ni)], else, vec![])
    }

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        threadLocal(thread_local, vec![CStatic(ni)])
    }

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        threadLocal(thread_local, vec![CExtern(ni)])
    }

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        error("impossible storage: static without linkage".to_string())
    }

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        vec![CStatic(ni)]
    }

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        vec![]
    }

    fn exportStorage($$$: Storage) -> Vec<CStorageSpec> {
        error("impossible storage: function without linkage".to_string())
    }

    fn exportType(ty: Type) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {
        exportTy(vec![], ty)
    }

    fn exportTypeDecl(ty: Type) -> CDecl {
        CDecl(declspecs, declrs, ni)
    }

    fn exportTypeDef($$$: TypeDef) -> CDecl {
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
            TyVoid => vec![CVoidType(ni)],
            TyIntegral ity => exportIntType(ity),
            TyFloating fty => exportFloatType(fty),
            TyComplex fty => exportComplexType(fty),
            TyComp comp => exportCompTypeDecl(comp),
            TyEnum enum => exportEnumTypeDecl(enum),
            TyBuiltin TyVaList => vec![CTypeDef((internalIdent("va_list".to_string())), ni)],
            TyBuiltin TyAny => vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)],
        }
    }

    fn exportVarDecl($$$: VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    }

    fn fromDirectType($$$: Type) -> TypeName {
        ty
    }

    fn fromDirectType($$$: Type) -> TypeName {
        maybe((error("undefined typeDef".to_string())), fromDirectType, ref)
    }

    fn fromDirectType(_: Type) -> TypeName {
        error("fromDirectType".to_string())
    }

    fn ni() -> NodeInfo {
        undefNode
    }

    fn threadLocal(False: Bool) -> Vec<CStorageSpec> {
        id
    }

    fn threadLocal(True: Bool) -> Vec<CStorageSpec> {
        ((CThread(ni))(Operator(":")))
    }

}

mod Language_C_Analysis_NameSpaceMap {
    fn defGlobal($$$: NameSpaceMap<k, a>, ident: k, def: a) -> (NameSpaceMap<k, a>, Maybe<a>) {
        (NsMap((Map.insert(ident, def, gs)), lss), Map.lookup(ident, gs))
    }

    fn defLocal(ns: NameSpaceMap<k, a>, $$$: k, $$$: a, ident: (NameSpaceMap<k, a>, Maybe<a>)) -> (NameSpaceMap<k, a>, Maybe<a>) {
        defGlobal(ns, ident, def)
    }

    fn defLocal($$$: NameSpaceMap<k, a>, ident: k, def: a) -> (NameSpaceMap<k, a>, Maybe<a>) {
        (NsMap(gs, (:((:((ident, def), ls)), lss))), Prelude.lookup(ident, ls))
    }

    fn enterNewScope($$$: NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
        NsMap(gs, (:(vec![], lss)))
    }

    fn globalNames($$$: NameSpaceMap<k, v>) -> Map<k, v> {
        g
    }

    fn hasLocalNames($$$: NameSpaceMap<k, v>) -> Bool {
        not((null(l)))
    }

    fn leaveScope($$$: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
        error("NsMaps.leaveScope: No local scope!".to_string())
    }

    fn leaveScope($$$: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
        (NsMap(gs, lss), ls)
    }

    fn localNames($$$: NameSpaceMap<k, v>) -> Vec<Vec<(k, v)>> {
        l
    }

    fn lookupGlobal($$$: NameSpaceMap<k, a>, ident: k) -> Maybe<a> {
        Map.lookup(ident, gs)
    }

    fn lookupInnermostScope(nsm: NameSpaceMap<k, a>, $$$: k, $$$: Maybe<a>) -> Maybe<a> {
        match localDefs {
            $$$ => Prelude.lookup(ident, ls),
            $$$ => lookupGlobal(nsm, ident),
        }
    }

    fn lookupName(ns: NameSpaceMap<k, a>, $$$: k, $$$: Maybe<a>) -> Maybe<a> {
        match (lookupLocal(localDefs)) {
            Nothing => lookupGlobal(ns, ident),
            Just def => Just(def),
        }
    }

    fn mergeNameSpace($$$: NameSpaceMap<k, a>, $$$: NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
        NsMap((Map.union(global1, global2)), (localUnion(local1, local2)))
    }

    fn nameSpaceMap() -> NameSpaceMap<k, v> {
        NsMap(Map.empty, vec![])
    }

    fn nsMapToList($$$: NameSpaceMap<k, a>) -> Vec<(k, a)> {
        ++(concat(lss), Map.toList(gs))
    }

}

mod Language_C_Analysis_SemError {
    fn badSpecifierError(node_info: NodeInfo, msg: String) -> BadSpecifierError {
        BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn invalidAST(node_info: NodeInfo, msg: String) -> InvalidASTError {
        InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn prevDeclMsg(old_node: NodeInfo) -> Vec<String> {
        vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
    }

    fn redefErrLabel($$$: RedefInfo) -> String {
        ++(ident, " redefined".to_string())
    }

    fn redefErrReason($$$: RedefInfo) -> String {
        ++("duplicate definition of ".to_string(), ident)
    }

    fn redefErrReason($$$: RedefInfo) -> String {
        ++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string()))
    }

    fn redefErrReason($$$: RedefInfo) -> String {
        ++(ident, " previously declared as a different kind of symbol".to_string())
    }

    fn redefErrReason($$$: RedefInfo) -> String {
        ++(ident, " previously declared with different linkage".to_string())
    }

    fn redefErrReason($$$: RedefInfo) -> String {
        ++(ident, " previously declared without linkage".to_string())
    }

    fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, $$$: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (++(vec![redefErrReason(info)], prevDeclMsg(old_node))))
    }

    fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
        RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
    }

    fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    fn typeMismatchInfo($$$: TypeMismatch) -> ErrorInfo {
        ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
    }

}

// ERROR: can't output "./language-c/src/Language/C/Analysis/SemRep.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/TravMonad.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/TypeCheck.hs"

mod Language_C_Analysis_TypeConversions {
    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(TyComplex(floatConversion(t1, t2)))
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(TyComplex(floatConversion(t1, t2)))
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(TyComplex(floatConversion(t1, t2)))
    }

    fn arithmeticConversion(t1: TypeName, $$$: TypeName, $$$: Maybe<TypeName>) -> Maybe<TypeName> {
        Just(t1)
    }

    fn arithmeticConversion($$$: TypeName, t2: TypeName, $$$: Maybe<TypeName>) -> Maybe<TypeName> {
        Just(t2)
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(TyFloating(floatConversion(t1, t2)))
    }

    fn arithmeticConversion(t1: TypeName, $$$: TypeName, $$$: Maybe<TypeName>) -> Maybe<TypeName> {
        Just(t1)
    }

    fn arithmeticConversion($$$: TypeName, t2: TypeName, $$$: Maybe<TypeName>) -> Maybe<TypeName> {
        Just(t2)
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(TyIntegral(intConversion(t1, t2)))
    }

    fn arithmeticConversion($$$: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(TyIntegral(TyInt))
    }

    fn arithmeticConversion($$$: TypeName, t2: TypeName) -> Maybe<TypeName> {
        Just(t2)
    }

    fn arithmeticConversion(t1: TypeName, $$$: TypeName) -> Maybe<TypeName> {
        Just(t1)
    }

    fn arithmeticConversion(_: TypeName, _: TypeName) -> Maybe<TypeName> {
        Nothing
    }

    fn floatConversion() -> FloatType {
        max
    }

    fn intConversion(t1: IntType, t2: IntType) -> IntType {
        max(TyInt, (max(t1, t2)))
    }

}

// ERROR: can't output "./language-c/src/Language/C/Analysis/TypeUtils.hs"

mod Language_C_Analysis {

}

// ERROR: can't output "./language-c/src/Language/C/Data/Error.hs"

mod Language_C_Data_Ident {
    fn bits14() -> Int {
        ^(2, (::(14, Int)))
    }

    fn bits21() -> Int {
        ^(2, (::(21, Int)))
    }

    fn bits28() -> Int {
        ^(2, (::(28, Int)))
    }

    fn bits7() -> Int {
        ^(2, (::(7, Int)))
    }

    fn builtinIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
    }

    fn dumpIdent(ide: Ident) -> String {
        ++(identToString(ide), ++(" at ".to_string(), show((nodeInfo(ide)))))
    }

    fn identToString($$$: Ident) -> String {
        s
    }

    fn internalIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
    }

    fn internalIdentAt(pos: Position, s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
    }

    fn isAnonymousRef($$$: SUERef) -> Bool {
        True
    }

    fn isAnonymousRef(_: SUERef) -> Bool {
        False
    }

    fn isInternalIdent($$$: Ident) -> Bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo'(pos, (pos, length(s)), name)))
    }

    fn quad($$$: String) -> Int {
        +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28)))
    }

    fn quad($$$: String) -> Int {
        *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))
    }

    fn quad($$$: String) -> Int {
        *(ord(c2), +(bits7, ord(c1)))
    }

    fn quad($$$: String) -> Int {
        ord(c1)
    }

    fn quad($$$: String) -> Int {
        0
    }

}

mod Language_C_Data_InputStream {
    fn countLines() -> Int {
        (length . BSC.lines)
    }

    fn countLines() -> Int {
        (length . lines)
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

    fn readInputStream() -> IO<InputStream> {
        BSW.readFile
    }

    fn readInputStream() -> IO<InputStream> {
        readFile
    }

    fn takeByte(bs: InputStream) -> (Word8, InputStream) {
        seq(BSW.head(bs), (BSW.head(bs), BSW.tail(bs)))
    }

    fn takeChar(bs: InputStream) -> (Char, InputStream) {
        seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs)))
    }

    fn takeChar(bs: InputStream) -> (Char, InputStream) {
        (head(bs), tail(bs))
    }

    fn takeChars($$$: Int, bstr: InputStream) -> Vec<Char> {
        BSC.unpack(BSC.take(n, bstr))
    }

    fn takeChars(n: Int, str: InputStream) -> Vec<Char> {
        take(n, str)
    }

}

mod Language_C_Data_Name {
    fn namesStartingFrom(k: Int) -> Vec<Name> {
        vec![Name(k..)]
    }

    fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    }

}

// ERROR: can't output "./language-c/src/Language/C/Data/Node.hs"

mod Language_C_Data_Position {
    fn adjustPos(fname: FilePath, row: Int, $$$: Position) -> Position {
        Position(offs, fname, row, 1)
    }

    fn adjustPos(_: FilePath, _: Int, p: Position) -> Position {
        p
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset($$$: Position, n: Int) -> Position {
        Position((+(o, n)), f, r, c)
    }

    fn incOffset(p: Position, n: Int) -> Position {
        p
    }

    fn incPos($$$: Position, n: Int) -> Position {
        Position((+(offs, n)), fname, row, (+(col, n)))
    }

    fn incPos(p: Position, _: Int) -> Position {
        p
    }

    fn initPos(file: FilePath) -> Position {
        Position(0, file, 1, 1)
    }

    fn internalPos() -> Position {
        InternalPosition
    }

    fn isBuiltinPos(BuiltinPosition: Position) -> Bool {
        True
    }

    fn isBuiltinPos(_: Position) -> Bool {
        False
    }

    fn isInternalPos(InternalPosition: Position) -> Bool {
        True
    }

    fn isInternalPos(_: Position) -> Bool {
        False
    }

    fn isNoPos(NoPosition: Position) -> Bool {
        True
    }

    fn isNoPos(_: Position) -> Bool {
        False
    }

    fn isSourcePos($$$: Position) -> Bool {
        True
    }

    fn isSourcePos(_: Position) -> Bool {
        False
    }

    fn nopos() -> Position {
        NoPosition
    }

    fn position() -> Position {
        Position
    }

    fn retPos($$$: Position) -> Position {
        Position((+(offs, 1)), fname, (+(row, 1)), 1)
    }

    fn retPos(p: Position) -> Position {
        p
    }

}

// ERROR: can't output "./language-c/src/Language/C/Data/RList.hs"

mod Language_C_Data {

}

mod Language_C_Parser_Builtin {
    fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}

// ERROR: can't output "./language-c/src/Language/C/Parser/Lexer.x"

// ERROR: can't output "./language-c/src/Language/C/Parser/Parser.y"

// ERROR: can't output "./language-c/src/Language/C/Parser/ParserMonad.hs"

// ERROR: can't output "./language-c/src/Language/C/Parser/Tokens.hs"

mod Language_C_Parser {
    fn execParser_(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}

// ERROR: can't output "./language-c/src/Language/C/Pretty.hs"

// ERROR: can't output "./language-c/src/Language/C/Syntax/AST.hs"

mod Language_C_Syntax_Constants {
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

    fn clearFlag(flag: f, $$$: Flags<f>) -> Flags<f> {
        Flags(clearBit(k, fromEnum(flag)))
    }

    fn concatCStrings(cs: Vec<CString>) -> CString {
        CString((concatMap(getCString, cs)), (any(isWideString, cs)))
    }

    fn dQuote(s: String, t: ShowS) -> ShowS {
        ++((:("\"\"".to_string(), s)), ++("\\\"".to_string(), t))
    }

    fn escapeCChar($$$: Char) -> String {
        "\\\\\'".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\\\\\".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\a".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\b".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\e".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\f".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\n".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\r".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\t".to_string()
    }

    fn escapeChar($$$: Char) -> String {
        "\\\\v".to_string()
    }

    fn getCChar($$$: CChar) -> Vec<Char> {
        vec![c]
    }

    fn getCChar($$$: CChar) -> Vec<Char> {
        cs
    }

    fn getCCharAsInt($$$: CChar) -> Integer {
        fromIntegral((fromEnum(c)))
    }

    fn getCCharAsInt($$$: CChar) -> Integer {
        error("integer value of multi-character character constants is implementation defined".to_string())
    }

    fn getCInteger($$$: CInteger) -> Integer {
        i
    }

    fn getCString($$$: CString) -> String {
        str
    }

    fn head'(err: String, $$$: Vec<a>) -> a {
        error(err)
    }

    fn head'(_: String, $$$: Vec<a>) -> a {
        x
    }

    fn isAsciiSourceChar(c: Char) -> Bool {
        &&(isAscii(c), isPrint(c))
    }

    fn isCChar($$$: Char) -> Bool {
        False
    }

    fn isCChar($$$: Char) -> Bool {
        False
    }

    fn isCChar($$$: Char) -> Bool {
        False
    }

    fn isCChar(c: Char) -> Bool {
        isAsciiSourceChar(c)
    }

    fn isSChar($$$: Char) -> Bool {
        False
    }

    fn isSChar($$$: Char) -> Bool {
        False
    }

    fn isSChar($$$: Char) -> Bool {
        False
    }

    fn isSChar(c: Char) -> Bool {
        isAsciiSourceChar(c)
    }

    fn isWideChar($$$: CChar) -> Bool {
        wideFlag
    }

    fn isWideChar($$$: CChar) -> Bool {
        wideFlag
    }

    fn isWideString($$$: CString) -> Bool {
        wideflag
    }

    fn noFlags() -> Flags<f> {
        Flags(0)
    }

    fn readCFloat() -> CFloat {
        CFloat
    }

    fn readCInteger(repr: CIntRepr, str: String) -> Either<String, CInteger> {
        match readNum(str) {
            $$$ => mkCInt(n, suffix),
            parseFailed => Left(++("Bad Integer literal: ".to_string(), show(parseFailed))),
        }
    }

    fn sQuote(s: String, t: ShowS) -> ShowS {
        ++("\'".to_string(), ++(s, ++("\'".to_string(), t)))
    }

    fn setFlag(flag: f, $$$: Flags<f>) -> Flags<f> {
        Flags(setBit(k, fromEnum(flag)))
    }

    fn showCharConst(c: Char) -> ShowS {
        sQuote(escapeCChar(c))
    }

    fn showStringLit() -> ShowS {
        (dQuote . concatMap(showStringChar))
    }

    fn testFlag(flag: f, $$$: Flags<f>) -> Bool {
        testBit(k, fromEnum(flag))
    }

    fn unescapeChar($$$: String) -> (Char, String) {
        match c {
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => ("\"\"".to_string(), cs),
            $$$ => match head'("bad escape sequence".to_string(), (readHex(cs))) {
                $$$ => (toEnum(i), cs'),
            },
            _ => match head'("bad escape sequence".to_string(), (readOct((:(c, cs))))) {
                $$$ => (toEnum(i), cs'),
            },
        }
    }

    fn unescapeChar($$$: String) -> (Char, String) {
        (c, cs)
    }

    fn unescapeChar($$$: String) -> (Char, String) {
        error("unescape char: empty string".to_string())
    }

    fn unescapeString($$$: String) -> String {
        vec![]
    }

    fn unescapeString(cs: String) -> String {
        match unescapeChar(cs) {
            $$$ => :(c, unescapeString(cs')),
        }
    }

}

mod Language_C_Syntax_Ops {
    fn assignBinop(CAssignOp: CAssignOp) -> CBinaryOp {
        error("direct assignment has no binary operator".to_string())
    }

    fn assignBinop(CMulAssOp: CAssignOp) -> CBinaryOp {
        CMulOp
    }

    fn assignBinop(CDivAssOp: CAssignOp) -> CBinaryOp {
        CDivOp
    }

    fn assignBinop(CRmdAssOp: CAssignOp) -> CBinaryOp {
        CRmdOp
    }

    fn assignBinop(CAddAssOp: CAssignOp) -> CBinaryOp {
        CAddOp
    }

    fn assignBinop(CSubAssOp: CAssignOp) -> CBinaryOp {
        CSubOp
    }

    fn assignBinop(CShlAssOp: CAssignOp) -> CBinaryOp {
        CShlOp
    }

    fn assignBinop(CShrAssOp: CAssignOp) -> CBinaryOp {
        CShrOp
    }

    fn assignBinop(CAndAssOp: CAssignOp) -> CBinaryOp {
        CAndOp
    }

    fn assignBinop(CXorAssOp: CAssignOp) -> CBinaryOp {
        CXorOp
    }

    fn assignBinop(COrAssOp: CAssignOp) -> CBinaryOp {
        COrOp
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

// ERROR: can't output "./language-c/src/Language/C/Syntax/Utils.hs"

mod Language_C_Syntax {

}

mod Language_C_System_GCC {
    fn buildCppArgs($$$: CppArgs) -> Vec<String> {
        ++({
                (concatMap(tOption, options))
            }, ++(outputFileOpt, ++(vec!["-E".to_string(), input_file], extra_args)))
    }

    fn gccParseCPPArgs(args: Vec<String>) -> Either<String, (CppArgs, Vec<String>)> {
        match mungeArgs(((Nothing, Nothing, RList.empty), (RList.empty, RList.empty)), args) {
            Left err => Left(err),
            Right $$$ => Left("No .c / .hc / .h source file given".to_string()),
            Right $$$ => Right(((rawCppArgs((RList.reverse(extra_args)), input_file))(hashmap! {
                        "outputFile" => output_file_opt,
                        "cppOptions" => RList.reverse(cpp_opts)
                    }), RList.reverse(other_args))),
        }
    }

    fn newGCC() -> GCC {
        GCC
    }

}

mod Language_C_System_Preprocess {
    fn addCppOption(cpp_args: CppArgs, opt: CppOption) -> CppArgs {
        cpp_args(hashmap! {
                "cppOptions" => :(opt, (cppOptions(cpp_args)))
            })
    }

    fn addExtraOption(cpp_args: CppArgs, extra: String) -> CppArgs {
        cpp_args(hashmap! {
                "extraOptions" => :(extra, (extraOptions(cpp_args)))
            })
    }

    fn cppFile(input_file: FilePath) -> CppArgs {
        CppArgs(hashmap! {
                "cppOptions" => vec![],
                "extraOptions" => vec![],
                "cppTmpDir" => Nothing,
                "inputFile" => input_file,
                "outputFile" => Nothing
            })
    }

    fn isPreprocessed() -> Bool {
        (".i".to_string()(Operator("isSuffixOf")))
    }

    fn mkOutputFile(tmp_dir_opt: Maybe<FilePath>, input_file: FilePath) -> IO<FilePath> {
        {
            let tmpDir = getTempDir(tmp_dir_opt);
            mkTmpFile(tmpDir, (getOutputFileName(input_file)))
        }
    }

    fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> IO<FilePath> {
        {
            let (path, file_handle) = openTempFile(tmp_dir, file_templ);
            hClose(file_handle);
            return(path)
        }
    }

    fn preprocessedExt() -> String {
        ".i".to_string()
    }

    fn rawCppArgs(opts: Vec<String>, input_file: FilePath) -> CppArgs {
        CppArgs(hashmap! {
                "inputFile" => input_file,
                "cppOptions" => vec![],
                "extraOptions" => opts,
                "outputFile" => Nothing,
                "cppTmpDir" => Nothing
            })
    }

    fn runPreprocessor(cpp: cpp, cpp_args: CppArgs) -> IO<Either<ExitCode, InputStream>> {
        {
            fn getActualOutFile() -> IO<FilePath> {
                maybe((mkOutputFile((cppTmpDir(cpp_args)), (inputFile(cpp_args)))), return, (outputFile(cpp_args)))
            }

            let invokeCpp = |actual_out_file| {
                {
                    let exit_code = runCPP(cpp, (cpp_args(hashmap! {
                                        "outputFile" => Just(actual_out_file)
                                    })));
                    match exit_code {
                            ExitSuccess => liftM(Right, (readInputStream(actual_out_file))),
                            ExitFailure _ => return(Left(exit_code)),
                        }
                }
            };

            let removeTmpOutFile = |out_file| {
                maybe((removeFile(out_file)), (Lambda(())), (outputFile(cpp_args)))
            };

            bracket(getActualOutFile, removeTmpOutFile, invokeCpp)
        }
    }

}

