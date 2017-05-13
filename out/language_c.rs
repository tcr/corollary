// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/AstAnalysis.hs"

mod Language_C_Analysis_Builtins {
    fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/ConstEval.hs"

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

mod Language_C_Analysis_Export {
    fn exportArraySize(__0: ArraySize) -> CArrSize {
        match (__0) {
            ArraySize(static, e) => CArrSize(static, e),
            UnknownArraySize(complete) => CNoArrSize(complete),
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
            AnonBitField(ty, expr, node_info) => CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info),
            MemberDecl(vardecl, bitfieldsz, node_info) => Let(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info),
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
            NoStorage => vec![],
            Auto(reg) => if(reg, then, vec![CRegister(ni)], else, vec![]),
            Static(InternalLinkage, thread_local) => threadLocal(thread_local, vec![CStatic(ni)]),
            Static(ExternalLinkage, thread_local) => threadLocal(thread_local, vec![CExtern(ni)]),
            Static(NoLinkage, _) => error("impossible storage: static without linkage".to_string()),
            FunLinkage(InternalLinkage) => vec![CStatic(ni)],
            FunLinkage(ExternalLinkage) => vec![],
            FunLinkage(NoLinkage) => error("impossible storage: function without linkage".to_string()),
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
                TyVoid => vec![CVoidType(ni)],
                TyIntegral, ity => exportIntType(ity),
                TyFloating, fty => exportFloatType(fty),
                TyComplex, fty => exportComplexType(fty),
                TyComp, comp => exportCompTypeDecl(comp),
                TyEnum, enum => exportEnumTypeDecl(enum),
                TyBuiltin, TyVaList => vec![CTypeDef((internalIdent("va_list".to_string())), ni)],
                TyBuiltin, TyAny => vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)],
            }
    }

    fn exportVarDecl(VarDecl(name, attrs, ty): VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    }

    fn fromDirectType(__0: Type) -> TypeName {
        match (__0) {
            DirectType(ty, _, _) => ty,
            TypeDefType(TypeDefRef(_, ref, _), _, _) => maybe((error("undefined typeDef".to_string())), fromDirectType, ref),
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
    struct NameSpaceMap(NsMap, Map(k, v), Vec<Vec<(k, v)>>);

    fn defGlobal(NsMap(gs, lss): NameSpaceMap(k, a), ident: k, def: a) -> (NameSpaceMap(k, a), Maybe(a)) {
        (NsMap((Map.insert(ident, def, gs)), lss), Map.lookup(ident, gs))
    }

    fn defLocal(__0: NameSpaceMap(k, a), __1: k, __2: a, __3: (NameSpaceMap(k, a), Maybe(a))) -> (NameSpaceMap(k, a), Maybe(a)) {
        match (__0, __1, __2, __3, __4) {
            ns, EmptyParen, NsMap(_, []), ident, def => defGlobal(ns, ident, def),
            NsMap(gs, ls(EmptyParen, lss)), ident, def => (NsMap(gs, (:((:((ident, def), ls)), lss))), Prelude.lookup(ident, ls)),
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
            NsMap(_, []) => error("NsMaps.leaveScope: No local scope!".to_string()),
            NsMap(gs, ls(EmptyParen, lss)) => (NsMap(gs, lss), ls),
        }
    }

    fn localNames(NsMap(_, l): NameSpaceMap(k, v)) -> Vec<Vec<(k, v)>> {
        l
    }

    fn lookupGlobal(NsMap(gs, _): NameSpaceMap(k, a), ident: k) -> Maybe(a) {
        Map.lookup(ident, gs)
    }

    fn lookupInnermostScope(nsm: NameSpaceMap(k, a), EmptyParen: k, NsMap(_gs, localDefs): Maybe(a)) -> Maybe(a) {
        match localDefs {
                ls(EmptyParen, _lss) => Prelude.lookup(ident, ls),
                [] => lookupGlobal(nsm, ident),
            }
    }

    fn lookupName(ns: NameSpaceMap(k, a), EmptyParen: k, NsMap(_, localDefs): Maybe(a)) -> Maybe(a) {
        match (lookupLocal(localDefs)) {
                Nothing => lookupGlobal(ns, ident),
                Just, def => Just(def),
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

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/SemRep.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TravMonad.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TypeCheck.hs"

mod Language_C_Analysis_TypeConversions {
    fn arithmeticConversion(__0: TypeName, __1: TypeName) -> Maybe(TypeName) {
        match (__0, __1) {
            TyComplex(t1), TyComplex(t2) => Just(TyComplex(floatConversion(t1, t2))),
            TyComplex(t1), TyFloating(t2) => Just(TyComplex(floatConversion(t1, t2))),
            TyFloating(t1), TyComplex(t2) => Just(TyComplex(floatConversion(t1, t2))),
            t1, EmptyParen, TyComplex(_), TyIntegral(_) => Just(t1),
            TyIntegral(_), t2, EmptyParen, TyComplex(_) => Just(t2),
            TyFloating(t1), TyFloating(t2) => Just(TyFloating(floatConversion(t1, t2))),
            t1, EmptyParen, TyFloating(_), TyIntegral(_) => Just(t1),
            TyIntegral(_), t2, EmptyParen, TyFloating(_) => Just(t2),
            TyIntegral(t1), TyIntegral(t2) => Just(TyIntegral(intConversion(t1, t2))),
            TyEnum(_), TyEnum(_) => Just(TyIntegral(TyInt)),
            TyEnum(_), t2 => Just(t2),
            t1, TyEnum(_) => Just(t1),
            _, _ => Nothing,
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
            c1(EmptyParen, c2, EmptyParen, c3, EmptyParen, []) => *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1))))),
            c1(EmptyParen, c2, EmptyParen, []) => *(ord(c2), +(bits7, ord(c1))),
            c1(EmptyParen, []) => ord(c1),
            [] => 0,
        }
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Data/InputStream.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Data/Name.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Data/Node.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Data/Position.hs"

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

    fn fmapInitList(_f: fn(a) -> b) -> CInitializerList(b) {
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

mod Language_C_Syntax_Constants {
    #[derive(Eq, Ord, Clone, Debug)]
    struct CChar(CChar, Char, Bool, CChars, Vec<Char>, Bool);

    #[derive(Eq, Ord, Enum, Bounded, Clone, Debug)]
    struct CIntRepr(DecRepr, HexRepr, OctalRepr);

    #[derive(Eq, Ord, Enum, Bounded, Clone, Debug)]
    struct CIntFlag(FlagUnsigned, FlagLong, FlagLongLong, FlagImag);

    #[derive(Eq, Ord, Clone, Debug)]
    struct CInteger(CInteger, Integer, CIntRepr, Flags(CIntFlag));

    #[derive(Eq, Ord, Clone, Debug)]
    struct CFloat(CFloat, String);

    #[derive(Eq, Ord, Clone, Debug)]
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
        ++((:("\"\"".to_string(), s)), ++("\\\"".to_string(), t))
    }

    fn escapeCChar("@@WEIRD BAD BASE64 STR@@": Char) -> String {
        "\\\\\'".to_string()
    }

    fn escapeChar(__0: Char) -> String {
        match (__0) {
            "@@WEIRD BAD BASE64 STR@@" => "\\\\\\\\".to_string(),
            "@@WEIRD BAD BASE64 STR@@" => "\\\\a".to_string(),
            "@@WEIRD BAD BASE64 STR@@" => "\\\\b".to_string(),
            "@@WEIRD BAD BASE64 STR@@" => "\\\\e".to_string(),
            "@@WEIRD BAD BASE64 STR@@" => "\\\\f".to_string(),
            "@@WEIRD BAD BASE64 STR@@" => "\\\\n".to_string(),
            "@@WEIRD BAD BASE64 STR@@" => "\\\\r".to_string(),
            "@@WEIRD BAD BASE64 STR@@" => "\\\\t".to_string(),
            "@@WEIRD BAD BASE64 STR@@" => "\\\\v".to_string(),
        }
    }

    fn getCChar(__0: CChar) -> Vec<Char> {
        match (__0) {
            CChar(c, _) => vec![c],
            CChars(cs, _) => cs,
        }
    }

    fn getCCharAsInt(__0: CChar) -> Integer {
        match (__0) {
            CChar(c, _) => fromIntegral((fromEnum(c))),
            CChars(_cs, _) => error("integer value of multi-character character constants is implementation defined".to_string()),
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
            err, [] => error(err),
            _, x(EmptyParen, _) => x,
        }
    }

    fn isAsciiSourceChar(c: Char) -> Bool {
        &&(isAscii(c), isPrint(c))
    }

    fn isCChar(__0: Char) -> Bool {
        match (__0) {
            "@@WEIRD BAD BASE64 STR@@" => False,
            "@@WEIRD BAD BASE64 STR@@" => False,
            "@@WEIRD BAD BASE64 STR@@" => False,
            c => isAsciiSourceChar(c),
        }
    }

    fn isSChar(__0: Char) -> Bool {
        match (__0) {
            "@@WEIRD BAD BASE64 STR@@" => False,
            "@@WEIRD BAD BASE64 STR@@" => False,
            "@@WEIRD BAD BASE64 STR@@" => False,
            c => isAsciiSourceChar(c),
        }
    }

    fn isWideChar(__0: CChar) -> Bool {
        match (__0) {
            CChar(_, wideFlag) => wideFlag,
            CChars(_, wideFlag) => wideFlag,
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
                [(n, suffix)] => mkCInt(n, suffix),
                parseFailed => Left(++("Bad Integer literal: ".to_string(), show(parseFailed))),
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
            "@@WEIRD BAD BASE64 STR@@"(EmptyParen, c, EmptyParen, cs) => match c {
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => ("\"\"".to_string(), cs),
                    "@@WEIRD BAD BASE64 STR@@" => match head'("bad escape sequence".to_string(), (readHex(cs))) {
                            (i, cs') => (toEnum(i), cs'),
                        },
                    _ => match head'("bad escape sequence".to_string(), (readOct((:(c, cs))))) {
                            (i, cs') => (toEnum(i), cs'),
                        },
                },
            c(EmptyParen, cs) => (c, cs),
            [] => error("unescape char: empty string".to_string()),
        }
    }

    fn unescapeString(__0: String) -> String {
        match (__0) {
            [] => vec![],
            cs => match unescapeChar(cs) {
                    (c, cs') => :(c, unescapeString(cs')),
                },
        }
    }

}

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

