mod Language_C_Analysis_AstAnalysis {
    struct StmtCtx(FunCtx, VarDecl, LoopCtx, SwitchCtx);

    #[derive(Debug, Eq)]
    struct ExprSide(LValue, RValue);


}

mod Language_C_Analysis_Builtins {

}

mod Language_C_Analysis_ConstEval {
    struct MachineDesc(MachineDesc, { /* struct def */ });


}

mod Language_C_Analysis_Debug {

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


}

mod Language_C_Analysis_DefTable {
    struct TagFwdDecl(CompDecl, CompTypeRef, EnumDecl, EnumTypeRef);

    struct DefTable(DefTable, { /* struct def */ });

    #[derive(Clone, Debug)]
    struct DeclarationStatus(NewDecl, Redeclared, t, KeepDef, t, Shadowed, t, KindMismatch, t);

    #[derive(Eq, Ord)]
    struct TagEntryKind(CompKind, CompTyKind, EnumKind);


}

mod Language_C_Analysis_Export {

}

mod Language_C_Analysis_NameSpaceMap {
    struct NameSpaceMap(NsMap, Map(k, v), Vec<Vec<(k, v)>>);


}

mod Language_C_Analysis_SemError {
    #[derive(Debug)]
    struct RedefError(RedefError, ErrorLevel, RedefInfo);

    struct RedefInfo(RedefInfo, String, RedefKind, NodeInfo, NodeInfo);

    struct RedefKind(DuplicateDef, DiffKindRedecl, ShadowedDef, DisagreeLinkage, NoLinkageOld);

    #[derive(Debug)]
    struct TypeMismatch(TypeMismatch, String, (NodeInfo, Type), (NodeInfo, Type));


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


}

mod Language_C_Analysis_TravMonad {
    struct CLanguage(C89, C99, GNU89, GNU99);

    struct TravOptions(TravOptions, { /* struct def */ });

    struct TravState(TravState, { /* struct def */ });


}

mod Language_C_Analysis_TypeCheck {

}

mod Language_C_Analysis_TypeConversions {

}

mod Language_C_Analysis_TypeUtils {

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


}

mod Language_C_Data_Ident {
    #[derive(Clone, Debug, Eq, Ord)]
    struct SUERef(AnonymousRef, Name, NamedRef, Ident);

    #[derive(Clone, Debug)]
    struct Ident(Ident, String, isize, NodeInfo);


}

mod Language_C_Data_InputStream {
    fn countLines() -> isize {
        match () {
            <todo> => { (length . BSC.lines) },
            <todo> => { (length . lines) },
        }
    }

    fn inputStreamEmpty() -> Bool {
        match () {
            <todo> => { BSW.null },
            <todo> => { null },
        }
    }

    fn inputStreamFromString() -> InputStream {
        match () {
            <todo> => { BSC.pack },
            <todo> => { id },
        }
    }

    fn inputStreamToString() -> String {
        match () {
            <todo> => { BSC.unpack },
            <todo> => { id },
        }
    }

    fn readInputStream() -> IO {
        match () {
            <todo> => { BSW.readFile },
            <todo> => { readFile },
        }
    }

    let takeChar(bs) = || {
        match () {
            <todo> => { seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs))) },
            <todo> => { (head(bs), tail(bs)) },
        }
    };

}

mod Language_C_Data_Name {

}

mod Language_C_Data_Node {
    #[derive(Clone, Debug)]
    struct NodeInfo(OnlyPos, Position, PosLength, NodeInfo, Position, PosLength, Name);


}

mod Language_C_Data_Position {
    #[derive(Clone, Debug, Eq, Ord)]
    struct Position(Position, { /* struct def */ }, NoPosition, BuiltinPosition, InternalPosition);


}

mod Language_C_Data_RList {

}

mod Language_C_Data {

}

mod Language_C_Parser_Builtin {

}

mod Language_C_Parser_ParserMonad {
    struct ParseResult(POk, PState, a, PFailed, Vec<String>, Position);

    struct PState(PState, { /* struct def */ });


}

