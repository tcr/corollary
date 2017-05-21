mod Language_C_Analysis_AstAnalysis {
    struct StmtCtx(FunCtx, VarDecl, LoopCtx, SwitchCtx);

    #[derive(Debug, Eq)]
    struct ExprSide(LValue, RValue);

        fn advanceDesigList(ds: Vec<CDesignator>) -> Vec<CDesignator> {
        drop(1)(dropWhile(((not . matchDesignator(d))), ds))
    }

    fn analyseAST((CTranslUnit(decls, _file_node)): m) -> m {
        {

            mapRecoverM_(analyseExt, decls);
            >>=(getDefTable, Lambda((not((inFileScope(dt)))))(error("Internal Error: Not in filescope after analysis".to_string())));
            liftM(globalDefs, getDefTable);
            
        }
    }

    fn analyseExt(__0: m) -> m {
        match (__0) {
            <todo> => { handleAsmBlock(asm) },
            <todo> => { analyseFunDef(fundef) },
            <todo> => { analyseDecl(False, decl) },
        }
    }

    fn analyseFunDef((CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info)): m) -> m {
        {

            let var_decl_info = analyseVarDecl'(True, declspecs, declr, oldstyle_decls, Nothing);
            Let([Assign([Span([Parens([Span([Ref(Ident("VarDeclInfo")), Ref(Ident("name")), Ref(Ident("is_inline")), Ref(Ident("storage_spec")), Ref(Ident("attrs")), Ref(Ident("ty")), Ref(Ident("declr_node"))])])])], Span([Ref(Ident("var_decl_info"))]))], []);
            when((isNoName(name)))(astError(node_info, "NoName in analyseFunDef".to_string()));
            Let([Assign([Span([Ref(Ident("ident"))])], Span([Ref(Ident("identOfVarName")), Ref(Ident("name"))]))], []);
            let ty' = improveFunDefType(ty);
            let fun_storage = computeFunDefStorage(ident, storage_spec);
            Let([Assign([Span([Ref(Ident("var_decl"))])], Span([Ref(Ident("VarDecl")), Ref(Ident("name")), Parens([Span([Ref(Ident("DeclAttrs")), Ref(Ident("is_inline")), Ref(Ident("fun_storage")), Ref(Ident("attrs"))])]), Ref(Ident("ty\'"))]))], []);
            handleVarDecl(False, (Decl(var_decl, node_info)));
            let stmt' = analyseFunctionBody(node_info, var_decl, stmt);
            handleFunDef(ident, (FunDef(var_decl, stmt', node_info)));
            
        }
    }

    fn analyseFunctionBody(__0: m) -> m {
        match (__0, __1, __2, __3, __4) {
            <todo> => { {

                enterFunctionScope;
                mapM_(((withDefTable . defineLabel)), (++(localLabels, getLabels(s))));
                defineParams(node_info, decl);
                mapM_((tBlockItem(vec![FunCtx(decl)])), items);
                leaveFunctionScope;
                return(s)
            } },
            <todo> => { astError((nodeInfo(s)), "Function body is no compound statement".to_string()) },
        }
    }

    fn analyseTypeDef(handle_sue_def: m) -> m {
        {

            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl'(handle_sue_def, declspecs, declr, vec![], Nothing);
            checkValidTypeDef(is_inline, storage_spec, attrs);
            when((isNoName(name)))(astError(node_info, "NoName in analyseTypeDef".to_string()));
            Let([Assign([Span([Ref(Ident("ident"))])], Span([Ref(Ident("identOfVarName")), Ref(Ident("name"))]))], []);
            handleTypeDef((TypeDef(ident, ty, attrs, node_info)));
            
        }
    }

    fn builtinType(__0: MonadTrav) -> MonadTrav {
        match (__0) {
            <todo> => { analyseTypeDecl(d) },
            <todo> => { return(size_tType) },
            <todo> => { return(boolType) },
        }
    }

    fn checkGuard(c: MonadTrav) -> MonadTrav {
        >>=(tExpr(c, RValue, e), checkScalar'((nodeInfo(e))))
    }

    fn checkInits(__0: MonadTrav) -> MonadTrav {
        match (__0, __1, __2) {
            <todo> => { return(()) },
            <todo> => { {

                let (dds', ds') = match (dds, ds) {
            ([], []) => { typeError((nodeInfo(i)), "excess elements in initializer".to_string()) },
            (dd'(:, rest), []) => { return((rest, vec![dd'])) },
            (_, d(:, _)) => { return((advanceDesigList(dds, d), ds)) },
        };
                let t' = tDesignator(t, ds');
                tInit(t', i);
                checkInits(t, dds', is)
            } },
        }
    }

    fn complexBaseType(ni: MonadTrav) -> MonadTrav {
        {

            let t = tExpr(c, side, e);
            match canonicalType(t) {
        DirectType, TyComplex(ft), quals, attrs => { return(DirectType((TyFloating(ft)), quals, attrs)) },
        _ => { typeError(ni)(++("expected complex type, got: ".to_string(), pType(t))) },
    }
        }
    }

    fn computeFunDefStorage(__0: m) -> m {
        match (__0, __1) {
            <todo> => { return(FunLinkage(InternalLinkage)) },
            <todo> => { {

                let obj_opt = lookupObject(ident);
                Let([Assign([Span([Ref(Ident("defaultSpec"))])], Span([Ref(Ident("FunLinkage")), Ref(Ident("ExternalLinkage"))]))], []);
                match other_spec {
        NoStorageSpec => { return(maybe(defaultSpec, declStorage, obj_opt)) },
        ExternSpec(False) => { return(maybe(defaultSpec, declStorage, obj_opt)) },
        bad_spec => { throwTravError(badSpecifierError((nodeInfo(ident)))(++("unexpected function storage specifier (only static or extern is allowed)".to_string(), show(bad_spec)))) },
    }
            } },
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
                Nothing => { astError(ni, "expecting complete function type in function definition".to_string()) },
                Just, params => { mapM_(handleParamDecl, params) },
            }
    }

    fn enclosingFunctionType(__0: Maybe) -> Maybe {
        match (__0) {
            <todo> => { Nothing },
            <todo> => { Just(declType(vd)) },
            <todo> => { enclosingFunctionType(cs) },
        }
    }

    fn extFunProto((VarDeclInfo(var_name, is_inline, storage_spec, attrs, ty, node_info)): m) -> m {
        {

            when((isNoName(var_name)))(astError(node_info, "NoName in extFunProto".to_string()));
            let old_fun = lookupObject((identOfVarName(var_name)));
            checkValidSpecs;
            Let([Assign([Span([Ref(Ident("decl"))])], Span([Ref(Ident("VarDecl")), Ref(Ident("var_name")), Parens([Span([Ref(Ident("DeclAttrs")), Ref(Ident("is_inline")), Parens([Span([Ref(Ident("funDeclLinkage")), Ref(Ident("old_fun"))])]), Ref(Ident("attrs"))])]), Ref(Ident("ty"))]))], []);
            handleVarDecl(False, (Decl(decl, node_info)));
            enterPrototypeScope;
            maybe((return(())), (mapM_(handleParamDecl)), (getParams(ty)));
            leavePrototypeScope
        }
    }

    fn extVarDecl((VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info)): m) -> m {
        {

            when((isNoName(var_name)))(astError(node_info, "NoName in extVarDecl".to_string()));
            let (storage, is_def) = globalStorage(storage_spec);
            Let([Assign([Span([Ref(Ident("vardecl"))])], Span([Ref(Ident("VarDecl")), Ref(Ident("var_name")), Parens([Span([Ref(Ident("DeclAttrs")), Ref(Ident("is_inline")), Ref(Ident("storage")), Ref(Ident("attrs"))])]), Ref(Ident("typ"))]))], []);
            if(is_def, then, handleObjectDef, False, ident)(ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, False)(Decl(vardecl, node_info)))
        }
    }

    fn getParams(__0: Maybe) -> Maybe {
        match (__0) {
            <todo> => { Just(params) },
            <todo> => { Nothing },
        }
    }

    fn hasTypeDef(declspecs: Maybe) -> Maybe {
        match foldr(hasTypeDefSpec, (False, vec![]), declspecs) {
                (True, specs') => { Just(specs') },
                (False, _) => { Nothing },
            }
    }

    fn inLoop(c: Bool) -> Bool {
        any(isLoop, c)
    }

    fn inSwitch(c: Bool) -> Bool {
        any(isSwitch, c)
    }

    fn localVarDecl((VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info)): m) -> m {
        {

            when((isNoName(var_name)))(astError(node_info, "NoName in localVarDecl".to_string()));
            let (storage, is_def) = localStorage(storage_spec);
            Let([Assign([Span([Ref(Ident("vardecl"))])], Span([Ref(Ident("VarDecl")), Ref(Ident("var_name")), Parens([Span([Ref(Ident("DeclAttrs")), Ref(Ident("is_inline")), Ref(Ident("storage")), Ref(Ident("attrs"))])]), Ref(Ident("typ"))]))], []);
            if(is_def, then, handleObjectDef, True, ident, (ObjDef(vardecl, init_opt, node_info)), else, handleVarDecl, True, (Decl(vardecl, node_info)))
        }
    }

    fn matchDesignator(__0: Bool) -> Bool {
        match (__0, __1) {
            <todo> => { ==(m1, m2) },
            <todo> => { True },
        }
    }

    fn tBlockItem(__0: MonadTrav) -> MonadTrav {
        match (__0, __1) {
            <todo> => { tStmt(c, s) },
            <todo> => { >>(analyseDecl(True, d), return(voidType)) },
            <todo> => { >>(analyseFunDef(fd), return(voidType)) },
        }
    }

    fn tDesignator(__0: MonadTrav) -> MonadTrav {
        match (__0, __1) {
            <todo> => { {

                >>=(tExpr(vec![], RValue, e), checkIntegral'(ni));
                tDesignator(bt, ds)
            } },
            <todo> => { {

                >>=(tExpr(vec![], RValue, e1), checkIntegral'(ni));
                >>=(tExpr(vec![], RValue, e2), checkIntegral'(ni));
                tDesignator(bt, ds)
            } },
            <todo> => { typeError((nodeInfo(d)), "member designator in array initializer".to_string()) },
            <todo> => { {

                let mt = fieldType(ni, m, t);
                tDesignator((canonicalType(mt)), ds)
            } },
            <todo> => { typeError((nodeInfo(d)), "array designator in compound initializer".to_string()) },
            <todo> => { return(t) },
        }
    }

    fn tExpr(c: MonadTrav) -> MonadTrav {
        match nameOfNode((nodeInfo(e))) {
                Just, n => { {

                    let dt = getDefTable;
                    match lookupType(dt, n) {
        Just, t => { return(t) },
        Nothing => { {

            let t = tExpr'(c, side, e);
            withDefTable((Lambda))
        } },
    }
                } },
                Nothing => { tExpr'(c, side, e) },
            }
    }

    fn tExpr'(__0: MonadTrav) -> MonadTrav {
        match (__0, __1, __2) {
            <todo> => { {

                when((==(side, LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);
                let rt = tExpr(c, RValue, re);
                binopType'(ni, op, lt, rt)
            } },
            <todo> => { {

                when((==(side, LValue)))(typeError(ni, "address-of operator as lvalue".to_string()));
                match e {
        CCompoundLit, _, _, _ => { liftM(simplePtr, tExpr(c, RValue, e)) },
        CVar, i, _ => { >>=(lookupObject(i), (typeErrorOnLeft(ni) . maybe((notFound(i)), varAddrType))) },
        _ => { liftM(simplePtr, tExpr(c, LValue, e)) },
    }
            } },
            <todo> => { >>=(tExpr(c, RValue, e), ((typeErrorOnLeft(ni) . derefType))) },
            <todo> => { {

                let t = tExpr(c, RValue, e);
                checkIntegral'(ni, t);
                return(t)
            } },
            <todo> => { {

                when((==(side, LValue)))(typeError(ni, "logical negation used as lvalue".to_string()));
                >>=(tExpr(c, RValue, e), checkScalar'(ni));
                return(boolType)
            } },
            <todo> => { tExpr(c, (if(isEffectfulOp, op, then, LValue, else, side)), e) },
            <todo> => { {

                let bt = tExpr(c, RValue, b);
                let it = tExpr(c, RValue, i);
                let addrTy = binopType'(ni, CAddOp, bt, it);
                typeErrorOnLeft(ni)(derefType(addrTy))
            } },
            <todo> => { {

                let t1 = tExpr(c, RValue, e1);
                checkScalar'((nodeInfo(e1)), t1);
                let t3 = tExpr(c, side, e3);
                match me2 {
        Just, e2 => { {

            let t2 = tExpr(c, side, e2);
            conditionalType'(ni, t2, t3)
        } },
        Nothing => { conditionalType'(ni, t1, t3) },
    }
            } },
            <todo> => { {

                let t = tExpr(c, RValue, e);
                let bt = if(deref, then, typeErrorOnLeft, ni, (derefType(t)), else, return, t);
                fieldType(ni, m, bt)
            } },
            <todo> => { >>=(mapM((tExpr(c, side)), es), (return . last)) },
            <todo> => { {

                let dt = analyseTypeDecl(d);
                let et = tExpr(c, side, e);
                typeErrorOnLeft(ni)(castCompatible(dt, et));
                return(dt)
            } },
            <todo> => { {

                when((==(side, LValue)))(typeError(ni, "sizeof as lvalue".to_string()));
                tExpr(c, RValue, e);
                return(size_tType)
            } },
            <todo> => { {

                when((==(side, LValue)))(typeError(ni, "alignof as lvalue".to_string()));
                tExpr(c, RValue, e);
                return(size_tType)
            } },
            <todo> => { complexBaseType(ni, c, side, e) },
            <todo> => { complexBaseType(ni, c, side, e) },
            <todo> => { {

                when((==(side, LValue)))(typeError(ni, "label address as lvalue".to_string()));
                return(PtrType(voidType, noTypeQuals, vec![]))
            } },
            <todo> => { {

                when((==(side, LValue)))(typeError(ni, "compound literal as lvalue".to_string()));
                let lt = analyseTypeDecl(d);
                tInitList(ni, (canonicalType(lt)), initList);
                return(lt)
            } },
            <todo> => { return(size_tType) },
            <todo> => { return(size_tType) },
            <todo> => { typeError(ni, "alignoftype as lvalue".to_string()) },
            <todo> => { typeError(ni, "sizeoftype as lvalue".to_string()) },
            <todo> => { >>=(lookupObject(i), maybe((typeErrorOnLeft(ni)(notFound(i))), ((return . declType)))) },
            <todo> => { constType(c) },
            <todo> => { builtinType(b) },
            <todo> => { {

                Let([Assign([Span([Ref(Ident("defType"))])], Span([Ref(Ident("FunctionType")), Parens([Span([Ref(Ident("FunTypeIncomplete")), Parens([Span([Ref(Ident("DirectType")), Parens([Span([Ref(Ident("TyIntegral")), Ref(Ident("TyInt"))])]), Ref(Ident("noTypeQuals")), Ref(Ident("noAttributes"))])])])]), Ref(Ident("noAttributes"))])), Assign([Span([Ref(Ident("fallback")), Ref(Ident("i"))])], Span([Do([Expression(Span([Ref(Ident("warn")), Operator("$"), Ref(Ident("invalidAST")), Ref(Ident("ni")), Operator("$"), Str("unknown function: "), Operator("++"), Ref(Ident("identToString")), Ref(Ident("i"))]), []), Expression(Span([Ref(Ident("return")), Ref(Ident("defType"))]), [])], [])]))], []);
                let t = match fe {
            CVar, i, _ => { >>=(lookupObject(i), maybe((fallback(i)), (const(tExpr(c, RValue, fe))))) },
            _ => { tExpr(c, RValue, fe) },
        };
                let atys = mapM((tExpr(c, RValue)), args);
                match canonicalType(t) {
        PtrType, FunctionType(FunType(rt, pdecls, varargs), _), _, _ => { {

            Let([Assign([Span([Ref(Ident("ptys"))])], Span([Ref(Ident("map")), Ref(Ident("declType")), Ref(Ident("pdecls"))]))], []);
            mapM_(checkArg)(zip3(ptys, atys, args));
            unless(varargs)(when((/=(length(atys), length(ptys))))(typeError(ni, "incorrect number of arguments".to_string())));
            return(canonicalType(rt))
        } },
        PtrType, FunctionType(FunTypeIncomplete(rt), _), _, _ => { {

            return(canonicalType(rt))
        } },
        _ => { typeError(ni)(++("attempt to call non-function of type ".to_string(), pType(t))) },
    }
            } },
            <todo> => { {

                let lt = tExpr(c, LValue, le);
                let rt = tExpr(c, RValue, re);
                when((constant(typeQuals(lt))))(typeError(ni)(++("assignment to lvalue with `constant\' qualifier: ".to_string(), ((render . pretty))(le))));
                match (canonicalType(lt), re) {
    (lt', CConst(CIntConst(i, _))) => if &&(isPointerType(lt'), ==(getCInteger(i), 0)) { return(()) },
        (_, _) => { assignCompatible'(ni, op, lt, rt) },
    };
                return(lt)
            } },
            <todo> => { {

                enterBlockScope;
                mapM_(((withDefTable . defineLabel)), (getLabels(s)));
                let t = tStmt(c, s);
                leaveBlockScope;
                return(t)
            } },
        }
    }

    fn tInit(__0: MonadTrav) -> MonadTrav {
        match (__0, __1, __2, __3) {
            <todo> => { {

                let it = tExpr(vec![], RValue, e);
                assignCompatible'(ni, CAssignOp, t, it);
                return(i)
            } },
            <todo> => { >>(tInitList(ni, (canonicalType(t)), initList), return(i)) },
        }
    }

    fn tInitList(__0: MonadTrav) -> MonadTrav {
        match (__0, __1, __2, __3, __4) {
            <todo> => { >>(tExpr(vec![], RValue, e), return(())) },
            <todo> => { {

                Let([Assign([Span([Ref(Ident("default_ds"))])], Span([Ref(Ident("repeat")), Parens([Span([Ref(Ident("CArrDesig")), Parens([Span([Ref(Ident("CConst")), Parens([Span([Ref(Ident("CIntConst")), Parens([Span([Ref(Ident("cInteger")), Number(0)])]), Ref(Ident("ni"))])])])]), Ref(Ident("ni"))])])]))], []);
                checkInits(t, default_ds, initList)
            } },
            <todo> => { {

                let td = lookupSUE(ni, (sueRef(ctr)));
                let ms = tagMembers(ni, td);
                Let([Assign([Span([Ref(Ident("default_ds"))])], Span([Ref(Ident("map")), Parens([Span([Lambda, Parens([Span([Ref(Ident("fst")), Ref(Ident("m"))])]), Ref(Ident("ni"))])]), Ref(Ident("ms"))]))], []);
                checkInits(t, default_ds, initList)
            } },
            <todo> => { return(()) },
            <todo> => { >>(tInit(t, i), return(())) },
            <todo> => { typeError(ni)(++("initializer list for type: ".to_string(), pType(t))) },
        }
    }

    fn tStmt(__0: MonadTrav) -> MonadTrav {
        match (__0, __1) {
            <todo> => { tStmt(c, s) },
            <todo> => { maybe((return(voidType)), (tExpr(c, RValue)), e) },
            <todo> => { {

                enterBlockScope;
                mapM_(((withDefTable . defineLabel)), ls);
                let t = foldM((const(tBlockItem(c))), voidType, body);
                leaveBlockScope;
                return(t)
            } },
            <todo> => { >>(checkGuard(c, e), >>(tStmt(c, sthen), >>(maybe((return(())), (>>(Lambda(c, s), return(()))), selse), return(voidType)))) },
            <todo> => { >>=(tExpr(c, RValue, e), >>(checkIntegral'(ni), tStmt((:(SwitchCtx, c)), s))) },
            <todo> => { >>(checkGuard(c, e), tStmt((:(LoopCtx, c)), s)) },
            <todo> => { {

                let dt = getDefTable;
                match lookupLabel(l, dt) {
        Just, _ => { return(voidType) },
        Nothing => { typeError(ni)(++("undefined label in goto: ".to_string(), identToString(l))) },
    }
            } },
            <todo> => { {

                unless((inLoop(c)))(astError(ni, "continue statement outside of loop".to_string()));
                return(voidType)
            } },
            <todo> => { {

                unless((||(inLoop(c), inSwitch(c))))(astError(ni, "break statement outside of loop or switch statement".to_string()));
                return(voidType)
            } },
            <todo> => { {

                let t = tExpr(c, RValue, e);
                let rt = match enclosingFunctionType(c) {
            Just, FunctionType(FunType(rt, _, _), _) => { return(rt) },
            Just, FunctionType(FunTypeIncomplete(rt), _) => { return(rt) },
            Just, ft => { astError(ni)(++("bad function type: ".to_string(), pType(ft))) },
            Nothing => { astError(ni, "return statement outside function".to_string()) },
        };
                match (rt, t) {
        (DirectType(TyVoid, _, _), DirectType(TyVoid, _, _)) => { return(()) },
        _ => { assignCompatible'(ni, CAssignOp, rt, t) },
    };
                return(voidType)
            } },
            <todo> => { return(voidType) },
            <todo> => { return(voidType) },
            <todo> => { {

                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                >>=(tExpr(c, RValue, e), checkIntegral'(ni));
                tStmt(c, s)
            } },
            <todo> => { {

                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                >>=(tExpr(c, RValue, e1), checkIntegral'(ni));
                >>=(tExpr(c, RValue, e2), checkIntegral'(ni));
                tStmt(c, s)
            } },
            <todo> => { {

                unless((inSwitch(c)))(astError(ni, "default statement outside of switch statement".to_string()));
                tStmt(c, s)
            } },
            <todo> => { {

                enterBlockScope;
                either((maybe((return(())), checkExpr)), (analyseDecl(True)), i);
                maybe((return(())), (checkGuard(c)), g);
                maybe((return(())), checkExpr, inc);
                tStmt((:(LoopCtx, c)), s);
                leaveBlockScope;
                return(voidType)
            } },
            <todo> => { {

                let t = tExpr(c, RValue, e);
                match t {
        PtrType(_, _, _) => { return(voidType) },
        _ => { typeError(ni, "can\'t goto non-pointer".to_string()) },
    }
            } },
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
            <todo> => { return(voidAlign(md)) },
            <todo> => { return(iAlign(md, it)) },
            <todo> => { return(fAlign(md, ft)) },
            <todo> => { return(fAlign(md, ft)) },
            <todo> => { return(iAlign(md, TyInt)) },
            <todo> => { return(builtinAlign(md, b)) },
            <todo> => { return(ptrAlign(md)) },
            <todo> => { return(ptrAlign(md)) },
            <todo> => { alignofType(md, n, bt) },
            <todo> => { alignofType(md, n, t) },
            <todo> => { astError((nodeInfo(n)))(++("can\'t find alignment of type: ".to_string(), ((render . pretty))(t))) },
        }
    }

    fn boolValue(__0: Maybe) -> Maybe {
        match (__0) {
            <todo> => { Just(/=(getCInteger(i), 0)) },
            <todo> => { Just(/=(getCCharAsInt(c), 0)) },
            <todo> => { Just(True) },
            <todo> => { Nothing },
        }
    }

    fn compSize(md: MonadTrav) -> MonadTrav {
        {

            let dt = getDefTable;
            match lookupTag((sueRef(ctr)), dt) {
        Just, Left(_) => { astError((nodeInfo(ctr)), "composite declared but not defined".to_string()) },
        Just, Right(CompDef(CompType(_, tag, ms, _, ni))) => { {

            Let([Assign([Span([Ref(Ident("ts"))])], Span([Ref(Ident("map")), Ref(Ident("declType")), Ref(Ident("ms"))]))], []);
            let sizes = mapM((sizeofType(md, ni)), ts);
            match tag {
        StructTag => { return(sum(sizes)) },
        UnionTag => { return(maximum(sizes)) },
    }
        } },
        Just, Right(EnumDef(_)) => { return(iSize(md, TyInt)) },
        Nothing => { astError((nodeInfo(ctr)), "unknown composite".to_string()) },
    }
        }
    }

    fn constEval(__0: Map.Map) -> Map.Map {
        match (__0, __1, __2) {
            <todo> => { {

                let e1' = constEval(md, env, e1);
                let me2' = maybe((return(Nothing)), (liftM(Lambda, constEval(md, env, e))), me2);
                let e3' = constEval(md, env, e3);
                match boolValue(e1') {
        Just, True => { return(fromMaybe(e1', me2')) },
        Just, False => { return(e3') },
        Nothing => { return(CCond(e1', me2', e3', ni)) },
    }
            } },
            <todo> => { {

                let e1' = constEval(md, env, e1);
                let e2' = constEval(md, env, e2);
                let t = tExpr(vec![], RValue, e);
                let bytes = liftM(fromIntegral, sizeofType(md, e, t));
                match (intValue(e1'), intValue(e2')) {
        (Just(i1), Just(i2)) => { intExpr(ni, (withWordBytes(bytes, (intOp(op, i1, i2))))) },
        (_, _) => { return(CBinary(op, e1', e2', ni)) },
    }
            } },
            <todo> => { {

                let e' = constEval(md, env, e);
                let t = tExpr(vec![], RValue, e);
                let bytes = liftM(fromIntegral, sizeofType(md, e, t));
                match intValue(e') {
        Just, i => { match intUnOp(op, i) {
                Just, i' => { intExpr(ni, (withWordBytes(bytes, i'))) },
                Nothing => { astError(ni, "invalid unary operator applied to constant".to_string()) },
            } },
        Nothing => { return(CUnary(op, e', ni)) },
    }
            } },
            <todo> => { {

                let e' = constEval(md, env, e);
                let t = analyseTypeDecl(d);
                let bytes = liftM(fromIntegral, sizeofType(md, d, t));
                match intValue(e') {
        Just, i => { intExpr(ni, (withWordBytes(bytes, i))) },
        Nothing => { return(CCast(d, e', ni)) },
    }
            } },
            <todo> => { {

                let t = tExpr(vec![], RValue, e);
                let sz = sizeofType(md, e, t);
                intExpr(ni, sz)
            } },
            <todo> => { {

                let t = analyseTypeDecl(d);
                let sz = sizeofType(md, d, t);
                intExpr(ni, sz)
            } },
            <todo> => { {

                let t = tExpr(vec![], RValue, e);
                let sz = alignofType(md, e, t);
                intExpr(ni, sz)
            } },
            <todo> => { {

                let t = analyseTypeDecl(d);
                let sz = alignofType(md, d, t);
                intExpr(ni, sz)
            } },
            <todo> => { {

                let t = tExpr(vec![], RValue, e);
                match derefTypeDef(t) {
        DirectType, TyEnum(etr), _, _ => { {

            let dt = getDefTable;
            match lookupTag((sueRef(etr)), dt) {
        Just, Right(EnumDef(EnumType(_, es, _, _))) => { {

            let env' = foldM(enumConst, env, es);
            return(fromMaybe(e)(Map.lookup(i, env')))
        } },
        _ => { return(e) },
    }
        } },
        _ => { return(e) },
    }
            } },
            <todo> => { return(e) },
        }
    }

    fn intExpr(n: m) -> m {
        >>=(genName, Lambda(CConst(CIntConst((cInteger(i)), (mkNodeInfo((posOf(n)), name))))))
    }

    fn intOp(__0: Integer) -> Integer {
        match (__0, __1, __2) {
            <todo> => { +(i1, i2) },
            <todo> => { -(i1, i2) },
            <todo> => { *(i1, i2) },
            <todo> => { div(i1, i2) },
            <todo> => { mod(i1, i2) },
            <todo> => { shiftL(i1, fromInteger(i2)) },
            <todo> => { shiftR(i1, fromInteger(i2)) },
            <todo> => { toInteger(fromEnum(<(i1, i2))) },
            <todo> => { toInteger(fromEnum(>(i1, i2))) },
            <todo> => { toInteger(fromEnum(<=(i1, i2))) },
            <todo> => { toInteger(fromEnum(>=(i1, i2))) },
            <todo> => { toInteger(fromEnum(==(i1, i2))) },
            <todo> => { toInteger(fromEnum(/=(i1, i2))) },
            <todo> => { .&.(i1, i2) },
            <todo> => { xor(i1, i2) },
            <todo> => { .|.(i1, i2) },
            <todo> => { toInteger(fromEnum(&&((/=(i1, 0)), (/=(i2, 0))))) },
            <todo> => { toInteger(fromEnum(||((/=(i1, 0)), (/=(i2, 0))))) },
        }
    }

    fn intUnOp(__0: Maybe) -> Maybe {
        match (__0, __1) {
            <todo> => { Just(i) },
            <todo> => { Just(Operator("-")(i)) },
            <todo> => { Just(complement(i)) },
            <todo> => { Just(toInteger(fromEnum(==(i, 0)))) },
            <todo> => { Nothing },
        }
    }

    fn intValue(__0: Maybe) -> Maybe {
        match (__0) {
            <todo> => { Just(getCInteger(i)) },
            <todo> => { Just(getCCharAsInt(c)) },
            <todo> => { Nothing },
        }
    }

    fn sizeofType(__0: m) -> m {
        match (__0, __1, __2) {
            <todo> => { return(voidSize(md)) },
            <todo> => { return(iSize(md, it)) },
            <todo> => { return(fSize(md, ft)) },
            <todo> => { return(*(2, fSize(md, ft))) },
            <todo> => { compSize(md, ctr) },
            <todo> => { return(iSize(md, TyInt)) },
            <todo> => { return(builtinSize(md, b)) },
            <todo> => { return(ptrSize(md)) },
            <todo> => { return(ptrSize(md)) },
            <todo> => { {

                let sz' = constEval(md, Map.empty, sz);
                match sz' {
        CConst, CIntConst(i, _) => { {

            let s = sizeofType(md, n, bt);
            return(*(getCInteger(i), s))
        } },
        _ => { return(ptrSize(md)) },
    }
            } },
            <todo> => { sizeofType(md, n, t) },
            <todo> => { return(ptrSize(md)) },
            <todo> => { astError((nodeInfo(n)))(++("can\'t find size of type: ".to_string(), ((render . pretty))(t))) },
        }
    }

    fn withWordBytes(bytes: Integer) -> Integer {
        rem(n, (shiftL(1, (shiftL(bytes, 3)))))
    }

}

mod Language_C_Analysis_Debug {
        fn globalDeclStats(file_filter: Vec<(String, isize)>) -> Vec<(String, isize)> {
        vec![("Enumeration Constants".to_string(), Map.size(enumerators)), ("Total Object/Function Declarations".to_string(), Map.size(all_decls)), ("Object definitions".to_string(), Map.size(objDefs)), ("Function Definitions".to_string(), Map.size(funDefs)), ("Tag definitions".to_string(), Map.size(tagDefs)), ("TypeDefs".to_string(), Map.size(typeDefs))]
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
    struct StorageSpec(NoStorageSpec, AutoSpec, RegSpec, ThreadSpec, StaticSpec, Bool, ExternSpec, Bool);

    struct VarDeclInfo(VarDeclInfo, VarName, Bool, StorageSpec, Attributes, Type, NodeInfo);

    #[derive(Eq, Ord)]
    struct NumBaseType(NoBaseType, BaseChar, BaseInt, BaseFloat, BaseDouble);

    #[derive(Eq, Ord)]
    struct SignSpec(NoSignSpec, Signed, Unsigned);

    #[derive(Eq, Ord)]
    struct SizeMod(NoSizeMod, ShortMod, LongMod, LongLongMod);

    struct NumTypeSpec(NumTypeSpec, { /* struct def */ });

    struct TypeSpecAnalysis(TSNone, TSVoid, TSBool, TSNum, NumTypeSpec, TSTypeDef, TypeDefRef, TSType, Type, TSNonBasic, CTypeSpec);

        fn analyseVarDecl(handle_sue_def: m) -> m {
        {

            let storage_spec = canonicalStorageSpec(storage_specs);
            let typ = tType(handle_sue_def, node, typequals, canonTySpecs, derived_declrs, oldstyle_params);
            let attrs' = mapM(tAttr, (++(decl_attrs, declr_attrs)));
            let name = mkVarName(node, name_opt, asmname_opt);
            return(VarDeclInfo(name, inline, storage_spec, attrs', typ, node))
        }
    }

    fn analyseVarDecl'(handle_sue_def: m) -> m {
        {

            Let([Assign([Span([Parens([Span([Ref(Ident("storage_specs"))]), Span([Ref(Ident("attrs"))]), Span([Ref(Ident("type_quals"))]), Span([Ref(Ident("type_specs"))]), Span([Ref(Ident("inline"))])])])], Span([Ref(Ident("partitionDeclSpecs")), Ref(Ident("declspecs"))]))], []);
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
            <todo> => { Right((Auto(False))) },
            <todo> => { Right((Auto(True))) },
            <todo> => { (Left . badSpecifierError(node)(++("Bad storage specified for parameter: ".to_string(), show(spec)))) },
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
            <todo> => { return(declr) },
            <todo> => { internalErr("getOnlyDeclr: declaration doesn\'t have a unique declarator".to_string()) },
        }
    }

    fn hasThreadLocalSpec(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { True },
            <todo> => { b },
            <todo> => { b },
            <todo> => { False },
        }
    }

    fn isTypeDef(declspecs: Bool) -> Bool {
        not(null(Dummy))
    }

    fn mergeOldStyle(__0: m) -> m {
        match (__0, __1, __2) {
            <todo> => { return(declrs) },
            <todo> => { match params {
                    Left, list => { {

                        let oldstyle_params' = liftM(concat)(mapM(splitCDecl, oldstyle_params));
                        let param_map = liftM(Map.fromList)(mapM(attachNameOfDecl, oldstyle_params'));
                        let (newstyle_params, param_map') = foldrM(insertParamDecl, (vec![], param_map), list);
                        when((not(Map.null(param_map'))))(astError(node)(++("declarations for parameter(s) ".to_string(), ++(showParamMap(param_map'), " but no such parameter".to_string()))));
                        return((:(CFunDeclr((Right((newstyle_params, False))), attrs, fdnode), dds)))
                    } },
                    Right, _newstyle => { astError(node, "oldstyle parameter list, but newstyle function declaration".to_string()) },
                } },
            <todo> => { astError(node, "oldstyle parameter list, but not function type".to_string()) },
        }
    }

    fn mergeTypeAttributes(node_info: m) -> m {
        match typ {
                DirectType, ty_name, quals', attrs' => { merge(quals', attrs')(mkDirect(ty_name)) },
                PtrType, ty, quals', attrs' => { merge(quals', attrs')(PtrType(ty)) },
                ArrayType, ty, array_sz, quals', attrs' => { merge(quals', attrs')(ArrayType(ty, array_sz)) },
                FunctionType, FunType(return_ty, params, inline), attrs' => { return(FunctionType((FunType(return_ty, params, inline)), (++(attrs', attrs)))) },
                TypeDefType, tdr, quals', attrs' => { merge(quals', attrs')(TypeDefType(tdr)) },
            }
    }

    fn mkVarName(__0: Maybe) -> Maybe {
        match (__0, __1, __2) {
            <todo> => { return(NoName) },
            <todo> => { return(VarName(n, asm)) },
        }
    }

    fn nameOfDecl(d: m) -> m {
        >>=(getOnlyDeclr(d), Lambda)
    }

    fn splitCDecl(decl: m) -> m {
        match declrs {
                [] => { internalErr("splitCDecl applied to empty declaration".to_string()) },
                [declr] => { return(vec![decl]) },
                d1:ds => { Let([Assign([Span([Ref(Ident("declspecs\'"))])], Span([Ref(Ident("map")), Ref(Ident("elideSUEDef")), Ref(Ident("declspecs")), Ref(Ident("in"))]))], [])(return)(:((CDecl(declspecs, vec![d1], node)), Dummy)) },
            }
    }

    fn tArraySize(__0: m) -> m {
        match (__0) {
            <todo> => { return((UnknownArraySize(False))) },
            <todo> => { return((UnknownArraySize(True))) },
            <todo> => { liftM((ArraySize(static)), (return(szexpr))) },
        }
    }

    fn tAttr((CAttr(name, cexpr, node)): m) -> m {
        return(Attr(name, cexpr, node))
    }

    fn tCompType(tag: m) -> m {
        ap(return((CompType(tag, sue_ref))), ap((concatMapM(tMemberDecls, member_decls)), ap((return(attrs)), (return(node)))))
    }

    fn tCompTypeDecl(handle_def: m) -> m {
        {

            let sue_ref = createSUERef(node_info, ident_opt);
            Let([Assign([Span([Ref(Ident("tag\'"))])], Span([Ref(Ident("tTag")), Ref(Ident("tag"))]))], []);
            let attrs' = mapM(tAttr, attrs);
            Let([Assign([Span([Ref(Ident("decl"))])], Span([Ref(Ident("CompTypeRef")), Ref(Ident("sue_ref")), Ref(Ident("tag\'")), Ref(Ident("node_info"))]))], []);
            handleTagDecl((CompDecl(decl)));
            when((handle_def))({

        maybeM(member_decls_opt)(>>=(Lambda(sue_ref, tag', decls, (attrs'), node_info), (handleTagDef.CompDef)))
    });
            return(decl)
        }
    }

    fn tDirectType(handle_sue_def: m) -> m {
        {

            let (quals, attrs) = tTypeQuals(ty_quals);
            Let([Assign([Span([Ref(Ident("baseType")), Ref(Ident("ty_name"))])], Span([Ref(Ident("DirectType")), Ref(Ident("ty_name")), Ref(Ident("quals")), Ref(Ident("attrs"))]))], []);
            match canonTySpec {
        TSNone => { return(baseType((TyIntegral(TyInt)))) },
        TSVoid => { return(baseType(TyVoid)) },
        TSBool => { return(baseType((TyIntegral(TyBool)))) },
        TSNum, tsnum => { {

            let numType = tNumType(tsnum);
            (return . baseType(match numType {
            Left, (floatType, iscomplex) => if iscomplex { TyComplex(floatType) }
otherwise { TyFloating(floatType) },
                Right, intType => { TyIntegral(intType) },
            }))
        } },
        TSTypeDef, tdr => { return(TypeDefType(tdr, quals, attrs)) },
        TSNonBasic, CSUType(su, _tnode) => { liftM(((baseType . TyComp)))(tCompTypeDecl(handle_sue_def, su)) },
        TSNonBasic, CEnumType(enum, _tnode) => { liftM(((baseType . TyEnum)))(tEnumTypeDecl(handle_sue_def, enum)) },
        TSType, t => { mergeTypeAttributes(node, quals, attrs, t) },
        TSNonBasic, _ => { astError(node, "Unexpected typespec".to_string()) },
    }
        }
    }

    fn tEnumType(sue_ref: m) -> m {
        {

            mapM_(handleEnumeratorDef, enumerators');
            return(ty);
            
        }
    }

    fn tMemberDecls(__0: m) -> m {
        match (__0) {
            <todo> => { {

                Let([Assign([Span([Parens([Span([Ref(Ident("storage_specs"))]), Span([Ref(Ident("_attrs"))]), Span([Ref(Ident("typequals"))]), Span([Ref(Ident("typespecs"))]), Span([Ref(Ident("is_inline"))])])])], Span([Ref(Ident("partitionDeclSpecs")), Ref(Ident("declspecs"))]))], []);
                when(is_inline)(astError(node, "member declaration with inline specifier".to_string()));
                let canonTySpecs = canonicalTypeSpec(typespecs);
                let ty = tType(True, node, typequals, canonTySpecs, vec![], vec![]);
                match ty {
        DirectType, TyComp(_), _, _ => { return(vec![MemberDecl((VarDecl(NoName, (DeclAttrs(False, NoStorage, vec![])), ty)), Nothing, node)]) },
        _ => { astError(node, "anonymous member has a non-composite type".to_string()) },
    }
            } },
            <todo> => { mapM((uncurry(tMemberDecl)), (zip((True:repeat(False)), declrs))) },
        }
    }

    fn tNumType((NumTypeSpec(basetype, sgn, sz, iscomplex)): m) -> m {
        match (basetype, sgn, sz) {
            (BaseChar, _, NoSizeMod) => if let Signed = sgn { intType(TySChar) }
let Unsigned = sgn { intType(TyUChar) }
otherwise { intType(TyChar) },
            (intbase, _, NoSizeMod) => if optBase(BaseInt, intbase) { intType(match sgn {
                        Unsigned => { TyUInt },
                        _ => { TyInt },
                    }) },
            (intbase, signed, sizemod) => if optBase(BaseInt, intbase) && optSign(Signed, signed) { intType(match sizemod {
                        ShortMod => { TyShort },
                        LongMod => { TyLong },
                        LongLongMod => { TyLLong },
                        _ => { internalErr("numTypeMapping: unexpected pattern matching error".to_string()) },
                    }) },
            (intbase, Unsigned, sizemod) => if optBase(BaseInt, intbase) { intType(match sizemod {
                        ShortMod => { TyUShort },
                        LongMod => { TyULong },
                        LongLongMod => { TyULLong },
                        _ => { internalErr("numTypeMapping: unexpected pattern matching error".to_string()) },
                    }) },
                (BaseFloat, NoSignSpec, NoSizeMod) => { floatType(TyFloat) },
                (BaseDouble, NoSignSpec, NoSizeMod) => { floatType(TyDouble) },
                (BaseDouble, NoSignSpec, LongMod) => { floatType(TyLDouble) },
                (_, _, _) => { error("Bad AST analysis".to_string()) },
            }
    }

    fn tParamDecl((CDecl(declspecs, declrs, node)): m) -> m {
        {

            let declr = getParamDeclr;
            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl'(True, declspecs, declr, vec![], Nothing);
            when((is_inline))(throwTravError((badSpecifierError(node, "parameter declaration with inline specifier".to_string()))));
            let storage = throwOnLeft(computeParamStorage(node, storage_spec));
            Let([Assign([Span([Ref(Ident("paramDecl"))])], Span([Ref(Ident("mkParamDecl")), Ref(Ident("name")), Ref(Ident("storage")), Ref(Ident("attrs")), Ref(Ident("ty")), Ref(Ident("declr_node"))]))], []);
            return(paramDecl)
        }
    }

    fn tTag(__0: CompTyKind) -> CompTyKind {
        match (__0) {
            <todo> => { StructTag },
            <todo> => { UnionTag },
        }
    }

    fn tType(handle_sue_def: m) -> m {
        >>=(mergeOldStyle(top_node, oldstyle_params, derived_declrs), buildType)
    }

    fn tTypeQuals() -> m {
        foldrM(go, (noTypeQuals, vec![]))
    }

    fn typeDefRef(t_node: m) -> m {
        >>=(lookupTypeDef(name), Lambda((TypeDefRef(name, (Just(ty)), t_node))))
    }

}

mod Language_C_Analysis_DefTable {
    struct TagFwdDecl(CompDecl, CompTypeRef, EnumDecl, EnumTypeRef);

    struct DefTable(DefTable, { /* struct def */ });

    #[derive(Clone, Debug)]
    struct DeclarationStatus(NewDecl, Redeclared, t, KeepDef, t, Shadowed, t, KindMismatch, t);

    #[derive(Eq, Ord)]
    struct TagEntryKind(CompKind, CompTyKind, EnumKind);

        fn compatIdentEntry(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { either((const(True)), (const(False))) },
            <todo> => { either((const(False)))(Lambda) },
        }
    }

    fn compatTagEntry(te1: Bool) -> Bool {
        ==(tagKind(te1), tagKind(te2))
    }

    fn declStatusDescr(__0: DeclarationStatus) -> DeclarationStatus {
        match (__0) {
            <todo> => { "new".to_string() },
            <todo> => { "redeclared".to_string() },
            <todo> => { "keep old".to_string() },
            <todo> => { "shadowed".to_string() },
            <todo> => { "kind mismatch".to_string() },
        }
    }

    fn declareTag(sueref: (DeclarationStatus(TagEntry), DefTable)) -> (DeclarationStatus(TagEntry), DefTable) {
        match lookupTag(sueref, deftbl) {
                Nothing => { (NewDecl, deftbl({
                        tagDecls: fst(defLocal((tagDecls(deftbl)), sueref, (Left(decl))))
                        })) },
            Just, old_def => if ==(tagKind(old_def), tagKind((Left(decl)))) { (KeepDef(old_def), deftbl) }
otherwise { (KindMismatch(old_def), deftbl) },
            }
    }

    fn defRedeclStatus(sameKind: Maybe) -> Maybe {
        match oldDecl {
            Just, def' => if sameKind(def, def') { Redeclared(def') }
otherwise { KindMismatch(def') },
                Nothing => { NewDecl },
            }
    }

    fn defRedeclStatusLocal(sameKind: Maybe) -> Maybe {
        match defRedeclStatus(sameKind, def, oldDecl) {
                NewDecl => { match lookupName(nsm, ident) {
                        Just, shadowed => { Shadowed(shadowed) },
                        Nothing => { NewDecl },
                    } },
                redecl => { redecl },
            }
    }

    fn defineGlobalIdent(ident: (DeclarationStatus(IdentEntry), DefTable)) -> (DeclarationStatus(IdentEntry), DefTable) {
        (defRedeclStatus(compatIdentEntry, (Right(def)), oldDecl), deftbl({
                identDecls: decls'
                }))
    }

    fn defineLabel(ident: (DeclarationStatus(Ident), DefTable)) -> (DeclarationStatus(Ident), DefTable) {
        Let([Assign([Span([Parens([Span([Ref(Ident("labels\'"))]), Span([Ref(Ident("old_label"))])])])], Span([Ref(Ident("defLocal")), Parens([Span([Ref(Ident("labelDefs")), Ref(Ident("deftbl"))])]), Ref(Ident("ident")), Ref(Ident("ident"))]))], [])(in, (maybe(NewDecl, Redeclared, old_label), deftbl({
                labelDefs: labels'
                })))
    }

    fn defineScopedIdent() -> (DeclarationStatus(IdentEntry), DefTable) {
        defineScopedIdentWhen((const(True)))
    }

    fn defineScopedIdentWhen(override_def: (DeclarationStatus(IdentEntry), DefTable)) -> (DeclarationStatus(IdentEntry), DefTable) {
        (redecl_status, deftbl({
                identDecls: decls'
                }))
    }

    fn defineTag(sueref: (DeclarationStatus(TagEntry), DefTable)) -> (DeclarationStatus(TagEntry), DefTable) {
        (redeclStatus, deftbl({
                tagDecls: decls'
                }))
    }

    fn defineTypeDef(ident: (DeclarationStatus(IdentEntry), DefTable)) -> (DeclarationStatus(IdentEntry), DefTable) {
        (defRedeclStatus(compatIdentEntry, (Left(tydef)), oldDecl), deftbl({
                identDecls: decls'
                }))
    }

    fn emptyDefTable() -> DefTable {
        DefTable(nameSpaceMap, nameSpaceMap, nameSpaceMap, nameSpaceMap, IntMap.empty, IntMap.empty)
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
        Map.foldWithKey(insertDecl, (GlobalDecls(e, gtags, e)), (globalNames(identDecls(deftbl))))
    }

    fn identOfTyDecl() -> Ident {
        either(identOfTypeDef, declIdent)
    }

    fn inFileScope(dt: Bool) -> Bool {
        not((||(hasLocalNames((identDecls(dt))), hasLocalNames((labelDefs(dt))))))
    }

    fn insertType(dt: DefTable) -> DefTable {
        dt({
            typeTable: IntMap.insert((nameId(n)), t, (typeTable(dt)))
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
        Let([Assign([Span([Parens([Span([Ref(Ident("decls\'"))]), Span([Ref(Ident("members"))])])])], Span([Ref(Ident("leaveScope")), Parens([Span([Ref(Ident("memberDecls")), Ref(Ident("deftbl"))])])]))], [])(in, Dummy, (map(snd, members)), (deftbl({
                memberDecls: decls'
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
        IntMap.lookup((nameId(n)), (typeTable(dt)))
    }

    fn mergeDefTable((DefTable(i1, t1, l1, m1, r1, tt1)): DefTable) -> DefTable {
        DefTable((mergeNameSpace(i1, i2)), (mergeNameSpace(t1, t2)), (mergeNameSpace(l1, l2)), (mergeNameSpace(m1, m2)), (union(r1, r2)), (union(tt1, tt2)))
    }

    fn tagKind(__0: TagEntryKind) -> TagEntryKind {
        match (__0) {
            <todo> => { CompKind((compTag(cd))) },
            <todo> => { EnumKind },
            <todo> => { CompKind((compTag(cd))) },
            <todo> => { EnumKind },
        }
    }

}

mod Language_C_Analysis_Export {
        fn exportArraySize(__0: CArrSize) -> CArrSize {
        match (__0) {
            <todo> => { CArrSize(static, e) },
            <todo> => { CNoArrSize(complete) },
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
        :((CComplexType(ni)), exportFloatType(ty))
    }

    fn exportDeclAttrs((DeclAttrs(inline, storage, attrs)): Vec<CDeclSpec>) -> Vec<CDeclSpec> {
        ++((if(inline, then, vec![CTypeQual((CInlineQual(ni)))], else, vec![])), ++(map((CStorageSpec), (exportStorage(storage))), map(((CTypeQual . CAttrQual)), (exportAttrs(attrs)))))
    }

    fn exportDeclr(other_specs: (Vec<CDeclSpec>, CDeclr)) -> (Vec<CDeclSpec>, CDeclr) {
        (++(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
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
                TyFloat => { vec![CFloatType(ni)] },
                TyDouble => { vec![CDoubleType(ni)] },
                TyLDouble => { vec![CLongType(ni), CDoubleType(ni)] },
            }
    }

    fn exportIntType(ty: Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        match ty {
                TyBool => { vec![CBoolType(ni)] },
                TyChar => { vec![CCharType(ni)] },
                TySChar => { vec![CSignedType(ni), CCharType(ni)] },
                TyUChar => { vec![CUnsigType(ni), CCharType(ni)] },
                TyShort => { vec![CShortType(ni)] },
                TyUShort => { vec![CUnsigType(ni), CShortType(ni)] },
                TyInt => { vec![CIntType(ni)] },
                TyUInt => { vec![CUnsigType(ni), CIntType(ni)] },
                TyLong => { vec![CLongType(ni)] },
                TyULong => { vec![CUnsigType(ni), CLongType(ni)] },
                TyLLong => { vec![CLongType(ni), CLongType(ni)] },
                TyULLong => { vec![CUnsigType(ni), CLongType(ni), CLongType(ni)] },
            }
    }

    fn exportMemberDecl(__0: CDecl) -> CDecl {
        match (__0) {
            <todo> => { CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info) },
            <todo> => { Let([Assign([Span([Parens([Span([Ref(Ident("specs"))]), Span([Ref(Ident("declarator"))])])])], Span([Ref(Ident("exportVarDecl")), Ref(Ident("vardecl"))]))], [])(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info) },
        }
    }

    fn exportParamDecl(paramdecl: CDecl) -> CDecl {
        Let([Assign([Span([Parens([Span([Ref(Ident("specs"))]), Span([Ref(Ident("declr"))])])])], Span([Ref(Ident("exportVarDecl")), Parens([Span([Ref(Ident("getVarDecl")), Ref(Ident("paramdecl"))])])]))], [])(in, CDecl, specs, vec![(Just(declr), Nothing, Nothing)], (nodeInfo(paramdecl)))
    }

    fn exportSUERef() -> Maybe {
        (Just . (internalIdent . show))
    }

    fn exportStorage(__0: Vec<CStorageSpec>) -> Vec<CStorageSpec> {
        match (__0) {
            <todo> => { vec![] },
            <todo> => { if(reg, then, vec![CRegister(ni)], else, vec![]) },
            <todo> => { threadLocal(thread_local, vec![CStatic(ni)]) },
            <todo> => { threadLocal(thread_local, vec![CExtern(ni)]) },
            <todo> => { error("impossible storage: static without linkage".to_string()) },
            <todo> => { vec![CStatic(ni)] },
            <todo> => { vec![] },
            <todo> => { error("impossible storage: function without linkage".to_string()) },
        }
    }

    fn exportType(ty: (Vec<CDeclSpec>, Vec<CDerivedDeclr>)) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {
        exportTy(vec![], ty)
    }

    fn exportTypeDecl(ty: CDecl) -> CDecl {
        CDecl(declspecs, declrs, ni)
    }

    fn exportTypeDef((TypeDef(ident, ty, attrs, node_info)): CDecl) -> CDecl {
        CDecl((:(CStorageSpec((CTypedef(ni))), declspecs)), vec![declr], node_info)
    }

    fn exportTypeQuals(quals: Vec<CTypeQual>) -> Vec<CTypeQual> {
        mapMaybe(select, vec![(constant, CConstQual(ni)), (volatile, CVolatQual(ni)), (restrict, CRestrQual(ni))])
    }

    fn exportTypeQualsAttrs(tyqs: Vec<CTypeQual>) -> Vec<CTypeQual> {
        (++(exportTypeQuals(tyqs), map(CAttrQual, (exportAttrs(attrs)))))
    }

    fn exportTypeSpec(tyname: Vec<CTypeSpec>) -> Vec<CTypeSpec> {
        match tyname {
                TyVoid => { vec![CVoidType(ni)] },
                TyIntegral, ity => { exportIntType(ity) },
                TyFloating, fty => { exportFloatType(fty) },
                TyComplex, fty => { exportComplexType(fty) },
                TyComp, comp => { exportCompTypeDecl(comp) },
                TyEnum, enum => { exportEnumTypeDecl(enum) },
                TyBuiltin, TyVaList => { vec![CTypeDef((internalIdent("va_list".to_string())), ni)] },
                TyBuiltin, TyAny => { vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)] },
            }
    }

    fn exportVarDecl((VarDecl(name, attrs, ty)): (Vec<CDeclSpec>, CDeclr)) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    }

    fn fromDirectType(__0: TypeName) -> TypeName {
        match (__0) {
            <todo> => { ty },
            <todo> => { maybe((error("undefined typeDef".to_string())), fromDirectType, ref) },
            <todo> => { error("fromDirectType".to_string()) },
        }
    }

    fn ni() -> NodeInfo {
        undefNode
    }

    fn threadLocal(__0: Vec<CStorageSpec>) -> Vec<CStorageSpec> {
        match (__0) {
            <todo> => { id },
            <todo> => { ((CThread(ni))(Operator(":"))) },
        }
    }

}

mod Language_C_Analysis_NameSpaceMap {
    struct NameSpaceMap(NsMap, Map(k, v), Vec<Vec<(k, v)>>);

        fn defGlobal((NsMap(gs, lss)): NameSpaceMap) -> NameSpaceMap {
        (NsMap((Map.insert(ident, def, gs)), lss), Map.lookup(ident, gs))
    }

    fn defLocal(__0: NameSpaceMap) -> NameSpaceMap {
        match (__0, __1, __2, __3, __4) {
            <todo> => { defGlobal(ns, ident, def) },
            <todo> => { (NsMap(gs, (:((:((ident, def), ls)), lss))), Prelude.lookup(ident, ls)) },
        }
    }

    fn enterNewScope((NsMap(gs, lss)): NameSpaceMap) -> NameSpaceMap {
        NsMap(gs, (:(vec![], lss)))
    }

    fn globalNames((NsMap(g, _)): NameSpaceMap) -> NameSpaceMap {
        g
    }

    fn hasLocalNames((NsMap(_, l)): NameSpaceMap) -> NameSpaceMap {
        not((null(l)))
    }

    fn leaveScope(__0: NameSpaceMap) -> NameSpaceMap {
        match (__0) {
            <todo> => { error("NsMaps.leaveScope: No local scope!".to_string()) },
            <todo> => { (NsMap(gs, lss), ls) },
        }
    }

    fn localNames((NsMap(_, l)): NameSpaceMap) -> NameSpaceMap {
        l
    }

    fn lookupGlobal((NsMap(gs, _)): NameSpaceMap) -> NameSpaceMap {
        Map.lookup(ident, gs)
    }

    fn lookupInnermostScope(nsm: NameSpaceMap) -> NameSpaceMap {
        match localDefs {
                ls(:, _lss) => { Prelude.lookup(ident, ls) },
                [] => { lookupGlobal(nsm, ident) },
            }
    }

    fn lookupName(ns: NameSpaceMap) -> NameSpaceMap {
        match (lookupLocal(localDefs)) {
                Nothing => { lookupGlobal(ns, ident) },
                Just, def => { Just(def) },
            }
    }

    fn mergeNameSpace((NsMap(global1, local1)): NameSpaceMap) -> NameSpaceMap {
        NsMap((Map.union(global1, global2)), (localUnion(local1, local2)))
    }

    fn nameSpaceMap() -> NameSpaceMap {
        NsMap(Map.empty, vec![])
    }

    fn nsMapToList((NsMap(gs, lss)): NameSpaceMap) -> NameSpaceMap {
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
        ++(ident, " redefined".to_string())
    }

    fn redefErrReason(__0: String) -> String {
        match (__0) {
            <todo> => { ++("duplicate definition of ".to_string(), ident) },
            <todo> => { ++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string())) },
            <todo> => { ++(ident, " previously declared as a different kind of symbol".to_string()) },
            <todo> => { ++(ident, " previously declared with different linkage".to_string()) },
            <todo> => { ++(ident, " previously declared without linkage".to_string()) },
        }
    }

    fn redefErrorInfo(lvl: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (++(vec![redefErrReason(info)], prevDeclMsg(old_node))))
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
    struct TagDef(CompDef, CompType, EnumDef, EnumType);

    #[derive(Clone, Debug)]
    struct IdentDecl(Declaration, Decl, ObjectDef, ObjDef, FunctionDef, FunDef, EnumeratorDef, Enumerator);

    struct GlobalDecls(GlobalDecls, { /* struct def */ });

    struct DeclEvent(TagEvent, TagDef, DeclEvent, IdentDecl, ParamEvent, ParamDecl, LocalEvent, IdentDecl, TypeDefEvent, TypeDef, AsmEvent, AsmBlock);

    #[derive(Clone, Debug)]
    struct Decl(Decl, VarDecl, NodeInfo);

    #[derive(Clone, Debug)]
    struct ObjDef(ObjDef, VarDecl, Maybe(Initializer), NodeInfo);

    #[derive(Clone, Debug)]
    struct FunDef(FunDef, VarDecl, Stmt, NodeInfo);

    #[derive(Clone, Debug)]
    struct ParamDecl(ParamDecl, VarDecl, NodeInfo, AbstractParamDecl, VarDecl, NodeInfo);

    #[derive(Clone, Debug)]
    struct MemberDecl(MemberDecl, VarDecl, Maybe(Expr), NodeInfo, AnonBitField, Type, Expr, NodeInfo);

    #[derive(Clone, Debug)]
    struct TypeDef(TypeDef, Ident, Type, Attributes, NodeInfo);

    #[derive(Clone, Debug)]
    struct VarDecl(VarDecl, VarName, DeclAttrs, Type);

    #[derive(Clone, Debug)]
    struct DeclAttrs(DeclAttrs, Bool, Storage, Attributes);

    #[derive(Clone, Debug, Eq, Ord)]
    struct Storage(NoStorage, Auto, Register, Static, Linkage, ThreadLocal, FunLinkage, Linkage);

    #[derive(Clone, Debug, Eq, Ord)]
    struct Linkage(NoLinkage, InternalLinkage, ExternalLinkage);

    #[derive(Clone, Debug)]
    struct Type(DirectType, TypeName, TypeQuals, Attributes, PtrType, Type, TypeQuals, Attributes, ArrayType, Type, ArraySize, TypeQuals, Attributes, FunctionType, FunType, Attributes, TypeDefType, TypeDefRef, TypeQuals, Attributes);

    #[derive(Clone, Debug)]
    struct FunType(FunType, Type, Vec<ParamDecl>, Bool, FunTypeIncomplete, Type);

    #[derive(Clone, Debug)]
    struct ArraySize(UnknownArraySize, Bool, ArraySize, Bool, Expr);

    #[derive(Clone, Debug)]
    struct TypeName(TyVoid, TyIntegral, IntType, TyFloating, FloatType, TyComplex, FloatType, TyComp, CompTypeRef, TyEnum, EnumTypeRef, TyBuiltin, BuiltinType);

    #[derive(Clone, Debug)]
    struct BuiltinType(TyVaList, TyAny);

    #[derive(Clone, Debug)]
    struct TypeDefRef(TypeDefRef, Ident, Maybe(Type), NodeInfo);

    #[derive(Clone, Debug, Eq, Ord)]
    struct IntType(TyBool, TyChar, TySChar, TyUChar, TyShort, TyUShort, TyInt, TyUInt, TyLong, TyULong, TyLLong, TyULLong);

    #[derive(Clone, Debug, Eq, Ord)]
    struct FloatType(TyFloat, TyDouble, TyLDouble);

    #[derive(Clone, Debug)]
    struct CompTypeRef(CompTypeRef, SUERef, CompTyKind, NodeInfo);

    #[derive(Clone, Debug)]
    struct EnumTypeRef(EnumTypeRef, SUERef, NodeInfo);

    #[derive(Clone, Debug)]
    struct CompType(CompType, SUERef, CompTyKind, Vec<MemberDecl>, Attributes, NodeInfo);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CompTyKind(StructTag, UnionTag);

    #[derive(Clone, Debug)]
    struct EnumType(EnumType, SUERef, Vec<Enumerator>, Attributes, NodeInfo);

    #[derive(Clone, Debug)]
    struct Enumerator(Enumerator, Ident, Expr, EnumType, NodeInfo);

    #[derive(Clone, Debug)]
    struct TypeQuals(TypeQuals, { /* struct def */ });

    #[derive(Clone, Debug)]
    struct VarName(VarName, Ident, Maybe(AsmName), NoName);

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
                NoStorage => { undefined },
                Auto, _ => { NoLinkage },
                Static, linkage, _ => { linkage },
                FunLinkage, linkage => { linkage },
            }
    }

    fn declName() -> VarName {
        ((Lambda) . getVarDecl)
    }

    fn declOfDef(def: Decl) -> Decl {
        Let([Assign([Span([Ref(Ident("vd"))])], Span([Ref(Ident("getVarDecl")), Ref(Ident("def")), Ref(Ident("in")), Ref(Ident("Decl")), Ref(Ident("vd")), Parens([Span([Ref(Ident("nodeInfo")), Ref(Ident("def"))])])]))], [])
    }

    fn declStorage(d: Storage) -> Storage {
        match declAttrs(d) {
                DeclAttrs(_, st, _) => { st },
            }
    }

    fn declType() -> Type {
        ((Lambda) . getVarDecl)
    }

    fn emptyGlobalDecls() -> GlobalDecls {
        GlobalDecls(Map.empty, Map.empty, Map.empty)
    }

    fn filterGlobalDecls(decl_filter: GlobalDecls) -> GlobalDecls {
        GlobalDecls({
            gObjs: Map.filter(((decl_filter . DeclEvent)), (gObjs(gmap))),
            gTags: Map.filter(((decl_filter . TagEvent)), (gTags(gmap))),
            gTypeDefs: Map.filter(((decl_filter . TypeDefEvent)), (gTypeDefs(gmap)))
            })
    }

    fn hasLinkage(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { False },
            <todo> => { False },
            <todo> => { True },
        }
    }

    fn identOfTypeDef((TypeDef(ide, _, _, _)): Ident) -> Ident {
        ide
    }

    fn identOfVarName(__0: Ident) -> Ident {
        match (__0) {
            <todo> => { error("identOfVarName: NoName".to_string()) },
            <todo> => { ident },
        }
    }

    fn isExtDecl() -> Bool {
        (hasLinkage . declStorage)
    }

    fn isNoName(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn mergeAttributes() -> Attributes {
        (Operator("++"))
    }

    fn mergeGlobalDecls(gmap1: GlobalDecls) -> GlobalDecls {
        GlobalDecls({
            gObjs: Map.union((gObjs(gmap1)), (gObjs(gmap2))),
            gTags: Map.union((gTags(gmap1)), (gTags(gmap2))),
            gTypeDefs: Map.union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
            })
    }

    fn mergeTypeQuals((TypeQuals(c1, v1, r1)): TypeQuals) -> TypeQuals {
        TypeQuals((&&(c1, c2)), (&&(v1, v2)), (&&(r1, r2)))
    }

    fn noAttributes() -> Attributes {
        vec![]
    }

    fn noTypeQuals() -> TypeQuals {
        TypeQuals(False, False, False)
    }

    fn objKindDescr(__0: String) -> String {
        match (__0) {
            <todo> => { "declaration".to_string() },
            <todo> => { "object definition".to_string() },
            <todo> => { "function definition".to_string() },
            <todo> => { "enumerator definition".to_string() },
        }
    }

    fn splitIdentDecls(include_all: Map) -> Map {
        Map.foldWithKey((if(include_all, then, deal, else, deal')), (Map.empty, (Map.empty, Map.empty, Map.empty)))
    }

    fn typeOfCompDef((CompType(ref, tag, _, _, _)): TypeName) -> TypeName {
        TyComp((CompTypeRef(ref, tag, undefNode)))
    }

    fn typeOfEnumDef((EnumType(ref, _, _, _)): TypeName) -> TypeName {
        TyEnum((EnumTypeRef(ref, undefNode)))
    }

    fn typeOfTagDef(__0: TypeName) -> TypeName {
        match (__0) {
            <todo> => { typeOfCompDef(comptype) },
            <todo> => { typeOfEnumDef(enumtype) },
        }
    }

}

mod Language_C_Analysis_TravMonad {
    struct CLanguage(C89, C99, GNU89, GNU99);

    struct TravOptions(TravOptions, { /* struct def */ });

    struct TravState(TravState, { /* struct def */ });

        fn addRef(use: m) -> m {
        match (nodeInfo(use), nodeInfo(def)) {
                (NodeInfo(_, _, useName), NodeInfo(_, _, defName)) => { withDefTable((Lambda)) },
                (_, _) => { return(()) },
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
            <todo> => { checkVarRedef(decl, status) },
            <todo> => { redefErr((identOfTypeDef(tydef)), LevelError, tydef, old_def, DiffKindRedecl) },
            <todo> => { redefErr((identOfTypeDef(tydef)), LevelError, tydef, old_def, DuplicateDef) },
            <todo> => { return(()) },
        }
    }

    fn checkRedef(subject: m) -> m {
        match redecl_status {
                NewDecl => { return(()) },
                Redeclared, old_def => { throwTravError(redefinition(LevelError, subject, DuplicateDef, (nodeInfo(new_decl)), (nodeInfo(old_def)))) },
                KindMismatch, old_def => { throwTravError(redefinition(LevelError, subject, DiffKindRedecl, (nodeInfo(new_decl)), (nodeInfo(old_def)))) },
                Shadowed, _old_def => { return(()) },
                KeepDef, _old_def => { return(()) },
            }
    }

    fn checkVarRedef(def: m) -> m {
        match redecl {
                KindMismatch, old_def => { redefVarErr(old_def, DiffKindRedecl) },
            KeepDef, Right(old_def) => if not((agreeOnLinkage(def, old_def))) { linkageErr(def, old_def) }
otherwise { throwOnLeft(checkCompatibleTypes(new_ty, (declType(old_def)))) },
            Redeclared, Right(old_def) => if not((agreeOnLinkage(def, old_def))) { linkageErr(def, old_def) }
not((canBeOverwritten(old_def))) { redefVarErr(old_def, DuplicateDef) }
otherwise { throwOnLeft(checkCompatibleTypes(new_ty, (declType(old_def)))) },
                _ => { return(()) },
            }
    }

    fn concatMapM(f: m) -> m {
        (liftM(concat) . mapM(f))
    }

    fn createSUERef(_node_info: Maybe) -> Maybe {
        return(NamedRef(ident))
    }

    fn enterBlockScope() -> m {
        updDefTable((ST.enterBlockScope))
    }

    fn enterDecl(decl: m) -> m {
        {

            Let([Assign([Span([Ref(Ident("def"))])], Span([Ref(Ident("Declaration")), Ref(Ident("decl"))]))], []);
            let redecl = withDefTable(defineScopedIdentWhen(cond, (declIdent(def)), def));
            checkVarRedef(def, redecl);
            return(def)
        }
    }

    fn enterFunctionScope() -> m {
        updDefTable((ST.enterFunctionScope))
    }

    fn enterPrototypeScope() -> m {
        updDefTable((ST.enterBlockScope))
    }

    fn generateName() -> Trav {
        >>=(get, Lambda)
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
        {

            Let([Assign([Span([Ref(Ident("ident"))])], Span([Ref(Ident("declIdent")), Ref(Ident("enumerator"))]))], []);
            let redecl = withDefTable(defineScopedIdent(ident, (EnumeratorDef(enumerator))));
            checkRedef((show(ident)), ident, redecl);
            return(())
        }
    }

    fn handleFunDef(ident: m) -> m {
        {

            Let([Assign([Span([Ref(Ident("def"))])], Span([Ref(Ident("FunctionDef")), Ref(Ident("fun_def"))]))], []);
            let redecl = withDefTable(defineScopedIdentWhen(isDeclaration, ident, def));
            checkVarRedef(def, redecl);
            handleDecl((DeclEvent(def)))
        }
    }

    fn handleObjectDef(local: m) -> m {
        {

            Let([Assign([Span([Ref(Ident("def"))])], Span([Ref(Ident("ObjectDef")), Ref(Ident("obj_def"))]))], []);
            let redecl = withDefTable(defineScopedIdentWhen((Lambda(def, old)), ident, def));
            checkVarRedef(def, redecl);
            handleDecl(((if(local, then, LocalEvent, else, DeclEvent))(def)));
            
        }
    }

    fn handleParamDecl(__0: m) -> m {
        match (__0, __1, __2) {
            <todo> => { handleDecl((ParamEvent(pd))) },
            <todo> => { {

                Let([Assign([Span([Ref(Ident("def"))])], Span([Ref(Ident("ObjectDef")), Parens([Span([Ref(Ident("ObjDef")), Ref(Ident("vardecl")), Ref(Ident("Nothing")), Ref(Ident("node"))])])]))], []);
                let redecl = withDefTable(defineScopedIdent((declIdent(def)), def));
                checkVarRedef(def, redecl);
                handleDecl((ParamEvent(pd)))
            } },
        }
    }

    fn handleTagDecl(decl: m) -> m {
        {

            let redecl = withDefTable(declareTag((sueRef(decl)), decl));
            checkRedef((show(sueRef(decl))), decl, redecl)
        }
    }

    fn handleTagDef(def: m) -> m {
        {

            let redecl = withDefTable(defineTag((sueRef(def)), def));
            checkRedef((show(sueRef(def))), def, redecl);
            handleDecl((TagEvent(def)))
        }
    }

    fn handleTravError(a: m) -> m {
        catchTravError(liftM(Just, a), (>>(Lambda(e), return(Nothing))))
    }

    fn handleTypeDef(typeDef: m) -> m {
        {

            let redecl = withDefTable(defineTypeDef(ident, typeDef));
            checkRedef((show(ident)), typeDef, redecl);
            handleDecl((TypeDefEvent(typeDef)));
            return(())
        }
    }

    fn handleVarDecl(is_local: m) -> m {
        {

            let def = enterDecl(decl, (const(False)));
            handleDecl(((if(is_local, then, LocalEvent, else, DeclEvent))(def)))
        }
    }

    fn initTravState(userst: TravState) -> TravState {
        TravState({
            symbolTable: emptyDefTable,
            rerrors: RList.empty,
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
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn leaveBlockScope() -> m {
        updDefTable((ST.leaveBlockScope))
    }

    fn leaveFunctionScope() -> m {
        updDefTable((ST.leaveFunctionScope))
    }

    fn leavePrototypeScope() -> m {
        updDefTable((ST.leaveBlockScope))
    }

    fn lookupObject(ident: m) -> m {
        {

            let old_decl = liftM((lookupIdent(ident)), getDefTable);
            mapMaybeM(old_decl)(Lambda)
        }
    }

    fn lookupTypeDef(ident: m) -> m {
        >>=(getDefTable, Lambda)
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
        ++(ctx, ++(": Expected ".to_string(), ++(expect, ++(", but found: ".to_string(), found))))
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
                Left, trav_err => { Left(vec![trav_err]) },
            Right, (v, ts) => if hadHardErrors((travErrors(ts))) { Left((travErrors(ts))) }
otherwise { Right((v, ts)) },
            }
    }

    fn runTrav_(t: Trav) -> Trav {
        (fmap(fst) . runTrav(())({

                    let r = t;
                    let es = getErrors;
                    return((r, es))
                }))
    }

    fn throwOnLeft(__0: Either) -> Either {
        match (__0) {
            <todo> => { throwTravError(err) },
            <todo> => { return(v) },
        }
    }

    fn travErrors() -> TravState {
        (RList.reverse . rerrors)
    }

    fn updDefTable(f: m) -> m {
        withDefTable((Lambda))
    }

    fn warn(err: m) -> m {
        recordError((changeErrorLevel(err, LevelWarn)))
    }

    fn withExtDeclHandler(action: Trav) -> Trav {
        {

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
            <todo> => { match (canonicalType(t1), canonicalType(t2)) {
                    (DirectType(TyBuiltin(TyAny), _, _), _) => { return(()) },
                    (_, DirectType(TyBuiltin(TyAny), _, _)) => { return(()) },
                (PtrType(DirectType(TyVoid, _, _), _, _), t2') => if isPointerType(t2') { return(()) },
                (t1', PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1') { return(()) },
                (PtrType(_, _, _), t2') => if isIntegralType(t2') { return(()) },
                (t1', t2') => if &&(isPointerType(t1'), isPointerType(t2')) { {

                    compatible((baseType(t1')), (baseType(t2')))
                } },
                (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) => if ==(sueRef(c1), sueRef(c2)) { return(()) }
otherwise { fail(++("incompatible compound types in assignment: ".to_string(), ++(pType(t1), ++(", ".to_string(), pType(t2))))) },
                    (DirectType(TyBuiltin(TyVaList), _, _), DirectType(TyBuiltin(TyVaList), _, _)) => { return(()) },
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) => if isJust((arithmeticConversion(tn1, tn2))) { return(()) }
otherwise { fail(++("incompatible direct types in assignment: ".to_string(), ++(pType(t1), ++(", ".to_string(), pType(t2))))) },
                    (t1', t2') => { compatible(t1', t2') },
                } },
            <todo> => { >>(binopType((assignBinop(op)), t1, t2), return(())) },
        }
    }

    fn assignCompatible'(ni: MonadCError) -> MonadCError {
        typeErrorOnLeft(ni, (assignCompatible(op, t1, t2)))
    }

    fn binopType(op: Either) -> Either {
        match (op, canonicalType(t1), canonicalType(t2)) {
            (_, t1', t2') => if isLogicOp(op) { >>(checkScalar(t1'), >>(checkScalar(t2'), return(boolType))) }
isCmpOp(op) { match (t1', t2') {
                    (DirectType(tn1, _, _), DirectType(tn2, _, _)) => { match arithmeticConversion(tn1, tn2) {
                            Just, _ => { return(boolType) },
                            Nothing => { fail("incompatible arithmetic types in comparison".to_string()) },
                        } },
                (PtrType(DirectType(TyVoid, _, _), _, _), _) => if isPointerType(t2') { return(boolType) },
                (_, PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1') { return(boolType) },
                (_, _) => if &&(isPointerType(t1'), isIntegralType(t2')) { return(boolType) }
&&(isIntegralType(t1'), isPointerType(t2')) { return(boolType) }
&&(isPointerType(t1'), isPointerType(t2')) { >>(compatible(t1', t2'), return(boolType)) },
                    (_, _) => { fail("incompatible types in comparison".to_string()) },
                } },
                (CSubOp, ArrayType(t1', _, _, _), ArrayType(t2', _, _, _)) => { >>(compatible(t1', t2'), return(ptrDiffType)) },
                (CSubOp, ArrayType(t1', _, _, _), PtrType(t2', _, _)) => { >>(compatible(t1', t2'), return(ptrDiffType)) },
                (CSubOp, PtrType(t1', _, _), ArrayType(t2', _, _, _)) => { >>(compatible(t1', t2'), return(ptrDiffType)) },
                (CSubOp, PtrType(t1', _, _), PtrType(t2', _, _)) => { >>(compatible(t1', t2'), return(ptrDiffType)) },
            (_, PtrType(_, _, _), t2') => if &&(isPtrOp(op), isIntegralType(t2')) { return(t1) }
otherwise { fail(++("invalid pointer operation: ".to_string(), render((pretty(op))))) },
            (CAddOp, t1', PtrType(_, _, _)) => if isIntegralType(t1') { return(t2) },
            (_, ArrayType(_, _, _, _), t2') => if &&(isPtrOp(op), isIntegralType(t2')) { return(t1) }
otherwise { fail(++("invalid pointer operation: ".to_string(), render((pretty(op))))) },
            (CAddOp, t1', ArrayType(_, _, _, _)) => if isIntegralType(t1') { return(t2) },
                (_, DirectType(tn1, q1, a1), DirectType(tn2, q2, a2)) => { {

                    when((isBitOp(op)), (>>(checkIntegral(t1), checkIntegral(t2))));
                    match arithmeticConversion(tn1, tn2) {
        Just, tn => { return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2)))) },
        Nothing => { fail(render(<+>(text("invalid binary operation:".to_string()), <+>(pretty(t1), <+>(pretty(op), pretty(t2)))))) },
    }
                } },
                (_, _, _) => { fail(render(<+>(text("unhandled binary operation:".to_string()), <+>(pretty(t1), <+>(pretty(op), pretty(t2)))))) },
            }
    }

    fn binopType'(ni: MonadCError) -> MonadCError {
        typeErrorOnLeft(ni, (binopType(op, t1, t2)))
    }

    fn castCompatible(t1: Either) -> Either {
        match (canonicalType(t1), canonicalType(t2)) {
                (DirectType(TyVoid, _, _), _) => { return(()) },
                (_, _) => { >>(checkScalar(t1), checkScalar(t2)) },
            }
    }

    fn checkIntegral'(ni: MonadCError) -> MonadCError {
        (typeErrorOnLeft(ni) . checkIntegral)
    }

    fn checkScalar(t: Either) -> Either {
        match canonicalType(t) {
                DirectType, _, _, _ => { return(()) },
                PtrType, _, _, _ => { return(()) },
                ArrayType, _, _, _, _ => { return(()) },
                t' => { fail(++("expected scalar type, got: ".to_string(), ++(pType(t), ++(" (".to_string(), ++(pType(t'), ")".to_string()))))) },
            }
    }

    fn checkScalar'(ni: MonadCError) -> MonadCError {
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
            <todo> => { compositeParamDecl'(ParamDecl, vd1, vd2, ni1) },
            <todo> => { compositeParamDecl'(ParamDecl, vd1, vd2, ni2) },
            <todo> => { compositeParamDecl'(ParamDecl, vd1, vd2, ni1) },
            <todo> => { compositeParamDecl'(AbstractParamDecl, vd1, vd2, ni1) },
        }
    }

    fn compositeParamDecl'(f: Either) -> Either {
        {

            let vd = compositeVarDecl((VarDecl(n1, attrs1, t1')), (VarDecl(n2, attrs2, t2')));
            return(f(vd, dni))
        }
    }

    fn compositeSize(__0: Either) -> Either {
        match (__0, __1) {
            <todo> => { return(s2) },
            <todo> => { return(s1) },
        }
    }

    fn compositeType(__0: Either) -> Either {
        match (__0, __1) {
            <todo> => { return(t1) },
            <todo> => { return(t2) },
            <todo> => { {

                let tn = match (tn1, tn2) {
            (TyVoid, TyVoid) => { return(TyVoid) },
            (TyIntegral(_), TyEnum(_)) => { return(tn1) },
            (TyEnum(_), TyIntegral(_)) => { return(tn2) },
            (TyIntegral(i1), TyIntegral(i2)) => { return(TyIntegral((intConversion(i1, i2)))) },
            (TyFloating(f1), TyFloating(f2)) => { return(TyFloating((floatConversion(f1, f2)))) },
            (TyComplex(f1), TyComplex(f2)) => { return(TyComplex((floatConversion(f1, f2)))) },
            (TyComp(c1), TyComp(c2)) => { {

                when((/=(sueRef(c1), sueRef(c2))))(fail(++("incompatible composite types: ".to_string(), ++(pType(t1), ++(", ".to_string(), pType(t2))))));
                return(tn1)
            } },
            (TyEnum(e1), TyEnum(e2)) => { {

                when((/=(sueRef(e1), sueRef(e2))))(fail(++("incompatible enumeration types: ".to_string(), ++(pType(t1), ++(", ".to_string(), pType(t2))))));
                return(TyEnum(e1))
            } },
            (TyBuiltin(TyVaList), TyBuiltin(TyVaList)) => { return(TyBuiltin(TyVaList)) },
            (TyBuiltin(_), TyBuiltin(_)) => { fail(++("incompatible builtin types: ".to_string(), ++(pType(t1), ++(", ".to_string(), pType(t2))))) },
            (_, _) => { fail(++("incompatible direct types: ".to_string(), ++(pType(t1), ++(", ".to_string(), pType(t2))))) },
        };
                return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2))))
            } },
            <todo> => { return(PtrType(t1, (mergeTypeQuals(q1, q2)), a1)) },
            <todo> => { return(PtrType(t2, (mergeTypeQuals(q1, q2)), a2)) },
            <todo> => { {

                let t = compositeType(t1, t2);
                let s = compositeSize(s1, s2);
                Let([Assign([Span([Ref(Ident("quals"))])], Span([Ref(Ident("mergeTypeQuals")), Ref(Ident("q1")), Ref(Ident("q2"))])), Assign([Span([Ref(Ident("attrs"))])], Span([Ref(Ident("mergeAttrs")), Ref(Ident("a1")), Ref(Ident("a2"))]))], []);
                return((ArrayType(t, s, quals, attrs)))
            } },
            <todo> => { match (tdr1, tdr2) {
                    (TypeDefRef(i1, Nothing, _), TypeDefRef(i2, _, _)) => { doTypeDef(i1, i2, tdr1) },
                    (TypeDefRef(i1, _, _), TypeDefRef(i2, Nothing, _)) => { doTypeDef(i1, i2, tdr2) },
                    (TypeDefRef(_, Just(t1), _), TypeDefRef(_, Just(t2), _)) => { compositeType(t1, t2) },
                } },
            <todo> => { match (ft1, ft2) {
                    (FunType(rt1, args1, varargs1), FunType(rt2, args2, varargs2)) => { {

                        let args = mapM((uncurry(compositeParamDecl)), (zip(args1, args2)));
                        when((/=(varargs1, varargs2)))(fail("incompatible varargs declarations".to_string()));
                        doFunType(rt1, rt2, args, varargs1)
                    } },
                    (FunType(rt1, args1, varargs1), FunTypeIncomplete(rt2)) => { doFunType(rt1, rt2, args1, varargs1) },
                    (FunTypeIncomplete(rt1), FunType(rt2, args2, varargs2)) => { doFunType(rt1, rt2, args2, varargs2) },
                    (FunTypeIncomplete(rt1), FunTypeIncomplete(rt2)) => { {

                        let rt = compositeType(rt1, rt2);
                        return((FunctionType((FunTypeIncomplete(rt)), (mergeAttrs(attrs1, attrs2)))))
                    } },
                } },
            <todo> => { fail(++("incompatible types: ".to_string(), ++(pType(t1), ++(", ".to_string(), pType(t2))))) },
        }
    }

    fn compositeVarDecl((VarDecl(n1, attrs1, t1)): Either) -> Either {
        {

            let t = compositeType(t1, t2);
            return((VarDecl(n1, (compositeDeclAttrs(attrs1, attrs2)), t)))
        }
    }

    fn conditionalType(t1: Either) -> Either {
        match (canonicalType(t1), canonicalType(t2)) {
            (PtrType(DirectType(TyVoid, _, _), _, _), t2') => if isPointerType(t2') { return(t2) },
            (t1', PtrType(DirectType(TyVoid, _, _), _, _)) => if isPointerType(t1') { return(t1) },
                (ArrayType(t1', _, q1, a1), ArrayType(t2', _, q2, a2)) => { {

                    let t = compositeType(t1', t2');
                    return(ArrayType(t, (UnknownArraySize(False)), (mergeTypeQuals(q1, q2)), (mergeAttrs(a1, a2))))
                } },
                (t1'(@, DirectType(tn1, q1, a1)), t2'(@, DirectType(tn2, q2, a2))) => { match arithmeticConversion(tn1, tn2) {
                        Just, tn => { return(DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2)))) },
                        Nothing => { compositeType(t1', t2') },
                    } },
                (t1', t2') => { compositeType(t1', t2') },
            }
    }

    fn conditionalType'(ni: MonadCError) -> MonadCError {
        typeErrorOnLeft(ni)(conditionalType(t1, t2))
    }

    fn constType(__0: m) -> m {
        match (__0) {
            <todo> => { return(DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes)) },
            <todo> => { return(DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes)) },
            <todo> => { return(DirectType((TyIntegral(TyChar)), noTypeQuals, noAttributes)) },
            <todo> => { return(DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes)) },
            <todo> => { return(DirectType((TyFloating((getFloatType(fs)))), noTypeQuals, noAttributes)) },
            <todo> => { {

                let n = genName;
                Let([GuardAssign, Assign([Span([Ref(Ident("ni\'"))])], Span([Ref(Ident("mkNodeInfo")), Parens([Span([Ref(Ident("posOf")), Ref(Ident("ni"))])]), Ref(Ident("n"))])), Assign([Span([Ref(Ident("arraySize"))])], Span([Ref(Ident("ArraySize")), Ref(Ident("True")), Parens([Span([Ref(Ident("CConst")), Parens([Span([Ref(Ident("CIntConst")), Parens([Span([Ref(Ident("cInteger")), Parens([Span([Ref(Ident("toInteger")), Parens([Span([Ref(Ident("length")), Ref(Ident("chars"))])])])])])]), Ref(Ident("ni\'"))])])])])]))], []);
                return(ArrayType((DirectType((TyIntegral(charType)), noTypeQuals, noAttributes)), arraySize, noTypeQuals, vec![]))
            } },
        }
    }

    fn deepTypeAttrs(__0: m) -> m {
        match (__0) {
            <todo> => { liftM((attrs(Operator("++"))), sueAttrs(ni, sue)) },
            <todo> => { liftM((attrs(Operator("++"))), sueAttrs(ni, sue)) },
            <todo> => { return(attrs) },
            <todo> => { liftM((attrs(Operator("++"))), deepTypeAttrs(t)) },
            <todo> => { liftM((attrs(Operator("++"))), deepTypeAttrs(t)) },
            <todo> => { liftM((attrs(Operator("++"))), deepTypeAttrs(t)) },
            <todo> => { liftM((attrs(Operator("++"))), deepTypeAttrs(t)) },
            <todo> => { liftM((attrs(Operator("++"))), typeDefAttrs(ni, i)) },
        }
    }

    fn derefType(__0: Either) -> Either {
        match (__0) {
            <todo> => { return(t) },
            <todo> => { return(t) },
            <todo> => { match canonicalType(t) {
                    PtrType, t', _, _ => { return(t') },
                    ArrayType, t', _, _, _ => { return(t') },
                    _ => { fail(++("dereferencing non-pointer: ".to_string(), pType(t))) },
                } },
        }
    }

    fn expandAnonymous(__0: m) -> m {
        match (__0, __1) {
            <todo> => { >>=(lookupSUE(ni, (sueRef(ctr))), tagMembers(ni)) },
            <todo> => { return(vec![]) },
            <todo> => { return(vec![(n, t)]) },
        }
    }

    fn fieldType(ni: m) -> m {
        match canonicalType(t) {
                DirectType, TyComp(ctr), _, _ => { {

                    let td = lookupSUE(ni, (sueRef(ctr)));
                    let ms = tagMembers(ni, td);
                    match lookup(m, ms) {
        Just, ft => { return(ft) },
        Nothing => { typeError(ni)(++("field not found: ".to_string(), identToString(m))) },
    }
                } },
                _t' => { astError(ni)(++("field of non-composite type: ".to_string(), ++(identToString(m), ++(", ".to_string(), pType(t))))) },
            }
    }

    fn lookupSUE(ni: m) -> m {
        {

            let dt = getDefTable;
            match lookupTag(sue, dt) {
        Just, Right(td) => { return(td) },
        _ => { typeError(ni)(++("unknown composite type: ".to_string(), ((render . pretty))(sue))) },
    }
        }
    }

    fn mergeAttrs() -> Attributes {
        (Operator("++"))
    }

    fn notFound(i: Either) -> Either {
        fail(++("not found: ".to_string(), identToString(i)))
    }

    fn pType() -> String {
        (render . pretty)
    }

    fn sizeEqual(__0: Bool) -> Bool {
        match (__0, __1) {
            <todo> => { ==(i1, i2) },
            <todo> => { ==(nodeInfo(e1), nodeInfo(e2)) },
        }
    }

    fn sueAttrs(ni: m) -> m {
        {

            let dt = getDefTable;
            match lookupTag(sue, dt) {
        Nothing => { astError(ni)(++("SUE not found: ".to_string(), render((pretty(sue))))) },
        Just, Left(_) => { return(vec![]) },
        Just, Right(CompDef(CompType(_, _, _, attrs, _))) => { return(attrs) },
        Just, Right(EnumDef(EnumType(_, _, attrs, _))) => { return(attrs) },
    }
        }
    }

    fn tagMembers(ni: m) -> m {
        match td {
                CompDef, CompType(_, _, ms, _, _) => { getMembers(ms) },
                EnumDef, EnumType(_, es, _, _) => { getMembers(es) },
            }
    }

    fn typeDefAttrs(ni: m) -> m {
        {

            let dt = getDefTable;
            match lookupIdent(i, dt) {
        Nothing => { astError(ni)(++("can\'t find typedef name: ".to_string(), identToString(i))) },
        Just, Left(TypeDef(_, t, attrs, _)) => { liftM((attrs(Operator("++"))), deepTypeAttrs(t)) },
        Just, Right(_) => { astError(ni)(++("not a typedef name: ".to_string(), identToString(i))) },
    }
        }
    }

    fn typeError() -> MonadCError {
        astError
    }

    fn typeErrorOnLeft(__0: Either) -> Either {
        match (__0, __1) {
            <todo> => { typeError(ni, err) },
            <todo> => { return(v) },
        }
    }

    fn varAddrType(d: Either) -> Either {
        {

            match declStorage(d) {
        Auto, True => { fail("address of register variable".to_string()) },
        _ => { return(()) },
    };
            match t {
        ArrayType, _, _, q, a => { return(PtrType(t, q, a)) },
        _ => { return(simplePtr(t)) },
    }
        }
    }

}

