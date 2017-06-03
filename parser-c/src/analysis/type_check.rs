// Original file: "TypeCheck.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Control::Monad;
// use Data::Either;
// use Data::Maybe;
// use Language::C::Data::Ident;
// use Language::C::Data::Node;
// use Language::C::Data::Position;
// use Language::C::Pretty;
// use Language::C::Syntax::AST;
// use Language::C::Syntax::Constants;
// use Language::C::Syntax::Ops;
// use Language::C::Analysis::Debug;
// use Language::C::Analysis::DefTable;
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::TravMonad;
// use Language::C::Analysis::TypeConversions;
// use Language::C::Analysis::TypeUtils;
// use Text::PrettyPrint::HughesPJ;

pub fn assignCompatible(_0: CAssignOp, _1: Type, _2: Type) -> Either<String, ()> {
    match (_0, _1, _2) {
        (CAssignOp, t1, t2) => {
            match (canonicalType(t1), canonicalType(t2)) {
                (DirectType(TyBuiltin(TyAny), _, _), _) => {
                    ()
                },
                (_, DirectType(TyBuiltin(TyAny), _, _)) => {
                    ()
                },
                (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) if isPointerType(t2_q) => { () }
                (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) if isPointerType(t1_q) => { () }
                (PtrType(_, _, _), t2_q) if isIntegralType(t2_q) => { () }
                (t1_q, t2_q) if (isPointerType(t1_q) && isPointerType(t2_q)) => { /*do*/ {
                    compatible((baseType(t1_q)), (baseType(t2_q)))
                } }
                (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) if (sueRef(c1) == sueRef(c2)) => { () }
                (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) => { fail(__op_addadd("incompatible compound types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) }
                (DirectType(TyBuiltin(TyVaList), _, _), DirectType(TyBuiltin(TyVaList), _, _)) => {
                    ()
                },
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) if isJust((arithmeticConversion(tn1, tn2))) => { () }
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) => { fail(__op_addadd("incompatible direct types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) }
                (t1_q, t2_q) => {
                    compatible(t1_q, t2_q)
                },
            }
        },
        (op, t1, t2) => {
            __op_rshift(binopType((assignBinop(op)), t1, t2), ())
        },
    }
}

pub fn assignCompatible_q(ni: NodeInfo, op: CAssignOp, t1: Type, t2: Type) -> m<()> {
    typeErrorOnLeft(ni, (assignCompatible(op, t1, t2)))
}

pub fn binopType(op: CBinaryOp, t1: Type, t2: Type) -> Either<String, Type> {
    match (op, canonicalType(t1), canonicalType(t2)) {
        (_, t1_q, t2_q) if isLogicOp(op) => { __op_rshift(checkScalar(t1_q), __op_rshift(checkScalar(t2_q), boolType)) }
        (_, t1_q, t2_q) if isCmpOp(op) => { match (t1_q, t2_q) {
            (DirectType(tn1, _, _), DirectType(tn2, _, _)) => {
                match arithmeticConversion(tn1, tn2) {
                    Some(_) => {
                        boolType
                    },
                    None => {
                        fail("incompatible arithmetic types in comparison".to_string())
                    },
                }
            },
            (PtrType(DirectType(TyVoid, _, _), _, _), _) if isPointerType(t2_q) => { boolType }
            (_, PtrType(DirectType(TyVoid, _, _), _, _)) if isPointerType(t1_q) => { boolType }
            (_, _) if (isPointerType(t1_q) && isIntegralType(t2_q)) => { boolType }
            (_, _) if (isIntegralType(t1_q) && isPointerType(t2_q)) => { boolType }
            (_, _) if (isPointerType(t1_q) && isPointerType(t2_q)) => { __op_rshift(compatible(t1_q, t2_q), boolType) }
            (_, _) => {
                fail("incompatible types in comparison".to_string())
            },
        } }
        (CSubOp, ArrayType(t1_q, _, _, _), ArrayType(t2_q, _, _, _)) => {
            __op_rshift(compatible(t1_q, t2_q), ptrDiffType)
        },
        (CSubOp, ArrayType(t1_q, _, _, _), PtrType(t2_q, _, _)) => {
            __op_rshift(compatible(t1_q, t2_q), ptrDiffType)
        },
        (CSubOp, PtrType(t1_q, _, _), ArrayType(t2_q, _, _, _)) => {
            __op_rshift(compatible(t1_q, t2_q), ptrDiffType)
        },
        (CSubOp, PtrType(t1_q, _, _), PtrType(t2_q, _, _)) => {
            __op_rshift(compatible(t1_q, t2_q), ptrDiffType)
        },
        (_, PtrType(_, _, _), t2_q) if (isPtrOp(op) && isIntegralType(t2_q)) => { t1 }
        (_, PtrType(_, _, _), t2_q) => { fail(__op_addadd("invalid pointer operation: ".to_string(), render((pretty(op))))) }
        (CAddOp, t1_q, PtrType(_, _, _)) if isIntegralType(t1_q) => { t2 }
        (_, ArrayType(_, _, _, _), t2_q) if (isPtrOp(op) && isIntegralType(t2_q)) => { t1 }
        (_, ArrayType(_, _, _, _), t2_q) => { fail(__op_addadd("invalid pointer operation: ".to_string(), render((pretty(op))))) }
        (CAddOp, t1_q, ArrayType(_, _, _, _)) if isIntegralType(t1_q) => { t2 }
        (_, DirectType(tn1, q1, a1), DirectType(tn2, q2, a2)) => {
            /*do*/ {
                when((isBitOp(op)), (__op_rshift(checkIntegral(t1), checkIntegral(t2))));
                match arithmeticConversion(tn1, tn2) {
                    Some(tn) => {
                        return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2))))
                    },
                    None => {
                        fail(render(__op_doc_conat(text("invalid binary operation:".to_string()), __op_doc_conat(pretty(t1), __op_doc_conat(pretty(op), pretty(t2))))))
                    },
                }
            }
        },
        (_, _, _) => {
            fail(render(__op_doc_conat(text("unhandled binary operation:".to_string()), __op_doc_conat(pretty(t1), __op_doc_conat(pretty(op), pretty(t2))))))
        },
    }
}

