use haskell_support::*;

pub fn exportArraySize(__0: ArraySize) -> CArrSize {
    match (__0) {
        ArraySize(__static, e) => {
            CArrSize(__static, e)
        },
        UnknownArraySize(complete) => {
            CNoArrSize(complete)
        },
    }
}

pub fn exportAttrs() -> Vec<CAttr> {
    map(exportAttr)
}

pub fn exportCompType(CompType(sue_ref, comp_tag, members, attrs, node_info): CompType) -> Vec<CTypeSpec> {
    vec![CSUType(comp, ni)]
}

pub fn exportCompTypeDecl(ty: CompTypeRef) -> Vec<CTypeSpec> {
    vec![CSUType((exportComp(ty)), ni)]
}

pub fn exportCompTypeRef(CompType(sue_ref, com_tag, _, _, node_info): CompType) -> Vec<CTypeSpec> {
    exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
}

pub fn exportComplexType(ty: FloatType) -> Vec<CTypeSpec> {
    __op_concat((CComplexType(ni)), exportFloatType(ty))
}

pub fn exportDeclAttrs(DeclAttrs(inline, storage, attrs): DeclAttrs) -> Vec<CDeclSpec> {
    __op_addadd((__TODO_if(inline, then, vec![CTypeQual((CInlineQual(ni)))], __TODO_else, vec![])), __op_addadd(map((CStorageSpec), (exportStorage(storage))), map((CTypeQual(CAttrQual)), (exportAttrs(attrs)))))
}

pub fn exportDeclr(other_specs: Vec<CDeclSpec>, ty: Type, attrs: Attributes, name: VarName) -> (Vec<CDeclSpec>, CDeclr) {
    (__op_addadd(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
}

pub fn exportEnumType(EnumType(sue_ref, enumerators, attrs, node_info): EnumType) -> Vec<CTypeSpec> {
    vec![CEnumType(__enum, ni)]
}

pub fn exportEnumTypeDecl(ty: EnumTypeRef) -> Vec<CTypeSpec> {
    vec![CEnumType((exportEnum(ty)), ni)]
}

pub fn exportEnumTypeRef(EnumType(sue_ref, _, _, node_info): EnumType) -> Vec<CTypeSpec> {
    exportEnumTypeDecl((EnumTypeRef(sue_ref, node_info)))
}

pub fn exportFloatType(ty: FloatType) -> Vec<CTypeSpec> {
    match ty {
        TyFloat => {
            vec![CFloatType(ni)]
        },
        TyDouble => {
            vec![CDoubleType(ni)]
        },
        TyLDouble => {
            vec![CLongType(ni), CDoubleType(ni)]
        },
    }
}

pub fn exportIntType(ty: IntType) -> Vec<CTypeSpec> {
    match ty {
        TyBool => {
            vec![CBoolType(ni)]
        },
        TyChar => {
            vec![CCharType(ni)]
        },
        TySChar => {
            vec![CSignedType(ni), CCharType(ni)]
        },
        TyUChar => {
            vec![CUnsigType(ni), CCharType(ni)]
        },
        TyShort => {
            vec![CShortType(ni)]
        },
        TyUShort => {
            vec![CUnsigType(ni), CShortType(ni)]
        },
        TyInt => {
            vec![CIntType(ni)]
        },
        TyUInt => {
            vec![CUnsigType(ni), CIntType(ni)]
        },
        TyLong => {
            vec![CLongType(ni)]
        },
        TyULong => {
            vec![CUnsigType(ni), CLongType(ni)]
        },
        TyLLong => {
            vec![CLongType(ni), CLongType(ni)]
        },
        TyULLong => {
            vec![CUnsigType(ni), CLongType(ni), CLongType(ni)]
        },
    }
}

pub fn exportMemberDecl(__0: MemberDecl) -> CDecl {
    match (__0) {
        AnonBitField(ty, expr, node_info) => {
            CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(None, None, Some(expr))], node_info)
        },
        MemberDecl(vardecl, bitfieldsz, node_info) => {
            {
                let (specs, declarator) = exportVarDecl(vardecl);

            CDecl(specs, vec![(Some(declarator), None, bitfieldsz)], node_info)            }
        },
    }
}