mod Language_C_Analysis_TypeConversions {
        fn arithmeticConversion(__0: Maybe) -> Maybe {
        match (__0, __1) {
            <todo> => { Just(TyComplex(floatConversion(t1, t2))) },
            <todo> => { Just(TyComplex(floatConversion(t1, t2))) },
            <todo> => { Just(TyComplex(floatConversion(t1, t2))) },
            <todo> => { Just(t1) },
            <todo> => { Just(t2) },
            <todo> => { Just(TyFloating(floatConversion(t1, t2))) },
            <todo> => { Just(t1) },
            <todo> => { Just(t2) },
            <todo> => { Just(TyIntegral(intConversion(t1, t2))) },
            <todo> => { Just(TyIntegral(TyInt)) },
            <todo> => { Just(t2) },
            <todo> => { Just(t1) },
            <todo> => { Nothing },
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
            <todo> => { t },
            <todo> => { t },
            <todo> => { error("base of non-pointer type".to_string()) },
        }
    }

    fn boolType() -> Type {
        integral(TyInt)
    }

    fn canonicalType(t: Type) -> Type {
        match deepDerefTypeDef(t) {
                FunctionType, ft, attrs => { simplePtr((FunctionType(ft, attrs))) },
                t' => { t' },
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
            <todo> => { PtrType((deepDerefTypeDef(t)), quals, attrs) },
            <todo> => { ArrayType((deepDerefTypeDef(t)), size, quals, attrs) },
            <todo> => { FunctionType((FunType((deepDerefTypeDef(rt)), params, varargs)), attrs) },
            <todo> => { FunctionType((FunTypeIncomplete((deepDerefTypeDef(rt)))), attrs) },
            <todo> => { ((typeAttrsUpd((mergeAttributes(a))) . typeQualsUpd((mergeTypeQuals(q)))))((deepDerefTypeDef(t))) },
            <todo> => { t },
        }
    }

    fn derefTypeDef(__0: Type) -> Type {
        match (__0) {
            <todo> => { ((typeAttrsUpd((mergeAttributes(a))) . typeQualsUpd((mergeTypeQuals(q)))))((derefTypeDef(t))) },
            <todo> => { ty },
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
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn isFunctionType(ty: Bool) -> Bool {
        match ty {
                TypeDefType, TypeDefRef(_, Just(actual_ty), _), _, _ => { isFunctionType(actual_ty) },
                TypeDefType, _, _, _ => { error("isFunctionType: unresolved typeDef".to_string()) },
                FunctionType, _, _ => { True },
                _ => { False },
            }
    }

    fn isIntegralType(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { True },
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn isPointerType(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { True },
            <todo> => { True },
            <todo> => { False },
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
            <todo> => { a },
            <todo> => { a },
            <todo> => { a },
            <todo> => { a },
            <todo> => { a },
            <todo> => { mergeAttributes(a, (typeAttrs(t))) },
        }
    }

    fn typeAttrsUpd(f: Type) -> Type {
        match ty {
                DirectType, ty_name, ty_quals, ty_attrs => { DirectType(ty_name, ty_quals, (f(ty_attrs))) },
                PtrType, ty_inner, ty_quals, ty_attrs => { PtrType(ty_inner, ty_quals, (f(ty_attrs))) },
                ArrayType, ty_inner, sz, ty_quals, ty_attrs => { ArrayType(ty_inner, sz, ty_quals, (f(ty_attrs))) },
                FunctionType, ty_inner, ty_attrs => { FunctionType(ty_inner, (f(ty_attrs))) },
                TypeDefType, ty_ref, ty_quals, ty_attrs => { TypeDefType(ty_ref, ty_quals, (f(ty_attrs))) },
            }
    }

    fn typeQuals(__0: TypeQuals) -> TypeQuals {
        match (__0) {
            <todo> => { q },
            <todo> => { q },
            <todo> => { q },
            <todo> => { noTypeQuals },
            <todo> => { q },
            <todo> => { mergeTypeQuals(q, (typeQuals(t))) },
        }
    }

    fn typeQualsUpd(f: Type) -> Type {
        match ty {
                DirectType, ty_name, ty_quals, ty_attrs => { DirectType(ty_name, (f(ty_quals)), ty_attrs) },
                PtrType, ty_inner, ty_quals, ty_attrs => { PtrType(ty_inner, (f(ty_quals)), ty_attrs) },
                ArrayType, ty_inner, sz, ty_quals, ty_attrs => { ArrayType(ty_inner, sz, (f(ty_quals)), ty_attrs) },
                FunctionType, ty_inner, ty_attrs => { FunctionType(ty_inner, ty_attrs) },
                TypeDefType, ty_ref, ty_quals, ty_attrs => { TypeDefType(ty_ref, (f(ty_quals)), ty_attrs) },
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

    fn internalErr(msg: a) -> a {
        error((++(internalErrPrefix, ++("\n".to_string(), ++(indentLines(msg), "\n".to_string())))))
    }

    fn internalErrPrefix() -> String {
        unlines(vec!["Language.C : Internal Error".to_string(), ++("This is propably a bug, and should be reported at ".to_string(), "http://www.sivity.net/projects/language.c/newticket".to_string())])
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
        ++(header, showMsgLines((if(null, short_msg, then, msgs, else, short_msg:msgs))))
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
    struct SUERef(AnonymousRef, Name, NamedRef, Ident);

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
        ++(identToString(ide), ++(" at ".to_string(), show((nodeInfo(ide)))))
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
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn isInternalIdent((Ident(_, _, nodeinfo)): Bool) -> Bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    fn mkIdent(pos: Ident) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo'(pos, (pos, length(s)), name)))
    }

    fn quad(__0: isize) -> isize {
        match (__0) {
            <todo> => { +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28))) },
            <todo> => { *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1))))) },
            <todo> => { *(ord(c2), +(bits7, ord(c1))) },
            <todo> => { ord(c1) },
            <todo> => { 0 },
        }
    }

}

