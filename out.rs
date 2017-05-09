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
    fn exportArraySize(__0: ArraySize) -> CArrSize {
        match (__0) {
            $$$ => CArrSize(static, e),
            $$$ => CNoArrSize(complete),
        }
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

    fn exportMemberDecl(__0: MemberDecl) -> CDecl {
        match (__0) {
            $$$ => CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info),
            $$$ => Let(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info),
        }
    }

    fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
        Let(in, CDecl, specs, vec![(Just(declr), Nothing, Nothing)], (nodeInfo(paramdecl)))
    }

    fn exportSUERef() -> Maybe<Ident> {
        (Just . (internalIdent . show))
    }

    fn exportStorage(__0: Storage) -> Vec<CStorageSpec> {
        match (__0) {
            NoStorage => vec![],
            $$$ => if(reg, then, vec![CRegister(ni)], else, vec![]),
            $$$ => threadLocal(thread_local, vec![CStatic(ni)]),
            $$$ => threadLocal(thread_local, vec![CExtern(ni)]),
            $$$ => error("impossible storage: static without linkage".to_string()),
            $$$ => vec![CStatic(ni)],
            $$$ => vec![],
            $$$ => error("impossible storage: function without linkage".to_string()),
        }
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

    fn fromDirectType(__0: Type) -> TypeName {
        match (__0) {
            $$$ => ty,
            $$$ => maybe((error("undefined typeDef".to_string())), fromDirectType, ref),
            _ => error("fromDirectType".to_string()),
        }
    }

    fn ni() -> NodeInfo {
        undefNode
    }

    fn threadLocal(__0: Bool) -> Vec<CStorageSpec> {
        match (__0) {
            False => id,
            True => ((CThread(ni))(Operator(":"))),
        }
    }

}

mod Language_C_Analysis_NameSpaceMap {
    fn defGlobal($$$: NameSpaceMap<k, a>, ident: k, def: a) -> (NameSpaceMap<k, a>, Maybe<a>) {
        (NsMap((Map.insert(ident, def, gs)), lss), Map.lookup(ident, gs))
    }

    fn defLocal(__0: NameSpaceMap<k, a>, __1: k, __2: a, __3: (NameSpaceMap<k, a>, Maybe<a>)) -> (NameSpaceMap<k, a>, Maybe<a>) {
        match (__0, __1, __2, __3, __4) {
            ns $$$ $$$ ident def => defGlobal(ns, ident, def),
            $$$ ident def => (NsMap(gs, (:((:((ident, def), ls)), lss))), Prelude.lookup(ident, ls)),
        }
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

    fn leaveScope(__0: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
        match (__0) {
            $$$ => error("NsMaps.leaveScope: No local scope!".to_string()),
            $$$ => (NsMap(gs, lss), ls),
        }
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

    fn redefErrReason(__0: RedefInfo) -> String {
        match (__0) {
            $$$ => ++("duplicate definition of ".to_string(), ident),
            $$$ => ++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string())),
            $$$ => ++(ident, " previously declared as a different kind of symbol".to_string()),
            $$$ => ++(ident, " previously declared with different linkage".to_string()),
            $$$ => ++(ident, " previously declared without linkage".to_string()),
        }
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
    fn arithmeticConversion(__0: TypeName, __1: TypeName) -> Maybe<TypeName> {
        match (__0, __1) {
            $$$ $$$ => Just(TyComplex(floatConversion(t1, t2))),
            $$$ $$$ => Just(TyComplex(floatConversion(t1, t2))),
            $$$ $$$ => Just(TyComplex(floatConversion(t1, t2))),
            t1 $$$ $$$ $$$ => Just(t1),
            $$$ t2 $$$ $$$ => Just(t2),
            $$$ $$$ => Just(TyFloating(floatConversion(t1, t2))),
            t1 $$$ $$$ $$$ => Just(t1),
            $$$ t2 $$$ $$$ => Just(t2),
            $$$ $$$ => Just(TyIntegral(intConversion(t1, t2))),
            $$$ $$$ => Just(TyIntegral(TyInt)),
            $$$ t2 => Just(t2),
            t1 $$$ => Just(t1),
            _ _ => Nothing,
        }
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

    fn isAnonymousRef(__0: SUERef) -> Bool {
        match (__0) {
            $$$ => True,
            _ => False,
        }
    }

    fn isInternalIdent($$$: Ident) -> Bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo'(pos, (pos, length(s)), name)))
    }

    fn quad(__0: String) -> Int {
        match (__0) {
            $$$ => +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28))),
            $$$ => *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1))))),
            $$$ => *(ord(c2), +(bits7, ord(c1))),
            $$$ => ord(c1),
            $$$ => 0,
        }
    }

}

mod Language_C_Data_InputStream {
    fn countLines() -> Int {
        match () {
             => (length . BSC.lines),
             => (length . lines),
        }
    }

    fn inputStreamEmpty() -> Bool {
        match () {
             => BSW.null,
             => null,
        }
    }

