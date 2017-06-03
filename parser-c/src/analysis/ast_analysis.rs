// Original file: "AstAnalysis.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::SemError;
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::TravMonad;
// use Language::C::Analysis::ConstEval;
// use Language::C::Analysis::Debug;
// use Language::C::Analysis::DefTable;
// use DefTable;
// use Language::C::Analysis::DeclAnalysis;
// use Language::C::Analysis::TypeUtils;
// use Language::C::Analysis::TypeCheck;
// use Language::C::Analysis::TypeConversions;
// use Language::C::Data;
// use Language::C::Pretty;
// use Language::C::Syntax::AST;
// use Language::C::Syntax::Constants;
// use Language::C::Syntax::Ops;
// use Language::C::Syntax::Utils;
// use Text::PrettyPrint::HughesPJ;
// use Control::Monad;
// use Prelude;
// use Data::Either;
// use rights;
// use Data::Foldable;
// use foldrM;
// use Data::List;
// use Data::Map;
// use Data::Maybe;

pub enum StmtCtx {
    FunCtx(VarDecl),
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

pub fn analyseAST(CTranslUnit(decls, _file_node): CTranslUnit) -> m<GlobalDecls> {
    /*do*/ {
        mapRecoverM_(analyseExt, decls);
        __op_bind(getDefTable, |dt| { when((not((inFileScope(dt)))))(__error!("Internal Error: Not in filescope after analysis".to_string())) });
        liftM(globalDefs, getDefTable)
    }
}

pub fn analyseDecl(is_local: bool, decl: CDecl, __OP__: m<()>) -> m<()> {
    /* Expr::Error */ Error
}

pub fn analyseExt(_0: CExtDecl) -> m<()> {
    match (_0) {
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

pub fn analyseFunDef(CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info): CFunDef) -> m<()> {
    /*do*/ {
        let var_decl_info = analyseVarDecl_q(true, declspecs, declr, oldstyle_decls, None);

        let VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node) = var_decl_info;

        when((isNoName(name)))(astError(node_info, "NoName in analyseFunDef".to_string()));
        let ident = identOfVarName(name);

        let ty_q = improveFunDefType(ty);

        let fun_storage = computeFunDefStorage(ident, storage_spec);

        let var_decl = VarDecl(name, (DeclAttrs(is_inline, fun_storage, attrs)), ty_q);

        handleVarDecl(false, (Decl(var_decl, node_info)));
        let stmt_q = analyseFunctionBody(node_info, var_decl, stmt);

        handleFunDef(ident, (FunDef(var_decl, stmt_q, node_info)))
    }
}

pub fn analyseFunctionBody(_0: NodeInfo, _1: VarDecl, _2: CStat, _3: m<Stmt>) -> m<Stmt> {
    match (_0, _1, _2, _3, _4) {
        (node_info, decl, s, __OP__, CCompound(localLabels, items, _)) => {
            /*do*/ {
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
    /*do*/ {
        let VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node) = analyseVarDecl_q(handle_sue_def, declspecs, declr, vec![], None);

        checkValidTypeDef(is_inline, storage_spec, attrs);
        when((isNoName(name)))(astError(node_info, "NoName in analyseTypeDef".to_string()));
        let ident = identOfVarName(name);

        handleTypeDef((TypeDef(ident, ty, attrs, node_info)))
    }
}

pub fn builtinType(_0: CBuiltin) -> m<Type> {
    match (_0) {
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

pub fn checkInits(_0: Type, _1: Vec<CDesignator>, _2: CInitList) -> m<()> {
    match (_0, _1, _2) {
        (_, _, []) => {
            ()
        },
        (t, dds, [(ds, i), is]) => {
            /*do*/ {
                let (dds_q, ds_q) = match (dds, ds) {
                        ([], []) => {
                            typeError((nodeInfo(i)), "excess elements in initializer".to_string())
                        },
                        ([dd_q, rest], []) => {
                            (rest, vec![dd_q])
                        },
                        (_, [d, _]) => {
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
    /*do*/ {
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

pub fn computeFunDefStorage(_0: Ident, _1: StorageSpec) -> m<Storage> {
    match (_0, _1) {
        (_, StaticSpec(b)) => {
            return(FunLinkage(InternalLinkage))
        },
        (ident, other_spec) => {
            /*do*/ {
                let obj_opt = lookupObject(ident);

                let defaultSpec = FunLinkage(ExternalLinkage);

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
    MachineDesc {
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
    }
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

pub fn enclosingFunctionType(_0: Vec<StmtCtx>) -> Option<Type> {
    match (_0) {
        [] => {
            None
        },
        [FunCtx(vd), _] => {
            Some(declType(vd))
        },
        [_, cs] => {
            enclosingFunctionType(cs)
        },
    }
}

pub fn extFunProto(VarDeclInfo(var_name, is_inline, storage_spec, attrs, ty, node_info): VarDeclInfo) -> m<()> {
    /*do*/ {
        when((isNoName(var_name)))(astError(node_info, "NoName in extFunProto".to_string()));
        let old_fun = lookupObject((identOfVarName(var_name)));

        checkValidSpecs;
        let decl = VarDecl(var_name, (DeclAttrs(is_inline, (funDeclLinkage(old_fun)), attrs)), ty);

        handleVarDecl(false, (Decl(decl, node_info)));
        enterPrototypeScope;
        maybe((()), (mapM_(handleParamDecl)), (getParams(ty)));
        leavePrototypeScope
    }
}

pub fn extVarDecl(VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Option<Initializer>) -> m<()> {
    /*do*/ {
        when((isNoName(var_name)))(astError(node_info, "NoName in extVarDecl".to_string()));
        let (storage, is_def) = globalStorage(storage_spec);

        let vardecl = VarDecl(var_name, (DeclAttrs(is_inline, storage, attrs)), typ);

        if is_def { handleObjectDef(false, ident) }(ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, false)(Decl(vardecl, node_info)))
    }
}

pub fn getParams(_0: Type) -> Option<Vec<ParamDecl>> {
    match (_0) {
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

pub fn localVarDecl(VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Option<Initializer>) -> m<()> {
    /*do*/ {
        when((isNoName(var_name)))(astError(node_info, "NoName in localVarDecl".to_string()));
        let (storage, is_def) = localStorage(storage_spec);

        let vardecl = VarDecl(var_name, (DeclAttrs(is_inline, storage, attrs)), typ);

        if is_def {         
handleObjectDef(true, ident, (ObjDef(vardecl, init_opt, node_info)))} else {
handleVarDecl(true, (Decl(vardecl, node_info)))
        }
    }
}

pub fn matchDesignator(_0: CDesignator, _1: CDesignator) -> bool {
    match (_0, _1) {
        (CMemberDesig(m1, _), CMemberDesig(m2, _)) => {
            (m1 == m2)
        },
        (_, _) => {
            true
        },
    }
}

pub fn tBlockItem(_0: Vec<StmtCtx>, _1: CBlockItem) -> m<Type> {
    match (_0, _1) {
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

pub fn tDesignator(_0: Type, _1: Vec<CDesignator>) -> m<Type> {
    match (_0, _1) {
        (ArrayType(bt, _, _, _), [CArrDesig(e, ni), ds]) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (ArrayType(bt, _, _, _), [CRangeDesig(e1, e2, ni), ds]) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e1), checkIntegral_q(ni));
                __op_bind(tExpr(vec![], RValue, e2), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (ArrayType(_, _, _, _), [d, ds]) => {
            typeError((nodeInfo(d)), "member designator in array initializer".to_string())
        },
        (t, __OP__, DirectType(TyComp(_), _, _), [CMemberDesig(m, ni), ds]) => {
            /*do*/ {
                let mt = fieldType(ni, m, t);

                tDesignator((canonicalType(mt)), ds)
            }
        },
        (t, __OP__, DirectType(TyComp(_), _, _), [d, _]) => {
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
            /*do*/ {
                let dt = getDefTable;

                match lookupType(dt, n) {
                    Some(t) => {
                        t
                    },
                    None => {
                        /*do*/ {
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

pub fn tExpr_q(_0: Vec<StmtCtx>, _1: ExprSide, _2: CExpr) -> m<Type> {
    match (_0, _1, _2) {
        (c, side, CBinary(op, le, re, ni)) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CUnary(CAdrOp, e, ni)) => {
            /*do*/ {
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
            /*do*/ {
                let t = tExpr(c, RValue, e);

                checkIntegral_q(ni, t);
                t
            }
        },
        (c, side, CUnary(CNegOp, e, ni)) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "logical negation used as lvalue".to_string()));
                __op_bind(tExpr(c, RValue, e), checkScalar_q(ni));
                boolType
            }
        },
        (c, side, CUnary(op, e, _)) => {
            tExpr(c, (if isEffectfulOp(op) {                 
LValue} else {
side
                }), e)
        },
        (c, _, CIndex(b, i, ni)) => {
            /*do*/ {
                let bt = tExpr(c, RValue, b);

                let it = tExpr(c, RValue, i);

                let addrTy = binopType_q(ni, CAddOp, bt, it);

                typeErrorOnLeft(ni)(derefType(addrTy))
            }
        },
        (c, side, CCond(e1, me2, e3, ni)) => {
            /*do*/ {
                let t1 = tExpr(c, RValue, e1);

                checkScalar_q((nodeInfo(e1)), t1);
                let t3 = tExpr(c, side, e3);

                match me2 {
                    Some(e2) => {
                        /*do*/ {
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
            /*do*/ {
                let t = tExpr(c, RValue, e);

                let bt = if deref {                     
typeErrorOnLeft(ni, (derefType(t)))} else {
t
                    };

                fieldType(ni, m, bt)
            }
        },
        (c, side, CComma(es, _)) => {
            __op_bind(mapM((tExpr(c, side)), es), last)
        },
        (c, side, CCast(d, e, ni)) => {
            /*do*/ {
                let dt = analyseTypeDecl(d);

                let et = tExpr(c, side, e);

                typeErrorOnLeft(ni)(castCompatible(dt, et));
                dt
            }
        },
        (c, side, CSizeofExpr(e, ni)) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "sizeof as lvalue".to_string()));
                tExpr(c, RValue, e);
                size_tType
            }
        },
        (c, side, CAlignofExpr(e, ni)) => {
            /*do*/ {
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
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "label address as lvalue".to_string()));
                return(PtrType(voidType, noTypeQuals, vec![]))
            }
        },
        (_, side, CCompoundLit(d, initList, ni)) => {
            /*do*/ {
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
        (c, side, CCall(CVar(i, _), args, ni)) => {
            /* Expr::Error */ Error
        },
        (c, _, CCall(fe, args, ni)) => {
            /*do*/ {
                let defType = FunctionType((FunTypeIncomplete((DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes)))), noAttributes);

                let fallback = |i| {
                    /*do*/ {
                        warn(invalidAST(ni)(__op_addadd("unknown function: ".to_string(), identToString(i))));
                        defType
                    }
                };

                let t = match fe {
                        CVar(i, _) => {
                            __op_bind(lookupObject(i), maybe((fallback(i)), (__TODO_const(tExpr(c, RValue, fe)))))
                        },
                        _ => {
                            tExpr(c, RValue, fe)
                        },
                    };

                let atys = mapM((tExpr(c, RValue)), args);

                match canonicalType(t) {
                    PtrType(FunctionType(FunType(rt, pdecls, varargs), _), _, _) => {
                        /*do*/ {
                            let ptys = __map!(declType, pdecls);

                            mapM_(checkArg)(zip3(ptys, atys, args));
                            unless(varargs)(when((__op_assign_div(length(atys), length(ptys))))(typeError(ni, "incorrect number of arguments".to_string())));
                            return(canonicalType(rt))
                        }
                    },
                    PtrType(FunctionType(FunTypeIncomplete(rt), _), _, _) => {
                        /*do*/ {
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
            /*do*/ {
                let lt = tExpr(c, LValue, le);

                let rt = tExpr(c, RValue, re);

                when((constant(typeQuals(lt))))(typeError(ni)(__op_addadd("assignment to lvalue with `constant\' qualifier: ".to_string(), (render(pretty))(le))));
                match (canonicalType(lt), re) {
                    (lt_q, CConst(CIntConst(i, _))) if (isPointerType(lt_q) && (getCInteger(i) == 0)) => { () }
                    (_, _) => {
                        assignCompatible_q(ni, op, lt, rt)
                    },
                };
                lt
            }
        },
        (c, _, CStatExpr(s, _)) => {
            /*do*/ {
                enterBlockScope;
                mapM_((withDefTable(defineLabel)), (getLabels(s)));
                let t = tStmt(c, s);

                leaveBlockScope;
                t
            }
        },
    }
}

pub fn tInit(_0: Type, _1: CInit, _2: m<Initializer>) -> m<Initializer> {
    match (_0, _1, _2, _3) {
        (t, i, __OP__, CInitExpr(e, ni)) => {
            /*do*/ {
                let it = tExpr(vec![], RValue, e);

                assignCompatible_q(ni, CAssignOp, t, it);
                i
            }
        },
        (t, i, __OP__, CInitList(initList, ni)) => {
            __op_rshift(tInitList(ni, (canonicalType(t)), initList), i)
        },
    }
}

pub fn tInitList(_0: NodeInfo, _1: Type, _2: CInitList, _3: m<()>) -> m<()> {
    match (_0, _1, _2, _3, _4) {
        (ni, t, __OP__, ArrayType(DirectType(TyIntegral(TyChar), _, _), _, _, _), [([], CInitExpr(e, __OP__, CConst(CStrConst(_, _)), _))]) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (ni, t, __OP__, ArrayType(_, _, _, _), initList) => {
            /*do*/ {
                let default_ds = repeat((CArrDesig((CConst((CIntConst((cInteger(0)), ni)))), ni)));

                checkInits(t, default_ds, initList)
            }
        },
        (ni, t, __OP__, DirectType(TyComp(ctr), _, _), initList) => {
            /*do*/ {
                let td = lookupSUE(ni, (sueRef(ctr)));

                let ms = tagMembers(ni, td);

                let default_ds = __map!((|m| { CMemberDesig((fst(m)), ni) }), ms);

                checkInits(t, default_ds, initList)
            }
        },
        (ni, PtrType(DirectType(TyVoid, _, _), _, _), _) => {
            ()
        },
        (_, t, [([], i)]) => {
            __op_rshift(tInit(t, i), ())
        },
        (ni, t, _) => {
            typeError(ni)(__op_addadd("initializer list for type: ".to_string(), pType(t)))
        },
    }
}

pub fn tStmt(_0: Vec<StmtCtx>, _1: CStat) -> m<Type> {
    match (_0, _1) {
        (c, CLabel(_, s, _, _)) => {
            tStmt(c, s)
        },
        (c, CExpr(e, _)) => {
            maybe((voidType), (tExpr(c, RValue)), e)
        },
        (c, CCompound(ls, body, _)) => {
            /*do*/ {
                enterBlockScope;
                mapM_((withDefTable(defineLabel)), ls);
                let t = foldM((__TODO_const(tBlockItem(c))), voidType, body);

                leaveBlockScope;
                t
            }
        },
        (c, CIf(e, sthen, selse, _)) => {
            __op_rshift(checkGuard(c, e), __op_rshift(tStmt(c, sthen), __op_rshift(maybe((()), (|s| { __op_rshift(tStmt(c, s), ()) }), selse), voidType)))
        },
        (c, CSwitch(e, s, ni)) => {
            __op_bind(tExpr(c, RValue, e), __op_rshift(checkIntegral_q(ni), tStmt((__op_concat(SwitchCtx, c)), s)))
        },
        (c, CWhile(e, s, _, _)) => {
            __op_rshift(checkGuard(c, e), tStmt((__op_concat(LoopCtx, c)), s))
        },
        (_, CGoto(l, ni)) => {
            /*do*/ {
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
            /*do*/ {
                unless((inLoop(c)))(astError(ni, "continue statement outside of loop".to_string()));
                voidType
            }
        },
        (c, CBreak(ni)) => {
            /*do*/ {
                unless(((inLoop(c) || inSwitch(c))))(astError(ni, "break statement outside of loop or switch statement".to_string()));
                voidType
            }
        },
        (c, CReturn(Some(e), ni)) => {
            /*do*/ {
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
            /*do*/ {
                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                __op_bind(tExpr(c, RValue, e), checkIntegral_q(ni));
                tStmt(c, s)
            }
        },
        (c, CCases(e1, e2, s, ni)) => {
            /*do*/ {
                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                __op_bind(tExpr(c, RValue, e1), checkIntegral_q(ni));
                __op_bind(tExpr(c, RValue, e2), checkIntegral_q(ni));
                tStmt(c, s)
            }
        },
        (c, CDefault(s, ni)) => {
            /*do*/ {
                unless((inSwitch(c)))(astError(ni, "default statement outside of switch statement".to_string()));
                tStmt(c, s)
            }
        },
        (c, CFor(i, g, inc, s, _)) => {
            /*do*/ {
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
            /*do*/ {
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



