// Original file: "TypeConversions.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::SemRep;

use analysis::sem_rep::*;

pub fn arithmeticConversion(_0: TypeName, _1: TypeName) -> Option<TypeName> {
    match (_0, _1) {
        (TyComplex(t1), TyComplex(t2)) => Some(TyComplex(floatConversion(t1, t2))),
        (TyComplex(t1), TyFloating(t2)) => Some(TyComplex(floatConversion(t1, t2))),
        (TyFloating(t1), TyComplex(t2)) => Some(TyComplex(floatConversion(t1, t2))),
        (t1, __OP__, TyComplex(_), TyIntegral(_)) => Some(t1),
        (TyIntegral(_), t2, __OP__, TyComplex(_)) => Some(t2),
        (TyFloating(t1), TyFloating(t2)) => Some(TyFloating(floatConversion(t1, t2))),
        (t1, __OP__, TyFloating(_), TyIntegral(_)) => Some(t1),
        (TyIntegral(_), t2, __OP__, TyFloating(_)) => Some(t2),
        (TyIntegral(t1), TyIntegral(t2)) => Some(TyIntegral(intConversion(t1, t2))),
        (TyEnum(_), TyEnum(_)) => Some(TyIntegral(TyInt)),
        (TyEnum(_), t2) => Some(t2),
        (t1, TyEnum(_)) => Some(t1),
        (_, _) => None,
    }
}

pub fn floatConversion() -> FloatType {
    max
}

pub fn intConversion(t1: IntType, t2: IntType) -> IntType {
    max(TyInt, (max(t1, t2)))
}
