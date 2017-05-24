mod haskell_support {
    pub trait Addable {
        fn add(self, right: Self) -> Self;
    }

    impl Addable for String {
        fn add(self, right: Self) -> Self {
            format!("{}{}", self, right)
        }
    }

    pub fn __op_addadd<A: Addable>(left: A, right: A) -> A {
        Addable::add(left, right)
    }
}


pub mod Language_C_Analysis_AstAnalysis {
    use haskell_support::*;
    pub enum StmtCtx {
        FunCtx<VarDecl>,
        LoopCtx,
        SwitchCtx
    }
    pub use self::StmtCtx::*;

    #[derive(Debug, Eq)]
    pub enum ExprSide {
        LValue,
        RValue
    }
    pub use self::ExprSide::*;

    pub fn advanceDesigList(ds: Vec<CDesignator>, d: CDesignator) -> Vec<CDesignator> {
        drop(1)(dropWhile((not(matchDesignator(d))), ds))
    }

    pub fn analyseAST((CTranslUnit(decls, _file_node)): CTranslUnit) -> m<GlobalDecls> {
        /* do */ {
            mapRecoverM_(analyseExt, decls);
            __op_bind(getDefTable, |dt| { when }((not((inFileScope(dt)))))(__error!("Internal Error: Not in filescope after analysis".to_string())));
            liftM(globalDefs, getDefTable);

        }
    }

    pub fn analyseExt(__0: CExtDecl) -> m<()> {
        match (__0) {
            CAsmExt(asm, _) => {
                handleAsmBlock(asm)
            },
            CFDefExt(fundef) => {
                analyseFunDef(fundef)
            },
            CDeclExt(decl) => {
                analyseDecl(false, decl)
            },
        }
    }

    pub fn analyseFunDef((CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info)): CFunDef) -> m<()> {
        /* do */ {
            let var_decl_info = analyseVarDecl_q(true, declspecs, declr, oldstyle_decls, None);
            {
                let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = var_decl_info;
            };
            when((isNoName(name)))(astError(node_info, "NoName in analyseFunDef".to_string()));
            {
                let ident = identOfVarName(name);
            };
            let ty_q = improveFunDefType(ty);
            let fun_storage = computeFunDefStorage(ident, storage_spec);
            {
                let var_decl = VarDecl(name, (DeclAttrs(is_inline, fun_storage, attrs)), ty_q);
            };
            handleVarDecl(false, (Decl(var_decl, node_info)));
            let stmt_q = analyseFunctionBody(node_info, var_decl, stmt);
            handleFunDef(ident, (FunDef(var_decl, stmt_q, node_info)));

        }
    }

    pub fn analyseFunctionBody(__0: NodeInfo, __1: VarDecl, __2: CStat, __3: m<Stmt>) -> m<Stmt> {
        match (__0, __1, __2, __3, __4) {
            (node_info, decl, s, @, CCompound(localLabels, items, _)) => {
                /* do */ {
                    enterFunctionScope;
                    mapM_((withDefTable(defineLabel)), (__op_addadd(localLabels, getLabels(s))));
                    defineParams(node_info, decl);
                    mapM_((tBlockItem(vec![FunCtx(decl)])), items);
                    leaveFunctionScope;
                    s
                }
            },
            (_, _, s) => {
                astError((nodeInfo(s)), "Function body is no compound statement".to_string())
            },
        }
    }

    pub fn analyseTypeDef(handle_sue_def: bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, node_info: NodeInfo) -> m<()> {
        /* do */ {
            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl_q(handle_sue_def, declspecs, declr, vec![], None);
            checkValidTypeDef(is_inline, storage_spec, attrs);
            when((isNoName(name)))(astError(node_info, "NoName in analyseTypeDef".to_string()));
            {
                let ident = identOfVarName(name);
            };
            handleTypeDef((TypeDef(ident, ty, attrs, node_info)));

        }
    }

    pub fn builtinType(__0: CBuiltin) -> m<Type> {
        match (__0) {
            CBuiltinVaArg(_, d, _) => {
                analyseTypeDecl(d)
            },
            CBuiltinOffsetOf(_, _, _) => {
                size_tType
            },
            CBuiltinTypesCompatible(_, _, _) => {
                boolType
            },
        }
    }

    pub fn checkGuard(c: Vec<StmtCtx>, e: CExpr) -> m<()> {
        __op_bind(tExpr(c, RValue, e), checkScalar_q((nodeInfo(e))))
    }

    pub fn checkInits(__0: Type, __1: Vec<CDesignator>, __2: CInitList) -> m<()> {
        match (__0, __1, __2) {
            (_, _, []) => {
                ()
            },
            (t, dds, [ds(i), ...is]) => {
                /* do */ {
                    let (dds_q, ds_q) = match (dds, ds) {
                        ([], []) => {
                            typeError((nodeInfo(i)), "excess elements in initializer".to_string())
                        },
                        (dd_q(__id_3a, rest), []) => {
                            (rest, vec![dd_q])
                        },
                        (_, d(__id_3a, _)) => {
                            (advanceDesigList(dds, d), ds)
                        },
                    };
                    let t_q = tDesignator(t, ds_q);
                    tInit(t_q, i);
                    checkInits(t, dds_q, is)
                }
            },
        }
    }

    pub fn complexBaseType(ni: NodeInfo, c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m<Type> {
        /* do */ {
            let t = tExpr(c, side, e);
            match canonicalType(t) {
                DirectType(TyComplex(ft), quals, attrs) => {
                    return(DirectType((TyFloating(ft)), quals, attrs))
                },
                _ => {
                    typeError(ni)(__op_addadd("expected complex type, got: ".to_string(), pType(t)))
                },
            }
        }
    }

    pub fn computeFunDefStorage(__0: Ident, __1: StorageSpec) -> m<Storage> {
        match (__0, __1) {
            (_, StaticSpec(b)) => {
                return(FunLinkage(InternalLinkage))
            },
            (ident, other_spec) => {
                /* do */ {
                    let obj_opt = lookupObject(ident);
                    {
                        let defaultSpec = FunLinkage(ExternalLinkage);
                    };
                    match other_spec {
                        NoStorageSpec => {
                            return(maybe(defaultSpec, declStorage, obj_opt))
                        },
                        ExternSpec(false) => {
                            return(maybe(defaultSpec, declStorage, obj_opt))
                        },
                        bad_spec => {
                            throwTravError(badSpecifierError((nodeInfo(ident)))(__op_addadd("unexpected function storage specifier (only static or extern is allowed)".to_string(), show(bad_spec))))
                        },
                    }
                }
            },
        }
    }

    pub fn defaultMD() -> MachineDesc {
        MachineDesc({
            iSize: |it| { match it {
                            TyBool => {
                                1
                            },
                            TyChar => {
                                1
                            },
                            TySChar => {
                                1
                            },
                            TyUChar => {
                                1
                            },
                            TyShort => {
                                2
                            },
                            TyUShort => {
                                2
                            },
                            TyInt => {
                                4
                            },
                            TyUInt => {
                                4
                            },
                            TyLong => {
                                4
                            },
                            TyULong => {
                                4
                            },
                            TyLLong => {
                                8
                            },
                            TyULLong => {
                                8
                            },
                        } },
            fSize: |ft| { match ft {
                            TyFloat => {
                                4
                            },
                            TyDouble => {
                                8
                            },
                            TyLDouble => {
                                16
                            },
                        } },
            builtinSize: |bt| { match bt {
                            TyVaList => {
                                4
                            },
                            TyAny => {
                                4
                            },
                        } },
            ptrSize: 4,
            voidSize: 1,
            iAlign: |it| { match it {
                            TyBool => {
                                1
                            },
                            TyChar => {
                                1
                            },
                            TySChar => {
                                1
                            },
                            TyUChar => {
                                1
                            },
                            TyShort => {
                                2
                            },
                            TyUShort => {
                                2
                            },
                            TyInt => {
                                4
                            },
                            TyUInt => {
                                4
                            },
                            TyLong => {
                                4
                            },
                            TyULong => {
                                4
                            },
                            TyLLong => {
                                8
                            },
                            TyULLong => {
                                8
                            },
                        } },
            fAlign: |ft| { match ft {
                            TyFloat => {
                                4
                            },
                            TyDouble => {
                                8
                            },
                            TyLDouble => {
                                16
                            },
                        } },
            builtinAlign: |bt| { match bt {
                            TyVaList => {
                                4
                            },
                            TyAny => {
                                4
                            },
                        } },
            ptrAlign: 4,
            voidAlign: 1
        })
    }

    pub fn defineParams(ni: NodeInfo, decl: VarDecl) -> m<()> {
        match (getParams(declType(decl))) {
            None => {
                astError(ni, "expecting complete function type in function definition".to_string())
            },
            Some(params) => {
                mapM_(handleParamDecl, params)
            },
        }
    }

    pub fn enclosingFunctionType(__0: Vec<StmtCtx>) -> Option<Type> {
        match (__0) {
            [] => {
                None
            },
            [FunCtx(vd), ..._] => {
                Some(declType(vd))
            },
            [_, ...cs] => {
                enclosingFunctionType(cs)
            },
        }
    }

    pub fn extFunProto((VarDeclInfo(var_name, is_inline, storage_spec, attrs, ty, node_info)): VarDeclInfo) -> m<()> {
        /* do */ {
            when((isNoName(var_name)))(astError(node_info, "NoName in extFunProto".to_string()));
            let old_fun = lookupObject((identOfVarName(var_name)));
            checkValidSpecs;
            {
                let decl = VarDecl(var_name, (DeclAttrs(is_inline, (funDeclLinkage(old_fun)), attrs)), ty);
            };
            handleVarDecl(false, (Decl(decl, node_info)));
            enterPrototypeScope;
            maybe((()), (mapM_(handleParamDecl)), (getParams(ty)));
            leavePrototypeScope
        }
    }

    pub fn extVarDecl((VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info)): VarDeclInfo, init_opt: Option<Initializer>) -> m<()> {
        /* do */ {
            when((isNoName(var_name)))(astError(node_info, "NoName in extVarDecl".to_string()));
            let (storage, is_def) = globalStorage(storage_spec);
            {
                let vardecl = VarDecl(var_name, (DeclAttrs(is_inline, storage, attrs)), typ);
            };
            if(is_def, then, handleObjectDef, false, ident)(ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, false)(Decl(vardecl, node_info)))
        }
    }

    pub fn getParams(__0: Type) -> Option<Vec<ParamDecl>> {
        match (__0) {
            FunctionType(FunType(_, params, _), _) => {
                Some(params)
            },
            _ => {
                None
            },
        }
    }

    pub fn hasTypeDef(declspecs: Vec<CDeclSpec>) -> Option<Vec<CDeclSpec>> {
        match foldr(hasTypeDefSpec, (false, vec![]), declspecs) {
            (true, specs_q) => {
                Some(specs_q)
            },
            (false, _) => {
                None
            },
        }
    }

    pub fn inLoop(c: Vec<StmtCtx>) -> bool {
        any(isLoop, c)
    }

    pub fn inSwitch(c: Vec<StmtCtx>) -> bool {
        any(isSwitch, c)
    }

    pub fn localVarDecl((VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info)): VarDeclInfo, init_opt: Option<Initializer>) -> m<()> {
        /* do */ {
            when((isNoName(var_name)))(astError(node_info, "NoName in localVarDecl".to_string()));
            let (storage, is_def) = localStorage(storage_spec);
            {
                let vardecl = VarDecl(var_name, (DeclAttrs(is_inline, storage, attrs)), typ);
            };
            if(is_def, then, handleObjectDef, true, ident, (ObjDef(vardecl, init_opt, node_info)), else, handleVarDecl, true, (Decl(vardecl, node_info)))
        }
    }

    pub fn matchDesignator(__0: CDesignator, __1: CDesignator) -> bool {
        match (__0, __1) {
            (CMemberDesig(m1, _), CMemberDesig(m2, _)) => {
                (m1 == m2)
            },
            (_, _) => {
                true
            },
        }
    }

    pub fn tBlockItem(__0: Vec<StmtCtx>, __1: CBlockItem) -> m<Type> {
        match (__0, __1) {
            (c, CBlockStmt(s)) => {
                tStmt(c, s)
            },
            (_, CBlockDecl(d)) => {
                __op_rshift(analyseDecl(true, d), voidType)
            },
            (_, CNestedFunDef(fd)) => {
                __op_rshift(analyseFunDef(fd), voidType)
            },
        }
    }

    pub fn tDesignator(__0: Type, __1: Vec<CDesignator>) -> m<Type> {
        match (__0, __1) {
            (ArrayType(bt, _, _, _), [CArrDesig(e, ni), ...ds]) => {
                /* do */ {
                    __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                    tDesignator(bt, ds)
                }
            },
            (ArrayType(bt, _, _, _), [CRangeDesig(e1, e2, ni), ...ds]) => {
                /* do */ {
                    __op_bind(tExpr(vec![], RValue, e1), checkIntegral_q(ni));
                    __op_bind(tExpr(vec![], RValue, e2), checkIntegral_q(ni));
                    tDesignator(bt, ds)
                }
            },
            (ArrayType(_, _, _, _), [d, ...ds]) => {
                typeError((nodeInfo(d)), "member designator in array initializer".to_string())
            },
            (t, @, DirectType(TyComp(_), _, _), [CMemberDesig(m, ni), ...ds]) => {
                /* do */ {
                    let mt = fieldType(ni, m, t);
                    tDesignator((canonicalType(mt)), ds)
                }
            },
            (t, @, DirectType(TyComp(_), _, _), [d, ..._]) => {
                typeError((nodeInfo(d)), "array designator in compound initializer".to_string())
            },
            (t, []) => {
                t
            },
        }
    }

    pub fn tExpr(c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m<Type> {
        match nameOfNode((nodeInfo(e))) {
            Some(n) => {
                /* do */ {
                    let dt = getDefTable;
                    match lookupType(dt, n) {
                        Some(t) => {
                            t
                        },
                        None => {
                            /* do */ {
                                let t = tExpr_q(c, side, e);
                                withDefTable((|dt| { (t, insertType(dt, n, t)) }))
                            }
                        },
                    }
                }
            },
            None => {
                tExpr_q(c, side, e)
            },
        }
    }

    pub fn tExpr_q(__0: Vec<StmtCtx>, __1: ExprSide, __2: CExpr) -> m<Type> {
        match (__0, __1, __2) {
            (c, side, CBinary(op, le, re, ni)) => {
                /* do */ {
                    when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                    let lt = tExpr(c, RValue, le);
                    let rt = tExpr(c, RValue, re);
                    binopType_q(ni, op, lt, rt)
                }
            },
            (c, side, CUnary(CAdrOp, e, ni)) => {
                /* do */ {
                    when(((side == LValue)))(typeError(ni, "address-of operator as lvalue".to_string()));
                    match e {
                        CCompoundLit(_, _, _) => {
                            liftM(simplePtr, tExpr(c, RValue, e))
                        },
                        CVar(i, _) => {
                            __op_bind(lookupObject(i), typeErrorOnLeft(ni, maybe((notFound(i)), varAddrType)))
                        },
                        _ => {
                            liftM(simplePtr, tExpr(c, LValue, e))
                        },
                    }
                }
            },
            (c, _, CUnary(CIndOp, e, ni)) => {
                __op_bind(tExpr(c, RValue, e), (typeErrorOnLeft(ni, derefType)))
            },
            (c, _, CUnary(CCompOp, e, ni)) => {
                /* do */ {
                    let t = tExpr(c, RValue, e);
                    checkIntegral_q(ni, t);
                    t
                }
            },
            (c, side, CUnary(CNegOp, e, ni)) => {
                /* do */ {
                    when(((side == LValue)))(typeError(ni, "logical negation used as lvalue".to_string()));
                    __op_bind(tExpr(c, RValue, e), checkScalar_q(ni));
                    boolType
                }
            },
            (c, side, CUnary(op, e, _)) => {
                tExpr(c, (if(isEffectfulOp, op, then, LValue, else, side)), e)
            },
            (c, _, CIndex(b, i, ni)) => {
                /* do */ {
                    let bt = tExpr(c, RValue, b);
                    let it = tExpr(c, RValue, i);
                    let addrTy = binopType_q(ni, CAddOp, bt, it);
                    typeErrorOnLeft(ni)(derefType(addrTy))
                }
            },
            (c, side, CCond(e1, me2, e3, ni)) => {
                /* do */ {
                    let t1 = tExpr(c, RValue, e1);
                    checkScalar_q((nodeInfo(e1)), t1);
                    let t3 = tExpr(c, side, e3);
                    match me2 {
                        Some(e2) => {
                            /* do */ {
                                let t2 = tExpr(c, side, e2);
                                conditionalType_q(ni, t2, t3)
                            }
                        },
                        None => {
                            conditionalType_q(ni, t1, t3)
                        },
                    }
                }
            },
            (c, side, CMember(e, m, deref, ni)) => {
                /* do */ {
                    let t = tExpr(c, RValue, e);
                    let bt = if(deref, then, typeErrorOnLeft, ni, (derefType(t)), else, return, t);
                    fieldType(ni, m, bt)
                }
            },
            (c, side, CComma(es, _)) => {
                __op_bind(mapM((tExpr(c, side)), es), last)
            },
            (c, side, CCast(d, e, ni)) => {
                /* do */ {
                    let dt = analyseTypeDecl(d);
                    let et = tExpr(c, side, e);
                    typeErrorOnLeft(ni)(castCompatible(dt, et));
                    dt
                }
            },
            (c, side, CSizeofExpr(e, ni)) => {
                /* do */ {
                    when(((side == LValue)))(typeError(ni, "sizeof as lvalue".to_string()));
                    tExpr(c, RValue, e);
                    size_tType
                }
            },
            (c, side, CAlignofExpr(e, ni)) => {
                /* do */ {
                    when(((side == LValue)))(typeError(ni, "alignof as lvalue".to_string()));
                    tExpr(c, RValue, e);
                    size_tType
                }
            },
            (c, side, CComplexReal(e, ni)) => {
                complexBaseType(ni, c, side, e)
            },
            (c, side, CComplexImag(e, ni)) => {
                complexBaseType(ni, c, side, e)
            },
            (_, side, CLabAddrExpr(_, ni)) => {
                /* do */ {
                    when(((side == LValue)))(typeError(ni, "label address as lvalue".to_string()));
                    return(PtrType(voidType, noTypeQuals, vec![]))
                }
            },
            (_, side, CCompoundLit(d, initList, ni)) => {
                /* do */ {
                    when(((side == LValue)))(typeError(ni, "compound literal as lvalue".to_string()));
                    let lt = analyseTypeDecl(d);
                    tInitList(ni, (canonicalType(lt)), initList);
                    lt
                }
            },
            (_, RValue, CAlignofType(_, _)) => {
                size_tType
            },
            (_, RValue, CSizeofType(_, _)) => {
                size_tType
            },
            (_, LValue, CAlignofType(_, ni)) => {
                typeError(ni, "alignoftype as lvalue".to_string())
            },
            (_, LValue, CSizeofType(_, ni)) => {
                typeError(ni, "sizeoftype as lvalue".to_string())
            },
            (_, side, CVar(i, ni)) => {
                __op_bind(lookupObject(i), maybe((typeErrorOnLeft(ni)(notFound(i))), (declType)))
            },
            (_, _, CConst(c)) => {
                constType(c)
            },
            (_, _, CBuiltinExpr(b)) => {
                builtinType(b)
            },
            (c, _, CCall(fe, args, ni)) => {
                /* do */ {
                    {
                        let defType = FunctionType((FunTypeIncomplete((DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes)))), noAttributes);
;
                        let fallback = |i| {
                            /* do */ {
                                warn(invalidAST(ni)(__op_addadd("unknown function: ".to_string(), identToString(i))));
                                defType
                            }
                        };
                    };
                    let t = match fe {
                        CVar(i, _) => {
                            __op_bind(lookupObject(i), maybe((fallback(i)), (const(tExpr(c, RValue, fe)))))
                        },
                        _ => {
                            tExpr(c, RValue, fe)
                        },
                    };
                    let atys = mapM((tExpr(c, RValue)), args);
                    match canonicalType(t) {
                        PtrType(FunctionType(FunType(rt, pdecls, varargs), _), _, _) => {
                            /* do */ {
                                {
                                    let ptys = map(declType, pdecls);
                                };
                                mapM_(checkArg)(zip3(ptys, atys, args));
                                unless(varargs)(when((/=(length(atys), length(ptys))))(typeError(ni, "incorrect number of arguments".to_string())));
                                return(canonicalType(rt))
                            }
                        },
                        PtrType(FunctionType(FunTypeIncomplete(rt), _), _, _) => {
                            /* do */ {
                                return(canonicalType(rt))
                            }
                        },
                        _ => {
                            typeError(ni)(__op_addadd("attempt to call non-function of type ".to_string(), pType(t)))
                        },
                    }
                }
            },
            (c, _, CAssign(op, le, re, ni)) => {
                /* do */ {
                    let lt = tExpr(c, LValue, le);
                    let rt = tExpr(c, RValue, re);
                    when((constant(typeQuals(lt))))(typeError(ni)(__op_addadd("assignment to lvalue with `constant\' qualifier: ".to_string(), (render(pretty))(le))));
                    match (canonicalType(lt), re) {
                        (lt_q, CConst(CIntConst(i, _))) => if (isPointerType(lt_q) && (getCInteger(i) == 0)) { () },
                        (_, _) => {
                            assignCompatible_q(ni, op, lt, rt)
                        },
                    };
                    lt
                }
            },
            (c, _, CStatExpr(s, _)) => {
                /* do */ {
                    enterBlockScope;
                    mapM_((withDefTable(defineLabel)), (getLabels(s)));
                    let t = tStmt(c, s);
                    leaveBlockScope;
                    t
                }
            },
        }
    }

    pub fn tInit(__0: Type, __1: CInit, __2: m<Initializer>) -> m<Initializer> {
        match (__0, __1, __2, __3) {
            (t, i, @, CInitExpr(e, ni)) => {
                /* do */ {
                    let it = tExpr(vec![], RValue, e);
                    assignCompatible_q(ni, CAssignOp, t, it);
                    i
                }
            },
            (t, i, @, CInitList(initList, ni)) => {
                __op_rshift(tInitList(ni, (canonicalType(t)), initList), i)
            },
        }
    }

    pub fn tInitList(__0: NodeInfo, __1: Type, __2: CInitList, __3: m<()>) -> m<()> {
        match (__0, __1, __2, __3, __4) {
            (ni, t, @, ArrayType(DirectType(TyIntegral(TyChar), _, _), _, _, _), [[](CInitExpr(e, @, CConst(CStrConst(_, _)), _))]) => {
                __op_rshift(tExpr(vec![], RValue, e), ())
            },
            (ni, t, @, ArrayType(_, _, _, _), initList) => {
                /* do */ {
                    {
                        let default_ds = repeat((CArrDesig((CConst((CIntConst((cInteger(0)), ni)))), ni)));
                    };
                    checkInits(t, default_ds, initList)
                }
            },
            (ni, t, @, DirectType(TyComp(ctr), _, _), initList) => {
                /* do */ {
                    let td = lookupSUE(ni, (sueRef(ctr)));
                    let ms = tagMembers(ni, td);
                    {
                        let default_ds = map((|m| { CMemberDesig }((fst(m)), ni)), ms);
                    };
                    checkInits(t, default_ds, initList)
                }
            },
            (ni, PtrType(DirectType(TyVoid, _, _), _, _), _) => {
                ()
            },
            (_, t, [[](i)]) => {
                __op_rshift(tInit(t, i), ())
            },
            (ni, t, _) => {
                typeError(ni)(__op_addadd("initializer list for type: ".to_string(), pType(t)))
            },
        }
    }

    pub fn tStmt(__0: Vec<StmtCtx>, __1: CStat) -> m<Type> {
        match (__0, __1) {
            (c, CLabel(_, s, _, _)) => {
                tStmt(c, s)
            },
            (c, CExpr(e, _)) => {
                maybe((voidType), (tExpr(c, RValue)), e)
            },
            (c, CCompound(ls, body, _)) => {
                /* do */ {
                    enterBlockScope;
                    mapM_((withDefTable(defineLabel)), ls);
                    let t = foldM((const(tBlockItem(c))), voidType, body);
                    leaveBlockScope;
                    t
                }
            },
            (c, CIf(e, sthen, selse, _)) => {
                __op_rshift(checkGuard(c, e), __op_rshift(tStmt(c, sthen), __op_rshift(maybe((()), (__op_rshift(|s| { tStmt }(c, s), ())), selse), voidType)))
            },
            (c, CSwitch(e, s, ni)) => {
                __op_bind(tExpr(c, RValue, e), __op_rshift(checkIntegral_q(ni), tStmt((__op_concat(SwitchCtx, c)), s)))
            },
            (c, CWhile(e, s, _, _)) => {
                __op_rshift(checkGuard(c, e), tStmt((__op_concat(LoopCtx, c)), s))
            },
            (_, CGoto(l, ni)) => {
                /* do */ {
                    let dt = getDefTable;
                    match lookupLabel(l, dt) {
                        Some(_) => {
                            voidType
                        },
                        None => {
                            typeError(ni)(__op_addadd("undefined label in goto: ".to_string(), identToString(l)))
                        },
                    }
                }
            },
            (c, CCont(ni)) => {
                /* do */ {
                    unless((inLoop(c)))(astError(ni, "continue statement outside of loop".to_string()));
                    voidType
                }
            },
            (c, CBreak(ni)) => {
                /* do */ {
                    unless(((inLoop(c) || inSwitch(c))))(astError(ni, "break statement outside of loop or switch statement".to_string()));
                    voidType
                }
            },
            (c, CReturn(Some(e), ni)) => {
                /* do */ {
                    let t = tExpr(c, RValue, e);
                    let rt = match enclosingFunctionType(c) {
                        Some(FunctionType(FunType(rt, _, _), _)) => {
                            rt
                        },
                        Some(FunctionType(FunTypeIncomplete(rt), _)) => {
                            rt
                        },
                        Some(ft) => {
                            astError(ni)(__op_addadd("bad function type: ".to_string(), pType(ft)))
                        },
                        None => {
                            astError(ni, "return statement outside function".to_string())
                        },
                    };
                    match (rt, t) {
                        (DirectType(TyVoid, _, _), DirectType(TyVoid, _, _)) => {
                            ()
                        },
                        _ => {
                            assignCompatible_q(ni, CAssignOp, rt, t)
                        },
                    };
                    voidType
                }
            },
            (_, CReturn(None, _)) => {
                voidType
            },
            (_, CAsm(_, _)) => {
                voidType
            },
            (c, CCase(e, s, ni)) => {
                /* do */ {
                    unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                    __op_bind(tExpr(c, RValue, e), checkIntegral_q(ni));
                    tStmt(c, s)
                }
            },
            (c, CCases(e1, e2, s, ni)) => {
                /* do */ {
                    unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                    __op_bind(tExpr(c, RValue, e1), checkIntegral_q(ni));
                    __op_bind(tExpr(c, RValue, e2), checkIntegral_q(ni));
                    tStmt(c, s)
                }
            },
            (c, CDefault(s, ni)) => {
                /* do */ {
                    unless((inSwitch(c)))(astError(ni, "default statement outside of switch statement".to_string()));
                    tStmt(c, s)
                }
            },
            (c, CFor(i, g, inc, s, _)) => {
                /* do */ {
                    enterBlockScope;
                    either((maybe((()), checkExpr)), (analyseDecl(true)), i);
                    maybe((()), (checkGuard(c)), g);
                    maybe((()), checkExpr, inc);
                    tStmt((__op_concat(LoopCtx, c)), s);
                    leaveBlockScope;
                    voidType
                }
            },
            (c, CGotoPtr(e, ni)) => {
                /* do */ {
                    let t = tExpr(c, RValue, e);
                    match t {
                        PtrType(_, _, _) => {
                            voidType
                        },
                        _ => {
                            typeError(ni, "can\'t goto non-pointer".to_string())
                        },
                    }
                }
            },
        }
    }

}


