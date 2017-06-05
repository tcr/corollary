// Original file: "DeclAnalysis.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::Error;
// use Language::C::Data::Node;
// use Language::C::Data::Ident;
// use Language::C::Pretty;
// use Language::C::Syntax;
// use Language::C::Analysis::AstAnalysis;
// use tExpr;
// use Language::C::Analysis::DefTable;
// use TagFwdDecl;
// use Language::C::Analysis::SemError;
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::TravMonad;
// use Data::Foldable;
// use Control::Monad;
// use liftM;
// use Data::List;
// use intercalate;
// use Data::Map;
// use Text::PrettyPrint::HughesPJ;

pub fn tParamDecl(_0: CDecl) -> m<ParamDecl> {
    match (_0) {
        CStaticAssert(_, _, node) => {
            astError(node, "expected parameter, not static assertion".to_string())
        },
        CDecl(declspecs, declrs, node) => {
            astError(node, "expected parameter, not static assertion".to_string())
        },
    }
}

pub fn computeParamStorage(_0: NodeInfo, _1: StorageSpec) -> Either<BadSpecifierError, Storage> {
    match (_0, _1) {
        (_, NoStorageSpec) => {
            Right((Auto(false)))
        },
        (_, RegSpec) => {
            Right((Auto(false)))
        },
        (node, spec) => {
            Right((Auto(false)))
        },
    }
}

pub fn tMemberDecls(_0: CDecl) -> m<Vec<MemberDecl>> {
    match (_0) {
        CStaticAssert(_, _, node) => {
            astError(node, "expected struct or union member, found static assertion".to_string())
        },
        CDecl(declspecs, [], node) => {
            astError(node, "expected struct or union member, found static assertion".to_string())
        },
        CDecl(declspecs, declrs, node) => {
            astError(node, "expected struct or union member, found static assertion".to_string())
        },
    }
}

#[derive(Debug, Eq, Ord, Read)]
pub enum StorageSpec {
    NoStorageSpec,
    AutoSpec,
    RegSpec,
    ThreadSpec,
    StaticSpec(bool),
    ExternSpec(bool)
}
pub use self::StorageSpec::*;

pub fn hasThreadLocalSpec(_0: StorageSpec) -> bool {
    match (_0) {
        ThreadSpec => {
            true
        },
        StaticSpec(b) => {
            true
        },
        ExternSpec(b) => {
            true
        },
        _ => {
            true
        },
    }
}

pub struct VarDeclInfo(VarName, FunctionAttrs, StorageSpec, Attributes, Type, NodeInfo);


pub fn analyseVarDecl_q(handle_sue_def: bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, oldstyle: Vec<CDecl>, init_opt: Option<CInit>) -> m<VarDeclInfo> {
    /*do*/ {
        let (storage_specs, attrs, type_quals, type_specs, funspecs, _alignspecs) = partitionDeclSpecs(declspecs);

        let canonTySpecs = canonicalTypeSpec(type_specs);

        analyseVarDecl(handle_sue_def, storage_specs, attrs, type_quals, canonTySpecs, funspecs, declr, oldstyle, init_opt)
    }
}

pub fn analyseVarDecl(handle_sue_def: bool, storage_specs: Vec<CStorageSpec>, decl_attrs: Vec<CAttr>, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, fun_specs: Vec<CFunSpec>, CDeclr(name_opt, derived_declrs, asmname_opt, declr_attrs, node): CDeclr, oldstyle_params: Vec<CDecl>, _init_opt: Option<CInit>) -> m<VarDeclInfo> {

    let updateFunSpec = |_0, _1| {
        match (_0, _1) {
            (CInlineQual(_), f) => {
                f {
                    isInline: true
                }
            },
            (CNoreturnQual(_), f) => {
                f {
                    isInline: true
                }
            },
        }
    };

    let function_spec = foldr(updateFunSpec, noFunctionAttrs, fun_specs);

    /*do*/ {
        let storage_spec = canonicalStorageSpec(storage_specs);

        let typ = tType(handle_sue_def, node, typequals, canonTySpecs, derived_declrs, oldstyle_params);

        let attrs_q = mapM(tAttr, (__op_addadd(decl_attrs, declr_attrs)));

        let name = mkVarName(node, name_opt, asmname_opt);

        VarDeclInfo(name, function_spec, storage_spec, attrs_q, typ, node)
    }
}