mod Language_C_Data_InputStream {
        fn countLines() -> isize {
        match () {
            <todo> => { (length . BSC.lines) },
            <todo> => { (length . lines) },
        }
    }

    fn inputStreamEmpty() -> Bool {
        match () {
            <todo> => { BSW.null },
            <todo> => { null },
        }
    }

    fn inputStreamFromString() -> InputStream {
        match () {
            <todo> => { BSC.pack },
            <todo> => { id },
        }
    }

    fn inputStreamToString() -> String {
        match () {
            <todo> => { BSC.unpack },
            <todo> => { id },
        }
    }

    fn readInputStream() -> IO {
        match () {
            <todo> => { BSW.readFile },
            <todo> => { readFile },
        }
    }

    fn takeByte(bs: (Word8, InputStream)) -> (Word8, InputStream) {
        seq(BSW.head(bs), (BSW.head(bs), BSW.tail(bs)))
    }

    fn takeChar(__0: (Char, InputStream)) -> (Char, InputStream) {
        match (__0) {
            <todo> => { seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs))) },
            <todo> => { (head(bs), tail(bs)) },
        }
    }

    fn takeChars(__0: Vec<Char>) -> Vec<Char> {
        match (__0, __1, __2) {
            <todo> => { BSC.unpack(BSC.take(n, bstr)) },
            <todo> => { take(n, str) },
        }
    }

}

