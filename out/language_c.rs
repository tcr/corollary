// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/AstAnalysis.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/Builtins.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/ConstEval.hs"

mod Language_C_Analysis_Debug {
    fn globalDeclStats(file_filter: Pair(Span([Ref(Ident("FilePath"))]), Span([Ref(Ident("Bool"))])), gmap: GlobalDecls) -> Vec<(String, isize)> {
        vec![("Enumeration Constants".to_string(), Map.size(enumerators)), ("Total Object/Function Declarations".to_string(), Map.size(all_decls)), ("Object definitions".to_string(), Map.size(objDefs)), ("Function Definitions".to_string(), Map.size(funDefs)), ("Tag definitions".to_string(), Map.size(tagDefs)), ("TypeDefs".to_string(), Map.size(typeDefs))]
    }

    fn joinComma() -> Doc {
        (hsep . (punctuate(comma) . map(pretty)))
    }

    fn prettyAssocs(label: String) -> Doc {
        prettyAssocsWith(label, pretty, pretty)
    }

    fn prettyAssocsWith(label: String, prettyKey: Pair(Span([Ref(Ident("k"))]), Span([Ref(Ident("Doc"))])), prettyVal: Pair(Span([Ref(Ident("v"))]), Span([Ref(Ident("Doc"))])), theMap: Vec<(k, v)>) -> Doc {
        text(label)(()((nest(8))((vcat(map(prettyEntry, theMap))))))
    }

    fn terminateSemi() -> Doc {
        (terminateSemi_ . map(pretty))
    }

