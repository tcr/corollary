// Original file: "TypeUtils.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::SemRep;
// use Language::C::Data::Node;
// use CNode;
// use Language::C::Syntax::AST;
// use CExpression;
// use Language::C::Syntax::Constants;

pub fn baseType(_0: Type) -> Type {
    match (_0) {
        PtrType(t, _, _) => {
            t
        },
        ArrayType(t, _, _, _) => {
            t
        },
        _ => {
            t
        },
    }
}

pub fn boolType() -> Type {
    integral(TyInt)
}

pub fn canonicalType(t: Type) -> Type {
    match deepDerefTypeDef(t) {
        FunctionType(ft, attrs) => {
            simplePtr((FunctionType(ft, attrs)))
        },
        t_q => {
            t_q
        },
    }
}

pub fn charPtr() -> Type {
    simplePtr((integral(TyChar)))
}

pub fn constCharPtr() -> Type {
    constPtr((integral(TyChar)))
}

pub fn constPtr(t: Type) -> Type {
    PtrType(t, (noTypeQuals {
            constant: true
        }), vec![])
}

pub fn constVoidPtr() -> Type {
    constPtr(voidType)
}

pub fn deepDerefTypeDef(_0: Type) -> Type {
    match (_0) {
        PtrType(t, quals, attrs) => {
            PtrType((deepDerefTypeDef(t)), quals, attrs)
        },
        ArrayType(t, size, quals, attrs) => {
            PtrType((deepDerefTypeDef(t)), quals, attrs)
        },
        FunctionType(FunType(rt, params, varargs), attrs) => {
            PtrType((deepDerefTypeDef(t)), quals, attrs)
        },
        FunctionType(FunTypeIncomplete(rt), attrs) => {
            PtrType((deepDerefTypeDef(t)), quals, attrs)
        },
        TypeDefType(TypeDefRef(_, t, _), q, a) => {
            PtrType((deepDerefTypeDef(t)), quals, attrs)
        },
        t => {
            PtrType((deepDerefTypeDef(t)), quals, attrs)
        },
    }
}

pub fn derefTypeDef(_0: Type) -> Type {
    match (_0) {
        TypeDefType(TypeDefRef(_, t, _), q, a) => {
            (typeAttrsUpd((mergeAttributes(a)), typeQualsUpd((mergeTypeQuals(q)))))((derefTypeDef(t)))
        },
        ty => {
            (typeAttrsUpd((mergeAttributes(a)), typeQualsUpd((mergeTypeQuals(q)))))((derefTypeDef(t)))
        },
    }
}

pub fn floating(ty: FloatType) -> Type {
    DirectType((TyFloating(ty)), noTypeQuals, noAttributes)
}

pub fn integral(ty: IntType) -> Type {
    DirectType((TyIntegral(ty)), noTypeQuals, noAttributes)
}

pub fn isFloatingType(_0: Type) -> bool {
    match (_0) {
        DirectType(TyFloating(_), _, _) => {
            true
        },
        _ => {
            true
        },
    }
}

