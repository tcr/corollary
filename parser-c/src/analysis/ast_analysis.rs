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
        __op_bind(getDefTable, |dt| { unless((inFileScope(dt)))(__error!("Internal Error: Not in filescope after analysis".to_string())) });
        liftM(globalDefs, getDefTable)
    }
}

pub fn analyseDecl(_0: bool, _1: CDecl) -> m<()> {
    match (_0, _1) {
        (_0, _1) => {
            ()
        },
        (_0, _1) => {
            ()
        },
    }
}

pub fn analyseExt(_0: CExtDecl) -> m<()> {
    match (_0) {
        _0 => {
            handleAsmBlock(asm)
        },
        _0 => {
            handleAsmBlock(asm)
        },
        _0 => {
            handleAsmBlock(asm)
        },
    }
}

pub fn analyseFunDef(CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info): CFunDef) -> m<()> {
    /*do*/ {
        let var_decl_info = analyseVarDecl_q(true, declspecs, declr, oldstyle_decls, None);

        let VarDeclInfo(name, fun_spec, storage_spec, attrs, ty, _declr_node) = var_decl_info;

        when((isNoName(name)))(astError(node_info, "NoName in analyseFunDef".to_string()));
        let ident = identOfVarName(name);

        let ty_q = improveFunDefType(ty);

        let fun_storage = computeFunDefStorage(ident, storage_spec);

        let var_decl = VarDecl(name, (DeclAttrs(fun_spec, fun_storage, attrs)), ty_q);

        handleVarDecl(false, (Decl(var_decl, node_info)));
        let stmt_q = analyseFunctionBody(node_info, var_decl, stmt);

        handleFunDef(ident, (FunDef(var_decl, stmt_q, node_info)))
    }
}