mod Language_C_Data_Name {
        fn namesStartingFrom(k: Vec<Name>) -> Vec<Name> {
        vec![Name(k..)]
    }

    fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    }

}

mod Language_C_Data_Node {
    #[derive(Clone, Debug)]
    struct NodeInfo(OnlyPos, Position, PosLength, NodeInfo, Position, PosLength, Name);

        fn eqByName(obj1: CNode) -> CNode {
        ==((nodeInfo(obj1)), (nodeInfo(obj2)))
    }

    fn fileOfNode() -> Maybe {
        (fmap(posFile) . (justIf(isSourcePos) . (posOfNode . nodeInfo)))
    }

    fn getLastTokenPos(__0: PosLength) -> PosLength {
        match (__0) {
            <todo> => { lastTok },
            <todo> => { lastTok },
        }
    }

    fn internalNode() -> NodeInfo {
        undefNode
    }

    fn isUndefNode(_: Bool) -> Bool {
        False
    }

    fn lengthOfNode(ni: Maybe) -> Maybe {
        len
    }

    fn mkNodeInfo(pos: NodeInfo) -> NodeInfo {
        NodeInfo(pos, (nopos, Operator("-")(1)), name)
    }

    fn mkNodeInfo'(pos: NodeInfo) -> NodeInfo {
        NodeInfo(pos, lasttok, name)
    }

    fn mkNodeInfoOnlyPos(pos: NodeInfo) -> NodeInfo {
        OnlyPos(pos, (nopos, Operator("-")(1)))
    }

    fn mkNodeInfoPosLen() -> NodeInfo {
        OnlyPos
    }

    fn nameOfNode(__0: Maybe) -> Maybe {
        match (__0) {
            <todo> => { Nothing },
            <todo> => { Just(name) },
        }
    }

    fn posOfNode(ni: Position) -> Position {
        match ni {
                OnlyPos(pos, _) => { pos },
                NodeInfo(pos, _, _) => { pos },
            }
    }

    fn undefNode() -> NodeInfo {
        OnlyPos(nopos, (nopos, Operator("-")(1)))
    }

}