pub fn isFunctionType(ty: Type) -> bool {
    match ty {
        TypeDefType(TypeDefRef(_, actual_ty, _), _, _) => {
            isFunctionType(actual_ty)
        },
        FunctionType(_, _) => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isIntegralType(_0: Type) -> bool {
    match (_0) {
        DirectType(TyIntegral(_), _, _) => {
            true
        },
        DirectType(TyEnum(_), _, _) => {
            true
        },
        _ => {
            true
        },
    }
}

pub fn isPointerType(_0: Type) -> bool {
    match (_0) {
        PtrType(_, _, _) => {
            true
        },
        ArrayType(_, _, _, _) => {
            true
        },
        _ => {
            true
        },
    }
}

pub fn isScalarType(t: Type) -> bool {
    (isIntegralType(t) || (isPointerType(t) || isFloatingType(t)))
}

pub fn isVariablyModifiedType(t: Type) -> bool {
    match derefTypeDef(t) {
        TypeDefType {

        } => {
            __error!("impossible: derefTypeDef t returned a TypeDefType".to_string())
        },
        DirectType {

        } => {
            false
        },
        PtrType(ptr_ty, _, _) => {
            isVariablyModifiedType(ptr_ty)
        },
        ArrayType(_, sz, _, _) => {
            isVariableArraySize(sz)
        },
        FunctionType {

        } => {
            false
        },
    }
}

pub fn ptrDiffType() -> Type {
    integral(TyInt)
}

pub fn sameArraySize(_0: ArraySize, _1: ArraySize) -> bool {
    match (_0, _1) {
        (UnknownArraySize(isStar1), UnknownArraySize(isStar2)) => {
            (isStar1 == isStar2)
        },
        (ArraySize(s1, e1), ArraySize(s2, e2)) => {
            (isStar1 == isStar2)
        },
        (_, _) => {
            (isStar1 == isStar2)
        },
    }
}

pub fn sameBuiltinType(_0: BuiltinType, _1: BuiltinType) -> bool {
    match (_0, _1) {
        (TyVaList, TyVaList) => {
            true
        },
        (TyAny, TyAny) => {
            true
        },
        (_, _) => {
            true
        },
    }
}

pub fn sameCompTypeRef(CompTypeRef(sue1, kind1, _): CompTypeRef, CompTypeRef(sue2, kind2, _): CompTypeRef) -> bool {
    (sue1 == (sue2 && (kind1 == kind2)))
}

pub fn sameEnumTypeRef(EnumTypeRef(sue1, _): EnumTypeRef, EnumTypeRef(sue2, _): EnumTypeRef) -> bool {
    (sue1 == sue2)
}

pub fn sameFunType(_0: FunType, _1: FunType) -> bool {
    match (_0, _1) {
        (FunType(rt1, params1, isVar1), FunType(rt2, params2, isVar2)) => {
            (sameType(rt1, rt2) && (sameParamDecls(params1, params2) && (isVar1 == isVar2)))
        },
        (FunTypeIncomplete(rt1), FunTypeIncomplete(rt2)) => {
            (sameType(rt1, rt2) && (sameParamDecls(params1, params2) && (isVar1 == isVar2)))
        },
        (_, _) => {
            (sameType(rt1, rt2) && (sameParamDecls(params1, params2) && (isVar1 == isVar2)))
        },
    }
}

pub fn sameQuals(TypeQuals {

    }: TypeQuals, TypeQuals {

    }: TypeQuals) -> bool {
    (c1 == (c2 && (v1 == (v2 && (r1 == r2)))))
}

pub fn sameType(t1: Type, t2: Type) -> bool {
    (not(((isVariablyModifiedType(t1) || isVariablyModifiedType(t2)))) && sameType_q)
}

pub fn sameTypeName(t1: TypeName, t2: TypeName) -> bool {
    match (t1, t2) {
        (TyVoid, TyVoid) => {
            true
        },
        (TyIntegral(i1), TyIntegral(i2)) => {
            (i1 == i2)
        },
        (TyFloating(f1), TyFloating(f2)) => {
            (f1 == f2)
        },
        (TyComplex(f1), TyComplex(f2)) => {
            (f1 == f2)
        },
        (TyComp(ctr1), TyComp(ctr2)) => {
            sameCompTypeRef(ctr1, ctr2)
        },
        (TyEnum(etr1), TyEnum(etr2)) => {
            sameEnumTypeRef(etr1, etr2)
        },
        (TyBuiltin(b1), TyBuiltin(b2)) => {
            sameBuiltinType(b1, b2)
        },
        _ => {
            false
        },
    }
}

pub fn simplePtr(t: Type) -> Type {
    PtrType(t, noTypeQuals, vec![])
}

pub fn size_tType() -> Type {
    integral(TyInt)
}

pub fn stringType() -> Type {
    ArrayType((DirectType((TyIntegral(TyChar)), (noTypeQuals {
                constant: true
            }), noAttributes)), (UnknownArraySize(false)), noTypeQuals, vec![])
}

pub fn testFlags(flags: Vec<f>, fi: Flags<f>) -> bool {
    all((testFlag(fi)), flags)
}

pub fn typeAttrs(_0: Type) -> Attributes {
    match (_0) {
        DirectType(_, _, a) => {
            a
        },
        PtrType(_, _, a) => {
            a
        },
        ArrayType(_, _, _, a) => {
            a
        },
        FunctionType(_, a) => {
            a
        },
        TypeDefType(TypeDefRef(_, t, _), _, a) => {
            a
        },
    }
}

pub fn typeAttrsUpd(f: fn(Attributes) -> Attributes, ty: Type) -> Type {
    match ty {
        DirectType(ty_name, ty_quals, ty_attrs) => {
            DirectType(ty_name, ty_quals, (f(ty_attrs)))
        },
        PtrType(ty_inner, ty_quals, ty_attrs) => {
            PtrType(ty_inner, ty_quals, (f(ty_attrs)))
        },
        ArrayType(ty_inner, sz, ty_quals, ty_attrs) => {
            ArrayType(ty_inner, sz, ty_quals, (f(ty_attrs)))
        },
        FunctionType(ty_inner, ty_attrs) => {
            FunctionType(ty_inner, (f(ty_attrs)))
        },
        TypeDefType(ty_ref, ty_quals, ty_attrs) => {
            TypeDefType(ty_ref, ty_quals, (f(ty_attrs)))
        },
    }
}

pub fn typeQuals(_0: Type) -> TypeQuals {
    match (_0) {
        DirectType(_, q, _) => {
            q
        },
        PtrType(_, q, _) => {
            q
        },
        ArrayType(_, _, q, _) => {
            q
        },
        FunctionType(_, _) => {
            q
        },
        TypeDefType(TypeDefRef(_, t, _), q, _) => {
            q
        },
    }
}

pub fn typeQualsUpd(f: fn(TypeQuals) -> TypeQuals, ty: Type) -> Type {
    match ty {
        DirectType(ty_name, ty_quals, ty_attrs) => {
            DirectType(ty_name, (f(ty_quals)), ty_attrs)
        },
        PtrType(ty_inner, ty_quals, ty_attrs) => {
            PtrType(ty_inner, (f(ty_quals)), ty_attrs)
        },
        ArrayType(ty_inner, sz, ty_quals, ty_attrs) => {
            ArrayType(ty_inner, sz, (f(ty_quals)), ty_attrs)
        },
        FunctionType(ty_inner, ty_attrs) => {
            FunctionType(ty_inner, ty_attrs)
        },
        TypeDefType(ty_ref, ty_quals, ty_attrs) => {
            TypeDefType(ty_ref, (f(ty_quals)), ty_attrs)
        },
    }
}

pub fn uint16_tType() -> Type {
    integral(TyUShort)
}

pub fn uint32_tType() -> Type {
    integral(TyUInt)
}

pub fn uint64_tType() -> Type {
    integral(TyULLong)
}

pub fn valistType() -> Type {
    DirectType((TyBuiltin(TyVaList)), noTypeQuals, noAttributes)
}

pub fn voidPtr() -> Type {
    simplePtr(voidType)
}

pub fn voidType() -> Type {
    DirectType(TyVoid, noTypeQuals, noAttributes)
}



