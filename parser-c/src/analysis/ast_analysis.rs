// Original file: "AstAnalysis.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::SemError;
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::TravMonad;
// use Language::C::Analysis::ConstEval;
// use Language::C::Analysis::DefTable;
// use globalDefs;
// use Language::C::Analysis::DeclAnalysis;
// use Language::C::Analysis::TypeUtils;
// use Language::C::Analysis::TypeCheck;
// use Language::C::Data;
// use Language::C::Pretty;
// use Language::C::Syntax::AST;
// use Language::C::Syntax::Constants;
// use Language::C::Syntax::Ops;
// use Language::C::Syntax::Utils;
// use Text::PrettyPrint::HughesPJ;
// use Prelude;
// use Control::Monad;
// use Data::Map;
// use Data::Maybe;
// use Data::Traversable;
// use mapM;
// use Data::Foldable;
// use mapM_;

pub fn analyseAST(CTranslUnit(decls, _file_node): CTranslUnit) -> m<GlobalDecls> {

    let mapRecoverM_ = |f| {
        mapM_((handleTravError(f)))
    };

    /*do*/ {
        mapRecoverM_(analyseExt, decls);
        __op_bind(getDefTable, |dt| { if !((inFileScope(dt))) { __error!("Internal Error: Not in filescope after analysis".to_string()) } });
        liftM(globalDefs, getDefTable)
    }
}

pub fn analyseExt(_0: CExtDecl) -> m<()> {
    match (_0) {
        CAsmExt(asm, _) => {
            handleAsmBlock(asm)
        },
        CFDefExt(fundef) => {
            handleAsmBlock(asm)
        },
        CDeclExt(decl) => {
            handleAsmBlock(asm)
        },
    }
}

pub fn analyseFunDef(CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info): CFunDef) -> m<()> {

    let improveFunDefType = |_0| {
        match (_0) {
            FunctionType(FunTypeIncomplete(return_ty), attrs) => {
                FunctionType((FunType(return_ty, vec![], false)), attrs)
            },
            ty => {
                FunctionType((FunType(return_ty, vec![], false)), attrs)
            },
        }
    };

    /*do*/ {
        let var_decl_info = analyseVarDecl_q(true, declspecs, declr, oldstyle_decls, None);

        let VarDeclInfo(name, fun_spec, storage_spec, attrs, ty, _declr_node) = var_decl_info;

        if (isNoName(name)) { astError(node_info, "NoName in analyseFunDef".to_string()) };
        let ident = identOfVarName(name);

        let ty_q = improveFunDefType(ty);

        let fun_storage = computeFunDefStorage(ident, storage_spec);

        let var_decl = VarDecl(name, (DeclAttrs(fun_spec, fun_storage, attrs)), ty_q);

        handleVarDecl(false, (Decl(var_decl, node_info)));
        let stmt_q = analyseFunctionBody(node_info, var_decl, stmt);

        handleFunDef(ident, (FunDef(var_decl, stmt_q, node_info)))
    }
}

pub fn voidM<a>(m: m<a>) -> m<()> {
    __op_rshift(m, ())
}

pub fn analyseDecl(_0: bool, _1: CDecl) -> m<()> {
    match (_0, _1) {
        (_is_local, CStaticAssert(_expr, _strlit, _annot)) => {
            ()
        },
        (is_local, decl, __OP__, CDecl(declspecs, declrs, node)) => {
            ()
        },
    }
}

pub fn analyseTypeDef(handle_sue_def: bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, node_info: NodeInfo) -> m<()> {

    /*do*/ {
        let VarDeclInfo(name, fun_attrs, storage_spec, attrs, ty, _node) = analyseVarDecl_q(handle_sue_def, declspecs, declr, vec![], None);

        checkValidTypeDef(fun_attrs, storage_spec, attrs);
        if (isNoName(name)) { astError(node_info, "NoName in analyseTypeDef".to_string()) };
        let ident = identOfVarName(name);

        handleTypeDef((TypeDef(ident, ty, attrs, node_info)))
    }
}