    fn terminateSemi_() -> Doc {
        (hsep . map((<((), Operator(">")(semi)))))
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/DeclAnalysis.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/DefTable.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/Export.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/NameSpaceMap.hs"

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
            RedefInfo(ident, DuplicateDef, _, _) => ++("duplicate definition of ".to_string(), ident),
            RedefInfo(ident, ShadowedDef, _, _) => ++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string())),
            RedefInfo(ident, DiffKindRedecl, _, _) => ++(ident, " previously declared as a different kind of symbol".to_string()),
            RedefInfo(ident, DisagreeLinkage, _, _) => ++(ident, " previously declared with different linkage".to_string()),
            RedefInfo(ident, NoLinkageOld, _, _) => ++(ident, " previously declared without linkage".to_string()),
        }
    }

    fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, EmptyParen: ErrorInfo) -> ErrorInfo {
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
    #[derive(Debug, Clone)]
    struct TagDef(CompDef, CompType, EnumDef, EnumType);

    #[derive(Debug, Clone)]
    struct IdentDecl(Declaration, Decl, ObjectDef, ObjDef, FunctionDef, FunDef, EnumeratorDef, Enumerator);

    struct GlobalDecls(GlobalDecls, RecordTODO);

    #[derive()]
    struct DeclEvent(TagEvent, TagDef, DeclEvent, IdentDecl, ParamEvent, ParamDecl, LocalEvent, IdentDecl, TypeDefEvent, TypeDef, AsmEvent, AsmBlock);

    #[derive(Debug, Clone)]
    struct Decl(Decl, VarDecl, NodeInfo);

    #[derive(Debug, Clone)]
    struct ObjDef(ObjDef, VarDecl, Maybe(Initializer), NodeInfo);

    #[derive(Debug, Clone)]
    struct FunDef(FunDef, VarDecl, Stmt, NodeInfo);

    #[derive(Debug, Clone)]
    struct ParamDecl(ParamDecl, VarDecl, NodeInfo, AbstractParamDecl, VarDecl, NodeInfo);

    #[derive(Debug, Clone)]
    struct MemberDecl(MemberDecl, VarDecl, Maybe(Expr), NodeInfo, AnonBitField, Type, Expr, NodeInfo);

    #[derive(Debug, Clone)]
    struct TypeDef(TypeDef, Ident, Type, Attributes, NodeInfo);

    #[derive(Debug, Clone)]
    struct VarDecl(VarDecl, VarName, DeclAttrs, Type);

    #[derive(Debug, Clone)]
    struct DeclAttrs(DeclAttrs, Bool, Storage, Attributes);

    #[derive(Debug, Clone, Show, Eq, Ord)]
    struct Storage(NoStorage, Auto, Register, Static, Linkage, ThreadLocal, FunLinkage, Linkage);

    #[derive(Debug, Clone, Show, Eq, Ord)]
    struct Linkage(NoLinkage, InternalLinkage, ExternalLinkage);

    #[derive(Debug, Clone)]
    struct Type(DirectType, TypeName, TypeQuals, Attributes, PtrType, Type, TypeQuals, Attributes, ArrayType, Type, ArraySize, TypeQuals, Attributes, FunctionType, FunType, Attributes, TypeDefType, TypeDefRef, TypeQuals, Attributes);

    #[derive(Debug, Clone)]
    struct FunType(FunType, Type, Vec<ParamDecl>, Bool, FunTypeIncomplete, Type);

    #[derive(Debug, Clone)]
    struct ArraySize(UnknownArraySize, Bool, ArraySize, Bool, Expr);

    #[derive(Debug, Clone)]
    struct TypeName(TyVoid, TyIntegral, IntType, TyFloating, FloatType, TyComplex, FloatType, TyComp, CompTypeRef, TyEnum, EnumTypeRef, TyBuiltin, BuiltinType);

    #[derive(Debug, Clone)]
    struct BuiltinType(TyVaList, TyAny);

    #[derive(Debug, Clone)]
    struct TypeDefRef(TypeDefRef, Ident, Maybe(Type), NodeInfo);

    #[derive(Debug, Clone, Eq, Ord)]
    struct IntType(TyBool, TyChar, TySChar, TyUChar, TyShort, TyUShort, TyInt, TyUInt, TyLong, TyULong, TyLLong, TyULLong);

    #[derive(Debug, Clone, Eq, Ord)]
    struct FloatType(TyFloat, TyDouble, TyLDouble);

    #[derive(Debug, Clone)]
    struct CompTypeRef(CompTypeRef, SUERef, CompTyKind, NodeInfo);

    #[derive(Debug, Clone)]
    struct EnumTypeRef(EnumTypeRef, SUERef, NodeInfo);

    #[derive(Debug, Clone)]
    struct CompType(CompType, SUERef, CompTyKind, Vec<MemberDecl>, Attributes, NodeInfo);

    #[derive(Eq, Ord, Debug, Clone)]
    struct CompTyKind(StructTag, UnionTag);

    #[derive(Debug, Clone)]
    struct EnumType(EnumType, SUERef, Vec<Enumerator>, Attributes, NodeInfo);

    #[derive(Debug, Clone)]
    struct Enumerator(Enumerator, Ident, Expr, EnumType, NodeInfo);

    #[derive(Debug, Clone)]
    struct TypeQuals(TypeQuals, RecordTODO);

    #[derive(Debug, Clone)]
    struct VarName(VarName, Ident, Maybe(AsmName), NoName);

    #[derive(Debug, Clone)]
    struct Attr(Attr, Ident, Vec<Expr>, NodeInfo);

    fn declAttrs() -> DeclAttrs {
        ((Lambda) . getVarDecl)
    }

    fn declIdent() -> Ident {
        (identOfVarName . declName)
    }

    fn declLinkage(decl: d) -> Linkage {
        match declStorage(decl) {
                NoStorage => undefined,
                Auto _ => NoLinkage,
                Static linkage _ => linkage,
                FunLinkage linkage => linkage,
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
                DeclAttrs(_, st, _) => st,
            }
    }

    fn declType() -> Type {
        ((Lambda) . getVarDecl)
    }

    fn emptyGlobalDecls() -> GlobalDecls {
        GlobalDecls(Map.empty, Map.empty, Map.empty)
    }

    fn filterGlobalDecls(decl_filter: Pair(Span([Ref(Ident("DeclEvent"))]), Span([Ref(Ident("Bool"))])), gmap: GlobalDecls) -> GlobalDecls {
        GlobalDecls(hashmap! {
            "gObjs" => Map.filter(((decl_filter . DeclEvent)), (gObjs(gmap))),
            "gTags" => Map.filter(((decl_filter . TagEvent)), (gTags(gmap))),
            "gTypeDefs" => Map.filter(((decl_filter . TypeDefEvent)), (gTypeDefs(gmap)))
            })
    }

    fn hasLinkage(__0: Storage) -> Bool {
        match (__0) {
            Auto(_) => False,
            Static(NoLinkage, _) => False,
            _ => True,
        }
    }

    fn identOfTypeDef(TypeDef(ide, _, _, _): TypeDef) -> Ident {
        ide
    }

    fn identOfVarName(__0: VarName) -> Ident {
        match (__0) {
            NoName => error("identOfVarName: NoName".to_string()),
            VarName(ident, _) => ident,
        }
    }

    fn isExtDecl() -> Bool {
        (hasLinkage . declStorage)
    }

    fn isNoName(__0: VarName) -> Bool {
        match (__0) {
            NoName => True,
            _ => False,
        }
    }

    fn mergeAttributes() -> Attributes {
        (Operator("++"))
    }

    fn mergeGlobalDecls(gmap1: GlobalDecls, gmap2: GlobalDecls) -> GlobalDecls {
        GlobalDecls(hashmap! {
            "gObjs" => Map.union((gObjs(gmap1)), (gObjs(gmap2))),
            "gTags" => Map.union((gTags(gmap1)), (gTags(gmap2))),
            "gTypeDefs" => Map.union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
            })
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
            Declaration(_) => "declaration".to_string(),
            ObjectDef(_) => "object definition".to_string(),
            FunctionDef(_) => "function definition".to_string(),
            EnumeratorDef(_) => "enumerator definition".to_string(),
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
            CompDef(comptype) => typeOfCompDef(comptype),
            EnumDef(enumtype) => typeOfEnumDef(enumtype),
        }
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TravMonad.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TypeCheck.hs"

mod Language_C_Analysis_TypeConversions {
    fn arithmeticConversion(__0: TypeName, __1: TypeName) -> Maybe(TypeName) {
        match (__0, __1) {
            TyComplex(t1) TyComplex(t2) => Just(TyComplex(floatConversion(t1, t2))),
            TyComplex(t1) TyFloating(t2) => Just(TyComplex(floatConversion(t1, t2))),
            TyFloating(t1) TyComplex(t2) => Just(TyComplex(floatConversion(t1, t2))),
            t1 EmptyParen TyComplex(_) TyIntegral(_) => Just(t1),
            TyIntegral(_) t2 EmptyParen TyComplex(_) => Just(t2),
            TyFloating(t1) TyFloating(t2) => Just(TyFloating(floatConversion(t1, t2))),
            t1 EmptyParen TyFloating(_) TyIntegral(_) => Just(t1),
            TyIntegral(_) t2 EmptyParen TyFloating(_) => Just(t2),
            TyIntegral(t1) TyIntegral(t2) => Just(TyIntegral(intConversion(t1, t2))),
            TyEnum(_) TyEnum(_) => Just(TyIntegral(TyInt)),
            TyEnum(_) t2 => Just(t2),
            t1 TyEnum(_) => Just(t1),
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

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TypeUtils.hs"

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
        error((++(internalErrPrefix, ++("\\n".to_string(), ++(indentLines(msg), "\\n".to_string())))))
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
        ++(header, showMsgLines((:(if(null, short_msg, then, msgs, else, short_msg), msgs))))
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
    #[derive(Debug, Clone, Ord, Eq)]
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
            AnonymousRef(_) => True,
            _ => False,
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
            c1(EmptyParen, c2, EmptyParen, c3, EmptyParen, c4, EmptyParen, s) => +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28))),
            c1(EmptyParen, c2, EmptyParen, c3, EmptyParen, Vec<>) => *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1))))),
            c1(EmptyParen, c2, EmptyParen, Vec<>) => *(ord(c2), +(bits7, ord(c1))),
            c1(EmptyParen, Vec<>) => ord(c1),
            Vec<> => 0,
        }
    }

}

