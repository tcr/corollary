// Original file: "AST.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

use data::node::*;
use data::ident::*;
use syntax::ops::*;
use syntax::constants::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Syntax::Constants;
// use Language::C::Syntax::Ops;
// use Language::C::Data::Ident;
// use Language::C::Data::Node;
// use Language::C::Data::Position;
// use Data::Generics;

pub type CTranslUnit = CTranslationUnit<NodeInfo>;

#[derive(Clone, Debug)]
//TODO derive CNode ,Functor, Annotated 
pub struct CTranslationUnit<A>(Vec<CExternalDeclaration<A>>, A);

pub type CExtDecl = CExternalDeclaration<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CExternalDeclaration<a> {
    CDeclExt(CDeclaration<a>),
    CFDefExt(CFunctionDef<a>),
    CAsmExt(CStringLiteral<a>, a),
}
pub use self::CExternalDeclaration::*;

pub type CFunDef = CFunctionDef<NodeInfo>;

#[derive(Clone, Debug)]
pub struct CFunctionDef<a>(pub Vec<CDeclarationSpecifier<a>>,
                           pub CDeclarator<a>,
                           pub Vec<CDeclaration<a>>,
                           pub CStatement<a>,
                           pub a);


pub type CDecl = CDeclaration<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CDeclaration<a> {
    CDecl(Vec<CDeclarationSpecifier<a>>,
          Vec<(Option<CDeclarator<a>>, Option<CInitializer<a>>, Option<CExpression<a>>)>,
          a),
    CStaticAssert(CExpression<a>, CStringLiteral<a>, a),
}
pub use self::CDeclaration::*;

pub type CDeclr = CDeclarator<NodeInfo>;

#[derive(Clone, Debug)]
pub struct CDeclarator<a>(pub Option<Ident>,
                          pub Vec<CDerivedDeclarator<a>>,
                          pub Option<CStringLiteral<a>>,
                          pub Vec<CAttribute<a>>,
                          pub a);


pub type CDerivedDeclr = CDerivedDeclarator<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CDerivedDeclarator<a> {
    CPtrDeclr(Vec<CTypeQualifier<a>>, a),
    CArrDeclr(Vec<CTypeQualifier<a>>, CArraySize<a>, a),
    CFunDeclr(Either<Vec<Ident>, (Vec<CDeclaration<a>>, bool)>, Vec<CAttribute<a>>, a),
}
pub use self::CDerivedDeclarator::*;

pub type CArrSize = CArraySize<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CArraySize<a> {
    CNoArrSize(bool),
    CArrSize(bool, CExpression<a>),
}
pub use self::CArraySize::*;

pub type CStat = CStatement<NodeInfo>;

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
    CFor(Either<Option<CExpression<a>>, CDeclaration<a>>,
         Option<CExpression<a>>,
         Option<CExpression<a>>,
         CStatement<a>,
         a),
    CGoto(Ident, a),
    CGotoPtr(CExpression<a>, a),
    CCont(a),
    CBreak(a),
    CReturn(Option<CExpression<a>>, a),
    CAsm(CAssemblyStatement<a>, a),
}
pub use self::CStatement::*;

pub type CAsmStmt = CAssemblyStatement<NodeInfo>;

#[derive(Clone, Debug)]
pub struct CAssemblyStatement<a>(pub Option<CTypeQualifier<a>>,
                                 pub CStringLiteral<a>,
                                 pub Vec<CAssemblyOperand<a>>,
                                 pub Vec<CAssemblyOperand<a>>,
                                 pub Vec<CStringLiteral<a>>,
                                 pub a);


pub type CAsmOperand = CAssemblyOperand<NodeInfo>;

#[derive(Clone, Debug)]
pub struct CAssemblyOperand<a>(pub Option<Ident>, pub CStringLiteral<a>, pub CExpression<a>, pub a);


pub type CBlockItem = CCompoundBlockItem<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CCompoundBlockItem<a> {
    CBlockStmt(CStatement<a>),
    CBlockDecl(CDeclaration<a>),
    CNestedFunDef(CFunctionDef<a>),
}
pub use self::CCompoundBlockItem::*;

pub type CDeclSpec = CDeclarationSpecifier<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CDeclarationSpecifier<a> {
    CStorageSpec(CStorageSpecifier<a>),
    CTypeSpec(CTypeSpecifier<a>),
    CTypeQual(CTypeQualifier<a>),
    CFunSpec(CFunctionSpecifier<a>),
    CAlignSpec(CAlignmentSpecifier<a>),
}
pub use self::CDeclarationSpecifier::*;

pub fn partitionDeclSpecs<a>()
    -> (Vec<CStorageSpecifier<a>>,
        Vec<CAttribute<a>>,
        Vec<CTypeQualifier<a>>,
        Vec<CTypeSpecifier<a>>,
        Vec<CFunctionSpecifier<a>>,
        Vec<CAlignmentSpecifier<a>>)
{

    let deals = |_0, _1| match (_0, _1) {
        (CStorageSpec(sp), (sts, ats, tqs, tss, fss, ass)) => {
            (__op_concat(sp, sts), ats, tqs, tss, fss, ass)
        }
        (CTypeQual(CAttrQual(attr)), (sts, ats, tqs, tss, fss, ass)) => {
            (sts, __op_concat(attr, ats), tqs, tss, fss, ass)
        }
        (CTypeQual(tq), (sts, ats, tqs, tss, fss, ass)) => {
            (sts, ats, __op_concat(tq, tqs), tss, fss, ass)
        }
        (CTypeSpec(ts), (sts, ats, tqs, tss, fss, ass)) => {
            (sts, ats, tqs, __op_concat(ts, tss), fss, ass)
        }
        (CFunSpec(fs), (sts, ats, tqs, tss, fss, ass)) => {
            (sts, ats, tqs, tss, __op_concat(fs, fss), ass)
        }
        (CAlignSpec(__as), (sts, ats, tqs, tss, fss, ass)) => {
            (sts, ats, tqs, tss, fss, __op_concat(__as, ass))
        }
    };

    __foldr!(deals, (vec![], vec![], vec![], vec![], vec![], vec![]))
}