pub fn computeFunDefStorage(_0: Ident, _1: StorageSpec) -> m<Storage> {
    match (_0, _1) {
        (_, StaticSpec(_)) => {
            FunLinkage(InternalLinkage)
        },
        (ident, other_spec) => {
            FunLinkage(InternalLinkage)
        },
    }
}

pub fn getParams(_0: Type) -> Option<Vec<ParamDecl>> {
    match (_0) {
        FunctionType(FunType(_, params, _), _) => {
            Some(params)
        },
        _ => {
            Some(params)
        },
    }
}

pub fn extFunProto(VarDeclInfo(var_name, fun_spec, storage_spec, attrs, ty, node_info): VarDeclInfo) -> m<()> {

    let funDeclLinkage = |old_fun| {
        match storage_spec {
            NoStorageSpec => {
                FunLinkage(ExternalLinkage)
            },
            StaticSpec(false) => {
                FunLinkage(InternalLinkage)
            },
            ExternSpec(false) => {
                match old_fun {
                    None => {
                        FunLinkage(ExternalLinkage)
                    },
                    Some(f) => {
                        declStorage(f)
                    },
                }
            },
            _ => {
                __error!(__op_addadd("funDeclLinkage: ".to_string(), show(storage_spec)))
            },
        }
    };

    /*do*/ {
        if (isNoName(var_name)) { astError(node_info, "NoName in extFunProto".to_string()) };
        let old_fun = lookupObject((identOfVarName(var_name)));

        checkValidSpecs;
        let decl = VarDecl(var_name, (DeclAttrs(fun_spec, (funDeclLinkage(old_fun)), attrs)), ty);

        handleVarDecl(false, (Decl(decl, node_info)));
        enterPrototypeScope;
        maybe((()), (mapM_(handleParamDecl)), (getParams(ty)));
        leavePrototypeScope
    }
}

pub fn extVarDecl(VarDeclInfo(var_name, fun_spec, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Option<Initializer>) -> m<()> {

    let ident = identOfVarName(var_name);

    let hasFunDef = |dt| {
        any((isFuncDef(snd)), (Map::toList(gObjs(globalDefs(dt)))))
    };

    let isFuncDef = |_0| {
        match (_0) {
            FunctionDef(fd) => {
                not((isInline(functionAttrs))(fd))
            },
            _ => {
                not((isInline(functionAttrs))(fd))
            },
        }
    };

    /*do*/ {
        if (isNoName(var_name)) { astError(node_info, "NoName in extVarDecl".to_string()) };
        let (storage, is_def) = globalStorage(storage_spec);

        let vardecl = VarDecl(var_name, (DeclAttrs(fun_spec, storage, attrs)), typ);

        if is_def { handleObjectDef(false, ident, ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, false, Decl(vardecl, node_info))) }
    }
}

pub fn localVarDecl(VarDeclInfo(var_name, fun_attrs, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Option<Initializer>) -> m<()> {

    let ident = identOfVarName(var_name);

    let localStorage = |_0| {
        match (_0) {
            NoStorageSpec => {
                (Auto(false), true)
            },
            ThreadSpec => {
                (Auto(false), true)
            },
            RegSpec => {
                (Auto(false), true)
            },
            StaticSpec(thread_local) => {
                (Auto(false), true)
            },
            ExternSpec(thread_local) => {
                (Auto(false), true)
            },
            _ => {
                (Auto(false), true)
            },
        }
    };

    /*do*/ {
        if (isNoName(var_name)) { astError(node_info, "NoName in localVarDecl".to_string()) };
        let (storage, is_def) = localStorage(storage_spec);

        let vardecl = VarDecl(var_name, (DeclAttrs(fun_attrs, storage, attrs)), typ);

        if is_def {         
handleObjectDef(true, ident, (ObjDef(vardecl, init_opt, node_info)))} else {
handleVarDecl(true, (Decl(vardecl, node_info)))
        }
    }
}

pub fn defineParams(ni: NodeInfo, decl: VarDecl) -> m<()> {
    match getParams((declType(decl))) {
        None => {
            astError(ni, "expecting complete function type in function definition".to_string())
        },
        Some(params) => {
            mapM_(handleParamDecl, params)
        },
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
            /*do*/ {
                enterFunctionScope;
                mapM_((withDefTable(defineLabel)), (__op_addadd(localLabels, getLabels(s))));
                defineParams(node_info, decl);
                mapM_((tBlockItem(vec![FunCtx(decl)])), items);
                leaveFunctionScope;
                s
            }
        },
    }
}