pub mod Language_C_Analysis_Builtins {
    use haskell_support::*;
    pub fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}


pub mod Language_C_Analysis_ConstEval {
    use haskell_support::*;
    struct MachineDesc(MachineDesc<{ /* struct def */ }>);

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


pub mod Language_C_Analysis_Debug {
    use haskell_support::*;
    pub fn globalDeclStats(file_filter: fn(FilePath) -> bool, gmap: GlobalDecls) -> Vec<(String, isize)> {
        vec![
            ("Enumeration Constants".to_string(), Map::size(enumerators)),
            ("Total Object/Function Declarations".to_string(), Map::size(all_decls)),
            ("Object definitions".to_string(), Map::size(objDefs)),
            ("Function Definitions".to_string(), Map::size(funDefs)),
            ("Tag definitions".to_string(), Map::size(tagDefs)),
            ("TypeDefs".to_string(), Map::size(typeDefs)),
        ]
    }

    pub fn joinComma() -> Doc {
        hsep(punctuate(comma, map(pretty)))
    }

    pub fn prettyAssocs(label: String) -> Doc {
        prettyAssocsWith(label, pretty, pretty)
    }

    pub fn prettyAssocsWith(label: String, prettyKey: fn(k) -> Doc, prettyVal: fn(v) -> Doc, theMap: Vec<(k, v)>) -> Doc {
        $$(text(label), (nest(8))((vcat(map(prettyEntry, theMap)))))
    }

    pub fn terminateSemi() -> Doc {
        terminateSemi_(map(pretty))
    }

    pub fn terminateSemi_() -> Doc {
        hsep(map((<>(semi))))
    }

}


pub mod Language_C_Analysis_DeclAnalysis {
    use haskell_support::*;
    #[derive(Debug, Eq, Ord, Read)]
    pub enum StorageSpec {
        NoStorageSpec,
        AutoSpec,
        RegSpec,
        ThreadSpec,
        StaticSpec<bool>,
        ExternSpec<bool>
    }
    pub use self::StorageSpec::*;

    struct VarDeclInfo(VarDeclInfo<VarName, bool, StorageSpec, Attributes, Type, NodeInfo>);

    #[derive(Eq, Ord)]
    pub enum NumBaseType {
        NoBaseType,
        BaseChar,
        BaseInt,
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

    struct NumTypeSpec(NumTypeSpec<{ /* struct def */ }>);

    pub enum TypeSpecAnalysis {
        TSNone,
        TSVoid,
        TSBool,
        TSNum<NumTypeSpec>,
        TSTypeDef<TypeDefRef>,
        TSType<Type>,
        TSNonBasic<CTypeSpec>
    }
    pub use self::TypeSpecAnalysis::*;

    pub fn analyseVarDecl(handle_sue_def: bool, storage_specs: Vec<CStorageSpec>, decl_attrs: Vec<CAttr>, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, inline: bool, (CDeclr(name_opt, derived_declrs, asmname_opt, declr_attrs, node)): CDeclr, oldstyle_params: Vec<CDecl>, init_opt: Option<CInit>) -> m<VarDeclInfo> {
        /* do */ {
            let storage_spec = canonicalStorageSpec(storage_specs);
            let typ = tType(handle_sue_def, node, typequals, canonTySpecs, derived_declrs, oldstyle_params);
            let attrs_q = mapM(tAttr, (__op_addadd(decl_attrs, declr_attrs)));
            let name = mkVarName(node, name_opt, asmname_opt);
            return(VarDeclInfo(name, inline, storage_spec, attrs_q, typ, node))
        }
    }

    pub fn analyseVarDecl_q(handle_sue_def: bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, oldstyle: Vec<CDecl>, init_opt: Option<CInit>) -> m<VarDeclInfo> {
        /* do */ {
            {
                let (storage_specs, attrs, type_quals, type_specs, inline) = partitionDeclSpecs(declspecs);
            };
            let canonTySpecs = canonicalTypeSpec(type_specs);
            analyseVarDecl(handle_sue_def, storage_specs, attrs, type_quals, canonTySpecs, inline, declr, oldstyle, init_opt)
        }
    }

    pub fn canonicalStorageSpec(storagespecs: Vec<CStorageSpec>) -> m<StorageSpec> {
        liftM(elideAuto)(foldrM(updStorage, NoStorageSpec, storagespecs))
    }

    pub fn canonicalTypeSpec() -> m<TypeSpecAnalysis> {
        foldrM(go, TSNone)
    }

    pub fn computeParamStorage(__0: NodeInfo, __1: StorageSpec) -> Either<BadSpecifierError, Storage> {
        match (__0, __1) {
            (_, NoStorageSpec) => {
                Right((Auto(false)))
            },
            (_, RegSpec) => {
                Right((Auto(true)))
            },
            (node, spec) => {
                Left(badSpecifierError(node)(__op_addadd("Bad storage specified for parameter: ".to_string(), show(spec))))
            },
        }
    }

    pub fn emptyDeclr(node: NodeInfo) -> CDeclr {
        CDeclr(None, vec![], None, vec![], node)
    }

    pub fn emptyNumTypeSpec() -> NumTypeSpec {
        NumTypeSpec({
            base: NoBaseType,
            signSpec: NoSignSpec,
            sizeMod: NoSizeMod,
            isComplex: false
        })
    }

    pub fn getOnlyDeclr(__0: CDecl) -> m<CDeclr> {
        match (__0) {
            CDecl(_, [Some(declr)(_, _)], _) => {
                declr
            },
            CDecl(_, _, node) => {
                internalErr("getOnlyDeclr: declaration doesn\'t have a unique declarator".to_string())
            },
        }
    }

    pub fn hasThreadLocalSpec(__0: StorageSpec) -> bool {
        match (__0) {
            ThreadSpec => {
                true
            },
            StaticSpec(b) => {
                b
            },
            ExternSpec(b) => {
                b
            },
            _ => {
                false
            },
        }
    }

    pub fn isTypeDef(declspecs: Vec<CDeclSpec>) -> bool {
        not(null(<Expr::Dummy>))
    }

    pub fn mergeOldStyle(__0: NodeInfo, __1: Vec<CDecl>, __2: Vec<CDerivedDeclr>) -> m<Vec<CDerivedDeclr>> {
        match (__0, __1, __2) {
            (_node, [], declrs) => {
                declrs
            },
            (node, oldstyle_params, [CFunDeclr(params, attrs, fdnode), ...dds]) => {
                match params {
                    Left(list) => {
                        /* do */ {
                            let oldstyle_params_q = liftM(concat)(mapM(splitCDecl, oldstyle_params));
                            let param_map = liftM(Map::fromList)(mapM(attachNameOfDecl, oldstyle_params_q));
                            let (newstyle_params, param_map_q) = foldrM(insertParamDecl, (vec![], param_map), list);
                            when((not(Map::null(param_map_q))))(astError(node)(__op_addadd("declarations for parameter(s) ".to_string(), __op_addadd(showParamMap(param_map_q), " but no such parameter".to_string()))));
                            return((__op_concat(CFunDeclr((Right((newstyle_params, false))), attrs, fdnode), dds)))
                        }
                    },
                    Right(_newstyle) => {
                        astError(node, "oldstyle parameter list, but newstyle function declaration".to_string())
                    },
                }
            },
            (node, _, _) => {
                astError(node, "oldstyle parameter list, but not function type".to_string())
            },
        }
    }

    pub fn mergeTypeAttributes(node_info: NodeInfo, quals: TypeQuals, attrs: Vec<Attr>, typ: Type) -> m<Type> {
        match typ {
            DirectType(ty_name, quals_q, attrs_q) => {
                merge(quals_q, attrs_q)(mkDirect(ty_name))
            },
            PtrType(ty, quals_q, attrs_q) => {
                merge(quals_q, attrs_q)(PtrType(ty))
            },
            ArrayType(ty, array_sz, quals_q, attrs_q) => {
                merge(quals_q, attrs_q)(ArrayType(ty, array_sz))
            },
            FunctionType(FunType(return_ty, params, inline), attrs_q) => {
                return(FunctionType((FunType(return_ty, params, inline)), (__op_addadd(attrs_q, attrs))))
            },
            TypeDefType(tdr, quals_q, attrs_q) => {
                merge(quals_q, attrs_q)(TypeDefType(tdr))
            },
        }
    }

    pub fn mkVarName(__0: NodeInfo, __1: Option<Ident>, __2: Option<AsmName>) -> m<VarName> {
        match (__0, __1, __2) {
            (node, None, _) => {
                NoName
            },
            (node, Some(n), asm) => {
                return(VarName(n, asm))
            },
        }
    }

    pub fn nameOfDecl(d: CDecl) -> m<Ident> {
        __op_bind(getOnlyDeclr(d), |declr| { match declr {
                CDeclr(Some(name), _, _, _, _) => {
                    name
                },
                CDeclr(None, _, _, _, node) => {
                    internalErr("nameOfDecl: abstract declarator".to_string())
                },
            } })
    }

    pub fn splitCDecl(decl: CDecl, @: m<Vec<CDecl>>) -> m<Vec<CDecl>> {
        match declrs {
            [] => {
                internalErr("splitCDecl applied to empty declaration".to_string())
            },
            [declr] => {
                vec![decl]
            },
            d1(__id_3a, ds) => {
                {
                    let declspecs_q = map(elideSUEDef, declspecs, in);
                }(return)(__op_concat((CDecl(declspecs, vec![d1], node)), <Expr::Dummy>))
            },
        }
    }

    pub fn tArraySize(__0: CArrSize) -> m<ArraySize> {
        match (__0) {
            CNoArrSize(false) => {
                (UnknownArraySize(false))
            },
            CNoArrSize(true) => {
                (UnknownArraySize(true))
            },
            CArrSize(static, szexpr) => {
                liftM((ArraySize(static)), (szexpr))
            },
        }
    }

    pub fn tAttr((CAttr(name, cexpr, node)): CAttr) -> m<Attr> {
        return(Attr(name, cexpr, node))
    }

    pub fn tCompType(tag: SUERef, sue_ref: CompTyKind, member_decls: Vec<CDecl>, attrs: Attributes, node: NodeInfo) -> m<CompType> {
        ap((CompType(tag, sue_ref)), ap((concatMapM(tMemberDecls, member_decls)), ap((attrs), (node))))
    }