pub fn isTypeDef(declspecs: Vec<CDeclSpec>) -> bool {
    not(null(/* Expr::Generator */ Generator))
}

pub fn analyseTypeDecl(_0: CDecl) -> m<Type> {
    match (_0) {
        CStaticAssert(_, _, node) => {
            astError(node, "Expected type declaration, found static assert".to_string())
        },
        CDecl(declspecs, declrs, node) => {
            astError(node, "Expected type declaration, found static assert".to_string())
        },
    }
}

pub fn tType(handle_sue_def: bool, top_node: NodeInfo, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, derived_declrs: Vec<CDerivedDeclr>, oldstyle_params: Vec<CDecl>) -> m<Type> {

    let buildType = |_0| {
        match (_0) {
            [] => {
                tDirectType(handle_sue_def, top_node, typequals, canonTySpecs)
            },
            [CPtrDeclr(ptrquals, node), dds] => {
                tDirectType(handle_sue_def, top_node, typequals, canonTySpecs)
            },
            [CArrDeclr(arrquals, size, node), dds] => {
                tDirectType(handle_sue_def, top_node, typequals, canonTySpecs)
            },
            [CFunDeclr(Right((params, isVariadic)), attrs, node), dds] => {
                tDirectType(handle_sue_def, top_node, typequals, canonTySpecs)
            },
            [CFunDeclr(Left(_), _, _), _] => {
                tDirectType(handle_sue_def, top_node, typequals, canonTySpecs)
            },
        }
    };

    let buildPointerType = |ptrquals, _node, inner_ty| {
        liftM((|(quals, attrs)| { PtrType }), (tTypeQuals(ptrquals)))
    };

    let buildArrayType = |arr_quals, size, _node, inner_ty| {
        /*do*/ {
            let (quals, attrs) = tTypeQuals(arr_quals);

            let arr_sz = tArraySize(size);

            ArrayType(inner_ty, arr_sz, quals, attrs)
        }
    };

    let buildFunctionType = |params, is_variadic, attrs, _node, return_ty| {
        /*do*/ {
            enterPrototypeScope;
            let params_q = mapM(tParamDecl, params);

            leavePrototypeScope;
            let attrs_q = mapM(tAttr, attrs);

            (|t| { (t, attrs_q) })(match (__map!(declType, params_q), is_variadic) {
                    ([], false) => {
                        FunTypeIncomplete(return_ty)
                    },
                    ([DirectType(TyVoid, _, _)], false) => {
                        FunType(return_ty, vec![], false)
                    },
                    _ => {
                        FunType(return_ty, params_q, is_variadic)
                    },
                })
        }
    };

    __op_bind(mergeOldStyle(top_node, oldstyle_params, derived_declrs), buildType)
}