pub enum StmtCtx {
    FunCtx(VarDecl),
    LoopCtx,
    SwitchCtx
}
pub use self::StmtCtx::*;

pub fn enclosingFunctionType(_0: Vec<StmtCtx>) -> Option<Type> {
    match (_0) {
        [] => {
            None
        },
        [FunCtx(vd), _] => {
            None
        },
        [_, cs] => {
            None
        },
    }
}

pub fn inLoop(c: Vec<StmtCtx>) -> bool {

    let isLoop = |_0| {
        match (_0) {
            LoopCtx => {
                true
            },
            _ => {
                true
            },
        }
    };

    any(isLoop, c)
}

pub fn inSwitch(c: Vec<StmtCtx>) -> bool {

    let isSwitch = |_0| {
        match (_0) {
            SwitchCtx => {
                true
            },
            _ => {
                true
            },
        }
    };

    any(isSwitch, c)
}

#[derive(Debug, Eq)]
pub enum ExprSide {
    LValue,
    RValue
}
pub use self::ExprSide::*;

pub fn tStmt(_0: Vec<StmtCtx>, _1: CStat) -> m<Type> {
    match (_0, _1) {
        (c, CLabel(_, s, _, _)) => {
            tStmt(c, s)
        },
        (c, CExpr(e, _)) => {
            tStmt(c, s)
        },
        (c, CCompound(ls, body, _)) => {
            tStmt(c, s)
        },
        (c, CIf(e, sthen, selse, _)) => {
            tStmt(c, s)
        },
        (c, CSwitch(e, s, ni)) => {
            tStmt(c, s)
        },
        (c, CWhile(e, s, _, _)) => {
            tStmt(c, s)
        },
        (_, CGoto(l, ni)) => {
            tStmt(c, s)
        },
        (c, CCont(ni)) => {
            tStmt(c, s)
        },
        (c, CBreak(ni)) => {
            tStmt(c, s)
        },
        (c, CReturn(Some(e), ni)) => {
            tStmt(c, s)
        },
        (_, CReturn(None, _)) => {
            tStmt(c, s)
        },
        (_, CAsm(_, _)) => {
            tStmt(c, s)
        },
        (c, CCase(e, s, ni)) => {
            tStmt(c, s)
        },
        (c, CCases(e1, e2, s, ni)) => {
            tStmt(c, s)
        },
        (c, CDefault(s, ni)) => {
            tStmt(c, s)
        },
        (c, CFor(i, g, inc, s, _)) => {
            tStmt(c, s)
        },
        (c, CGotoPtr(e, ni)) => {
            tStmt(c, s)
        },
    }
}

pub fn tBlockItem(_0: Vec<StmtCtx>, _1: CBlockItem) -> m<Type> {
    match (_0, _1) {
        (c, CBlockStmt(s)) => {
            tStmt(c, s)
        },
        (_, CBlockDecl(d)) => {
            tStmt(c, s)
        },
        (_, CNestedFunDef(fd)) => {
            tStmt(c, s)
        },
    }
}