pub fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
    {
        let (specs, declr) = exportVarDecl((getVarDecl(paramdecl)));

    CDecl(specs, vec![(Some(declr), None, None)], (nodeInfo(paramdecl)))    }
}

pub fn exportSUERef() -> Option<Ident> {
    Some(internalIdent(show))
}

pub fn exportStorage(__0: Storage) -> Vec<CStorageSpec> {
    match (__0) {
        NoStorage => {
            vec![]
        },
        Auto(reg) => {
            __TODO_if(reg, then, vec![CRegister(ni)], __TODO_else, vec![])
        },
        Static(InternalLinkage, thread_local) => {
            threadLocal(thread_local, vec![CStatic(ni)])
        },
        Static(ExternalLinkage, thread_local) => {
            threadLocal(thread_local, vec![CExtern(ni)])
        },
        Static(NoLinkage, _) => {
            __error!("impossible storage: static without linkage".to_string())
        },
        FunLinkage(InternalLinkage) => {
            vec![CStatic(ni)]
        },
        FunLinkage(ExternalLinkage) => {
            vec![]
        },
        FunLinkage(NoLinkage) => {
            __error!("impossible storage: function without linkage".to_string())
        },
    }
}

pub fn exportType(ty: Type) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {
    exportTy(vec![], ty)
}

pub fn exportTypeDecl(ty: Type) -> CDecl {
    CDecl(declspecs, declrs, ni)
}

pub fn exportTypeDef(TypeDef(ident, ty, attrs, node_info): TypeDef) -> CDecl {
    CDecl((__op_concat(CStorageSpec((CTypedef(ni))), declspecs)), vec![declr], node_info)
}

pub fn exportTypeQuals(quals: TypeQuals) -> Vec<CTypeQual> {
    mapMaybe(select, vec![(constant, CConstQual(ni)), (volatile, CVolatQual(ni)), (restrict, CRestrQual(ni))])
}

pub fn exportTypeQualsAttrs(tyqs: TypeQuals, attrs: Attributes) -> Vec<CTypeQual> {
    (__op_addadd(exportTypeQuals(tyqs), map(CAttrQual, (exportAttrs(attrs)))))
}

pub fn exportTypeSpec(tyname: TypeName) -> Vec<CTypeSpec> {
    match tyname {
        TyVoid => {
            vec![CVoidType(ni)]
        },
        TyIntegral | ity => {
            exportIntType(ity)
        },
        TyFloating | fty => {
            exportFloatType(fty)
        },
        TyComplex | fty => {
            exportComplexType(fty)
        },
        TyComp | comp => {
            exportCompTypeDecl(comp)
        },
        TyEnum | __enum => {
            exportEnumTypeDecl(__enum)
        },
        TyBuiltin | TyVaList => {
            vec![CTypeDef((internalIdent("va_list".to_string())), ni)]
        },
        TyBuiltin | TyAny => {
            vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)]
        },
    }
}

pub fn exportVarDecl(VarDecl(name, attrs, ty): VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
    exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
}

pub fn fromDirectType(__0: Type) -> TypeName {
    match (__0) {
        DirectType(ty, _, _) => {
            ty
        },
        TypeDefType(TypeDefRef(_, __ref, _), _, _) => {
            maybe((__error!("undefined typeDef".to_string())), fromDirectType, __ref)
        },
        _ => {
            __error!("fromDirectType".to_string())
        },
    }
}

pub fn ni() -> NodeInfo {
    undefNode
}

pub fn threadLocal(__0: bool) -> Vec<CStorageSpec> {
    match (__0) {
        false => {
            id
        },
        true => {
            ((CThread(ni))(__op_concat))
        },
    }
}