mod Language_C_Parser_Tokens {
    struct CToken(CTokLParen, PosLength, CTokRParen, PosLength, CTokLBracket, PosLength, CTokRBracket, PosLength, CTokArrow, PosLength, CTokDot, PosLength, CTokExclam, PosLength, CTokTilde, PosLength, CTokInc, PosLength, CTokDec, PosLength, CTokPlus, PosLength, CTokMinus, PosLength, CTokStar, PosLength, CTokSlash, PosLength, CTokPercent, PosLength, CTokAmper, PosLength, CTokShiftL, PosLength, CTokShiftR, PosLength, CTokLess, PosLength, CTokLessEq, PosLength, CTokHigh, PosLength, CTokHighEq, PosLength, CTokEqual, PosLength, CTokUnequal, PosLength, CTokHat, PosLength, CTokBar, PosLength, CTokAnd, PosLength, CTokOr, PosLength, CTokQuest, PosLength, CTokColon, PosLength, CTokAssign, PosLength, CTokPlusAss, PosLength, CTokMinusAss, PosLength, CTokStarAss, PosLength, CTokSlashAss, PosLength, CTokPercAss, PosLength, CTokAmpAss, PosLength, CTokHatAss, PosLength, CTokBarAss, PosLength, CTokSLAss, PosLength, CTokSRAss, PosLength, CTokComma, PosLength, CTokSemic, PosLength, CTokLBrace, PosLength, CTokRBrace, PosLength, CTokEllipsis, PosLength, CTokAlignof, PosLength, CTokAsm, PosLength, CTokAuto, PosLength, CTokBreak, PosLength, CTokBool, PosLength, CTokCase, PosLength, CTokChar, PosLength, CTokConst, PosLength, CTokContinue, PosLength, CTokComplex, PosLength, CTokDefault, PosLength, CTokDo, PosLength, CTokDouble, PosLength, CTokElse, PosLength, CTokEnum, PosLength, CTokExtern, PosLength, CTokFloat, PosLength, CTokFor, PosLength, CTokGoto, PosLength, CTokIf, PosLength, CTokInline, PosLength, CTokInt, PosLength, CTokLong, PosLength, CTokLabel, PosLength, CTokRegister, PosLength, CTokRestrict, PosLength, CTokReturn, PosLength, CTokShort, PosLength, CTokSigned, PosLength, CTokSizeof, PosLength, CTokStatic, PosLength, CTokStruct, PosLength, CTokSwitch, PosLength, CTokTypedef, PosLength, CTokTypeof, PosLength, CTokThread, PosLength, CTokUnion, PosLength, CTokUnsigned, PosLength, CTokVoid, PosLength, CTokVolatile, PosLength, CTokWhile, PosLength, CTokCLit, PosLength, CChar, CTokILit, PosLength, CInteger, CTokFLit, PosLength, CFloat, CTokSLit, PosLength, CString, CTokIdent, PosLength, Ident, CTokTyIdent, PosLength, Ident, CTokGnuC, GnuCTok, PosLength, CTokEof);

    struct GnuCTok(GnuCAttrTok, GnuCExtTok, GnuCVaArg, GnuCOffsetof, GnuCTyCompat, GnuCComplexReal, GnuCComplexImag);


}

mod Language_C_Parser {

}

mod Language_C_Pretty {

}

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


}

mod Language_C_Syntax_Ops {
    #[derive(Clone, Debug, Eq, Ord)]
    struct CAssignOp(CAssignOp, CMulAssOp, CDivAssOp, CRmdAssOp, CAddAssOp, CSubAssOp, CShlAssOp, CShrAssOp, CAndAssOp, CXorAssOp, COrAssOp);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CBinaryOp(CMulOp, CDivOp, CRmdOp, CAddOp, CSubOp, CShlOp, CShrOp, CLeOp, CGrOp, CLeqOp, CGeqOp, CEqOp, CNeqOp, CAndOp, CXorOp, COrOp, CLndOp, CLorOp);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CUnaryOp(CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp, CAdrOp, CIndOp, CPlusOp, CMinOp, CCompOp, CNegOp);


}

mod Language_C_Syntax_Utils {

}

mod Language_C_Syntax {

}

mod Language_C_System_GCC {

}

mod Language_C_System_Preprocess {
    struct CppOption(IncludeDir, FilePath, Define, String, String, Undefine, String, IncludeFile, FilePath);

    struct CppArgs(CppArgs, { /* struct def */ });


}



fn main() { /* demo */ }