pub fn binopType_q(ni: NodeInfo, op: CBinaryOp, t1: Type, t2: Type) -> m<Type> {
    typeErrorOnLeft(ni, (binopType(op, t1, t2)))
}

pub fn castCompatible(t1: Type, t2: Type) -> Either<String, ()> {
    match (canonicalType(t1), canonicalType(t2)) {
        (DirectType(TyVoid, _, _), _) => {
            ()
        },
        (_, _) => {
            __op_rshift(checkScalar(t1), checkScalar(t2))
        },
    }
}

pub fn checkIntegral(t: Type) -> Either<String, ()> {
    /* Expr::Error */ Error
}

pub fn checkIntegral_q(ni: NodeInfo) -> m<()> {
    typeErrorOnLeft(ni, checkIntegral)
}

pub fn checkScalar(t: Type) -> Either<String, ()> {
    match canonicalType(t) {
        DirectType(_, _, _) => {
            ()
        },
        PtrType(_, _, _) => {
            ()
        },
        ArrayType(_, _, _, _) => {
            ()
        },
        t_q => {
            fail(__op_addadd("expected scalar type, got: ".to_string(), __op_addadd(pType(t), __op_addadd(" (".to_string(), __op_addadd(pType(t_q), ")".to_string())))))
        },
    }
}

pub fn checkScalar_q(ni: NodeInfo) -> m<()> {
    typeErrorOnLeft(ni, checkScalar)
}

pub fn compatible(t1: Type, t2: Type) -> Either<String, ()> {
    __op_rshift(compositeType(t1, t2), ())
}