    pub fn tCompTypeDecl(handle_def: bool, (CStruct(tag, ident_opt, member_decls_opt, attrs, node_info)): CStructUnion) -> m<CompTypeRef> {
        /* do */ {
            let sue_ref = createSUERef(node_info, ident_opt);
            {
                let tag_q = tTag(tag);
            };
            let attrs_q = mapM(tAttr, attrs);
            {
                let decl = CompTypeRef(sue_ref, tag_q, node_info);
            };
            handleTagDecl((CompDecl(decl)));
            when((handle_def))(/* do */ {
                maybeM(member_decls_opt)(__op_bind(|decls| { tCompType }(sue_ref, tag_q, decls, (attrs_q), node_info), (handleTagDef::CompDef)))
            });
            decl
        }
    }

    pub fn tDirectType(handle_sue_def: bool, node: NodeInfo, ty_quals: Vec<CTypeQual>, canonTySpec: TypeSpecAnalysis) -> m<Type> {
        /* do */ {
            let (quals, attrs) = tTypeQuals(ty_quals);
            {
                let baseType = |ty_name| {
                    DirectType(ty_name, quals, attrs)
                };
            };
            match canonTySpec {
                TSNone => {
                    return(baseType((TyIntegral(TyInt))))
                },
                TSVoid => {
                    return(baseType(TyVoid))
                },
                TSBool => {
                    return(baseType((TyIntegral(TyBool))))
                },
                TSNum(tsnum) => {
                    /* do */ {
                        let numType = tNumType(tsnum);
                        baseType(match numType {
                            Left, (floatType, iscomplex) => if iscomplex { TyComplex(floatType) }
otherwise { TyFloating(floatType) },
                            Right(intType) => {
                                TyIntegral(intType)
                            },
                        })
                    }
                },
                TSTypeDef(tdr) => {
                    return(TypeDefType(tdr, quals, attrs))
                },
                TSNonBasic(CSUType(su, _tnode)) => {
                    liftM((baseType(TyComp)))(tCompTypeDecl(handle_sue_def, su))
                },
                TSNonBasic(CEnumType(enum, _tnode)) => {
                    liftM((baseType(TyEnum)))(tEnumTypeDecl(handle_sue_def, enum))
                },
                TSType(t) => {
                    mergeTypeAttributes(node, quals, attrs, t)
                },
                TSNonBasic(_) => {
                    astError(node, "Unexpected typespec".to_string())
                },
            }
        }
    }

    pub fn tEnumType(sue_ref: SUERef, enumerators: Vec<(Ident, Option<CExpr>)>, attrs: Attributes, node: NodeInfo) -> m<EnumType> {
        /* do */ {
            mapM_(handleEnumeratorDef, enumerators_q);
            ty;

        }
    }

    pub fn tMemberDecls(__0: CDecl) -> m<Vec<MemberDecl>> {
        match (__0) {
            CDecl(declspecs, [], node) => {
                /* do */ {
                    {
                        let (storage_specs, _attrs, typequals, typespecs, is_inline) = partitionDeclSpecs(declspecs);
                    };
                    when(is_inline)(astError(node, "member declaration with inline specifier".to_string()));
                    let canonTySpecs = canonicalTypeSpec(typespecs);
                    let ty = tType(true, node, typequals, canonTySpecs, vec![], vec![]);
                    match ty {
                        DirectType(TyComp(_), _, _) => {
                            return(vec![
                                MemberDecl((VarDecl(NoName, (DeclAttrs(false, NoStorage, vec![])), ty)), None, node),
                            ])
                        },
                        _ => {
                            astError(node, "anonymous member has a non-composite type".to_string())
                        },
                    }
                }
            },
            CDecl(declspecs, declrs, node) => {
                mapM((uncurry(tMemberDecl)), (zip((__op_concat(true, repeat(false))), declrs)))
            },
        }
    }

    pub fn tNumType((NumTypeSpec(basetype, sgn, sz, iscomplex)): NumTypeSpec) -> m<Either<(FloatType, bool), IntType>> {
        match (basetype, sgn, sz) {
            (BaseChar, _, NoSizeMod) => if let Signed = sgn { intType(TySChar) }
let Unsigned = sgn { intType(TyUChar) }
otherwise { intType(TyChar) },
            (intbase, _, NoSizeMod) => if optBase(BaseInt, intbase) { intType(match sgn {
            Unsigned => {
                TyUInt
            },
            _ => {
                TyInt
            },
        }) },
            (intbase, signed, sizemod) => if optBase(BaseInt, intbase) && optSign(Signed, signed) { intType(match sizemod {
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
        }) },
            (intbase, Unsigned, sizemod) => if optBase(BaseInt, intbase) { intType(match sizemod {
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
        }) },
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

    pub fn tParamDecl((CDecl(declspecs, declrs, node)): CDecl) -> m<ParamDecl> {
        /* do */ {
            let declr = getParamDeclr;
            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl_q(true, declspecs, declr, vec![], None);
            when((is_inline))(throwTravError((badSpecifierError(node, "parameter declaration with inline specifier".to_string()))));
            let storage = throwOnLeft(computeParamStorage(node, storage_spec));
            {
                let paramDecl = mkParamDecl(name, storage, attrs, ty, declr_node);
            };
            return(paramDecl)
        }
    }

    pub fn tTag(__0: CStructTag) -> CompTyKind {
        match (__0) {
            CStructTag => {
                StructTag
            },
            CUnionTag => {
                UnionTag
            },
        }
    }

    pub fn tType(handle_sue_def: bool, top_node: NodeInfo, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, derived_declrs: Vec<CDerivedDeclr>, oldstyle_params: Vec<CDecl>) -> m<Type> {
        __op_bind(mergeOldStyle(top_node, oldstyle_params, derived_declrs), buildType)
    }

    pub fn tTypeQuals() -> m<(TypeQuals, Attributes)> {
        foldrM(go, (noTypeQuals, vec![]))
    }

    pub fn typeDefRef(t_node: NodeInfo, name: Ident) -> m<TypeDefRef> {
        __op_bind(lookupTypeDef(name), |ty| { return }((TypeDefRef(name, (Some(ty)), t_node))))
    }

}


pub mod Language_C_Analysis_DefTable {
    use haskell_support::*;
    pub enum TagFwdDecl {
        CompDecl<CompTypeRef>,
        EnumDecl<EnumTypeRef>
    }
    pub use self::TagFwdDecl::*;

    struct DefTable(DefTable<{ /* struct def */ }>);

    #[derive(Clone, Debug)]
    pub enum DeclarationStatus<t> {
        NewDecl,
        Redeclared<t>,
        KeepDef<t>,
        Shadowed<t>,
        KindMismatch<t>
    }
    pub use self::DeclarationStatus::*;

    #[derive(Eq, Ord)]
    pub enum TagEntryKind {
        CompKind<CompTyKind>,
        EnumKind
    }
    pub use self::TagEntryKind::*;

    pub fn compatIdentEntry(__0: IdentEntry) -> bool {
        match (__0) {
            Left(_tydef) => {
                either((const(true)), (const(false)))
            },
            Right(def) => {
                either((const(false)))(|other_def| { match (def, other_def) {
                        (EnumeratorDef(_), EnumeratorDef(_)) => {
                            true
                        },
                        (EnumeratorDef(_), _) => {
                            true
                        },
                        (_, EnumeratorDef(_)) => {
                            true
                        },
                        (_, _) => {
                            true
                        },
                    } })
            },
        }
    }

    pub fn compatTagEntry(te1: TagEntry, te2: TagEntry) -> bool {
        (tagKind(te1) == tagKind(te2))
    }

    pub fn declStatusDescr(__0: DeclarationStatus<t>) -> String {
        match (__0) {
            NewDecl => {
                "new".to_string()
            },
            Redeclared(_) => {
                "redeclared".to_string()
            },
            KeepDef(_) => {
                "keep old".to_string()
            },
            Shadowed(_) => {
                "shadowed".to_string()
            },
            KindMismatch(_) => {
                "kind mismatch".to_string()
            },
        }
    }

    pub fn declareTag(sueref: SUERef, decl: TagFwdDecl, deftbl: DefTable) -> (DeclarationStatus<TagEntry>, DefTable) {
        match lookupTag(sueref, deftbl) {
            None => {
                (NewDecl, deftbl({
                    tagDecls: fst(defLocal((tagDecls(deftbl)), sueref, (Left(decl))))
                }))
            },
            Some, old_def => if (tagKind(old_def) == tagKind((Left(decl)))) { (KeepDef(old_def), deftbl) }
otherwise { (KindMismatch(old_def), deftbl) },
        }
    }

    pub fn defRedeclStatus(sameKind: fn(t) -> fn(t) -> bool, def: t, oldDecl: Option<t>) -> DeclarationStatus<t> {
        match oldDecl {
            Some, def_q => if sameKind(def, def_q) { Redeclared(def_q) }
otherwise { KindMismatch(def_q) },
            None => {
                NewDecl
            },
        }
    }

    pub fn defRedeclStatusLocal(sameKind: fn(t) -> fn(t) -> bool, ident: k, def: t, oldDecl: Option<t>, nsm: NameSpaceMap<k, t>) -> DeclarationStatus<t> {
        match defRedeclStatus(sameKind, def, oldDecl) {
            NewDecl => {
                match lookupName(nsm, ident) {
                    Some(shadowed) => {
                        Shadowed(shadowed)
                    },
                    None => {
                        NewDecl
                    },
                }
            },
            redecl => {
                redecl
            },
        }
    }

    pub fn defineGlobalIdent(ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {
        (defRedeclStatus(compatIdentEntry, (Right(def)), oldDecl), deftbl({
            identDecls: decls_q
        }))
    }

    pub fn defineLabel(ident: Ident, deftbl: DefTable) -> (DeclarationStatus<Ident>, DefTable) {
        {
            let (labels_q, old_label) = defLocal((labelDefs(deftbl)), ident, ident);
        }(in, (maybe(NewDecl, Redeclared, old_label), deftbl({
                labelDefs: labels_q
            })))
    }

    pub fn defineScopedIdent() -> (DeclarationStatus<IdentEntry>, DefTable) {
        defineScopedIdentWhen((const(true)))
    }

    pub fn defineScopedIdentWhen(override_def: fn(IdentDecl) -> bool, ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {
        (redecl_status, deftbl({
            identDecls: decls_q
        }))
    }

    pub fn defineTag(sueref: SUERef, def: TagDef, deftbl: DefTable) -> (DeclarationStatus<TagEntry>, DefTable) {
        (redeclStatus, deftbl({
            tagDecls: decls_q
        }))
    }

    pub fn defineTypeDef(ident: Ident, tydef: TypeDef, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {
        (defRedeclStatus(compatIdentEntry, (Left(tydef)), oldDecl), deftbl({
            identDecls: decls_q
        }))
    }

    pub fn emptyDefTable() -> DefTable {
        DefTable(nameSpaceMap, nameSpaceMap, nameSpaceMap, nameSpaceMap, IntMap::empty, IntMap::empty)
    }

    pub fn enterBlockScope(deftbl: DefTable) -> DefTable {
        enterLocalScope(deftbl({
            labelDefs: enterNewScope((labelDefs(deftbl)))
        }))
    }

    pub fn enterFunctionScope(deftbl: DefTable) -> DefTable {
        enterLocalScope(deftbl({
            labelDefs: enterNewScope((labelDefs(deftbl)))
        }))
    }

    pub fn enterLocalScope(deftbl: DefTable) -> DefTable {
        deftbl({
            identDecls: enterNewScope((identDecls(deftbl))),
            tagDecls: enterNewScope((tagDecls(deftbl)))
        })
    }

    pub fn enterMemberDecl(deftbl: DefTable) -> DefTable {
        deftbl({
            memberDecls: enterNewScope((memberDecls(deftbl)))
        })
    }

    pub fn globalDefs(deftbl: DefTable) -> GlobalDecls {
        Map::foldWithKey(insertDecl, (GlobalDecls(e, gtags, e)), (globalNames(identDecls(deftbl))))
    }

    pub fn identOfTyDecl() -> Ident {
        either(identOfTypeDef, declIdent)
    }

    pub fn inFileScope(dt: DefTable) -> bool {
        not(((hasLocalNames((identDecls(dt))) || hasLocalNames((labelDefs(dt))))))
    }

    pub fn insertType(dt: DefTable, n: Name, t: Type) -> DefTable {
        dt({
            typeTable: IntMap::insert((nameId(n)), t, (typeTable(dt)))
        })
    }

    pub fn leaveBlockScope(deftbl: DefTable) -> DefTable {
        leaveLocalScope(deftbl({
            labelDefs: leaveScope_((labelDefs(deftbl)))
        }))
    }

    pub fn leaveFunctionScope(deftbl: DefTable) -> DefTable {
        leaveLocalScope(deftbl({
            labelDefs: leaveScope_((labelDefs(deftbl)))
        }))
    }

    pub fn leaveLocalScope(deftbl: DefTable) -> DefTable {
        deftbl({
            identDecls: leaveScope_((identDecls(deftbl))),
            tagDecls: leaveScope_((tagDecls(deftbl)))
        })
    }

    pub fn leaveMemberDecl(deftbl: DefTable) -> (Vec<MemberDecl>, DefTable) {
        {
            let (decls_q, members) = leaveScope((memberDecls(deftbl)));
        }(in, <Expr::Dummy>, (map(snd, members)), (deftbl({
                memberDecls: decls_q
            })))
    }

    pub fn leaveScope_() -> NameSpaceMap<k, a> {
        fst(leaveScope)
    }

    pub fn lookupIdent(ident: Ident, deftbl: DefTable) -> Option<IdentEntry> {
        lookupName((identDecls(deftbl)), ident)
    }

    pub fn lookupIdentInner(ident: Ident, deftbl: DefTable) -> Option<IdentEntry> {
        lookupInnermostScope((identDecls(deftbl)), ident)
    }

    pub fn lookupLabel(ident: Ident, deftbl: DefTable) -> Option<Ident> {
        lookupName((labelDefs(deftbl)), ident)
    }

    pub fn lookupTag(sue_ref: SUERef, deftbl: DefTable) -> Option<TagEntry> {
        lookupName((tagDecls(deftbl)), sue_ref)
    }

    pub fn lookupTagInner(sue_ref: SUERef, deftbl: DefTable) -> Option<TagEntry> {
        lookupInnermostScope((tagDecls(deftbl)), sue_ref)
    }

    pub fn lookupType(dt: DefTable, n: Name) -> Option<Type> {
        IntMap::lookup((nameId(n)), (typeTable(dt)))
    }

    pub fn mergeDefTable((DefTable(i1, t1, l1, m1, r1, tt1)): DefTable, (DefTable(i2, t2, l2, m2, r2, tt2)): DefTable) -> DefTable {
        DefTable((mergeNameSpace(i1, i2)), (mergeNameSpace(t1, t2)), (mergeNameSpace(l1, l2)), (mergeNameSpace(m1, m2)), (union(r1, r2)), (union(tt1, tt2)))
    }

    pub fn tagKind(__0: TagEntry) -> TagEntryKind {
        match (__0) {
            Left(CompDecl(cd)) => {
                CompKind((compTag(cd)))
            },
            Left(EnumDecl(_)) => {
                EnumKind
            },
            Right(CompDef(cd)) => {
                CompKind((compTag(cd)))
            },
            Right(EnumDef(_)) => {
                EnumKind
            },
        }
    }

}


pub mod Language_C_Analysis_Export {
    use haskell_support::*;
    pub fn exportArraySize(__0: ArraySize) -> CArrSize {
        match (__0) {
            ArraySize(static, e) => {
                CArrSize(static, e)
            },
            UnknownArraySize(complete) => {
                CNoArrSize(complete)
            },
        }
    }

    pub fn exportAttrs() -> Vec<CAttr> {
        map(exportAttr)
    }

    pub fn exportCompType((CompType(sue_ref, comp_tag, members, attrs, node_info)): CompType) -> Vec<CTypeSpec> {
        vec![CSUType(comp, ni)]
    }

    pub fn exportCompTypeDecl(ty: CompTypeRef) -> Vec<CTypeSpec> {
        vec![CSUType((exportComp(ty)), ni)]
    }

    pub fn exportCompTypeRef((CompType(sue_ref, com_tag, _, _, node_info)): CompType) -> Vec<CTypeSpec> {
        exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
    }

    pub fn exportComplexType(ty: FloatType) -> Vec<CTypeSpec> {
        __op_concat((CComplexType(ni)), exportFloatType(ty))
    }

    pub fn exportDeclAttrs((DeclAttrs(inline, storage, attrs)): DeclAttrs) -> Vec<CDeclSpec> {
        __op_addadd((if(inline, then, vec![CTypeQual((CInlineQual(ni)))], else, vec![])), __op_addadd(map((CStorageSpec), (exportStorage(storage))), map((CTypeQual(CAttrQual)), (exportAttrs(attrs)))))
    }

    pub fn exportDeclr(other_specs: Vec<CDeclSpec>, ty: Type, attrs: Attributes, name: VarName) -> (Vec<CDeclSpec>, CDeclr) {
        (__op_addadd(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
    }

    pub fn exportEnumType((EnumType(sue_ref, enumerators, attrs, node_info)): EnumType) -> Vec<CTypeSpec> {
        vec![CEnumType(enum, ni)]
    }

    pub fn exportEnumTypeDecl(ty: EnumTypeRef) -> Vec<CTypeSpec> {
        vec![CEnumType((exportEnum(ty)), ni)]
    }

    pub fn exportEnumTypeRef((EnumType(sue_ref, _, _, node_info)): EnumType) -> Vec<CTypeSpec> {
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
                }(in, CDecl, specs, vec![(Some(declarator), None, bitfieldsz)], node_info)
            },
        }
    }

    pub fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
        {
            let (specs, declr) = exportVarDecl((getVarDecl(paramdecl)));
        }(in, CDecl, specs, vec![(Some(declr), None, None)], (nodeInfo(paramdecl)))
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
                if(reg, then, vec![CRegister(ni)], else, vec![])
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

    pub fn exportTypeDef((TypeDef(ident, ty, attrs, node_info)): TypeDef) -> CDecl {
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
            TyIntegral(ity) => {
                exportIntType(ity)
            },
            TyFloating(fty) => {
                exportFloatType(fty)
            },
            TyComplex(fty) => {
                exportComplexType(fty)
            },
            TyComp(comp) => {
                exportCompTypeDecl(comp)
            },
            TyEnum(enum) => {
                exportEnumTypeDecl(enum)
            },
            TyBuiltin(TyVaList) => {
                vec![CTypeDef((internalIdent("va_list".to_string())), ni)]
            },
            TyBuiltin(TyAny) => {
                vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)]
            },
        }
    }

    pub fn exportVarDecl((VarDecl(name, attrs, ty)): VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    }

    pub fn fromDirectType(__0: Type) -> TypeName {
        match (__0) {
            DirectType(ty, _, _) => {
                ty
            },
            TypeDefType(TypeDefRef(_, ref, _), _, _) => {
                maybe((__error!("undefined typeDef".to_string())), fromDirectType, ref)
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

}


pub mod Language_C_Analysis_NameSpaceMap {
    use haskell_support::*;
    struct NameSpaceMap<k<v>>(NsMap<Map<k, v>, Vec<Vec<(k, v)>>>);

    pub fn defGlobal((NsMap(gs, lss)): NameSpaceMap<k, a>, ident: k, def: a) -> (NameSpaceMap<k, a>, Option<a>) {
        (NsMap((Map::insert(ident, def, gs)), lss), Map::lookup(ident, gs))
    }

    pub fn defLocal(__0: NameSpaceMap<k, a>, __1: k, __2: a, __3: (NameSpaceMap<k, a>, Option<a>)) -> (NameSpaceMap<k, a>, Option<a>) {
        match (__0, __1, __2, __3, __4) {
            (ns, @, NsMap(_, []), ident, def) => {
                defGlobal(ns, ident, def)
            },
            (NsMap(gs, [ls, ...lss]), ident, def) => {
                (NsMap(gs, (__op_concat((__op_concat((ident, def), ls)), lss))), Prelude::lookup(ident, ls))
            },
        }
    }

    pub fn enterNewScope((NsMap(gs, lss)): NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
        NsMap(gs, (__op_concat(vec![], lss)))
    }

    pub fn globalNames((NsMap(g, _)): NameSpaceMap<k, v>) -> Map<k, v> {
        g
    }

    pub fn hasLocalNames((NsMap(_, l)): NameSpaceMap<k, v>) -> bool {
        not((null(l)))
    }

    pub fn leaveScope(__0: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
        match (__0) {
            NsMap(_, []) => {
                __error!("NsMaps.leaveScope: No local scope!".to_string())
            },
            NsMap(gs, [ls, ...lss]) => {
                (NsMap(gs, lss), ls)
            },
        }
    }

    pub fn localNames((NsMap(_, l)): NameSpaceMap<k, v>) -> Vec<Vec<(k, v)>> {
        l
    }

    pub fn lookupGlobal((NsMap(gs, _)): NameSpaceMap<k, a>, ident: k) -> Option<a> {
        Map::lookup(ident, gs)
    }

    pub fn lookupInnermostScope(nsm: NameSpaceMap<k, a>, @: k, (NsMap(_gs, localDefs)): Option<a>) -> Option<a> {
        match localDefs {
            ls(__id_3a, _lss) => {
                Prelude::lookup(ident, ls)
            },
            [] => {
                lookupGlobal(nsm, ident)
            },
        }
    }

    pub fn lookupName(ns: NameSpaceMap<k, a>, @: k, (NsMap(_, localDefs)): Option<a>) -> Option<a> {
        match (lookupLocal(localDefs)) {
            None => {
                lookupGlobal(ns, ident)
            },
            Some(def) => {
                Some(def)
            },
        }
    }

    pub fn mergeNameSpace((NsMap(global1, local1)): NameSpaceMap<k, a>, (NsMap(global2, local2)): NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
        NsMap((Map::union(global1, global2)), (localUnion(local1, local2)))
    }

    pub fn nameSpaceMap() -> NameSpaceMap<k, v> {
        NsMap(Map::empty, vec![])
    }

    pub fn nsMapToList((NsMap(gs, lss)): NameSpaceMap<k, a>) -> Vec<(k, a)> {
        __op_addadd(concat(lss), Map::toList(gs))
    }

}


pub mod Language_C_Analysis_SemError {
    use haskell_support::*;
    #[derive(Debug)]
    struct RedefError(RedefError<ErrorLevel, RedefInfo>);

    struct RedefInfo(RedefInfo<String, RedefKind, NodeInfo, NodeInfo>);

    pub enum RedefKind {
        DuplicateDef,
        DiffKindRedecl,
        ShadowedDef,
        DisagreeLinkage,
        NoLinkageOld
    }
    pub use self::RedefKind::*;

    #[derive(Debug)]
    struct TypeMismatch(TypeMismatch<String, (NodeInfo, Type), (NodeInfo, Type)>);

    pub fn badSpecifierError(node_info: NodeInfo, msg: String) -> BadSpecifierError {
        BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
    }

    pub fn invalidAST(node_info: NodeInfo, msg: String) -> InvalidASTError {
        InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
    }

    pub fn prevDeclMsg(old_node: NodeInfo) -> Vec<String> {
        vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
    }

    pub fn redefErrLabel((RedefInfo(ident, _, _, _)): RedefInfo) -> String {
        __op_addadd(ident, " redefined".to_string())
    }

    pub fn redefErrReason(__0: RedefInfo) -> String {
        match (__0) {
            RedefInfo(ident, DuplicateDef, _, _) => {
                __op_addadd("duplicate definition of ".to_string(), ident)
            },
            RedefInfo(ident, ShadowedDef, _, _) => {
                __op_addadd("this declaration of ".to_string(), __op_addadd(ident, " shadows a previous one".to_string()))
            },
            RedefInfo(ident, DiffKindRedecl, _, _) => {
                __op_addadd(ident, " previously declared as a different kind of symbol".to_string())
            },
            RedefInfo(ident, DisagreeLinkage, _, _) => {
                __op_addadd(ident, " previously declared with different linkage".to_string())
            },
            RedefInfo(ident, NoLinkageOld, _, _) => {
                __op_addadd(ident, " previously declared without linkage".to_string())
            },
        }
    }

    pub fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, @: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (__op_addadd(vec![redefErrReason(info)], prevDeclMsg(old_node))))
    }

    pub fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
        RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
    }

    pub fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    pub fn typeMismatchInfo((TypeMismatch(reason, (node1, _ty2), _t2)): TypeMismatch) -> ErrorInfo {
        ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
    }

}


pub mod Language_C_Analysis_SemRep {
    use haskell_support::*;
    #[derive(Clone, Debug)]
    pub enum TagDef {
        CompDef<CompType>,
        EnumDef<EnumType>
    }
    pub use self::TagDef::*;