pub fn analyseFunctionBody(_0: NodeInfo, _1: VarDecl, _2: CStat, _3: m<Stmt>) -> m<Stmt> {
    match (_0, _1, _2, _3, _4) {
        (_0, _1, _2, _3, _4) => {
            /*do*/ {
                enterFunctionScope;
                mapM_((withDefTable(defineLabel)), (__op_addadd(localLabels, getLabels(s))));
                defineParams(node_info, decl);
                mapM_((tBlockItem(vec![FunCtx(decl)])), items);
                leaveFunctionScope;
                s
            }
        },
        (_0, _1, _2, _3, _4) => {
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

pub fn analyseTypeDef(handle_sue_def: bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, node_info: NodeInfo) -> m<()> {
    /*do*/ {
        let VarDeclInfo(name, fun_attrs, storage_spec, attrs, ty, _node) = analyseVarDecl_q(handle_sue_def, declspecs, declr, vec![], None);

        checkValidTypeDef(fun_attrs, storage_spec, attrs);
        when((isNoName(name)))(astError(node_info, "NoName in analyseTypeDef".to_string()));
        let ident = identOfVarName(name);

        handleTypeDef((TypeDef(ident, ty, attrs, node_info)))
    }
}

pub fn builtinType(_0: CBuiltin) -> m<Type> {
    match (_0) {
        _0 => {
            analyseTypeDecl(d)
        },
        _0 => {
            analyseTypeDecl(d)
        },
        _0 => {
            analyseTypeDecl(d)
        },
    }
}

pub fn checkGuard(c: Vec<StmtCtx>, e: CExpr) -> m<()> {
    __op_bind(tExpr(c, RValue, e), checkScalar_q((nodeInfo(e))))
}

pub fn checkInits(_0: Type, _1: Vec<CDesignator>, _2: CInitList) -> m<()> {
    match (_0, _1, _2) {
        (_0, _1, _2) => {
            ()
        },
        (_0, _1, _2) => {
            ()
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
        (_0, _1) => {
            return(FunLinkage(InternalLinkage))
        },
        (_0, _1) => {
            return(FunLinkage(InternalLinkage))
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

pub fn enclosingFunctionType(_0: Vec<StmtCtx>) -> Option<Type> {
    match (_0) {
        _0 => {
            None
        },
        _0 => {
            None
        },
        _0 => {
            None
        },
    }
}

pub fn extFunProto(VarDeclInfo(var_name, fun_spec, storage_spec, attrs, ty, node_info): VarDeclInfo) -> m<()> {
    /*do*/ {
        when((isNoName(var_name)))(astError(node_info, "NoName in extFunProto".to_string()));
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
    /*do*/ {
        when((isNoName(var_name)))(astError(node_info, "NoName in extVarDecl".to_string()));
        let (storage, is_def) = globalStorage(storage_spec);

        let vardecl = VarDecl(var_name, (DeclAttrs(fun_spec, storage, attrs)), typ);

        if is_def { handleObjectDef(false, ident) }(ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, false)(Decl(vardecl, node_info)))
    }
}

pub fn getParams(_0: Type) -> Option<Vec<ParamDecl>> {
    match (_0) {
        _0 => {
            Some(params)
        },
        _0 => {
            Some(params)
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

pub fn localVarDecl(VarDeclInfo(var_name, fun_attrs, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Option<Initializer>) -> m<()> {
    /*do*/ {
        when((isNoName(var_name)))(astError(node_info, "NoName in localVarDecl".to_string()));
        let (storage, is_def) = localStorage(storage_spec);

        let vardecl = VarDecl(var_name, (DeclAttrs(fun_attrs, storage, attrs)), typ);

        if is_def {         
handleObjectDef(true, ident, (ObjDef(vardecl, init_opt, node_info)))} else {
handleVarDecl(true, (Decl(vardecl, node_info)))
        }
    }
}

pub fn matchDesignator(_0: CDesignator, _1: CDesignator) -> bool {
    match (_0, _1) {
        (_0, _1) => {
            (m1 == m2)
        },
        (_0, _1) => {
            (m1 == m2)
        },
    }
}

pub fn tBlockItem(_0: Vec<StmtCtx>, _1: CBlockItem) -> m<Type> {
    match (_0, _1) {
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
    }
}

pub fn tDesignator(_0: Type, _1: Vec<CDesignator>) -> m<Type> {
    match (_0, _1) {
        (_0, _1) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (_0, _1) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (_0, _1) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (_0, _1) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (_0, _1) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (_0, _1) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
        },
        (_0, _1) => {
            /*do*/ {
                __op_bind(tExpr(vec![], RValue, e), checkIntegral_q(ni));
                tDesignator(bt, ds)
            }
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
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
        (_0, _1, _2) => {
            /*do*/ {
                when(((side == LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);

                let rt = tExpr(c, RValue, re);

                binopType_q(ni, op, lt, rt)
            }
        },
    }
}

pub fn tInit(_0: Type, _1: CInit, _2: m<Initializer>) -> m<Initializer> {
    match (_0, _1, _2, _3) {
        (_0, _1, _2, _3) => {
            /*do*/ {
                let it = tExpr(vec![], RValue, e);

                assignCompatible_q(ni, CAssignOp, t, it);
                i
            }
        },
        (_0, _1, _2, _3) => {
            /*do*/ {
                let it = tExpr(vec![], RValue, e);

                assignCompatible_q(ni, CAssignOp, t, it);
                i
            }
        },
    }
}

pub fn tInitList(_0: NodeInfo, _1: Type, _2: CInitList) -> m<()> {
    match (_0, _1, _2) {
        (_0, _1, _2) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (_0, _1, _2) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (_0, _1, _2) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (_0, _1, _2) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (_0, _1, _2) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
        (_0, _1, _2) => {
            __op_rshift(tExpr(vec![], RValue, e), ())
        },
    }
}

pub fn tStmt(_0: Vec<StmtCtx>, _1: CStat) -> m<Type> {
    match (_0, _1) {
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
        (_0, _1) => {
            tStmt(c, s)
        },
    }
}

pub fn voidM<a>(m: m<a>) -> m<()> {
    __op_rshift(m, ())
}