mod Language_C_Data_Position {
    #[derive(Clone, Debug, Eq, Ord)]
    struct Position(Position, { /* struct def */ }, NoPosition, BuiltinPosition, InternalPosition);

        fn adjustPos(__0: Position) -> Position {
        match (__0, __1, __2) {
            <todo> => { Position(offs, fname, row, 1) },
            <todo> => { p },
        }
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset(__0: Position) -> Position {
        match (__0, __1) {
            <todo> => { Position((+(o, n)), f, r, c) },
            <todo> => { p },
        }
    }

    fn incPos(__0: Position) -> Position {
        match (__0, __1) {
            <todo> => { Position((+(offs, n)), fname, row, (+(col, n))) },
            <todo> => { p },
        }
    }

    fn initPos(file: Position) -> Position {
        Position(0, file, 1, 1)
    }

    fn internalPos() -> Position {
        InternalPosition
    }

    fn isBuiltinPos(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn isInternalPos(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn isNoPos(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn isSourcePos(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn nopos() -> Position {
        NoPosition
    }

    fn position() -> Position {
        Position
    }

    fn retPos(__0: Position) -> Position {
        match (__0) {
            <todo> => { Position((+(offs, 1)), fname, (+(row, 1)), 1) },
            <todo> => { p },
        }
    }

}

mod Language_C_Data_RList {
        fn appendr(xs: Reversed) -> Reversed {
        Reversed((++(ys, List.reverse(xs))))
    }

    fn empty() -> Reversed {
        Reversed(vec![])
    }

    fn rappend((Reversed(xs)): Reversed) -> Reversed {
        Reversed((++(List.reverse(ys), xs)))
    }

    fn rappendr((Reversed(xs)): Reversed) -> Reversed {
        Reversed((++(ys, xs)))
    }

    fn reverse((Reversed(xs)): Reversed) -> Reversed {
        List.reverse(xs)
    }

    fn rmap(f: Reversed) -> Reversed {
        Reversed((map(f, xs)))
    }

    fn singleton(x: Reversed) -> Reversed {
        Reversed(vec![x])
    }

    fn snoc((Reversed(xs)): Reversed) -> Reversed {
        Reversed((:(x, xs)))
    }

    fn viewr(__0: Reversed) -> Reversed {
        match (__0) {
            <todo> => { error("viewr: empty RList".to_string()) },
            <todo> => { (Reversed(xs), x) },
        }
    }

snoc(infixr(5), ())
}

mod Language_C_Data {
    
}

mod Language_C_Parser_Builtin {
        fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}

mod Language_C_Parser_ParserMonad {
    struct ParseResult(POk, PState, a, PFailed, Vec<String>, Position);

    struct PState(PState, { /* struct def */ });

        let (P(m)) = |Operator("thenP"), k| {
        P(Lambda)
    };

    fn addTypedef(ident: P) -> P {
        (P(Lambda(s, {
                    tyidents: Set.insert(ident, tyids)
                    }, ())))
    }

    fn enterScope() -> P {
        P(Lambda(s, {
                scopes: tyids:ss
                }, ()))
    }

    fn execParser((P(parser)): P) -> P {
        match parser(initialState) {
                PFailed, message, errpos => { Left((ParseError((message, errpos)))) },
                POk, st, result => { Right((result, namesupply(st))) },
            }
    }

    fn failP(pos: P) -> P {
        P(Lambda(msg, pos))
    }

    fn getCurrentPosition() -> P {
        P(Lambda(s, pos))
    }

    fn getInput() -> P {
        P(Lambda(s, i))
    }

    fn getLastToken() -> P {
        P(Lambda(s, tok))
    }

    fn getNewName() -> P {
        P(seq(Lambda, POk(s, {
                    namesupply: ns
                    }, n)))
    }

    fn getPos() -> P {
        P(Lambda(s, pos))
    }

    fn getSavedToken() -> P {
        P(Lambda(s, tok))
    }

    fn handleEofToken() -> P {
        P(Lambda(s, {
                savedToken: (prevToken(s))
                }, ()))
    }

    fn isTypeIdent(ident: P) -> P {
        P($!(Lambda(s), Set.member(ident, tyids)))
    }

    fn leaveScope() -> P {
        P(Lambda)
    }

    fn returnP(a: P) -> P {
        P(Lambda(s, a))
    }

    fn setInput(i: P) -> P {
        P(Lambda(s, {
                curInput: i
                }, ()))
    }

    fn setLastToken(__0: P) -> P {
        match (__0) {
            <todo> => { P(Lambda(s, {
                    savedToken: (prevToken(s))
                    }, ())) },
            <todo> => { P(Lambda(s, {
                    prevToken: tok,
                    savedToken: (prevToken(s))
                    }, ())) },
        }
    }

    fn setPos(pos: P) -> P {
        P(Lambda(s, {
                curPos: pos
                }, ()))
    }

    fn shadowTypedef(ident: P) -> P {
        (P(Lambda(s, {
                    tyidents: Set.member(if(ident), Set.delete(tyids(then, ident), tyids(else, tyids)))
                    }, ())))
    }

}

mod Language_C_Parser_Tokens {
    struct CToken(CTokLParen, PosLength, CTokRParen, PosLength, CTokLBracket, PosLength, CTokRBracket, PosLength, CTokArrow, PosLength, CTokDot, PosLength, CTokExclam, PosLength, CTokTilde, PosLength, CTokInc, PosLength, CTokDec, PosLength, CTokPlus, PosLength, CTokMinus, PosLength, CTokStar, PosLength, CTokSlash, PosLength, CTokPercent, PosLength, CTokAmper, PosLength, CTokShiftL, PosLength, CTokShiftR, PosLength, CTokLess, PosLength, CTokLessEq, PosLength, CTokHigh, PosLength, CTokHighEq, PosLength, CTokEqual, PosLength, CTokUnequal, PosLength, CTokHat, PosLength, CTokBar, PosLength, CTokAnd, PosLength, CTokOr, PosLength, CTokQuest, PosLength, CTokColon, PosLength, CTokAssign, PosLength, CTokPlusAss, PosLength, CTokMinusAss, PosLength, CTokStarAss, PosLength, CTokSlashAss, PosLength, CTokPercAss, PosLength, CTokAmpAss, PosLength, CTokHatAss, PosLength, CTokBarAss, PosLength, CTokSLAss, PosLength, CTokSRAss, PosLength, CTokComma, PosLength, CTokSemic, PosLength, CTokLBrace, PosLength, CTokRBrace, PosLength, CTokEllipsis, PosLength, CTokAlignof, PosLength, CTokAsm, PosLength, CTokAuto, PosLength, CTokBreak, PosLength, CTokBool, PosLength, CTokCase, PosLength, CTokChar, PosLength, CTokConst, PosLength, CTokContinue, PosLength, CTokComplex, PosLength, CTokDefault, PosLength, CTokDo, PosLength, CTokDouble, PosLength, CTokElse, PosLength, CTokEnum, PosLength, CTokExtern, PosLength, CTokFloat, PosLength, CTokFor, PosLength, CTokGoto, PosLength, CTokIf, PosLength, CTokInline, PosLength, CTokInt, PosLength, CTokLong, PosLength, CTokLabel, PosLength, CTokRegister, PosLength, CTokRestrict, PosLength, CTokReturn, PosLength, CTokShort, PosLength, CTokSigned, PosLength, CTokSizeof, PosLength, CTokStatic, PosLength, CTokStruct, PosLength, CTokSwitch, PosLength, CTokTypedef, PosLength, CTokTypeof, PosLength, CTokThread, PosLength, CTokUnion, PosLength, CTokUnsigned, PosLength, CTokVoid, PosLength, CTokVolatile, PosLength, CTokWhile, PosLength, CTokCLit, PosLength, CChar, CTokILit, PosLength, CInteger, CTokFLit, PosLength, CFloat, CTokSLit, PosLength, CString, CTokIdent, PosLength, Ident, CTokTyIdent, PosLength, Ident, CTokGnuC, GnuCTok, PosLength, CTokEof);

    struct GnuCTok(GnuCAttrTok, GnuCExtTok, GnuCVaArg, GnuCOffsetof, GnuCTyCompat, GnuCComplexReal, GnuCComplexImag);

        fn posLenOfTok(__0: (Position, isize)) -> (Position, isize) {
        match (__0) {
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { pos },
            <todo> => { error("tokenPos: Eof".to_string()) },
        }
    }

}

mod Language_C_Parser {
        fn execParser_(parser: P) -> P {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}

mod Language_C_Pretty {
        fn attrlistP(__0: Doc) -> Doc {
        match (__0) {
            <todo> => { empty },
            <todo> => { <>(text("__attribute__".to_string()), parens((parens(((hcat . (punctuate(comma) . map(pretty)(attrs)))))))) },
        }
    }

    fn binPrec(__0: isize) -> isize {
        match (__0) {
            <todo> => { 20 },
            <todo> => { 20 },
            <todo> => { 20 },
            <todo> => { 19 },
            <todo> => { 19 },
            <todo> => { 18 },
            <todo> => { 18 },
            <todo> => { 17 },
            <todo> => { 17 },
            <todo> => { 17 },
            <todo> => { 17 },
            <todo> => { 16 },
            <todo> => { 16 },
            <todo> => { 15 },
            <todo> => { 14 },
            <todo> => { 13 },
            <todo> => { 12 },
            <todo> => { 11 },
        }
    }

    fn identP() -> Doc {
        (text . identToString)
    }

    fn ifP(flag: Doc) -> Doc {
        if(flag, then, doc, else, empty)
    }

    fn ii() -> Doc {
        nest(4)
    }

    fn maybeP() -> Maybe {
        maybe(empty)
    }

    fn mlistP(pp: Doc) -> Doc {
        maybeP(pp, (if(null, xs, then, Nothing, else, Just, xs)))
    }

    fn parenPrec(prec: Doc) -> Doc {
        <=(if(prec), prec2(then, t, else, parens, t))
    }

    fn prettyDeclr(show_attrs: Doc) -> Doc {
        <+>(ppDeclr(prec, (reverse(derived_declrs))), <+>(prettyAsmName(asmname), ifP(show_attrs, (attrlistP(cattrs)))))
    }

    fn prettyUsingInclude((CTranslUnit(edecls, _)): Doc) -> Doc {
        $$(includeWarning(headerFiles), (vcat(map((either(includeHeader, pretty)), mappedDecls))))
    }

}

mod Language_C_Syntax_AST {
    #[derive(Clone, Debug)]
    struct CTranslationUnit(CTranslUnit, Vec<CExternalDeclaration(a)>, a);

    #[derive(Clone, Debug)]
    struct CExternalDeclaration(CDeclExt, CDeclaration(a), CFDefExt, CFunctionDef(a), CAsmExt, CStringLiteral(a), a);

    #[derive(Clone, Debug)]
    struct CFunctionDef(CFunDef, Vec<CDeclarationSpecifier(a)>, CDeclarator(a), Vec<CDeclaration(a)>, CStatement(a), a);

    #[derive(Clone, Debug)]
    struct CDeclaration(CDecl, Vec<CDeclarationSpecifier(a)>, Vec<(Maybe(CDeclarator(a)), Maybe(CInitializer(a)), Maybe(CExpression(a)))>, a);

    #[derive(Clone, Debug)]
    struct CDeclarator(CDeclr, Maybe(Ident), Vec<CDerivedDeclarator(a)>, Maybe(CStringLiteral(a)), Vec<CAttribute(a)>, a);

    #[derive(Clone, Debug)]
    struct CDerivedDeclarator(CPtrDeclr, Vec<CTypeQualifier(a)>, a, CArrDeclr, Vec<CTypeQualifier(a)>, CArraySize(a), a, CFunDeclr, Either(Vec<Ident>, (Vec<CDeclaration(a)>, Bool)), Vec<CAttribute(a)>, a);

    #[derive(Clone, Debug)]
    struct CArraySize(CNoArrSize, Bool, CArrSize, Bool, CExpression(a));

    #[derive(Clone, Debug)]
    struct CStatement(CLabel, Ident, CStatement(a), Vec<CAttribute(a)>, a, CCase, CExpression(a), CStatement(a), a, CCases, CExpression(a), CExpression(a), CStatement(a), a, CDefault, CStatement(a), a, CExpr, Maybe(CExpression(a)), a, CCompound, Vec<Ident>, Vec<CCompoundBlockItem(a)>, a, CIf, CExpression(a), CStatement(a), Maybe(CStatement(a)), a, CSwitch, CExpression(a), CStatement(a), a, CWhile, CExpression(a), CStatement(a), Bool, a, CFor, Either(Maybe(CExpression(a)), CDeclaration(a)), Maybe(CExpression(a)), Maybe(CExpression(a)), CStatement(a), a, CGoto, Ident, a, CGotoPtr, CExpression(a), a, CCont, a, CBreak, a, CReturn, Maybe(CExpression(a)), a, CAsm, CAssemblyStatement(a), a);

    #[derive(Clone, Debug)]
    struct CAssemblyStatement(CAsmStmt, Maybe(CTypeQualifier(a)), CStringLiteral(a), Vec<CAssemblyOperand(a)>, Vec<CAssemblyOperand(a)>, Vec<CStringLiteral(a)>, a);

    #[derive(Clone, Debug)]
    struct CAssemblyOperand(CAsmOperand, Maybe(Ident), CStringLiteral(a), CExpression(a), a);

    #[derive(Clone, Debug)]
    struct CCompoundBlockItem(CBlockStmt, CStatement(a), CBlockDecl, CDeclaration(a), CNestedFunDef, CFunctionDef(a));

    #[derive(Clone, Debug)]
    struct CDeclarationSpecifier(CStorageSpec, CStorageSpecifier(a), CTypeSpec, CTypeSpecifier(a), CTypeQual, CTypeQualifier(a));

    #[derive(Clone, Debug, Eq, Ord)]
    struct CStorageSpecifier(CAuto, a, CRegister, a, CStatic, a, CExtern, a, CTypedef, a, CThread, a);

    #[derive(Clone, Debug)]
    struct CTypeSpecifier(CVoidType, a, CCharType, a, CShortType, a, CIntType, a, CLongType, a, CFloatType, a, CDoubleType, a, CSignedType, a, CUnsigType, a, CBoolType, a, CComplexType, a, CSUType, CStructureUnion(a), a, CEnumType, CEnumeration(a), a, CTypeDef, Ident, a, CTypeOfExpr, CExpression(a), a, CTypeOfType, CDeclaration(a), a);

    #[derive(Clone, Debug)]
    struct CTypeQualifier(CConstQual, a, CVolatQual, a, CRestrQual, a, CInlineQual, a, CAttrQual, CAttribute(a));

    #[derive(Clone, Debug)]
    struct CStructureUnion(CStruct, CStructTag, Maybe(Ident), Maybe(Vec<CDeclaration(a)>), Vec<CAttribute(a)>, a);

    #[derive(Clone, Debug, Eq)]
    struct CStructTag(CStructTag, CUnionTag);

    #[derive(Clone, Debug)]
    struct CEnumeration(CEnum, Maybe(Ident), Maybe(Vec<(Ident, Maybe(CExpression(a)))>), Vec<CAttribute(a)>, a);

    #[derive(Clone, Debug)]
    struct CInitializer(CInitExpr, CExpression(a), a, CInitList, CInitializerList(a), a);

    #[derive(Clone, Debug)]
    struct CPartDesignator(CArrDesig, CExpression(a), a, CMemberDesig, Ident, a, CRangeDesig, CExpression(a), CExpression(a), a);

    #[derive(Clone, Debug)]
    struct CAttribute(CAttr, Ident, Vec<CExpression(a)>, a);

    #[derive(Clone, Debug)]
    struct CExpression(CComma, Vec<CExpression(a)>, a, CAssign, CAssignOp, CExpression(a), CExpression(a), a, CCond, CExpression(a), Maybe(CExpression(a)), CExpression(a), a, CBinary, CBinaryOp, CExpression(a), CExpression(a), a, CCast, CDeclaration(a), CExpression(a), a, CUnary, CUnaryOp, CExpression(a), a, CSizeofExpr, CExpression(a), a, CSizeofType, CDeclaration(a), a, CAlignofExpr, CExpression(a), a, CAlignofType, CDeclaration(a), a, CComplexReal, CExpression(a), a, CComplexImag, CExpression(a), a, CIndex, CExpression(a), CExpression(a), a, CCall, CExpression(a), Vec<CExpression(a)>, a, CMember, CExpression(a), Ident, Bool, a, CVar, Ident, a, CConst, CConstant(a), CCompoundLit, CDeclaration(a), CInitializerList(a), a, CStatExpr, CStatement(a), a, CLabAddrExpr, Ident, a, CBuiltinExpr, CBuiltinThing(a));

    #[derive(Clone, Debug)]
    struct CBuiltinThing(CBuiltinVaArg, CExpression(a), CDeclaration(a), a, CBuiltinOffsetOf, CDeclaration(a), Vec<CPartDesignator(a)>, a, CBuiltinTypesCompatible, CDeclaration(a), CDeclaration(a), a);

    #[derive(Clone, Debug)]
    struct CConstant(CIntConst, CInteger, a, CCharConst, CChar, a, CFloatConst, CFloat, a, CStrConst, CString, a);

    #[derive(Clone, Debug)]
    struct CStringLiteral(CStrLit, CString, a);

        fn cstringOfLit((CStrLit(cstr, _)): CStringLiteral) -> CStringLiteral {
        cstr
    }

    fn fmapInitList(_f: CInitializerList(b)) -> CInitializerList(b) {
        map((Lambda))
    }

    fn isSUEDef(__0: CTypeSpecifier) -> CTypeSpecifier {
        match (__0) {
            <todo> => { True },
            <todo> => { True },
            <todo> => { False },
        }
    }

    fn liftStrLit((CStrLit(str, at)): CStringLiteral) -> CStringLiteral {
        CStrConst(str, at)
    }

    fn partitionDeclSpecs() -> (Vec<CStorageSpecifier(a)>, Vec<CAttribute(a)>, Vec<CTypeQualifier(a)>, Vec<CTypeSpecifier(a)>, Bool) {
        foldr(deals, (vec![], vec![], vec![], vec![], False))
    }

}

mod Language_C_Syntax_Constants {
    #[derive(Clone, Debug, Eq, Ord)]
    struct CChar(CChar, Char, Bool, CChars, Vec<Char>, Bool);

    #[derive(Bounded, Clone, Debug, Enum, Eq, Ord)]
    struct CIntRepr(DecRepr, HexRepr, OctalRepr);

    #[derive(Bounded, Clone, Debug, Enum, Eq, Ord)]
    struct CIntFlag(FlagUnsigned, FlagLong, FlagLongLong, FlagImag);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CInteger(CInteger, Integer, CIntRepr, Flags(CIntFlag));

    #[derive(Clone, Debug, Eq, Ord)]
    struct CFloat(CFloat, String);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CString(CString, Vec<Char>, Bool);

        fn _showWideFlag(flag: ShowS) -> ShowS {
        if(flag, then, showString, "L".to_string(), else, id)
    }

    fn cChar(c: CChar) -> CChar {
        CChar(c, False)
    }

    fn cChar_w(c: CChar) -> CChar {
        CChar(c, True)
    }

    fn cChars() -> CChar {
        CChars
    }

    fn cFloat() -> CFloat {
        (CFloat . show)
    }

    fn cInteger(i: CInteger) -> CInteger {
        CInteger(i, DecRepr, noFlags)
    }

    fn cString(str: CString) -> CString {
        CString(str, False)
    }

    fn cString_w(str: CString) -> CString {
        CString(str, True)
    }

    fn clearFlag(flag: Flags) -> Flags {
        Flags(clearBit(k, fromEnum(flag)))
    }

    fn concatCStrings(cs: CString) -> CString {
        CString((concatMap(getCString, cs)), (any(isWideString, cs)))
    }

    fn dQuote(s: ShowS) -> ShowS {
        ++((:('\"', s)), ++("\"".to_string(), t))
    }

    fn escapeCChar('\'': String) -> String {
        "\\\'".to_string()
    }

    fn escapeChar(__0: String) -> String {
        match (__0) {
            <todo> => { "\\\\".to_string() },
            <todo> => { "\\a".to_string() },
            <todo> => { "\\b".to_string() },
            <todo> => { "\\e".to_string() },
            <todo> => { "\\f".to_string() },
            <todo> => { "\\n".to_string() },
            <todo> => { "\\r".to_string() },
            <todo> => { "\\t".to_string() },
            <todo> => { "\\v".to_string() },
        }
    }

    fn getCChar(__0: Vec<Char>) -> Vec<Char> {
        match (__0) {
            <todo> => { vec![c] },
            <todo> => { cs },
        }
    }

    fn getCCharAsInt(__0: Integer) -> Integer {
        match (__0) {
            <todo> => { fromIntegral((fromEnum(c))) },
            <todo> => { error("integer value of multi-character character constants is implementation defined".to_string()) },
        }
    }

    fn getCInteger((CInteger(i, _, _)): Integer) -> Integer {
        i
    }

    fn getCString((CString(str, _)): String) -> String {
        str
    }

    fn head'(__0: a) -> a {
        match (__0, __1) {
            <todo> => { error(err) },
            <todo> => { x },
        }
    }

    fn isAsciiSourceChar(c: Bool) -> Bool {
        &&(isAscii(c), isPrint(c))
    }

    fn isCChar(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { False },
            <todo> => { False },
            <todo> => { False },
            <todo> => { isAsciiSourceChar(c) },
        }
    }

    fn isSChar(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { False },
            <todo> => { False },
            <todo> => { False },
            <todo> => { isAsciiSourceChar(c) },
        }
    }

    fn isWideChar(__0: Bool) -> Bool {
        match (__0) {
            <todo> => { wideFlag },
            <todo> => { wideFlag },
        }
    }

    fn isWideString((CString(_, wideflag)): Bool) -> Bool {
        wideflag
    }

    fn noFlags() -> Flags {
        Flags(0)
    }

    fn readCFloat() -> CFloat {
        CFloat
    }

    fn readCInteger(repr: Either) -> Either {
        match readNum(str) {
                [(n, suffix)] => { mkCInt(n, suffix) },
                parseFailed => { Left(++("Bad Integer literal: ".to_string(), show(parseFailed))) },
            }
    }

    fn sQuote(s: ShowS) -> ShowS {
        ++("\'".to_string(), ++(s, ++("\'".to_string(), t)))
    }

    fn setFlag(flag: Flags) -> Flags {
        Flags(setBit(k, fromEnum(flag)))
    }

    fn showCharConst(c: ShowS) -> ShowS {
        sQuote(escapeCChar(c))
    }

    fn showStringLit() -> ShowS {
        (dQuote . concatMap(showStringChar))
    }

    fn testFlag(flag: Flags) -> Flags {
        testBit(k, fromEnum(flag))
    }

    fn unescapeChar(__0: (Char, String)) -> (Char, String) {
        match (__0) {
            <todo> => { match c {
                    'n' => { ('\n', cs) },
                    't' => { ('\t', cs) },
                    'v' => { ('\u{b}', cs) },
                    'b' => { ('\u{8}', cs) },
                    'r' => { ('\r', cs) },
                    'f' => { ('\u{c}', cs) },
                    'a' => { ('\u{7}', cs) },
                    'e' => { ('\u{1b}', cs) },
                    'E' => { ('\u{1b}', cs) },
                    '\\' => { ('\\', cs) },
                    '?' => { ('?', cs) },
                    '\'' => { ('\'', cs) },
                    '\"' => { ('\"', cs) },
                    'x' => { match head'("bad escape sequence".to_string(), (readHex(cs))) {
                            (i, cs') => { (toEnum(i), cs') },
                        } },
                    _ => { match head'("bad escape sequence".to_string(), (readOct((c:cs)))) {
                            (i, cs') => { (toEnum(i), cs') },
                        } },
                } },
            <todo> => { (c, cs) },
            <todo> => { error("unescape char: empty string".to_string()) },
        }
    }

    fn unescapeString(__0: String) -> String {
        match (__0) {
            <todo> => { vec![] },
            <todo> => { match unescapeChar(cs) {
                    (c, cs') => { :(c, unescapeString(cs')) },
                } },
        }
    }

}