pub fn compositeDeclAttrs(DeclAttrs(inl, stor, attrs1): DeclAttrs, DeclAttrs(_, _, attrs2): DeclAttrs) -> DeclAttrs {
    DeclAttrs(inl, stor, (mergeAttrs(attrs1, attrs2)))
}

pub fn compositeParamDecl(_0: ParamDecl, _1: ParamDecl) -> Either<String, ParamDecl> {
    match (_0, _1) {
        (ParamDecl(vd1, ni1), ParamDecl(vd2, _)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni1)
        },
        (AbstractParamDecl(vd1, _), ParamDecl(vd2, ni2)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni2)
        },
        (ParamDecl(vd1, ni1), AbstractParamDecl(vd2, _)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni1)
        },
        (AbstractParamDecl(vd1, ni1), AbstractParamDecl(vd2, _)) => {
            compositeParamDecl_q(AbstractParamDecl, vd1, vd2, ni1)
        },
    }
}

pub fn compositeParamDecl_q(f: fn(VarDecl) -> fn(NodeInfo) -> ParamDecl, VarDecl(n1, attrs1, t1): VarDecl, VarDecl(n2, attrs2, t2): VarDecl, dni: NodeInfo) -> Either<String, ParamDecl> {
    /*do*/ {
        let vd = compositeVarDecl((VarDecl(n1, attrs1, t1_q)), (VarDecl(n2, attrs2, t2_q)));

        return(f(vd, dni))
    }
}

pub fn compositeSize(_0: ArraySize, _1: ArraySize) -> Either<String, ArraySize> {
    match (_0, _1) {
        (UnknownArraySize(_), s2) => {
            s2
        },
        (s1, UnknownArraySize(_)) => {
            s1
        },
        (ArraySize(s1, e1), ArraySize(s2, e2)) => {
            /* Expr::Error */ Error
        },
    }
}

