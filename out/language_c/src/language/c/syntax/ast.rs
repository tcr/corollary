use haskell_support::*;

#[derive(Clone, Debug)]
struct CTranslationUnit<a>(CTranslUnit<Vec<CExternalDeclaration<a>>, a>);

#[derive(Clone, Debug)]
pub enum CExternalDeclaration<a> {
    CDeclExt(CDeclaration<a>),
    CFDefExt(CFunctionDef<a>),
    CAsmExt(CStringLiteral<a>, a)
}
pub use self::CExternalDeclaration::*;

#[derive(Clone, Debug)]
struct CFunctionDef<a>(CFunDef<Vec<CDeclarationSpecifier<a>>, CDeclarator<a>, Vec<CDeclaration<a>>, CStatement<a>, a>);

#[derive(Clone, Debug)]
struct CDeclaration<a>(CDecl<Vec<CDeclarationSpecifier<a>>, Vec<(Option<CDeclarator<a>>, Option<CInitializer<a>>, Option<CExpression<a>>)>, a>);

#[derive(Clone, Debug)]
struct CDeclarator<a>(CDeclr<Option<Ident>, Vec<CDerivedDeclarator<a>>, Option<CStringLiteral<a>>, Vec<CAttribute<a>>, a>);

#[derive(Clone, Debug)]
pub enum CDerivedDeclarator<a> {
    CPtrDeclr(Vec<CTypeQualifier<a>>, a),
    CArrDeclr(Vec<CTypeQualifier<a>>, CArraySize<a>, a),
    CFunDeclr(Either<Vec<Ident>, (Vec<CDeclaration<a>>, bool)>, Vec<CAttribute<a>>, a)
}
pub use self::CDerivedDeclarator::*;

#[derive(Clone, Debug)]
pub enum CArraySize<a> {
    CNoArrSize(bool),
    CArrSize(bool, CExpression<a>)
}
pub use self::CArraySize::*;

#[derive(Clone, Debug)]
pub enum CStatement<a> {
    CLabel(Ident, CStatement<a>, Vec<CAttribute<a>>, a),
    CCase(CExpression<a>, CStatement<a>, a),
    CCases(CExpression<a>, CExpression<a>, CStatement<a>, a),
    CDefault(CStatement<a>, a),
    CExpr(Option<CExpression<a>>, a),
    CCompound(Vec<Ident>, Vec<CCompoundBlockItem<a>>, a),
    CIf(CExpression<a>, CStatement<a>, Option<CStatement<a>>, a),
    CSwitch(CExpression<a>, CStatement<a>, a),
    CWhile(CExpression<a>, CStatement<a>, bool, a),
    CFor(Either<Option<CExpression<a>>, CDeclaration<a>>, Option<CExpression<a>>, Option<CExpression<a>>, CStatement<a>, a),
    CGoto(Ident, a),
    CGotoPtr(CExpression<a>, a),
    CCont(a),
    CBreak(a),
    CReturn(Option<CExpression<a>>, a),
    CAsm(CAssemblyStatement<a>, a)
}
pub use self::CStatement::*;

#[derive(Clone, Debug)]
struct CAssemblyStatement<a>(CAsmStmt<Option<CTypeQualifier<a>>, CStringLiteral<a>, Vec<CAssemblyOperand<a>>, Vec<CAssemblyOperand<a>>, Vec<CStringLiteral<a>>, a>);

#[derive(Clone, Debug)]
struct CAssemblyOperand<a>(CAsmOperand<Option<Ident>, CStringLiteral<a>, CExpression<a>, a>);

#[derive(Clone, Debug)]
pub enum CCompoundBlockItem<a> {
    CBlockStmt(CStatement<a>),
    CBlockDecl(CDeclaration<a>),
    CNestedFunDef(CFunctionDef<a>)
}
pub use self::CCompoundBlockItem::*;

#[derive(Clone, Debug)]
pub enum CDeclarationSpecifier<a> {
    CStorageSpec(CStorageSpecifier<a>),
    CTypeSpec(CTypeSpecifier<a>),
    CTypeQual(CTypeQualifier<a>)
}
pub use self::CDeclarationSpecifier::*;

#[derive(Clone, Debug, Eq, Ord)]
pub enum CStorageSpecifier<a> {
    CAuto(a),
    CRegister(a),
    CStatic(a),
    CExtern(a),
    CTypedef(a),
    CThread(a)
}
pub use self::CStorageSpecifier::*;

#[derive(Clone, Debug)]
pub enum CTypeSpecifier<a> {
    CVoidType(a),
    CCharType(a),
    CShortType(a),
    CIntType(a),
    CLongType(a),
    CFloatType(a),
    CDoubleType(a),
    CSignedType(a),
    CUnsigType(a),
    CBoolType(a),
    CComplexType(a),
    CSUType(CStructureUnion<a>, a),
    CEnumType(CEnumeration<a>, a),
    CTypeDef(Ident, a),
    CTypeOfExpr(CExpression<a>, a),
    CTypeOfType(CDeclaration<a>, a)
}
pub use self::CTypeSpecifier::*;