pub fn checkGuard(c: Vec<StmtCtx>, e: CExpr) -> m<()> {
    __op_bind(tExpr(c, RValue, e), checkScalar_q((nodeInfo(e))))
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
                    TyInt128 => {
                        16
                    },
                    TyUInt128 => {
                        16
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
                    TyInt128 => {
                        16
                    },
                    TyUInt128 => {
                        16
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

                            withDefTable((|dt_q| { (t, insertType(dt_q, n, t)) }))
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
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CUnary(CAdrOp, e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, _, CUnary(CIndOp, e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, _, CUnary(CCompOp, e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CUnary(CNegOp, e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CUnary(op, e, _)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, _, CIndex(b, i, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CCond(e1, me2, e3, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, _, CMember(e, m, deref, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CComma(es, _)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CCast(d, e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CSizeofExpr(e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CAlignofExpr(e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CComplexReal(e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CComplexImag(e, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, side, CLabAddrExpr(_, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, side, CCompoundLit(d, initList, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, RValue, CAlignofType(_, _)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, RValue, CSizeofType(_, _)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, LValue, CAlignofType(_, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, LValue, CSizeofType(_, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (ctx, side, CGenericSelection(expr, list, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, _, CVar(i, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, _, CConst(c)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_, _, CBuiltinExpr(b)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, side, CCall(CVar(i, _), args, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, _, CCall(fe, args, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, _, CAssign(op, le, re, ni)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (c, _, CStatExpr(s, _)) => {
            /*do*/ {
                if ((side == LValue)) { typeError(ni, "binary operator as lvalue".to_string()) };
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
    }
}

pub fn tInitList(_0: NodeInfo, _1: Type, _2: CInitList) -> m<()> {
    match (_0, _1, _2) {
        (_, ArrayType(DirectType(TyIntegral(TyChar), _, _), _, _, _), [([], CInitExpr(e, __OP__, CConst(CStrConst(_, _)), _))]) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (ni, t, __OP__, ArrayType(_, _, _, _), initList) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (ni, t, __OP__, DirectType(TyComp(ctr), _, _), initList) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (_, PtrType(DirectType(TyVoid, _, _), _, _), _) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (_, t, [([], i)]) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (ni, t, _) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
    }
}

pub fn checkInits(_0: Type, _1: Vec<CDesignator>, _2: CInitList) -> m<()> {
    match (_0, _1, _2) {
        (_, _, []) => {
            ()
        },
        (t, dds, [(ds, i), is]) => {
            ()
        },
    }
}

pub fn advanceDesigList(ds: Vec<CDesignator>, d: CDesignator) -> Vec<CDesignator> {
    drop(1, dropWhile((not(matchDesignator(d))), ds))
}

pub fn matchDesignator(_0: CDesignator, _1: CDesignator) -> bool {
    match (_0, _1) {
        (CMemberDesig(m1, _), CMemberDesig(m2, _)) => {
            (m1 == m2)
        },
        (_, _) => {
            (m1 == m2)
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
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (ArrayType(_, _, _, _), [d, _]) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (t, __OP__, DirectType(TyComp(_), _, _), [CMemberDesig(m, ni), ds]) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (DirectType(TyComp(_), _, _), [d, _]) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (t, []) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (_t, _) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
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
            /*do*/ {
                let it = tExpr(vec![], RValue, e);

                assignCompatible_q(ni, CAssignOp, t, it);
                i
            }
        },
    }
}

pub fn complexBaseType(ni: NodeInfo, c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m<Type> {
    /*do*/ {
        let t = tExpr(c, side, e);

        match canonicalType(t) {
            DirectType(TyComplex(ft), quals, attrs) => {
                DirectType((TyFloating(ft)), quals, attrs)
            },
            _ => {
                typeError(ni, __op_addadd("expected complex type, got: ".to_string(), pType(t)))
            },
        }
    }
}

pub fn builtinType(_0: CBuiltin) -> m<Type> {
    match (_0) {
        CBuiltinVaArg(_, d, _) => {
            analyseTypeDecl(d)
        },
        CBuiltinOffsetOf(_, _, _) => {
            analyseTypeDecl(d)
        },
        CBuiltinTypesCompatible(_, _, _) => {
            analyseTypeDecl(d)
        },
    }
}

pub fn hasTypeDef(declspecs: Vec<CDeclSpec>) -> Option<Vec<CDeclSpec>> {

    let hasTypeDefSpec = |_0, _1| {
        match (_0, _1) {
            (CStorageSpec(CTypedef(_)), (_, specs)) => {
                (true, specs)
            },
            (spec, (b, specs)) => {
                (true, specs)
            },
        }
    };

    match foldr(hasTypeDefSpec, (false, vec![]), declspecs) {
        (true, specs_q) => {
            Some(specs_q)
        },
        (false, _) => {
            None
        },
    }
}



