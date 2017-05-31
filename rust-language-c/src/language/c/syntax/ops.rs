use haskell_support::*;

use Data::Generics;

#[derive(Clone, Debug, Eq, Ord)]
pub enum CAssignOp {
    CAssignOp,
    CMulAssOp,
    CDivAssOp,
    CRmdAssOp,
    CAddAssOp,
    CSubAssOp,
    CShlAssOp,
    CShrAssOp,
    CAndAssOp,
    CXorAssOp,
    COrAssOp
}
pub use self::CAssignOp::*;

#[derive(Clone, Debug, Eq, Ord)]
pub enum CBinaryOp {
    CMulOp,
    CDivOp,
    CRmdOp,
    CAddOp,
    CSubOp,
    CShlOp,
    CShrOp,
    CLeOp,
    CGrOp,
    CLeqOp,
    CGeqOp,
    CEqOp,
    CNeqOp,
    CAndOp,
    CXorOp,
    COrOp,
    CLndOp,
    CLorOp
}
pub use self::CBinaryOp::*;

#[derive(Clone, Debug, Eq, Ord)]
pub enum CUnaryOp {
    CPreIncOp,
    CPreDecOp,
    CPostIncOp,
    CPostDecOp,
    CAdrOp,
    CIndOp,
    CPlusOp,
    CMinOp,
    CCompOp,
    CNegOp
}
pub use self::CUnaryOp::*;

pub fn assignBinop(__0: CAssignOp) -> CBinaryOp {
    match (__0) {
        CAssignOp => {
            __error!("direct assignment has no binary operator".to_string())
        },
        CMulAssOp => {
            CMulOp
        },
        CDivAssOp => {
            CDivOp
        },
        CRmdAssOp => {
            CRmdOp
        },
        CAddAssOp => {
            CAddOp
        },
        CSubAssOp => {
            CSubOp
        },
        CShlAssOp => {
            CShlOp
        },
        CShrAssOp => {
            CShrOp
        },
        CAndAssOp => {
            CAndOp
        },
        CXorAssOp => {
            CXorOp
        },
        COrAssOp => {
            COrOp
        },
    }
}

pub fn isBitOp(op: CBinaryOp) -> bool {
    elem(op, vec![CShlOp, CShrOp, CAndOp, COrOp, CXorOp])
}

pub fn isCmpOp(op: CBinaryOp) -> bool {
    elem(op, vec![CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp])
}

pub fn isEffectfulOp(op: CUnaryOp) -> bool {
    elem(op, vec![CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp])
}

pub fn isLogicOp(op: CBinaryOp) -> bool {
    elem(op, vec![CLndOp, CLorOp])
}

pub fn isPtrOp(op: CBinaryOp) -> bool {
    elem(op, vec![CAddOp, CSubOp])
}