pub type CStorageSpec = CStorageSpecifier<NodeInfo>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CStorageSpecifier<a> {
    CAuto(a),
    CRegister(a),
    CStatic(a),
    CExtern(a),
    CTypedef(a),
    CThread(a),
}
pub use self::CStorageSpecifier::*;

pub type CTypeSpec = CTypeSpecifier<NodeInfo>;

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
    CInt128Type(a),
    CSUType(CStructureUnion<a>, a),
    CEnumType(CEnumeration<a>, a),
    CTypeDef(Ident, a),
    CTypeOfExpr(CExpression<a>, a),
    CTypeOfType(CDeclaration<a>, a),
    CAtomicType(CDeclaration<a>, a),
}
pub use self::CTypeSpecifier::*;

pub fn isSUEDef<a>(_0: CTypeSpecifier<a>) -> bool {
    match (_0) {
        CSUType(CStructureUnion(_, _, Some(_), _, _), _) => true,
        CEnumType(CEnumeration(_, Some(_), _, _), _) => true,
        _ => true,
    }
}

pub type CTypeQual = CTypeQualifier<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CTypeQualifier<a> {
    CConstQual(a),
    CVolatQual(a),
    CRestrQual(a),
    CAtomicQual(a),
    CAttrQual(CAttribute<a>),
    CNullableQual(a),
    CNonnullQual(a),
}
pub use self::CTypeQualifier::*;

pub type CFunSpec = CFunctionSpecifier<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CFunctionSpecifier<a> {
    CInlineQual(a),
    CNoreturnQual(a),
}
pub use self::CFunctionSpecifier::*;

pub type CAlignSpec = CAlignmentSpecifier<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CAlignmentSpecifier<a> {
    CAlignAsType(CDeclaration<a>, a),
    CAlignAsExpr(CExpression<a>, a),
}
pub use self::CAlignmentSpecifier::*;

pub type CStructUnion = CStructureUnion<NodeInfo>;

#[derive(Clone, Debug)]
pub struct CStructureUnion<a>(pub CStructTag,
                              pub Option<Ident>,
                              pub Option<Vec<CDeclaration<a>>>,
                              pub Vec<CAttribute<a>>,
                              pub a);


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CStructTag {
    CStructTag,
    CUnionTag,
}
pub use self::CStructTag::*;

pub type CEnum = CEnumeration<NodeInfo>;

#[derive(Clone, Debug)]
pub struct CEnumeration<a>(pub Option<Ident>,
                           pub Option<Vec<(Ident, Option<CExpression<a>>)>>,
                           pub Vec<CAttribute<a>>,
                           pub a);


pub type CInit = CInitializer<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CInitializer<a> {
    CInitExpr(CExpression<a>, a),
    CInitList(CInitializerList<a>, a),
}
pub use self::CInitializer::*;

pub fn fmapInitList<a, b>(_f: fn(a) -> b) -> CInitializerList<b> {
    //TODO
    // __map!((|(desigs, initializer)| { (fmap((fmap(_f)), desigs), fmap(_f, initializer)) }))
}

pub type CInitList = CInitializerList<NodeInfo>;

pub type CInitializerList<a> = Vec<(Vec<CPartDesignator<a>>, CInitializer<a>)>;

pub type CDesignator = CPartDesignator<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CPartDesignator<a> {
    CArrDesig(CExpression<a>, a),
    CMemberDesig(Ident, a),
    CRangeDesig(CExpression<a>, CExpression<a>, a),
}
pub use self::CPartDesignator::*;

pub type CAttr = CAttribute<NodeInfo>;

#[derive(Clone, Debug)]
pub struct CAttribute<a>(pub Ident, pub Vec<CExpression<a>>, pub a);


pub type CExpr = CExpression<NodeInfo>;

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
    CGenericSelection(CExpression<a>, Vec<(Option<CDeclaration<a>>, CExpression<a>)>, a),
    CStatExpr(CStatement<a>, a),
    CLabAddrExpr(Ident, a),
    CBuiltinExpr(CBuiltinThing<a>),
}
pub use self::CExpression::*;

pub type CBuiltin = CBuiltinThing<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CBuiltinThing<a> {
    CBuiltinVaArg(CExpression<a>, CDeclaration<a>, a),
    CBuiltinOffsetOf(CDeclaration<a>, Vec<CPartDesignator<a>>, a),
    CBuiltinTypesCompatible(CDeclaration<a>, CDeclaration<a>, a),
}
pub use self::CBuiltinThing::*;

pub type CConst = CConstant<NodeInfo>;

#[derive(Clone, Debug)]
pub enum CConstant<a> {
    CIntConst(CInteger, a),
    CCharConst(CChar, a),
    CFloatConst(CFloat, a),
    CStrConst(CString, a),
}
pub use self::CConstant::*;

pub type CStrLit = CStringLiteral<NodeInfo>;

#[derive(Clone, Debug)]
pub struct CStringLiteral<a>(pub CString, pub a);


pub fn cstringOfLit<a>(CStringLiteral(cstr, _): CStringLiteral<a>) -> CString {
    cstr
}

pub fn liftStrLit<a>(CStringLiteral(__str, at): CStringLiteral<a>) -> CConstant<a> {
    CStrConst(__str, at)
}