    #[derive(Clone, Debug)]
    pub enum IdentDecl {
        Declaration<Decl>,
        ObjectDef<ObjDef>,
        FunctionDef<FunDef>,
        EnumeratorDef<Enumerator>
    }
    pub use self::IdentDecl::*;

    struct GlobalDecls(GlobalDecls<{ /* struct def */ }>);

    pub enum DeclEvent {
        TagEvent<TagDef>,
        DeclEvent<IdentDecl>,
        ParamEvent<ParamDecl>,
        LocalEvent<IdentDecl>,
        TypeDefEvent<TypeDef>,
        AsmEvent<AsmBlock>
    }
    pub use self::DeclEvent::*;

    #[derive(Clone, Debug)]
    struct Decl(Decl<VarDecl, NodeInfo>);

    #[derive(Clone, Debug)]
    struct ObjDef(ObjDef<VarDecl, Option<Initializer>, NodeInfo>);

    #[derive(Clone, Debug)]
    struct FunDef(FunDef<VarDecl, Stmt, NodeInfo>);

    #[derive(Clone, Debug)]
    pub enum ParamDecl {
        ParamDecl<VarDecl, NodeInfo>,
        AbstractParamDecl<VarDecl, NodeInfo>
    }
    pub use self::ParamDecl::*;

    #[derive(Clone, Debug)]
    pub enum MemberDecl {
        MemberDecl<VarDecl, Option<Expr>, NodeInfo>,
        AnonBitField<Type, Expr, NodeInfo>
    }
    pub use self::MemberDecl::*;

    #[derive(Clone, Debug)]
    struct TypeDef(TypeDef<Ident, Type, Attributes, NodeInfo>);

    #[derive(Clone, Debug)]
    struct VarDecl(VarDecl<VarName, DeclAttrs, Type>);

    #[derive(Clone, Debug)]
    struct DeclAttrs(DeclAttrs<bool, Storage, Attributes>);

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum Storage {
        NoStorage,
        Auto<Register>,
        Static<Linkage, ThreadLocal>,
        FunLinkage<Linkage>
    }
    pub use self::Storage::*;

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum Linkage {
        NoLinkage,
        InternalLinkage,
        ExternalLinkage
    }
    pub use self::Linkage::*;

    #[derive(Clone, Debug)]
    pub enum Type {
        DirectType<TypeName, TypeQuals, Attributes>,
        PtrType<Type, TypeQuals, Attributes>,
        ArrayType<Type, ArraySize, TypeQuals, Attributes>,
        FunctionType<FunType, Attributes>,
        TypeDefType<TypeDefRef, TypeQuals, Attributes>
    }
    pub use self::Type::*;

    #[derive(Clone, Debug)]
    pub enum FunType {
        FunType<Type, Vec<ParamDecl>, bool>,
        FunTypeIncomplete<Type>
    }
    pub use self::FunType::*;

    #[derive(Clone, Debug)]
    pub enum ArraySize {
        UnknownArraySize<bool>,
        ArraySize<bool, Expr>
    }
    pub use self::ArraySize::*;

    #[derive(Clone, Debug)]
    pub enum TypeName {
        TyVoid,
        TyIntegral<IntType>,
        TyFloating<FloatType>,
        TyComplex<FloatType>,
        TyComp<CompTypeRef>,
        TyEnum<EnumTypeRef>,
        TyBuiltin<BuiltinType>
    }
    pub use self::TypeName::*;

    #[derive(Clone, Debug)]
    pub enum BuiltinType {
        TyVaList,
        TyAny
    }
    pub use self::BuiltinType::*;

    #[derive(Clone, Debug)]
    struct TypeDefRef(TypeDefRef<Ident, Option<Type>, NodeInfo>);

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum IntType {
        TyBool,
        TyChar,
        TySChar,
        TyUChar,
        TyShort,
        TyUShort,
        TyInt,
        TyUInt,
        TyLong,
        TyULong,
        TyLLong,
        TyULLong
    }
    pub use self::IntType::*;

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum FloatType {
        TyFloat,
        TyDouble,
        TyLDouble
    }
    pub use self::FloatType::*;

    #[derive(Clone, Debug)]
    struct CompTypeRef(CompTypeRef<SUERef, CompTyKind, NodeInfo>);

    #[derive(Clone, Debug)]
    struct EnumTypeRef(EnumTypeRef<SUERef, NodeInfo>);

    #[derive(Clone, Debug)]
    struct CompType(CompType<SUERef, CompTyKind, Vec<MemberDecl>, Attributes, NodeInfo>);

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum CompTyKind {
        StructTag,
        UnionTag
    }
    pub use self::CompTyKind::*;

    #[derive(Clone, Debug)]
    struct EnumType(EnumType<SUERef, Vec<Enumerator>, Attributes, NodeInfo>);

    #[derive(Clone, Debug)]
    struct Enumerator(Enumerator<Ident, Expr, EnumType, NodeInfo>);

    #[derive(Clone, Debug)]
    struct TypeQuals(TypeQuals<{ /* struct def */ }>);

    #[derive(Clone, Debug)]
    pub enum VarName {
        VarName<Ident, Option<AsmName>>,
        NoName
    }
    pub use self::VarName::*;

    #[derive(Clone, Debug)]
    struct Attr(Attr<Ident, Vec<Expr>, NodeInfo>);

    pub fn declAttrs() -> DeclAttrs {
        (|VarDecl(_, specs, _)| { specs }(getVarDecl))
    }

    pub fn declIdent() -> Ident {
        identOfVarName(declName)
    }

    pub fn declLinkage(decl: d) -> Linkage {
        match declStorage(decl) {
            NoStorage => {
                undefined
            },
            Auto(_) => {
                NoLinkage
            },
            Static(linkage, _) => {
                linkage
            },
            FunLinkage(linkage) => {
                linkage
            },
        }
    }

    pub fn declName() -> VarName {
        (|VarDecl(n, _, _)| { n }(getVarDecl))
    }

    pub fn declOfDef(def: n) -> Decl {
        {
            let vd = getVarDecl(def, in, Decl, vd, (nodeInfo(def)));
        }
    }

    pub fn declStorage(d: d) -> Storage {
        match declAttrs(d) {
            DeclAttrs(_, st, _) => {
                st
            },
        }
    }

    pub fn declType() -> Type {
        (|VarDecl(_, _, ty)| { ty }(getVarDecl))
    }

    pub fn emptyGlobalDecls() -> GlobalDecls {
        GlobalDecls(Map::empty, Map::empty, Map::empty)
    }

    pub fn filterGlobalDecls(decl_filter: fn(DeclEvent) -> bool, gmap: GlobalDecls) -> GlobalDecls {
        GlobalDecls({
            gObjs: Map::filter((decl_filter(DeclEvent)), (gObjs(gmap))),
            gTags: Map::filter((decl_filter(TagEvent)), (gTags(gmap))),
            gTypeDefs: Map::filter((decl_filter(TypeDefEvent)), (gTypeDefs(gmap)))
        })
    }

    pub fn hasLinkage(__0: Storage) -> bool {
        match (__0) {
            Auto(_) => {
                false
            },
            Static(NoLinkage, _) => {
                false
            },
            _ => {
                true
            },
        }
    }

    pub fn identOfTypeDef((TypeDef(ide, _, _, _)): TypeDef) -> Ident {
        ide
    }

    pub fn identOfVarName(__0: VarName) -> Ident {
        match (__0) {
            NoName => {
                __error!("identOfVarName: NoName".to_string())
            },
            VarName(ident, _) => {
                ident
            },
        }
    }

    pub fn isExtDecl() -> bool {
        hasLinkage(declStorage)
    }

