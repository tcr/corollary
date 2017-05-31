use haskell_support::*;

use Language::C::Analysis::SemRep;

pub fn arithmeticConversion(__0: TypeName, __1: TypeName) -> Option<TypeName> {
    match (__0, __1) {
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
            Some(t1)
        },
        (TyIntegral(_), t2, __OP__, TyComplex(_)) => {
            Some(t2)
        },
        (TyFloating(t1), TyFloating(t2)) => {
            Some(TyFloating(floatConversion(t1, t2)))
        },
        (t1, __OP__, TyFloating(_), TyIntegral(_)) => {
            Some(t1)
        },
        (TyIntegral(_), t2, __OP__, TyFloating(_)) => {
            Some(t2)
        },
        (TyIntegral(t1), TyIntegral(t2)) => {
            Some(TyIntegral(intConversion(t1, t2)))
        },
        (TyEnum(_), TyEnum(_)) => {
            Some(TyIntegral(TyInt))
        },
        (TyEnum(_), t2) => {
            Some(t2)
        },
        (t1, TyEnum(_)) => {
            Some(t1)
        },
        (_, _) => {
            None
        },
    }
}

pub fn floatConversion() -> FloatType {
    max
}

pub fn intConversion(t1: IntType, t2: IntType) -> IntType {
    max(TyInt, (max(t1, t2)))
}