mod Language_C_Syntax_Ops {
    #[derive(Clone, Debug, Eq, Ord)]
    struct CAssignOp(CAssignOp, CMulAssOp, CDivAssOp, CRmdAssOp, CAddAssOp, CSubAssOp, CShlAssOp, CShrAssOp, CAndAssOp, CXorAssOp, COrAssOp);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CBinaryOp(CMulOp, CDivOp, CRmdOp, CAddOp, CSubOp, CShlOp, CShrOp, CLeOp, CGrOp, CLeqOp, CGeqOp, CEqOp, CNeqOp, CAndOp, CXorOp, COrOp, CLndOp, CLorOp);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CUnaryOp(CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp, CAdrOp, CIndOp, CPlusOp, CMinOp, CCompOp, CNegOp);

        fn assignBinop(__0: CBinaryOp) -> CBinaryOp {
        match (__0) {
            <todo> => { error("direct assignment has no binary operator".to_string()) },
            <todo> => { CMulOp },
            <todo> => { CDivOp },
            <todo> => { CRmdOp },
            <todo> => { CAddOp },
            <todo> => { CSubOp },
            <todo> => { CShlOp },
            <todo> => { CShrOp },
            <todo> => { CAndOp },
            <todo> => { CXorOp },
            <todo> => { COrOp },
        }
    }

