#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::SemRep;
// use Language::C::Syntax::Constants;

pub fn baseType<a>(_0: Type) -> Type {
    match (_0) {
        PtrType(t, _, _) => {
            t
        },
        ArrayType(t, _, _, _) => {
            t
        },
        _ => {
            __error!("base of non-pointer type".to_string())
        },
    }
}

pub fn boolType<a>() -> Type {
    integral(TyInt)
}

pub fn canonicalType<a>(t: Type) -> Type {
    match deepDerefTypeDef(t) {
        FunctionType(ft, attrs) => {
            simplePtr((FunctionType(ft, attrs)))
        },
        t_q => {
            t_q
        },
    }
}

pub fn charPtr<a>() -> Type {
    simplePtr((integral(TyChar)))
}

pub fn constCharPtr<a>() -> Type {
    constPtr((integral(TyChar)))
}

pub fn constPtr<a>(t: Type) -> Type {
    PtrType(t, (TypeQuals(true, false, false)), vec![])
}

pub fn constVoidPtr<a>() -> Type {
    constPtr(voidType)
}

pub fn deepDerefTypeDef<a>(_0: Type) -> Type {
    match (_0) {
        PtrType(t, quals, attrs) => {
            PtrType((deepDerefTypeDef(t)), quals, attrs)
        },
        ArrayType(t, size, quals, attrs) => {
            ArrayType((deepDerefTypeDef(t)), size, quals, attrs)
        },
        FunctionType(FunType(rt, params, varargs), attrs) => {
            FunctionType((FunType((deepDerefTypeDef(rt)), params, varargs)), attrs)
        },
        FunctionType(FunTypeIncomplete(rt), attrs) => {
            FunctionType((FunTypeIncomplete((deepDerefTypeDef(rt)))), attrs)
        },
        TypeDefType(TypeDefRef(_, Some(t), _), q, a) => {
            (typeAttrsUpd((mergeAttributes(a)), typeQualsUpd((mergeTypeQuals(q)))))((deepDerefTypeDef(t)))
        },
        t => {
            t
        },
    }
}

pub fn derefTypeDef<a>(_0: Type) -> Type {
    match (_0) {
        TypeDefType(TypeDefRef(_, Some(t), _), q, a) => {
            (typeAttrsUpd((mergeAttributes(a)), typeQualsUpd((mergeTypeQuals(q)))))((derefTypeDef(t)))
        },
        ty => {
            ty
        },
    }
}

pub fn floating<a>(ty: FloatType) -> Type {
    DirectType((TyFloating(ty)), noTypeQuals, noAttributes)
}

pub fn getFloatType<a>(fs: String) -> FloatType {
    /* Expr::Error */ Error
}

pub fn getIntType<a>(flags: Flags<CIntFlag>) -> IntType {
    /* Expr::Error */ Error
}

pub fn integral<a>(ty: IntType) -> Type {
    DirectType((TyIntegral(ty)), noTypeQuals, noAttributes)
}

pub fn isFloatingType<a>(_0: Type) -> bool {
    match (_0) {
        DirectType(TyFloating(_), _, _) => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isFunctionType<a>(ty: Type) -> bool {
    match ty {
        TypeDefType(TypeDefRef(_, Some(actual_ty), _), _, _) => {
            isFunctionType(actual_ty)
        },
        TypeDefType(_, _, _) => {
            __error!("isFunctionType: unresolved typeDef".to_string())
        },
        FunctionType(_, _) => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isIntegralType<a>(_0: Type) -> bool {
    match (_0) {
        DirectType(TyIntegral(_), _, _) => {
            true
        },
        DirectType(TyEnum(_), _, _) => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isPointerType<a>(_0: Type) -> bool {
    match (_0) {
        PtrType(_, _, _) => {
            true
        },
        ArrayType(_, _, _, _) => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isScalarType<a>(t: Type) -> bool {
    (isIntegralType(t) || (isPointerType(t) || isFloatingType(t)))
}

pub fn ptrDiffType<a>() -> Type {
    integral(TyInt)
}

pub fn simplePtr<a>(t: Type) -> Type {
    PtrType(t, noTypeQuals, vec![])
}

pub fn size_tType<a>() -> Type {
    integral(TyInt)
}

pub fn stringType<a>() -> Type {
    ArrayType((DirectType((TyIntegral(TyChar)), (TypeQuals(true, false, false)), noAttributes)), (UnknownArraySize(false)), noTypeQuals, vec![])
}

pub fn testFlags<a>(flags: Vec<f>, fi: Flags<f>) -> bool {
    and(__map!(((flip(testFlag))(fi)), flags))
}

pub fn typeAttrs<a>(_0: Type) -> Attributes {
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
        TypeDefType(TypeDefRef(_, None, _), _, a) => {
            a
        },
        TypeDefType(TypeDefRef(_, Some(t), _), _, a) => {
            mergeAttributes(a, (typeAttrs(t)))
        },
    }
}

pub fn typeAttrsUpd<a>(f: fn(Attributes) -> Attributes, ty: Type) -> Type {
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

pub fn typeQuals<a>(_0: Type) -> TypeQuals {
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
            noTypeQuals
        },
        TypeDefType(TypeDefRef(_, None, _), q, _) => {
            q
        },
        TypeDefType(TypeDefRef(_, Some(t), _), q, _) => {
            mergeTypeQuals(q, (typeQuals(t)))
        },
    }
}

pub fn typeQualsUpd<a>(f: fn(TypeQuals) -> TypeQuals, ty: Type) -> Type {
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

pub fn valistType<a>() -> Type {
    DirectType((TyBuiltin(TyVaList)), noTypeQuals, noAttributes)
}

pub fn voidPtr<a>() -> Type {
    simplePtr(voidType)
}

pub fn voidType<a>() -> Type {
    DirectType(TyVoid, noTypeQuals, noAttributes)
}



