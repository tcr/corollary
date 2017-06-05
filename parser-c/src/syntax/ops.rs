// Original file: "Ops.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Generics;

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

pub fn assignBinop(_0: CAssignOp) -> CBinaryOp {
    match (_0) {
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

pub fn isCmpOp(op: CBinaryOp) -> bool {
    elem(op, vec![CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp])
}

pub fn isPtrOp(op: CBinaryOp) -> bool {
    elem(op, vec![CAddOp, CSubOp])
}

pub fn isBitOp(op: CBinaryOp) -> bool {
    elem(op, vec![CShlOp, CShrOp, CAndOp, COrOp, CXorOp])
}

pub fn isLogicOp(op: CBinaryOp) -> bool {
    elem(op, vec![CLndOp, CLorOp])
}

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

pub fn isEffectfulOp(op: CUnaryOp) -> bool {
    elem(op, vec![CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp])
}