    fn isBitOp(op: Bool) -> Bool {
        elem(op, vec![CShlOp, CShrOp, CAndOp, COrOp, CXorOp])
    }

    fn isCmpOp(op: Bool) -> Bool {
        elem(op, vec![CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp])
    }

    fn isEffectfulOp(op: Bool) -> Bool {
        elem(op, vec![CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp])
    }

    fn isLogicOp(op: Bool) -> Bool {
        elem(op, vec![CLndOp, CLorOp])
    }

    fn isPtrOp(op: Bool) -> Bool {
        elem(op, vec![CAddOp, CSubOp])
    }

}

mod Language_C_Syntax_Utils {
        fn compoundSubStmts(__0: Vec<CStat>) -> Vec<CStat> {
        match (__0) {
            <todo> => { vec![s] },
            <todo> => { vec![] },
            <todo> => { vec![] },
        }
    }

    fn getLabels(__0: Vec<Ident>) -> Vec<Ident> {
        match (__0) {
            <todo> => { :(l, getLabels(s)) },
            <todo> => { \\(concatMap(((concatMap(getLabels) . compoundSubStmts)), body), ls) },
            <todo> => { concatMap(getLabels, (getSubStmts(stmt))) },
        }
    }

    fn getSubStmts(__0: Vec<CStat>) -> Vec<CStat> {
        match (__0) {
            <todo> => { vec![s] },
            <todo> => { vec![s] },
            <todo> => { vec![s] },
            <todo> => { vec![s] },
            <todo> => { vec![] },
            <todo> => { concatMap(compoundSubStmts, body) },
            <todo> => { maybe(vec![sthen], (Lambda), selse) },
            <todo> => { vec![s] },
            <todo> => { vec![s] },
            <todo> => { vec![s] },
            <todo> => { vec![] },
            <todo> => { vec![] },
            <todo> => { vec![] },
            <todo> => { vec![] },
            <todo> => { vec![] },
            <todo> => { vec![] },
        }
    }