pub fn tDirectType(handle_sue_def: bool, node: NodeInfo, ty_quals: Vec<CTypeQual>, canonTySpec: TypeSpecAnalysis) -> m<Type> {
    /*do*/ {
        let (quals, attrs) = tTypeQuals(ty_quals);

        let baseType = |ty_name| {
            DirectType(ty_name, quals, attrs)
        };

        match canonTySpec {
            TSNone => {
                baseType((TyIntegral(TyInt)))
            },
            TSVoid => {
                baseType(TyVoid)
            },
            TSBool => {
                baseType((TyIntegral(TyBool)))
            },
            TSNum(tsnum) => {
                /*do*/ {
                    let numType = tNumType(tsnum);

                    baseType(match numType {
                            Left((floatType, iscomplex)) if iscomplex => { TyComplex(floatType) }
                            Left((floatType, iscomplex)) => { TyFloating(floatType) }
                            Right(intType) => {
                                TyIntegral(intType)
                            },
                        })
                }
            },
            TSTypeDef(tdr) => {
                TypeDefType(tdr, quals, attrs)
            },
            TSNonBasic(CSUType(su, _tnode)) => {
                liftM((baseType(TyComp)), tCompTypeDecl(handle_sue_def, su))
            },
            TSNonBasic(CEnumType(__enum, _tnode)) => {
                liftM((baseType(TyEnum)), tEnumTypeDecl(handle_sue_def, __enum))
            },
            TSType(t) => {
                mergeTypeAttributes(node, quals, attrs, t)
            },
            TSNonBasic(t) => {
                astError(node, (__op_addadd("Unexpected typespec: ".to_string(), show(t))))
            },
        }
    }
}

pub fn mergeTypeAttributes(node_info: NodeInfo, quals: TypeQuals, attrs: Vec<Attr>, typ: Type) -> m<Type> {

    let merge = |quals_q, attrs_q, tyf| {
        tyf((mergeTypeQuals(quals, quals_q)), (__op_addadd(attrs_q, attrs)))
    };

    match typ {
        DirectType(ty_name, quals_q, attrs_q) => {
            merge(quals_q, attrs_q, DirectType(ty_name))
        },
        PtrType(ty, quals_q, attrs_q) => {
            merge(quals_q, attrs_q, PtrType(ty))
        },
        ArrayType(ty, array_sz, quals_q, attrs_q) => {
            merge(quals_q, attrs_q, ArrayType(ty, array_sz))
        },
        FunctionType(fty, attrs_q) if __op_assign_div(quals, noTypeQuals) => { astError(node_info, "type qualifiers for function type".to_string()) }
        FunctionType(fty, attrs_q) => { FunctionType(fty, (__op_addadd(attrs_q, attrs))) }
        TypeDefType(tdr, quals_q, attrs_q) => {
            merge(quals_q, attrs_q, TypeDefType(tdr))
        },
    }
}

pub fn typeDefRef(t_node: NodeInfo, name: Ident) -> m<TypeDefRef> {
    __op_bind(lookupTypeDef(name), |ty| { (TypeDefRef(name, ty, t_node)) })
}

pub fn tCompTypeDecl(handle_def: bool, CStruct(tag, ident_opt, member_decls_opt, attrs, node_info): CStructUnion) -> m<CompTypeRef> {
    /*do*/ {
        let sue_ref = createSUERef(node_info, ident_opt);

        let tag_q = tTag(tag);

        let attrs_q = mapM(tAttr, attrs);

        let decl = CompTypeRef(sue_ref, tag_q, node_info);

        handleTagDecl((CompDecl(decl)));
        if handle_def { maybeM(member_decls_opt, |decls| { __op_bind(tCompType(sue_ref, tag_q, decls, attrs_q, node_info), handleTagDef::CompDef()) }) };
        decl
    }
}

pub fn tTag(_0: CStructTag) -> CompTyKind {
    match (_0) {
        CStructTag => {
            StructTag
        },
        CUnionTag => {
            StructTag
        },
    }
}

pub fn tCompType(tag: SUERef, sue_ref: CompTyKind, member_decls: Vec<CDecl>, attrs: Attributes, node: NodeInfo) -> m<CompType> {
    ap((CompType(tag, sue_ref)), ap((concatMapM(tMemberDecls, member_decls)), ap((attrs), (node))))
}