#[derive(Clone, Debug)]
pub enum CTypeQualifier<a> {
    CConstQual(a),
    CVolatQual(a),
    CRestrQual(a),
    CInlineQual(a),
    CAttrQual(CAttribute<a>)
}
pub use self::CTypeQualifier::*;

#[derive(Clone, Debug)]
struct CStructureUnion<a>(CStruct<CStructTag, Option<Ident>, Option<Vec<CDeclaration<a>>>, Vec<CAttribute<a>>, a>);

#[derive(Clone, Debug, Eq)]
pub enum CStructTag {
    CStructTag,
    CUnionTag
}
pub use self::CStructTag::*;

#[derive(Clone, Debug)]
struct CEnumeration<a>(CEnum<Option<Ident>, Option<Vec<(Ident, Option<CExpression<a>>)>>, Vec<CAttribute<a>>, a>);

#[derive(Clone, Debug)]
pub enum CInitializer<a> {
    CInitExpr(CExpression<a>, a),
    CInitList(CInitializerList<a>, a)
}
pub use self::CInitializer::*;

#[derive(Clone, Debug)]
pub enum CPartDesignator<a> {
    CArrDesig(CExpression<a>, a),
    CMemberDesig(Ident, a),
    CRangeDesig(CExpression<a>, CExpression<a>, a)
}
pub use self::CPartDesignator::*;

#[derive(Clone, Debug)]
struct CAttribute<a>(CAttr<Ident, Vec<CExpression<a>>, a>);

#[derive(Clone, Debug)]
pub enum CExpression<a> {
    CComma(Vec<CExpression<a>>, a),
    CAssign(CAssignOp, CExpression<a>, CExpression<a>, a),
    CCond(CExpression<a>, Option<CExpression<a>>, CExpression<a>, a),
    CBinary(CBinaryOp, CExpression<a>, CExpression<a>, a),
    CCast(CDeclaration<a>, CExpression<a>, a),
    CUnary(CUnaryOp, CExpression<a>, a),
    CSizeofExpr(CExpression<a>, a),
    CSizeofType(CDeclaration<a>, a),
    CAlignofExpr(CExpression<a>, a),
    CAlignofType(CDeclaration<a>, a),
    CComplexReal(CExpression<a>, a),
    CComplexImag(CExpression<a>, a),
    CIndex(CExpression<a>, CExpression<a>, a),
    CCall(CExpression<a>, Vec<CExpression<a>>, a),
    CMember(CExpression<a>, Ident, bool, a),
    CVar(Ident, a),
    CConst(CConstant<a>),
    CCompoundLit(CDeclaration<a>, CInitializerList<a>, a),
    CStatExpr(CStatement<a>, a),
    CLabAddrExpr(Ident, a),
    CBuiltinExpr(CBuiltinThing<a>)
}
pub use self::CExpression::*;

#[derive(Clone, Debug)]
pub enum CBuiltinThing<a> {
    CBuiltinVaArg(CExpression<a>, CDeclaration<a>, a),
    CBuiltinOffsetOf(CDeclaration<a>, Vec<CPartDesignator<a>>, a),
    CBuiltinTypesCompatible(CDeclaration<a>, CDeclaration<a>, a)
}
pub use self::CBuiltinThing::*;

#[derive(Clone, Debug)]
pub enum CConstant<a> {
    CIntConst(CInteger, a),
    CCharConst(CChar, a),
    CFloatConst(CFloat, a),
    CStrConst(CString, a)
}
pub use self::CConstant::*;

#[derive(Clone, Debug)]
struct CStringLiteral<a>(CStrLit<CString, a>);

pub fn cstringOfLit((CStrLit(cstr, _)): CStringLiteral<a>) -> CString {
    cstr
}

pub fn fmapInitList(_f: fn(a) -> b) -> CInitializerList<b> {
    map((|(desigs, initializer)| { (fmap((fmap(_f)), desigs), fmap(_f, initializer)) }))
}

pub fn isSUEDef(__0: CTypeSpecifier<a>) -> bool {
    match (__0) {
        CSUType(CStruct(_, _, Some(_), _, _), _) => {
            true
        },
        CEnumType(CEnum(_, Some(_), _, _), _) => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn liftStrLit((CStrLit(__str, at)): CStringLiteral<a>) -> CConstant<a> {
    CStrConst(__str, at)
}

pub fn partitionDeclSpecs() -> (Vec<CStorageSpecifier<a>>, Vec<CAttribute<a>>, Vec<CTypeQualifier<a>>, Vec<CTypeSpecifier<a>>, bool) {
    foldr(deals, (vec![], vec![], vec![], vec![], false))
}

