pub mod Language_C_Analysis_ConstEval {
    use haskell_support::*;
    struct MachineDesc(MachineDesc<{ /* type record */ }>);

    pub fn alignofType(__0: MachineDesc, __1: n, __2: Type) -> m<Integer> {
        match (__0, __1, __2) {
            (md, _, DirectType(TyVoid, _, _)) => {
                return(voidAlign(md))
            },
            (md, _, DirectType(TyIntegral(it), _, _)) => {
                return(iAlign(md, it))
            },
            (md, _, DirectType(TyFloating(ft), _, _)) => {
                return(fAlign(md, ft))
            },
            (md, _, DirectType(TyComplex(ft), _, _)) => {
                return(fAlign(md, ft))
            },
            (md, _, DirectType(TyEnum(_), _, _)) => {
                return(iAlign(md, TyInt))
            },
            (md, _, DirectType(TyBuiltin(b), _, _)) => {
                return(builtinAlign(md, b))
            },
            (md, _, PtrType(_, _, _)) => {
                return(ptrAlign(md))
            },
            (md, n, ArrayType(bt, UnknownArraySize(_), _, _)) => {
                return(ptrAlign(md))
            },
            (md, n, ArrayType(bt, ArraySize(_, sz), _, _)) => {
                alignofType(md, n, bt)
            },
            (md, n, TypeDefType(TypeDefRef(_, Some(t), _), _, _)) => {
                alignofType(md, n, t)
            },
            (_, n, t) => {
                astError((nodeInfo(n)))(__op_addadd("can\'t find alignment of type: ".to_string(), (render(pretty))(t)))
            },
        }
    }

    pub fn boolValue(__0: CExpr) -> Option<bool> {
        match (__0) {
            CConst(CIntConst(i, _)) => {
                Some(/=(getCInteger(i), 0))
            },
            CConst(CCharConst(c, _)) => {
                Some(/=(getCCharAsInt(c), 0))
            },
            CConst(CStrConst(_, _)) => {
                Some(true)
            },
            _ => {
                None
            },
        }
    }

