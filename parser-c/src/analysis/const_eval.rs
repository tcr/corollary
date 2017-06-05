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

pub fn intExpr(n: n, i: Integer) -> m<CExpr> {
    __op_bind(genName, |name| { CConst(CIntConst((cInteger(i)), (mkNodeInfo((posOf(n)), name)))) })
}

pub fn sizeofType(_0: MachineDesc, _1: n, _2: Type) -> m<Integer> {
    match (_0, _1, _2) {
        (md, _, DirectType(TyVoid, _, _)) => {
            voidSize(md)
        },
        (md, _, DirectType(TyIntegral(it), _, _)) => {
            voidSize(md)
        },
        (md, _, DirectType(TyFloating(ft), _, _)) => {
            voidSize(md)
        },
        (md, _, DirectType(TyComplex(ft), _, _)) => {
            voidSize(md)
        },
        (md, _, DirectType(TyComp(ctr), _, _)) => {
            voidSize(md)
        },
        (md, _, DirectType(TyEnum(_), _, _)) => {
            voidSize(md)
        },
        (md, _, DirectType(TyBuiltin(b), _, _)) => {
            voidSize(md)
        },
        (md, _, PtrType(_, _, _)) => {
            voidSize(md)
        },
        (md, _, ArrayType(_, UnknownArraySize(_), _, _)) => {
            voidSize(md)
        },
        (md, n, ArrayType(bt, ArraySize(_, sz), _, _)) => {
            voidSize(md)
        },
        (md, n, TypeDefType(TypeDefRef(_, t, _), _, _)) => {
            voidSize(md)
        },
        (md, _, FunctionType(_, _)) => {
            voidSize(md)
        },
    }
}

pub fn alignofType(_0: MachineDesc, _1: n, _2: Type) -> m<Integer> {
    match (_0, _1, _2) {
        (md, _, DirectType(TyVoid, _, _)) => {
            voidAlign(md)
        },
        (md, _, DirectType(TyIntegral(it), _, _)) => {
            voidAlign(md)
        },
        (md, _, DirectType(TyFloating(ft), _, _)) => {
            voidAlign(md)
        },
        (md, _, DirectType(TyComplex(ft), _, _)) => {
            voidAlign(md)
        },
        (md, _, DirectType(TyEnum(_), _, _)) => {
            voidAlign(md)
        },
        (md, _, DirectType(TyBuiltin(b), _, _)) => {
            voidAlign(md)
        },
        (md, _, PtrType(_, _, _)) => {
            voidAlign(md)
        },
        (md, _, ArrayType(_, UnknownArraySize(_), _, _)) => {
            voidAlign(md)
        },
        (md, n, ArrayType(bt, ArraySize(_, _), _, _)) => {
            voidAlign(md)
        },
        (md, n, TypeDefType(TypeDefRef(_, t, _), _, _)) => {
            voidAlign(md)
        },
        (_, n, t) => {
            voidAlign(md)
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
                            sum(sizes)
                        },
                        UnionTag => {
                            maximum(sizes)
                        },
                    }
                }
            },
            Some(Right(EnumDef(_))) => {
                iSize(md, TyInt)
            },
            None => {
                astError((nodeInfo(ctr)), "unknown composite".to_string())
            },
        }
    }
}

pub fn intOp(_0: CBinaryOp, _1: Integer, _2: Integer) -> Integer {
    match (_0, _1, _2) {
        (CAddOp, i1, i2) => {
            (i1 + i2)
        },
        (CSubOp, i1, i2) => {
            (i1 + i2)
        },
        (CMulOp, i1, i2) => {
            (i1 + i2)
        },
        (CDivOp, i1, i2) => {
            (i1 + i2)
        },
        (CRmdOp, i1, i2) => {
            (i1 + i2)
        },
        (CShlOp, i1, i2) => {
            (i1 + i2)
        },
        (CShrOp, i1, i2) => {
            (i1 + i2)
        },
        (CLeOp, i1, i2) => {
            (i1 + i2)
        },
        (CGrOp, i1, i2) => {
            (i1 + i2)
        },
        (CLeqOp, i1, i2) => {
            (i1 + i2)
        },
        (CGeqOp, i1, i2) => {
            (i1 + i2)
        },
        (CEqOp, i1, i2) => {
            (i1 + i2)
        },
        (CNeqOp, i1, i2) => {
            (i1 + i2)
        },
        (CAndOp, i1, i2) => {
            (i1 + i2)
        },
        (CXorOp, i1, i2) => {
            (i1 + i2)
        },
        (COrOp, i1, i2) => {
            (i1 + i2)
        },
        (CLndOp, i1, i2) => {
            (i1 + i2)
        },
        (CLorOp, i1, i2) => {
            (i1 + i2)
        },
    }
}

pub fn intUnOp(_0: CUnaryOp, _1: Integer) -> Option<Integer> {
    match (_0, _1) {
        (CPlusOp, i) => {
            Some(i)
        },
        (CMinOp, i) => {
            Some(i)
        },
        (CCompOp, i) => {
            Some(i)
        },
        (CNegOp, i) => {
            Some(i)
        },
        (_, _) => {
            Some(i)
        },
    }
}

pub fn withWordBytes(bytes: isize, n: Integer) -> Integer {
    rem(n, (shiftL(1, (shiftL(bytes, 3)))))
}

pub fn boolValue(_0: CExpr) -> Option<bool> {
    match (_0) {
        CConst(CIntConst(i, _)) => {
            Some(__op_assign_div(getCInteger(i), 0))
        },
        CConst(CCharConst(c, _)) => {
            Some(__op_assign_div(getCInteger(i), 0))
        },
        CConst(CStrConst(_, _)) => {
            Some(__op_assign_div(getCInteger(i), 0))
        },
        _ => {
            Some(__op_assign_div(getCInteger(i), 0))
        },
    }
}

pub fn intValue(_0: CExpr) -> Option<Integer> {
    match (_0) {
        CConst(CIntConst(i, _)) => {
            Some(getCInteger(i))
        },
        CConst(CCharConst(c, _)) => {
            Some(getCInteger(i))
        },
        _ => {
            Some(getCInteger(i))
        },
    }
}

pub fn constEval(_0: MachineDesc, _1: Map::Map<Ident, CExpr>, _2: CExpr) -> m<CExpr> {
    match (_0, _1, _2) {
        (md, env, CCond(e1, me2, e3, ni)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (md, env, e, __OP__, CBinary(op, e1, e2, ni)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (md, env, CUnary(op, e, ni)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (md, env, CCast(d, e, ni)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (md, _, CSizeofExpr(e, ni)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (md, _, CSizeofType(d, ni)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (md, _, CAlignofExpr(e, ni)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (md, _, CAlignofType(d, ni)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (_, env, e, __OP__, CVar(i, _)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (md, env, e, __OP__, CVar(i, _)) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
        (_, _, e) => {
            /*do*/ {
                let e1_q = constEval(md, env, e1);

                let me2_q = maybe((None), (|e| { liftM(Some, constEval(md, env, e)) }), me2);

                let e3_q = constEval(md, env, e3);

                match boolValue(e1_q) {
                    Some(true) => {
                        fromMaybe(e1_q, me2_q)
                    },
                    Some(false) => {
                        e3_q
                    },
                    None => {
                        CCond(e1_q, me2_q, e3_q, ni)
                    },
                }
            }
        },
    }
}