    fn inputStreamFromString() -> InputStream {
        match () {
             => BSC.pack,
             => id,
        }
    }

    fn inputStreamToString() -> String {
        match () {
             => BSC.unpack,
             => id,
        }
    }

    fn readInputStream() -> IO<InputStream> {
        match () {
             => BSW.readFile,
             => readFile,
        }
    }

    fn takeByte(bs: InputStream) -> (Word8, InputStream) {
        seq(BSW.head(bs), (BSW.head(bs), BSW.tail(bs)))
    }

    fn takeChar(__0: InputStream) -> (Char, InputStream) {
        match (__0) {
            bs => seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs))),
            bs => (head(bs), tail(bs)),
        }
    }

    fn takeChars(__0: Int, __1: InputStream) -> Vec<Char> {
        match (__0, __1) {
            $$$ bstr => BSC.unpack(BSC.take(n, bstr)),
            n str => take(n, str),
        }
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
    fn adjustPos(__0: FilePath, __1: Int, __2: Position) -> Position {
        match (__0, __1, __2) {
            fname row $$$ => Position(offs, fname, row, 1),
            _ _ p => p,
        }
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset(__0: Position, __1: Int) -> Position {
        match (__0, __1) {
            $$$ n => Position((+(o, n)), f, r, c),
            p n => p,
        }
    }

    fn incPos(__0: Position, __1: Int) -> Position {
        match (__0, __1) {
            $$$ n => Position((+(offs, n)), fname, row, (+(col, n))),
            p _ => p,
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
            BuiltinPosition => True,
            _ => False,
        }
    }

    fn isInternalPos(__0: Position) -> Bool {
        match (__0) {
            InternalPosition => True,
            _ => False,
        }
    }

    fn isNoPos(__0: Position) -> Bool {
        match (__0) {
            NoPosition => True,
            _ => False,
        }
    }

    fn isSourcePos(__0: Position) -> Bool {
        match (__0) {
            $$$ => True,
            _ => False,
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
            $$$ => Position((+(offs, 1)), fname, (+(row, 1)), 1),
            p => p,
        }
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

    fn escapeChar(__0: Char) -> String {
        match (__0) {
            $$$ => "\\\\\\\\".to_string(),
            $$$ => "\\\\a".to_string(),
            $$$ => "\\\\b".to_string(),
            $$$ => "\\\\e".to_string(),
            $$$ => "\\\\f".to_string(),
            $$$ => "\\\\n".to_string(),
            $$$ => "\\\\r".to_string(),
            $$$ => "\\\\t".to_string(),
            $$$ => "\\\\v".to_string(),
        }
    }

    fn getCChar(__0: CChar) -> Vec<Char> {
        match (__0) {
            $$$ => vec![c],
            $$$ => cs,
        }
    }

    fn getCCharAsInt(__0: CChar) -> Integer {
        match (__0) {
            $$$ => fromIntegral((fromEnum(c))),
            $$$ => error("integer value of multi-character character constants is implementation defined".to_string()),
        }
    }

    fn getCInteger($$$: CInteger) -> Integer {
        i
    }

    fn getCString($$$: CString) -> String {
        str
    }

    fn head'(__0: String, __1: Vec<a>) -> a {
        match (__0, __1) {
            err $$$ => error(err),
            _ $$$ => x,
        }
    }

    fn isAsciiSourceChar(c: Char) -> Bool {
        &&(isAscii(c), isPrint(c))
    }

    fn isCChar(__0: Char) -> Bool {
        match (__0) {
            $$$ => False,
            $$$ => False,
            $$$ => False,
            c => isAsciiSourceChar(c),
        }
    }

    fn isSChar(__0: Char) -> Bool {
        match (__0) {
            $$$ => False,
            $$$ => False,
            $$$ => False,
            c => isAsciiSourceChar(c),
        }
    }

    fn isWideChar(__0: CChar) -> Bool {
        match (__0) {
            $$$ => wideFlag,
            $$$ => wideFlag,
        }
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

    fn unescapeChar(__0: String) -> (Char, String) {
        match (__0) {
            $$$ => match c {
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
                },
            $$$ => (c, cs),
            $$$ => error("unescape char: empty string".to_string()),
        }
    }

    fn unescapeString(__0: String) -> String {
        match (__0) {
            $$$ => vec![],
            cs => match unescapeChar(cs) {
                    $$$ => :(c, unescapeString(cs')),
                },
        }
    }

}

mod Language_C_Syntax_Ops {
    fn assignBinop(__0: CAssignOp) -> CBinaryOp {
        match (__0) {
            CAssignOp => error("direct assignment has no binary operator".to_string()),
            CMulAssOp => CMulOp,
            CDivAssOp => CDivOp,
            CRmdAssOp => CRmdOp,
            CAddAssOp => CAddOp,
            CSubAssOp => CSubOp,
            CShlAssOp => CShlOp,
            CShrAssOp => CShrOp,
            CAndAssOp => CAndOp,
            CXorAssOp => CXorOp,
            COrAssOp => COrOp,
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

