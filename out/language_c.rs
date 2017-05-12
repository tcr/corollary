mod Language_C_Analysis_AstAnalysis {
    struct StmtCtx(FunCtx, VarDecl, LoopCtx, SwitchCtx);

    #[derive(Eq, Show)]
    struct ExprSide(LValue, RValue);

    fn advanceDesigList(ds: Vec<CDesignator>, d: CDesignator) -> Vec<CDesignator> {
        drop(1)(dropWhile(((not . matchDesignator(d))), ds))
    }

    fn analyseAST(CTranslUnit(decls, _file_node): CTranslUnit) -> m(GlobalDecls) {
        {
            let mapRecoverM_ = |f| {
                mapM_(((handleTravError . f)))
            };

            mapRecoverM_(analyseExt, decls);
            >>=(getDefTable, Lambda((not((inFileScope(dt)))))(error("Internal Error: Not in filescope after analysis".to_string())));
            liftM(globalDefs, getDefTable)
        }
    }

    fn analyseExt(__0: CExtDecl) -> m(EmptyParen) {
        match ((__0)).as_ref() {
            CAsmExt(asm, _) => (handleAsmBlock(asm)).into(),
            CFDefExt(fundef) => (analyseFunDef(fundef)).into(),
            CDeclExt(decl) => (analyseDecl(False, decl)).into(),
        }
    }

    fn analyseFunDef(CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info): CFunDef) -> m(EmptyParen) {
        {
            let improveFunDefType = |__0| {
                match ((__0)).as_ref() {
                    FunctionType(FunTypeIncomplete(return_ty), attrs) => (return(FunctionType((FunType(return_ty, vec![], False)), attrs))).into(),
                    ty => (return(ty)).into(),
                }
            };

            let var_decl_info = analyseVarDecl'(True, declspecs, declr, oldstyle_decls, Nothing);
            Let;
            when((isNoName(name)))(astError(node_info, "NoName in analyseFunDef".to_string()));
            Let;
            let ty' = improveFunDefType(ty);
            let fun_storage = computeFunDefStorage(ident, storage_spec);
            Let;
            handleVarDecl(False, (Decl(var_decl, node_info)));
            let stmt' = analyseFunctionBody(node_info, var_decl, stmt);
            handleFunDef(ident, (FunDef(var_decl, stmt', node_info)))
        }
    }

    fn analyseFunctionBody(__0: NodeInfo, __1: VarDecl, __2: CStat, __3: m(Stmt)) -> m(Stmt) {
        match ((__0, __1, __2, __3, __4)).as_ref() {
            node_info decl s EmptyParen CCompound(localLabels, items, _) => ({
                enterFunctionScope;
                mapM_(((withDefTable . defineLabel)), (++(localLabels, getLabels(s))));
                defineParams(node_info, decl);
                mapM_((tBlockItem(vec![FunCtx(decl)])), items);
                leaveFunctionScope;
                return(s)
            }).into(),
            _ _ s => (astError((nodeInfo(s)), "Function body is no compound statement".to_string())).into(),
        }
    }

    fn analyseTypeDef(handle_sue_def: Bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, node_info: NodeInfo) -> m(EmptyParen) {
        {
            let checkValidTypeDef = |__0, __1, __2| {
                match ((__0, __1, __2)).as_ref() {
                    True _ _ => (astError(node_info, "inline specifier for typeDef".to_string())).into(),
                    _ NoStorageSpec _ => (return(())).into(),
                    _ bad_storage _ => (astError(node_info)(++("storage specified for typeDef: ".to_string(), show(bad_storage)))).into(),
                }
            };

            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl'(handle_sue_def, declspecs, declr, vec![], Nothing);
            checkValidTypeDef(is_inline, storage_spec, attrs);
            when((isNoName(name)))(astError(node_info, "NoName in analyseTypeDef".to_string()));
            Let;
            handleTypeDef((TypeDef(ident, ty, attrs, node_info)))
        }
    }

    fn builtinType(__0: CBuiltin) -> m(Type) {
        match ((__0)).as_ref() {
            CBuiltinVaArg(_, d, _) => (analyseTypeDecl(d)).into(),
            CBuiltinOffsetOf(_, _, _) => (return(size_tType)).into(),
            CBuiltinTypesCompatible(_, _, _) => (return(boolType)).into(),
        }
    }

    fn checkGuard(c: Vec<StmtCtx>, e: CExpr) -> m(EmptyParen) {
        >>=(tExpr(c, RValue, e), checkScalar'((nodeInfo(e))))
    }

    fn checkInits(__0: Type, __1: Vec<CDesignator>, __2: CInitList) -> m(EmptyParen) {
        match ((__0, __1, __2)).as_ref() {
            _ _ Vec<> => (return(())).into(),
            t dds (ds, i)(EmptyParen, is) => ({
                let (dds', ds') = match ((dds, ds)).as_ref() {
                                (Vec<>, Vec<>) => (typeError((nodeInfo(i)), "excess elements in initializer".to_string())).into(),
                                (dd'(EmptyParen, rest), Vec<>) => (return((rest, vec![dd']))).into(),
                                (_, d(EmptyParen, _)) => (return((advanceDesigList(dds, d), ds))).into(),
                            };
                let t' = tDesignator(t, ds');
                tInit(t', i);
                checkInits(t, dds', is)
            }).into(),
        }
    }

    fn complexBaseType(ni: NodeInfo, c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m(Type) {
        {
            let t = tExpr(c, side, e);
            match (canonicalType(t)).as_ref() {
                        DirectType TyComplex(ft) quals attrs => (return(DirectType((TyFloating(ft)), quals, attrs))).into(),
                        _ => (typeError(ni)(++("expected complex type, got: ".to_string(), pType(t)))).into(),
                    }
        }
    }

    fn computeFunDefStorage(__0: Ident, __1: StorageSpec) -> m(Storage) {
        match ((__0, __1)).as_ref() {
            _ StaticSpec(b) => (return(FunLinkage(InternalLinkage))).into(),
            ident other_spec => ({
                let obj_opt = lookupObject(ident);
                Let;
                match (other_spec).as_ref() {
                            NoStorageSpec => (return(maybe(defaultSpec, declStorage, obj_opt))).into(),
                            ExternSpec(False) => (return(maybe(defaultSpec, declStorage, obj_opt))).into(),
                            bad_spec => (throwTravError(badSpecifierError((nodeInfo(ident)))(++("unexpected function storage specifier (only static or extern is allowed)".to_string(), show(bad_spec))))).into(),
                        }
            }).into(),
        }
    }

    fn defaultMD() -> MachineDesc {
        MachineDesc(hashmap! {
            "iSize" => Lambda,
            "fSize" => Lambda,
            "builtinSize" => Lambda,
            "ptrSize" => 4,
            "voidSize" => 1,
            "iAlign" => Lambda,
            "fAlign" => Lambda,
            "builtinAlign" => Lambda,
            "ptrAlign" => 4,
            "voidAlign" => 1
            })
    }

    fn defineParams(ni: NodeInfo, decl: VarDecl) -> m(EmptyParen) {
        match ((getParams(declType(decl)))).as_ref() {
                Nothing => (astError(ni, "expecting complete function type in function definition".to_string())).into(),
                Just params => (mapM_(handleParamDecl, params)).into(),
            }
    }

    fn enclosingFunctionType(__0: Vec<StmtCtx>) -> Maybe(Type) {
        match ((__0)).as_ref() {
            Vec<> => (Nothing).into(),
            FunCtx(vd, EmptyParen, _) => (Just(declType(vd))).into(),
            _(EmptyParen, cs) => (enclosingFunctionType(cs)).into(),
        }
    }

    fn extFunProto(VarDeclInfo(var_name, is_inline, storage_spec, attrs, ty, node_info): VarDeclInfo) -> m(EmptyParen) {
        {
            when((isNoName(var_name)))(astError(node_info, "NoName in extFunProto".to_string()));
            let old_fun = lookupObject((identOfVarName(var_name)));
            checkValidSpecs;
            Let;
            handleVarDecl(False, (Decl(decl, node_info)));
            enterPrototypeScope;
            maybe((return(())), (mapM_(handleParamDecl)), (getParams(ty)));
            leavePrototypeScope
        }
    }

    fn extVarDecl(VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Maybe(Initializer)) -> m(EmptyParen) {
        {
            when((isNoName(var_name)))(astError(node_info, "NoName in extVarDecl".to_string()));
            let (storage, is_def) = globalStorage(storage_spec);
            Let;
            if(is_def, then, handleObjectDef, False, ident)(ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, False)(Decl(vardecl, node_info)))
        }
    }

    fn getParams(__0: Type) -> Maybe(Vec<ParamDecl>) {
        match ((__0)).as_ref() {
            FunctionType(FunType(_, params, _), _) => (Just(params)).into(),
            _ => (Nothing).into(),
        }
    }

    fn hasTypeDef(declspecs: Vec<CDeclSpec>) -> Maybe(Vec<CDeclSpec>) {
        match (foldr(hasTypeDefSpec, (False, vec![]), declspecs)).as_ref() {
                (True, specs') => (Just(specs')).into(),
                (False, _) => (Nothing).into(),
            }
    }

    fn inLoop(c: Vec<StmtCtx>) -> Bool {
        any(isLoop, c)
    }

    fn inSwitch(c: Vec<StmtCtx>) -> Bool {
        any(isSwitch, c)
    }

    fn localVarDecl(VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info): VarDeclInfo, init_opt: Maybe(Initializer)) -> m(EmptyParen) {
        {
            when((isNoName(var_name)))(astError(node_info, "NoName in localVarDecl".to_string()));
            let (storage, is_def) = localStorage(storage_spec);
            Let;
            if(is_def, then, handleObjectDef, True, ident, (ObjDef(vardecl, init_opt, node_info)), else, handleVarDecl, True, (Decl(vardecl, node_info)))
        }
    }

    fn matchDesignator(__0: CDesignator, __1: CDesignator) -> Bool {
        match ((__0, __1)).as_ref() {
            CMemberDesig(m1, _) CMemberDesig(m2, _) => (==(m1, m2)).into(),
            _ _ => (True).into(),
        }
    }

    fn tBlockItem(__0: Vec<StmtCtx>, __1: CBlockItem) -> m(Type) {
        match ((__0, __1)).as_ref() {
            c CBlockStmt(s) => (tStmt(c, s)).into(),
            _ CBlockDecl(d) => (>(analyseDecl(True, d), >((), return(voidType)))).into(),
            _ CNestedFunDef(fd) => (>(analyseFunDef(fd), >((), return(voidType)))).into(),
        }
    }

    fn tDesignator(__0: Type, __1: Vec<CDesignator>) -> m(Type) {
        match ((__0, __1)).as_ref() {
            ArrayType(bt, _, _, _) CArrDesig(e, ni, EmptyParen, ds) => ({
                >>=(tExpr(vec![], RValue, e), checkIntegral'(ni));
                tDesignator(bt, ds)
            }).into(),
            ArrayType(bt, _, _, _) CRangeDesig(e1, e2, ni, EmptyParen, ds) => ({
                >>=(tExpr(vec![], RValue, e1), checkIntegral'(ni));
                >>=(tExpr(vec![], RValue, e2), checkIntegral'(ni));
                tDesignator(bt, ds)
            }).into(),
            ArrayType(_, _, _, _) d(EmptyParen, ds) => (typeError((nodeInfo(d)), "member designator in array initializer".to_string())).into(),
            t EmptyParen DirectType(TyComp(_), _, _) CMemberDesig(m, ni, EmptyParen, ds) => ({
                let mt = fieldType(ni, m, t);
                tDesignator((canonicalType(mt)), ds)
            }).into(),
            t EmptyParen DirectType(TyComp(_), _, _) d(EmptyParen, _) => (typeError((nodeInfo(d)), "array designator in compound initializer".to_string())).into(),
            t Vec<> => (return(t)).into(),
        }
    }

    fn tExpr(c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m(Type) {
        match (nameOfNode((nodeInfo(e)))).as_ref() {
                Just n => ({
                    let dt = getDefTable;
                    match (lookupType(dt, n)).as_ref() {
                                Just t => (return(t)).into(),
                                Nothing => ({
                                    let t = tExpr'(c, side, e);
                                    withDefTable((Lambda))
                                }).into(),
                            }
                }).into(),
                Nothing => (tExpr'(c, side, e)).into(),
            }
    }

    fn tExpr'(__0: Vec<StmtCtx>, __1: ExprSide, __2: CExpr) -> m(Type) {
        match ((__0, __1, __2)).as_ref() {
            c side CBinary(op, le, re, ni) => ({
                when((==(side, LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);
                let rt = tExpr(c, RValue, re);
                binopType'(ni, op, lt, rt)
            }).into(),
            c side CUnary(CAdrOp, e, ni) => ({
                when((==(side, LValue)))(typeError(ni, "address-of operator as lvalue".to_string()));
                match (e).as_ref() {
                            CCompoundLit _ _ _ => (liftM(simplePtr, tExpr(c, RValue, e))).into(),
                            CVar i _ => (>>=(lookupObject(i), (typeErrorOnLeft(ni) . maybe((notFound(i)), varAddrType)))).into(),
                            _ => (liftM(simplePtr, tExpr(c, LValue, e))).into(),
                        }
            }).into(),
            c _ CUnary(CIndOp, e, ni) => (>>=(tExpr(c, RValue, e), ((typeErrorOnLeft(ni) . derefType)))).into(),
            c _ CUnary(CCompOp, e, ni) => ({
                let t = tExpr(c, RValue, e);
                checkIntegral'(ni, t);
                return(t)
            }).into(),
            c side CUnary(CNegOp, e, ni) => ({
                when((==(side, LValue)))(typeError(ni, "logical negation used as lvalue".to_string()));
                >>=(tExpr(c, RValue, e), checkScalar'(ni));
                return(boolType)
            }).into(),
            c side CUnary(op, e, _) => (tExpr(c, (if(isEffectfulOp, op, then, LValue, else, side)), e)).into(),
            c _ CIndex(b, i, ni) => ({
                let bt = tExpr(c, RValue, b);
                let it = tExpr(c, RValue, i);
                let addrTy = binopType'(ni, CAddOp, bt, it);
                typeErrorOnLeft(ni)(derefType(addrTy))
            }).into(),
            c side CCond(e1, me2, e3, ni) => ({
                let t1 = tExpr(c, RValue, e1);
                checkScalar'((nodeInfo(e1)), t1);
                let t3 = tExpr(c, side, e3);
                match (me2).as_ref() {
                            Just e2 => ({
                                let t2 = tExpr(c, side, e2);
                                conditionalType'(ni, t2, t3)
                            }).into(),
                            Nothing => (conditionalType'(ni, t1, t3)).into(),
                        }
            }).into(),
            c side CMember(e, m, deref, ni) => ({
                let t = tExpr(c, RValue, e);
                let bt = if(deref, then, typeErrorOnLeft, ni, (derefType(t)), else, return, t);
                fieldType(ni, m, bt)
            }).into(),
            c side CComma(es, _) => (>>=(mapM((tExpr(c, side)), es), (return . last))).into(),
            c side CCast(d, e, ni) => ({
                let dt = analyseTypeDecl(d);
                let et = tExpr(c, side, e);
                typeErrorOnLeft(ni)(castCompatible(dt, et));
                return(dt)
            }).into(),
            c side CSizeofExpr(e, ni) => ({
                when((==(side, LValue)))(typeError(ni, "sizeof as lvalue".to_string()));
                tExpr(c, RValue, e);
                return(size_tType)
            }).into(),
            c side CAlignofExpr(e, ni) => ({
                when((==(side, LValue)))(typeError(ni, "alignof as lvalue".to_string()));
                tExpr(c, RValue, e);
                return(size_tType)
            }).into(),
            c side CComplexReal(e, ni) => (complexBaseType(ni, c, side, e)).into(),
            c side CComplexImag(e, ni) => (complexBaseType(ni, c, side, e)).into(),
            _ side CLabAddrExpr(_, ni) => ({
                when((==(side, LValue)))(typeError(ni, "label address as lvalue".to_string()));
                return(PtrType(voidType, noTypeQuals, vec![]))
            }).into(),
            _ side CCompoundLit(d, initList, ni) => ({
                when((==(side, LValue)))(typeError(ni, "compound literal as lvalue".to_string()));
                let lt = analyseTypeDecl(d);
                tInitList(ni, (canonicalType(lt)), initList);
                return(lt)
            }).into(),
            _ RValue CAlignofType(_, _) => (return(size_tType)).into(),
            _ RValue CSizeofType(_, _) => (return(size_tType)).into(),
            _ LValue CAlignofType(_, ni) => (typeError(ni, "alignoftype as lvalue".to_string())).into(),
            _ LValue CSizeofType(_, ni) => (typeError(ni, "sizeoftype as lvalue".to_string())).into(),
            _ side CVar(i, ni) => (>>=(lookupObject(i), maybe((typeErrorOnLeft(ni)(notFound(i))), ((return . declType))))).into(),
            _ _ CConst(c) => (constType(c)).into(),
            _ _ CBuiltinExpr(b) => (builtinType(b)).into(),
            c _ CCall(fe, args, ni) => ({
                Let;
                let t = match (fe).as_ref() {
                                CVar i _ => (>>=(lookupObject(i), maybe((fallback(i)), (const(tExpr(c, RValue, fe)))))).into(),
                                _ => (tExpr(c, RValue, fe)).into(),
                            };
                let atys = mapM((tExpr(c, RValue)), args);
                match (canonicalType(t)).as_ref() {
                            PtrType FunctionType(FunType(rt, pdecls, varargs), _) _ _ => ({
                                Let;
                                mapM_(checkArg)(zip3(ptys, atys, args));
                                unless(varargs)(when((/=(length(atys), length(ptys))))(typeError(ni, "incorrect number of arguments".to_string())));
                                return(canonicalType(rt))
                            }).into(),
                            PtrType FunctionType(FunTypeIncomplete(rt), _) _ _ => ({
                                return(canonicalType(rt))
                            }).into(),
                            _ => (typeError(ni)(++("attempt to call non-function of type ".to_string(), pType(t)))).into(),
                        }
            }).into(),
            c _ CAssign(op, le, re, ni) => ({
                let lt = tExpr(c, LValue, le);
                let rt = tExpr(c, RValue, re);
                when((constant(typeQuals(lt))))(typeError(ni)(++("assignment to lvalue with `constant\' qualifier: ".to_string(), ((render . pretty))(le))));
                match ((canonicalType(lt), re)).as_ref() {
                        (lt', CConst(CIntConst(i, _))) => if &&(isPointerType(lt'), ==(getCInteger(i), 0)) { return(()) },
                            (_, _) => (assignCompatible'(ni, op, lt, rt)).into(),
                        };
                return(lt)
            }).into(),
            c _ CStatExpr(s, _) => ({
                enterBlockScope;
                mapM_(((withDefTable . defineLabel)), (getLabels(s)));
                let t = tStmt(c, s);
                leaveBlockScope;
                return(t)
            }).into(),
        }
    }

    fn tInit(__0: Type, __1: CInit, __2: m(Initializer)) -> m(Initializer) {
        match ((__0, __1, __2, __3)).as_ref() {
            t i EmptyParen CInitExpr(e, ni) => ({
                let it = tExpr(vec![], RValue, e);
                assignCompatible'(ni, CAssignOp, t, it);
                return(i)
            }).into(),
            t i EmptyParen CInitList(initList, ni) => (>(tInitList(ni, (canonicalType(t)), initList), >((), return(i)))).into(),
        }
    }

    fn tInitList(__0: NodeInfo, __1: Type, __2: CInitList, __3: m(EmptyParen)) -> m(EmptyParen) {
        match ((__0, __1, __2, __3, __4)).as_ref() {
            ni t EmptyParen ArrayType(DirectType(TyIntegral(TyChar), _, _), _, _, _) Vec<(Vec<>, CInitExpr(e, EmptyParen, CConst(CStrConst(_, _)), _))> => (>(tExpr(vec![], RValue, e), >((), return(())))).into(),
            ni t EmptyParen ArrayType(_, _, _, _) initList => ({
                Let;
                checkInits(t, default_ds, initList)
            }).into(),
            ni t EmptyParen DirectType(TyComp(ctr), _, _) initList => ({
                let td = lookupSUE(ni, (sueRef(ctr)));
                let ms = tagMembers(ni, td);
                Let;
                checkInits(t, default_ds, initList)
            }).into(),
            ni PtrType(DirectType(TyVoid, _, _), _, _) _ => (return(())).into(),
            _ t Vec<(Vec<>, i)> => (>(tInit(t, i), >((), return(())))).into(),
            ni t _ => (typeError(ni)(++("initializer list for type: ".to_string(), pType(t)))).into(),
        }
    }

    fn tStmt(__0: Vec<StmtCtx>, __1: CStat) -> m(Type) {
        match ((__0, __1)).as_ref() {
            c CLabel(_, s, _, _) => (tStmt(c, s)).into(),
            c CExpr(e, _) => (maybe((return(voidType)), (tExpr(c, RValue)), e)).into(),
            c CCompound(ls, body, _) => ({
                enterBlockScope;
                mapM_(((withDefTable . defineLabel)), ls);
                let t = foldM((const(tBlockItem(c))), voidType, body);
                leaveBlockScope;
                return(t)
            }).into(),
            c CIf(e, sthen, selse, _) => (>(checkGuard(c, e), >((), >(tStmt(c, sthen), >((), >(maybe((return(())), (>(Lambda(c, s), >((), return(())))), selse), >((), return(voidType)))))))).into(),
            c CSwitch(e, s, ni) => (>>=(tExpr(c, RValue, e), >(checkIntegral'(ni), >((), tStmt((:(SwitchCtx, c)), s))))).into(),
            c CWhile(e, s, _, _) => (>(checkGuard(c, e), >((), tStmt((:(LoopCtx, c)), s)))).into(),
            _ CGoto(l, ni) => ({
                let dt = getDefTable;
                match (lookupLabel(l, dt)).as_ref() {
                            Just _ => (return(voidType)).into(),
                            Nothing => (typeError(ni)(++("undefined label in goto: ".to_string(), identToString(l)))).into(),
                        }
            }).into(),
            c CCont(ni) => ({
                unless((inLoop(c)))(astError(ni, "continue statement outside of loop".to_string()));
                return(voidType)
            }).into(),
            c CBreak(ni) => ({
                unless((||(inLoop(c), inSwitch(c))))(astError(ni, "break statement outside of loop or switch statement".to_string()));
                return(voidType)
            }).into(),
            c CReturn(Just(e), ni) => ({
                let t = tExpr(c, RValue, e);
                let rt = match (enclosingFunctionType(c)).as_ref() {
                                Just FunctionType(FunType(rt, _, _), _) => (return(rt)).into(),
                                Just FunctionType(FunTypeIncomplete(rt), _) => (return(rt)).into(),
                                Just ft => (astError(ni)(++("bad function type: ".to_string(), pType(ft)))).into(),
                                Nothing => (astError(ni, "return statement outside function".to_string())).into(),
                            };
                match ((rt, t)).as_ref() {
                            (DirectType(TyVoid, _, _), DirectType(TyVoid, _, _)) => (return(())).into(),
                            _ => (assignCompatible'(ni, CAssignOp, rt, t)).into(),
                        };
                return(voidType)
            }).into(),
            _ CReturn(Nothing, _) => (return(voidType)).into(),
            _ CAsm(_, _) => (return(voidType)).into(),
            c CCase(e, s, ni) => ({
                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                >>=(tExpr(c, RValue, e), checkIntegral'(ni));
                tStmt(c, s)
            }).into(),
            c CCases(e1, e2, s, ni) => ({
                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                >>=(tExpr(c, RValue, e1), checkIntegral'(ni));
                >>=(tExpr(c, RValue, e2), checkIntegral'(ni));
                tStmt(c, s)
            }).into(),
            c CDefault(s, ni) => ({
                unless((inSwitch(c)))(astError(ni, "default statement outside of switch statement".to_string()));
                tStmt(c, s)
            }).into(),
            c CFor(i, g, inc, s, _) => ({
                enterBlockScope;
                either((maybe((return(())), checkExpr)), (analyseDecl(True)), i);
                maybe((return(())), (checkGuard(c)), g);
                maybe((return(())), checkExpr, inc);
                tStmt((:(LoopCtx, c)), s);
                leaveBlockScope;
                return(voidType)
            }).into(),
            c CGotoPtr(e, ni) => ({
                let t = tExpr(c, RValue, e);
                match (t).as_ref() {
                            PtrType(_, _, _) => (return(voidType)).into(),
                            _ => (typeError(ni, "can\'t goto non-pointer".to_string())).into(),
                        }
            }).into(),
        }
    }

}

mod Language_C_Analysis_Builtins {
    fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/ConstEval.hs"

mod Language_C_Analysis_Debug {
    fn globalDeclStats(file_filter: Pair(Span([Ref(Ident("FilePath"))]), Span([Ref(Ident("Bool"))])), gmap: GlobalDecls) -> Vec<(String, isize)> {
        vec![("Enumeration Constants".to_string(), Map.size(enumerators)), ("Total Object/Function Declarations".to_string(), Map.size(all_decls)), ("Object definitions".to_string(), Map.size(objDefs)), ("Function Definitions".to_string(), Map.size(funDefs)), ("Tag definitions".to_string(), Map.size(tagDefs)), ("TypeDefs".to_string(), Map.size(typeDefs))]
    }

    fn joinComma() -> Doc {
        (hsep . (punctuate(comma) . map(pretty)))
    }

    fn prettyAssocs(label: String) -> Doc {
        prettyAssocsWith(label, pretty, pretty)
    }

    fn prettyAssocsWith(label: String, prettyKey: Pair(Span([Ref(Ident("k"))]), Span([Ref(Ident("Doc"))])), prettyVal: Pair(Span([Ref(Ident("v"))]), Span([Ref(Ident("Doc"))])), theMap: Vec<(k, v)>) -> Doc {
        text(label)(()((nest(8))((vcat(map(prettyEntry, theMap))))))
    }

    fn terminateSemi() -> Doc {
        (terminateSemi_ . map(pretty))
    }

    fn terminateSemi_() -> Doc {
        (hsep . map((<((), Operator(">")(semi)))))
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/DeclAnalysis.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/DefTable.hs"

mod Language_C_Analysis_Export {
    fn exportArraySize(__0: ArraySize) -> CArrSize {
        match ((__0)).as_ref() {
            ArraySize(static, e) => (CArrSize(static, e)).into(),
            UnknownArraySize(complete) => (CNoArrSize(complete)).into(),
        }
    }

    fn exportAttrs() -> Vec<CAttr> {
        map(exportAttr)
    }

    fn exportCompType(CompType(sue_ref, comp_tag, members, attrs, node_info): CompType) -> Vec<CTypeSpec> {
        vec![CSUType(comp, ni)]
    }

    fn exportCompTypeDecl(ty: CompTypeRef) -> Vec<CTypeSpec> {
        vec![CSUType((exportComp(ty)), ni)]
    }

    fn exportCompTypeRef(CompType(sue_ref, com_tag, _, _, node_info): CompType) -> Vec<CTypeSpec> {
        exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
    }

    fn exportComplexType(ty: FloatType) -> Vec<CTypeSpec> {
        :((CComplexType(ni)), exportFloatType(ty))
    }

    fn exportDeclAttrs(DeclAttrs(inline, storage, attrs): DeclAttrs) -> Vec<CDeclSpec> {
        ++((if(inline, then, vec![CTypeQual((CInlineQual(ni)))], else, vec![])), ++(map((CStorageSpec), (exportStorage(storage))), map(((CTypeQual . CAttrQual)), (exportAttrs(attrs)))))
    }

    fn exportDeclr(other_specs: Vec<CDeclSpec>, ty: Type, attrs: Attributes, name: VarName) -> (Vec<CDeclSpec>, CDeclr) {
        (++(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
    }

    fn exportEnumType(EnumType(sue_ref, enumerators, attrs, node_info): EnumType) -> Vec<CTypeSpec> {
        vec![CEnumType(enum, ni)]
    }

    fn exportEnumTypeDecl(ty: EnumTypeRef) -> Vec<CTypeSpec> {
        vec![CEnumType((exportEnum(ty)), ni)]
    }

    fn exportEnumTypeRef(EnumType(sue_ref, _, _, node_info): EnumType) -> Vec<CTypeSpec> {
        exportEnumTypeDecl((EnumTypeRef(sue_ref, node_info)))
    }

    fn exportFloatType(ty: FloatType) -> Vec<CTypeSpec> {
        match (ty).as_ref() {
                TyFloat => (vec![CFloatType(ni)]).into(),
                TyDouble => (vec![CDoubleType(ni)]).into(),
                TyLDouble => (vec![CLongType(ni), CDoubleType(ni)]).into(),
            }
    }

    fn exportIntType(ty: IntType) -> Vec<CTypeSpec> {
        match (ty).as_ref() {
                TyBool => (vec![CBoolType(ni)]).into(),
                TyChar => (vec![CCharType(ni)]).into(),
                TySChar => (vec![CSignedType(ni), CCharType(ni)]).into(),
                TyUChar => (vec![CUnsigType(ni), CCharType(ni)]).into(),
                TyShort => (vec![CShortType(ni)]).into(),
                TyUShort => (vec![CUnsigType(ni), CShortType(ni)]).into(),
                TyInt => (vec![CIntType(ni)]).into(),
                TyUInt => (vec![CUnsigType(ni), CIntType(ni)]).into(),
                TyLong => (vec![CLongType(ni)]).into(),
                TyULong => (vec![CUnsigType(ni), CLongType(ni)]).into(),
                TyLLong => (vec![CLongType(ni), CLongType(ni)]).into(),
                TyULLong => (vec![CUnsigType(ni), CLongType(ni), CLongType(ni)]).into(),
            }
    }

    fn exportMemberDecl(__0: MemberDecl) -> CDecl {
        match ((__0)).as_ref() {
            AnonBitField(ty, expr, node_info) => (CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info)).into(),
            MemberDecl(vardecl, bitfieldsz, node_info) => (Let(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info)).into(),
        }
    }

    fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
        Let(in, CDecl, specs, vec![(Just(declr), Nothing, Nothing)], (nodeInfo(paramdecl)))
    }

    fn exportSUERef() -> Maybe(Ident) {
        (Just . (internalIdent . show))
    }

    fn exportStorage(__0: Storage) -> Vec<CStorageSpec> {
        match ((__0)).as_ref() {
            NoStorage => (vec![]).into(),
            Auto(reg) => (if(reg, then, vec![CRegister(ni)], else, vec![])).into(),
            Static(InternalLinkage, thread_local) => (threadLocal(thread_local, vec![CStatic(ni)])).into(),
            Static(ExternalLinkage, thread_local) => (threadLocal(thread_local, vec![CExtern(ni)])).into(),
            Static(NoLinkage, _) => (error("impossible storage: static without linkage".to_string())).into(),
            FunLinkage(InternalLinkage) => (vec![CStatic(ni)]).into(),
            FunLinkage(ExternalLinkage) => (vec![]).into(),
            FunLinkage(NoLinkage) => (error("impossible storage: function without linkage".to_string())).into(),
        }
    }

    fn exportType(ty: Type) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {
        exportTy(vec![], ty)
    }

    fn exportTypeDecl(ty: Type) -> CDecl {
        CDecl(declspecs, declrs, ni)
    }

    fn exportTypeDef(TypeDef(ident, ty, attrs, node_info): TypeDef) -> CDecl {
        CDecl((:(CStorageSpec((CTypedef(ni))), declspecs)), vec![declr], node_info)
    }

    fn exportTypeQuals(quals: TypeQuals) -> Vec<CTypeQual> {
        mapMaybe(select, vec![(constant, CConstQual(ni)), (volatile, CVolatQual(ni)), (restrict, CRestrQual(ni))])
    }

    fn exportTypeQualsAttrs(tyqs: TypeQuals, attrs: Attributes) -> Vec<CTypeQual> {
        (++(exportTypeQuals(tyqs), map(CAttrQual, (exportAttrs(attrs)))))
    }

    fn exportTypeSpec(tyname: TypeName) -> Vec<CTypeSpec> {
        match (tyname).as_ref() {
                TyVoid => (vec![CVoidType(ni)]).into(),
                TyIntegral ity => (exportIntType(ity)).into(),
                TyFloating fty => (exportFloatType(fty)).into(),
                TyComplex fty => (exportComplexType(fty)).into(),
                TyComp comp => (exportCompTypeDecl(comp)).into(),
                TyEnum enum => (exportEnumTypeDecl(enum)).into(),
                TyBuiltin TyVaList => (vec![CTypeDef((internalIdent("va_list".to_string())), ni)]).into(),
                TyBuiltin TyAny => (vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)]).into(),
            }
    }

    fn exportVarDecl(VarDecl(name, attrs, ty): VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    }

    fn fromDirectType(__0: Type) -> TypeName {
        match ((__0)).as_ref() {
            DirectType(ty, _, _) => (ty).into(),
            TypeDefType(TypeDefRef(_, ref, _), _, _) => (maybe((error("undefined typeDef".to_string())), fromDirectType, ref)).into(),
            _ => (error("fromDirectType".to_string())).into(),
        }
    }

    fn ni() -> NodeInfo {
        undefNode
    }

    fn threadLocal(__0: Bool) -> Vec<CStorageSpec> {
        match ((__0)).as_ref() {
            False => (id).into(),
            True => (((CThread(ni))(Operator(":")))).into(),
        }
    }

}

mod Language_C_Analysis_NameSpaceMap {
    struct NameSpaceMap(NsMap, Map(k, v), Vec<Vec<(k, v)>>);

    fn defGlobal(NsMap(gs, lss): NameSpaceMap(k, a), ident: k, def: a) -> (NameSpaceMap(k, a), Maybe(a)) {
        (NsMap((Map.insert(ident, def, gs)), lss), Map.lookup(ident, gs))
    }

    fn defLocal(__0: NameSpaceMap(k, a), __1: k, __2: a, __3: (NameSpaceMap(k, a), Maybe(a))) -> (NameSpaceMap(k, a), Maybe(a)) {
        match ((__0, __1, __2, __3, __4)).as_ref() {
            ns EmptyParen NsMap(_, Vec<>) ident def => (defGlobal(ns, ident, def)).into(),
            NsMap(gs, ls(EmptyParen, lss)) ident def => ((NsMap(gs, (:((:((ident, def), ls)), lss))), Prelude.lookup(ident, ls))).into(),
        }
    }

    fn enterNewScope(NsMap(gs, lss): NameSpaceMap(k, a)) -> NameSpaceMap(k, a) {
        NsMap(gs, (:(vec![], lss)))
    }

    fn globalNames(NsMap(g, _): NameSpaceMap(k, v)) -> Map(k, v) {
        g
    }

    fn hasLocalNames(NsMap(_, l): NameSpaceMap(k, v)) -> Bool {
        not((null(l)))
    }

    fn leaveScope(__0: NameSpaceMap(k, a)) -> (NameSpaceMap(k, a), Vec<(k, a)>) {
        match ((__0)).as_ref() {
            NsMap(_, Vec<>) => (error("NsMaps.leaveScope: No local scope!".to_string())).into(),
            NsMap(gs, ls(EmptyParen, lss)) => ((NsMap(gs, lss), ls)).into(),
        }
    }

    fn localNames(NsMap(_, l): NameSpaceMap(k, v)) -> Vec<Vec<(k, v)>> {
        l
    }

    fn lookupGlobal(NsMap(gs, _): NameSpaceMap(k, a), ident: k) -> Maybe(a) {
        Map.lookup(ident, gs)
    }

    fn lookupInnermostScope(nsm: NameSpaceMap(k, a), EmptyParen: k, NsMap(_gs, localDefs): Maybe(a)) -> Maybe(a) {
        match (localDefs).as_ref() {
                ls(EmptyParen, _lss) => (Prelude.lookup(ident, ls)).into(),
                Vec<> => (lookupGlobal(nsm, ident)).into(),
            }
    }

    fn lookupName(ns: NameSpaceMap(k, a), EmptyParen: k, NsMap(_, localDefs): Maybe(a)) -> Maybe(a) {
        match ((lookupLocal(localDefs))).as_ref() {
                Nothing => (lookupGlobal(ns, ident)).into(),
                Just def => (Just(def)).into(),
            }
    }

    fn mergeNameSpace(NsMap(global1, local1): NameSpaceMap(k, a), NsMap(global2, local2): NameSpaceMap(k, a)) -> NameSpaceMap(k, a) {
        NsMap((Map.union(global1, global2)), (localUnion(local1, local2)))
    }

    fn nameSpaceMap() -> NameSpaceMap(k, v) {
        NsMap(Map.empty, vec![])
    }

    fn nsMapToList(NsMap(gs, lss): NameSpaceMap(k, a)) -> Vec<(k, a)> {
        ++(concat(lss), Map.toList(gs))
    }

}

mod Language_C_Analysis_SemError {
    #[derive(Debug)]
    struct RedefError(RedefError, ErrorLevel, RedefInfo);

    struct RedefInfo(RedefInfo, String, RedefKind, NodeInfo, NodeInfo);

    struct RedefKind(DuplicateDef, DiffKindRedecl, ShadowedDef, DisagreeLinkage, NoLinkageOld);

    #[derive(Debug)]
    struct TypeMismatch(TypeMismatch, String, (NodeInfo, Type), (NodeInfo, Type));

    fn badSpecifierError(node_info: NodeInfo, msg: String) -> BadSpecifierError {
        BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn invalidAST(node_info: NodeInfo, msg: String) -> InvalidASTError {
        InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn prevDeclMsg(old_node: NodeInfo) -> Vec<String> {
        vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
    }

    fn redefErrLabel(RedefInfo(ident, _, _, _): RedefInfo) -> String {
        ++(ident, " redefined".to_string())
    }

    fn redefErrReason(__0: RedefInfo) -> String {
        match ((__0)).as_ref() {
            RedefInfo(ident, DuplicateDef, _, _) => (++("duplicate definition of ".to_string(), ident)).into(),
            RedefInfo(ident, ShadowedDef, _, _) => (++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string()))).into(),
            RedefInfo(ident, DiffKindRedecl, _, _) => (++(ident, " previously declared as a different kind of symbol".to_string())).into(),
            RedefInfo(ident, DisagreeLinkage, _, _) => (++(ident, " previously declared with different linkage".to_string())).into(),
            RedefInfo(ident, NoLinkageOld, _, _) => (++(ident, " previously declared without linkage".to_string())).into(),
        }
    }

    fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, EmptyParen: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (++(vec![redefErrReason(info)], prevDeclMsg(old_node))))
    }

    fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
        RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
    }

    fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    fn typeMismatchInfo(TypeMismatch(reason, (node1, _ty2), _t2): TypeMismatch) -> ErrorInfo {
        ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
    }

}

mod Language_C_Analysis_SemRep {
    #[derive(Debug, Clone)]
    struct TagDef(CompDef, CompType, EnumDef, EnumType);

    #[derive(Debug, Clone)]
    struct IdentDecl(Declaration, Decl, ObjectDef, ObjDef, FunctionDef, FunDef, EnumeratorDef, Enumerator);

    struct GlobalDecls(GlobalDecls, RecordTODO);

    #[derive()]
    struct DeclEvent(TagEvent, TagDef, DeclEvent, IdentDecl, ParamEvent, ParamDecl, LocalEvent, IdentDecl, TypeDefEvent, TypeDef, AsmEvent, AsmBlock);

    #[derive(Debug, Clone)]
    struct Decl(Decl, VarDecl, NodeInfo);

    #[derive(Debug, Clone)]
    struct ObjDef(ObjDef, VarDecl, Maybe(Initializer), NodeInfo);

    #[derive(Debug, Clone)]
    struct FunDef(FunDef, VarDecl, Stmt, NodeInfo);

    #[derive(Debug, Clone)]
    struct ParamDecl(ParamDecl, VarDecl, NodeInfo, AbstractParamDecl, VarDecl, NodeInfo);

    #[derive(Debug, Clone)]
    struct MemberDecl(MemberDecl, VarDecl, Maybe(Expr), NodeInfo, AnonBitField, Type, Expr, NodeInfo);

    #[derive(Debug, Clone)]
    struct TypeDef(TypeDef, Ident, Type, Attributes, NodeInfo);

    #[derive(Debug, Clone)]
    struct VarDecl(VarDecl, VarName, DeclAttrs, Type);

    #[derive(Debug, Clone)]
    struct DeclAttrs(DeclAttrs, Bool, Storage, Attributes);

    #[derive(Debug, Clone, Show, Eq, Ord)]
    struct Storage(NoStorage, Auto, Register, Static, Linkage, ThreadLocal, FunLinkage, Linkage);

    #[derive(Debug, Clone, Show, Eq, Ord)]
    struct Linkage(NoLinkage, InternalLinkage, ExternalLinkage);

    #[derive(Debug, Clone)]
    struct Type(DirectType, TypeName, TypeQuals, Attributes, PtrType, Type, TypeQuals, Attributes, ArrayType, Type, ArraySize, TypeQuals, Attributes, FunctionType, FunType, Attributes, TypeDefType, TypeDefRef, TypeQuals, Attributes);

    #[derive(Debug, Clone)]
    struct FunType(FunType, Type, Vec<ParamDecl>, Bool, FunTypeIncomplete, Type);

    #[derive(Debug, Clone)]
    struct ArraySize(UnknownArraySize, Bool, ArraySize, Bool, Expr);

    #[derive(Debug, Clone)]
    struct TypeName(TyVoid, TyIntegral, IntType, TyFloating, FloatType, TyComplex, FloatType, TyComp, CompTypeRef, TyEnum, EnumTypeRef, TyBuiltin, BuiltinType);

    #[derive(Debug, Clone)]
    struct BuiltinType(TyVaList, TyAny);

    #[derive(Debug, Clone)]
    struct TypeDefRef(TypeDefRef, Ident, Maybe(Type), NodeInfo);

    #[derive(Debug, Clone, Eq, Ord)]
    struct IntType(TyBool, TyChar, TySChar, TyUChar, TyShort, TyUShort, TyInt, TyUInt, TyLong, TyULong, TyLLong, TyULLong);

    #[derive(Debug, Clone, Eq, Ord)]
    struct FloatType(TyFloat, TyDouble, TyLDouble);

    #[derive(Debug, Clone)]
    struct CompTypeRef(CompTypeRef, SUERef, CompTyKind, NodeInfo);

    #[derive(Debug, Clone)]
    struct EnumTypeRef(EnumTypeRef, SUERef, NodeInfo);

    #[derive(Debug, Clone)]
    struct CompType(CompType, SUERef, CompTyKind, Vec<MemberDecl>, Attributes, NodeInfo);

    #[derive(Eq, Ord, Debug, Clone)]
    struct CompTyKind(StructTag, UnionTag);

    #[derive(Debug, Clone)]
    struct EnumType(EnumType, SUERef, Vec<Enumerator>, Attributes, NodeInfo);

    #[derive(Debug, Clone)]
    struct Enumerator(Enumerator, Ident, Expr, EnumType, NodeInfo);

    #[derive(Debug, Clone)]
    struct TypeQuals(TypeQuals, RecordTODO);

    #[derive(Debug, Clone)]
    struct VarName(VarName, Ident, Maybe(AsmName), NoName);

    #[derive(Debug, Clone)]
    struct Attr(Attr, Ident, Vec<Expr>, NodeInfo);

    fn declAttrs() -> DeclAttrs {
        ((Lambda) . getVarDecl)
    }

    fn declIdent() -> Ident {
        (identOfVarName . declName)
    }

    fn declLinkage(decl: d) -> Linkage {
        match (declStorage(decl)).as_ref() {
                NoStorage => (undefined).into(),
                Auto _ => (NoLinkage).into(),
                Static linkage _ => (linkage).into(),
                FunLinkage linkage => (linkage).into(),
            }
    }

    fn declName() -> VarName {
        ((Lambda) . getVarDecl)
    }

    fn declOfDef(def: n) -> Decl {
        Let
    }

    fn declStorage(d: d) -> Storage {
        match (declAttrs(d)).as_ref() {
                DeclAttrs(_, st, _) => (st).into(),
            }
    }

    fn declType() -> Type {
        ((Lambda) . getVarDecl)
    }

    fn emptyGlobalDecls() -> GlobalDecls {
        GlobalDecls(Map.empty, Map.empty, Map.empty)
    }

    fn filterGlobalDecls(decl_filter: Pair(Span([Ref(Ident("DeclEvent"))]), Span([Ref(Ident("Bool"))])), gmap: GlobalDecls) -> GlobalDecls {
        GlobalDecls(hashmap! {
            "gObjs" => Map.filter(((decl_filter . DeclEvent)), (gObjs(gmap))),
            "gTags" => Map.filter(((decl_filter . TagEvent)), (gTags(gmap))),
            "gTypeDefs" => Map.filter(((decl_filter . TypeDefEvent)), (gTypeDefs(gmap)))
            })
    }

    fn hasLinkage(__0: Storage) -> Bool {
        match ((__0)).as_ref() {
            Auto(_) => (False).into(),
            Static(NoLinkage, _) => (False).into(),
            _ => (True).into(),
        }
    }

    fn identOfTypeDef(TypeDef(ide, _, _, _): TypeDef) -> Ident {
        ide
    }

    fn identOfVarName(__0: VarName) -> Ident {
        match ((__0)).as_ref() {
            NoName => (error("identOfVarName: NoName".to_string())).into(),
            VarName(ident, _) => (ident).into(),
        }
    }

    fn isExtDecl() -> Bool {
        (hasLinkage . declStorage)
    }

    fn isNoName(__0: VarName) -> Bool {
        match ((__0)).as_ref() {
            NoName => (True).into(),
            _ => (False).into(),
        }
    }

    fn mergeAttributes() -> Attributes {
        (Operator("++"))
    }

    fn mergeGlobalDecls(gmap1: GlobalDecls, gmap2: GlobalDecls) -> GlobalDecls {
        GlobalDecls(hashmap! {
            "gObjs" => Map.union((gObjs(gmap1)), (gObjs(gmap2))),
            "gTags" => Map.union((gTags(gmap1)), (gTags(gmap2))),
            "gTypeDefs" => Map.union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
            })
    }

    fn mergeTypeQuals(TypeQuals(c1, v1, r1): TypeQuals, TypeQuals(c2, v2, r2): TypeQuals) -> TypeQuals {
        TypeQuals((&&(c1, c2)), (&&(v1, v2)), (&&(r1, r2)))
    }

    fn noAttributes() -> Attributes {
        vec![]
    }

    fn noTypeQuals() -> TypeQuals {
        TypeQuals(False, False, False)
    }

    fn objKindDescr(__0: IdentDecl) -> String {
        match ((__0)).as_ref() {
            Declaration(_) => ("declaration".to_string()).into(),
            ObjectDef(_) => ("object definition".to_string()).into(),
            FunctionDef(_) => ("function definition".to_string()).into(),
            EnumeratorDef(_) => ("enumerator definition".to_string()).into(),
        }
    }

    fn splitIdentDecls(include_all: Bool) -> (Map(Ident, Decl), (Map(Ident, Enumerator), Map(Ident, ObjDef), Map(Ident, FunDef))) {
        Map.foldWithKey((if(include_all, then, deal, else, deal')), (Map.empty, (Map.empty, Map.empty, Map.empty)))
    }

    fn typeOfCompDef(CompType(ref, tag, _, _, _): CompType) -> TypeName {
        TyComp((CompTypeRef(ref, tag, undefNode)))
    }

    fn typeOfEnumDef(EnumType(ref, _, _, _): EnumType) -> TypeName {
        TyEnum((EnumTypeRef(ref, undefNode)))
    }

    fn typeOfTagDef(__0: TagDef) -> TypeName {
        match ((__0)).as_ref() {
            CompDef(comptype) => (typeOfCompDef(comptype)).into(),
            EnumDef(enumtype) => (typeOfEnumDef(enumtype)).into(),
        }
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TravMonad.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TypeCheck.hs"

mod Language_C_Analysis_TypeConversions {
    fn arithmeticConversion(__0: TypeName, __1: TypeName) -> Maybe(TypeName) {
        match ((__0, __1)).as_ref() {
            TyComplex(t1) TyComplex(t2) => (Just(TyComplex(floatConversion(t1, t2)))).into(),
            TyComplex(t1) TyFloating(t2) => (Just(TyComplex(floatConversion(t1, t2)))).into(),
            TyFloating(t1) TyComplex(t2) => (Just(TyComplex(floatConversion(t1, t2)))).into(),
            t1 EmptyParen TyComplex(_) TyIntegral(_) => (Just(t1)).into(),
            TyIntegral(_) t2 EmptyParen TyComplex(_) => (Just(t2)).into(),
            TyFloating(t1) TyFloating(t2) => (Just(TyFloating(floatConversion(t1, t2)))).into(),
            t1 EmptyParen TyFloating(_) TyIntegral(_) => (Just(t1)).into(),
            TyIntegral(_) t2 EmptyParen TyFloating(_) => (Just(t2)).into(),
            TyIntegral(t1) TyIntegral(t2) => (Just(TyIntegral(intConversion(t1, t2)))).into(),
            TyEnum(_) TyEnum(_) => (Just(TyIntegral(TyInt))).into(),
            TyEnum(_) t2 => (Just(t2)).into(),
            t1 TyEnum(_) => (Just(t1)).into(),
            _ _ => (Nothing).into(),
        }
    }

    fn floatConversion() -> FloatType {
        max
    }

    fn intConversion(t1: IntType, t2: IntType) -> IntType {
        max(TyInt, (max(t1, t2)))
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TypeUtils.hs"

mod Language_C_Analysis {

}

mod Language_C_Data_Error {
    #[derive(Eq, Ord)]
    struct ErrorLevel(LevelWarn, LevelError, LevelFatal);

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

    fn internalErr(msg: String) -> a {
        error((++(internalErrPrefix, ++("\\n".to_string(), ++(indentLines(msg), "\\n".to_string())))))
    }

    fn internalErrPrefix() -> String {
        unlines(vec!["Language.C : Internal Error".to_string(), ++("This is propably a bug, and should be reported at ".to_string(), "http://www.sivity.net/projects/language.c/newticket".to_string())])
    }

    fn isHardError() -> Bool {
        ((Operator(">")(LevelWarn)) . errorLevel)
    }

    fn mkErrorInfo(lvl: ErrorLevel, msg: String, node: NodeInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (lines(msg)))
    }

    fn showError(short_msg: String) -> String {
        (showErrorInfo(short_msg) . errorInfo)
    }

    fn showErrorInfo(short_msg: String, ErrorInfo(level, pos, msgs): ErrorInfo) -> String {
        ++(header, showMsgLines((:(if(null, short_msg, then, msgs, else, short_msg), msgs))))
    }

    fn unsupportedFeature(msg: String, a: a) -> UnsupportedFeature {
        UnsupportedFeature(msg, (posOf(a)))
    }

    fn unsupportedFeature_(msg: String) -> UnsupportedFeature {
        UnsupportedFeature(msg, internalPos)
    }

    fn userErr(msg: String) -> UserError {
        UserError((ErrorInfo(LevelError, internalPos, (lines(msg)))))
    }

}

mod Language_C_Data_Ident {
    #[derive(Debug, Clone, Ord, Eq)]
    struct SUERef(AnonymousRef, Name, NamedRef, Ident);

    #[derive(Clone, Debug)]
    struct Ident(Ident, String, isize, NodeInfo);

    fn bits14() -> isize {
        ^(2, (::(14, Int)))
    }

    fn bits21() -> isize {
        ^(2, (::(21, Int)))
    }

    fn bits28() -> isize {
        ^(2, (::(28, Int)))
    }

    fn bits7() -> isize {
        ^(2, (::(7, Int)))
    }

    fn builtinIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
    }

    fn dumpIdent(ide: Ident) -> String {
        ++(identToString(ide), ++(" at ".to_string(), show((nodeInfo(ide)))))
    }

    fn identToString(Ident(s, _, _): Ident) -> String {
        s
    }

    fn internalIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
    }

    fn internalIdentAt(pos: Position, s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
    }

    fn isAnonymousRef(__0: SUERef) -> Bool {
        match ((__0)).as_ref() {
            AnonymousRef(_) => (True).into(),
            _ => (False).into(),
        }
    }

    fn isInternalIdent(Ident(_, _, nodeinfo): Ident) -> Bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo'(pos, (pos, length(s)), name)))
    }

    fn quad(__0: String) -> isize {
        match ((__0)).as_ref() {
            c1(EmptyParen, c2, EmptyParen, c3, EmptyParen, c4, EmptyParen, s) => (+((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28)))).into(),
            c1(EmptyParen, c2, EmptyParen, c3, EmptyParen, Vec<>) => (*(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))).into(),
            c1(EmptyParen, c2, EmptyParen, Vec<>) => (*(ord(c2), +(bits7, ord(c1)))).into(),
            c1(EmptyParen, Vec<>) => (ord(c1)).into(),
            Vec<> => (0).into(),
        }
    }

}

mod Language_C_Data_InputStream {
    fn countLines() -> isize {
        match (()).as_ref() {
             => ((length . BSC.lines)).into(),
             => ((length . lines)).into(),
        }
    }

    fn inputStreamEmpty() -> Bool {
        match (()).as_ref() {
             => (BSW.null).into(),
             => (null).into(),
        }
    }

    fn inputStreamFromString() -> InputStream {
        match (()).as_ref() {
             => (BSC.pack).into(),
             => (id).into(),
        }
    }

    fn inputStreamToString() -> String {
        match (()).as_ref() {
             => (BSC.unpack).into(),
             => (id).into(),
        }
    }

    fn readInputStream() -> IO(InputStream) {
        match (()).as_ref() {
             => (BSW.readFile).into(),
             => (readFile).into(),
        }
    }

    fn takeByte(bs: InputStream) -> (Word8, InputStream) {
        seq(BSW.head(bs), (BSW.head(bs), BSW.tail(bs)))
    }

    fn takeChar(__0: InputStream) -> (Char, InputStream) {
        match ((__0)).as_ref() {
            bs => (seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs)))).into(),
            bs => ((head(bs), tail(bs))).into(),
        }
    }

    fn takeChars(__0: isize, __1: InputStream) -> Vec<Char> {
        match ((__0, __1)).as_ref() {
            n bstr => (BSC.unpack(BSC.take(n, bstr))).into(),
            n str => (take(n, str)).into(),
        }
    }

}

mod Language_C_Data_Name {
    fn namesStartingFrom(k: isize) -> Vec<Name> {
        vec![Name(k..)]
    }

    fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Data/Node.hs"

mod Language_C_Data_Position {
    #[derive(Eq, Ord, Debug, Clone)]
    struct Position(Position, RecordTODO, NoPosition, BuiltinPosition, InternalPosition);

    fn adjustPos(__0: FilePath, __1: isize, __2: Position) -> Position {
        match ((__0, __1, __2)).as_ref() {
            fname row Position(offs, _, _, _) => (Position(offs, fname, row, 1)).into(),
            _ _ p => (p).into(),
        }
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset(__0: Position, __1: isize) -> Position {
        match ((__0, __1)).as_ref() {
            Position(o, f, r, c) n => (Position((+(o, n)), f, r, c)).into(),
            p n => (p).into(),
        }
    }

    fn incPos(__0: Position, __1: isize) -> Position {
        match ((__0, __1)).as_ref() {
            Position(offs, fname, row, col) n => (Position((+(offs, n)), fname, row, (+(col, n)))).into(),
            p _ => (p).into(),
        }
    }

    fn initPos(file: FilePath) -> Position {
        Position(0, file, 1, 1)
    }

    fn internalPos() -> Position {
        InternalPosition
    }

    fn isBuiltinPos(__0: Position) -> Bool {
        match ((__0)).as_ref() {
            BuiltinPosition => (True).into(),
            _ => (False).into(),
        }
    }

    fn isInternalPos(__0: Position) -> Bool {
        match ((__0)).as_ref() {
            InternalPosition => (True).into(),
            _ => (False).into(),
        }
    }

    fn isNoPos(__0: Position) -> Bool {
        match ((__0)).as_ref() {
            NoPosition => (True).into(),
            _ => (False).into(),
        }
    }

    fn isSourcePos(__0: Position) -> Bool {
        match ((__0)).as_ref() {
            Position(_, _, _, _) => (True).into(),
            _ => (False).into(),
        }
    }

    fn nopos() -> Position {
        NoPosition
    }

    fn position() -> Position {
        Position
    }

    fn retPos(__0: Position) -> Position {
        match ((__0)).as_ref() {
            Position(offs, fname, row, _) => (Position((+(offs, 1)), fname, (+(row, 1)), 1)).into(),
            p => (p).into(),
        }
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Data/RList.hs"

mod Language_C_Data {

}

mod Language_C_Parser_Builtin {
    fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Parser/ParserMonad.hs"

// ERROR: cannot yet convert file "./language-c/src/Language/C/Parser/Tokens.hs"

mod Language_C_Parser {
    fn execParser_(parser: P(a), input: InputStream, pos: Position) -> Either(ParseError, a) {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}

// ERROR: cannot yet convert file "./language-c/src/Language/C/Pretty.hs"

mod Language_C_Syntax_AST {
    #[derive(Show, Clone, Debug)]
    struct CTranslationUnit(CTranslUnit, Vec<CExternalDeclaration(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CExternalDeclaration(CDeclExt, CDeclaration(a), CFDefExt, CFunctionDef(a), CAsmExt, CStringLiteral(a), a);

    #[derive(Show, Clone, Debug)]
    struct CFunctionDef(CFunDef, Vec<CDeclarationSpecifier(a)>, CDeclarator(a), Vec<CDeclaration(a)>, CStatement(a), a);

    #[derive(Show, Clone, Debug)]
    struct CDeclaration(CDecl, Vec<CDeclarationSpecifier(a)>, Vec<(Maybe(CDeclarator(a)), Maybe(CInitializer(a)), Maybe(CExpression(a)))>, a);

    #[derive(Show, Clone, Debug)]
    struct CDeclarator(CDeclr, Maybe(Ident), Vec<CDerivedDeclarator(a)>, Maybe(CStringLiteral(a)), Vec<CAttribute(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CDerivedDeclarator(CPtrDeclr, Vec<CTypeQualifier(a)>, a, CArrDeclr, Vec<CTypeQualifier(a)>, CArraySize(a), a, CFunDeclr, Either(Vec<Ident>, (Vec<CDeclaration(a)>, Bool)), Vec<CAttribute(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CArraySize(CNoArrSize, Bool, CArrSize, Bool, CExpression(a));

    #[derive(Show, Clone, Debug)]
    struct CStatement(CLabel, Ident, CStatement(a), Vec<CAttribute(a)>, a, CCase, CExpression(a), CStatement(a), a, CCases, CExpression(a), CExpression(a), CStatement(a), a, CDefault, CStatement(a), a, CExpr, Maybe(CExpression(a)), a, CCompound, Vec<Ident>, Vec<CCompoundBlockItem(a)>, a, CIf, CExpression(a), CStatement(a), Maybe(CStatement(a)), a, CSwitch, CExpression(a), CStatement(a), a, CWhile, CExpression(a), CStatement(a), Bool, a, CFor, Either(Maybe(CExpression(a)), CDeclaration(a)), Maybe(CExpression(a)), Maybe(CExpression(a)), CStatement(a), a, CGoto, Ident, a, CGotoPtr, CExpression(a), a, CCont, a, CBreak, a, CReturn, Maybe(CExpression(a)), a, CAsm, CAssemblyStatement(a), a);

    #[derive(Show, Clone, Debug)]
    struct CAssemblyStatement(CAsmStmt, Maybe(CTypeQualifier(a)), CStringLiteral(a), Vec<CAssemblyOperand(a)>, Vec<CAssemblyOperand(a)>, Vec<CStringLiteral(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CAssemblyOperand(CAsmOperand, Maybe(Ident), CStringLiteral(a), CExpression(a), a);

    #[derive(Show, Clone, Debug)]
    struct CCompoundBlockItem(CBlockStmt, CStatement(a), CBlockDecl, CDeclaration(a), CNestedFunDef, CFunctionDef(a));

    #[derive(Show, Clone, Debug)]
    struct CDeclarationSpecifier(CStorageSpec, CStorageSpecifier(a), CTypeSpec, CTypeSpecifier(a), CTypeQual, CTypeQualifier(a));

    #[derive(Show, Eq, Ord, Clone, Debug)]
    struct CStorageSpecifier(CAuto, a, CRegister, a, CStatic, a, CExtern, a, CTypedef, a, CThread, a);

    #[derive(Show, Clone, Debug)]
    struct CTypeSpecifier(CVoidType, a, CCharType, a, CShortType, a, CIntType, a, CLongType, a, CFloatType, a, CDoubleType, a, CSignedType, a, CUnsigType, a, CBoolType, a, CComplexType, a, CSUType, CStructureUnion(a), a, CEnumType, CEnumeration(a), a, CTypeDef, Ident, a, CTypeOfExpr, CExpression(a), a, CTypeOfType, CDeclaration(a), a);

    #[derive(Show, Clone, Debug)]
    struct CTypeQualifier(CConstQual, a, CVolatQual, a, CRestrQual, a, CInlineQual, a, CAttrQual, CAttribute(a));

    #[derive(Show, Clone, Debug)]
    struct CStructureUnion(CStruct, CStructTag, Maybe(Ident), Maybe(Vec<CDeclaration(a)>), Vec<CAttribute(a)>, a);

    #[derive(Show, Eq, Clone, Debug)]
    struct CStructTag(CStructTag, CUnionTag);

    #[derive(Show, Clone, Debug)]
    struct CEnumeration(CEnum, Maybe(Ident), Maybe(Vec<(Ident, Maybe(CExpression(a)))>), Vec<CAttribute(a)>, a);

    #[derive(Show, Clone, Debug)]
    struct CInitializer(CInitExpr, CExpression(a), a, CInitList, CInitializerList(a), a);

    #[derive(Show, Clone, Debug)]
    struct CPartDesignator(CArrDesig, CExpression(a), a, CMemberDesig, Ident, a, CRangeDesig, CExpression(a), CExpression(a), a);

    #[derive(Show, Clone, Debug)]
    struct CAttribute(CAttr, Ident, Vec<CExpression(a)>, a);

    #[derive(Clone, Debug, Show)]
    struct CExpression(CComma, Vec<CExpression(a)>, a, CAssign, CAssignOp, CExpression(a), CExpression(a), a, CCond, CExpression(a), Maybe(CExpression(a)), CExpression(a), a, CBinary, CBinaryOp, CExpression(a), CExpression(a), a, CCast, CDeclaration(a), CExpression(a), a, CUnary, CUnaryOp, CExpression(a), a, CSizeofExpr, CExpression(a), a, CSizeofType, CDeclaration(a), a, CAlignofExpr, CExpression(a), a, CAlignofType, CDeclaration(a), a, CComplexReal, CExpression(a), a, CComplexImag, CExpression(a), a, CIndex, CExpression(a), CExpression(a), a, CCall, CExpression(a), Vec<CExpression(a)>, a, CMember, CExpression(a), Ident, Bool, a, CVar, Ident, a, CConst, CConstant(a), CCompoundLit, CDeclaration(a), CInitializerList(a), a, CStatExpr, CStatement(a), a, CLabAddrExpr, Ident, a, CBuiltinExpr, CBuiltinThing(a));

    #[derive(Show, Clone, Debug)]
    struct CBuiltinThing(CBuiltinVaArg, CExpression(a), CDeclaration(a), a, CBuiltinOffsetOf, CDeclaration(a), Vec<CPartDesignator(a)>, a, CBuiltinTypesCompatible, CDeclaration(a), CDeclaration(a), a);

    #[derive(Show, Clone, Debug)]
    struct CConstant(CIntConst, CInteger, a, CCharConst, CChar, a, CFloatConst, CFloat, a, CStrConst, CString, a);

    #[derive(Show, Clone, Debug)]
    struct CStringLiteral(CStrLit, CString, a);

    fn cstringOfLit(CStrLit(cstr, _): CStringLiteral(a)) -> CString {
        cstr
    }

    fn fmapInitList(_f: Pair(Span([Ref(Ident("a"))]), Span([Ref(Ident("b"))]))) -> CInitializerList(b) {
        map((Lambda))
    }

    fn isSUEDef(__0: CTypeSpecifier(a)) -> Bool {
        match ((__0)).as_ref() {
            CSUType(CStruct(_, _, Just(_), _, _), _) => (True).into(),
            CEnumType(CEnum(_, Just(_), _, _), _) => (True).into(),
            _ => (False).into(),
        }
    }

    fn liftStrLit(CStrLit(str, at): CStringLiteral(a)) -> CConstant(a) {
        CStrConst(str, at)
    }

    fn partitionDeclSpecs() -> (Vec<CStorageSpecifier(a)>, Vec<CAttribute(a)>, Vec<CTypeQualifier(a)>, Vec<CTypeSpecifier(a)>, Bool) {
        foldr(deals, (vec![], vec![], vec![], vec![], False))
    }

}

mod Language_C_Syntax_Constants {
    #[derive(Eq, Ord, Clone, Debug)]
    struct CChar(CChar, Char, Bool, CChars, Vec<Char>, Bool);

    #[derive(Eq, Ord, Enum, Bounded, Clone, Debug)]
    struct CIntRepr(DecRepr, HexRepr, OctalRepr);

    #[derive(Eq, Ord, Enum, Bounded, Clone, Debug)]
    struct CIntFlag(FlagUnsigned, FlagLong, FlagLongLong, FlagImag);

    #[derive(Eq, Ord, Clone, Debug)]
    struct CInteger(CInteger, Integer, CIntRepr, Flags(CIntFlag));

    #[derive(Eq, Ord, Clone, Debug)]
    struct CFloat(CFloat, String);

    #[derive(Eq, Ord, Clone, Debug)]
    struct CString(CString, Vec<Char>, Bool);