mod Language_C_Syntax_Utils {
    fn compoundSubStmts(__0: CBlockItem) -> Vec<CStat> {
        match (__0) {
            CBlockStmt(s) => vec![s],
            CBlockDecl(_) => vec![],
            CNestedFunDef(_) => vec![],
        }
    }

    fn getLabels(__0: CStat) -> Vec<Ident> {
        match (__0) {
            CLabel(l, s, _, _) => :(l, getLabels(s)),
            CCompound(ls, body, _) => \\(concatMap(((concatMap(getLabels) . compoundSubStmts)), body), ls),
            stmt => concatMap(getLabels, (getSubStmts(stmt))),
        }
    }

    fn getSubStmts(__0: CStat) -> Vec<CStat> {
        match (__0) {
            CLabel(_, s, _, _) => vec![s],
            CCase(_, s, _) => vec![s],
            CCases(_, _, s, _) => vec![s],
            CDefault(s, _) => vec![s],
            CExpr(_, _) => vec![],
            CCompound(_, body, _) => concatMap(compoundSubStmts, body),
            CIf(_, sthen, selse, _) => maybe(vec![sthen], (Lambda), selse),
            CSwitch(_, s, _) => vec![s],
            CWhile(_, s, _, _) => vec![s],
            CFor(_, _, _, s, _) => vec![s],
            CGoto(_, _) => vec![],
            CGotoPtr(_, _) => vec![],
            CCont(_) => vec![],
            CBreak(_) => vec![],
            CReturn(_, _) => vec![],
            CAsm(_, _) => vec![],
        }
    }