    pub fn compSize(md: MachineDesc, ctr: CompTypeRef) -> m<Integer> {
        /* do */ {
            let dt = getDefTable;
            match lookupTag((sueRef(ctr)), dt) {
                Some(Left(_)) => {
                    astError((nodeInfo(ctr)), "composite declared but not defined".to_string())
                },
                Some(Right(CompDef(CompType(_, tag, ms, _, ni)))) => {
                    /* do */ {
                        {
                            let ts = map(declType, ms);
                        };
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

    pub fn constEval(__0: MachineDesc, __1: Map::Map<Ident, CExpr>, __2: CExpr) -> m<CExpr> {
        match (__0, __1, __2) {
            (md, env, CCond(e1, me2, e3, ni)) => {
                /* do */ {
                    let e1_q = constEval(md, env, e1);
                    let me2_q = maybe((None), (liftM(|e| { Some }, constEval(md, env, e))), me2);
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
            (md, env, e, @, CBinary(op, e1, e2, ni)) => {
                /* do */ {
                    let e1_q = constEval(md, env, e1);
                    let e2_q = constEval(md, env, e2);
                    let t = tExpr(vec![], RValue, e);
                    let bytes = liftM(fromIntegral, sizeofType(md, e, t));
                    match (intValue(e1_q), intValue(e2_q)) {
                        (Some(i1), Some(i2)) => {
                            intExpr(ni, (withWordBytes(bytes, (intOp(op, i1, i2)))))
                        },
                        (_, _) => {
                            return(CBinary(op, e1_q, e2_q, ni))
                        },
                    }
                }
            },
            (md, env, CUnary(op, e, ni)) => {
                /* do */ {
                    let e_q = constEval(md, env, e);
                    let t = tExpr(vec![], RValue, e);
                    let bytes = liftM(fromIntegral, sizeofType(md, e, t));
                    match intValue(e_q) {
                        Some(i) => {
                            match intUnOp(op, i) {
                                Some(i_q) => {
                                    intExpr(ni, (withWordBytes(bytes, i_q)))
                                },
                                None => {
                                    astError(ni, "invalid unary operator applied to constant".to_string())
                                },
                            }
                        },
                        None => {
                            return(CUnary(op, e_q, ni))
                        },
                    }
                }
            },
            (md, env, CCast(d, e, ni)) => {
                /* do */ {
                    let e_q = constEval(md, env, e);
                    let t = analyseTypeDecl(d);
                    let bytes = liftM(fromIntegral, sizeofType(md, d, t));
                    match intValue(e_q) {
                        Some(i) => {
                            intExpr(ni, (withWordBytes(bytes, i)))
                        },
                        None => {
                            return(CCast(d, e_q, ni))
                        },
                    }
                }
            },
            (md, _, CSizeofExpr(e, ni)) => {
                /* do */ {
                    let t = tExpr(vec![], RValue, e);
                    let sz = sizeofType(md, e, t);
                    intExpr(ni, sz)
                }
            },
            (md, _, CSizeofType(d, ni)) => {
                /* do */ {
                    let t = analyseTypeDecl(d);
                    let sz = sizeofType(md, d, t);
                    intExpr(ni, sz)
                }
            },
            (md, _, CAlignofExpr(e, ni)) => {
                /* do */ {
                    let t = tExpr(vec![], RValue, e);
                    let sz = alignofType(md, e, t);
                    intExpr(ni, sz)
                }
            },
            (md, _, CAlignofType(d, ni)) => {
                /* do */ {
                    let t = analyseTypeDecl(d);
                    let sz = alignofType(md, d, t);
                    intExpr(ni, sz)
                }
            },
            (md, env, e, @, CVar(i, _)) => {
                /* do */ {
                    let t = tExpr(vec![], RValue, e);
                    match derefTypeDef(t) {
                        DirectType(TyEnum(etr), _, _) => {
                            /* do */ {
                                let dt = getDefTable;
                                match lookupTag((sueRef(etr)), dt) {
                                    Some(Right(EnumDef(EnumType(_, es, _, _)))) => {
                                        /* do */ {
                                            let env_q = foldM(enumConst, env, es);
                                            return(fromMaybe(e)(Map::lookup(i, env_q)))
                                        }
                                    },
                                    _ => {
                                        e
                                    },
                                }
                            }
                        },
                        _ => {
                            e
                        },
                    }
                }
            },
            (_, _, e) => {
                e
            },
        }
    }

    pub fn intExpr(n: n, i: Integer) -> m<CExpr> {
        __op_bind(genName, |name| { return }(CConst(CIntConst((cInteger(i)), (mkNodeInfo((posOf(n)), name))))))
    }

    pub fn intOp(__0: CBinaryOp, __1: Integer, __2: Integer) -> Integer {
        match (__0, __1, __2) {
            (CAddOp, i1, i2) => {
                +(i1, i2)
            },
            (CSubOp, i1, i2) => {
                -(i1, i2)
            },
            (CMulOp, i1, i2) => {
                (i1 * i2)
            },
            (CDivOp, i1, i2) => {
                div(i1, i2)
            },
            (CRmdOp, i1, i2) => {
                mod(i1, i2)
            },
            (CShlOp, i1, i2) => {
                shiftL(i1, fromInteger(i2))
            },
            (CShrOp, i1, i2) => {
                shiftR(i1, fromInteger(i2))
            },
            (CLeOp, i1, i2) => {
                toInteger(fromEnum(<(i1, i2)))
            },
            (CGrOp, i1, i2) => {
                toInteger(fromEnum(>(i1, i2)))
            },
            (CLeqOp, i1, i2) => {
                toInteger(fromEnum(<=(i1, i2)))
            },
            (CGeqOp, i1, i2) => {
                toInteger(fromEnum(>=(i1, i2)))
            },
            (CEqOp, i1, i2) => {
                toInteger(fromEnum((i1 == i2)))
            },
            (CNeqOp, i1, i2) => {
                toInteger(fromEnum(/=(i1, i2)))
            },
            (CAndOp, i1, i2) => {
                .&.(i1, i2)
            },
            (CXorOp, i1, i2) => {
                xor(i1, i2)
            },
            (COrOp, i1, i2) => {
                .|.(i1, i2)
            },
            (CLndOp, i1, i2) => {
                toInteger(fromEnum(((/=(i1, 0)) && (/=(i2, 0)))))
            },
            (CLorOp, i1, i2) => {
                toInteger(fromEnum(((/=(i1, 0)) || (/=(i2, 0)))))
            },
        }
    }

    pub fn intUnOp(__0: CUnaryOp, __1: Integer) -> Option<Integer> {
        match (__0, __1) {
            (CPlusOp, i) => {
                Some(i)
            },
            (CMinOp, i) => {
                Some(-(i))
            },
            (CCompOp, i) => {
                Some(complement(i))
            },
            (CNegOp, i) => {
                Some(toInteger(fromEnum((i == 0))))
            },
            (_, _) => {
                None
            },
        }
    }

    pub fn intValue(__0: CExpr) -> Option<Integer> {
        match (__0) {
            CConst(CIntConst(i, _)) => {
                Some(getCInteger(i))
            },
            CConst(CCharConst(c, _)) => {
                Some(getCCharAsInt(c))
            },
            _ => {
                None
            },
        }
    }

    pub fn sizeofType(__0: MachineDesc, __1: n, __2: Type) -> m<Integer> {
        match (__0, __1, __2) {
            (md, _, DirectType(TyVoid, _, _)) => {
                return(voidSize(md))
            },
            (md, _, DirectType(TyIntegral(it), _, _)) => {
                return(iSize(md, it))
            },
            (md, _, DirectType(TyFloating(ft), _, _)) => {
                return(fSize(md, ft))
            },
            (md, _, DirectType(TyComplex(ft), _, _)) => {
                return((2 * fSize(md, ft)))
            },
            (md, _, DirectType(TyComp(ctr), _, _)) => {
                compSize(md, ctr)
            },
            (md, _, DirectType(TyEnum(_), _, _)) => {
                return(iSize(md, TyInt))
            },
            (md, _, DirectType(TyBuiltin(b), _, _)) => {
                return(builtinSize(md, b))
            },
            (md, _, PtrType(_, _, _)) => {
                return(ptrSize(md))
            },
            (md, n, ArrayType(bt, UnknownArraySize(_), _, _)) => {
                return(ptrSize(md))
            },
            (md, n, ArrayType(bt, ArraySize(_, sz), _, _)) => {
                /* do */ {
                    let sz_q = constEval(md, Map::empty, sz);
                    match sz_q {
                        CConst(CIntConst(i, _)) => {
                            /* do */ {
                                let s = sizeofType(md, n, bt);
                                return((getCInteger(i) * s))
                            }
                        },
                        _ => {
                            return(ptrSize(md))
                        },
                    }
                }
            },
            (md, n, TypeDefType(TypeDefRef(_, Some(t), _), _, _)) => {
                sizeofType(md, n, t)
            },
            (md, _, FunctionType(_, _)) => {
                return(ptrSize(md))
            },
            (_, n, t) => {
                astError((nodeInfo(n)))(__op_addadd("can\'t find size of type: ".to_string(), (render(pretty))(t)))
            },
        }
    }

    pub fn withWordBytes(bytes: isize, n: Integer) -> Integer {
        rem(n, (shiftL(1, (shiftL(bytes, 3)))))
    }

}

