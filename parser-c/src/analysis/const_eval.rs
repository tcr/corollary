// Original file: "ConstEval.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Control::Monad;
// use Data::Bits;
// use Data::Maybe;
// use Data::Map;
// use Language::C::Syntax::AST;
// use Language::C::Syntax::Constants;
// use Language::C::Analysis::AstAnalysis;
// use tExpr;
// use Language::C::Analysis::Debug;
// use Language::C::Analysis::DeclAnalysis;
// use Language::C::Analysis::DefTable;
// use Language::C::Data;
// use Language::C::Pretty;
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::TravMonad;
// use Language::C::Analysis::TypeUtils;
// use Text::PrettyPrint::HughesPJ;

pub struct MachineDesc{
    iSize: fn(IntType) -> Integer,
    fSize: fn(FloatType) -> Integer,
    builtinSize: fn(BuiltinType) -> Integer,
    ptrSize: Integer,
    voidSize: Integer,
    iAlign: fn(IntType) -> Integer,
    fAlign: fn(FloatType) -> Integer,
    builtinAlign: fn(BuiltinType) -> Integer,
    ptrAlign: Integer,
    voidAlign: Integer
}
fn iSize(a: MachineDesc) -> fn(IntType) -> Integer { a.iSize }
fn fSize(a: MachineDesc) -> fn(FloatType) -> Integer { a.fSize }
fn builtinSize(a: MachineDesc) -> fn(BuiltinType) -> Integer { a.builtinSize }
fn ptrSize(a: MachineDesc) -> Integer { a.ptrSize }
fn voidSize(a: MachineDesc) -> Integer { a.voidSize }
fn iAlign(a: MachineDesc) -> fn(IntType) -> Integer { a.iAlign }
fn fAlign(a: MachineDesc) -> fn(FloatType) -> Integer { a.fAlign }
fn builtinAlign(a: MachineDesc) -> fn(BuiltinType) -> Integer { a.builtinAlign }
fn ptrAlign(a: MachineDesc) -> Integer { a.ptrAlign }
fn voidAlign(a: MachineDesc) -> Integer { a.voidAlign }

pub fn alignofType(_0: MachineDesc, _1: n, _2: Type) -> m<Integer> {
    match (_0, _1, _2) {
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
        (_0, _1, _2) => {
            return(voidAlign(md))
        },
    }
}

pub fn boolValue(_0: CExpr) -> Option<bool> {
    match (_0) {
        _0 => {
            Some(__op_assign_div(getCInteger(i), 0))
        },
        _0 => {
            Some(__op_assign_div(getCInteger(i), 0))
        },
        _0 => {
            Some(__op_assign_div(getCInteger(i), 0))
        },
        _0 => {
            Some(__op_assign_div(getCInteger(i), 0))
        },
    }
}

pub fn compSize(md: MachineDesc, ctr: CompTypeRef) -> m<Integer> {
    /*do*/ {
        let dt = getDefTable;

        match lookupTag((sueRef(ctr)), dt) {
            Some(Left(_)) => {
                astError((nodeInfo(ctr)), "composite declared but not defined".to_string())
            },
            Some(Right(CompDef(CompType(_, tag, ms, _, ni)))) => {
                /*do*/ {
                    let ts = __map!(declType, ms);

                    let sizes = mapM((sizeofType(md, ni)), ts);

                    match tag {
                        StructTag => {
                            return(sum(sizes))
                        },
                        UnionTag => {
                            return(maximum(sizes))
                        },
                    }
                }
            },
            Some(Right(EnumDef(_))) => {
                return(iSize(md, TyInt))
            },
            None => {
                astError((nodeInfo(ctr)), "unknown composite".to_string())
            },
        }
    }
}

pub fn constEval(_0: MachineDesc, _1: Map::Map<Ident, CExpr>, _2: CExpr) -> m<CExpr> {
    match (_0, _1, _2) {
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
    }
}

pub fn intExpr(n: n, i: Integer) -> m<CExpr> {
    __op_bind(genName, |name| { return(CConst(CIntConst((cInteger(i)), (mkNodeInfo((posOf(n)), name))))) })
}

pub fn intOp(_0: CBinaryOp, _1: Integer, _2: Integer) -> Integer {
    match (_0, _1, _2) {
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
        (_0, _1, _2) => {
            (i1 + i2)
        },
    }
}

pub fn intUnOp(_0: CUnaryOp, _1: Integer) -> Option<Integer> {
    match (_0, _1) {
        (_0, _1) => {
            Some(i)
        },
        (_0, _1) => {
            Some(i)
        },
        (_0, _1) => {
            Some(i)
        },
        (_0, _1) => {
            Some(i)
        },
        (_0, _1) => {
            Some(i)
        },
    }
}

pub fn intValue(_0: CExpr) -> Option<Integer> {
    match (_0) {
        _0 => {
            Some(getCInteger(i))
        },
        _0 => {
            Some(getCInteger(i))
        },
        _0 => {
            Some(getCInteger(i))
        },
    }
}

pub fn sizeofType(_0: MachineDesc, _1: n, _2: Type) -> m<Integer> {
    match (_0, _1, _2) {
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
        (_0, _1, _2) => {
            return(voidSize(md))
        },
    }
}

pub fn withWordBytes(bytes: isize, n: Integer) -> Integer {
    rem(n, (shiftL(1, (shiftL(bytes, 3)))))
}



