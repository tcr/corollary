mod Language_C_Analysis_AstAnalysis {
    enum StmtCtx {
        FunCtx(VarDecl),
        LoopCtx,
        SwitchCtx
    }

    #[derive(Debug, Eq)]
    enum ExprSide {
        LValue,
        RValue
    }

    fn advanceDesigList(ds: Vec<CDesignator>) -> Vec<CDesignator> {
        drop(1)(dropWhile(((not . matchDesignator(d))), ds))
    }

    fn analyseAST((CTranslUnit(decls, _file_node)): m) -> m {
        /* do */ {
            mapRecoverM_(analyseExt, decls);
            __op_bind(getDefTable, Lambda((not((inFileScope(dt)))))(error("Internal Error: Not in filescope after analysis".to_string())));
            liftM(globalDefs, getDefTable);

        }
    }

    fn analyseExt(__0: m) -> m {
        match (__0) {
        CAsmExt(asm, _) => {
            handleAsmBlock(asm)
        },
        CFDefExt(fundef) => {
            analyseFunDef(fundef)
        },
        CDeclExt(decl) => {
            analyseDecl(False, decl)
        },
    }
    }

    fn analyseFunDef((CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info)): m) -> m {
        /* do */ {
            let var_decl_info = analyseVarDecl_q(True, declspecs, declr, oldstyle_decls, Nothing);
            {
                    let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = || {
                        var_decl_info
                    };
            };
            when((isNoName(name)))(astError(node_info, "NoName in analyseFunDef".to_string()));
            {
                    let ident = || {
                        identOfVarName(name)
                    };
            };
            let ty_q = improveFunDefType(ty);
            let fun_storage = computeFunDefStorage(ident, storage_spec);
            {
                    let var_decl = || {
                        VarDecl(name, (DeclAttrs(is_inline, fun_storage, attrs)), ty_q)
                    };
            };
            handleVarDecl(False, (Decl(var_decl, node_info)));
            let stmt_q = analyseFunctionBody(node_info, var_decl, stmt);
            handleFunDef(ident, (FunDef(var_decl, stmt_q, node_info)));

        }
    }

    fn analyseFunctionBody(__0: m) -> m {
        match (__0, __1, __2, __3, __4) {
        node_info(decl, s, <todo>, CCompound(localLabels, items, _)) => {
            /* do */ {
                enterFunctionScope;
                mapM_(((withDefTable . defineLabel)), (__op_addadd(localLabels, getLabels(s))));
                defineParams(node_info, decl);
                mapM_((tBlockItem(vec![FunCtx(decl)])), items);
                leaveFunctionScope;
                return(s)
            }
        },
        _(_, s) => {
            astError((nodeInfo(s)), "Function body is no compound statement".to_string())
        },
    }
    }

    fn analyseTypeDef(handle_sue_def: m) -> m {
        /* do */ {
            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl_q(handle_sue_def, declspecs, declr, vec![], Nothing);
            checkValidTypeDef(is_inline, storage_spec, attrs);
            when((isNoName(name)))(astError(node_info, "NoName in analyseTypeDef".to_string()));
            {
                    let ident = || {
                        identOfVarName(name)
                    };
            };
            handleTypeDef((TypeDef(ident, ty, attrs, node_info)));

        }
    }

    fn builtinType(__0: MonadTrav) -> MonadTrav {
        match (__0) {
        CBuiltinVaArg(_, d, _) => {
            analyseTypeDecl(d)
        },
        CBuiltinOffsetOf(_, _, _) => {
            return(size_tType)
        },
        CBuiltinTypesCompatible(_, _, _) => {
            return(boolType)
        },
    }
    }

    fn checkGuard(c: MonadTrav) -> MonadTrav {
        __op_bind(tExpr(c, RValue, e), checkScalar_q((nodeInfo(e))))
    }

    fn checkInits(__0: MonadTrav) -> MonadTrav {
        match (__0, __1, __2) {
        _(_, []) => {
            return(())
        },
        t(dds, ds(i)(<todo>, is)) => {
            /* do */ {
                let (dds_q, ds_q) = match (dds, ds) {
                        ([], []) => {
                            typeError((nodeInfo(i)), "excess elements in initializer".to_string())
                        },
                        (dd_q(:, rest), []) => {
                            return((rest, vec![dd_q]))
                        },
                        (_, d(:, _)) => {
                            return((advanceDesigList(dds, d), ds))
                        },
                    };
                let t_q = tDesignator(t, ds_q);
                tInit(t_q, i);
                checkInits(t, dds_q, is)
            }
        },
    }
    }

    fn complexBaseType(ni: MonadTrav) -> MonadTrav {
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

    fn computeFunDefStorage(__0: m) -> m {
        match (__0, __1) {
        _(StaticSpec(b)) => {
            return(FunLinkage(InternalLinkage))
        },
        ident(other_spec) => {
            /* do */ {
                let obj_opt = lookupObject(ident);
                {
                        let defaultSpec = || {
                            FunLinkage(ExternalLinkage)
                        };
                };
                match other_spec {
                    NoStorageSpec => {
                        return(maybe(defaultSpec, declStorage, obj_opt))
                    },
                    ExternSpec(False) => {
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

    fn defaultMD() -> MachineDesc {
        MachineDesc({
        iSize: Lambda,
        fSize: Lambda,
        builtinSize: Lambda,
        ptrSize: 4,
        voidSize: 1,
        iAlign: Lambda,
        fAlign: Lambda,
        builtinAlign: Lambda,
        ptrAlign: 4,
        voidAlign: 1
        })
    }

    fn defineParams(ni: MonadTrav) -> MonadTrav {
        match (getParams(declType(decl))) {
            Nothing => {
                astError(ni, "expecting complete function type in function definition".to_string())
            },
            Just(params) => {
                mapM_(handleParamDecl, params)
            },
        }
    }

    fn enclosingFunctionType(__0: Maybe) -> Maybe {
        match (__0) {
        [] => {
            Nothing
        },
        FunCtx(vd, <todo>, _) => {
            Just(declType(vd))
        },
        _(<todo>, cs) => {
            enclosingFunctionType(cs)
        },
    }
    }

    fn extFunProto((VarDeclInfo(var_name, is_inline, storage_spec, attrs, ty, node_info)): m) -> m {
        /* do */ {
            when((isNoName(var_name)))(astError(node_info, "NoName in extFunProto".to_string()));
            let old_fun = lookupObject((identOfVarName(var_name)));
            checkValidSpecs;
            {
                    let decl = || {
                        VarDecl(var_name, (DeclAttrs(is_inline, (funDeclLinkage(old_fun)), attrs)), ty)
                    };
            };
            handleVarDecl(False, (Decl(decl, node_info)));
            enterPrototypeScope;
            maybe((return(())), (mapM_(handleParamDecl)), (getParams(ty)));
            leavePrototypeScope
        }
    }

    fn extVarDecl((VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info)): m) -> m {
        /* do */ {
            when((isNoName(var_name)))(astError(node_info, "NoName in extVarDecl".to_string()));
            let (storage, is_def) = globalStorage(storage_spec);
            {
                    let vardecl = || {
                        VarDecl(var_name, (DeclAttrs(is_inline, storage, attrs)), typ)
                    };
            };
            if(is_def, then, handleObjectDef, False, ident)(ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, False)(Decl(vardecl, node_info)))
        }
    }

    fn getParams(__0: Maybe) -> Maybe {
        match (__0) {
        FunctionType(FunType(_, params, _), _) => {
            Just(params)
        },
        _ => {
            Nothing
        },
    }
    }

    fn hasTypeDef(declspecs: Maybe) -> Maybe {
        match foldr(hasTypeDefSpec, (False, vec![]), declspecs) {
            (True, specs_q) => {
                Just(specs_q)
            },
            (False, _) => {
                Nothing
            },
        }
    }

    fn inLoop(c: Bool) -> Bool {
        any(isLoop, c)
    }

    fn inSwitch(c: Bool) -> Bool {
        any(isSwitch, c)
    }

    fn localVarDecl((VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info)): m) -> m {
        /* do */ {
            when((isNoName(var_name)))(astError(node_info, "NoName in localVarDecl".to_string()));
            let (storage, is_def) = localStorage(storage_spec);
            {
                    let vardecl = || {
                        VarDecl(var_name, (DeclAttrs(is_inline, storage, attrs)), typ)
                    };
            };
            if(is_def, then, handleObjectDef, True, ident, (ObjDef(vardecl, init_opt, node_info)), else, handleVarDecl, True, (Decl(vardecl, node_info)))
        }
    }

    fn matchDesignator(__0: Bool) -> Bool {
        match (__0, __1) {
        CMemberDesig(m1, _)(CMemberDesig(m2, _)) => {
            (m1 == m2)
        },
        _(_) => {
            True
        },
    }
    }

    fn tBlockItem(__0: MonadTrav) -> MonadTrav {
        match (__0, __1) {
        c(CBlockStmt(s)) => {
            tStmt(c, s)
        },
        _(CBlockDecl(d)) => {
            >>(analyseDecl(True, d), return(voidType))
        },
        _(CNestedFunDef(fd)) => {
            >>(analyseFunDef(fd), return(voidType))
        },
    }
    }

    fn tDesignator(__0: MonadTrav) -> MonadTrav {
        match (__0, __1) {
        ArrayType(bt, _, _, _)(CArrDesig(e, ni, <todo>, ds)) => {
            /* do */ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        ArrayType(bt, _, _, _)(CRangeDesig(e1, e2, ni, <todo>, ds)) => {
            /* do */ {
                __op_bind(tExpr(vec![], RValue, e1), checkIntegral_q(ni));
                __op_bind(tExpr(vec![], RValue, e2), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        ArrayType(_, _, _, _)(d(<todo>, ds)) => {
            typeError((nodeInfo(d)), "member designator in array initializer".to_string())
        },
        t(<todo>, DirectType(TyComp(_), _, _), CMemberDesig(m, ni, <todo>, ds)) => {
            /* do */ {
                let mt = fieldType(ni, m, t);
                tDesignator((canonicalType(mt)), ds)
            }
        },
        t(<todo>, DirectType(TyComp(_), _, _), d(<todo>, _)) => {
            typeError((nodeInfo(d)), "array designator in compound initializer".to_string())
        },
        t([]) => {
            return(t)
        },
    }
    }

    fn tExpr(c: MonadTrav) -> MonadTrav {
        match nameOfNode((nodeInfo(e))) {
            Just(n) => {
                /* do */ {
                    let dt = getDefTable;
                    match lookupType(dt, n) {
                        Just(t) => {
                            return(t)
                        },
                        Nothing => {
                            /* do */ {
                                let t = tExpr_q(c, side, e);
                                withDefTable((Lambda))
                            }
                        },
                    }
                }
            },
            Nothing => {
                tExpr_q(c, side, e)
            },
        }
    }

    fn tExpr_q(__0: MonadTrav) -> MonadTrav {
        match (__0, __1, __2) {
        c(side, CBinary(op, le, re, ni)) => {
            /* do */ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);
                let rt = tExpr(c, RValue, re);
                binopType_q(ni, op, lt, rt)
            }
        },
        c(side, CUnary(CAdrOp, e, ni)) => {
            /* do */ {
                when(((side == LValue)))(typeError(ni, "address-of operator as lvalue".to_string()));
                match e {
                    CCompoundLit(_, _, _) => {
                        liftM(simplePtr, tExpr(c, RValue, e))
                    },
                    CVar(i, _) => {
                        __op_bind(lookupObject(i), (typeErrorOnLeft(ni) . maybe((notFound(i)), varAddrType)))
                    },
                    _ => {
                        liftM(simplePtr, tExpr(c, LValue, e))
                    },
                }
            }
        },
        c(_, CUnary(CIndOp, e, ni)) => {
            __op_bind(tExpr(c, RValue, e), ((typeErrorOnLeft(ni) . derefType)))
        },
        c(_, CUnary(CCompOp, e, ni)) => {
            /* do */ {
                let t = tExpr(c, RValue, e);
                checkIntegral_q(ni, t);
                return(t)
            }
        },
        c(side, CUnary(CNegOp, e, ni)) => {
            /* do */ {
                when(((side == LValue)))(typeError(ni, "logical negation used as lvalue".to_string()));
                __op_bind(tExpr(c, RValue, e), checkScalar_q(ni));
                return(boolType)
            }
        },
        c(side, CUnary(op, e, _)) => {
            tExpr(c, (if(isEffectfulOp, op, then, LValue, else, side)), e)
        },
        c(_, CIndex(b, i, ni)) => {
            /* do */ {
                let bt = tExpr(c, RValue, b);
                let it = tExpr(c, RValue, i);
                let addrTy = binopType_q(ni, CAddOp, bt, it);
                typeErrorOnLeft(ni)(derefType(addrTy))
            }
        },
        c(side, CCond(e1, me2, e3, ni)) => {
            /* do */ {
                let t1 = tExpr(c, RValue, e1);
                checkScalar_q((nodeInfo(e1)), t1);
                let t3 = tExpr(c, side, e3);
                match me2 {
                    Just(e2) => {
                        /* do */ {
                            let t2 = tExpr(c, side, e2);
                            conditionalType_q(ni, t2, t3)
                        }
                    },
                    Nothing => {
                        conditionalType_q(ni, t1, t3)
                    },
                }
            }
        },
        c(side, CMember(e, m, deref, ni)) => {
            /* do */ {
                let t = tExpr(c, RValue, e);
                let bt = if(deref, then, typeErrorOnLeft, ni, (derefType(t)), else, return, t);
                fieldType(ni, m, bt)
            }
        },
        c(side, CComma(es, _)) => {
            __op_bind(mapM((tExpr(c, side)), es), (return . last))
        },
        c(side, CCast(d, e, ni)) => {
            /* do */ {
                let dt = analyseTypeDecl(d);
                let et = tExpr(c, side, e);
                typeErrorOnLeft(ni)(castCompatible(dt, et));
                return(dt)
            }
        },
        c(side, CSizeofExpr(e, ni)) => {
            /* do */ {
                when(((side == LValue)))(typeError(ni, "sizeof as lvalue".to_string()));
                tExpr(c, RValue, e);
                return(size_tType)
            }
        },
        c(side, CAlignofExpr(e, ni)) => {
            /* do */ {
                when(((side == LValue)))(typeError(ni, "alignof as lvalue".to_string()));
                tExpr(c, RValue, e);
                return(size_tType)
            }
        },
        c(side, CComplexReal(e, ni)) => {
            complexBaseType(ni, c, side, e)
        },
        c(side, CComplexImag(e, ni)) => {
            complexBaseType(ni, c, side, e)
        },
        _(side, CLabAddrExpr(_, ni)) => {
            /* do */ {
                when(((side == LValue)))(typeError(ni, "label address as lvalue".to_string()));
                return(PtrType(voidType, noTypeQuals, vec![]))
            }
        },
        _(side, CCompoundLit(d, initList, ni)) => {
            /* do */ {
                when(((side == LValue)))(typeError(ni, "compound literal as lvalue".to_string()));
                let lt = analyseTypeDecl(d);
                tInitList(ni, (canonicalType(lt)), initList);
                return(lt)
            }
        },
        _(RValue, CAlignofType(_, _)) => {
            return(size_tType)
        },
        _(RValue, CSizeofType(_, _)) => {
            return(size_tType)
        },
        _(LValue, CAlignofType(_, ni)) => {
            typeError(ni, "alignoftype as lvalue".to_string())
        },
        _(LValue, CSizeofType(_, ni)) => {
            typeError(ni, "sizeoftype as lvalue".to_string())
        },
        _(side, CVar(i, ni)) => {
            __op_bind(lookupObject(i), maybe((typeErrorOnLeft(ni)(notFound(i))), ((return . declType))))
        },
        _(_, CConst(c)) => {
            constType(c)
        },
        _(_, CBuiltinExpr(b)) => {
            builtinType(b)
        },
        c(_, CCall(fe, args, ni)) => {
            /* do */ {
                {
                        let defType = || {
                            FunctionType((FunTypeIncomplete((DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes)))), noAttributes)
                        };
;
                        let fallback = |i| {
                            /* do */ {
                                warn(invalidAST(ni)(__op_addadd("unknown function: ".to_string(), identToString(i))));
                                return(defType)
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
                                    let ptys = || {
                                        map(declType, pdecls)
                                    };
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
        c(_, CAssign(op, le, re, ni)) => {
            /* do */ {
                let lt = tExpr(c, LValue, le);
                let rt = tExpr(c, RValue, re);
                when((constant(typeQuals(lt))))(typeError(ni)(__op_addadd("assignment to lvalue with `constant\' qualifier: ".to_string(), ((render . pretty))(le))));
                match (canonicalType(lt), re) {
                    (lt_q, CConst(CIntConst(i, _))) => if (isPointerType(lt_q) && (getCInteger(i) == 0)) { return(()) },
                    (_, _) => {
                        assignCompatible_q(ni, op, lt, rt)
                    },
                };
                return(lt)
            }
        },
        c(_, CStatExpr(s, _)) => {
            /* do */ {
                enterBlockScope;
                mapM_(((withDefTable . defineLabel)), (getLabels(s)));
                let t = tStmt(c, s);
                leaveBlockScope;
                return(t)
            }
        },
    }
    }

    fn tInit(__0: MonadTrav) -> MonadTrav {
        match (__0, __1, __2, __3) {
        t(i, <todo>, CInitExpr(e, ni)) => {
            /* do */ {
                let it = tExpr(vec![], RValue, e);
                assignCompatible_q(ni, CAssignOp, t, it);
                return(i)
            }
        },
        t(i, <todo>, CInitList(initList, ni)) => {
            >>(tInitList(ni, (canonicalType(t)), initList), return(i))
        },
    }
    }

    fn tInitList(__0: MonadTrav) -> MonadTrav {
        match (__0, __1, __2, __3, __4) {
        ni(t, <todo>, ArrayType(DirectType(TyIntegral(TyChar), _, _), _, _, _), [[](CInitExpr(e, <todo>, CConst(CStrConst(_, _)), _))]) => {
            >>(tExpr(vec![], RValue, e), return(()))
        },
        ni(t, <todo>, ArrayType(_, _, _, _), initList) => {
            /* do */ {
                {
                        let default_ds = || {
                            repeat((CArrDesig((CConst((CIntConst((cInteger(0)), ni)))), ni)))
                        };
                };
                checkInits(t, default_ds, initList)
            }
        },
        ni(t, <todo>, DirectType(TyComp(ctr), _, _), initList) => {
            /* do */ {
                let td = lookupSUE(ni, (sueRef(ctr)));
                let ms = tagMembers(ni, td);
                {
                        let default_ds = || {
                            map((Lambda((fst(m)), ni)), ms)
                        };
                };
                checkInits(t, default_ds, initList)
            }
        },
        ni(PtrType(DirectType(TyVoid, _, _), _, _), _) => {
            return(())
        },
        _(t, [[](i)]) => {
            >>(tInit(t, i), return(()))
        },
        ni(t, _) => {
            typeError(ni)(__op_addadd("initializer list for type: ".to_string(), pType(t)))
        },
    }
    }

    fn tStmt(__0: MonadTrav) -> MonadTrav {
        match (__0, __1) {
        c(CLabel(_, s, _, _)) => {
            tStmt(c, s)
        },
        c(CExpr(e, _)) => {
            maybe((return(voidType)), (tExpr(c, RValue)), e)
        },
        c(CCompound(ls, body, _)) => {
            /* do */ {
                enterBlockScope;
                mapM_(((withDefTable . defineLabel)), ls);
                let t = foldM((const(tBlockItem(c))), voidType, body);
                leaveBlockScope;
                return(t)
            }
        },
        c(CIf(e, sthen, selse, _)) => {
            >>(checkGuard(c, e), >>(tStmt(c, sthen), >>(maybe((return(())), (>>(Lambda(c, s), return(()))), selse), return(voidType))))
        },
        c(CSwitch(e, s, ni)) => {
            __op_bind(tExpr(c, RValue, e), >>(checkIntegral_q(ni), tStmt((__op_concat(SwitchCtx, c)), s)))
        },
        c(CWhile(e, s, _, _)) => {
            >>(checkGuard(c, e), tStmt((__op_concat(LoopCtx, c)), s))
        },
        _(CGoto(l, ni)) => {
            /* do */ {
                let dt = getDefTable;
                match lookupLabel(l, dt) {
                    Just(_) => {
                        return(voidType)
                    },
                    Nothing => {
                        typeError(ni)(__op_addadd("undefined label in goto: ".to_string(), identToString(l)))
                    },
                }
            }
        },
        c(CCont(ni)) => {
            /* do */ {
                unless((inLoop(c)))(astError(ni, "continue statement outside of loop".to_string()));
                return(voidType)
            }
        },
        c(CBreak(ni)) => {
            /* do */ {
                unless((||(inLoop(c), inSwitch(c))))(astError(ni, "break statement outside of loop or switch statement".to_string()));
                return(voidType)
            }
        },
        c(CReturn(Just(e), ni)) => {
            /* do */ {
                let t = tExpr(c, RValue, e);
                let rt = match enclosingFunctionType(c) {
                        Just(FunctionType(FunType(rt, _, _), _)) => {
                            return(rt)
                        },
                        Just(FunctionType(FunTypeIncomplete(rt), _)) => {
                            return(rt)
                        },
                        Just(ft) => {
                            astError(ni)(__op_addadd("bad function type: ".to_string(), pType(ft)))
                        },
                        Nothing => {
                            astError(ni, "return statement outside function".to_string())
                        },
                    };
                match (rt, t) {
                    (DirectType(TyVoid, _, _), DirectType(TyVoid, _, _)) => {
                        return(())
                    },
                    _ => {
                        assignCompatible_q(ni, CAssignOp, rt, t)
                    },
                };
                return(voidType)
            }
        },
        _(CReturn(Nothing, _)) => {
            return(voidType)
        },
        _(CAsm(_, _)) => {
            return(voidType)
        },
        c(CCase(e, s, ni)) => {
            /* do */ {
                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                __op_bind(tExpr(c, RValue, e), checkIntegral_q(ni));
                tStmt(c, s)
            }
        },
        c(CCases(e1, e2, s, ni)) => {
            /* do */ {
                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                __op_bind(tExpr(c, RValue, e1), checkIntegral_q(ni));
                __op_bind(tExpr(c, RValue, e2), checkIntegral_q(ni));
                tStmt(c, s)
            }
        },
        c(CDefault(s, ni)) => {
            /* do */ {
                unless((inSwitch(c)))(astError(ni, "default statement outside of switch statement".to_string()));
                tStmt(c, s)
            }
        },
        c(CFor(i, g, inc, s, _)) => {
            /* do */ {
                enterBlockScope;
                either((maybe((return(())), checkExpr)), (analyseDecl(True)), i);
                maybe((return(())), (checkGuard(c)), g);
                maybe((return(())), checkExpr, inc);
                tStmt((__op_concat(LoopCtx, c)), s);
                leaveBlockScope;
                return(voidType)
            }
        },
        c(CGotoPtr(e, ni)) => {
            /* do */ {
                let t = tExpr(c, RValue, e);
                match t {
                    PtrType(_, _, _) => {
                        return(voidType)
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

mod Language_C_Analysis_Builtins {
    fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}

mod Language_C_Analysis_ConstEval {
    struct MachineDesc(MachineDesc, { /* struct def */ });

    fn alignofType(__0: m) -> m {
        match (__0, __1, __2) {
        md(_, DirectType(TyVoid, _, _)) => {
            return(voidAlign(md))
        },
        md(_, DirectType(TyIntegral(it), _, _)) => {
            return(iAlign(md, it))
        },
        md(_, DirectType(TyFloating(ft), _, _)) => {
            return(fAlign(md, ft))
        },
        md(_, DirectType(TyComplex(ft), _, _)) => {
            return(fAlign(md, ft))
        },
        md(_, DirectType(TyEnum(_), _, _)) => {
            return(iAlign(md, TyInt))
        },
        md(_, DirectType(TyBuiltin(b), _, _)) => {
            return(builtinAlign(md, b))
        },
        md(_, PtrType(_, _, _)) => {
            return(ptrAlign(md))
        },
        md(n, ArrayType(bt, UnknownArraySize(_), _, _)) => {
            return(ptrAlign(md))
        },
        md(n, ArrayType(bt, ArraySize(_, sz), _, _)) => {
            alignofType(md, n, bt)
        },
        md(n, TypeDefType(TypeDefRef(_, Just(t), _), _, _)) => {
            alignofType(md, n, t)
        },
        _(n, t) => {
            astError((nodeInfo(n)))(__op_addadd("can\'t find alignment of type: ".to_string(), ((render . pretty))(t)))
        },
    }
    }

    fn boolValue(__0: Maybe) -> Maybe {
        match (__0) {
        CConst(CIntConst(i, _)) => {
            Just(/=(getCInteger(i), 0))
        },
        CConst(CCharConst(c, _)) => {
            Just(/=(getCCharAsInt(c), 0))
        },
        CConst(CStrConst(_, _)) => {
            Just(True)
        },
        _ => {
            Nothing
        },
    }
    }

    fn compSize(md: MonadTrav) -> MonadTrav {
        /* do */ {
            let dt = getDefTable;
            match lookupTag((sueRef(ctr)), dt) {
                Just(Left(_)) => {
                    astError((nodeInfo(ctr)), "composite declared but not defined".to_string())
                },
                Just(Right(CompDef(CompType(_, tag, ms, _, ni)))) => {
                    /* do */ {
                        {
                                let ts = || {
                                    map(declType, ms)
                                };
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
                Just(Right(EnumDef(_))) => {
                    return(iSize(md, TyInt))
                },
                Nothing => {
                    astError((nodeInfo(ctr)), "unknown composite".to_string())
                },
            }
        }
    }

    fn constEval(__0: Map.Map) -> Map.Map {
        match (__0, __1, __2) {
        md(env, CCond(e1, me2, e3, ni)) => {
            /* do */ {
                let e1_q = constEval(md, env, e1);
                let me2_q = maybe((return(Nothing)), (liftM(Lambda, constEval(md, env, e))), me2);
                let e3_q = constEval(md, env, e3);
                match boolValue(e1_q) {
                    Just(True) => {
                        return(fromMaybe(e1_q, me2_q))
                    },
                    Just(False) => {
                        return(e3_q)
                    },
                    Nothing => {
                        return(CCond(e1_q, me2_q, e3_q, ni))
                    },
                }
            }
        },
        md(env, e, <todo>, CBinary(op, e1, e2, ni)) => {
            /* do */ {
                let e1_q = constEval(md, env, e1);
                let e2_q = constEval(md, env, e2);
                let t = tExpr(vec![], RValue, e);
                let bytes = liftM(fromIntegral, sizeofType(md, e, t));
                match (intValue(e1_q), intValue(e2_q)) {
                    (Just(i1), Just(i2)) => {
                        intExpr(ni, (withWordBytes(bytes, (intOp(op, i1, i2)))))
                    },
                    (_, _) => {
                        return(CBinary(op, e1_q, e2_q, ni))
                    },
                }
            }
        },
        md(env, CUnary(op, e, ni)) => {
            /* do */ {
                let e_q = constEval(md, env, e);
                let t = tExpr(vec![], RValue, e);
                let bytes = liftM(fromIntegral, sizeofType(md, e, t));
                match intValue(e_q) {
                    Just(i) => {
                        match intUnOp(op, i) {
                            Just(i_q) => {
                                intExpr(ni, (withWordBytes(bytes, i_q)))
                            },
                            Nothing => {
                                astError(ni, "invalid unary operator applied to constant".to_string())
                            },
                        }
                    },
                    Nothing => {
                        return(CUnary(op, e_q, ni))
                    },
                }
            }
        },
        md(env, CCast(d, e, ni)) => {
            /* do */ {
                let e_q = constEval(md, env, e);
                let t = analyseTypeDecl(d);
                let bytes = liftM(fromIntegral, sizeofType(md, d, t));
                match intValue(e_q) {
                    Just(i) => {
                        intExpr(ni, (withWordBytes(bytes, i)))
                    },
                    Nothing => {
                        return(CCast(d, e_q, ni))
                    },
                }
            }
        },
        md(_, CSizeofExpr(e, ni)) => {
            /* do */ {
                let t = tExpr(vec![], RValue, e);
                let sz = sizeofType(md, e, t);
                intExpr(ni, sz)
            }
        },
        md(_, CSizeofType(d, ni)) => {
            /* do */ {
                let t = analyseTypeDecl(d);
                let sz = sizeofType(md, d, t);
                intExpr(ni, sz)
            }
        },
        md(_, CAlignofExpr(e, ni)) => {
            /* do */ {
                let t = tExpr(vec![], RValue, e);
                let sz = alignofType(md, e, t);
                intExpr(ni, sz)
            }
        },
        md(_, CAlignofType(d, ni)) => {
            /* do */ {
                let t = analyseTypeDecl(d);
                let sz = alignofType(md, d, t);
                intExpr(ni, sz)
            }
        },
        md(env, e, <todo>, CVar(i, _)) => {
            /* do */ {
                let t = tExpr(vec![], RValue, e);
                match derefTypeDef(t) {
                    DirectType(TyEnum(etr), _, _) => {
                        /* do */ {
                            let dt = getDefTable;
                            match lookupTag((sueRef(etr)), dt) {
                                Just(Right(EnumDef(EnumType(_, es, _, _)))) => {
                                    /* do */ {
                                        let env_q = foldM(enumConst, env, es);
                                        return(fromMaybe(e)(Map_lookup(i, env_q)))
                                    }
                                },
                                _ => {
                                    return(e)
                                },
                            }
                        }
                    },
                    _ => {
                        return(e)
                    },
                }
            }
        },
        _(_, e) => {
            return(e)
        },
    }
    }

    fn intExpr(n: m) -> m {
        __op_bind(genName, Lambda(CConst(CIntConst((cInteger(i)), (mkNodeInfo((posOf(n)), name))))))
    }

    fn intOp(__0: Integer) -> Integer {
        match (__0, __1, __2) {
        CAddOp(i1, i2) => {
            +(i1, i2)
        },
        CSubOp(i1, i2) => {
            -(i1, i2)
        },
        CMulOp(i1, i2) => {
            *(i1, i2)
        },
        CDivOp(i1, i2) => {
            div(i1, i2)
        },
        CRmdOp(i1, i2) => {
            mod(i1, i2)
        },
        CShlOp(i1, i2) => {
            shiftL(i1, fromInteger(i2))
        },
        CShrOp(i1, i2) => {
            shiftR(i1, fromInteger(i2))
        },
        CLeOp(i1, i2) => {
            toInteger(fromEnum(<(i1, i2)))
        },
        CGrOp(i1, i2) => {
            toInteger(fromEnum(>(i1, i2)))
        },
        CLeqOp(i1, i2) => {
            toInteger(fromEnum(<=(i1, i2)))
        },
        CGeqOp(i1, i2) => {
            toInteger(fromEnum(>=(i1, i2)))
        },
        CEqOp(i1, i2) => {
            toInteger(fromEnum((i1 == i2)))
        },
        CNeqOp(i1, i2) => {
            toInteger(fromEnum(/=(i1, i2)))
        },
        CAndOp(i1, i2) => {
            .&.(i1, i2)
        },
        CXorOp(i1, i2) => {
            xor(i1, i2)
        },
        COrOp(i1, i2) => {
            .|.(i1, i2)
        },
        CLndOp(i1, i2) => {
            toInteger(fromEnum(((/=(i1, 0)) && (/=(i2, 0)))))
        },
        CLorOp(i1, i2) => {
            toInteger(fromEnum(||((/=(i1, 0)), (/=(i2, 0)))))
        },
    }
    }

    fn intUnOp(__0: Maybe) -> Maybe {
        match (__0, __1) {
        CPlusOp(i) => {
            Just(i)
        },
        CMinOp(i) => {
            Just(Operator("-")(i))
        },
        CCompOp(i) => {
            Just(complement(i))
        },
        CNegOp(i) => {
            Just(toInteger(fromEnum((i == 0))))
        },
        _(_) => {
            Nothing
        },
    }
    }

    fn intValue(__0: Maybe) -> Maybe {
        match (__0) {
        CConst(CIntConst(i, _)) => {
            Just(getCInteger(i))
        },
        CConst(CCharConst(c, _)) => {
            Just(getCCharAsInt(c))
        },
        _ => {
            Nothing
        },
    }
    }

    fn sizeofType(__0: m) -> m {
        match (__0, __1, __2) {
        md(_, DirectType(TyVoid, _, _)) => {
            return(voidSize(md))
        },
        md(_, DirectType(TyIntegral(it), _, _)) => {
            return(iSize(md, it))
        },
        md(_, DirectType(TyFloating(ft), _, _)) => {
            return(fSize(md, ft))
        },
        md(_, DirectType(TyComplex(ft), _, _)) => {
            return(*(2, fSize(md, ft)))
        },
        md(_, DirectType(TyComp(ctr), _, _)) => {
            compSize(md, ctr)
        },
        md(_, DirectType(TyEnum(_), _, _)) => {
            return(iSize(md, TyInt))
        },
        md(_, DirectType(TyBuiltin(b), _, _)) => {
            return(builtinSize(md, b))
        },
        md(_, PtrType(_, _, _)) => {
            return(ptrSize(md))
        },
        md(n, ArrayType(bt, UnknownArraySize(_), _, _)) => {
            return(ptrSize(md))
        },
        md(n, ArrayType(bt, ArraySize(_, sz), _, _)) => {
            /* do */ {
                let sz_q = constEval(md, Map_empty, sz);
                match sz_q {
                    CConst(CIntConst(i, _)) => {
                        /* do */ {
                            let s = sizeofType(md, n, bt);
                            return(*(getCInteger(i), s))
                        }
                    },
                    _ => {
                        return(ptrSize(md))
                    },
                }
            }
        },
        md(n, TypeDefType(TypeDefRef(_, Just(t), _), _, _)) => {
            sizeofType(md, n, t)
        },
        md(_, FunctionType(_, _)) => {
            return(ptrSize(md))
        },
        _(n, t) => {
            astError((nodeInfo(n)))(__op_addadd("can\'t find size of type: ".to_string(), ((render . pretty))(t)))
        },
    }
    }

    fn withWordBytes(bytes: Integer) -> Integer {
        rem(n, (shiftL(1, (shiftL(bytes, 3)))))
    }

}

mod Language_C_Analysis_Debug {
    fn globalDeclStats(file_filter: Vec<(String, isize)>) -> Vec<(String, isize)> {
        vec![("Enumeration Constants".to_string(), Map_size(enumerators)), ("Total Object/Function Declarations".to_string(), Map_size(all_decls)), ("Object definitions".to_string(), Map_size(objDefs)), ("Function Definitions".to_string(), Map_size(funDefs)), ("Tag definitions".to_string(), Map_size(tagDefs)), ("TypeDefs".to_string(), Map_size(typeDefs))]
    }

    fn joinComma() -> Doc {
        (hsep . (punctuate(comma) . map(pretty)))
    }

    fn prettyAssocs(label: Doc) -> Doc {
        prettyAssocsWith(label, pretty, pretty)
    }

    fn prettyAssocsWith(label: Doc) -> Doc {
        $$(text(label), (nest(8))((vcat(map(prettyEntry, theMap)))))
    }

    fn terminateSemi() -> Doc {
        (terminateSemi_ . map(pretty))
    }

    fn terminateSemi_() -> Doc {
        (hsep . map((Operator("<>")(semi))))
    }

}

mod Language_C_Analysis_DeclAnalysis {
    #[derive(Debug, Eq, Ord, Read)]
    enum StorageSpec {
        NoStorageSpec,
        AutoSpec,
        RegSpec,
        ThreadSpec,
        StaticSpec(Bool),
        ExternSpec(Bool)
    }

    struct VarDeclInfo(VarDeclInfo, VarName, Bool, StorageSpec, Attributes, Type, NodeInfo);

    #[derive(Eq, Ord)]
    enum NumBaseType {
        NoBaseType,
        BaseChar,
        BaseInt,
        BaseFloat,
        BaseDouble
    }

    #[derive(Eq, Ord)]
    enum SignSpec {
        NoSignSpec,
        Signed,
        Unsigned
    }

    #[derive(Eq, Ord)]
    enum SizeMod {
        NoSizeMod,
        ShortMod,
        LongMod,
        LongLongMod
    }

    struct NumTypeSpec(NumTypeSpec, { /* struct def */ });

    enum TypeSpecAnalysis {
        TSNone,
        TSVoid,
        TSBool,
        TSNum(NumTypeSpec),
        TSTypeDef(TypeDefRef),
        TSType(Type),
        TSNonBasic(CTypeSpec)
    }

    fn analyseVarDecl(handle_sue_def: m) -> m {
        /* do */ {
            let storage_spec = canonicalStorageSpec(storage_specs);
            let typ = tType(handle_sue_def, node, typequals, canonTySpecs, derived_declrs, oldstyle_params);
            let attrs_q = mapM(tAttr, (__op_addadd(decl_attrs, declr_attrs)));
            let name = mkVarName(node, name_opt, asmname_opt);
            return(VarDeclInfo(name, inline, storage_spec, attrs_q, typ, node))
        }
    }

    fn analyseVarDecl_q(handle_sue_def: m) -> m {
        /* do */ {
            {
                    let (storage_specs, attrs, type_quals, type_specs, inline) = || {
                        partitionDeclSpecs(declspecs)
                    };
            };
            let canonTySpecs = canonicalTypeSpec(type_specs);
            analyseVarDecl(handle_sue_def, storage_specs, attrs, type_quals, canonTySpecs, inline, declr, oldstyle, init_opt)
        }
    }

    fn canonicalStorageSpec(storagespecs: m) -> m {
        liftM(elideAuto)(foldrM(updStorage, NoStorageSpec, storagespecs))
    }

    fn canonicalTypeSpec() -> m {
        foldrM(go, TSNone)
    }

    fn computeParamStorage(__0: Either) -> Either {
        match (__0, __1) {
        _(NoStorageSpec) => {
            Right((Auto(False)))
        },
        _(RegSpec) => {
            Right((Auto(True)))
        },
        node(spec) => {
            (Left . badSpecifierError(node)(__op_addadd("Bad storage specified for parameter: ".to_string(), show(spec))))
        },
    }
    }

    fn emptyDeclr(node: CDeclr) -> CDeclr {
        CDeclr(Nothing, vec![], Nothing, vec![], node)
    }

    fn emptyNumTypeSpec() -> NumTypeSpec {
        NumTypeSpec({
        base: NoBaseType,
        signSpec: NoSignSpec,
        sizeMod: NoSizeMod,
        isComplex: False
        })
    }

    fn getOnlyDeclr(__0: m) -> m {
        match (__0) {
        CDecl(_, [Just(declr)(_, _)], _) => {
            return(declr)
        },
        CDecl(_, _, node) => {
            internalErr("getOnlyDeclr: declaration doesn\'t have a unique declarator".to_string())
        },
    }
    }

    fn hasThreadLocalSpec(__0: Bool) -> Bool {
        match (__0) {
        ThreadSpec => {
            True
        },
        StaticSpec(b) => {
            b
        },
        ExternSpec(b) => {
            b
        },
        _ => {
            False
        },
    }
    }

    fn isTypeDef(declspecs: Bool) -> Bool {
        not(null(Dummy))
    }

    fn mergeOldStyle(__0: m) -> m {
        match (__0, __1, __2) {
        _node([], declrs) => {
            return(declrs)
        },
        node(oldstyle_params, CFunDeclr(params, attrs, fdnode, <todo>, dds)) => {
            match params {
                Left(list) => {
                    /* do */ {
                        let oldstyle_params_q = liftM(concat)(mapM(splitCDecl, oldstyle_params));
                        let param_map = liftM(Map_fromList)(mapM(attachNameOfDecl, oldstyle_params_q));
                        let (newstyle_params, param_map_q) = foldrM(insertParamDecl, (vec![], param_map), list);
                        when((not(Map_null(param_map_q))))(astError(node)(__op_addadd("declarations for parameter(s) ".to_string(), __op_addadd(showParamMap(param_map_q), " but no such parameter".to_string()))));
                        return((__op_concat(CFunDeclr((Right((newstyle_params, False))), attrs, fdnode), dds)))
                    }
                },
                Right(_newstyle) => {
                    astError(node, "oldstyle parameter list, but newstyle function declaration".to_string())
                },
            }
        },
        node(_, _) => {
            astError(node, "oldstyle parameter list, but not function type".to_string())
        },
    }
    }

    fn mergeTypeAttributes(node_info: m) -> m {
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

    fn mkVarName(__0: Maybe) -> Maybe {
        match (__0, __1, __2) {
        node(Nothing, _) => {
            return(NoName)
        },
        node(Just(n), asm) => {
            return(VarName(n, asm))
        },
    }
    }

    fn nameOfDecl(d: m) -> m {
        __op_bind(getOnlyDeclr(d), Lambda)
    }

    fn splitCDecl(decl: m) -> m {
        match declrs {
            [] => {
                internalErr("splitCDecl applied to empty declaration".to_string())
            },
            [declr] => {
                return(vec![decl])
            },
            d1:ds => {
                {
                        let declspecs_q = || {
                            map(elideSUEDef, declspecs, in)
                        };
                }(return)(__op_concat((CDecl(declspecs, vec![d1], node)), Dummy))
            },
        }
    }

    fn tArraySize(__0: m) -> m {
        match (__0) {
        CNoArrSize(False) => {
            return((UnknownArraySize(False)))
        },
        CNoArrSize(True) => {
            return((UnknownArraySize(True)))
        },
        CArrSize(static, szexpr) => {
            liftM((ArraySize(static)), (return(szexpr)))
        },
    }
    }

    fn tAttr((CAttr(name, cexpr, node)): m) -> m {
        return(Attr(name, cexpr, node))
    }

    fn tCompType(tag: m) -> m {
        ap(return((CompType(tag, sue_ref))), ap((concatMapM(tMemberDecls, member_decls)), ap((return(attrs)), (return(node)))))
    }

    fn tCompTypeDecl(handle_def: m) -> m {
        /* do */ {
            let sue_ref = createSUERef(node_info, ident_opt);
            {
                    let tag_q = || {
                        tTag(tag)
                    };
            };
            let attrs_q = mapM(tAttr, attrs);
            {
                    let decl = || {
                        CompTypeRef(sue_ref, tag_q, node_info)
                    };
            };
            handleTagDecl((CompDecl(decl)));
            when((handle_def))(/* do */ {
                    maybeM(member_decls_opt)(__op_bind(Lambda(sue_ref, tag_q, decls, (attrs_q), node_info), (handleTagDef_CompDef)))
                });
            return(decl)
        }
    }

    fn tDirectType(handle_sue_def: m) -> m {
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
                        (return . baseType(match numType {
                                    Left, (floatType, iscomplex) => if iscomplex { TyComplex(floatType) }
otherwise { TyFloating(floatType) },
                                    Right(intType) => {
                                        TyIntegral(intType)
                                    },
                                }))
                    }
                },
                TSTypeDef(tdr) => {
                    return(TypeDefType(tdr, quals, attrs))
                },
                TSNonBasic(CSUType(su, _tnode)) => {
                    liftM(((baseType . TyComp)))(tCompTypeDecl(handle_sue_def, su))
                },
                TSNonBasic(CEnumType(enum, _tnode)) => {
                    liftM(((baseType . TyEnum)))(tEnumTypeDecl(handle_sue_def, enum))
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

    fn tEnumType(sue_ref: m) -> m {
        /* do */ {
            mapM_(handleEnumeratorDef, enumerators_q);
            return(ty);

        }
    }

    fn tMemberDecls(__0: m) -> m {
        match (__0) {
        CDecl(declspecs, [], node) => {
            /* do */ {
                {
                        let (storage_specs, _attrs, typequals, typespecs, is_inline) = || {
                            partitionDeclSpecs(declspecs)
                        };
                };
                when(is_inline)(astError(node, "member declaration with inline specifier".to_string()));
                let canonTySpecs = canonicalTypeSpec(typespecs);
                let ty = tType(True, node, typequals, canonTySpecs, vec![], vec![]);
                match ty {
                    DirectType(TyComp(_), _, _) => {
                        return(vec![MemberDecl((VarDecl(NoName, (DeclAttrs(False, NoStorage, vec![])), ty)), Nothing, node)])
                    },
                    _ => {
                        astError(node, "anonymous member has a non-composite type".to_string())
                    },
                }
            }
        },
        CDecl(declspecs, declrs, node) => {
            mapM((uncurry(tMemberDecl)), (zip((True:repeat(False)), declrs)))
        },
    }
    }

    fn tNumType((NumTypeSpec(basetype, sgn, sz, iscomplex)): m) -> m {
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
                error("Bad AST analysis".to_string())
            },
        }
    }

    fn tParamDecl((CDecl(declspecs, declrs, node)): m) -> m {
        /* do */ {
            let declr = getParamDeclr;
            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl_q(True, declspecs, declr, vec![], Nothing);
            when((is_inline))(throwTravError((badSpecifierError(node, "parameter declaration with inline specifier".to_string()))));
            let storage = throwOnLeft(computeParamStorage(node, storage_spec));
            {
                    let paramDecl = || {
                        mkParamDecl(name, storage, attrs, ty, declr_node)
                    };
            };
            return(paramDecl)
        }
    }

    fn tTag(__0: CompTyKind) -> CompTyKind {
        match (__0) {
        CStructTag => {
            StructTag
        },
        CUnionTag => {
            UnionTag
        },
    }
    }

    fn tType(handle_sue_def: m) -> m {
        __op_bind(mergeOldStyle(top_node, oldstyle_params, derived_declrs), buildType)
    }

    fn tTypeQuals() -> m {
        foldrM(go, (noTypeQuals, vec![]))
    }

    fn typeDefRef(t_node: m) -> m {
        __op_bind(lookupTypeDef(name), Lambda((TypeDefRef(name, (Just(ty)), t_node))))
    }

}

mod Language_C_Analysis_DefTable {
    enum TagFwdDecl {
        CompDecl(CompTypeRef),
        EnumDecl(EnumTypeRef)
    }

    struct DefTable(DefTable, { /* struct def */ });

    #[derive(Clone, Debug)]
    enum DeclarationStatus {
        NewDecl,
        Redeclared(t),
        KeepDef(t),
        Shadowed(t),
        KindMismatch(t)
    }

    #[derive(Eq, Ord)]
    enum TagEntryKind {
        CompKind(CompTyKind),
        EnumKind
    }

    fn compatIdentEntry(__0: Bool) -> Bool {
        match (__0) {
        Left(_tydef) => {
            either((const(True)), (const(False)))
        },
        Right(def) => {
            either((const(False)))(Lambda)
        },
    }
    }

    fn compatTagEntry(te1: Bool) -> Bool {
        (tagKind(te1) == tagKind(te2))
    }

    fn declStatusDescr(__0: DeclarationStatus) -> DeclarationStatus {
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

    fn declareTag(sueref: (DeclarationStatus<TagEntry>, DefTable)) -> (DeclarationStatus<TagEntry>, DefTable) {
        match lookupTag(sueref, deftbl) {
            Nothing => {
                (NewDecl, deftbl({
                    tagDecls: fst(defLocal((tagDecls(deftbl)), sueref, (Left(decl))))
                    }))
            },
            Just, old_def => if (tagKind(old_def) == tagKind((Left(decl)))) { (KeepDef(old_def), deftbl) }
otherwise { (KindMismatch(old_def), deftbl) },
        }
    }

    fn defRedeclStatus(sameKind: Maybe) -> Maybe {
        match oldDecl {
            Just, def_q => if sameKind(def, def_q) { Redeclared(def_q) }
otherwise { KindMismatch(def_q) },
            Nothing => {
                NewDecl
            },
        }
    }

    fn defRedeclStatusLocal(sameKind: Maybe) -> Maybe {
        match defRedeclStatus(sameKind, def, oldDecl) {
            NewDecl => {
                match lookupName(nsm, ident) {
                    Just(shadowed) => {
                        Shadowed(shadowed)
                    },
                    Nothing => {
                        NewDecl
                    },
                }
            },
            redecl => {
                redecl
            },
        }
    }

    fn defineGlobalIdent(ident: (DeclarationStatus<IdentEntry>, DefTable)) -> (DeclarationStatus<IdentEntry>, DefTable) {
        (defRedeclStatus(compatIdentEntry, (Right(def)), oldDecl), deftbl({
            identDecls: decls_q
            }))
    }

    fn defineLabel(ident: (DeclarationStatus<Ident>, DefTable)) -> (DeclarationStatus<Ident>, DefTable) {
        {
            let (labels_q, old_label) = || {
                defLocal((labelDefs(deftbl)), ident, ident)
            };
    }(in, (maybe(NewDecl, Redeclared, old_label), deftbl({
        labelDefs: labels_q
        })))
    }

    fn defineScopedIdent() -> (DeclarationStatus<IdentEntry>, DefTable) {
        defineScopedIdentWhen((const(True)))
    }

    fn defineScopedIdentWhen(override_def: (DeclarationStatus<IdentEntry>, DefTable)) -> (DeclarationStatus<IdentEntry>, DefTable) {
        (redecl_status, deftbl({
            identDecls: decls_q
            }))
    }

    fn defineTag(sueref: (DeclarationStatus<TagEntry>, DefTable)) -> (DeclarationStatus<TagEntry>, DefTable) {
        (redeclStatus, deftbl({
            tagDecls: decls_q
            }))
    }

    fn defineTypeDef(ident: (DeclarationStatus<IdentEntry>, DefTable)) -> (DeclarationStatus<IdentEntry>, DefTable) {
        (defRedeclStatus(compatIdentEntry, (Left(tydef)), oldDecl), deftbl({
            identDecls: decls_q
            }))
    }

    fn emptyDefTable() -> DefTable {
        DefTable(nameSpaceMap, nameSpaceMap, nameSpaceMap, nameSpaceMap, IntMap_empty, IntMap_empty)
    }

    fn enterBlockScope(deftbl: DefTable) -> DefTable {
        enterLocalScope(deftbl({
            labelDefs: enterNewScope((labelDefs(deftbl)))
            }))
    }

    fn enterFunctionScope(deftbl: DefTable) -> DefTable {
        enterLocalScope(deftbl({
            labelDefs: enterNewScope((labelDefs(deftbl)))
            }))
    }

    fn enterLocalScope(deftbl: DefTable) -> DefTable {
        deftbl({
        identDecls: enterNewScope((identDecls(deftbl))),
        tagDecls: enterNewScope((tagDecls(deftbl)))
        })
    }

    fn enterMemberDecl(deftbl: DefTable) -> DefTable {
        deftbl({
        memberDecls: enterNewScope((memberDecls(deftbl)))
        })
    }

    fn globalDefs(deftbl: GlobalDecls) -> GlobalDecls {
        Map_foldWithKey(insertDecl, (GlobalDecls(e, gtags, e)), (globalNames(identDecls(deftbl))))
    }

    fn identOfTyDecl() -> Ident {
        either(identOfTypeDef, declIdent)
    }

    fn inFileScope(dt: Bool) -> Bool {
        not((||(hasLocalNames((identDecls(dt))), hasLocalNames((labelDefs(dt))))))
    }

    fn insertType(dt: DefTable) -> DefTable {
        dt({
        typeTable: IntMap_insert((nameId(n)), t, (typeTable(dt)))
        })
    }

    fn leaveBlockScope(deftbl: DefTable) -> DefTable {
        leaveLocalScope(deftbl({
            labelDefs: leaveScope_((labelDefs(deftbl)))
            }))
    }

    fn leaveFunctionScope(deftbl: DefTable) -> DefTable {
        leaveLocalScope(deftbl({
            labelDefs: leaveScope_((labelDefs(deftbl)))
            }))
    }

    fn leaveLocalScope(deftbl: DefTable) -> DefTable {
        deftbl({
        identDecls: leaveScope_((identDecls(deftbl))),
        tagDecls: leaveScope_((tagDecls(deftbl)))
        })
    }

    fn leaveMemberDecl(deftbl: (Vec<MemberDecl>, DefTable)) -> (Vec<MemberDecl>, DefTable) {
        {
            let (decls_q, members) = || {
                leaveScope((memberDecls(deftbl)))
            };
    }(in, Dummy, (map(snd, members)), (deftbl({
        memberDecls: decls_q
        })))
    }

    fn leaveScope_() -> NameSpaceMap {
        (fst . leaveScope)
    }

    fn lookupIdent(ident: Maybe) -> Maybe {
        lookupName((identDecls(deftbl)), ident)
    }

    fn lookupIdentInner(ident: Maybe) -> Maybe {
        lookupInnermostScope((identDecls(deftbl)), ident)
    }

    fn lookupLabel(ident: Maybe) -> Maybe {
        lookupName((labelDefs(deftbl)), ident)
    }

    fn lookupTag(sue_ref: Maybe) -> Maybe {
        lookupName((tagDecls(deftbl)), sue_ref)
    }

    fn lookupTagInner(sue_ref: Maybe) -> Maybe {
        lookupInnermostScope((tagDecls(deftbl)), sue_ref)
    }

    fn lookupType(dt: Maybe) -> Maybe {
        IntMap_lookup((nameId(n)), (typeTable(dt)))
    }

    fn mergeDefTable((DefTable(i1, t1, l1, m1, r1, tt1)): DefTable) -> DefTable {
        DefTable((mergeNameSpace(i1, i2)), (mergeNameSpace(t1, t2)), (mergeNameSpace(l1, l2)), (mergeNameSpace(m1, m2)), (union(r1, r2)), (union(tt1, tt2)))
    }

    fn tagKind(__0: TagEntryKind) -> TagEntryKind {
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

mod Language_C_Analysis_Export {
    fn exportArraySize(__0: CArrSize) -> CArrSize {
        match (__0) {
        ArraySize(static, e) => {
            CArrSize(static, e)
        },
        UnknownArraySize(complete) => {
            CNoArrSize(complete)
        },
    }
    }

    fn exportAttrs() -> Vec<CAttr> {
        map(exportAttr)
    }

    fn exportCompType((CompType(sue_ref, comp_tag, members, attrs, node_info)): Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        vec![CSUType(comp, ni)]
    }

    fn exportCompTypeDecl(ty: Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        vec![CSUType((exportComp(ty)), ni)]
    }

    fn exportCompTypeRef((CompType(sue_ref, com_tag, _, _, node_info)): Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
    }

    fn exportComplexType(ty: Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        __op_concat((CComplexType(ni)), exportFloatType(ty))
    }

    fn exportDeclAttrs((DeclAttrs(inline, storage, attrs)): Vec<CDeclSpec>) -> Vec<CDeclSpec> {
        __op_addadd((if(inline, then, vec![CTypeQual((CInlineQual(ni)))], else, vec![])), __op_addadd(map((CStorageSpec), (exportStorage(storage))), map(((CTypeQual . CAttrQual)), (exportAttrs(attrs)))))
    }

    fn exportDeclr(other_specs: (Vec<CDeclSpec>, CDeclr)) -> (Vec<CDeclSpec>, CDeclr) {
        (__op_addadd(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
    }

    fn exportEnumType((EnumType(sue_ref, enumerators, attrs, node_info)): Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        vec![CEnumType(enum, ni)]
    }

    fn exportEnumTypeDecl(ty: Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        vec![CEnumType((exportEnum(ty)), ni)]
    }

    fn exportEnumTypeRef((EnumType(sue_ref, _, _, node_info)): Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        exportEnumTypeDecl((EnumTypeRef(sue_ref, node_info)))
    }

    fn exportFloatType(ty: Vec<CTypeSpec>) -> Vec<CTypeSpec> {
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

    fn exportIntType(ty: Vec<CTypeSpec>) -> Vec<CTypeSpec> {
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

    fn exportMemberDecl(__0: CDecl) -> CDecl {
        match (__0) {
        AnonBitField(ty, expr, node_info) => {
            CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info)
        },
        MemberDecl(vardecl, bitfieldsz, node_info) => {
            {
                let (specs, declarator) = || {
                    exportVarDecl(vardecl)
                };
        }(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info)
        },
    }
    }

    fn exportParamDecl(paramdecl: CDecl) -> CDecl {
        {
            let (specs, declr) = || {
                exportVarDecl((getVarDecl(paramdecl)))
            };
    }(in, CDecl, specs, vec![(Just(declr), Nothing, Nothing)], (nodeInfo(paramdecl)))
    }

    fn exportSUERef() -> Maybe {
        (Just . (internalIdent . show))
    }

    fn exportStorage(__0: Vec<CStorageSpec>) -> Vec<CStorageSpec> {
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
            error("impossible storage: static without linkage".to_string())
        },
        FunLinkage(InternalLinkage) => {
            vec![CStatic(ni)]
        },
        FunLinkage(ExternalLinkage) => {
            vec![]
        },
        FunLinkage(NoLinkage) => {
            error("impossible storage: function without linkage".to_string())
        },
    }
    }

    fn exportType(ty: (Vec<CDeclSpec>, Vec<CDerivedDeclr>)) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {
        exportTy(vec![], ty)
    }

    fn exportTypeDecl(ty: CDecl) -> CDecl {
        CDecl(declspecs, declrs, ni)
    }

    fn exportTypeDef((TypeDef(ident, ty, attrs, node_info)): CDecl) -> CDecl {
        CDecl((__op_concat(CStorageSpec((CTypedef(ni))), declspecs)), vec![declr], node_info)
    }

    fn exportTypeQuals(quals: Vec<CTypeQual>) -> Vec<CTypeQual> {
        mapMaybe(select, vec![(constant, CConstQual(ni)), (volatile, CVolatQual(ni)), (restrict, CRestrQual(ni))])
    }

    fn exportTypeQualsAttrs(tyqs: Vec<CTypeQual>) -> Vec<CTypeQual> {
        (__op_addadd(exportTypeQuals(tyqs), map(CAttrQual, (exportAttrs(attrs)))))
    }

    fn exportTypeSpec(tyname: Vec<CTypeSpec>) -> Vec<CTypeSpec> {
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

    fn exportVarDecl((VarDecl(name, attrs, ty)): (Vec<CDeclSpec>, CDeclr)) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    }

    fn fromDirectType(__0: TypeName) -> TypeName {
        match (__0) {
        DirectType(ty, _, _) => {
            ty
        },
        TypeDefType(TypeDefRef(_, ref, _), _, _) => {
            maybe((error("undefined typeDef".to_string())), fromDirectType, ref)
        },
        _ => {
            error("fromDirectType".to_string())
        },
    }
    }

    fn ni() -> NodeInfo {
        undefNode
    }

    fn threadLocal(__0: Vec<CStorageSpec>) -> Vec<CStorageSpec> {
        match (__0) {
        False => {
            id
        },
        True => {
            ((CThread(ni))(Operator(":")))
        },
    }
    }

}

mod Language_C_Analysis_NameSpaceMap {
    struct NameSpaceMap(NsMap, Map<k, v>, Vec<Vec<(k, v)>>);

    fn defGlobal((NsMap(gs, lss)): NameSpaceMap) -> NameSpaceMap {
        (NsMap((Map_insert(ident, def, gs)), lss), Map_lookup(ident, gs))
    }

    fn defLocal(__0: NameSpaceMap) -> NameSpaceMap {
        match (__0, __1, __2, __3, __4) {
        ns(<todo>, NsMap(_, []), ident, def) => {
            defGlobal(ns, ident, def)
        },
        NsMap(gs, ls:lss)(ident, def) => {
            (NsMap(gs, (__op_concat((__op_concat((ident, def), ls)), lss))), Prelude_lookup(ident, ls))
        },
    }
    }

    fn enterNewScope((NsMap(gs, lss)): NameSpaceMap) -> NameSpaceMap {
        NsMap(gs, (__op_concat(vec![], lss)))
    }

    fn globalNames((NsMap(g, _)): NameSpaceMap) -> NameSpaceMap {
        g
    }

    fn hasLocalNames((NsMap(_, l)): NameSpaceMap) -> NameSpaceMap {
        not((null(l)))
    }

    fn leaveScope(__0: NameSpaceMap) -> NameSpaceMap {
        match (__0) {
        NsMap(_, []) => {
            error("NsMaps.leaveScope: No local scope!".to_string())
        },
        NsMap(gs, ls:lss) => {
            (NsMap(gs, lss), ls)
        },
    }
    }

    fn localNames((NsMap(_, l)): NameSpaceMap) -> NameSpaceMap {
        l
    }

    fn lookupGlobal((NsMap(gs, _)): NameSpaceMap) -> NameSpaceMap {
        Map_lookup(ident, gs)
    }

    fn lookupInnermostScope(nsm: NameSpaceMap) -> NameSpaceMap {
        match localDefs {
            ls(:, _lss) => {
                Prelude_lookup(ident, ls)
            },
            [] => {
                lookupGlobal(nsm, ident)
            },
        }
    }

    fn lookupName(ns: NameSpaceMap) -> NameSpaceMap {
        match (lookupLocal(localDefs)) {
            Nothing => {
                lookupGlobal(ns, ident)
            },
            Just(def) => {
                Just(def)
            },
        }
    }

    fn mergeNameSpace((NsMap(global1, local1)): NameSpaceMap) -> NameSpaceMap {
        NsMap((Map_union(global1, global2)), (localUnion(local1, local2)))
    }

    fn nameSpaceMap() -> NameSpaceMap {
        NsMap(Map_empty, vec![])
    }

    fn nsMapToList((NsMap(gs, lss)): NameSpaceMap) -> NameSpaceMap {
        __op_addadd(concat(lss), Map_toList(gs))
    }

}

mod Language_C_Analysis_SemError {
    #[derive(Debug)]
    struct RedefError(RedefError, ErrorLevel, RedefInfo);

    struct RedefInfo(RedefInfo, String, RedefKind, NodeInfo, NodeInfo);

    enum RedefKind {
        DuplicateDef,
        DiffKindRedecl,
        ShadowedDef,
        DisagreeLinkage,
        NoLinkageOld
    }

    #[derive(Debug)]
    struct TypeMismatch(TypeMismatch, String, (NodeInfo, Type), (NodeInfo, Type));

    fn badSpecifierError(node_info: BadSpecifierError) -> BadSpecifierError {
        BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn invalidAST(node_info: InvalidASTError) -> InvalidASTError {
        InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn prevDeclMsg(old_node: Vec<String>) -> Vec<String> {
        vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
    }

    fn redefErrLabel((RedefInfo(ident, _, _, _)): String) -> String {
        __op_addadd(ident, " redefined".to_string())
    }

    fn redefErrReason(__0: String) -> String {
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

    fn redefErrorInfo(lvl: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (__op_addadd(vec![redefErrReason(info)], prevDeclMsg(old_node))))
    }

    fn redefinition(lvl: RedefError) -> RedefError {
        RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
    }

    fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    fn typeMismatchInfo((TypeMismatch(reason, (node1, _ty2), _t2)): ErrorInfo) -> ErrorInfo {
        ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
    }

}

mod Language_C_Analysis_SemRep {
    #[derive(Clone, Debug)]
    enum TagDef {
        CompDef(CompType),
        EnumDef(EnumType)
    }

    #[derive(Clone, Debug)]
    enum IdentDecl {
        Declaration(Decl),
        ObjectDef(ObjDef),
        FunctionDef(FunDef),
        EnumeratorDef(Enumerator)
    }

    struct GlobalDecls(GlobalDecls, { /* struct def */ });

    enum DeclEvent {
        TagEvent(TagDef),
        DeclEvent(IdentDecl),
        ParamEvent(ParamDecl),
        LocalEvent(IdentDecl),
        TypeDefEvent(TypeDef),
        AsmEvent(AsmBlock)
    }

    #[derive(Clone, Debug)]
    struct Decl(Decl, VarDecl, NodeInfo);

    #[derive(Clone, Debug)]
    struct ObjDef(ObjDef, VarDecl, Maybe<Initializer>, NodeInfo);

    #[derive(Clone, Debug)]
    struct FunDef(FunDef, VarDecl, Stmt, NodeInfo);

    #[derive(Clone, Debug)]
    enum ParamDecl {
        ParamDecl(VarDecl, NodeInfo),
        AbstractParamDecl(VarDecl, NodeInfo)
    }

    #[derive(Clone, Debug)]
    enum MemberDecl {
        MemberDecl(VarDecl, Maybe<Expr>, NodeInfo),
        AnonBitField(Type, Expr, NodeInfo)
    }

    #[derive(Clone, Debug)]
    struct TypeDef(TypeDef, Ident, Type, Attributes, NodeInfo);

    #[derive(Clone, Debug)]
    struct VarDecl(VarDecl, VarName, DeclAttrs, Type);

    #[derive(Clone, Debug)]
    struct DeclAttrs(DeclAttrs, Bool, Storage, Attributes);

    #[derive(Clone, Debug, Eq, Ord)]
    enum Storage {
        NoStorage,
        Auto(Register),
        Static(Linkage, ThreadLocal),
        FunLinkage(Linkage)
    }

    #[derive(Clone, Debug, Eq, Ord)]
    enum Linkage {
        NoLinkage,
        InternalLinkage,
        ExternalLinkage
    }

    #[derive(Clone, Debug)]
    enum Type {
        DirectType(TypeName, TypeQuals, Attributes),
        PtrType(Type, TypeQuals, Attributes),
        ArrayType(Type, ArraySize, TypeQuals, Attributes),
        FunctionType(FunType, Attributes),
        TypeDefType(TypeDefRef, TypeQuals, Attributes)
    }

    #[derive(Clone, Debug)]
    enum FunType {
        FunType(Type, Vec<ParamDecl>, Bool),
        FunTypeIncomplete(Type)
    }

    #[derive(Clone, Debug)]
    enum ArraySize {
        UnknownArraySize(Bool),
        ArraySize(Bool, Expr)
    }

    #[derive(Clone, Debug)]
    enum TypeName {
        TyVoid,
        TyIntegral(IntType),
        TyFloating(FloatType),
        TyComplex(FloatType),
        TyComp(CompTypeRef),
        TyEnum(EnumTypeRef),
        TyBuiltin(BuiltinType)
    }

    #[derive(Clone, Debug)]
    enum BuiltinType {
        TyVaList,
        TyAny
    }

    #[derive(Clone, Debug)]
    struct TypeDefRef(TypeDefRef, Ident, Maybe<Type>, NodeInfo);

    #[derive(Clone, Debug, Eq, Ord)]
    enum IntType {
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

    #[derive(Clone, Debug, Eq, Ord)]
    enum FloatType {
        TyFloat,
        TyDouble,
        TyLDouble
    }

    #[derive(Clone, Debug)]
    struct CompTypeRef(CompTypeRef, SUERef, CompTyKind, NodeInfo);

    #[derive(Clone, Debug)]
    struct EnumTypeRef(EnumTypeRef, SUERef, NodeInfo);

    #[derive(Clone, Debug)]
    struct CompType(CompType, SUERef, CompTyKind, Vec<MemberDecl>, Attributes, NodeInfo);

    #[derive(Clone, Debug, Eq, Ord)]
    enum CompTyKind {
        StructTag,
        UnionTag
    }

    #[derive(Clone, Debug)]
    struct EnumType(EnumType, SUERef, Vec<Enumerator>, Attributes, NodeInfo);

    #[derive(Clone, Debug)]
    struct Enumerator(Enumerator, Ident, Expr, EnumType, NodeInfo);

    #[derive(Clone, Debug)]
    struct TypeQuals(TypeQuals, { /* struct def */ });

    #[derive(Clone, Debug)]
    enum VarName {
        VarName(Ident, Maybe<AsmName>),
        NoName
    }

    #[derive(Clone, Debug)]
    struct Attr(Attr, Ident, Vec<Expr>, NodeInfo);

    fn declAttrs() -> DeclAttrs {
        ((Lambda) . getVarDecl)
    }

    fn declIdent() -> Ident {
        (identOfVarName . declName)
    }

    fn declLinkage(decl: Linkage) -> Linkage {
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

    fn declName() -> VarName {
        ((Lambda) . getVarDecl)
    }

    fn declOfDef(def: Decl) -> Decl {
        {
                let vd = || {
                    getVarDecl(def, in, Decl, vd, (nodeInfo(def)))
                };
        }
    }

    fn declStorage(d: Storage) -> Storage {
        match declAttrs(d) {
            DeclAttrs(_, st, _) => {
                st
            },
        }
    }

    fn declType() -> Type {
        ((Lambda) . getVarDecl)
    }

    fn emptyGlobalDecls() -> GlobalDecls {
        GlobalDecls(Map_empty, Map_empty, Map_empty)
    }

    fn filterGlobalDecls(decl_filter: GlobalDecls) -> GlobalDecls {
        GlobalDecls({
        gObjs: Map_filter(((decl_filter . DeclEvent)), (gObjs(gmap))),
        gTags: Map_filter(((decl_filter . TagEvent)), (gTags(gmap))),
        gTypeDefs: Map_filter(((decl_filter . TypeDefEvent)), (gTypeDefs(gmap)))
        })
    }

    fn hasLinkage(__0: Bool) -> Bool {
        match (__0) {
        Auto(_) => {
            False
        },
        Static(NoLinkage, _) => {
            False
        },
        _ => {
            True
        },
    }
    }

    fn identOfTypeDef((TypeDef(ide, _, _, _)): Ident) -> Ident {
        ide
    }

    fn identOfVarName(__0: Ident) -> Ident {
        match (__0) {
        NoName => {
            error("identOfVarName: NoName".to_string())
        },
        VarName(ident, _) => {
            ident
        },
    }
    }

    fn isExtDecl() -> Bool {
        (hasLinkage . declStorage)
    }

    fn isNoName(__0: Bool) -> Bool {
        match (__0) {
        NoName => {
            True
        },
        _ => {
            False
        },
    }
    }

    fn mergeAttributes() -> Attributes {
        (Operator("++"))
    }

    fn mergeGlobalDecls(gmap1: GlobalDecls) -> GlobalDecls {
        GlobalDecls({
        gObjs: Map_union((gObjs(gmap1)), (gObjs(gmap2))),
        gTags: Map_union((gTags(gmap1)), (gTags(gmap2))),
        gTypeDefs: Map_union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
        })
    }

    fn mergeTypeQuals((TypeQuals(c1, v1, r1)): TypeQuals) -> TypeQuals {
        TypeQuals(((c1 && c2)), ((v1 && v2)), ((r1 && r2)))
    }

    fn noAttributes() -> Attributes {
        vec![]
    }

    fn noTypeQuals() -> TypeQuals {
        TypeQuals(False, False, False)
    }

    fn objKindDescr(__0: String) -> String {
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

    fn splitIdentDecls(include_all: Map) -> Map {
        Map_foldWithKey((if(include_all, then, deal, else, deal_q)), (Map_empty, (Map_empty, Map_empty, Map_empty)))
    }

    fn typeOfCompDef((CompType(ref, tag, _, _, _)): TypeName) -> TypeName {
        TyComp((CompTypeRef(ref, tag, undefNode)))
    }

    fn typeOfEnumDef((EnumType(ref, _, _, _)): TypeName) -> TypeName {
        TyEnum((EnumTypeRef(ref, undefNode)))
    }

    fn typeOfTagDef(__0: TypeName) -> TypeName {
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

mod Language_C_Analysis_TravMonad {
    enum CLanguage {
        C89,
        C99,
        GNU89,
        GNU99
    }

    struct TravOptions(TravOptions, { /* struct def */ });

    struct TravState(TravState, { /* struct def */ });

    fn addRef(use: m) -> m {
        match (nodeInfo(use), nodeInfo(def)) {
            (NodeInfo(_, _, useName), NodeInfo(_, _, defName)) => {
                withDefTable((Lambda))
            },
            (_, _) => {
                return(())
            },
        }
    }

    fn astError(node: m) -> m {
        throwTravError(invalidAST(node, msg))
    }

    fn checkCompatibleTypes(_: Either) -> Either {
        Right(())
    }

    fn checkIdentTyRedef(__0: m) -> m {
        match (__0, __1) {
        Right(decl)(status) => {
            checkVarRedef(decl, status)
        },
        Left(tydef)(KindMismatch(old_def)) => {
            redefErr((identOfTypeDef(tydef)), LevelError, tydef, old_def, DiffKindRedecl)
        },
        Left(tydef)(Redeclared(old_def)) => {
            redefErr((identOfTypeDef(tydef)), LevelError, tydef, old_def, DuplicateDef)
        },
        Left(_tydef)(_) => {
            return(())
        },
    }
    }

    fn checkRedef(subject: m) -> m {
        match redecl_status {
            NewDecl => {
                return(())
            },
            Redeclared(old_def) => {
                throwTravError(redefinition(LevelError, subject, DuplicateDef, (nodeInfo(new_decl)), (nodeInfo(old_def))))
            },
            KindMismatch(old_def) => {
                throwTravError(redefinition(LevelError, subject, DiffKindRedecl, (nodeInfo(new_decl)), (nodeInfo(old_def))))
            },
            Shadowed(_old_def) => {
                return(())
            },
            KeepDef(_old_def) => {
                return(())
            },
        }
    }

    fn checkVarRedef(def: m) -> m {
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
                return(())
            },
        }
    }

    fn concatMapM(f: m) -> m {
        (liftM(concat) . mapM(f))
    }

    fn createSUERef(_node_info: Maybe) -> Maybe {
        return(NamedRef(ident))
    }

    fn enterBlockScope() -> m {
        updDefTable((ST_enterBlockScope))
    }

    fn enterDecl(decl: m) -> m {
        /* do */ {
            {
                    let def = || {
                        Declaration(decl)
                    };
            };
            let redecl = withDefTable(defineScopedIdentWhen(cond, (declIdent(def)), def));
            checkVarRedef(def, redecl);
            return(def)
        }
    }

    fn enterFunctionScope() -> m {
        updDefTable((ST_enterFunctionScope))
    }

    fn enterPrototypeScope() -> m {
        updDefTable((ST_enterBlockScope))
    }

    fn generateName() -> Trav {
        __op_bind(get, Lambda)
    }

    fn get() -> Trav {
        Trav((Lambda((s, s))))
    }

    fn getUserState() -> Trav {
        liftM(userState, get)
    }

    fn gets(f: Trav) -> Trav {
        Trav((Lambda((f(s), s))))
    }

    fn hadHardErrors() -> Bool {
        ((not . (null . filter(isHardError))))
    }

    fn handleAsmBlock(asm: m) -> m {
        handleDecl((AsmEvent(asm)))
    }

    fn handleEnumeratorDef(enumerator: m) -> m {
        /* do */ {
            {
                    let ident = || {
                        declIdent(enumerator)
                    };
            };
            let redecl = withDefTable(defineScopedIdent(ident, (EnumeratorDef(enumerator))));
            checkRedef((show(ident)), ident, redecl);
            return(())
        }
    }

    fn handleFunDef(ident: m) -> m {
        /* do */ {
            {
                    let def = || {
                        FunctionDef(fun_def)
                    };
            };
            let redecl = withDefTable(defineScopedIdentWhen(isDeclaration, ident, def));
            checkVarRedef(def, redecl);
            handleDecl((DeclEvent(def)))
        }
    }

    fn handleObjectDef(local: m) -> m {
        /* do */ {
            {
                    let def = || {
                        ObjectDef(obj_def)
                    };
            };
            let redecl = withDefTable(defineScopedIdentWhen((Lambda(def, old)), ident, def));
            checkVarRedef(def, redecl);
            handleDecl(((if(local, then, LocalEvent, else, DeclEvent))(def)));

        }
    }

    fn handleParamDecl(__0: m) -> m {
        match (__0, __1, __2) {
        pd(<todo>, AbstractParamDecl(_, _)) => {
            handleDecl((ParamEvent(pd)))
        },
        pd(<todo>, ParamDecl(vardecl, node)) => {
            /* do */ {
                {
                        let def = || {
                            ObjectDef((ObjDef(vardecl, Nothing, node)))
                        };
                };
                let redecl = withDefTable(defineScopedIdent((declIdent(def)), def));
                checkVarRedef(def, redecl);
                handleDecl((ParamEvent(pd)))
            }
        },
    }
    }

    fn handleTagDecl(decl: m) -> m {
        /* do */ {
            let redecl = withDefTable(declareTag((sueRef(decl)), decl));
            checkRedef((show(sueRef(decl))), decl, redecl)
        }
    }

    fn handleTagDef(def: m) -> m {
        /* do */ {
            let redecl = withDefTable(defineTag((sueRef(def)), def));
            checkRedef((show(sueRef(def))), def, redecl);
            handleDecl((TagEvent(def)))
        }
    }

    fn handleTravError(a: m) -> m {
        catchTravError(liftM(Just, a), (>>(Lambda(e), return(Nothing))))
    }

    fn handleTypeDef(typeDef: m) -> m {
        /* do */ {
            let redecl = withDefTable(defineTypeDef(ident, typeDef));
            checkRedef((show(ident)), typeDef, redecl);
            handleDecl((TypeDefEvent(typeDef)));
            return(())
        }
    }

    fn handleVarDecl(is_local: m) -> m {
        /* do */ {
            let def = enterDecl(decl, (const(False)));
            handleDecl(((if(is_local, then, LocalEvent, else, DeclEvent))(def)))
        }
    }

    fn initTravState(userst: TravState) -> TravState {
        TravState({
        symbolTable: emptyDefTable,
        rerrors: RList_empty,
        nameGenerator: newNameSupply,
        doHandleExtDecl: const((return(()))),
        userState: userst,
        options: TravOptions({
                language: C99
                })
        })
    }

    fn isDeclaration(__0: Bool) -> Bool {
        match (__0) {
        Declaration(_) => {
            True
        },
        _ => {
            False
        },
    }
    }

    fn leaveBlockScope() -> m {
        updDefTable((ST_leaveBlockScope))
    }

    fn leaveFunctionScope() -> m {
        updDefTable((ST_leaveFunctionScope))
    }

    fn leavePrototypeScope() -> m {
        updDefTable((ST_leaveBlockScope))
    }

    fn lookupObject(ident: m) -> m {
        /* do */ {
            let old_decl = liftM((lookupIdent(ident)), getDefTable);
            mapMaybeM(old_decl)(Lambda)
        }
    }

    fn lookupTypeDef(ident: m) -> m {
        __op_bind(getDefTable, Lambda)
    }

    fn mapMaybeM(m: m) -> m {
        maybe((return(Nothing)), ((liftM(Just) . f)), m)
    }

    fn mapSndM(f: m) -> m {
        liftM((Dummy(a)), (f(b)))
    }

    fn maybeM(m: m) -> m {
        maybe((return(())), f, m)
    }

    fn mismatchErr(ctx: String) -> String {
        __op_addadd(ctx, __op_addadd(": Expected ".to_string(), __op_addadd(expect, __op_addadd(", but found: ".to_string(), found))))
    }

    fn modify(f: Trav) -> Trav {
        Trav((Lambda(((), f(s)))))
    }

    fn modifyOptions(f: Trav) -> Trav {
        modify(Lambda({
            options: f((options(ts)))
            }))
    }

    fn modifyUserState(f: Trav) -> Trav {
        modify(Lambda({
            userState: f((userState(ts)))
            }))
    }

    fn put(s: TravState) -> TravState {
        Trav((Lambda(((), s))))
    }

    fn redefErr(name: m) -> m {
        throwTravError(redefinition(lvl, (show(name)), kind, (nodeInfo(new)), (nodeInfo(old))))
    }

    fn runTrav(state: forall) -> forall {
        match unTrav(action, (initTravState(state))) {
            Left(trav_err) => {
                Left(vec![trav_err])
            },
            Right, (v, ts) => if hadHardErrors((travErrors(ts))) { Left((travErrors(ts))) }
otherwise { Right((v, ts)) },
        }
    }

    fn runTrav_(t: Trav) -> Trav {
        (fmap(fst) . runTrav(())(/* do */ {
                    let r = t;
                    let es = getErrors;
                    return((r, es))
                }))
    }

    fn throwOnLeft(__0: Either) -> Either {
        match (__0) {
        Left(err) => {
            throwTravError(err)
        },
        Right(v) => {
            return(v)
        },
    }
    }

    fn travErrors() -> TravState {
        (RList_reverse . rerrors)
    }

    fn updDefTable(f: m) -> m {
        withDefTable((Lambda))
    }

    fn warn(err: m) -> m {
        recordError((changeErrorLevel(err, LevelWarn)))
    }

    fn withExtDeclHandler(action: Trav) -> Trav {
        /* do */ {
            modify(Lambda({
                doHandleExtDecl: handler
                }));
            action
        }
    }

}

mod Language_C_Analysis_TypeCheck {
    fn assignCompatible(__0: Either) -> Either {
        match (__0, __1, __2) {
        CAssignOp(t1, t2) => {
            match (canonicalType(t1), canonicalType(t2)) {
                (DirectType(TyBuiltin(TyAny), _, _), _) => {
                    return(())
                },
                (_, DirectType(TyBuiltin(TyAny), _, _)) => {
                    return(())
                },
                (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) => if isPointerType(t2_q) { return(()) },
                (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1_q) { return(()) },
                (PtrType(_, _, _), t2_q) => if isIntegralType(t2_q) { return(()) },
                (t1_q, t2_q) => if (isPointerType(t1_q) && isPointerType(t2_q)) { /* do */ {
                    compatible((baseType(t1_q)), (baseType(t2_q)))
                } },
                (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) => if (sueRef(c1) == sueRef(c2)) { return(()) }
otherwise { fail(__op_addadd("incompatible compound types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) },
                (DirectType(TyBuiltin(TyVaList), _, _), DirectType(TyBuiltin(TyVaList), _, _)) => {
                    return(())
                },
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) => if isJust((arithmeticConversion(tn1, tn2))) { return(()) }
otherwise { fail(__op_addadd("incompatible direct types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) },
                (t1_q, t2_q) => {
                    compatible(t1_q, t2_q)
                },
            }
        },
        op(t1, t2) => {
            >>(binopType((assignBinop(op)), t1, t2), return(()))
        },
    }
    }

    fn assignCompatible_q(ni: MonadCError) -> MonadCError {
        typeErrorOnLeft(ni, (assignCompatible(op, t1, t2)))
    }

    fn binopType(op: Either) -> Either {
        match (op, canonicalType(t1), canonicalType(t2)) {
            (_, t1_q, t2_q) => if isLogicOp(op) { >>(checkScalar(t1_q), >>(checkScalar(t2_q), return(boolType))) }
isCmpOp(op) { match (t1_q, t2_q) {
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) => {
                    match arithmeticConversion(tn1, tn2) {
                        Just(_) => {
                            return(boolType)
                        },
                        Nothing => {
                            fail("incompatible arithmetic types in comparison".to_string())
                        },
                    }
                },
                (PtrType(DirectType(TyVoid, _, _), _, _), _) => if isPointerType(t2_q) { return(boolType) },
                (_, PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1_q) { return(boolType) },
                (_, _) => if (isPointerType(t1_q) && isIntegralType(t2_q)) { return(boolType) }
(isIntegralType(t1_q) && isPointerType(t2_q)) { return(boolType) }
(isPointerType(t1_q) && isPointerType(t2_q)) { >>(compatible(t1_q, t2_q), return(boolType)) },
                (_, _) => {
                    fail("incompatible types in comparison".to_string())
                },
            } },
            (CSubOp, ArrayType(t1_q, _, _, _), ArrayType(t2_q, _, _, _)) => {
                >>(compatible(t1_q, t2_q), return(ptrDiffType))
            },
            (CSubOp, ArrayType(t1_q, _, _, _), PtrType(t2_q, _, _)) => {
                >>(compatible(t1_q, t2_q), return(ptrDiffType))
            },
            (CSubOp, PtrType(t1_q, _, _), ArrayType(t2_q, _, _, _)) => {
                >>(compatible(t1_q, t2_q), return(ptrDiffType))
            },
            (CSubOp, PtrType(t1_q, _, _), PtrType(t2_q, _, _)) => {
                >>(compatible(t1_q, t2_q), return(ptrDiffType))
            },
            (_, PtrType(_, _, _), t2_q) => if (isPtrOp(op) && isIntegralType(t2_q)) { return(t1) }
otherwise { fail(__op_addadd("invalid pointer operation: ".to_string(), render((pretty(op))))) },
            (CAddOp, t1_q, PtrType(_, _, _)) => if isIntegralType(t1_q) { return(t2) },
            (_, ArrayType(_, _, _, _), t2_q) => if (isPtrOp(op) && isIntegralType(t2_q)) { return(t1) }
otherwise { fail(__op_addadd("invalid pointer operation: ".to_string(), render((pretty(op))))) },
            (CAddOp, t1_q, ArrayType(_, _, _, _)) => if isIntegralType(t1_q) { return(t2) },
            (_, DirectType(tn1, q1, a1), DirectType(tn2, q2, a2)) => {
                /* do */ {
                    when((isBitOp(op)), (>>(checkIntegral(t1), checkIntegral(t2))));
                    match arithmeticConversion(tn1, tn2) {
                        Just(tn) => {
                            return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2))))
                        },
                        Nothing => {
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

    fn binopType_q(ni: MonadCError) -> MonadCError {
        typeErrorOnLeft(ni, (binopType(op, t1, t2)))
    }

    fn castCompatible(t1: Either) -> Either {
        match (canonicalType(t1), canonicalType(t2)) {
            (DirectType(TyVoid, _, _), _) => {
                return(())
            },
            (_, _) => {
                >>(checkScalar(t1), checkScalar(t2))
            },
        }
    }

    fn checkIntegral_q(ni: MonadCError) -> MonadCError {
        (typeErrorOnLeft(ni) . checkIntegral)
    }

    fn checkScalar(t: Either) -> Either {
        match canonicalType(t) {
            DirectType(_, _, _) => {
                return(())
            },
            PtrType(_, _, _) => {
                return(())
            },
            ArrayType(_, _, _, _) => {
                return(())
            },
            t_q => {
                fail(__op_addadd("expected scalar type, got: ".to_string(), __op_addadd(pType(t), __op_addadd(" (".to_string(), __op_addadd(pType(t_q), ")".to_string())))))
            },
        }
    }

    fn checkScalar_q(ni: MonadCError) -> MonadCError {
        (typeErrorOnLeft(ni) . checkScalar)
    }

    fn compatible(t1: Either) -> Either {
        >>(compositeType(t1, t2), return(()))
    }

    fn compositeDeclAttrs((DeclAttrs(inl, stor, attrs1)): DeclAttrs) -> DeclAttrs {
        DeclAttrs(inl, stor, (mergeAttrs(attrs1, attrs2)))
    }

    fn compositeParamDecl(__0: Either) -> Either {
        match (__0, __1) {
        ParamDecl(vd1, ni1)(ParamDecl(vd2, _)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni1)
        },
        AbstractParamDecl(vd1, _)(ParamDecl(vd2, ni2)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni2)
        },
        ParamDecl(vd1, ni1)(AbstractParamDecl(vd2, _)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni1)
        },
        AbstractParamDecl(vd1, ni1)(AbstractParamDecl(vd2, _)) => {
            compositeParamDecl_q(AbstractParamDecl, vd1, vd2, ni1)
        },
    }
    }

    fn compositeParamDecl_q(f: Either) -> Either {
        /* do */ {
            let vd = compositeVarDecl((VarDecl(n1, attrs1, t1_q)), (VarDecl(n2, attrs2, t2_q)));
            return(f(vd, dni))
        }
    }

    fn compositeSize(__0: Either) -> Either {
        match (__0, __1) {
        UnknownArraySize(_)(s2) => {
            return(s2)
        },
        s1(UnknownArraySize(_)) => {
            return(s1)
        },
    }
    }

    fn compositeType(__0: Either) -> Either {
        match (__0, __1) {
        t1(DirectType(TyBuiltin(TyAny), _, _)) => {
            return(t1)
        },
        DirectType(TyBuiltin(TyAny), _, _)(t2) => {
            return(t2)
        },
        t1(<todo>, DirectType(tn1, q1, a1), t2, <todo>, DirectType(tn2, q2, a2)) => {
            /* do */ {
                let tn = match (tn1, tn2) {
                        (TyVoid, TyVoid) => {
                            return(TyVoid)
                        },
                        (TyIntegral(_), TyEnum(_)) => {
                            return(tn1)
                        },
                        (TyEnum(_), TyIntegral(_)) => {
                            return(tn2)
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
                                return(tn1)
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
        PtrType(t1, q1, a1)(PtrType(DirectType(TyVoid, _, _), q2, _)) => {
            return(PtrType(t1, (mergeTypeQuals(q1, q2)), a1))
        },
        PtrType(DirectType(TyVoid, _, _), q1, _)(PtrType(t2, q2, a2)) => {
            return(PtrType(t2, (mergeTypeQuals(q1, q2)), a2))
        },
        ArrayType(t1, s1, q1, a1)(ArrayType(t2, s2, q2, a2)) => {
            /* do */ {
                let t = compositeType(t1, t2);
                let s = compositeSize(s1, s2);
                {
                        let quals = || {
                            mergeTypeQuals(q1, q2)
                        };
;
                        let attrs = || {
                            mergeAttrs(a1, a2)
                        };
                };
                return((ArrayType(t, s, quals, attrs)))
            }
        },
        TypeDefType(tdr1, q1, a1)(TypeDefType(tdr2, q2, a2)) => {
            match (tdr1, tdr2) {
                (TypeDefRef(i1, Nothing, _), TypeDefRef(i2, _, _)) => {
                    doTypeDef(i1, i2, tdr1)
                },
                (TypeDefRef(i1, _, _), TypeDefRef(i2, Nothing, _)) => {
                    doTypeDef(i1, i2, tdr2)
                },
                (TypeDefRef(_, Just(t1), _), TypeDefRef(_, Just(t2), _)) => {
                    compositeType(t1, t2)
                },
            }
        },
        FunctionType(ft1, attrs1)(FunctionType(ft2, attrs2)) => {
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
                        return((FunctionType((FunTypeIncomplete(rt)), (mergeAttrs(attrs1, attrs2)))))
                    }
                },
            }
        },
        t1(t2) => {
            fail(__op_addadd("incompatible types: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2)))))
        },
    }
    }

    fn compositeVarDecl((VarDecl(n1, attrs1, t1)): Either) -> Either {
        /* do */ {
            let t = compositeType(t1, t2);
            return((VarDecl(n1, (compositeDeclAttrs(attrs1, attrs2)), t)))
        }
    }

    fn conditionalType(t1: Either) -> Either {
        match (canonicalType(t1), canonicalType(t2)) {
            (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) => if isPointerType(t2_q) { return(t2) },
            (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1_q) { return(t1) },
            (ArrayType(t1_q, _, q1, a1), ArrayType(t2_q, _, q2, a2)) => {
                /* do */ {
                    let t = compositeType(t1_q, t2_q);
                    return(ArrayType(t, (UnknownArraySize(False)), (mergeTypeQuals(q1, q2)), (mergeAttrs(a1, a2))))
                }
            },
            (t1_q(@, DirectType(tn1, q1, a1)), t2_q(@, DirectType(tn2, q2, a2))) => {
                match arithmeticConversion(tn1, tn2) {
                    Just(tn) => {
                        return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2))))
                    },
                    Nothing => {
                        compositeType(t1_q, t2_q)
                    },
                }
            },
            (t1_q, t2_q) => {
                compositeType(t1_q, t2_q)
            },
        }
    }

    fn conditionalType_q(ni: MonadCError) -> MonadCError {
        typeErrorOnLeft(ni)(conditionalType(t1, t2))
    }

    fn constType(__0: m) -> m {
        match (__0) {
        CIntConst(CInteger(_, _, flags), _) => {
            return(DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes))
        },
        CCharConst(CChar(_, True), _) => {
            return(DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes))
        },
        CCharConst(CChar(_, False), _) => {
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
                        let ni_q = || {
                            mkNodeInfo((posOf(ni)), n)
                        };
;
                        let arraySize = || {
                            ArraySize(True, (CConst((CIntConst((cInteger((toInteger((length(chars)))))), ni_q)))))
                        };
                };
                return(ArrayType((DirectType((TyIntegral(charType)), noTypeQuals, noAttributes)), arraySize, noTypeQuals, vec![]))
            }
        },
    }
    }

    fn deepTypeAttrs(__0: m) -> m {
        match (__0) {
        DirectType(TyComp(CompTypeRef(sue, _, ni)), _, attrs) => {
            liftM((attrs(Operator("++"))), sueAttrs(ni, sue))
        },
        DirectType(TyEnum(EnumTypeRef(sue, ni)), _, attrs) => {
            liftM((attrs(Operator("++"))), sueAttrs(ni, sue))
        },
        DirectType(_, _, attrs) => {
            return(attrs)
        },
        PtrType(t, _, attrs) => {
            liftM((attrs(Operator("++"))), deepTypeAttrs(t))
        },
        ArrayType(t, _, _, attrs) => {
            liftM((attrs(Operator("++"))), deepTypeAttrs(t))
        },
        FunctionType(FunType(t, _, _), attrs) => {
            liftM((attrs(Operator("++"))), deepTypeAttrs(t))
        },
        FunctionType(FunTypeIncomplete(t), attrs) => {
            liftM((attrs(Operator("++"))), deepTypeAttrs(t))
        },
        TypeDefType(TypeDefRef(i, _, ni), _, attrs) => {
            liftM((attrs(Operator("++"))), typeDefAttrs(ni, i))
        },
    }
    }

    fn derefType(__0: Either) -> Either {
        match (__0) {
        PtrType(t, _, _) => {
            return(t)
        },
        ArrayType(t, _, _, _) => {
            return(t)
        },
        t => {
            match canonicalType(t) {
                PtrType(t_q, _, _) => {
                    return(t_q)
                },
                ArrayType(t_q, _, _, _) => {
                    return(t_q)
                },
                _ => {
                    fail(__op_addadd("dereferencing non-pointer: ".to_string(), pType(t)))
                },
            }
        },
    }
    }

    fn expandAnonymous(__0: m) -> m {
        match (__0, __1) {
        ni(NoName(DirectType(TyComp(ctr), _, _))) => {
            __op_bind(lookupSUE(ni, (sueRef(ctr))), tagMembers(ni))
        },
        _(NoName(_)) => {
            return(vec![])
        },
        _(VarName(n, _)(t)) => {
            return(vec![(n, t)])
        },
    }
    }

    fn fieldType(ni: m) -> m {
        match canonicalType(t) {
            DirectType(TyComp(ctr), _, _) => {
                /* do */ {
                    let td = lookupSUE(ni, (sueRef(ctr)));
                    let ms = tagMembers(ni, td);
                    match lookup(m, ms) {
                        Just(ft) => {
                            return(ft)
                        },
                        Nothing => {
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

    fn lookupSUE(ni: m) -> m {
        /* do */ {
            let dt = getDefTable;
            match lookupTag(sue, dt) {
                Just(Right(td)) => {
                    return(td)
                },
                _ => {
                    typeError(ni)(__op_addadd("unknown composite type: ".to_string(), ((render . pretty))(sue)))
                },
            }
        }
    }

    fn mergeAttrs() -> Attributes {
        (Operator("++"))
    }

    fn notFound(i: Either) -> Either {
        fail(__op_addadd("not found: ".to_string(), identToString(i)))
    }

    fn pType() -> String {
        (render . pretty)
    }

    fn sizeEqual(__0: Bool) -> Bool {
        match (__0, __1) {
        CConst(CIntConst(i1, _))(CConst(CIntConst(i2, _))) => {
            (i1 == i2)
        },
        e1(e2) => {
            (nodeInfo(e1) == nodeInfo(e2))
        },
    }
    }

    fn sueAttrs(ni: m) -> m {
        /* do */ {
            let dt = getDefTable;
            match lookupTag(sue, dt) {
                Nothing => {
                    astError(ni)(__op_addadd("SUE not found: ".to_string(), render((pretty(sue)))))
                },
                Just(Left(_)) => {
                    return(vec![])
                },
                Just(Right(CompDef(CompType(_, _, _, attrs, _)))) => {
                    return(attrs)
                },
                Just(Right(EnumDef(EnumType(_, _, attrs, _)))) => {
                    return(attrs)
                },
            }
        }
    }

    fn tagMembers(ni: m) -> m {
        match td {
            CompDef(CompType(_, _, ms, _, _)) => {
                getMembers(ms)
            },
            EnumDef(EnumType(_, es, _, _)) => {
                getMembers(es)
            },
        }
    }

    fn typeDefAttrs(ni: m) -> m {
        /* do */ {
            let dt = getDefTable;
            match lookupIdent(i, dt) {
                Nothing => {
                    astError(ni)(__op_addadd("can\'t find typedef name: ".to_string(), identToString(i)))
                },
                Just(Left(TypeDef(_, t, attrs, _))) => {
                    liftM((attrs(Operator("++"))), deepTypeAttrs(t))
                },
                Just(Right(_)) => {
                    astError(ni)(__op_addadd("not a typedef name: ".to_string(), identToString(i)))
                },
            }
        }
    }

    fn typeError() -> MonadCError {
        astError
    }

    fn typeErrorOnLeft(__0: Either) -> Either {
        match (__0, __1) {
        ni(Left(err)) => {
            typeError(ni, err)
        },
        _(Right(v)) => {
            return(v)
        },
    }
    }

    fn varAddrType(d: Either) -> Either {
        /* do */ {
            match declStorage(d) {
                Auto(True) => {
                    fail("address of register variable".to_string())
                },
                _ => {
                    return(())
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

mod Language_C_Analysis_TypeConversions {
    fn arithmeticConversion(__0: Maybe) -> Maybe {
        match (__0, __1) {
        TyComplex(t1)(TyComplex(t2)) => {
            Just(TyComplex(floatConversion(t1, t2)))
        },
        TyComplex(t1)(TyFloating(t2)) => {
            Just(TyComplex(floatConversion(t1, t2)))
        },
        TyFloating(t1)(TyComplex(t2)) => {
            Just(TyComplex(floatConversion(t1, t2)))
        },
        t1(<todo>, TyComplex(_), TyIntegral(_)) => {
            Just(t1)
        },
        TyIntegral(_)(t2, <todo>, TyComplex(_)) => {
            Just(t2)
        },
        TyFloating(t1)(TyFloating(t2)) => {
            Just(TyFloating(floatConversion(t1, t2)))
        },
        t1(<todo>, TyFloating(_), TyIntegral(_)) => {
            Just(t1)
        },
        TyIntegral(_)(t2, <todo>, TyFloating(_)) => {
            Just(t2)
        },
        TyIntegral(t1)(TyIntegral(t2)) => {
            Just(TyIntegral(intConversion(t1, t2)))
        },
        TyEnum(_)(TyEnum(_)) => {
            Just(TyIntegral(TyInt))
        },
        TyEnum(_)(t2) => {
            Just(t2)
        },
        t1(TyEnum(_)) => {
            Just(t1)
        },
        _(_) => {
            Nothing
        },
    }
    }

    fn floatConversion() -> FloatType {
        max
    }

    fn intConversion(t1: IntType) -> IntType {
        max(TyInt, (max(t1, t2)))
    }

}

mod Language_C_Analysis_TypeUtils {
    fn baseType(__0: Type) -> Type {
        match (__0) {
        PtrType(t, _, _) => {
            t
        },
        ArrayType(t, _, _, _) => {
            t
        },
        _ => {
            error("base of non-pointer type".to_string())
        },
    }
    }

    fn boolType() -> Type {
        integral(TyInt)
    }

    fn canonicalType(t: Type) -> Type {
        match deepDerefTypeDef(t) {
            FunctionType(ft, attrs) => {
                simplePtr((FunctionType(ft, attrs)))
            },
            t_q => {
                t_q
            },
        }
    }

    fn charPtr() -> Type {
        simplePtr((integral(TyChar)))
    }

    fn constCharPtr() -> Type {
        constPtr((integral(TyChar)))
    }

    fn constPtr(t: Type) -> Type {
        PtrType(t, (TypeQuals(True, False, False)), vec![])
    }

    fn constVoidPtr() -> Type {
        constPtr(voidType)
    }

    fn deepDerefTypeDef(__0: Type) -> Type {
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
        TypeDefType(TypeDefRef(_, Just(t), _), q, a) => {
            ((typeAttrsUpd((mergeAttributes(a))) . typeQualsUpd((mergeTypeQuals(q)))))((deepDerefTypeDef(t)))
        },
        t => {
            t
        },
    }
    }

    fn derefTypeDef(__0: Type) -> Type {
        match (__0) {
        TypeDefType(TypeDefRef(_, Just(t), _), q, a) => {
            ((typeAttrsUpd((mergeAttributes(a))) . typeQualsUpd((mergeTypeQuals(q)))))((derefTypeDef(t)))
        },
        ty => {
            ty
        },
    }
    }

    fn floating(ty: Type) -> Type {
        DirectType((TyFloating(ty)), noTypeQuals, noAttributes)
    }

    fn integral(ty: Type) -> Type {
        DirectType((TyIntegral(ty)), noTypeQuals, noAttributes)
    }

    fn isFloatingType(__0: Bool) -> Bool {
        match (__0) {
        DirectType(TyFloating(_), _, _) => {
            True
        },
        _ => {
            False
        },
    }
    }

    fn isFunctionType(ty: Bool) -> Bool {
        match ty {
            TypeDefType(TypeDefRef(_, Just(actual_ty), _), _, _) => {
                isFunctionType(actual_ty)
            },
            TypeDefType(_, _, _) => {
                error("isFunctionType: unresolved typeDef".to_string())
            },
            FunctionType(_, _) => {
                True
            },
            _ => {
                False
            },
        }
    }

    fn isIntegralType(__0: Bool) -> Bool {
        match (__0) {
        DirectType(TyIntegral(_), _, _) => {
            True
        },
        DirectType(TyEnum(_), _, _) => {
            True
        },
        _ => {
            False
        },
    }
    }

    fn isPointerType(__0: Bool) -> Bool {
        match (__0) {
        PtrType(_, _, _) => {
            True
        },
        ArrayType(_, _, _, _) => {
            True
        },
        _ => {
            False
        },
    }
    }

    fn isScalarType(t: Bool) -> Bool {
        ||(isIntegralType(t), ||(isPointerType(t), isFloatingType(t)))
    }

    fn ptrDiffType() -> Type {
        integral(TyInt)
    }

    fn simplePtr(t: Type) -> Type {
        PtrType(t, noTypeQuals, vec![])
    }

    fn size_tType() -> Type {
        integral(TyInt)
    }

    fn stringType() -> Type {
        ArrayType((DirectType((TyIntegral(TyChar)), (TypeQuals(True, False, False)), noAttributes)), (UnknownArraySize(False)), noTypeQuals, vec![])
    }

    fn testFlags(flags: Enum) -> Enum {
        and(map(((flip(testFlag))(fi)), flags))
    }

    fn typeAttrs(__0: Attributes) -> Attributes {
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
        TypeDefType(TypeDefRef(_, Nothing, _), _, a) => {
            a
        },
        TypeDefType(TypeDefRef(_, Just(t), _), _, a) => {
            mergeAttributes(a, (typeAttrs(t)))
        },
    }
    }

    fn typeAttrsUpd(f: Type) -> Type {
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

    fn typeQuals(__0: TypeQuals) -> TypeQuals {
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
        TypeDefType(TypeDefRef(_, Nothing, _), q, _) => {
            q
        },
        TypeDefType(TypeDefRef(_, Just(t), _), q, _) => {
            mergeTypeQuals(q, (typeQuals(t)))
        },
    }
    }

    fn typeQualsUpd(f: Type) -> Type {
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

    fn valistType() -> Type {
        DirectType((TyBuiltin(TyVaList)), noTypeQuals, noAttributes)
    }

    fn voidPtr() -> Type {
        simplePtr(voidType)
    }

    fn voidType() -> Type {
        DirectType(TyVoid, noTypeQuals, noAttributes)
    }

}

mod Language_C_Analysis {

}

mod Language_C_Data_Error {
    #[derive(Eq, Ord)]
    enum ErrorLevel {
        LevelWarn,
        LevelError,
        LevelFatal
    }

    #[derive(Debug)]
    struct ErrorInfo(ErrorInfo, ErrorLevel, Position, Vec<String>);

    #[derive(Debug)]
    struct CError(forall, err., CError, err);

    #[derive(Debug)]
    struct UnsupportedFeature(UnsupportedFeature, String, Position);

    fn errorLevel() -> ErrorLevel {
        ((Lambda) . errorInfo)
    }

    fn errorMsgs() -> Vec<String> {
        ((Lambda) . errorInfo)
    }

    fn errorPos() -> Position {
        ((Lambda) . errorInfo)
    }

    fn indent() -> String {
        "  ".to_string()
    }

    fn indentLines() -> String {
        (unlines . (map((indent(Operator("++")))) . lines))
    }

    fn internalErr(msg: a) -> a {
        error((__op_addadd(internalErrPrefix, __op_addadd("\n".to_string(), __op_addadd(indentLines(msg), "\n".to_string())))))
    }

    fn internalErrPrefix() -> String {
        unlines(vec!["Language.C : Internal Error".to_string(), __op_addadd("This is propably a bug, and should be reported at ".to_string(), "http://www.sivity.net/projects/language.c/newticket".to_string())])
    }

    fn isHardError() -> Bool {
        ((Operator(">")(LevelWarn)) . errorLevel)
    }

    fn mkErrorInfo(lvl: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (lines(msg)))
    }

    fn showError(short_msg: String) -> String {
        (showErrorInfo(short_msg) . errorInfo)
    }

    fn showErrorInfo(short_msg: String) -> String {
        __op_addadd(header, showMsgLines((if(null, short_msg, then, msgs, else, short_msg:msgs))))
    }

    fn unsupportedFeature(msg: UnsupportedFeature) -> UnsupportedFeature {
        UnsupportedFeature(msg, (posOf(a)))
    }

    fn unsupportedFeature_(msg: UnsupportedFeature) -> UnsupportedFeature {
        UnsupportedFeature(msg, internalPos)
    }

    fn userErr(msg: UserError) -> UserError {
        UserError((ErrorInfo(LevelError, internalPos, (lines(msg)))))
    }

}

mod Language_C_Data_Ident {
    #[derive(Clone, Debug, Eq, Ord)]
    enum SUERef {
        AnonymousRef(Name),
        NamedRef(Ident)
    }

    #[derive(Clone, Debug)]
    struct Ident(Ident, String, isize, NodeInfo);

    fn bits14() -> isize {
        ^(2, (14))
    }

    fn bits21() -> isize {
        ^(2, (21))
    }

    fn bits28() -> isize {
        ^(2, (28))
    }

    fn bits7() -> isize {
        ^(2, (7))
    }

    fn builtinIdent(s: Ident) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
    }

    fn dumpIdent(ide: String) -> String {
        __op_addadd(identToString(ide), __op_addadd(" at ".to_string(), show((nodeInfo(ide)))))
    }

    fn identToString((Ident(s, _, _)): String) -> String {
        s
    }

    fn internalIdent(s: Ident) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
    }

    fn internalIdentAt(pos: Ident) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
    }

    fn isAnonymousRef(__0: Bool) -> Bool {
        match (__0) {
        AnonymousRef(_) => {
            True
        },
        _ => {
            False
        },
    }
    }

    fn isInternalIdent((Ident(_, _, nodeinfo)): Bool) -> Bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    fn mkIdent(pos: Ident) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo_q(pos, (pos, length(s)), name)))
    }

    fn quad(__0: isize) -> isize {
        match (__0) {
        c1:c2:c3:c4:s => {
            +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28)))
        },
        c1:c2:c3:([]) => {
            *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))
        },
        c1:c2:([]) => {
            *(ord(c2), +(bits7, ord(c1)))
        },
        c1:([]) => {
            ord(c1)
        },
        [] => {
            0
        },
    }
    }

}

mod Language_C_Data_InputStream {