mod Language_C_Data_InputStream {
    fn countLines() -> isize {
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

    fn readInputStream() -> IO(InputStream) {
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

    fn takeChars(__0: isize, __1: InputStream) -> Vec<Char> {
        match (__0, __1) {
            n bstr => BSC.unpack(BSC.take(n, bstr)),
            n str => take(n, str),
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

// ERROR: cannot yet convert file "./language-c/src/Language/C/Data/Node.hs"

mod Language_C_Data_Position {
    #[derive(Eq, Ord, Debug, Clone)]
    struct Position(Position, RecordTODO, NoPosition, BuiltinPosition, InternalPosition);

    fn adjustPos(__0: FilePath, __1: isize, __2: Position) -> Position {
        match (__0, __1, __2) {
            fname row Position(offs, _, _, _) => Position(offs, fname, row, 1),
            _ _ p => p,
        }
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset(__0: Position, __1: isize) -> Position {
        match (__0, __1) {
            Position(o, f, r, c) n => Position((+(o, n)), f, r, c),
            p n => p,
        }
    }

    fn incPos(__0: Position, __1: isize) -> Position {
        match (__0, __1) {
            Position(offs, fname, row, col) n => Position((+(offs, n)), fname, row, (+(col, n))),
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
            Position(_, _, _, _) => True,
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
            Position(offs, fname, row, _) => Position((+(offs, 1)), fname, (+(row, 1)), 1),
            p => p,
        }
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Data/RList.hs"

mod Language_C_Data {

}

mod Language_C_Parser_Builtin {
    fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Parser/ParserMonad.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Parser/Tokens.hs"

mod Language_C_Parser {
    fn execParser_(parser: P(a), input: InputStream, pos: Position) -> Either(ParseError, a) {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Pretty.hs"

mod Language_C_Syntax_AST {
    #[derive(Show, Clone, Debug)]
    struct CTranslationUnit(CTranslUnit, Vec<CExternalDeclaration(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CExternalDeclaration(CDeclExt, CDeclaration(a), CFDefExt, CFunctionDef(a), CAsmExt, CStringLiteral(a), a);

    #[derive(Show, Clone, Debug)]
    struct CFunctionDef(CFunDef, Vec<CDeclarationSpecifier(a)>, CDeclarator(a), Vec<CDeclaration(a)>, CStatement(a), a);

    #[derive(Show, Clone, Debug)]
    struct CDeclaration(CDecl, Vec<CDeclarationSpecifier(a)>, Vec<(Maybe(CDeclarator(a)), Maybe(CInitializer(a)), Maybe(CExpression(a)))>, a);

    #[derive(Show, Clone, Debug)]
    struct CDeclarator(CDeclr, Maybe(Ident), Vec<CDerivedDeclarator(a)>, Maybe(CStringLiteral(a)), Vec<CAttribute(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CDerivedDeclarator(CPtrDeclr, Vec<CTypeQualifier(a)>, a, CArrDeclr, Vec<CTypeQualifier(a)>, CArraySize(a), a, CFunDeclr, Either(Vec<Ident>, (Vec<CDeclaration(a)>, Bool)), Vec<CAttribute(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CArraySize(CNoArrSize, Bool, CArrSize, Bool, CExpression(a));

    #[derive(Show, Clone, Debug)]
    struct CStatement(CLabel, Ident, CStatement(a), Vec<CAttribute(a)>, a, CCase, CExpression(a), CStatement(a), a, CCases, CExpression(a), CExpression(a), CStatement(a), a, CDefault, CStatement(a), a, CExpr, Maybe(CExpression(a)), a, CCompound, Vec<Ident>, Vec<CCompoundBlockItem(a)>, a, CIf, CExpression(a), CStatement(a), Maybe(CStatement(a)), a, CSwitch, CExpression(a), CStatement(a), a, CWhile, CExpression(a), CStatement(a), Bool, a, CFor, Either(Maybe(CExpression(a)), CDeclaration(a)), Maybe(CExpression(a)), Maybe(CExpression(a)), CStatement(a), a, CGoto, Ident, a, CGotoPtr, CExpression(a), a, CCont, a, CBreak, a, CReturn, Maybe(CExpression(a)), a, CAsm, CAssemblyStatement(a), a);

    #[derive(Show, Clone, Debug)]
    struct CAssemblyStatement(CAsmStmt, Maybe(CTypeQualifier(a)), CStringLiteral(a), Vec<CAssemblyOperand(a)>, Vec<CAssemblyOperand(a)>, Vec<CStringLiteral(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CAssemblyOperand(CAsmOperand, Maybe(Ident), CStringLiteral(a), CExpression(a), a);

    #[derive(Show, Clone, Debug)]
    struct CCompoundBlockItem(CBlockStmt, CStatement(a), CBlockDecl, CDeclaration(a), CNestedFunDef, CFunctionDef(a));

    #[derive(Show, Clone, Debug)]
    struct CDeclarationSpecifier(CStorageSpec, CStorageSpecifier(a), CTypeSpec, CTypeSpecifier(a), CTypeQual, CTypeQualifier(a));

    #[derive(Show, Eq, Ord, Clone, Debug)]
    struct CStorageSpecifier(CAuto, a, CRegister, a, CStatic, a, CExtern, a, CTypedef, a, CThread, a);

    #[derive(Show, Clone, Debug)]
    struct CTypeSpecifier(CVoidType, a, CCharType, a, CShortType, a, CIntType, a, CLongType, a, CFloatType, a, CDoubleType, a, CSignedType, a, CUnsigType, a, CBoolType, a, CComplexType, a, CSUType, CStructureUnion(a), a, CEnumType, CEnumeration(a), a, CTypeDef, Ident, a, CTypeOfExpr, CExpression(a), a, CTypeOfType, CDeclaration(a), a);

    #[derive(Show, Clone, Debug)]
    struct CTypeQualifier(CConstQual, a, CVolatQual, a, CRestrQual, a, CInlineQual, a, CAttrQual, CAttribute(a));

    #[derive(Show, Clone, Debug)]
    struct CStructureUnion(CStruct, CStructTag, Maybe(Ident), Maybe(Vec<CDeclaration(a)>), Vec<CAttribute(a)>, a);

    #[derive(Show, Eq, Clone, Debug)]
    struct CStructTag(CStructTag, CUnionTag);

    #[derive(Show, Clone, Debug)]
    struct CEnumeration(CEnum, Maybe(Ident), Maybe(Vec<(Ident, Maybe(CExpression(a)))>), Vec<CAttribute(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CInitializer(CInitExpr, CExpression(a), a, CInitList, CInitializerList(a), a);

    #[derive(Show, Clone, Debug)]
    struct CPartDesignator(CArrDesig, CExpression(a), a, CMemberDesig, Ident, a, CRangeDesig, CExpression(a), CExpression(a), a);

    #[derive(Show, Clone, Debug)]
    struct CAttribute(CAttr, Ident, Vec<CExpression(a)>, a);

    #[derive(Clone, Debug, Show)]
    struct CExpression(CComma, Vec<CExpression(a)>, a, CAssign, CAssignOp, CExpression(a), CExpression(a), a, CCond, CExpression(a), Maybe(CExpression(a)), CExpression(a), a, CBinary, CBinaryOp, CExpression(a), CExpression(a), a, CCast, CDeclaration(a), CExpression(a), a, CUnary, CUnaryOp, CExpression(a), a, CSizeofExpr, CExpression(a), a, CSizeofType, CDeclaration(a), a, CAlignofExpr, CExpression(a), a, CAlignofType, CDeclaration(a), a, CComplexReal, CExpression(a), a, CComplexImag, CExpression(a), a, CIndex, CExpression(a), CExpression(a), a, CCall, CExpression(a), Vec<CExpression(a)>, a, CMember, CExpression(a), Ident, Bool, a, CVar, Ident, a, CConst, CConstant(a), CCompoundLit, CDeclaration(a), CInitializerList(a), a, CStatExpr, CStatement(a), a, CLabAddrExpr, Ident, a, CBuiltinExpr, CBuiltinThing(a));

    #[derive(Show, Clone, Debug)]
    struct CBuiltinThing(CBuiltinVaArg, CExpression(a), CDeclaration(a), a, CBuiltinOffsetOf, CDeclaration(a), Vec<CPartDesignator(a)>, a, CBuiltinTypesCompatible, CDeclaration(a), CDeclaration(a), a);

    #[derive(Show, Clone, Debug)]
    struct CConstant(CIntConst, CInteger, a, CCharConst, CChar, a, CFloatConst, CFloat, a, CStrConst, CString, a);

    #[derive(Show, Clone, Debug)]
    struct CStringLiteral(CStrLit, CString, a);

    fn cstringOfLit(CStrLit(cstr, _): CStringLiteral(a)) -> CString {
        cstr
    }

    fn fmapInitList(_f: Pair(Span([Ref(Ident("a"))]), Span([Ref(Ident("b"))]))) -> CInitializerList(b) {
        map((Lambda))
    }

    fn isSUEDef(__0: CTypeSpecifier(a)) -> Bool {
        match (__0) {
            CSUType(CStruct(_, _, Just(_), _, _), _) => True,
            CEnumType(CEnum(_, Just(_), _, _), _) => True,
            _ => False,
        }
    }

    fn liftStrLit(CStrLit(str, at): CStringLiteral(a)) -> CConstant(a) {
        CStrConst(str, at)
    }

    fn partitionDeclSpecs() -> (Vec<CStorageSpecifier(a)>, Vec<CAttribute(a)>, Vec<CTypeQualifier(a)>, Vec<CTypeSpecifier(a)>, Bool) {
        foldr(deals, (vec![], vec![], vec![], vec![], False))
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Syntax/Constants.hs"

mod Language_C_Syntax_Ops {
    #[derive(Eq, Ord, Show, Clone, Debug)]
    struct CAssignOp(CAssignOp, CMulAssOp, CDivAssOp, CRmdAssOp, CAddAssOp, CSubAssOp, CShlAssOp, CShrAssOp, CAndAssOp, CXorAssOp, COrAssOp);

    #[derive(Eq, Ord, Show, Clone, Debug)]
    struct CBinaryOp(CMulOp, CDivOp, CRmdOp, CAddOp, CSubOp, CShlOp, CShrOp, CLeOp, CGrOp, CLeqOp, CGeqOp, CEqOp, CNeqOp, CAndOp, CXorOp, COrOp, CLndOp, CLorOp);

    #[derive(Eq, Ord, Show, Clone, Debug)]
    struct CUnaryOp(CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp, CAdrOp, CIndOp, CPlusOp, CMinOp, CCompOp, CNegOp);

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

// ERROR: cannot yet convert file "./language-c/src/Language/C/Syntax/Utils.hs"

mod Language_C_Syntax {

}

mod Language_C_System_GCC {
    fn buildCppArgs(CppArgs(options, extra_args, _tmpdir, input_file, output_file_opt): CppArgs) -> Vec<String> {
        ++({
                (concatMap(tOption, options))
            }, ++(outputFileOpt, ++(vec!["-E".to_string(), input_file], extra_args)))
    }

    fn gccParseCPPArgs(args: Vec<String>) -> Either(String, (CppArgs, Vec<String>)) {
        match mungeArgs(((Nothing, Nothing, RList.empty), (RList.empty, RList.empty)), args) {
                Left err => Left(err),
                Right ((Nothing, _, _), _) => Left("No .c / .hc / .h source file given".to_string()),
                Right ((Just(input_file), output_file_opt, cpp_opts), (extra_args, other_args)) => Right(((rawCppArgs((RList.reverse(extra_args)), input_file))(hashmap! {
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
    struct CppOption(IncludeDir, FilePath, Define, String, String, Undefine, String, IncludeFile, FilePath);

    struct CppArgs(CppArgs, RecordTODO);

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

    fn mkOutputFile(tmp_dir_opt: Maybe(FilePath), input_file: FilePath) -> IO(FilePath) {
        {
            let getTempDir = |__0| {
                match (__0) {
                    Just(tmpdir) => return(tmpdir),
                    Nothing => getTemporaryDirectory,
                }
            };

            let tmpDir = getTempDir(tmp_dir_opt, mkTmpFile, tmpDir, (getOutputFileName(input_file)))
        }
    }

    fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> IO(FilePath) {
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

    fn runPreprocessor(cpp: cpp, cpp_args: CppArgs) -> IO(Either(ExitCode, InputStream)) {
        {
            fn getActualOutFile() -> IO(FilePath) {
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



fn main() { /* demo */ }