    fn mapBlockItemStmts(__0: CBlockItem) -> CBlockItem {
        match (__0, __1, __2) {
            <todo> => { CBlockStmt((mapSubStmts(stop, f, s))) },
            <todo> => { bi },
        }
    }

    fn mapSubStmts(__0: CStat) -> CStat {
        match (__0, __1, __2) {
            <todo> => { f((CLabel(i, (mapSubStmts(stop, f, s)), attrs, ni))) },
            <todo> => { f((CCase(e, (mapSubStmts(stop, f, s)), ni))) },
            <todo> => { f((CCases(e1, e2, (mapSubStmts(stop, f, s)), ni))) },
            <todo> => { f((CDefault((mapSubStmts(stop, f, s)), ni))) },
            <todo> => { f((CCompound(ls, (map((mapBlockItemStmts(stop, f)), body)), ni))) },
            <todo> => { f((CIf(e, (mapSubStmts(stop, f, sthen)), (maybe(Nothing, ((Just . mapSubStmts(stop, f))), selse)), ni))) },
            <todo> => { f((CSwitch(e, (mapSubStmts(stop, f, s)), ni))) },
            <todo> => { f((CWhile(e, (mapSubStmts(stop, f, s)), isdo, ni))) },
            <todo> => { f((CFor(i, t, a, (mapSubStmts(stop, f, s)), ni))) },
            <todo> => { f(s) },
        }
    }

}

mod Language_C_Syntax {
    
}

mod Language_C_System_GCC {
        fn buildCppArgs((CppArgs(options, extra_args, _tmpdir, input_file, output_file_opt)): Vec<String>) -> Vec<String> {
        ++({

                (concatMap(tOption, options))
            }, ++(outputFileOpt, ++(vec!["-E".to_string(), input_file], extra_args)))
    }

    fn gccParseCPPArgs(args: Either) -> Either {
        match mungeArgs(((Nothing, Nothing, RList.empty), (RList.empty, RList.empty)), args) {
                Left, err => { Left(err) },
                Right, ((Nothing, _, _), _) => { Left("No .c / .hc / .h source file given".to_string()) },
                Right, ((Just(input_file), output_file_opt, cpp_opts), (extra_args, other_args)) => { Right(((rawCppArgs((RList.reverse(extra_args)), input_file))({
                        outputFile: output_file_opt,
                        cppOptions: RList.reverse(cpp_opts)
                        }), RList.reverse(other_args))) },
            }
    }

    fn newGCC() -> GCC {
        GCC
    }

}

mod Language_C_System_Preprocess {
    struct CppOption(IncludeDir, FilePath, Define, String, String, Undefine, String, IncludeFile, FilePath);

    struct CppArgs(CppArgs, { /* struct def */ });

        fn addCppOption(cpp_args: CppArgs) -> CppArgs {
        cpp_args({
            cppOptions: :(opt, (cppOptions(cpp_args)))
            })
    }

    fn addExtraOption(cpp_args: CppArgs) -> CppArgs {
        cpp_args({
            extraOptions: :(extra, (extraOptions(cpp_args)))
            })
    }

    fn cppFile(input_file: CppArgs) -> CppArgs {
        CppArgs({
            cppOptions: vec![],
            extraOptions: vec![],
            cppTmpDir: Nothing,
            inputFile: input_file,
            outputFile: Nothing
            })
    }

    fn isPreprocessed() -> Bool {
        (".i".to_string()(Operator("isSuffixOf")))
    }

    fn mkOutputFile(tmp_dir_opt: Maybe) -> Maybe {
        {

            let tmpDir = getTempDir(tmp_dir_opt);
            mkTmpFile(tmpDir, (getOutputFileName(input_file)))
        }
    }

    fn mkTmpFile(tmp_dir: IO) -> IO {
        {

            let (path, file_handle) = openTempFile(tmp_dir, file_templ);
            hClose(file_handle);
            return(path)
        }
    }

    fn preprocessedExt() -> String {
        ".i".to_string()
    }

    fn rawCppArgs(opts: CppArgs) -> CppArgs {
        CppArgs({
            inputFile: input_file,
            cppOptions: vec![],
            extraOptions: opts,
            outputFile: Nothing,
            cppTmpDir: Nothing
            })
    }

    fn runPreprocessor(cpp: IO) -> IO {
        {

            bracket(getActualOutFile, removeTmpOutFile, invokeCpp);
            
        }
    }

}



fn main() { /* demo */ }