pub fn tEnumType(sue_ref: SUERef, enumerators: Vec<(Ident, Option<CExpr>)>, attrs: Attributes, node: NodeInfo) -> m<EnumType> {

    let ty = EnumType(sue_ref, enumerators_q, attrs, node);

    let nextEnumerator = |memo, (ident, e)| {
        {
            let (memo_q, expr) = nextEnrExpr(memo, e);

        (memo_q, Enumerator(ident, expr, ty, (nodeInfo(ident))))        }
    };

    pub fn nextEnrExpr(_0: Either<Integer, (Expr, Integer)>, _1: Option<CExpr>) -> (Either<Integer, (Expr, Integer)>, CExpr) {
        match (_0, _1) {
            (Left(i), None) => {
                (Left((succ(i))), intExpr(i))
            },
            (Right((e, offs)), None) => {
                (Left((succ(i))), intExpr(i))
            },
            (_, Some(e)) => {
                (Left((succ(i))), intExpr(i))
            },
        }
    }

    let intExpr = |i| {
        CConst((CIntConst((cInteger(i)), undefNode)))
    };

    let offsExpr = |e, offs| {
        CBinary(CAddOp, e, (intExpr(offs)), undefNode)
    };

    /*do*/ {
        mapM_(handleEnumeratorDef, enumerators_q);
        ty
    }
}

pub fn tNumType(NumTypeSpec(basetype, sgn, sz, iscomplex): NumTypeSpec) -> m<Either<(FloatType, bool), IntType>> {

    let optBase = |_0, _1| {
        match (_0, _1) {
            (_, NoBaseType) => {
                true
            },
            (expect, baseTy) => {
                true
            },
        }
    };

    let optSign = |_0, _1| {
        match (_0, _1) {
            (_, NoSignSpec) => {
                true
            },
            (expect, sign) => {
                true
            },
        }
    };

    let intType = Right;

    let floatType = |ft| {
        (Left((ft, iscomplex)))
    };

    match (basetype, sgn, sz) {
        (BaseChar, _, NoSizeMod) if Signed => { intType(TySChar) }
        (BaseChar, _, NoSizeMod) if Unsigned => { intType(TyUChar) }
        (BaseChar, _, NoSizeMod) => { intType(TyChar) }
        (intbase, _, NoSizeMod) if optBase(BaseInt, intbase) => { intType(match sgn {
                Unsigned => {
                    TyUInt
                },
                _ => {
                    TyInt
                },
            }) }
        (intbase, _, NoSizeMod) if optBase(BaseInt128, intbase) => { intType(match sgn {
                Unsigned => {
                    TyUInt128
                },
                _ => {
                    TyInt128
                },
            }) }
        (intbase, signed, sizemod) if optBase(BaseInt, intbase) && optSign(Signed, signed) => { intType(match sizemod {
                ShortMod => {
                    TyShort
                },
                LongMod => {
                    TyLong
                },
                LongLongMod => {
                    TyLLong
                },
                _ => {
                    internalErr("numTypeMapping: unexpected pattern matching error".to_string())
                },
            }) }
        (intbase, Unsigned, sizemod) if optBase(BaseInt, intbase) => { intType(match sizemod {
                ShortMod => {
                    TyUShort
                },
                LongMod => {
                    TyULong
                },
                LongLongMod => {
                    TyULLong
                },
                _ => {
                    internalErr("numTypeMapping: unexpected pattern matching error".to_string())
                },
            }) }
        (BaseFloat, NoSignSpec, NoSizeMod) => {
            floatType(TyFloat)
        },
        (BaseDouble, NoSignSpec, NoSizeMod) => {
            floatType(TyDouble)
        },
        (BaseDouble, NoSignSpec, LongMod) => {
            floatType(TyLDouble)
        },
        (_, _, _) => {
            __error!("Bad AST analysis".to_string())
        },
    }
}

pub fn tArraySize(_0: CArrSize) -> m<ArraySize> {
    match (_0) {
        CNoArrSize(false) => {
            (UnknownArraySize(false))
        },
        CNoArrSize(true) => {
            (UnknownArraySize(false))
        },
        CArrSize(__static, szexpr) => {
            (UnknownArraySize(false))
        },
    }
}