pub fn compositeType(_0: Type, _1: Type) -> Either<String, Type> {
    match (_0, _1) {
        (t1, DirectType(TyBuiltin(TyAny), _, _)) => {
            t1
        },
        (DirectType(TyBuiltin(TyAny), _, _), t2) => {
            t2
        },
        (t1, __OP__, DirectType(tn1, q1, a1), t2, __OP__, DirectType(tn2, q2, a2)) => {
            /*do*/ {
                let tn = match (tn1, tn2) {
                        (TyVoid, TyVoid) => {
                            TyVoid
                        },
                        (TyIntegral(_), TyEnum(_)) => {
                            tn1
                        },
                        (TyEnum(_), TyIntegral(_)) => {
                            tn2
                        },
                        (TyIntegral(i1), TyIntegral(i2)) => {
                            return(TyIntegral((intConversion(i1, i2))))
                        },
                        (TyFloating(f1), TyFloating(f2)) => {
                            return(TyFloating((floatConversion(f1, f2))))
                        },
                        (TyComplex(f1), TyComplex(f2)) => {
                            return(TyComplex((floatConversion(f1, f2))))
                        },
                        (TyComp(c1), TyComp(c2)) => {
                            /*do*/ {
                                when((__op_assign_div(sueRef(c1), sueRef(c2))))(fail(__op_addadd("incompatible composite types: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))));
                                tn1
                            }
                        },
                        (TyEnum(e1), TyEnum(e2)) => {
                            /*do*/ {
                                when((__op_assign_div(sueRef(e1), sueRef(e2))))(fail(__op_addadd("incompatible enumeration types: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))));
                                return(TyEnum(e1))
                            }
                        },
                        (TyBuiltin(TyVaList), TyBuiltin(TyVaList)) => {
                            return(TyBuiltin(TyVaList))
                        },
                        (TyBuiltin(_), TyBuiltin(_)) => {
                            fail(__op_addadd("incompatible builtin types: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2)))))
                        },
                        (_, _) => {
                            fail(__op_addadd("incompatible direct types: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2)))))
                        },
                    };

                return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2))))
            }
        },
        (PtrType(t1, q1, a1), PtrType(DirectType(TyVoid, _, _), q2, _)) => {
            return(PtrType(t1, (mergeTypeQuals(q1, q2)), a1))
        },
        (PtrType(DirectType(TyVoid, _, _), q1, _), PtrType(t2, q2, a2)) => {
            return(PtrType(t2, (mergeTypeQuals(q1, q2)), a2))
        },
        (PtrType(t1, q1, a1), t2) => {
            /* Expr::Error */ Error
        },
        (t1, PtrType(t2, q2, a2)) => {
            /* Expr::Error */ Error
        },
        (ArrayType(t1, sz1, q1, a1), t2) => {
            /* Expr::Error */ Error
        },
        (t1, ArrayType(t2, sz2, q2, a2)) => {
            /* Expr::Error */ Error
        },
        (ArrayType(t1, s1, q1, a1), ArrayType(t2, s2, q2, a2)) => {
            /*do*/ {
                let t = compositeType(t1, t2);

                let s = compositeSize(s1, s2);

                let quals = mergeTypeQuals(q1, q2);

                let attrs = mergeAttrs(a1, a2);

                (ArrayType(t, s, quals, attrs))
            }
        },
        (t1, t2) => {
            /* Expr::Error */ Error
        },
        (TypeDefType(tdr1, q1, a1), TypeDefType(tdr2, q2, a2)) => {
            match (tdr1, tdr2) {
                (TypeDefRef(i1, None, _), TypeDefRef(i2, _, _)) => {
                    doTypeDef(i1, i2, tdr1)
                },
                (TypeDefRef(i1, _, _), TypeDefRef(i2, None, _)) => {
                    doTypeDef(i1, i2, tdr2)
                },
                (TypeDefRef(_, Some(t1), _), TypeDefRef(_, Some(t2), _)) => {
                    compositeType(t1, t2)
                },
            }
        },
        (FunctionType(ft1, attrs1), FunctionType(ft2, attrs2)) => {
            match (ft1, ft2) {
                (FunType(rt1, args1, varargs1), FunType(rt2, args2, varargs2)) => {
                    /*do*/ {
                        let args = mapM((uncurry(compositeParamDecl)), (zip(args1, args2)));

                        when((__op_assign_div(varargs1, varargs2)))(fail("incompatible varargs declarations".to_string()));
                        doFunType(rt1, rt2, args, varargs1)
                    }
                },
                (FunType(rt1, args1, varargs1), FunTypeIncomplete(rt2)) => {
                    doFunType(rt1, rt2, args1, varargs1)
                },
                (FunTypeIncomplete(rt1), FunType(rt2, args2, varargs2)) => {
                    doFunType(rt1, rt2, args2, varargs2)
                },
                (FunTypeIncomplete(rt1), FunTypeIncomplete(rt2)) => {
                    /*do*/ {
                        let rt = compositeType(rt1, rt2);

                        (FunctionType((FunTypeIncomplete(rt)), (mergeAttrs(attrs1, attrs2))))
                    }
                },
            }
        },
        (t1, t2) => {
            fail(__op_addadd("incompatible types: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2)))))
        },
    }
}

pub fn compositeVarDecl(VarDecl(n1, attrs1, t1): VarDecl, VarDecl(_, attrs2, t2): VarDecl) -> Either<String, VarDecl> {
    /*do*/ {
        let t = compositeType(t1, t2);

        (VarDecl(n1, (compositeDeclAttrs(attrs1, attrs2)), t))
    }
}

pub fn conditionalType(t1: Type, t2: Type) -> Either<String, Type> {
    match (canonicalType(t1), canonicalType(t2)) {
        (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) if isPointerType(t2_q) => { t2 }
        (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) if isPointerType(t1_q) => { t1 }
        (ArrayType(t1_q, _, q1, a1), ArrayType(t2_q, _, q2, a2)) => {
            /*do*/ {
                let t = compositeType(t1_q, t2_q);

                return(ArrayType(t, (UnknownArraySize(false)), (mergeTypeQuals(q1, q2)), (mergeAttrs(a1, a2))))
            }
        },
        (t1_q(__OP__, DirectType(tn1, q1, a1)), t2_q(__OP__, DirectType(tn2, q2, a2))) => {
            match arithmeticConversion(tn1, tn2) {
                Some(tn) => {
                    return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2))))
                },
                None => {
                    compositeType(t1_q, t2_q)
                },
            }
        },
        (t1_q, t2_q) => {
            compositeType(t1_q, t2_q)
        },
    }
}

pub fn conditionalType_q(ni: NodeInfo, t1: Type, t2: Type) -> m<Type> {
    typeErrorOnLeft(ni)(conditionalType(t1, t2))
}

pub fn constType(_0: CConst) -> m<Type> {
    match (_0) {
        CIntConst(CInteger(_, _, flags), _) => {
            return(DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes))
        },
        CCharConst(CChar(_, true), _) => {
            return(DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes))
        },
        CCharConst(CChar(_, false), _) => {
            return(DirectType((TyIntegral(TyChar)), noTypeQuals, noAttributes))
        },
        CCharConst(CChars(_, _), _) => {
            return(DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes))
        },
        CFloatConst(CFloat(fs), _) => {
            return(DirectType((TyFloating((getFloatType(fs)))), noTypeQuals, noAttributes))
        },
        CStrConst(CString(chars, wide), ni) => {
            /*do*/ {
                let n = genName;

                let charType = /* Expr::Error */ Error;

                let ni_q = mkNodeInfo((posOf(ni)), n);

                let arraySize = ArraySize(true, (CConst((CIntConst((cInteger((toInteger((length(chars)))))), ni_q)))));

                return(ArrayType((DirectType((TyIntegral(charType)), noTypeQuals, noAttributes)), arraySize, noTypeQuals, vec![]))
            }
        },
    }
}

pub fn deepTypeAttrs(_0: Type) -> m<Attributes> {
    match (_0) {
        DirectType(TyComp(CompTypeRef(sue, _, ni)), _, attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        DirectType(TyEnum(EnumTypeRef(sue, ni)), _, attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        DirectType(_, _, attrs) => {
            attrs
        },
        PtrType(t, _, attrs) => {
            liftM((attrs(__op_addadd)), deepTypeAttrs(t))
        },
        ArrayType(t, _, _, attrs) => {
            liftM((attrs(__op_addadd)), deepTypeAttrs(t))
        },
        FunctionType(FunType(t, _, _), attrs) => {
            liftM((attrs(__op_addadd)), deepTypeAttrs(t))
        },
        FunctionType(FunTypeIncomplete(t), attrs) => {
            liftM((attrs(__op_addadd)), deepTypeAttrs(t))
        },
        TypeDefType(TypeDefRef(i, _, ni), _, attrs) => {
            liftM((attrs(__op_addadd)), typeDefAttrs(ni, i))
        },
    }
}

pub fn derefType(_0: Type) -> Either<String, Type> {
    match (_0) {
        PtrType(t, _, _) => {
            t
        },
        ArrayType(t, _, _, _) => {
            t
        },
        t => {
            match canonicalType(t) {
                PtrType(t_q, _, _) => {
                    t_q
                },
                ArrayType(t_q, _, _, _) => {
                    t_q
                },
                _ => {
                    fail(__op_addadd("dereferencing non-pointer: ".to_string(), pType(t)))
                },
            }
        },
    }
}

pub fn expandAnonymous(_0: NodeInfo, _1: (VarName, Type)) -> m<Vec<(Ident, Type)>> {
    match (_0, _1) {
        (ni, (NoName, DirectType(TyComp(ctr), _, _))) => {
            __op_bind(lookupSUE(ni, (sueRef(ctr))), tagMembers(ni))
        },
        (_, (NoName, _)) => {
            vec![]
        },
        (_, (VarName(n, _), t)) => {
            vec![(n, t)]
        },
    }
}

pub fn fieldType(ni: NodeInfo, m: Ident, t: Type) -> m<Type> {
    match canonicalType(t) {
        DirectType(TyComp(ctr), _, _) => {
            /*do*/ {
                let td = lookupSUE(ni, (sueRef(ctr)));

                let ms = tagMembers(ni, td);

                match lookup(m, ms) {
                    Some(ft) => {
                        ft
                    },
                    None => {
                        typeError(ni)(__op_addadd("field not found: ".to_string(), identToString(m)))
                    },
                }
            }
        },
        _t_q => {
            astError(ni)(__op_addadd("field of non-composite type: ".to_string(), __op_addadd(identToString(m), __op_addadd(", ".to_string(), pType(t)))))
        },
    }
}

pub fn lookupSUE(ni: NodeInfo, sue: SUERef) -> m<TagDef> {
    /*do*/ {
        let dt = getDefTable;

        match lookupTag(sue, dt) {
            Some(Right(td)) => {
                td
            },
            _ => {
                typeError(ni)(__op_addadd("unknown composite type: ".to_string(), (render(pretty))(sue)))
            },
        }
    }
}

pub fn mergeAttrs() -> Attributes {
    (__op_addadd)
}

pub fn notFound<a>(i: Ident) -> Either<String, a> {
    fail(__op_addadd("not found: ".to_string(), identToString(i)))
}

pub fn pType() -> String {
    render(pretty)
}

pub fn sizeEqual(_0: CExpr, _1: CExpr) -> bool {
    match (_0, _1) {
        (CConst(CIntConst(i1, _)), CConst(CIntConst(i2, _))) => {
            (i1 == i2)
        },
        (e1, e2) => {
            (nodeInfo(e1) == nodeInfo(e2))
        },
    }
}

pub fn sueAttrs(ni: NodeInfo, sue: SUERef) -> m<Attributes> {
    /*do*/ {
        let dt = getDefTable;

        match lookupTag(sue, dt) {
            None => {
                astError(ni)(__op_addadd("SUE not found: ".to_string(), render((pretty(sue)))))
            },
            Some(Left(_)) => {
                vec![]
            },
            Some(Right(CompDef(CompType(_, _, _, attrs, _)))) => {
                attrs
            },
            Some(Right(EnumDef(EnumType(_, _, attrs, _)))) => {
                attrs
            },
        }
    }
}

pub fn tagMembers(ni: NodeInfo, td: TagDef) -> m<Vec<(Ident, Type)>> {
    match td {
        CompDef(CompType(_, _, ms, _, _)) => {
            getMembers(ms)
        },
        EnumDef(EnumType(_, es, _, _)) => {
            getMembers(es)
        },
    }
}

pub fn typeDefAttrs(ni: NodeInfo, i: Ident) -> m<Attributes> {
    /*do*/ {
        let dt = getDefTable;

        match lookupIdent(i, dt) {
            None => {
                astError(ni)(__op_addadd("can\'t find typedef name: ".to_string(), identToString(i)))
            },
            Some(Left(TypeDef(_, t, attrs, _))) => {
                liftM((attrs(__op_addadd)), deepTypeAttrs(t))
            },
            Some(Right(_)) => {
                astError(ni)(__op_addadd("not a typedef name: ".to_string(), identToString(i)))
            },
        }
    }
}

pub fn typeError<a>() -> m<a> {
    astError
}

pub fn typeErrorOnLeft<a>(_0: NodeInfo, _1: Either<String, a>) -> m<a> {
    match (_0, _1) {
        (ni, Left(err)) => {
            typeError(ni, err)
        },
        (_, Right(v)) => {
            v
        },
    }
}

pub fn varAddrType(d: IdentDecl) -> Either<String, Type> {
    /*do*/ {
        match declStorage(d) {
            Auto(true) => {
                fail("address of register variable".to_string())
            },
            _ => {
                ()
            },
        };
        match t {
            ArrayType(_, _, q, a) => {
                return(PtrType(t, q, a))
            },
            _ => {
                return(simplePtr(t))
            },
        }
    }
}