    fn mapBlockItemStmts(__0: fn(CStat) -> Bool, __1: fn(CStat) -> CStat, __2: CBlockItem) -> CBlockItem {
        match (__0, __1, __2) {
            stop, f, CBlockStmt(s) => CBlockStmt((mapSubStmts(stop, f, s))),
            _, _, bi => bi,
        }
    }

    fn mapSubStmts(__0: fn(CStat) -> Bool, __1: fn(CStat) -> CStat, __2: CStat) -> CStat {
        match (__0, __1, __2) {
            stop, f, CLabel(i, s, attrs, ni) => f((CLabel(i, (mapSubStmts(stop, f, s)), attrs, ni))),
            stop, f, CCase(e, s, ni) => f((CCase(e, (mapSubStmts(stop, f, s)), ni))),
            stop, f, CCases(e1, e2, s, ni) => f((CCases(e1, e2, (mapSubStmts(stop, f, s)), ni))),
            stop, f, CDefault(s, ni) => f((CDefault((mapSubStmts(stop, f, s)), ni))),
            stop, f, CCompound(ls, body, ni) => f((CCompound(ls, (map((mapBlockItemStmts(stop, f)), body)), ni))),
            stop, f, CIf(e, sthen, selse, ni) => f((CIf(e, (mapSubStmts(stop, f, sthen)), (maybe(Nothing, ((Just . mapSubStmts(stop, f))), selse)), ni))),
            stop, f, CSwitch(e, s, ni) => f((CSwitch(e, (mapSubStmts(stop, f, s)), ni))),
            stop, f, CWhile(e, s, isdo, ni) => f((CWhile(e, (mapSubStmts(stop, f, s)), isdo, ni))),
            stop, f, CFor(i, t, a, s, ni) => f((CFor(i, t, a, (mapSubStmts(stop, f, s)), ni))),
            _, f, s => f(s),
        }
    }

}

mod Language_C_Syntax {

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/System/GCC.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/System/Preprocess.hs"



fn main() { /* demo */ }