pub fn tTypeQuals() -> m<(TypeQuals, Attributes)> {

    let go = |_0, _1| {
        match (_0, _1) {
            (CConstQual(_), (tq, attrs)) => {
                (tq {
                    constant: true
                }, attrs)
            },
            (CVolatQual(_), (tq, attrs)) => {
                (tq {
                    constant: true
                }, attrs)
            },
            (CRestrQual(_), (tq, attrs)) => {
                (tq {
                    constant: true
                }, attrs)
            },
            (CAtomicQual(_), (tq, attrs)) => {
                (tq {
                    constant: true
                }, attrs)
            },
            (CAttrQual(attr), (tq, attrs)) => {
                (tq {
                    constant: true
                }, attrs)
            },
            (CNullableQual(_), (tq, attrs)) => {
                (tq {
                    constant: true
                }, attrs)
            },
            (CNonnullQual(_), (tq, attrs)) => {
                (tq {
                    constant: true
                }, attrs)
            },
        }
    };

    foldrM(go, (noTypeQuals, vec![]))
}

#[derive(Eq, Ord)]
pub enum NumBaseType {
    NoBaseType,
    BaseChar,
    BaseInt,
    BaseInt128,
    BaseFloat,
    BaseDouble
}
pub use self::NumBaseType::*;

#[derive(Eq, Ord)]
pub enum SignSpec {
    NoSignSpec,
    Signed,
    Unsigned
}
pub use self::SignSpec::*;

#[derive(Eq, Ord)]
pub enum SizeMod {
    NoSizeMod,
    ShortMod,
    LongMod,
    LongLongMod
}
pub use self::SizeMod::*;

pub struct NumTypeSpec{
    base: NumBaseType,
    signSpec: SignSpec,
    sizeMod: SizeMod,
    isComplex: bool
}
fn base(a: NumTypeSpec) -> NumBaseType { a.base }
fn signSpec(a: NumTypeSpec) -> SignSpec { a.signSpec }
fn sizeMod(a: NumTypeSpec) -> SizeMod { a.sizeMod }
fn isComplex(a: NumTypeSpec) -> bool { a.isComplex }

pub fn emptyNumTypeSpec() -> NumTypeSpec {
    NumTypeSpec {
        base: NoBaseType,
        signSpec: NoSignSpec,
        sizeMod: NoSizeMod,
        isComplex: false
    }
}

pub enum TypeSpecAnalysis {
    TSNone,
    TSVoid,
    TSBool,
    TSNum(NumTypeSpec),
    TSTypeDef(TypeDefRef),
    TSType(Type),
    TSNonBasic(CTypeSpec)
}
pub use self::TypeSpecAnalysis::*;

pub fn canonicalTypeSpec() -> m<TypeSpecAnalysis> {

    let getNTS = |_0| {
        match (_0) {
            TSNone => {
                Some(emptyNumTypeSpec)
            },
            TSNum(nts) => {
                Some(emptyNumTypeSpec)
            },
            _ => {
                Some(emptyNumTypeSpec)
            },
        }
    };

    let updLongMod = |_0| {
        match (_0) {
            NoSizeMod => {
                Some(LongMod)
            },
            LongMod => {
                Some(LongMod)
            },
            _ => {
                Some(LongMod)
            },
        }
    };

    pub fn go(_0: CTypeSpec, _1: TypeSpecAnalysis) -> m<TypeSpecAnalysis> {
        match (_0, _1) {
            (CVoidType(_), TSNone) => {
                TSVoid
            },
            (CBoolType(_), TSNone) => {
                TSVoid
            },
            (CCharType(_), tsa) => {
                TSVoid
            },
            (CIntType(_), tsa) => {
                TSVoid
            },
            (CInt128Type(_), tsa) => {
                TSVoid
            },
            (CFloatType(_), tsa) => {
                TSVoid
            },
            (CDoubleType(_), tsa) => {
                TSVoid
            },
            (CShortType(_), tsa) => {
                TSVoid
            },
            (CLongType(_), tsa) => {
                TSVoid
            },
            (CSignedType(_), tsa) => {
                TSVoid
            },
            (CUnsigType(_), tsa) => {
                TSVoid
            },
            (CComplexType(_), tsa) => {
                TSVoid
            },
            (CTypeDef(i, ni), TSNone) => {
                TSVoid
            },
            (CTypeOfType(d, _ni), TSNone) => {
                TSVoid
            },
            (CTypeOfExpr(e, _), TSNone) => {
                TSVoid
            },
            (CAtomicType(d, _ni), TSNone) => {
                TSVoid
            },
            (otherType, TSNone) => {
                TSVoid
            },
            (ty, _ts) => {
                TSVoid
            },
        }
    }

    foldrM(go, TSNone)
}

