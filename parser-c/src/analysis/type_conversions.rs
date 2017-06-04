// Original file: "TypeConversions.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::SemRep;

pub fn arithmeticConversion(_0: TypeName, _1: TypeName) -> Option<TypeName> {
    match (_0, _1) {
        (TyComplex(t1), TyComplex(t2)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (TyComplex(t1), TyFloating(t2)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (TyFloating(t1), TyComplex(t2)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (t1, __OP__, TyComplex(_), TyIntegral(_)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (TyIntegral(_), t2, __OP__, TyComplex(_)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (TyFloating(t1), TyFloating(t2)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (t1, __OP__, TyFloating(_), TyIntegral(_)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (TyIntegral(_), t2, __OP__, TyFloating(_)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (TyIntegral(t1), TyIntegral(t2)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (TyEnum(_), TyEnum(_)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (TyEnum(_), t2) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (t1, TyEnum(_)) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_, _) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
    }
}

pub fn floatConversion() -> FloatType {
    max
}

pub fn intConversion(t1: IntType, t2: IntType) -> IntType {
    max(TyInt, (max(t1, t2)))
}