    pub fn isNoName(__0: VarName) -> bool {
        match (__0) {
            NoName => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn mergeAttributes() -> Attributes {
        (__op_addadd)
    }

    pub fn mergeGlobalDecls(gmap1: GlobalDecls, gmap2: GlobalDecls) -> GlobalDecls {
        GlobalDecls({
            gObjs: Map::union((gObjs(gmap1)), (gObjs(gmap2))),
            gTags: Map::union((gTags(gmap1)), (gTags(gmap2))),
            gTypeDefs: Map::union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
        })
    }

    pub fn mergeTypeQuals((TypeQuals(c1, v1, r1)): TypeQuals, (TypeQuals(c2, v2, r2)): TypeQuals) -> TypeQuals {
        TypeQuals(((c1 && c2)), ((v1 && v2)), ((r1 && r2)))
    }

    pub fn noAttributes() -> Attributes {
        vec![]
    }

    pub fn noTypeQuals() -> TypeQuals {
        TypeQuals(false, false, false)
    }

    pub fn objKindDescr(__0: IdentDecl) -> String {
        match (__0) {
            Declaration(_) => {
                "declaration".to_string()
            },
            ObjectDef(_) => {
                "object definition".to_string()
            },
            FunctionDef(_) => {
                "function definition".to_string()
            },
            EnumeratorDef(_) => {
                "enumerator definition".to_string()
            },
        }
    }

    pub fn splitIdentDecls(include_all: bool) -> (Map<Ident, Decl>, (Map<Ident, Enumerator>, Map<Ident, ObjDef>, Map<Ident, FunDef>)) {
        Map::foldWithKey((if(include_all, then, deal, else, deal_q)), (Map::empty, (Map::empty, Map::empty, Map::empty)))
    }

    pub fn typeOfCompDef((CompType(ref, tag, _, _, _)): CompType) -> TypeName {
        TyComp((CompTypeRef(ref, tag, undefNode)))
    }

    pub fn typeOfEnumDef((EnumType(ref, _, _, _)): EnumType) -> TypeName {
        TyEnum((EnumTypeRef(ref, undefNode)))
    }

    pub fn typeOfTagDef(__0: TagDef) -> TypeName {
        match (__0) {
            CompDef(comptype) => {
                typeOfCompDef(comptype)
            },
            EnumDef(enumtype) => {
                typeOfEnumDef(enumtype)
            },
        }
    }

}


pub mod Language_C_Analysis_TravMonad {
    use haskell_support::*;
    pub enum CLanguage {
        C89,
        C99,
        GNU89,
        GNU99
    }
    pub use self::CLanguage::*;

    struct TravOptions(TravOptions<{ /* struct def */ }>);

    struct TravState<s>(TravState<{ /* struct def */ }>);

    pub fn addRef(use: u, def: d) -> m<()> {
        match (nodeInfo(use), nodeInfo(def)) {
            (NodeInfo(_, _, useName), NodeInfo(_, _, defName)) => {
                withDefTable((|dt| { ((), dt({
                            refTable: insert((nameId(useName)), defName, (refTable(dt)))
                        })) }))
            },
            (_, _) => {
                ()
            },
        }
    }

    pub fn astError(node: NodeInfo, msg: String) -> m<a> {
        throwTravError(invalidAST(node, msg))
    }

    pub fn checkCompatibleTypes(_: Type, _: Type) -> Either<TypeMismatch, ()> {
        Right(())
    }

    pub fn checkIdentTyRedef(__0: IdentEntry, __1: DeclarationStatus<IdentEntry>) -> m<()> {
        match (__0, __1) {
            (Right(decl), status) => {
                checkVarRedef(decl, status)
            },
            (Left(tydef), KindMismatch(old_def)) => {
                redefErr((identOfTypeDef(tydef)), LevelError, tydef, old_def, DiffKindRedecl)
            },
            (Left(tydef), Redeclared(old_def)) => {
                redefErr((identOfTypeDef(tydef)), LevelError, tydef, old_def, DuplicateDef)
            },
            (Left(_tydef), _) => {
                ()
            },
        }
    }

    pub fn checkRedef(subject: String, new_decl: t, redecl_status: DeclarationStatus<t1>) -> m<()> {
        match redecl_status {
            NewDecl => {
                ()
            },
            Redeclared(old_def) => {
                throwTravError(redefinition(LevelError, subject, DuplicateDef, (nodeInfo(new_decl)), (nodeInfo(old_def))))
            },
            KindMismatch(old_def) => {
                throwTravError(redefinition(LevelError, subject, DiffKindRedecl, (nodeInfo(new_decl)), (nodeInfo(old_def))))
            },
            Shadowed(_old_def) => {
                ()
            },
            KeepDef(_old_def) => {
                ()
            },
        }
    }

    pub fn checkVarRedef(def: IdentDecl, redecl: DeclarationStatus<IdentEntry>) -> m<()> {
        match redecl {
            KindMismatch(old_def) => {
                redefVarErr(old_def, DiffKindRedecl)
            },
            KeepDef, Right(old_def) => if not((agreeOnLinkage(def, old_def))) { linkageErr(def, old_def) }
otherwise { throwOnLeft(checkCompatibleTypes(new_ty, (declType(old_def)))) },
            Redeclared, Right(old_def) => if not((agreeOnLinkage(def, old_def))) { linkageErr(def, old_def) }
not((canBeOverwritten(old_def))) { redefVarErr(old_def, DuplicateDef) }
otherwise { throwOnLeft(checkCompatibleTypes(new_ty, (declType(old_def)))) },
            _ => {
                ()
            },
        }
    }

    pub fn concatMapM(f: fn(a) -> m<Vec<b>>) -> m<Vec<b>> {
        liftM(concat, mapM(f))
    }

    pub fn createSUERef(_node_info: NodeInfo, (Some(ident)): Option<Ident>) -> m<SUERef> {
        return(NamedRef(ident))
    }

    pub fn enterBlockScope() -> m<()> {
        updDefTable((ST::enterBlockScope))
    }

    pub fn enterDecl(decl: Decl, cond: fn(IdentDecl) -> bool) -> m<IdentDecl> {
        /* do */ {
            {
                let def = Declaration(decl);
            };
            let redecl = withDefTable(defineScopedIdentWhen(cond, (declIdent(def)), def));
            checkVarRedef(def, redecl);
            def
        }
    }

    pub fn enterFunctionScope() -> m<()> {
        updDefTable((ST::enterFunctionScope))
    }

    pub fn enterPrototypeScope() -> m<()> {
        updDefTable((ST::enterBlockScope))
    }

    pub fn generateName() -> Trav<s, Name> {
        __op_bind(get, |ts| { /* do */ {
                {
                    let (__op_concat(new_name, gen_q)) = nameGenerator(ts);
                };
                put(ts({
                    nameGenerator: gen_q
                }));
                new_name
            } })
    }

    pub fn get() -> Trav<s, TravState<s>> {
        Trav((|s| { Right }((s, s))))
    }

    pub fn getUserState() -> Trav<s, s> {
        liftM(userState, get)
    }

    pub fn gets(f: fn(TravState<s>) -> a) -> Trav<s, a> {
        Trav((|s| { Right }((f(s), s))))
    }

    pub fn hadHardErrors() -> bool {
        (not(null(filter(isHardError))))
    }

    pub fn handleAsmBlock(asm: AsmBlock) -> m<()> {
        handleDecl((AsmEvent(asm)))
    }

    pub fn handleEnumeratorDef(enumerator: Enumerator) -> m<()> {
        /* do */ {
            {
                let ident = declIdent(enumerator);
            };
            let redecl = withDefTable(defineScopedIdent(ident, (EnumeratorDef(enumerator))));
            checkRedef((show(ident)), ident, redecl);
            ()
        }
    }

    pub fn handleFunDef(ident: Ident, fun_def: FunDef) -> m<()> {
        /* do */ {
            {
                let def = FunctionDef(fun_def);
            };
            let redecl = withDefTable(defineScopedIdentWhen(isDeclaration, ident, def));
            checkVarRedef(def, redecl);
            handleDecl((DeclEvent(def)))
        }
    }

    pub fn handleObjectDef(local: bool, ident: Ident, obj_def: ObjDef) -> m<()> {
        /* do */ {
            {
                let def = ObjectDef(obj_def);
            };
            let redecl = withDefTable(defineScopedIdentWhen((|old| { shouldOverride }(def, old)), ident, def));
            checkVarRedef(def, redecl);
            handleDecl(((if(local, then, LocalEvent, else, DeclEvent))(def)));

        }
    }

    pub fn handleParamDecl(__0: ParamDecl, __1: m<()>) -> m<()> {
        match (__0, __1, __2) {
            (pd, @, AbstractParamDecl(_, _)) => {
                handleDecl((ParamEvent(pd)))
            },
            (pd, @, ParamDecl(vardecl, node)) => {
                /* do */ {
                    {
                        let def = ObjectDef((ObjDef(vardecl, None, node)));
                    };
                    let redecl = withDefTable(defineScopedIdent((declIdent(def)), def));
                    checkVarRedef(def, redecl);
                    handleDecl((ParamEvent(pd)))
                }
            },
        }
    }

    pub fn handleTagDecl(decl: TagFwdDecl) -> m<()> {
        /* do */ {
            let redecl = withDefTable(declareTag((sueRef(decl)), decl));
            checkRedef((show(sueRef(decl))), decl, redecl)
        }
    }

    pub fn handleTagDef(def: TagDef) -> m<()> {
        /* do */ {
            let redecl = withDefTable(defineTag((sueRef(def)), def));
            checkRedef((show(sueRef(def))), def, redecl);
            handleDecl((TagEvent(def)))
        }
    }

    pub fn handleTravError(a: m<a>) -> m<Option<a>> {
        catchTravError(liftM(Some, a), (__op_rshift(|e| { recordError }(e), None)))
    }

    pub fn handleTypeDef(typeDef: TypeDef, @: m<()>) -> m<()> {
        /* do */ {
            let redecl = withDefTable(defineTypeDef(ident, typeDef));
            checkRedef((show(ident)), typeDef, redecl);
            handleDecl((TypeDefEvent(typeDef)));
            ()
        }
    }

    pub fn handleVarDecl(is_local: bool, decl: Decl) -> m<()> {
        /* do */ {
            let def = enterDecl(decl, (const(false)));
            handleDecl(((if(is_local, then, LocalEvent, else, DeclEvent))(def)))
        }
    }

    pub fn initTravState(userst: s) -> TravState<s> {
        TravState({
            symbolTable: emptyDefTable,
            rerrors: RList::empty,
            nameGenerator: newNameSupply,
            doHandleExtDecl: const((())),
            userState: userst,
            options: TravOptions({
                        language: C99
                    })
        })
    }

    pub fn isDeclaration(__0: IdentDecl) -> bool {
        match (__0) {
            Declaration(_) => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn leaveBlockScope() -> m<()> {
        updDefTable((ST::leaveBlockScope))
    }

    pub fn leaveFunctionScope() -> m<()> {
        updDefTable((ST::leaveFunctionScope))
    }

    pub fn leavePrototypeScope() -> m<()> {
        updDefTable((ST::leaveBlockScope))
    }

    pub fn lookupObject(ident: Ident) -> m<Option<IdentDecl>> {
        /* do */ {
            let old_decl = liftM((lookupIdent(ident)), getDefTable);
            mapMaybeM(old_decl)(|obj| { match obj {
                    Right(objdef) => {
                        __op_rshift(addRef(ident, objdef), objdef)
                    },
                    Left(_tydef) => {
                        astError((nodeInfo(ident)), (mismatchErr("lookupObject".to_string(), "an object".to_string(), "a typeDef".to_string())))
                    },
                } })
        }
    }

    pub fn lookupTypeDef(ident: Ident) -> m<Type> {
        __op_bind(getDefTable, |symt| { match lookupIdent(ident, symt) {
                None => {
                    astError((nodeInfo(ident)))(__op_addadd("unbound typeDef: ".to_string(), identToString(ident)))
                },
                Some(Left(TypeDef(def_ident, ty, _, _))) => {
                    __op_rshift(addRef(ident, def_ident), ty)
                },
                Some(Right(d)) => {
                    astError((nodeInfo(ident)), (wrongKindErrMsg(d)))
                },
            } })
    }

    pub fn mapMaybeM(m: Option<a>, f: fn(a) -> m<b>) -> m<Option<b>> {
        maybe((None), (liftM(Some, f)), m)
    }

    pub fn mapSndM(f: fn(b) -> m<c>, (a, b): (a, b)) -> m<(a, c)> {
        liftM((<Expr::Dummy>(a)), (f(b)))
    }

    pub fn maybeM(m: Option<a>, f: fn(a) -> m<()>) -> m<()> {
        maybe((()), f, m)
    }

    pub fn mismatchErr(ctx: String, expect: String, found: String) -> String {
        __op_addadd(ctx, __op_addadd(": Expected ".to_string(), __op_addadd(expect, __op_addadd(", but found: ".to_string(), found))))
    }

    pub fn modify(f: fn(TravState<s>) -> TravState<s>) -> Trav<s, ()> {
        Trav((|s| { Right }(((), f(s)))))
    }

    pub fn modifyOptions(f: fn(TravOptions) -> TravOptions) -> Trav<s, ()> {
        modify(|ts| { ts }({
            options: f((options(ts)))
        }))
    }

    pub fn modifyUserState(f: fn(s) -> s) -> Trav<s, ()> {
        modify(|ts| { ts }({
            userState: f((userState(ts)))
        }))
    }

    pub fn put(s: TravState<s>) -> Trav<s, ()> {
        Trav((|_| { Right }(((), s))))
    }

    pub fn redefErr(name: Ident, lvl: ErrorLevel, new: new, old: old, kind: RedefKind) -> m<()> {
        throwTravError(redefinition(lvl, (show(name)), kind, (nodeInfo(new)), (nodeInfo(old))))
    }

    pub fn runTrav(state: forall<s, a::, s>, traversal: Trav<s, a>) -> Either<Vec<CError>, (a, TravState<s>)> {
        match unTrav(action, (initTravState(state))) {
            Left(trav_err) => {
                Left(vec![trav_err])
            },
            Right, (v, ts) => if hadHardErrors((travErrors(ts))) { Left((travErrors(ts))) }
otherwise { Right((v, ts)) },
        }
    }

    pub fn runTrav_(t: Trav<(), a>) -> Either<Vec<CError>, (a, Vec<CError>)> {
        fmap(fst, runTrav(())(/* do */ {
                let r = t;
                let es = getErrors;
                (r, es)
            }))
    }

    pub fn throwOnLeft(__0: Either<e, a>) -> m<a> {
        match (__0) {
            Left(err) => {
                throwTravError(err)
            },
            Right(v) => {
                v
            },
        }
    }

    pub fn travErrors() -> Vec<CError> {
        RList::reverse(rerrors)
    }

    pub fn updDefTable(f: fn(DefTable) -> DefTable) -> m<()> {
        withDefTable((|st| { ((), f(st)) }))
    }

    pub fn warn(err: e) -> m<()> {
        recordError((changeErrorLevel(err, LevelWarn)))
    }

    pub fn withExtDeclHandler(action: Trav<s, a>, handler: fn(DeclEvent) -> Trav<s, ()>) -> Trav<s, a> {
        /* do */ {
            modify(|st| { st }({
                doHandleExtDecl: handler
            }));
            action
        }
    }

}


pub mod Language_C_Analysis_TypeCheck {
    use haskell_support::*;
    pub fn assignCompatible(__0: CAssignOp, __1: Type, __2: Type) -> Either<String, ()> {
        match (__0, __1, __2) {
            (CAssignOp, t1, t2) => {
                match (canonicalType(t1), canonicalType(t2)) {
                    (DirectType(TyBuiltin(TyAny), _, _), _) => {
                        ()
                    },
                    (_, DirectType(TyBuiltin(TyAny), _, _)) => {
                        ()
                    },
                    (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) => if isPointerType(t2_q) { () },
                    (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1_q) { () },
                    (PtrType(_, _, _), t2_q) => if isIntegralType(t2_q) { () },
                    (t1_q, t2_q) => if (isPointerType(t1_q) && isPointerType(t2_q)) { /* do */ {
                    compatible((baseType(t1_q)), (baseType(t2_q)))
                } },
                    (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) => if (sueRef(c1) == sueRef(c2)) { () }
otherwise { fail(__op_addadd("incompatible compound types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) },
                    (DirectType(TyBuiltin(TyVaList), _, _), DirectType(TyBuiltin(TyVaList), _, _)) => {
                        ()
                    },
                    (DirectType(tn1, _, _), DirectType(tn2, _, _)) => if isJust((arithmeticConversion(tn1, tn2))) { () }
otherwise { fail(__op_addadd("incompatible direct types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) },
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
            (_, t1_q, t2_q) => if isLogicOp(op) { __op_rshift(checkScalar(t1_q), __op_rshift(checkScalar(t2_q), boolType)) }
isCmpOp(op) { match (t1_q, t2_q) {
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
            (PtrType(DirectType(TyVoid, _, _), _, _), _) => if isPointerType(t2_q) { boolType },
            (_, PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1_q) { boolType },
            (_, _) => if (isPointerType(t1_q) && isIntegralType(t2_q)) { boolType }
(isIntegralType(t1_q) && isPointerType(t2_q)) { boolType }
(isPointerType(t1_q) && isPointerType(t2_q)) { __op_rshift(compatible(t1_q, t2_q), boolType) },
            (_, _) => {
                fail("incompatible types in comparison".to_string())
            },
        } },
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
            (_, PtrType(_, _, _), t2_q) => if (isPtrOp(op) && isIntegralType(t2_q)) { t1 }
otherwise { fail(__op_addadd("invalid pointer operation: ".to_string(), render((pretty(op))))) },
            (CAddOp, t1_q, PtrType(_, _, _)) => if isIntegralType(t1_q) { t2 },
            (_, ArrayType(_, _, _, _), t2_q) => if (isPtrOp(op) && isIntegralType(t2_q)) { t1 }
otherwise { fail(__op_addadd("invalid pointer operation: ".to_string(), render((pretty(op))))) },
            (CAddOp, t1_q, ArrayType(_, _, _, _)) => if isIntegralType(t1_q) { t2 },
            (_, DirectType(tn1, q1, a1), DirectType(tn2, q2, a2)) => {
                /* do */ {
                    when((isBitOp(op)), (__op_rshift(checkIntegral(t1), checkIntegral(t2))));
                    match arithmeticConversion(tn1, tn2) {
                        Some(tn) => {
                            return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2))))
                        },
                        None => {
                            fail(render(<+>(text("invalid binary operation:".to_string()), <+>(pretty(t1), <+>(pretty(op), pretty(t2))))))
                        },
                    }
                }
            },
            (_, _, _) => {
                fail(render(<+>(text("unhandled binary operation:".to_string()), <+>(pretty(t1), <+>(pretty(op), pretty(t2))))))
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

    pub fn compositeDeclAttrs((DeclAttrs(inl, stor, attrs1)): DeclAttrs, (DeclAttrs(_, _, attrs2)): DeclAttrs) -> DeclAttrs {
        DeclAttrs(inl, stor, (mergeAttrs(attrs1, attrs2)))
    }

    pub fn compositeParamDecl(__0: ParamDecl, __1: ParamDecl) -> Either<String, ParamDecl> {
        match (__0, __1) {
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

    pub fn compositeParamDecl_q(f: fn(VarDecl) -> fn(NodeInfo) -> ParamDecl, (VarDecl(n1, attrs1, t1)): VarDecl, (VarDecl(n2, attrs2, t2)): VarDecl, dni: NodeInfo) -> Either<String, ParamDecl> {
        /* do */ {
            let vd = compositeVarDecl((VarDecl(n1, attrs1, t1_q)), (VarDecl(n2, attrs2, t2_q)));
            return(f(vd, dni))
        }
    }

    pub fn compositeSize(__0: ArraySize, __1: ArraySize) -> Either<String, ArraySize> {
        match (__0, __1) {
            (UnknownArraySize(_), s2) => {
                s2
            },
            (s1, UnknownArraySize(_)) => {
                s1
            },
        }
    }

    pub fn compositeType(__0: Type, __1: Type) -> Either<String, Type> {
        match (__0, __1) {
            (t1, DirectType(TyBuiltin(TyAny), _, _)) => {
                t1
            },
            (DirectType(TyBuiltin(TyAny), _, _), t2) => {
                t2
            },
            (t1, @, DirectType(tn1, q1, a1), t2, @, DirectType(tn2, q2, a2)) => {
                /* do */ {
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
                            /* do */ {
                                when((/=(sueRef(c1), sueRef(c2))))(fail(__op_addadd("incompatible composite types: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))));
                                tn1
                            }
                        },
                        (TyEnum(e1), TyEnum(e2)) => {
                            /* do */ {
                                when((/=(sueRef(e1), sueRef(e2))))(fail(__op_addadd("incompatible enumeration types: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))));
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
            (ArrayType(t1, s1, q1, a1), ArrayType(t2, s2, q2, a2)) => {
                /* do */ {
                    let t = compositeType(t1, t2);
                    let s = compositeSize(s1, s2);
                    {
                        let quals = mergeTypeQuals(q1, q2);
;
                        let attrs = mergeAttrs(a1, a2);
                    };
                    (ArrayType(t, s, quals, attrs))
                }
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
                        /* do */ {
                            let args = mapM((uncurry(compositeParamDecl)), (zip(args1, args2)));
                            when((/=(varargs1, varargs2)))(fail("incompatible varargs declarations".to_string()));
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
                        /* do */ {
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

    pub fn compositeVarDecl((VarDecl(n1, attrs1, t1)): VarDecl, (VarDecl(_, attrs2, t2)): VarDecl) -> Either<String, VarDecl> {
        /* do */ {
            let t = compositeType(t1, t2);
            (VarDecl(n1, (compositeDeclAttrs(attrs1, attrs2)), t))
        }
    }

    pub fn conditionalType(t1: Type, t2: Type) -> Either<String, Type> {
        match (canonicalType(t1), canonicalType(t2)) {
            (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) => if isPointerType(t2_q) { t2 },
            (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1_q) { t1 },
            (ArrayType(t1_q, _, q1, a1), ArrayType(t2_q, _, q2, a2)) => {
                /* do */ {
                    let t = compositeType(t1_q, t2_q);
                    return(ArrayType(t, (UnknownArraySize(false)), (mergeTypeQuals(q1, q2)), (mergeAttrs(a1, a2))))
                }
            },
            (t1_q(@, DirectType(tn1, q1, a1)), t2_q(@, DirectType(tn2, q2, a2))) => {
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

    pub fn constType(__0: CConst) -> m<Type> {
        match (__0) {
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
                /* do */ {
                    let n = genName;
                    {
;
                        let ni_q = mkNodeInfo((posOf(ni)), n);
;
                        let arraySize = ArraySize(true, (CConst((CIntConst((cInteger((toInteger((length(chars)))))), ni_q)))));
                    };
                    return(ArrayType((DirectType((TyIntegral(charType)), noTypeQuals, noAttributes)), arraySize, noTypeQuals, vec![]))
                }
            },
        }
    }

    pub fn deepTypeAttrs(__0: Type) -> m<Attributes> {
        match (__0) {
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

    pub fn derefType(__0: Type) -> Either<String, Type> {
        match (__0) {
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

    pub fn expandAnonymous(__0: NodeInfo, __1: (VarName, Type)) -> m<Vec<(Ident, Type)>> {
        match (__0, __1) {
            (ni, NoName(DirectType(TyComp(ctr), _, _))) => {
                __op_bind(lookupSUE(ni, (sueRef(ctr))), tagMembers(ni))
            },
            (_, NoName(_)) => {
                vec![]
            },
            (_, VarName(n, _)(t)) => {
                vec![(n, t)]
            },
        }
    }

    pub fn fieldType(ni: NodeInfo, m: Ident, t: Type) -> m<Type> {
        match canonicalType(t) {
            DirectType(TyComp(ctr), _, _) => {
                /* do */ {
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
        /* do */ {
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

    pub fn notFound(i: Ident) -> Either<String, a> {
        fail(__op_addadd("not found: ".to_string(), identToString(i)))
    }

    pub fn pType() -> String {
        render(pretty)
    }

    pub fn sizeEqual(__0: CExpr, __1: CExpr) -> bool {
        match (__0, __1) {
            (CConst(CIntConst(i1, _)), CConst(CIntConst(i2, _))) => {
                (i1 == i2)
            },
            (e1, e2) => {
                (nodeInfo(e1) == nodeInfo(e2))
            },
        }
    }

    pub fn sueAttrs(ni: NodeInfo, sue: SUERef) -> m<Attributes> {
        /* do */ {
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
        /* do */ {
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

    pub fn typeError() -> m<a> {
        astError
    }

    pub fn typeErrorOnLeft(__0: NodeInfo, __1: Either<String, a>) -> m<a> {
        match (__0, __1) {
            (ni, Left(err)) => {
                typeError(ni, err)
            },
            (_, Right(v)) => {
                v
            },
        }
    }

    pub fn varAddrType(d: IdentDecl) -> Either<String, Type> {
        /* do */ {
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

}


pub mod Language_C_Analysis_TypeConversions {
    use haskell_support::*;
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
            (t1, @, TyComplex(_), TyIntegral(_)) => {
                Some(t1)
            },
            (TyIntegral(_), t2, @, TyComplex(_)) => {
                Some(t2)
            },
            (TyFloating(t1), TyFloating(t2)) => {
                Some(TyFloating(floatConversion(t1, t2)))
            },
            (t1, @, TyFloating(_), TyIntegral(_)) => {
                Some(t1)
            },
            (TyIntegral(_), t2, @, TyFloating(_)) => {
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

}


pub mod Language_C_Analysis_TypeUtils {
    use haskell_support::*;
    pub fn baseType(__0: Type) -> Type {
        match (__0) {
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
        PtrType(t, (TypeQuals(true, false, false)), vec![])
    }

    pub fn constVoidPtr() -> Type {
        constPtr(voidType)
    }

    pub fn deepDerefTypeDef(__0: Type) -> Type {
        match (__0) {
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

    pub fn derefTypeDef(__0: Type) -> Type {
        match (__0) {
            TypeDefType(TypeDefRef(_, Some(t), _), q, a) => {
                (typeAttrsUpd((mergeAttributes(a)), typeQualsUpd((mergeTypeQuals(q)))))((derefTypeDef(t)))
            },
            ty => {
                ty
            },
        }
    }

    pub fn floating(ty: FloatType) -> Type {
        DirectType((TyFloating(ty)), noTypeQuals, noAttributes)
    }

    pub fn integral(ty: IntType) -> Type {
        DirectType((TyIntegral(ty)), noTypeQuals, noAttributes)
    }

    pub fn isFloatingType(__0: Type) -> bool {
        match (__0) {
            DirectType(TyFloating(_), _, _) => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn isFunctionType(ty: Type) -> bool {
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

    pub fn isIntegralType(__0: Type) -> bool {
        match (__0) {
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

    pub fn isPointerType(__0: Type) -> bool {
        match (__0) {
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

    pub fn isScalarType(t: Type) -> bool {
        (isIntegralType(t) || (isPointerType(t) || isFloatingType(t)))
    }

    pub fn ptrDiffType() -> Type {
        integral(TyInt)
    }

    pub fn simplePtr(t: Type) -> Type {
        PtrType(t, noTypeQuals, vec![])
    }

    pub fn size_tType() -> Type {
        integral(TyInt)
    }

    pub fn stringType() -> Type {
        ArrayType((DirectType((TyIntegral(TyChar)), (TypeQuals(true, false, false)), noAttributes)), (UnknownArraySize(false)), noTypeQuals, vec![])
    }

    pub fn testFlags(flags: Vec<f>, fi: Flags<f>) -> bool {
        and(map(((flip(testFlag))(fi)), flags))
    }

    pub fn typeAttrs(__0: Type) -> Attributes {
        match (__0) {
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

    pub fn typeQuals(__0: Type) -> TypeQuals {
        match (__0) {
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

    pub fn valistType() -> Type {
        DirectType((TyBuiltin(TyVaList)), noTypeQuals, noAttributes)
    }

    pub fn voidPtr() -> Type {
        simplePtr(voidType)
    }

    pub fn voidType() -> Type {
        DirectType(TyVoid, noTypeQuals, noAttributes)
    }

}


pub mod Language_C_Analysis {
    use haskell_support::*;

}


pub mod Language_C_Data_Error {
    use haskell_support::*;
    #[derive(Eq, Ord)]
    pub enum ErrorLevel {
        LevelWarn,
        LevelError,
        LevelFatal
    }
    pub use self::ErrorLevel::*;

    #[derive(Debug)]
    struct ErrorInfo(ErrorInfo<ErrorLevel, Position, Vec<String>>);

    #[derive(Debug)]
    struct CError(CError<err>);

    #[derive(Debug)]
    struct UnsupportedFeature(UnsupportedFeature<String, Position>);

    pub fn errorLevel() -> ErrorLevel {
        (|ErrorInfo(lvl, _, _)| { lvl }(errorInfo))
    }

    pub fn errorMsgs() -> Vec<String> {
        (|ErrorInfo(_, _, msgs)| { msgs }(errorInfo))
    }

    pub fn errorPos() -> Position {
        (|ErrorInfo(_, pos, _)| { pos }(errorInfo))
    }

    pub fn indent() -> String {
        "  ".to_string()
    }

    pub fn indentLines() -> String {
        unlines(map((indent(__op_addadd)), lines))
    }

    pub fn internalErr(msg: String) -> a {
        __error!((__op_addadd(internalErrPrefix, __op_addadd("\n".to_string(), __op_addadd(indentLines(msg), "\n".to_string())))))
    }

    pub fn internalErrPrefix() -> String {
        unlines(vec![
                "Language.C : Internal Error".to_string(),
                __op_addadd("This is propably a bug, and should be reported at ".to_string(), "http://www.sivity.net/projects/language.c/newticket".to_string()),
            ])
    }

    pub fn isHardError() -> bool {
        (>((), LevelWarn(errorLevel)))
    }

    pub fn mkErrorInfo(lvl: ErrorLevel, msg: String, node: NodeInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (lines(msg)))
    }

    pub fn showError(short_msg: String) -> String {
        showErrorInfo(short_msg, errorInfo)
    }

    pub fn showErrorInfo(short_msg: String, (ErrorInfo(level, pos, msgs)): ErrorInfo) -> String {
        __op_addadd(header, showMsgLines((__op_concat(if(null, short_msg, then, msgs, else, short_msg), msgs))))
    }

    pub fn unsupportedFeature(msg: String, a: a) -> UnsupportedFeature {
        UnsupportedFeature(msg, (posOf(a)))
    }

    pub fn unsupportedFeature_(msg: String) -> UnsupportedFeature {
        UnsupportedFeature(msg, internalPos)
    }

    pub fn userErr(msg: String) -> UserError {
        UserError((ErrorInfo(LevelError, internalPos, (lines(msg)))))
    }

}


pub mod Language_C_Data_Ident {
    use haskell_support::*;
    #[derive(Clone, Debug, Eq, Ord)]
    pub enum SUERef {
        AnonymousRef<Name>,
        NamedRef<Ident>
    }
    pub use self::SUERef::*;

    #[derive(Clone, Debug)]
    struct Ident(Ident<String, isize, NodeInfo>);

    pub fn bits14() -> isize {
        ^(2, (14))
    }

    pub fn bits21() -> isize {
        ^(2, (21))
    }

    pub fn bits28() -> isize {
        ^(2, (28))
    }

    pub fn bits7() -> isize {
        ^(2, (7))
    }

    pub fn builtinIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
    }

    pub fn dumpIdent(ide: Ident) -> String {
        __op_addadd(identToString(ide), __op_addadd(" at ".to_string(), show((nodeInfo(ide)))))
    }

    pub fn identToString((Ident(s, _, _)): Ident) -> String {
        s
    }

    pub fn internalIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
    }

    pub fn internalIdentAt(pos: Position, s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
    }

    pub fn isAnonymousRef(__0: SUERef) -> bool {
        match (__0) {
            AnonymousRef(_) => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn isInternalIdent((Ident(_, _, nodeinfo)): Ident) -> bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    pub fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo_q(pos, (pos, length(s)), name)))
    }

    pub fn quad(__0: String) -> isize {
        match (__0) {
            [c1, ...[c2, ...[c3, ...[c4, ...s]]]] => {
                +((mod(((ord(c4) * +(bits21, (ord(c3) * +(bits14, (ord(c2) * +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28)))
            },
            [c1, ...[c2, ...[c3, ...[]]]] => {
                (ord(c3) * +(bits14, (ord(c2) * +(bits7, ord(c1)))))
            },
            [c1, ...[c2, ...[]]] => {
                (ord(c2) * +(bits7, ord(c1)))
            },
            [c1, ...[]] => {
                ord(c1)
            },
            [] => {
                0
            },
        }
    }

}


pub mod Language_C_Data_InputStream {
    use haskell_support::*;
    pub fn countLines() -> isize {
        match () {
            () => {
                length(BSC::lines)
            },
            () => {
                length(lines)
            },
        }
    }

    pub fn inputStreamEmpty() -> bool {
        match () {
            () => {
                BSW::null
            },
            () => {
                null
            },
        }
    }

    pub fn inputStreamFromString() -> InputStream {
        match () {
            () => {
                BSC::pack
            },
            () => {
                id
            },
        }
    }

    pub fn inputStreamToString() -> String {
        match () {
            () => {
                BSC::unpack
            },
            () => {
                id
            },
        }
    }

    pub fn readInputStream() -> IO<InputStream> {
        match () {
            () => {
                BSW::readFile
            },
            () => {
                readFile
            },
        }
    }

    pub fn takeByte(bs: InputStream) -> (Word8, InputStream) {
        seq(BSW::head(bs), (BSW::head(bs), BSW::tail(bs)))
    }

    pub fn takeChar(__0: InputStream) -> (Char, InputStream) {
        match (__0) {
            bs => {
                seq(BSC::head(bs), (BSC::head(bs), BSC::tail(bs)))
            },
            bs => {
                (head(bs), tail(bs))
            },
        }
    }

    pub fn takeChars(__0: isize, __1: InputStream, __2: Vec<Char>) -> Vec<Char> {
        match (__0, __1, __2) {
            (!, n, bstr) => {
                BSC::unpack(BSC::take(n, bstr))
            },
            (n, __str) => {
                take(n, __str)
            },
        }
    }

}


pub mod Language_C_Data_Name {
    use haskell_support::*;
    pub fn namesStartingFrom(k: isize) -> Vec<Name> {
        vec![Name(k::::)]
    }

    pub fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    }

}


pub mod Language_C_Data_Node {
    use haskell_support::*;
    #[derive(Clone, Debug)]
    pub enum NodeInfo {
        OnlyPos<Position, PosLength>,
        NodeInfo<Position, PosLength, Name>
    }
    pub use self::NodeInfo::*;

    pub fn eqByName(obj1: a, obj2: a) -> bool {
        ((nodeInfo(obj1)) == (nodeInfo(obj2)))
    }

    pub fn fileOfNode() -> Option<FilePath> {
        fmap(posFile, justIf(isSourcePos, posOfNode(nodeInfo)))
    }

    pub fn getLastTokenPos(__0: NodeInfo) -> PosLength {
        match (__0) {
            NodeInfo(_, lastTok, _) => {
                lastTok
            },
            OnlyPos(_, lastTok) => {
                lastTok
            },
        }
    }

    pub fn internalNode() -> NodeInfo {
        undefNode
    }

    pub fn isUndefNode(_: NodeInfo) -> bool {
        false
    }

    pub fn lengthOfNode(ni: NodeInfo) -> Option<isize> {
        len
    }

    pub fn mkNodeInfo(pos: Position, name: Name) -> NodeInfo {
        NodeInfo(pos, (nopos, -(1)), name)
    }

    pub fn mkNodeInfoOnlyPos(pos: Position) -> NodeInfo {
        OnlyPos(pos, (nopos, -(1)))
    }

    pub fn mkNodeInfoPosLen() -> NodeInfo {
        OnlyPos
    }

    pub fn mkNodeInfo_q(pos: Position, lasttok: PosLength, name: Name) -> NodeInfo {
        NodeInfo(pos, lasttok, name)
    }

    pub fn nameOfNode(__0: NodeInfo) -> Option<Name> {
        match (__0) {
            OnlyPos(_, _) => {
                None
            },
            NodeInfo(_, _, name) => {
                Some(name)
            },
        }
    }

    pub fn posOfNode(ni: NodeInfo) -> Position {
        match ni {
            OnlyPos(pos, _) => {
                pos
            },
            NodeInfo(pos, _, _) => {
                pos
            },
        }
    }

    pub fn undefNode() -> NodeInfo {
        OnlyPos(nopos, (nopos, -(1)))
    }

}


pub mod Language_C_Data_Position {
    use haskell_support::*;
    #[derive(Clone, Debug, Eq, Ord)]
    pub enum Position {
        Position<{ /* struct def */ }>,
        NoPosition,
        BuiltinPosition,
        InternalPosition
    }
    pub use self::Position::*;

    pub fn adjustPos(__0: FilePath, __1: isize, __2: Position) -> Position {
        match (__0, __1, __2) {
            (fname, row, Position(offs, _, _, _)) => {
                Position(offs, fname, row, 1)
            },
            (_, _, p) => {
                p
            },
        }
    }

    pub fn builtinPos() -> Position {
        BuiltinPosition
    }

    pub fn incOffset(__0: Position, __1: isize) -> Position {
        match (__0, __1) {
            (Position(o, f, r, c), n) => {
                Position((+(o, n)), f, r, c)
            },
            (p, n) => {
                p
            },
        }
    }

    pub fn incPos(__0: Position, __1: isize) -> Position {
        match (__0, __1) {
            (Position(offs, fname, row, col), n) => {
                Position((+(offs, n)), fname, row, (+(col, n)))
            },
            (p, _) => {
                p
            },
        }
    }

    pub fn initPos(file: FilePath) -> Position {
        Position(0, file, 1, 1)
    }

    pub fn internalPos() -> Position {
        InternalPosition
    }

    pub fn isBuiltinPos(__0: Position) -> bool {
        match (__0) {
            BuiltinPosition => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn isInternalPos(__0: Position) -> bool {
        match (__0) {
            InternalPosition => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn isNoPos(__0: Position) -> bool {
        match (__0) {
            NoPosition => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn isSourcePos(__0: Position) -> bool {
        match (__0) {
            Position(_, _, _, _) => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn nopos() -> Position {
        NoPosition
    }

    pub fn position() -> Position {
        Position
    }

    pub fn retPos(__0: Position) -> Position {
        match (__0) {
            Position(offs, fname, row, _) => {
                Position((+(offs, 1)), fname, (+(row, 1)), 1)
            },
            p => {
                p
            },
        }
    }

}


pub mod Language_C_Data_RList {
    use haskell_support::*;
    pub fn appendr(xs: Vec<a>, (Reversed(ys)): Reversed<Vec<a>>) -> Reversed<Vec<a>> {
        Reversed((__op_addadd(ys, List::reverse(xs))))
    }

    pub fn empty() -> Reversed<Vec<a>> {
        Reversed(vec![])
    }

    pub fn rappend((Reversed(xs)): Reversed<Vec<a>>, ys: Vec<a>) -> Reversed<Vec<a>> {
        Reversed((__op_addadd(List::reverse(ys), xs)))
    }

    pub fn rappendr((Reversed(xs)): Reversed<Vec<a>>, (Reversed(ys)): Reversed<Vec<a>>) -> Reversed<Vec<a>> {
        Reversed((__op_addadd(ys, xs)))
    }

    pub fn reverse((Reversed(xs)): Reversed<Vec<a>>) -> Vec<a> {
        List::reverse(xs)
    }

    pub fn rmap(f: fn(a) -> b, (Reversed(xs)): Reversed<Vec<a>>) -> Reversed<Vec<b>> {
        Reversed((map(f, xs)))
    }

    pub fn singleton(x: a) -> Reversed<Vec<a>> {
        Reversed(vec![x])
    }

    pub fn snoc((Reversed(xs)): Reversed<Vec<a>>, x: a) -> Reversed<Vec<a>> {
        Reversed((__op_concat(x, xs)))
    }

    pub fn viewr(__0: Reversed<Vec<a>>) -> (Reversed<Vec<a>>, a) {
        match (__0) {
            Reversed([]) => {
                __error!("viewr: empty RList".to_string())
            },
            Reversed([x, ...xs]) => {
                (Reversed(xs), x)
            },
        }
    }

    snoc(infixr(5), ())
}


pub mod Language_C_Data {
    use haskell_support::*;

}


pub mod Language_C_Parser_Builtin {
    use haskell_support::*;
    pub fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}


pub mod Language_C_Parser_ParserMonad {
    use haskell_support::*;
    pub enum ParseResult<a> {
        POk<PState, a>,
        PFailed<Vec<String>, Position>
    }
    pub use self::ParseResult::*;

    struct PState(PState<{ /* struct def */ }>);

    let (P(m)) = |thenP, k| {
        P(|s| { match m(s) {
                POk(s_q, a) => {
                    (unP((k(a))))(s_q)
                },
                PFailed(err, pos) => {
                    PFailed(err, pos)
                },
            } })
    };

    pub fn addTypedef(ident: Ident) -> P<()> {
        (P(|s, @, PState, { .. }| { POk }(s, {
            tyidents: Set.insert(ident, tyids)
        }, ())))
    }

    pub fn enterScope() -> P<()> {
        P(|s, @, PState, { .. }| { POk }(s, {
            scopes: __op_concat(tyids, ss)
        }, ()))
    }

    pub fn execParser((P(parser)): P<a>, input: InputStream, pos: Position, builtins: Vec<Ident>, names: Vec<Name>) -> Either<ParseError, (a, Vec<Name>)> {
        match parser(initialState) {
            PFailed(message, errpos) => {
                Left((ParseError((message, errpos))))
            },
            POk(st, result) => {
                Right((result, namesupply(st)))
            },
        }
    }

    pub fn failP(pos: Position, msg: Vec<String>) -> P<a> {
        P(|_| { PFailed }(msg, pos))
    }

    pub fn getCurrentPosition() -> P<Position> {
        P(|s, @, PState, { .. }| { POk }(s, pos))
    }

    pub fn getInput() -> P<InputStream> {
        P(|s, @, PState, { .. }| { POk }(s, i))
    }

    pub fn getLastToken() -> P<CToken> {
        P(|s, @, PState, { .. }| { POk }(s, tok))
    }

    pub fn getNewName() -> P<Name> {
        P(seq(|s, @, PState, { .. }| { n }, POk(s, {
            namesupply: ns
        }, n)))
    }

    pub fn getPos() -> P<Position> {
        P(|s, @, PState, { .. }| { POk }(s, pos))
    }

    pub fn getSavedToken() -> P<CToken> {
        P(|s, @, PState, { .. }| { POk }(s, tok))
    }

    pub fn handleEofToken() -> P<()> {
        P(|s| { POk }(s, {
            savedToken: (prevToken(s))
        }, ()))
    }

    pub fn isTypeIdent(ident: Ident) -> P<bool> {
        P($!(|s, @, PState, { .. }| { POk }(s), Set::member(ident, tyids)))
    }

    pub fn leaveScope() -> P<()> {
        P(|s, @, PState, { .. }| { match ss {
                [] => {
                    __error!("leaveScope: already in global scope".to_string())
                },
                tyids(__id_3a, ss_q) => {
                    POk(s, {
                        tyidents: tyids,
                        scopes: ss_q
                    }, ())
                },
            } })
    }

    pub fn returnP(a: a) -> P<a> {
        P(|s| { POk }(s, a))
    }

    pub fn setInput(i: InputStream) -> P<()> {
        P(|s| { POk }(s, {
            curInput: i
        }, ()))
    }

    pub fn setLastToken(__0: CToken) -> P<()> {
        match (__0) {
            CTokEof => {
                P(|s| { POk }(s, {
                    savedToken: (prevToken(s))
                }, ()))
            },
            tok => {
                P(|s| { POk }(s, {
                    prevToken: tok,
                    savedToken: (prevToken(s))
                }, ()))
            },
        }
    }

    pub fn setPos(pos: Position) -> P<()> {
        P(|s| { POk }(s, {
            curPos: pos
        }, ()))
    }

    pub fn shadowTypedef(ident: Ident) -> P<()> {
        (P(|s, @, PState, { .. }| { POk }(s, {
            tyidents: Set.member(if(ident), Set.delete(tyids(then, ident), tyids(else, tyids)))
        }, ())))
    }

}


pub mod Language_C_Parser_Tokens {
    use haskell_support::*;
    pub enum CToken {
        CTokLParen<PosLength>,
        CTokRParen<PosLength>,
        CTokLBracket<PosLength>,
        CTokRBracket<PosLength>,
        CTokArrow<PosLength>,
        CTokDot<PosLength>,
        CTokExclam<PosLength>,
        CTokTilde<PosLength>,
        CTokInc<PosLength>,
        CTokDec<PosLength>,
        CTokPlus<PosLength>,
        CTokMinus<PosLength>,
        CTokStar<PosLength>,
        CTokSlash<PosLength>,
        CTokPercent<PosLength>,
        CTokAmper<PosLength>,
        CTokShiftL<PosLength>,
        CTokShiftR<PosLength>,
        CTokLess<PosLength>,
        CTokLessEq<PosLength>,
        CTokHigh<PosLength>,
        CTokHighEq<PosLength>,
        CTokEqual<PosLength>,
        CTokUnequal<PosLength>,
        CTokHat<PosLength>,
        CTokBar<PosLength>,
        CTokAnd<PosLength>,
        CTokOr<PosLength>,
        CTokQuest<PosLength>,
        CTokColon<PosLength>,
        CTokAssign<PosLength>,
        CTokPlusAss<PosLength>,
        CTokMinusAss<PosLength>,
        CTokStarAss<PosLength>,
        CTokSlashAss<PosLength>,
        CTokPercAss<PosLength>,
        CTokAmpAss<PosLength>,
        CTokHatAss<PosLength>,
        CTokBarAss<PosLength>,
        CTokSLAss<PosLength>,
        CTokSRAss<PosLength>,
        CTokComma<PosLength>,
        CTokSemic<PosLength>,
        CTokLBrace<PosLength>,
        CTokRBrace<PosLength>,
        CTokEllipsis<PosLength>,
        CTokAlignof<PosLength>,
        CTokAsm<PosLength>,
        CTokAuto<PosLength>,
        CTokBreak<PosLength>,
        CTokBool<PosLength>,
        CTokCase<PosLength>,
        CTokChar<PosLength>,
        CTokConst<PosLength>,
        CTokContinue<PosLength>,
        CTokComplex<PosLength>,
        CTokDefault<PosLength>,
        CTokDo<PosLength>,
        CTokDouble<PosLength>,
        CTokElse<PosLength>,
        CTokEnum<PosLength>,
        CTokExtern<PosLength>,
        CTokFloat<PosLength>,
        CTokFor<PosLength>,
        CTokGoto<PosLength>,
        CTokIf<PosLength>,
        CTokInline<PosLength>,
        CTokInt<PosLength>,
        CTokLong<PosLength>,
        CTokLabel<PosLength>,
        CTokRegister<PosLength>,
        CTokRestrict<PosLength>,
        CTokReturn<PosLength>,
        CTokShort<PosLength>,
        CTokSigned<PosLength>,
        CTokSizeof<PosLength>,
        CTokStatic<PosLength>,
        CTokStruct<PosLength>,
        CTokSwitch<PosLength>,
        CTokTypedef<PosLength>,
        CTokTypeof<PosLength>,
        CTokThread<PosLength>,
        CTokUnion<PosLength>,
        CTokUnsigned<PosLength>,
        CTokVoid<PosLength>,
        CTokVolatile<PosLength>,
        CTokWhile<PosLength>,
        CTokCLit<PosLength, CChar>,
        CTokILit<PosLength, CInteger>,
        CTokFLit<PosLength, CFloat>,
        CTokSLit<PosLength, CString>,
        CTokIdent<PosLength, Ident>,
        CTokTyIdent<PosLength, Ident>,
        CTokGnuC<GnuCTok, PosLength>,
        CTokEof
    }
    pub use self::CToken::*;

    pub enum GnuCTok {
        GnuCAttrTok,
        GnuCExtTok,
        GnuCVaArg,
        GnuCOffsetof,
        GnuCTyCompat,
        GnuCComplexReal,
        GnuCComplexImag
    }
    pub use self::GnuCTok::*;

    pub fn posLenOfTok(__0: CToken) -> (Position, isize) {
        match (__0) {
            CTokLParen(pos) => {
                pos
            },
            CTokRParen(pos) => {
                pos
            },
            CTokLBracket(pos) => {
                pos
            },
            CTokRBracket(pos) => {
                pos
            },
            CTokArrow(pos) => {
                pos
            },
            CTokDot(pos) => {
                pos
            },
            CTokExclam(pos) => {
                pos
            },
            CTokTilde(pos) => {
                pos
            },
            CTokInc(pos) => {
                pos
            },
            CTokDec(pos) => {
                pos
            },
            CTokPlus(pos) => {
                pos
            },
            CTokMinus(pos) => {
                pos
            },
            CTokStar(pos) => {
                pos
            },
            CTokSlash(pos) => {
                pos
            },
            CTokPercent(pos) => {
                pos
            },
            CTokAmper(pos) => {
                pos
            },
            CTokShiftL(pos) => {
                pos
            },
            CTokShiftR(pos) => {
                pos
            },
            CTokLess(pos) => {
                pos
            },
            CTokLessEq(pos) => {
                pos
            },
            CTokHigh(pos) => {
                pos
            },
            CTokHighEq(pos) => {
                pos
            },
            CTokEqual(pos) => {
                pos
            },
            CTokUnequal(pos) => {
                pos
            },
            CTokHat(pos) => {
                pos
            },
            CTokBar(pos) => {
                pos
            },
            CTokAnd(pos) => {
                pos
            },
            CTokOr(pos) => {
                pos
            },
            CTokQuest(pos) => {
                pos
            },
            CTokColon(pos) => {
                pos
            },
            CTokAssign(pos) => {
                pos
            },
            CTokPlusAss(pos) => {
                pos
            },
            CTokMinusAss(pos) => {
                pos
            },
            CTokStarAss(pos) => {
                pos
            },
            CTokSlashAss(pos) => {
                pos
            },
            CTokPercAss(pos) => {
                pos
            },
            CTokAmpAss(pos) => {
                pos
            },
            CTokHatAss(pos) => {
                pos
            },
            CTokBarAss(pos) => {
                pos
            },
            CTokSLAss(pos) => {
                pos
            },
            CTokSRAss(pos) => {
                pos
            },
            CTokComma(pos) => {
                pos
            },
            CTokSemic(pos) => {
                pos
            },
            CTokLBrace(pos) => {
                pos
            },
            CTokRBrace(pos) => {
                pos
            },
            CTokEllipsis(pos) => {
                pos
            },
            CTokAlignof(pos) => {
                pos
            },
            CTokAsm(pos) => {
                pos
            },
            CTokAuto(pos) => {
                pos
            },
            CTokBreak(pos) => {
                pos
            },
            CTokBool(pos) => {
                pos
            },
            CTokCase(pos) => {
                pos
            },
            CTokChar(pos) => {
                pos
            },
            CTokConst(pos) => {
                pos
            },
            CTokContinue(pos) => {
                pos
            },
            CTokComplex(pos) => {
                pos
            },
            CTokDefault(pos) => {
                pos
            },
            CTokDo(pos) => {
                pos
            },
            CTokDouble(pos) => {
                pos
            },
            CTokElse(pos) => {
                pos
            },
            CTokEnum(pos) => {
                pos
            },
            CTokExtern(pos) => {
                pos
            },
            CTokFloat(pos) => {
                pos
            },
            CTokFor(pos) => {
                pos
            },
            CTokGoto(pos) => {
                pos
            },
            CTokInt(pos) => {
                pos
            },
            CTokInline(pos) => {
                pos
            },
            CTokIf(pos) => {
                pos
            },
            CTokLong(pos) => {
                pos
            },
            CTokLabel(pos) => {
                pos
            },
            CTokRegister(pos) => {
                pos
            },
            CTokRestrict(pos) => {
                pos
            },
            CTokReturn(pos) => {
                pos
            },
            CTokShort(pos) => {
                pos
            },
            CTokSigned(pos) => {
                pos
            },
            CTokSizeof(pos) => {
                pos
            },
            CTokStatic(pos) => {
                pos
            },
            CTokStruct(pos) => {
                pos
            },
            CTokSwitch(pos) => {
                pos
            },
            CTokTypedef(pos) => {
                pos
            },
            CTokTypeof(pos) => {
                pos
            },
            CTokThread(pos) => {
                pos
            },
            CTokUnion(pos) => {
                pos
            },
            CTokUnsigned(pos) => {
                pos
            },
            CTokVoid(pos) => {
                pos
            },
            CTokVolatile(pos) => {
                pos
            },
            CTokWhile(pos) => {
                pos
            },
            CTokCLit(pos, _) => {
                pos
            },
            CTokILit(pos, _) => {
                pos
            },
            CTokFLit(pos, _) => {
                pos
            },
            CTokSLit(pos, _) => {
                pos
            },
            CTokIdent(pos, _) => {
                pos
            },
            CTokTyIdent(pos, _) => {
                pos
            },
            CTokGnuC(_, pos) => {
                pos
            },
            CTokEof => {
                __error!("tokenPos: Eof".to_string())
            },
        }
    }

}


pub mod Language_C_Parser {
    use haskell_support::*;
    pub fn execParser_(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}


pub mod Language_C_Pretty {
    use haskell_support::*;
    pub fn attrlistP(__0: Vec<CAttr>) -> Doc {
        match (__0) {
            [] => {
                empty
            },
            attrs => {
                <>(text("__attribute__".to_string()), parens((parens((hcat(punctuate(comma, map(pretty)(attrs))))))))
            },
        }
    }

    pub fn binPrec(__0: CBinaryOp) -> isize {
        match (__0) {
            CMulOp => {
                20
            },
            CDivOp => {
                20
            },
            CRmdOp => {
                20
            },
            CAddOp => {
                19
            },
            CSubOp => {
                19
            },
            CShlOp => {
                18
            },
            CShrOp => {
                18
            },
            CLeOp => {
                17
            },
            CGrOp => {
                17
            },
            CLeqOp => {
                17
            },
            CGeqOp => {
                17
            },
            CEqOp => {
                16
            },
            CNeqOp => {
                16
            },
            CAndOp => {
                15
            },
            CXorOp => {
                14
            },
            COrOp => {
                13
            },
            CLndOp => {
                12
            },
            CLorOp => {
                11
            },
        }
    }

    pub fn identP() -> Doc {
        text(identToString)
    }

    pub fn ifP(flag: bool, doc: Doc) -> Doc {
        if(flag, then, doc, else, empty)
    }

    pub fn ii() -> Doc {
        nest(4)
    }

    pub fn maybeP() -> Doc {
        maybe(empty)
    }

    pub fn mlistP(pp: fn(Vec<p>) -> Doc, xs: Vec<p>) -> Doc {
        maybeP(pp, (if(null, xs, then, None, else, Some, xs)))
    }

    pub fn parenPrec(prec: isize, prec2: isize, t: Doc) -> Doc {
        <=(if(prec), prec2(then, t, else, parens, t))
    }

    pub fn prettyDeclr(show_attrs: bool, prec: isize, (CDeclr(name, derived_declrs, asmname, cattrs, _)): CDeclr) -> Doc {
        <+>(ppDeclr(prec, (reverse(derived_declrs))), <+>(prettyAsmName(asmname), ifP(show_attrs, (attrlistP(cattrs)))))
    }

    pub fn prettyUsingInclude((CTranslUnit(edecls, _)): CTranslUnit) -> Doc {
        $$(includeWarning(headerFiles), (vcat(map((either(includeHeader, pretty)), mappedDecls))))
    }

}


pub mod Language_C_Syntax_AST {
    use haskell_support::*;
    #[derive(Clone, Debug)]
    struct CTranslationUnit<a>(CTranslUnit<Vec<CExternalDeclaration<a>>, a>);

    #[derive(Clone, Debug)]
    pub enum CExternalDeclaration<a> {
        CDeclExt<CDeclaration<a>>,
        CFDefExt<CFunctionDef<a>>,
        CAsmExt<CStringLiteral<a>, a>
    }
    pub use self::CExternalDeclaration::*;

    #[derive(Clone, Debug)]
    struct CFunctionDef<a>(CFunDef<Vec<CDeclarationSpecifier<a>>, CDeclarator<a>, Vec<CDeclaration<a>>, CStatement<a>, a>);

    #[derive(Clone, Debug)]
    struct CDeclaration<a>(CDecl<Vec<CDeclarationSpecifier<a>>, Vec<(Option<CDeclarator<a>>, Option<CInitializer<a>>, Option<CExpression<a>>)>, a>);

    #[derive(Clone, Debug)]
    struct CDeclarator<a>(CDeclr<Option<Ident>, Vec<CDerivedDeclarator<a>>, Option<CStringLiteral<a>>, Vec<CAttribute<a>>, a>);

    #[derive(Clone, Debug)]
    pub enum CDerivedDeclarator<a> {
        CPtrDeclr<Vec<CTypeQualifier<a>>, a>,
        CArrDeclr<Vec<CTypeQualifier<a>>, CArraySize<a>, a>,
        CFunDeclr<Either<Vec<Ident>, (Vec<CDeclaration<a>>, bool)>, Vec<CAttribute<a>>, a>
    }
    pub use self::CDerivedDeclarator::*;

    #[derive(Clone, Debug)]
    pub enum CArraySize<a> {
        CNoArrSize<bool>,
        CArrSize<bool, CExpression<a>>
    }
    pub use self::CArraySize::*;

    #[derive(Clone, Debug)]
    pub enum CStatement<a> {
        CLabel<Ident, CStatement<a>, Vec<CAttribute<a>>, a>,
        CCase<CExpression<a>, CStatement<a>, a>,
        CCases<CExpression<a>, CExpression<a>, CStatement<a>, a>,
        CDefault<CStatement<a>, a>,
        CExpr<Option<CExpression<a>>, a>,
        CCompound<Vec<Ident>, Vec<CCompoundBlockItem<a>>, a>,
        CIf<CExpression<a>, CStatement<a>, Option<CStatement<a>>, a>,
        CSwitch<CExpression<a>, CStatement<a>, a>,
        CWhile<CExpression<a>, CStatement<a>, bool, a>,
        CFor<Either<Option<CExpression<a>>, CDeclaration<a>>, Option<CExpression<a>>, Option<CExpression<a>>, CStatement<a>, a>,
        CGoto<Ident, a>,
        CGotoPtr<CExpression<a>, a>,
        CCont<a>,
        CBreak<a>,
        CReturn<Option<CExpression<a>>, a>,
        CAsm<CAssemblyStatement<a>, a>
    }
    pub use self::CStatement::*;

    #[derive(Clone, Debug)]
    struct CAssemblyStatement<a>(CAsmStmt<Option<CTypeQualifier<a>>, CStringLiteral<a>, Vec<CAssemblyOperand<a>>, Vec<CAssemblyOperand<a>>, Vec<CStringLiteral<a>>, a>);

    #[derive(Clone, Debug)]
    struct CAssemblyOperand<a>(CAsmOperand<Option<Ident>, CStringLiteral<a>, CExpression<a>, a>);

    #[derive(Clone, Debug)]
    pub enum CCompoundBlockItem<a> {
        CBlockStmt<CStatement<a>>,
        CBlockDecl<CDeclaration<a>>,
        CNestedFunDef<CFunctionDef<a>>
    }
    pub use self::CCompoundBlockItem::*;

    #[derive(Clone, Debug)]
    pub enum CDeclarationSpecifier<a> {
        CStorageSpec<CStorageSpecifier<a>>,
        CTypeSpec<CTypeSpecifier<a>>,
        CTypeQual<CTypeQualifier<a>>
    }
    pub use self::CDeclarationSpecifier::*;

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum CStorageSpecifier<a> {
        CAuto<a>,
        CRegister<a>,
        CStatic<a>,
        CExtern<a>,
        CTypedef<a>,
        CThread<a>
    }
    pub use self::CStorageSpecifier::*;

    #[derive(Clone, Debug)]
    pub enum CTypeSpecifier<a> {
        CVoidType<a>,
        CCharType<a>,
        CShortType<a>,
        CIntType<a>,
        CLongType<a>,
        CFloatType<a>,
        CDoubleType<a>,
        CSignedType<a>,
        CUnsigType<a>,
        CBoolType<a>,
        CComplexType<a>,
        CSUType<CStructureUnion<a>, a>,
        CEnumType<CEnumeration<a>, a>,
        CTypeDef<Ident, a>,
        CTypeOfExpr<CExpression<a>, a>,
        CTypeOfType<CDeclaration<a>, a>
    }
    pub use self::CTypeSpecifier::*;

    #[derive(Clone, Debug)]
    pub enum CTypeQualifier<a> {
        CConstQual<a>,
        CVolatQual<a>,
        CRestrQual<a>,
        CInlineQual<a>,
        CAttrQual<CAttribute<a>>
    }
    pub use self::CTypeQualifier::*;

    #[derive(Clone, Debug)]
    struct CStructureUnion<a>(CStruct<CStructTag, Option<Ident>, Option<Vec<CDeclaration<a>>>, Vec<CAttribute<a>>, a>);

    #[derive(Clone, Debug, Eq)]
    pub enum CStructTag {
        CStructTag,
        CUnionTag
    }
    pub use self::CStructTag::*;

    #[derive(Clone, Debug)]
    struct CEnumeration<a>(CEnum<Option<Ident>, Option<Vec<(Ident, Option<CExpression<a>>)>>, Vec<CAttribute<a>>, a>);

    #[derive(Clone, Debug)]
    pub enum CInitializer<a> {
        CInitExpr<CExpression<a>, a>,
        CInitList<CInitializerList<a>, a>
    }
    pub use self::CInitializer::*;

    #[derive(Clone, Debug)]
    pub enum CPartDesignator<a> {
        CArrDesig<CExpression<a>, a>,
        CMemberDesig<Ident, a>,
        CRangeDesig<CExpression<a>, CExpression<a>, a>
    }
    pub use self::CPartDesignator::*;

    #[derive(Clone, Debug)]
    struct CAttribute<a>(CAttr<Ident, Vec<CExpression<a>>, a>);

    #[derive(Clone, Debug)]
    pub enum CExpression<a> {
        CComma<Vec<CExpression<a>>, a>,
        CAssign<CAssignOp, CExpression<a>, CExpression<a>, a>,
        CCond<CExpression<a>, Option<CExpression<a>>, CExpression<a>, a>,
        CBinary<CBinaryOp, CExpression<a>, CExpression<a>, a>,
        CCast<CDeclaration<a>, CExpression<a>, a>,
        CUnary<CUnaryOp, CExpression<a>, a>,
        CSizeofExpr<CExpression<a>, a>,
        CSizeofType<CDeclaration<a>, a>,
        CAlignofExpr<CExpression<a>, a>,
        CAlignofType<CDeclaration<a>, a>,
        CComplexReal<CExpression<a>, a>,
        CComplexImag<CExpression<a>, a>,
        CIndex<CExpression<a>, CExpression<a>, a>,
        CCall<CExpression<a>, Vec<CExpression<a>>, a>,
        CMember<CExpression<a>, Ident, bool, a>,
        CVar<Ident, a>,
        CConst<CConstant<a>>,
        CCompoundLit<CDeclaration<a>, CInitializerList<a>, a>,
        CStatExpr<CStatement<a>, a>,
        CLabAddrExpr<Ident, a>,
        CBuiltinExpr<CBuiltinThing<a>>
    }
    pub use self::CExpression::*;

    #[derive(Clone, Debug)]
    pub enum CBuiltinThing<a> {
        CBuiltinVaArg<CExpression<a>, CDeclaration<a>, a>,
        CBuiltinOffsetOf<CDeclaration<a>, Vec<CPartDesignator<a>>, a>,
        CBuiltinTypesCompatible<CDeclaration<a>, CDeclaration<a>, a>
    }
    pub use self::CBuiltinThing::*;

    #[derive(Clone, Debug)]
    pub enum CConstant<a> {
        CIntConst<CInteger, a>,
        CCharConst<CChar, a>,
        CFloatConst<CFloat, a>,
        CStrConst<CString, a>
    }
    pub use self::CConstant::*;

    #[derive(Clone, Debug)]
    struct CStringLiteral<a>(CStrLit<CString, a>);

    pub fn cstringOfLit((CStrLit(cstr, _)): CStringLiteral<a>) -> CString {
        cstr
    }

    pub fn fmapInitList(_f: fn(a) -> b) -> CInitializerList<b> {
        map((|(desigs, initializer)| { (fmap((fmap(_f)), desigs), fmap(_f, initializer)) }))
    }

    pub fn isSUEDef(__0: CTypeSpecifier<a>) -> bool {
        match (__0) {
            CSUType(CStruct(_, _, Some(_), _, _), _) => {
                true
            },
            CEnumType(CEnum(_, Some(_), _, _), _) => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn liftStrLit((CStrLit(__str, at)): CStringLiteral<a>) -> CConstant<a> {
        CStrConst(__str, at)
    }

    pub fn partitionDeclSpecs() -> (Vec<CStorageSpecifier<a>>, Vec<CAttribute<a>>, Vec<CTypeQualifier<a>>, Vec<CTypeSpecifier<a>>, bool) {
        foldr(deals, (vec![], vec![], vec![], vec![], false))
    }

}


pub mod Language_C_Syntax_Constants {
    use haskell_support::*;
    #[derive(Clone, Debug, Eq, Ord)]
    pub enum CChar {
        CChar<Char, bool>,
        CChars<Vec<Char>, bool>
    }
    pub use self::CChar::*;

    #[derive(Bounded, Clone, Debug, Enum, Eq, Ord)]
    pub enum CIntRepr {
        DecRepr,
        HexRepr,
        OctalRepr
    }
    pub use self::CIntRepr::*;

    #[derive(Bounded, Clone, Debug, Enum, Eq, Ord)]
    pub enum CIntFlag {
        FlagUnsigned,
        FlagLong,
        FlagLongLong,
        FlagImag
    }
    pub use self::CIntFlag::*;

    #[derive(Clone, Debug, Eq, Ord)]
    struct CInteger(CInteger<Integer, CIntRepr, Flags<CIntFlag>>);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CFloat(CFloat<String>);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CString(CString<Vec<Char>, bool>);

    pub fn _showWideFlag(flag: bool) -> ShowS {
        if(flag, then, showString, "L".to_string(), else, id)
    }

    pub fn cChar(c: Char) -> CChar {
        CChar(c, false)
    }

    pub fn cChar_w(c: Char) -> CChar {
        CChar(c, true)
    }

    pub fn cChars() -> CChar {
        CChars
    }

    pub fn cFloat() -> CFloat {
        CFloat(show)
    }

    pub fn cInteger(i: Integer) -> CInteger {
        CInteger(i, DecRepr, noFlags)
    }

    pub fn cString(__str: String) -> CString {
        CString(__str, false)
    }

    pub fn cString_w(__str: String) -> CString {
        CString(__str, true)
    }

    pub fn clearFlag(flag: f, (Flags(k)): Flags<f>) -> Flags<f> {
        Flags(clearBit(k, fromEnum(flag)))
    }

    pub fn concatCStrings(cs: Vec<CString>) -> CString {
        CString((concatMap(getCString, cs)), (any(isWideString, cs)))
    }

    pub fn dQuote(s: String, t: ShowS) -> ShowS {
        __op_addadd((__op_concat('\"', s)), __op_addadd("\"".to_string(), t))
    }

    pub fn escapeCChar('\'': Char) -> String {
        "\\\'".to_string()
    }

    pub fn escapeChar(__0: Char) -> String {
        match (__0) {
            '\\' => {
                "\\\\".to_string()
            },
            '\u{7}' => {
                "\\a".to_string()
            },
            '\u{8}' => {
                "\\b".to_string()
            },
            '\u{1b}' => {
                "\\e".to_string()
            },
            '\u{c}' => {
                "\\f".to_string()
            },
            '\n' => {
                "\\n".to_string()
            },
            '\r' => {
                "\\r".to_string()
            },
            '\t' => {
                "\\t".to_string()
            },
            '\u{b}' => {
                "\\v".to_string()
            },
        }
    }

    pub fn getCChar(__0: CChar) -> Vec<Char> {
        match (__0) {
            CChar(c, _) => {
                vec![c]
            },
            CChars(cs, _) => {
                cs
            },
        }
    }

    pub fn getCCharAsInt(__0: CChar) -> Integer {
        match (__0) {
            CChar(c, _) => {
                fromIntegral((fromEnum(c)))
            },
            CChars(_cs, _) => {
                __error!("integer value of multi-character character constants is implementation defined".to_string())
            },
        }
    }

    pub fn getCInteger((CInteger(i, _, _)): CInteger) -> Integer {
        i
    }

    pub fn getCString((CString(__str, _)): CString) -> String {
        __str
    }

    pub fn head_q(__0: String, __1: Vec<a>) -> a {
        match (__0, __1) {
            (err, []) => {
                __error!(err)
            },
            (_, [x, ..._]) => {
                x
            },
        }
    }

    pub fn isAsciiSourceChar(c: Char) -> bool {
        (isAscii(c) && isPrint(c))
    }

    pub fn isCChar(__0: Char) -> bool {
        match (__0) {
            '\\' => {
                false
            },
            '\'' => {
                false
            },
            '\n' => {
                false
            },
            c => {
                isAsciiSourceChar(c)
            },
        }
    }

    pub fn isSChar(__0: Char) -> bool {
        match (__0) {
            '\\' => {
                false
            },
            '\"' => {
                false
            },
            '\n' => {
                false
            },
            c => {
                isAsciiSourceChar(c)
            },
        }
    }

    pub fn isWideChar(__0: CChar) -> bool {
        match (__0) {
            CChar(_, wideFlag) => {
                wideFlag
            },
            CChars(_, wideFlag) => {
                wideFlag
            },
        }
    }

    pub fn isWideString((CString(_, wideflag)): CString) -> bool {
        wideflag
    }

    pub fn noFlags() -> Flags<f> {
        Flags(0)
    }

    pub fn readCFloat() -> CFloat {
        CFloat
    }

    pub fn readCInteger(repr: CIntRepr, __str: String) -> Either<String, CInteger> {
        match readNum(__str) {
            [(n, suffix)] => {
                mkCInt(n, suffix)
            },
            parseFailed => {
                Left(__op_addadd("Bad Integer literal: ".to_string(), show(parseFailed)))
            },
        }
    }

    pub fn sQuote(s: String, t: ShowS) -> ShowS {
        __op_addadd("\'".to_string(), __op_addadd(s, __op_addadd("\'".to_string(), t)))
    }

    pub fn setFlag(flag: f, (Flags(k)): Flags<f>) -> Flags<f> {
        Flags(setBit(k, fromEnum(flag)))
    }

    pub fn showCharConst(c: Char) -> ShowS {
        sQuote(escapeCChar(c))
    }

    pub fn showStringLit() -> ShowS {
        dQuote(concatMap(showStringChar))
    }

    pub fn testFlag(flag: f, (Flags(k)): Flags<f>) -> bool {
        testBit(k, fromEnum(flag))
    }

    pub fn unescapeChar(__0: String) -> (Char, String) {
        match (__0) {
            ['\\', ...[c, ...cs]] => {
                match c {
                    'n' => {
                        ('\n', cs)
                    },
                    't' => {
                        ('\t', cs)
                    },
                    'v' => {
                        ('\u{b}', cs)
                    },
                    'b' => {
                        ('\u{8}', cs)
                    },
                    'r' => {
                        ('\r', cs)
                    },
                    'f' => {
                        ('\u{c}', cs)
                    },
                    'a' => {
                        ('\u{7}', cs)
                    },
                    'e' => {
                        ('\u{1b}', cs)
                    },
                    'E' => {
                        ('\u{1b}', cs)
                    },
                    '\\' => {
                        ('\\', cs)
                    },
                    '?' => {
                        ('?', cs)
                    },
                    '\'' => {
                        ('\'', cs)
                    },
                    '\"' => {
                        ('\"', cs)
                    },
                    'x' => {
                        match head_q("bad escape sequence".to_string(), (readHex(cs))) {
                            (i, cs_q) => {
                                (toEnum(i), cs_q)
                            },
                        }
                    },
                    _ => {
                        match head_q("bad escape sequence".to_string(), (readOct((__op_concat(c, cs))))) {
                            (i, cs_q) => {
                                (toEnum(i), cs_q)
                            },
                        }
                    },
                }
            },
            [c, ...cs] => {
                (c, cs)
            },
            [] => {
                __error!("unescape char: empty string".to_string())
            },
        }
    }

    pub fn unescapeString(__0: String) -> String {
        match (__0) {
            [] => {
                vec![]
            },
            cs => {
                match unescapeChar(cs) {
                    (c, cs_q) => {
                        __op_concat(c, unescapeString(cs_q))
                    },
                }
            },
        }
    }

}


pub mod Language_C_Syntax_Ops {
    use haskell_support::*;
    #[derive(Clone, Debug, Eq, Ord)]
    pub enum CAssignOp {
        CAssignOp,
        CMulAssOp,
        CDivAssOp,
        CRmdAssOp,
        CAddAssOp,
        CSubAssOp,
        CShlAssOp,
        CShrAssOp,
        CAndAssOp,
        CXorAssOp,
        COrAssOp
    }
    pub use self::CAssignOp::*;

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum CBinaryOp {
        CMulOp,
        CDivOp,
        CRmdOp,
        CAddOp,
        CSubOp,
        CShlOp,
        CShrOp,
        CLeOp,
        CGrOp,
        CLeqOp,
        CGeqOp,
        CEqOp,
        CNeqOp,
        CAndOp,
        CXorOp,
        COrOp,
        CLndOp,
        CLorOp
    }
    pub use self::CBinaryOp::*;

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum CUnaryOp {
        CPreIncOp,
        CPreDecOp,
        CPostIncOp,
        CPostDecOp,
        CAdrOp,
        CIndOp,
        CPlusOp,
        CMinOp,
        CCompOp,
        CNegOp
    }
    pub use self::CUnaryOp::*;

    pub fn assignBinop(__0: CAssignOp) -> CBinaryOp {
        match (__0) {
            CAssignOp => {
                __error!("direct assignment has no binary operator".to_string())
            },
            CMulAssOp => {
                CMulOp
            },
            CDivAssOp => {
                CDivOp
            },
            CRmdAssOp => {
                CRmdOp
            },
            CAddAssOp => {
                CAddOp
            },
            CSubAssOp => {
                CSubOp
            },
            CShlAssOp => {
                CShlOp
            },
            CShrAssOp => {
                CShrOp
            },
            CAndAssOp => {
                CAndOp
            },
            CXorAssOp => {
                CXorOp
            },
            COrAssOp => {
                COrOp
            },
        }
    }

    pub fn isBitOp(op: CBinaryOp) -> bool {
        elem(op, vec![CShlOp, CShrOp, CAndOp, COrOp, CXorOp])
    }

    pub fn isCmpOp(op: CBinaryOp) -> bool {
        elem(op, vec![CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp])
    }

    pub fn isEffectfulOp(op: CUnaryOp) -> bool {
        elem(op, vec![CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp])
    }

    pub fn isLogicOp(op: CBinaryOp) -> bool {
        elem(op, vec![CLndOp, CLorOp])
    }

    pub fn isPtrOp(op: CBinaryOp) -> bool {
        elem(op, vec![CAddOp, CSubOp])
    }

}


pub mod Language_C_Syntax_Utils {
    use haskell_support::*;
    pub fn compoundSubStmts(__0: CBlockItem) -> Vec<CStat> {
        match (__0) {
            CBlockStmt(s) => {
                vec![s]
            },
            CBlockDecl(_) => {
                vec![]
            },
            CNestedFunDef(_) => {
                vec![]
            },
        }
    }

    pub fn getLabels(__0: CStat) -> Vec<Ident> {
        match (__0) {
            CLabel(l, s, _, _) => {
                __op_concat(l, getLabels(s))
            },
            CCompound(ls, body, _) => {
                \\(concatMap((concatMap(getLabels, compoundSubStmts)), body), ls)
            },
            stmt => {
                concatMap(getLabels, (getSubStmts(stmt)))
            },
        }
    }

    pub fn getSubStmts(__0: CStat) -> Vec<CStat> {
        match (__0) {
            CLabel(_, s, _, _) => {
                vec![s]
            },
            CCase(_, s, _) => {
                vec![s]
            },
            CCases(_, _, s, _) => {
                vec![s]
            },
            CDefault(s, _) => {
                vec![s]
            },
            CExpr(_, _) => {
                vec![]
            },
            CCompound(_, body, _) => {
                concatMap(compoundSubStmts, body)
            },
            CIf(_, sthen, selse, _) => {
                maybe(vec![sthen], (|s| { vec![sthen, s] }), selse)
            },
            CSwitch(_, s, _) => {
                vec![s]
            },
            CWhile(_, s, _, _) => {
                vec![s]
            },
            CFor(_, _, _, s, _) => {
                vec![s]
            },
            CGoto(_, _) => {
                vec![]
            },
            CGotoPtr(_, _) => {
                vec![]
            },
            CCont(_) => {
                vec![]
            },
            CBreak(_) => {
                vec![]
            },
            CReturn(_, _) => {
                vec![]
            },
            CAsm(_, _) => {
                vec![]
            },
        }
    }

    pub fn mapBlockItemStmts(__0: fn(CStat) -> bool, __1: fn(CStat) -> CStat, __2: CBlockItem) -> CBlockItem {
        match (__0, __1, __2) {
            (stop, f, CBlockStmt(s)) => {
                CBlockStmt((mapSubStmts(stop, f, s)))
            },
            (_, _, bi) => {
                bi
            },
        }
    }

    pub fn mapSubStmts(__0: fn(CStat) -> bool, __1: fn(CStat) -> CStat, __2: CStat) -> CStat {
        match (__0, __1, __2) {
            (stop, f, CLabel(i, s, attrs, ni)) => {
                f((CLabel(i, (mapSubStmts(stop, f, s)), attrs, ni)))
            },
            (stop, f, CCase(e, s, ni)) => {
                f((CCase(e, (mapSubStmts(stop, f, s)), ni)))
            },
            (stop, f, CCases(e1, e2, s, ni)) => {
                f((CCases(e1, e2, (mapSubStmts(stop, f, s)), ni)))
            },
            (stop, f, CDefault(s, ni)) => {
                f((CDefault((mapSubStmts(stop, f, s)), ni)))
            },
            (stop, f, CCompound(ls, body, ni)) => {
                f((CCompound(ls, (map((mapBlockItemStmts(stop, f)), body)), ni)))
            },
            (stop, f, CIf(e, sthen, selse, ni)) => {
                f((CIf(e, (mapSubStmts(stop, f, sthen)), (maybe(None, (Some(mapSubStmts(stop, f))), selse)), ni)))
            },
            (stop, f, CSwitch(e, s, ni)) => {
                f((CSwitch(e, (mapSubStmts(stop, f, s)), ni)))
            },
            (stop, f, CWhile(e, s, isdo, ni)) => {
                f((CWhile(e, (mapSubStmts(stop, f, s)), isdo, ni)))
            },
            (stop, f, CFor(i, t, a, s, ni)) => {
                f((CFor(i, t, a, (mapSubStmts(stop, f, s)), ni)))
            },
            (_, f, s) => {
                f(s)
            },
        }
    }

}


pub mod Language_C_Syntax {
    use haskell_support::*;

}


pub mod Language_C_System_GCC {
    use haskell_support::*;
    pub fn buildCppArgs((CppArgs(options, extra_args, _tmpdir, input_file, output_file_opt)): CppArgs) -> Vec<String> {
        __op_addadd(/* do */ {
            (concatMap(tOption, options))
        }, __op_addadd(outputFileOpt, __op_addadd(vec!["-E".to_string(), input_file], extra_args)))
    }

    pub fn gccParseCPPArgs(args: Vec<String>) -> Either<String, (CppArgs, Vec<String>)> {
        match mungeArgs(((None, None, RList::empty), (RList::empty, RList::empty)), args) {
            Left(err) => {
                Left(err)
            },
            Right(((None, _, _), _)) => {
                Left("No .c / .hc / .h source file given".to_string())
            },
            Right(((Some(input_file), output_file_opt, cpp_opts), (extra_args, other_args))) => {
                Right(((rawCppArgs((RList::reverse(extra_args)), input_file))({
                        outputFile: output_file_opt,
                        cppOptions: RList::reverse(cpp_opts)
                    }), RList::reverse(other_args)))
            },
        }
    }

    pub fn newGCC() -> GCC {
        GCC
    }

}


pub mod Language_C_System_Preprocess {
    use haskell_support::*;
    pub enum CppOption {
        IncludeDir<FilePath>,
        Define<String, String>,
        Undefine<String>,
        IncludeFile<FilePath>
    }
    pub use self::CppOption::*;

    struct CppArgs(CppArgs<{ /* struct def */ }>);

    pub fn addCppOption(cpp_args: CppArgs, opt: CppOption) -> CppArgs {
        cpp_args({
            cppOptions: __op_concat(opt, (cppOptions(cpp_args)))
        })
    }

    pub fn addExtraOption(cpp_args: CppArgs, extra: String) -> CppArgs {
        cpp_args({
            extraOptions: __op_concat(extra, (extraOptions(cpp_args)))
        })
    }

    pub fn cppFile(input_file: FilePath) -> CppArgs {
        CppArgs({
            cppOptions: vec![],
            extraOptions: vec![],
            cppTmpDir: None,
            inputFile: input_file,
            outputFile: None
        })
    }

    pub fn isPreprocessed() -> bool {
        (".i".to_string()(isSuffixOf))
    }

    pub fn mkOutputFile(tmp_dir_opt: Option<FilePath>, input_file: FilePath) -> IO<FilePath> {
        /* do */ {
            let tmpDir = getTempDir(tmp_dir_opt);
            mkTmpFile(tmpDir, (getOutputFileName(input_file)))
        }
    }

    pub fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> IO<FilePath> {
        /* do */ {
            let (path, file_handle) = openTempFile(tmp_dir, file_templ);
            hClose(file_handle);
            path
        }
    }

    pub fn preprocessedExt() -> String {
        ".i".to_string()
    }

    pub fn rawCppArgs(opts: Vec<String>, input_file: FilePath) -> CppArgs {
        CppArgs({
            inputFile: input_file,
            cppOptions: vec![],
            extraOptions: opts,
            outputFile: None,
            cppTmpDir: None
        })
    }

    pub fn runPreprocessor(cpp: cpp, cpp_args: CppArgs) -> IO<Either<ExitCode, InputStream>> {
        /* do */ {
            bracket(getActualOutFile, removeTmpOutFile, invokeCpp);

        }
    }

}