pub fn canonicalStorageSpec(storagespecs: Vec<CStorageSpec>) -> m<StorageSpec> {

    let updStorage = |_0, _1| {
        match (_0, _1) {
            (CAuto(_), NoStorageSpec) => {
                AutoSpec
            },
            (CRegister(_), NoStorageSpec) => {
                AutoSpec
            },
            (CThread(_), NoStorageSpec) => {
                AutoSpec
            },
            (CThread(_), StaticSpec(_)) => {
                AutoSpec
            },
            (CThread(_), ExternSpec(_)) => {
                AutoSpec
            },
            (CStatic(_), NoStorageSpec) => {
                AutoSpec
            },
            (CExtern(_), NoStorageSpec) => {
                AutoSpec
            },
            (CStatic(_), ThreadSpec) => {
                AutoSpec
            },
            (CExtern(_), ThreadSpec) => {
                AutoSpec
            },
            (badSpec, old) => {
                AutoSpec
            },
        }
    };

    let elideAuto = |_0| {
        match (_0) {
            AutoSpec => {
                NoStorageSpec
            },
            spec => {
                NoStorageSpec
            },
        }
    };

    liftM(elideAuto, foldrM(updStorage, NoStorageSpec, storagespecs))
}

pub fn mergeOldStyle(_0: NodeInfo, _1: Vec<CDecl>, _2: Vec<CDerivedDeclr>) -> m<Vec<CDerivedDeclr>> {
    match (_0, _1, _2) {
        (_node, [], declrs) => {
            declrs
        },
        (node, oldstyle_params, [CFunDeclr(params, attrs, fdnode), dds]) => {
            declrs
        },
        (node, _, _) => {
            declrs
        },
    }
}

pub fn splitCDecl(_0: CDecl, _1: m<Vec<CDecl>>) -> m<Vec<CDecl>> {
    match (_0, _1, _2) {
        (decl, __OP__, CStaticAssert(_, _, _)) => {
            vec![decl]
        },
        (decl, __OP__, CDecl(declspecs, declrs, node)) => {
            vec![decl]
        },
    }
}

pub fn tAttr(CAttr(name, cexpr, node): CAttr) -> m<Attr> {
    Attr(name, cexpr, node)
}

pub fn mkVarName(_0: NodeInfo, _1: Option<Ident>, _2: Option<AsmName>) -> m<VarName> {
    match (_0, _1, _2) {
        (_node, None, _) => {
            NoName
        },
        (_node, Some(n), asm) => {
            NoName
        },
    }
}

pub fn nameOfDecl(d: CDecl) -> m<Ident> {
    __op_bind(getOnlyDeclr(d), |declr| { match declr {
            CDeclr(Some(name), _, _, _, _node) => {
                name
            },
            CDeclr(None, _, _, _, _node) => {
                internalErr("nameOfDecl: abstract declarator".to_string())
            },
        } })
}

pub fn emptyDeclr(node: NodeInfo) -> CDeclr {
    CDeclr(None, vec![], None, vec![], node)
}

pub fn getOnlyDeclr(_0: CDecl) -> m<CDeclr> {
    match (_0) {
        CDecl(_, [(Some(declr), _, _)], _) => {
            declr
        },
        CDecl(_, _, _node) => {
            declr
        },
        CStaticAssert(_, _, _) => {
            declr
        },
    }
}



