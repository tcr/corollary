// Original file: "TypeConversions.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::SemRep;

pub fn arithmeticConversion(_0: TypeName, _1: TypeName) -> Option<TypeName> {
    match (_0, _1) {
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
            Some(TyComplex(floatConversion(t1, t2)))
        },
        (_0, _1) => {
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



