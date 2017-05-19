mod Language_C_Analysis_AstAnalysis {
    struct StmtCtx(FunCtx, VarDecl, LoopCtx, SwitchCtx);

    #[derive(Debug, Eq)]
    struct ExprSide(LValue, RValue);

    let @(analyseFunctionBody(node_info, decl, s), (CCompound(localLabels, items, _))) = |()| {
        {

            enterFunctionScope;
            mapM_(((withDefTable . defineLabel)), (++(localLabels, getLabels(s))));
            defineParams(node_info, decl);
            mapM_((tBlockItem(vec![FunCtx(decl)])), items);
            leaveFunctionScope;
            return(s)
        }
    };

    let @(tDesignator(t), (DirectType((TyComp(_)), _, _))((:(CMemberDesig(m, ni), ds)))) = |()| {
        {

            let mt = fieldType(ni, m, t);
            tDesignator((canonicalType(mt)), ds)
        }
    };

    let @(tDesignator(t), (DirectType((TyComp(_)), _, _))((:(d, _)))) = |()| {
        typeError((nodeInfo(d)), "array designator in compound initializer".to_string())
    };

    let @(tInit(t, i), (CInitExpr(e, ni))) = |()| {
        {

            let it = tExpr(vec![], RValue, e);
            assignCompatible'(ni, CAssignOp, t, it);
            return(i)
        }
    };

    let @(tInit(t, i), (CInitList(initList, ni))) = |()| {
        >>(tInitList(ni, (canonicalType(t)), initList), return(i))
    };

    let @(tInitList(ni, t), (ArrayType((DirectType((TyIntegral(TyChar)), _, _)), _, _, _))(vec![(vec![], @(CInitExpr(e), (CConst((CStrConst(_, _))))(_)))])) = |()| {
        >>(tExpr(vec![], RValue, e), return(()))
    };

    let @(tInitList(ni, t), (ArrayType(_, _, _, _))(initList)) = |()| {
        {

            Let([Assign([Span([Ref(Ident("default_ds"))])], Span([Ref(Ident("repeat")), Parens([Span([Ref(Ident("CArrDesig")), Parens([Span([Ref(Ident("CConst")), Parens([Span([Ref(Ident("CIntConst")), Parens([Span([Ref(Ident("cInteger")), Number(0)])]), Ref(Ident("ni"))])])])]), Ref(Ident("ni"))])])]))], []);
            checkInits(t, default_ds, initList)
        }
    };

    let @(tInitList(ni, t), (DirectType((TyComp(ctr)), _, _))(initList)) = |()| {
        {

            let td = lookupSUE(ni, (sueRef(ctr)));
            let ms = tagMembers(ni, td);
            Let([Assign([Span([Ref(Ident("default_ds"))])], Span([Ref(Ident("map")), Parens([Span([Lambda, Parens([Span([Ref(Ident("fst")), Ref(Ident("m"))])]), Ref(Ident("ni"))])]), Ref(Ident("ms"))]))], []);
            checkInits(t, default_ds, initList)
        }
    };

    let advanceDesigList(ds, d) = |()| {
        drop(1)(dropWhile(((not . matchDesignator(d))), ds))
    };

    let analyseAST((CTranslUnit(decls, _file_node))) = |()| {
        {

            mapRecoverM_(analyseExt, decls);
            >>=(getDefTable, Lambda((not((inFileScope(dt)))))(error("Internal Error: Not in filescope after analysis".to_string())));
            liftM(globalDefs, getDefTable);
            
        }
    };

    let analyseExt((CAsmExt(asm, _))) = |()| {
        handleAsmBlock(asm)
    };

    let analyseExt((CDeclExt(decl))) = |()| {
        analyseDecl(False, decl)
    };

    let analyseExt((CFDefExt(fundef))) = |()| {
        analyseFunDef(fundef)
    };

    let analyseFunDef((CFunDef(declspecs, declr, oldstyle_decls, stmt, node_info))) = |()| {
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
    };

    let analyseFunctionBody(_, _, s) = |()| {
        astError((nodeInfo(s)), "Function body is no compound statement".to_string())
    };

    let analyseTypeDef(handle_sue_def, declspecs, declr, node_info) = |()| {
        {

            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl'(handle_sue_def, declspecs, declr, vec![], Nothing);
            checkValidTypeDef(is_inline, storage_spec, attrs);
            when((isNoName(name)))(astError(node_info, "NoName in analyseTypeDef".to_string()));
            Let([Assign([Span([Ref(Ident("ident"))])], Span([Ref(Ident("identOfVarName")), Ref(Ident("name"))]))], []);
            handleTypeDef((TypeDef(ident, ty, attrs, node_info)));
            
        }
    };

    let builtinType((CBuiltinOffsetOf(_, _, _))) = |()| {
        return(size_tType)
    };

    let builtinType((CBuiltinTypesCompatible(_, _, _))) = |()| {
        return(boolType)
    };

    let builtinType((CBuiltinVaArg(_, d, _))) = |()| {
        analyseTypeDecl(d)
    };

    let checkGuard(c, e) = |()| {
        >>=(tExpr(c, RValue, e), checkScalar'((nodeInfo(e))))
    };

    let checkInits(_, _, vec![]) = |()| {
        return(())
    };

    let checkInits(t, dds, (:((ds, i), is))) = |()| {
        {

            let (dds', ds') = match (dds, ds) {
            ([], []) => { typeError((nodeInfo(i)), "excess elements in initializer".to_string()) },
            (dd'(:, rest), []) => { return((rest, vec![dd'])) },
            (_, d(:, _)) => { return((advanceDesigList(dds, d), ds)) },
        };
            let t' = tDesignator(t, ds');
            tInit(t', i);
            checkInits(t, dds', is)
        }
    };

    let complexBaseType(ni, c, side, e) = |()| {
        {

            let t = tExpr(c, side, e);
            match canonicalType(t) {
        DirectType, TyComplex(ft), quals, attrs => { return(DirectType((TyFloating(ft)), quals, attrs)) },
        _ => { typeError(ni)(++("expected complex type, got: ".to_string(), pType(t))) },
    }
        }
    };

    let computeFunDefStorage(_, (StaticSpec(b))) = |()| {
        return(FunLinkage(InternalLinkage))
    };

    let computeFunDefStorage(ident, other_spec) = |()| {
        {

            let obj_opt = lookupObject(ident);
            Let([Assign([Span([Ref(Ident("defaultSpec"))])], Span([Ref(Ident("FunLinkage")), Ref(Ident("ExternalLinkage"))]))], []);
            match other_spec {
        NoStorageSpec => { return(maybe(defaultSpec, declStorage, obj_opt)) },
        ExternSpec(False) => { return(maybe(defaultSpec, declStorage, obj_opt)) },
        bad_spec => { throwTravError(badSpecifierError((nodeInfo(ident)))(++("unexpected function storage specifier (only static or extern is allowed)".to_string(), show(bad_spec)))) },
    }
        }
    };

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

    let defineParams(ni, decl) = |()| {
        match (getParams(declType(decl))) {
                Nothing => { astError(ni, "expecting complete function type in function definition".to_string()) },
                Just, params => { mapM_(handleParamDecl, params) },
            }
    };

    let enclosingFunctionType((:(FunCtx(vd), _))) = |()| {
        Just(declType(vd))
    };

    let enclosingFunctionType((:(_, cs))) = |()| {
        enclosingFunctionType(cs)
    };

    let enclosingFunctionType(vec![]) = |()| {
        Nothing
    };

    let extFunProto((VarDeclInfo(var_name, is_inline, storage_spec, attrs, ty, node_info))) = |()| {
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
    };

    let extVarDecl((VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info)), init_opt) = |()| {
        {

            when((isNoName(var_name)))(astError(node_info, "NoName in extVarDecl".to_string()));
            let (storage, is_def) = globalStorage(storage_spec);
            Let([Assign([Span([Ref(Ident("vardecl"))])], Span([Ref(Ident("VarDecl")), Ref(Ident("var_name")), Parens([Span([Ref(Ident("DeclAttrs")), Ref(Ident("is_inline")), Ref(Ident("storage")), Ref(Ident("attrs"))])]), Ref(Ident("typ"))]))], []);
            if(is_def, then, handleObjectDef, False, ident)(ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, False)(Decl(vardecl, node_info)))
        }
    };

    let getParams((FunctionType((FunType(_, params, _)), _))) = |()| {
        Just(params)
    };

    let getParams(_) = |()| {
        Nothing
    };

    let hasTypeDef(declspecs) = |()| {
        match foldr(hasTypeDefSpec, (False, vec![]), declspecs) {
                (True, specs') => { Just(specs') },
                (False, _) => { Nothing },
            }
    };

    let inLoop(c) = |()| {
        any(isLoop, c)
    };

    let inSwitch(c) = |()| {
        any(isSwitch, c)
    };

    let localVarDecl((VarDeclInfo(var_name, is_inline, storage_spec, attrs, typ, node_info)), init_opt) = |()| {
        {

            when((isNoName(var_name)))(astError(node_info, "NoName in localVarDecl".to_string()));
            let (storage, is_def) = localStorage(storage_spec);
            Let([Assign([Span([Ref(Ident("vardecl"))])], Span([Ref(Ident("VarDecl")), Ref(Ident("var_name")), Parens([Span([Ref(Ident("DeclAttrs")), Ref(Ident("is_inline")), Ref(Ident("storage")), Ref(Ident("attrs"))])]), Ref(Ident("typ"))]))], []);
            if(is_def, then, handleObjectDef, True, ident, (ObjDef(vardecl, init_opt, node_info)), else, handleVarDecl, True, (Decl(vardecl, node_info)))
        }
    };

    let matchDesignator((CMemberDesig(m1, _)), (CMemberDesig(m2, _))) = |()| {
        ==(m1, m2)
    };

    let matchDesignator(_, _) = |()| {
        True
    };

    let tBlockItem(_, (CBlockDecl(d))) = |()| {
        >>(analyseDecl(True, d), return(voidType))
    };

    let tBlockItem(_, (CNestedFunDef(fd))) = |()| {
        >>(analyseFunDef(fd), return(voidType))
    };

    let tBlockItem(c, (CBlockStmt(s))) = |()| {
        tStmt(c, s)
    };

    let tDesignator((ArrayType(_, _, _, _)), (:(d, ds))) = |()| {
        typeError((nodeInfo(d)), "member designator in array initializer".to_string())
    };

    let tDesignator((ArrayType(bt, _, _, _)), (:(CArrDesig(e, ni), ds))) = |()| {
        {

            >>=(tExpr(vec![], RValue, e), checkIntegral'(ni));
            tDesignator(bt, ds)
        }
    };

    let tDesignator((ArrayType(bt, _, _, _)), (:(CRangeDesig(e1, e2, ni), ds))) = |()| {
        {

            >>=(tExpr(vec![], RValue, e1), checkIntegral'(ni));
            >>=(tExpr(vec![], RValue, e2), checkIntegral'(ni));
            tDesignator(bt, ds)
        }
    };

    let tDesignator(t, vec![]) = |()| {
        return(t)
    };

    let tExpr'(_, LValue, (CAlignofType(_, ni))) = |()| {
        typeError(ni, "alignoftype as lvalue".to_string())
    };

    let tExpr'(_, LValue, (CSizeofType(_, ni))) = |()| {
        typeError(ni, "sizeoftype as lvalue".to_string())
    };

    let tExpr'(_, RValue, (CAlignofType(_, _))) = |()| {
        return(size_tType)
    };

    let tExpr'(_, RValue, (CSizeofType(_, _))) = |()| {
        return(size_tType)
    };

    let tExpr'(_, _, (CBuiltinExpr(b))) = |()| {
        builtinType(b)
    };

    let tExpr'(_, _, (CConst(c))) = |()| {
        constType(c)
    };

    let tExpr'(_, side, (CCompoundLit(d, initList, ni))) = |()| {
        {

            when((==(side, LValue)))(typeError(ni, "compound literal as lvalue".to_string()));
            let lt = analyseTypeDecl(d);
            tInitList(ni, (canonicalType(lt)), initList);
            return(lt)
        }
    };

    let tExpr'(_, side, (CLabAddrExpr(_, ni))) = |()| {
        {

            when((==(side, LValue)))(typeError(ni, "label address as lvalue".to_string()));
            return(PtrType(voidType, noTypeQuals, vec![]))
        }
    };

    let tExpr'(_, side, (CVar(i, ni))) = |()| {
        >>=(lookupObject(i), maybe((typeErrorOnLeft(ni)(notFound(i))), ((return . declType))))
    };

    let tExpr'(c, _, (CAssign(op, le, re, ni))) = |()| {
        {

            let lt = tExpr(c, LValue, le);
            let rt = tExpr(c, RValue, re);
            when((constant(typeQuals(lt))))(typeError(ni)(++("assignment to lvalue with `constant\' qualifier: ".to_string(), ((render . pretty))(le))));
            match (canonicalType(lt), re) {
    (lt', CConst(CIntConst(i, _))) => if &&(isPointerType(lt'), ==(getCInteger(i), 0)) { return(()) },
        (_, _) => { assignCompatible'(ni, op, lt, rt) },
    };
            return(lt)
        }
    };

    let tExpr'(c, _, (CCall(fe, args, ni))) = |()| {
        {

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
        }
    };

    let tExpr'(c, _, (CIndex(b, i, ni))) = |()| {
        {

            let bt = tExpr(c, RValue, b);
            let it = tExpr(c, RValue, i);
            let addrTy = binopType'(ni, CAddOp, bt, it);
            typeErrorOnLeft(ni)(derefType(addrTy))
        }
    };

    let tExpr'(c, _, (CStatExpr(s, _))) = |()| {
        {

            enterBlockScope;
            mapM_(((withDefTable . defineLabel)), (getLabels(s)));
            let t = tStmt(c, s);
            leaveBlockScope;
            return(t)
        }
    };

    let tExpr'(c, _, (CUnary(CCompOp, e, ni))) = |()| {
        {

            let t = tExpr(c, RValue, e);
            checkIntegral'(ni, t);
            return(t)
        }
    };

    let tExpr'(c, _, (CUnary(CIndOp, e, ni))) = |()| {
        >>=(tExpr(c, RValue, e), ((typeErrorOnLeft(ni) . derefType)))
    };

    let tExpr'(c, side, (CAlignofExpr(e, ni))) = |()| {
        {

            when((==(side, LValue)))(typeError(ni, "alignof as lvalue".to_string()));
            tExpr(c, RValue, e);
            return(size_tType)
        }
    };

    let tExpr'(c, side, (CBinary(op, le, re, ni))) = |()| {
        {

            when((==(side, LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
            let lt = tExpr(c, RValue, le);
            let rt = tExpr(c, RValue, re);
            binopType'(ni, op, lt, rt)
        }
    };

    let tExpr'(c, side, (CCast(d, e, ni))) = |()| {
        {

            let dt = analyseTypeDecl(d);
            let et = tExpr(c, side, e);
            typeErrorOnLeft(ni)(castCompatible(dt, et));
            return(dt)
        }
    };

    let tExpr'(c, side, (CComma(es, _))) = |()| {
        >>=(mapM((tExpr(c, side)), es), (return . last))
    };

    let tExpr'(c, side, (CComplexImag(e, ni))) = |()| {
        complexBaseType(ni, c, side, e)
    };

    let tExpr'(c, side, (CComplexReal(e, ni))) = |()| {
        complexBaseType(ni, c, side, e)
    };

    let tExpr'(c, side, (CCond(e1, me2, e3, ni))) = |()| {
        {

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
        }
    };

    let tExpr'(c, side, (CMember(e, m, deref, ni))) = |()| {
        {

            let t = tExpr(c, RValue, e);
            let bt = if(deref, then, typeErrorOnLeft, ni, (derefType(t)), else, return, t);
            fieldType(ni, m, bt)
        }
    };

    let tExpr'(c, side, (CSizeofExpr(e, ni))) = |()| {
        {

            when((==(side, LValue)))(typeError(ni, "sizeof as lvalue".to_string()));
            tExpr(c, RValue, e);
            return(size_tType)
        }
    };

    let tExpr'(c, side, (CUnary(CAdrOp, e, ni))) = |()| {
        {

            when((==(side, LValue)))(typeError(ni, "address-of operator as lvalue".to_string()));
            match e {
        CCompoundLit, _, _, _ => { liftM(simplePtr, tExpr(c, RValue, e)) },
        CVar, i, _ => { >>=(lookupObject(i), (typeErrorOnLeft(ni) . maybe((notFound(i)), varAddrType))) },
        _ => { liftM(simplePtr, tExpr(c, LValue, e)) },
    }
        }
    };

    let tExpr'(c, side, (CUnary(CNegOp, e, ni))) = |()| {
        {

            when((==(side, LValue)))(typeError(ni, "logical negation used as lvalue".to_string()));
            >>=(tExpr(c, RValue, e), checkScalar'(ni));
            return(boolType)
        }
    };

    let tExpr'(c, side, (CUnary(op, e, _))) = |()| {
        tExpr(c, (if(isEffectfulOp, op, then, LValue, else, side)), e)
    };

    let tExpr(c, side, e) = |()| {
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
    };

    let tInitList(_, t, vec![(vec![], i)]) = |()| {
        >>(tInit(t, i), return(()))
    };

    let tInitList(ni, (PtrType((DirectType(TyVoid, _, _)), _, _)), _) = |()| {
        return(())
    };

    let tInitList(ni, t, _) = |()| {
        typeError(ni)(++("initializer list for type: ".to_string(), pType(t)))
    };

    let tStmt(_, (CAsm(_, _))) = |()| {
        return(voidType)
    };

    let tStmt(_, (CGoto(l, ni))) = |()| {
        {

            let dt = getDefTable;
            match lookupLabel(l, dt) {
        Just, _ => { return(voidType) },
        Nothing => { typeError(ni)(++("undefined label in goto: ".to_string(), identToString(l))) },
    }
        }
    };

    let tStmt(_, (CReturn(Nothing, _))) = |()| {
        return(voidType)
    };

    let tStmt(c, (CBreak(ni))) = |()| {
        {

            unless((||(inLoop(c), inSwitch(c))))(astError(ni, "break statement outside of loop or switch statement".to_string()));
            return(voidType)
        }
    };

    let tStmt(c, (CCase(e, s, ni))) = |()| {
        {

            unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
            >>=(tExpr(c, RValue, e), checkIntegral'(ni));
            tStmt(c, s)
        }
    };

    let tStmt(c, (CCases(e1, e2, s, ni))) = |()| {
        {

            unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
            >>=(tExpr(c, RValue, e1), checkIntegral'(ni));
            >>=(tExpr(c, RValue, e2), checkIntegral'(ni));
            tStmt(c, s)
        }
    };

    let tStmt(c, (CCompound(ls, body, _))) = |()| {
        {

            enterBlockScope;
            mapM_(((withDefTable . defineLabel)), ls);
            let t = foldM((const(tBlockItem(c))), voidType, body);
            leaveBlockScope;
            return(t)
        }
    };

    let tStmt(c, (CCont(ni))) = |()| {
        {

            unless((inLoop(c)))(astError(ni, "continue statement outside of loop".to_string()));
            return(voidType)
        }
    };

    let tStmt(c, (CDefault(s, ni))) = |()| {
        {

            unless((inSwitch(c)))(astError(ni, "default statement outside of switch statement".to_string()));
            tStmt(c, s)
        }
    };

    let tStmt(c, (CExpr(e, _))) = |()| {
        maybe((return(voidType)), (tExpr(c, RValue)), e)
    };

    let tStmt(c, (CFor(i, g, inc, s, _))) = |()| {
        {

            enterBlockScope;
            either((maybe((return(())), checkExpr)), (analyseDecl(True)), i);
            maybe((return(())), (checkGuard(c)), g);
            maybe((return(())), checkExpr, inc);
            tStmt((:(LoopCtx, c)), s);
            leaveBlockScope;
            return(voidType)
        }
    };

    let tStmt(c, (CGotoPtr(e, ni))) = |()| {
        {

            let t = tExpr(c, RValue, e);
            match t {
        PtrType(_, _, _) => { return(voidType) },
        _ => { typeError(ni, "can\'t goto non-pointer".to_string()) },
    }
        }
    };

    let tStmt(c, (CIf(e, sthen, selse, _))) = |()| {
        >>(checkGuard(c, e), >>(tStmt(c, sthen), >>(maybe((return(())), (>>(Lambda(c, s), return(()))), selse), return(voidType))))
    };

    let tStmt(c, (CLabel(_, s, _, _))) = |()| {
        tStmt(c, s)
    };

    let tStmt(c, (CReturn((Just(e)), ni))) = |()| {
        {

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
        }
    };

    let tStmt(c, (CSwitch(e, s, ni))) = |()| {
        >>=(tExpr(c, RValue, e), >>(checkIntegral'(ni), tStmt((:(SwitchCtx, c)), s)))
    };

    let tStmt(c, (CWhile(e, s, _, _))) = |()| {
        >>(checkGuard(c, e), tStmt((:(LoopCtx, c)), s))
    };

}

mod Language_C_Analysis_Builtins {
    fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}

mod Language_C_Analysis_ConstEval {
    struct MachineDesc(MachineDesc, { /* struct def */ });

    let @(constEval(md, env, e), (CBinary(op, e1, e2, ni))) = |()| {
        {

            let e1' = constEval(md, env, e1);
            let e2' = constEval(md, env, e2);
            let t = tExpr(vec![], RValue, e);
            let bytes = liftM(fromIntegral, sizeofType(md, e, t));
            match (intValue(e1'), intValue(e2')) {
        (Just(i1), Just(i2)) => { intExpr(ni, (withWordBytes(bytes, (intOp(op, i1, i2))))) },
        (_, _) => { return(CBinary(op, e1', e2', ni)) },
    }
        }
    };

    let @(constEval(md, env, e), (CVar(i, _))) = |()| {
        {

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
        }
    };

    let alignofType(_, n, t) = |()| {
        astError((nodeInfo(n)))(++("can\'t find alignment of type: ".to_string(), ((render . pretty))(t)))
    };

    let alignofType(md, _, (DirectType((TyBuiltin(b)), _, _))) = |()| {
        return(builtinAlign(md, b))
    };

    let alignofType(md, _, (DirectType((TyComplex(ft)), _, _))) = |()| {
        return(fAlign(md, ft))
    };

    let alignofType(md, _, (DirectType((TyEnum(_)), _, _))) = |()| {
        return(iAlign(md, TyInt))
    };

    let alignofType(md, _, (DirectType((TyFloating(ft)), _, _))) = |()| {
        return(fAlign(md, ft))
    };

    let alignofType(md, _, (DirectType((TyIntegral(it)), _, _))) = |()| {
        return(iAlign(md, it))
    };

    let alignofType(md, _, (DirectType(TyVoid, _, _))) = |()| {
        return(voidAlign(md))
    };

    let alignofType(md, _, (PtrType(_, _, _))) = |()| {
        return(ptrAlign(md))
    };

    let alignofType(md, n, (ArrayType(bt, (ArraySize(_, sz)), _, _))) = |()| {
        alignofType(md, n, bt)
    };

    let alignofType(md, n, (ArrayType(bt, (UnknownArraySize(_)), _, _))) = |()| {
        return(ptrAlign(md))
    };

    let alignofType(md, n, (TypeDefType((TypeDefRef(_, (Just(t)), _)), _, _))) = |()| {
        alignofType(md, n, t)
    };

    let boolValue((CConst((CCharConst(c, _))))) = |()| {
        Just(/=(getCCharAsInt(c), 0))
    };

    let boolValue((CConst((CIntConst(i, _))))) = |()| {
        Just(/=(getCInteger(i), 0))
    };

    let boolValue((CConst((CStrConst(_, _))))) = |()| {
        Just(True)
    };

    let boolValue(_) = |()| {
        Nothing
    };

    let compSize(md, ctr) = |()| {
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
    };

    let constEval(_, _, e) = |()| {
        return(e)
    };

    let constEval(md, _, (CAlignofExpr(e, ni))) = |()| {
        {

            let t = tExpr(vec![], RValue, e);
            let sz = alignofType(md, e, t);
            intExpr(ni, sz)
        }
    };

    let constEval(md, _, (CAlignofType(d, ni))) = |()| {
        {

            let t = analyseTypeDecl(d);
            let sz = alignofType(md, d, t);
            intExpr(ni, sz)
        }
    };

    let constEval(md, _, (CSizeofExpr(e, ni))) = |()| {
        {

            let t = tExpr(vec![], RValue, e);
            let sz = sizeofType(md, e, t);
            intExpr(ni, sz)
        }
    };

    let constEval(md, _, (CSizeofType(d, ni))) = |()| {
        {

            let t = analyseTypeDecl(d);
            let sz = sizeofType(md, d, t);
            intExpr(ni, sz)
        }
    };

    let constEval(md, env, (CCast(d, e, ni))) = |()| {
        {

            let e' = constEval(md, env, e);
            let t = analyseTypeDecl(d);
            let bytes = liftM(fromIntegral, sizeofType(md, d, t));
            match intValue(e') {
        Just, i => { intExpr(ni, (withWordBytes(bytes, i))) },
        Nothing => { return(CCast(d, e', ni)) },
    }
        }
    };

    let constEval(md, env, (CCond(e1, me2, e3, ni))) = |()| {
        {

            let e1' = constEval(md, env, e1);
            let me2' = maybe((return(Nothing)), (liftM(Lambda, constEval(md, env, e))), me2);
            let e3' = constEval(md, env, e3);
            match boolValue(e1') {
        Just, True => { return(fromMaybe(e1', me2')) },
        Just, False => { return(e3') },
        Nothing => { return(CCond(e1', me2', e3', ni)) },
    }
        }
    };

    let constEval(md, env, (CUnary(op, e, ni))) = |()| {
        {

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
        }
    };

    let intExpr(n, i) = |()| {
        >>=(genName, Lambda(CConst(CIntConst((cInteger(i)), (mkNodeInfo((posOf(n)), name))))))
    };

    let intOp(CAddOp, i1, i2) = |()| {
        +(i1, i2)
    };

    let intOp(CAndOp, i1, i2) = |()| {
        .&.(i1, i2)
    };

    let intOp(CDivOp, i1, i2) = |()| {
        div(i1, i2)
    };

    let intOp(CEqOp, i1, i2) = |()| {
        toInteger(fromEnum(==(i1, i2)))
    };

    let intOp(CGeqOp, i1, i2) = |()| {
        toInteger(fromEnum(>=(i1, i2)))
    };

    let intOp(CGrOp, i1, i2) = |()| {
        toInteger(fromEnum(>(i1, i2)))
    };

    let intOp(CLeOp, i1, i2) = |()| {
        toInteger(fromEnum(<(i1, i2)))
    };

    let intOp(CLeqOp, i1, i2) = |()| {
        toInteger(fromEnum(<=(i1, i2)))
    };

    let intOp(CLndOp, i1, i2) = |()| {
        toInteger(fromEnum(&&((/=(i1, 0)), (/=(i2, 0)))))
    };

    let intOp(CLorOp, i1, i2) = |()| {
        toInteger(fromEnum(||((/=(i1, 0)), (/=(i2, 0)))))
    };

    let intOp(CMulOp, i1, i2) = |()| {
        *(i1, i2)
    };

    let intOp(CNeqOp, i1, i2) = |()| {
        toInteger(fromEnum(/=(i1, i2)))
    };

    let intOp(COrOp, i1, i2) = |()| {
        .|.(i1, i2)
    };

    let intOp(CRmdOp, i1, i2) = |()| {
        mod(i1, i2)
    };

    let intOp(CShlOp, i1, i2) = |()| {
        shiftL(i1, fromInteger(i2))
    };

    let intOp(CShrOp, i1, i2) = |()| {
        shiftR(i1, fromInteger(i2))
    };

    let intOp(CSubOp, i1, i2) = |()| {
        -(i1, i2)
    };

    let intOp(CXorOp, i1, i2) = |()| {
        xor(i1, i2)
    };

    let intUnOp(CCompOp, i) = |()| {
        Just(complement(i))
    };

    let intUnOp(CMinOp, i) = |()| {
        Just(Operator("-")(i))
    };

    let intUnOp(CNegOp, i) = |()| {
        Just(toInteger(fromEnum(==(i, 0))))
    };

    let intUnOp(CPlusOp, i) = |()| {
        Just(i)
    };

    let intUnOp(_, _) = |()| {
        Nothing
    };

    let intValue((CConst((CCharConst(c, _))))) = |()| {
        Just(getCCharAsInt(c))
    };

    let intValue((CConst((CIntConst(i, _))))) = |()| {
        Just(getCInteger(i))
    };

    let intValue(_) = |()| {
        Nothing
    };

    let sizeofType(_, n, t) = |()| {
        astError((nodeInfo(n)))(++("can\'t find size of type: ".to_string(), ((render . pretty))(t)))
    };

    let sizeofType(md, _, (DirectType((TyBuiltin(b)), _, _))) = |()| {
        return(builtinSize(md, b))
    };

    let sizeofType(md, _, (DirectType((TyComp(ctr)), _, _))) = |()| {
        compSize(md, ctr)
    };

    let sizeofType(md, _, (DirectType((TyComplex(ft)), _, _))) = |()| {
        return(*(2, fSize(md, ft)))
    };

    let sizeofType(md, _, (DirectType((TyEnum(_)), _, _))) = |()| {
        return(iSize(md, TyInt))
    };

    let sizeofType(md, _, (DirectType((TyFloating(ft)), _, _))) = |()| {
        return(fSize(md, ft))
    };

    let sizeofType(md, _, (DirectType((TyIntegral(it)), _, _))) = |()| {
        return(iSize(md, it))
    };

    let sizeofType(md, _, (DirectType(TyVoid, _, _))) = |()| {
        return(voidSize(md))
    };

    let sizeofType(md, _, (FunctionType(_, _))) = |()| {
        return(ptrSize(md))
    };

    let sizeofType(md, _, (PtrType(_, _, _))) = |()| {
        return(ptrSize(md))
    };

    let sizeofType(md, n, (ArrayType(bt, (ArraySize(_, sz)), _, _))) = |()| {
        {

            let sz' = constEval(md, Map.empty, sz);
            match sz' {
        CConst, CIntConst(i, _) => { {

            let s = sizeofType(md, n, bt);
            return(*(getCInteger(i), s))
        } },
        _ => { return(ptrSize(md)) },
    }
        }
    };

    let sizeofType(md, n, (ArrayType(bt, (UnknownArraySize(_)), _, _))) = |()| {
        return(ptrSize(md))
    };

    let sizeofType(md, n, (TypeDefType((TypeDefRef(_, (Just(t)), _)), _, _))) = |()| {
        sizeofType(md, n, t)
    };

    let withWordBytes(bytes, n) = |()| {
        rem(n, (shiftL(1, (shiftL(bytes, 3)))))
    };

}

mod Language_C_Analysis_Debug {
    let globalDeclStats(file_filter, gmap) = |()| {
        vec![("Enumeration Constants".to_string(), Map.size(enumerators)), ("Total Object/Function Declarations".to_string(), Map.size(all_decls)), ("Object definitions".to_string(), Map.size(objDefs)), ("Function Definitions".to_string(), Map.size(funDefs)), ("Tag definitions".to_string(), Map.size(tagDefs)), ("TypeDefs".to_string(), Map.size(typeDefs))]
    };

    fn joinComma() -> Doc {
        (hsep . (punctuate(comma) . map(pretty)))
    }

    let prettyAssocs(label) = |()| {
        prettyAssocsWith(label, pretty, pretty)
    };

    let prettyAssocsWith(label, prettyKey, prettyVal, theMap) = |()| {
        $$(text(label), (nest(8))((vcat(map(prettyEntry, theMap)))))
    };

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

    let @(splitCDecl(decl), (CDecl(declspecs, declrs, node))) = |()| {
        match declrs {
                [] => { internalErr("splitCDecl applied to empty declaration".to_string()) },
                [declr] => { return(vec![decl]) },
                d1:ds => { Let([Assign([Span([Ref(Ident("declspecs\'"))])], Span([Ref(Ident("map")), Ref(Ident("elideSUEDef")), Ref(Ident("declspecs")), Ref(Ident("in"))]))], [])(return)(:((CDecl(declspecs, vec![d1], node)), Dummy)) },
            }
    };

    let analyseVarDecl'(handle_sue_def, declspecs, declr, oldstyle, init_opt) = |()| {
        {

            Let([Assign([Span([Parens([Span([Ref(Ident("storage_specs"))]), Span([Ref(Ident("attrs"))]), Span([Ref(Ident("type_quals"))]), Span([Ref(Ident("type_specs"))]), Span([Ref(Ident("inline"))])])])], Span([Ref(Ident("partitionDeclSpecs")), Ref(Ident("declspecs"))]))], []);
            let canonTySpecs = canonicalTypeSpec(type_specs);
            analyseVarDecl(handle_sue_def, storage_specs, attrs, type_quals, canonTySpecs, inline, declr, oldstyle, init_opt)
        }
    };

    let analyseVarDecl(handle_sue_def, storage_specs, decl_attrs, typequals, canonTySpecs, inline, (CDeclr(name_opt, derived_declrs, asmname_opt, declr_attrs, node)), oldstyle_params, init_opt) = |()| {
        {

            let storage_spec = canonicalStorageSpec(storage_specs);
            let typ = tType(handle_sue_def, node, typequals, canonTySpecs, derived_declrs, oldstyle_params);
            let attrs' = mapM(tAttr, (++(decl_attrs, declr_attrs)));
            let name = mkVarName(node, name_opt, asmname_opt);
            return(VarDeclInfo(name, inline, storage_spec, attrs', typ, node))
        }
    };

    let canonicalStorageSpec(storagespecs) = |()| {
        liftM(elideAuto)(foldrM(updStorage, NoStorageSpec, storagespecs))
    };

    fn canonicalTypeSpec() -> m {
        foldrM(go, TSNone)
    }

    let computeParamStorage(_, NoStorageSpec) = |()| {
        Right((Auto(False)))
    };

    let computeParamStorage(_, RegSpec) = |()| {
        Right((Auto(True)))
    };

    let computeParamStorage(node, spec) = |()| {
        (Left . badSpecifierError(node)(++("Bad storage specified for parameter: ".to_string(), show(spec))))
    };

    let emptyDeclr(node) = |()| {
        CDeclr(Nothing, vec![], Nothing, vec![], node)
    };

    fn emptyNumTypeSpec() -> NumTypeSpec {
        NumTypeSpec({
            base: NoBaseType,
            signSpec: NoSignSpec,
            sizeMod: NoSizeMod,
            isComplex: False
            })
    }

    let getOnlyDeclr((CDecl(_, _, node))) = |()| {
        internalErr("getOnlyDeclr: declaration doesn\'t have a unique declarator".to_string())
    };

    let getOnlyDeclr((CDecl(_, vec![(Just(declr), _, _)], _))) = |()| {
        return(declr)
    };

    let hasThreadLocalSpec((ExternSpec(b))) = |()| {
        b
    };

    let hasThreadLocalSpec((StaticSpec(b))) = |()| {
        b
    };

    let hasThreadLocalSpec(ThreadSpec) = |()| {
        True
    };

    let hasThreadLocalSpec(_) = |()| {
        False
    };

    let isTypeDef(declspecs) = |()| {
        not(null(Dummy))
    };

    let mergeOldStyle(_node, vec![], declrs) = |()| {
        return(declrs)
    };

    let mergeOldStyle(node, _, _) = |()| {
        astError(node, "oldstyle parameter list, but not function type".to_string())
    };

    let mergeOldStyle(node, oldstyle_params, (:(CFunDeclr(params, attrs, fdnode), dds))) = |()| {
        match params {
                Left, list => { {

                    let oldstyle_params' = liftM(concat)(mapM(splitCDecl, oldstyle_params));
                    let param_map = liftM(Map.fromList)(mapM(attachNameOfDecl, oldstyle_params'));
                    let (newstyle_params, param_map') = foldrM(insertParamDecl, (vec![], param_map), list);
                    when((not(Map.null(param_map'))))(astError(node)(++("declarations for parameter(s) ".to_string(), ++(showParamMap(param_map'), " but no such parameter".to_string()))));
                    return((:(CFunDeclr((Right((newstyle_params, False))), attrs, fdnode), dds)))
                } },
                Right, _newstyle => { astError(node, "oldstyle parameter list, but newstyle function declaration".to_string()) },
            }
    };

    let mergeTypeAttributes(node_info, quals, attrs, typ) = |()| {
        match typ {
                DirectType, ty_name, quals', attrs' => { merge(quals', attrs')(mkDirect(ty_name)) },
                PtrType, ty, quals', attrs' => { merge(quals', attrs')(PtrType(ty)) },
                ArrayType, ty, array_sz, quals', attrs' => { merge(quals', attrs')(ArrayType(ty, array_sz)) },
                FunctionType, FunType(return_ty, params, inline), attrs' => { return(FunctionType((FunType(return_ty, params, inline)), (++(attrs', attrs)))) },
                TypeDefType, tdr, quals', attrs' => { merge(quals', attrs')(TypeDefType(tdr)) },
            }
    };

    let mkVarName(node, (Just(n)), asm) = |()| {
        return(VarName(n, asm))
    };

    let mkVarName(node, Nothing, _) = |()| {
        return(NoName)
    };

    let nameOfDecl(d) = |()| {
        >>=(getOnlyDeclr(d), Lambda)
    };

    let tArraySize((CArrSize(static, szexpr))) = |()| {
        liftM((ArraySize(static)), (return(szexpr)))
    };

    let tArraySize((CNoArrSize(False))) = |()| {
        return((UnknownArraySize(False)))
    };

    let tArraySize((CNoArrSize(True))) = |()| {
        return((UnknownArraySize(True)))
    };

    let tAttr((CAttr(name, cexpr, node))) = |()| {
        return(Attr(name, cexpr, node))
    };

    let tCompType(tag, sue_ref, member_decls, attrs, node) = |()| {
        ap(return((CompType(tag, sue_ref))), ap((concatMapM(tMemberDecls, member_decls)), ap((return(attrs)), (return(node)))))
    };

    let tCompTypeDecl(handle_def, (CStruct(tag, ident_opt, member_decls_opt, attrs, node_info))) = |()| {
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
    };

    let tDirectType(handle_sue_def, node, ty_quals, canonTySpec) = |()| {
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
    };

    let tEnumType(sue_ref, enumerators, attrs, node) = |()| {
        {

            mapM_(handleEnumeratorDef, enumerators');
            return(ty);
            
        }
    };

    let tMemberDecls((CDecl(declspecs, declrs, node))) = |()| {
        mapM((uncurry(tMemberDecl)), (zip((True:repeat(False)), declrs)))
    };

    let tMemberDecls((CDecl(declspecs, vec![], node))) = |()| {
        {

            Let([Assign([Span([Parens([Span([Ref(Ident("storage_specs"))]), Span([Ref(Ident("_attrs"))]), Span([Ref(Ident("typequals"))]), Span([Ref(Ident("typespecs"))]), Span([Ref(Ident("is_inline"))])])])], Span([Ref(Ident("partitionDeclSpecs")), Ref(Ident("declspecs"))]))], []);
            when(is_inline)(astError(node, "member declaration with inline specifier".to_string()));
            let canonTySpecs = canonicalTypeSpec(typespecs);
            let ty = tType(True, node, typequals, canonTySpecs, vec![], vec![]);
            match ty {
        DirectType, TyComp(_), _, _ => { return(vec![MemberDecl((VarDecl(NoName, (DeclAttrs(False, NoStorage, vec![])), ty)), Nothing, node)]) },
        _ => { astError(node, "anonymous member has a non-composite type".to_string()) },
    }
        }
    };

    let tNumType((NumTypeSpec(basetype, sgn, sz, iscomplex))) = |()| {
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
    };

    let tParamDecl((CDecl(declspecs, declrs, node))) = |()| {
        {

            let declr = getParamDeclr;
            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl'(True, declspecs, declr, vec![], Nothing);
            when((is_inline))(throwTravError((badSpecifierError(node, "parameter declaration with inline specifier".to_string()))));
            let storage = throwOnLeft(computeParamStorage(node, storage_spec));
            Let([Assign([Span([Ref(Ident("paramDecl"))])], Span([Ref(Ident("mkParamDecl")), Ref(Ident("name")), Ref(Ident("storage")), Ref(Ident("attrs")), Ref(Ident("ty")), Ref(Ident("declr_node"))]))], []);
            return(paramDecl)
        }
    };

    let tTag(CStructTag) = |()| {
        StructTag
    };

    let tTag(CUnionTag) = |()| {
        UnionTag
    };

    let tType(handle_sue_def, top_node, typequals, canonTySpecs, derived_declrs, oldstyle_params) = |()| {
        >>=(mergeOldStyle(top_node, oldstyle_params, derived_declrs), buildType)
    };

    fn tTypeQuals() -> m {
        foldrM(go, (noTypeQuals, vec![]))
    }

    let typeDefRef(t_node, name) = |()| {
        >>=(lookupTypeDef(name), Lambda((TypeDefRef(name, (Just(ty)), t_node))))
    };

}

mod Language_C_Analysis_DefTable {
    struct TagFwdDecl(CompDecl, CompTypeRef, EnumDecl, EnumTypeRef);

    struct DefTable(DefTable, { /* struct def */ });

    #[derive(Clone, Debug)]
    struct DeclarationStatus(NewDecl, Redeclared, t, KeepDef, t, Shadowed, t, KindMismatch, t);

    #[derive(Eq, Ord)]
    struct TagEntryKind(CompKind, CompTyKind, EnumKind);

    let compatIdentEntry((Left(_tydef))) = |()| {
        either((const(True)), (const(False)))
    };

    let compatIdentEntry((Right(def))) = |()| {
        either((const(False)))(Lambda)
    };

    let compatTagEntry(te1, te2) = |()| {
        ==(tagKind(te1), tagKind(te2))
    };

    let declStatusDescr((KeepDef(_))) = |()| {
        "keep old".to_string()
    };

    let declStatusDescr((KindMismatch(_))) = |()| {
        "kind mismatch".to_string()
    };

    let declStatusDescr((Redeclared(_))) = |()| {
        "redeclared".to_string()
    };

    let declStatusDescr((Shadowed(_))) = |()| {
        "shadowed".to_string()
    };

    let declStatusDescr(NewDecl) = |()| {
        "new".to_string()
    };

    let declareTag(sueref, decl, deftbl) = |()| {
        match lookupTag(sueref, deftbl) {
                Nothing => { (NewDecl, deftbl({
                        tagDecls: fst(defLocal((tagDecls(deftbl)), sueref, (Left(decl))))
                        })) },
            Just, old_def => if ==(tagKind(old_def), tagKind((Left(decl)))) { (KeepDef(old_def), deftbl) }
otherwise { (KindMismatch(old_def), deftbl) },
            }
    };

    let defRedeclStatus(sameKind, def, oldDecl) = |()| {
        match oldDecl {
            Just, def' => if sameKind(def, def') { Redeclared(def') }
otherwise { KindMismatch(def') },
                Nothing => { NewDecl },
            }
    };

    let defRedeclStatusLocal(sameKind, ident, def, oldDecl, nsm) = |()| {
        match defRedeclStatus(sameKind, def, oldDecl) {
                NewDecl => { match lookupName(nsm, ident) {
                        Just, shadowed => { Shadowed(shadowed) },
                        Nothing => { NewDecl },
                    } },
                redecl => { redecl },
            }
    };

    let defineGlobalIdent(ident, def, deftbl) = |()| {
        (defRedeclStatus(compatIdentEntry, (Right(def)), oldDecl), deftbl({
                identDecls: decls'
                }))
    };

    let defineLabel(ident, deftbl) = |()| {
        Let([Assign([Span([Parens([Span([Ref(Ident("labels\'"))]), Span([Ref(Ident("old_label"))])])])], Span([Ref(Ident("defLocal")), Parens([Span([Ref(Ident("labelDefs")), Ref(Ident("deftbl"))])]), Ref(Ident("ident")), Ref(Ident("ident"))]))], [])(in, (maybe(NewDecl, Redeclared, old_label), deftbl({
                labelDefs: labels'
                })))
    };

    fn defineScopedIdent() -> (DeclarationStatus(IdentEntry), DefTable) {
        defineScopedIdentWhen((const(True)))
    }

    let defineScopedIdentWhen(override_def, ident, def, deftbl) = |()| {
        (redecl_status, deftbl({
                identDecls: decls'
                }))
    };

    let defineTag(sueref, def, deftbl) = |()| {
        (redeclStatus, deftbl({
                tagDecls: decls'
                }))
    };

    let defineTypeDef(ident, tydef, deftbl) = |()| {
        (defRedeclStatus(compatIdentEntry, (Left(tydef)), oldDecl), deftbl({
                identDecls: decls'
                }))
    };

    fn emptyDefTable() -> DefTable {
        DefTable(nameSpaceMap, nameSpaceMap, nameSpaceMap, nameSpaceMap, IntMap.empty, IntMap.empty)
    }

    let enterBlockScope(deftbl) = |()| {
        enterLocalScope(deftbl({
                labelDefs: enterNewScope((labelDefs(deftbl)))
                }))
    };

    let enterFunctionScope(deftbl) = |()| {
        enterLocalScope(deftbl({
                labelDefs: enterNewScope((labelDefs(deftbl)))
                }))
    };

    let enterLocalScope(deftbl) = |()| {
        deftbl({
            identDecls: enterNewScope((identDecls(deftbl))),
            tagDecls: enterNewScope((tagDecls(deftbl)))
            })
    };

    let enterMemberDecl(deftbl) = |()| {
        deftbl({
            memberDecls: enterNewScope((memberDecls(deftbl)))
            })
    };

    let globalDefs(deftbl) = |()| {
        Map.foldWithKey(insertDecl, (GlobalDecls(e, gtags, e)), (globalNames(identDecls(deftbl))))
    };

    fn identOfTyDecl() -> Ident {
        either(identOfTypeDef, declIdent)
    }

    let inFileScope(dt) = |()| {
        not((||(hasLocalNames((identDecls(dt))), hasLocalNames((labelDefs(dt))))))
    };

    let insertType(dt, n, t) = |()| {
        dt({
            typeTable: IntMap.insert((nameId(n)), t, (typeTable(dt)))
            })
    };

    let leaveBlockScope(deftbl) = |()| {
        leaveLocalScope(deftbl({
                labelDefs: leaveScope_((labelDefs(deftbl)))
                }))
    };

    let leaveFunctionScope(deftbl) = |()| {
        leaveLocalScope(deftbl({
                labelDefs: leaveScope_((labelDefs(deftbl)))
                }))
    };

    let leaveLocalScope(deftbl) = |()| {
        deftbl({
            identDecls: leaveScope_((identDecls(deftbl))),
            tagDecls: leaveScope_((tagDecls(deftbl)))
            })
    };

    let leaveMemberDecl(deftbl) = |()| {
        Let([Assign([Span([Parens([Span([Ref(Ident("decls\'"))]), Span([Ref(Ident("members"))])])])], Span([Ref(Ident("leaveScope")), Parens([Span([Ref(Ident("memberDecls")), Ref(Ident("deftbl"))])])]))], [])(in, Dummy, (map(snd, members)), (deftbl({
                memberDecls: decls'
                })))
    };

    fn leaveScope_() -> NameSpaceMap {
        (fst . leaveScope)
    }

    let lookupIdent(ident, deftbl) = |()| {
        lookupName((identDecls(deftbl)), ident)
    };

    let lookupIdentInner(ident, deftbl) = |()| {
        lookupInnermostScope((identDecls(deftbl)), ident)
    };

    let lookupLabel(ident, deftbl) = |()| {
        lookupName((labelDefs(deftbl)), ident)
    };

    let lookupTag(sue_ref, deftbl) = |()| {
        lookupName((tagDecls(deftbl)), sue_ref)
    };

    let lookupTagInner(sue_ref, deftbl) = |()| {
        lookupInnermostScope((tagDecls(deftbl)), sue_ref)
    };

    let lookupType(dt, n) = |()| {
        IntMap.lookup((nameId(n)), (typeTable(dt)))
    };

    let mergeDefTable((DefTable(i1, t1, l1, m1, r1, tt1)), (DefTable(i2, t2, l2, m2, r2, tt2))) = |()| {
        DefTable((mergeNameSpace(i1, i2)), (mergeNameSpace(t1, t2)), (mergeNameSpace(l1, l2)), (mergeNameSpace(m1, m2)), (union(r1, r2)), (union(tt1, tt2)))
    };

    let tagKind((Left((CompDecl(cd))))) = |()| {
        CompKind((compTag(cd)))
    };

    let tagKind((Left((EnumDecl(_))))) = |()| {
        EnumKind
    };

    let tagKind((Right((CompDef(cd))))) = |()| {
        CompKind((compTag(cd)))
    };

    let tagKind((Right((EnumDef(_))))) = |()| {
        EnumKind
    };

}

mod Language_C_Analysis_Export {
    let exportArraySize((ArraySize(static, e))) = |()| {
        CArrSize(static, e)
    };

    let exportArraySize((UnknownArraySize(complete))) = |()| {
        CNoArrSize(complete)
    };

    fn exportAttrs() -> Vec<CAttr> {
        map(exportAttr)
    }

    let exportCompType((CompType(sue_ref, comp_tag, members, attrs, node_info))) = |()| {
        vec![CSUType(comp, ni)]
    };

    let exportCompTypeDecl(ty) = |()| {
        vec![CSUType((exportComp(ty)), ni)]
    };

    let exportCompTypeRef((CompType(sue_ref, com_tag, _, _, node_info))) = |()| {
        exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
    };

    let exportComplexType(ty) = |()| {
        :((CComplexType(ni)), exportFloatType(ty))
    };

    let exportDeclAttrs((DeclAttrs(inline, storage, attrs))) = |()| {
        ++((if(inline, then, vec![CTypeQual((CInlineQual(ni)))], else, vec![])), ++(map((CStorageSpec), (exportStorage(storage))), map(((CTypeQual . CAttrQual)), (exportAttrs(attrs)))))
    };

    let exportDeclr(other_specs, ty, attrs, name) = |()| {
        (++(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
    };

    let exportEnumType((EnumType(sue_ref, enumerators, attrs, node_info))) = |()| {
        vec![CEnumType(enum, ni)]
    };

    let exportEnumTypeDecl(ty) = |()| {
        vec![CEnumType((exportEnum(ty)), ni)]
    };

    let exportEnumTypeRef((EnumType(sue_ref, _, _, node_info))) = |()| {
        exportEnumTypeDecl((EnumTypeRef(sue_ref, node_info)))
    };

    let exportFloatType(ty) = |()| {
        match ty {
                TyFloat => { vec![CFloatType(ni)] },
                TyDouble => { vec![CDoubleType(ni)] },
                TyLDouble => { vec![CLongType(ni), CDoubleType(ni)] },
            }
    };

    let exportIntType(ty) = |()| {
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
    };

    let exportMemberDecl((AnonBitField(ty, expr, node_info))) = |()| {
        CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info)
    };

    let exportMemberDecl((MemberDecl(vardecl, bitfieldsz, node_info))) = |()| {
        Let([Assign([Span([Parens([Span([Ref(Ident("specs"))]), Span([Ref(Ident("declarator"))])])])], Span([Ref(Ident("exportVarDecl")), Ref(Ident("vardecl"))]))], [])(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info)
    };

    let exportParamDecl(paramdecl) = |()| {
        Let([Assign([Span([Parens([Span([Ref(Ident("specs"))]), Span([Ref(Ident("declr"))])])])], Span([Ref(Ident("exportVarDecl")), Parens([Span([Ref(Ident("getVarDecl")), Ref(Ident("paramdecl"))])])]))], [])(in, CDecl, specs, vec![(Just(declr), Nothing, Nothing)], (nodeInfo(paramdecl)))
    };

    fn exportSUERef() -> Maybe {
        (Just . (internalIdent . show))
    }

    let exportStorage((Auto(reg))) = |()| {
        if(reg, then, vec![CRegister(ni)], else, vec![])
    };

    let exportStorage((FunLinkage(ExternalLinkage))) = |()| {
        vec![]
    };

    let exportStorage((FunLinkage(InternalLinkage))) = |()| {
        vec![CStatic(ni)]
    };

    let exportStorage((FunLinkage(NoLinkage))) = |()| {
        error("impossible storage: function without linkage".to_string())
    };

    let exportStorage((Static(ExternalLinkage, thread_local))) = |()| {
        threadLocal(thread_local, vec![CExtern(ni)])
    };

    let exportStorage((Static(InternalLinkage, thread_local))) = |()| {
        threadLocal(thread_local, vec![CStatic(ni)])
    };

    let exportStorage((Static(NoLinkage, _))) = |()| {
        error("impossible storage: static without linkage".to_string())
    };

    let exportStorage(NoStorage) = |()| {
        vec![]
    };

    let exportType(ty) = |()| {
        exportTy(vec![], ty)
    };

    let exportTypeDecl(ty) = |()| {
        CDecl(declspecs, declrs, ni)
    };

    let exportTypeDef((TypeDef(ident, ty, attrs, node_info))) = |()| {
        CDecl((:(CStorageSpec((CTypedef(ni))), declspecs)), vec![declr], node_info)
    };

    let exportTypeQuals(quals) = |()| {
        mapMaybe(select, vec![(constant, CConstQual(ni)), (volatile, CVolatQual(ni)), (restrict, CRestrQual(ni))])
    };

    let exportTypeQualsAttrs(tyqs, attrs) = |()| {
        (++(exportTypeQuals(tyqs), map(CAttrQual, (exportAttrs(attrs)))))
    };

    let exportTypeSpec(tyname) = |()| {
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
    };

    let exportVarDecl((VarDecl(name, attrs, ty))) = |()| {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    };

    let fromDirectType((DirectType(ty, _, _))) = |()| {
        ty
    };

    let fromDirectType((TypeDefType((TypeDefRef(_, ref, _)), _, _))) = |()| {
        maybe((error("undefined typeDef".to_string())), fromDirectType, ref)
    };

    let fromDirectType(_) = |()| {
        error("fromDirectType".to_string())
    };

    fn ni() -> NodeInfo {
        undefNode
    }

    let threadLocal(False) = |()| {
        id
    };

    let threadLocal(True) = |()| {
        ((CThread(ni))(Operator(":")))
    };

}

mod Language_C_Analysis_NameSpaceMap {
    struct NameSpaceMap(NsMap, Map(k, v), Vec<Vec<(k, v)>>);

    let @(defLocal(ns), (NsMap(_, vec![]))(ident, def)) = |()| {
        defGlobal(ns, ident, def)
    };

    let @(lookupInnermostScope(nsm), (NsMap(_gs, localDefs))(ident)) = |()| {
        match localDefs {
                ls(:, _lss) => { Prelude.lookup(ident, ls) },
                [] => { lookupGlobal(nsm, ident) },
            }
    };

    let @(lookupName(ns), (NsMap(_, localDefs))(ident)) = |()| {
        match (lookupLocal(localDefs)) {
                Nothing => { lookupGlobal(ns, ident) },
                Just, def => { Just(def) },
            }
    };

    let defGlobal((NsMap(gs, lss)), ident, def) = |()| {
        (NsMap((Map.insert(ident, def, gs)), lss), Map.lookup(ident, gs))
    };

    let defLocal((NsMap(gs, (ls:lss))), ident, def) = |()| {
        (NsMap(gs, (:((:((ident, def), ls)), lss))), Prelude.lookup(ident, ls))
    };

    let enterNewScope((NsMap(gs, lss))) = |()| {
        NsMap(gs, (:(vec![], lss)))
    };

    let globalNames((NsMap(g, _))) = |()| {
        g
    };

    let hasLocalNames((NsMap(_, l))) = |()| {
        not((null(l)))
    };

    let leaveScope((NsMap(_, vec![]))) = |()| {
        error("NsMaps.leaveScope: No local scope!".to_string())
    };

    let leaveScope((NsMap(gs, (ls:lss)))) = |()| {
        (NsMap(gs, lss), ls)
    };

    let localNames((NsMap(_, l))) = |()| {
        l
    };

    let lookupGlobal((NsMap(gs, _)), ident) = |()| {
        Map.lookup(ident, gs)
    };

    let mergeNameSpace((NsMap(global1, local1)), (NsMap(global2, local2))) = |()| {
        NsMap((Map.union(global1, global2)), (localUnion(local1, local2)))
    };

    fn nameSpaceMap() -> NameSpaceMap {
        NsMap(Map.empty, vec![])
    }

    let nsMapToList((NsMap(gs, lss))) = |()| {
        ++(concat(lss), Map.toList(gs))
    };

}

mod Language_C_Analysis_SemError {
    #[derive(Debug)]
    struct RedefError(RedefError, ErrorLevel, RedefInfo);

    struct RedefInfo(RedefInfo, String, RedefKind, NodeInfo, NodeInfo);

    struct RedefKind(DuplicateDef, DiffKindRedecl, ShadowedDef, DisagreeLinkage, NoLinkageOld);

    #[derive(Debug)]
    struct TypeMismatch(TypeMismatch, String, (NodeInfo, Type), (NodeInfo, Type));

    let @(redefErrorInfo(lvl, info), (RedefInfo(_, _, node, old_node))) = |()| {
        ErrorInfo(lvl, (posOfNode(node)), (++(vec![redefErrReason(info)], prevDeclMsg(old_node))))
    };

    let badSpecifierError(node_info, msg) = |()| {
        BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
    };

    let invalidAST(node_info, msg) = |()| {
        InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
    };

    let prevDeclMsg(old_node) = |()| {
        vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
    };

    let redefErrLabel((RedefInfo(ident, _, _, _))) = |()| {
        ++(ident, " redefined".to_string())
    };

    let redefErrReason((RedefInfo(ident, DiffKindRedecl, _, _))) = |()| {
        ++(ident, " previously declared as a different kind of symbol".to_string())
    };

    let redefErrReason((RedefInfo(ident, DisagreeLinkage, _, _))) = |()| {
        ++(ident, " previously declared with different linkage".to_string())
    };

    let redefErrReason((RedefInfo(ident, DuplicateDef, _, _))) = |()| {
        ++("duplicate definition of ".to_string(), ident)
    };

    let redefErrReason((RedefInfo(ident, NoLinkageOld, _, _))) = |()| {
        ++(ident, " previously declared without linkage".to_string())
    };

    let redefErrReason((RedefInfo(ident, ShadowedDef, _, _))) = |()| {
        ++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string()))
    };

    let redefinition(lvl, ctx, kind, new, old) = |()| {
        RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
    };

    fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    let typeMismatchInfo((TypeMismatch(reason, (node1, _ty2), _t2))) = |()| {
        ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
    };

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

    let declLinkage(decl) = |()| {
        match declStorage(decl) {
                NoStorage => { undefined },
                Auto, _ => { NoLinkage },
                Static, linkage, _ => { linkage },
                FunLinkage, linkage => { linkage },
            }
    };

    fn declName() -> VarName {
        ((Lambda) . getVarDecl)
    }

    let declOfDef(def) = |()| {
        Let([Assign([Span([Ref(Ident("vd"))])], Span([Ref(Ident("getVarDecl")), Ref(Ident("def")), Ref(Ident("in")), Ref(Ident("Decl")), Ref(Ident("vd")), Parens([Span([Ref(Ident("nodeInfo")), Ref(Ident("def"))])])]))], [])
    };

    let declStorage(d) = |()| {
        match declAttrs(d) {
                DeclAttrs(_, st, _) => { st },
            }
    };

    fn declType() -> Type {
        ((Lambda) . getVarDecl)
    }

    fn emptyGlobalDecls() -> GlobalDecls {
        GlobalDecls(Map.empty, Map.empty, Map.empty)
    }

    let filterGlobalDecls(decl_filter, gmap) = |()| {
        GlobalDecls({
            gObjs: Map.filter(((decl_filter . DeclEvent)), (gObjs(gmap))),
            gTags: Map.filter(((decl_filter . TagEvent)), (gTags(gmap))),
            gTypeDefs: Map.filter(((decl_filter . TypeDefEvent)), (gTypeDefs(gmap)))
            })
    };

    let hasLinkage((Auto(_))) = |()| {
        False
    };

    let hasLinkage((Static(NoLinkage, _))) = |()| {
        False
    };

    let hasLinkage(_) = |()| {
        True
    };

    let identOfTypeDef((TypeDef(ide, _, _, _))) = |()| {
        ide
    };

    let identOfVarName((VarName(ident, _))) = |()| {
        ident
    };

    let identOfVarName(NoName) = |()| {
        error("identOfVarName: NoName".to_string())
    };

    fn isExtDecl() -> Bool {
        (hasLinkage . declStorage)
    }

    let isNoName(NoName) = |()| {
        True
    };

    let isNoName(_) = |()| {
        False
    };

    fn mergeAttributes() -> Attributes {
        (Operator("++"))
    }

    let mergeGlobalDecls(gmap1, gmap2) = |()| {
        GlobalDecls({
            gObjs: Map.union((gObjs(gmap1)), (gObjs(gmap2))),
            gTags: Map.union((gTags(gmap1)), (gTags(gmap2))),
            gTypeDefs: Map.union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
            })
    };

    let mergeTypeQuals((TypeQuals(c1, v1, r1)), (TypeQuals(c2, v2, r2))) = |()| {
        TypeQuals((&&(c1, c2)), (&&(v1, v2)), (&&(r1, r2)))
    };

    fn noAttributes() -> Attributes {
        vec![]
    }

    fn noTypeQuals() -> TypeQuals {
        TypeQuals(False, False, False)
    }

    let objKindDescr((Declaration(_))) = |()| {
        "declaration".to_string()
    };

    let objKindDescr((EnumeratorDef(_))) = |()| {
        "enumerator definition".to_string()
    };

    let objKindDescr((FunctionDef(_))) = |()| {
        "function definition".to_string()
    };

    let objKindDescr((ObjectDef(_))) = |()| {
        "object definition".to_string()
    };

    let splitIdentDecls(include_all) = |()| {
        Map.foldWithKey((if(include_all, then, deal, else, deal')), (Map.empty, (Map.empty, Map.empty, Map.empty)))
    };

    let typeOfCompDef((CompType(ref, tag, _, _, _))) = |()| {
        TyComp((CompTypeRef(ref, tag, undefNode)))
    };

    let typeOfEnumDef((EnumType(ref, _, _, _))) = |()| {
        TyEnum((EnumTypeRef(ref, undefNode)))
    };

    let typeOfTagDef((CompDef(comptype))) = |()| {
        typeOfCompDef(comptype)
    };

    let typeOfTagDef((EnumDef(enumtype))) = |()| {
        typeOfEnumDef(enumtype)
    };

}

mod Language_C_Analysis_TravMonad {
    struct CLanguage(C89, C99, GNU89, GNU99);

    struct TravOptions(TravOptions, { /* struct def */ });

    struct TravState(TravState, { /* struct def */ });

    let @(handleParamDecl(pd), (AbstractParamDecl(_, _))) = |()| {
        handleDecl((ParamEvent(pd)))
    };

    let @(handleParamDecl(pd), (ParamDecl(vardecl, node))) = |()| {
        {

            Let([Assign([Span([Ref(Ident("def"))])], Span([Ref(Ident("ObjectDef")), Parens([Span([Ref(Ident("ObjDef")), Ref(Ident("vardecl")), Ref(Ident("Nothing")), Ref(Ident("node"))])])]))], []);
            let redecl = withDefTable(defineScopedIdent((declIdent(def)), def));
            checkVarRedef(def, redecl);
            handleDecl((ParamEvent(pd)))
        }
    };

    let @(handleTypeDef(typeDef), (TypeDef(ident, _, _, _))) = |()| {
        {

            let redecl = withDefTable(defineTypeDef(ident, typeDef));
            checkRedef((show(ident)), typeDef, redecl);
            handleDecl((TypeDefEvent(typeDef)));
            return(())
        }
    };

    let addRef(use, def) = |()| {
        match (nodeInfo(use), nodeInfo(def)) {
                (NodeInfo(_, _, useName), NodeInfo(_, _, defName)) => { withDefTable((Lambda)) },
                (_, _) => { return(()) },
            }
    };

    let astError(node, msg) = |()| {
        throwTravError(invalidAST(node, msg))
    };

    let checkCompatibleTypes(_, _) = |()| {
        Right(())
    };

    let checkIdentTyRedef((Left(_tydef)), _) = |()| {
        return(())
    };

    let checkIdentTyRedef((Left(tydef)), (KindMismatch(old_def))) = |()| {
        redefErr((identOfTypeDef(tydef)), LevelError, tydef, old_def, DiffKindRedecl)
    };

    let checkIdentTyRedef((Left(tydef)), (Redeclared(old_def))) = |()| {
        redefErr((identOfTypeDef(tydef)), LevelError, tydef, old_def, DuplicateDef)
    };

    let checkIdentTyRedef((Right(decl)), status) = |()| {
        checkVarRedef(decl, status)
    };

    let checkRedef(subject, new_decl, redecl_status) = |()| {
        match redecl_status {
                NewDecl => { return(()) },
                Redeclared, old_def => { throwTravError(redefinition(LevelError, subject, DuplicateDef, (nodeInfo(new_decl)), (nodeInfo(old_def)))) },
                KindMismatch, old_def => { throwTravError(redefinition(LevelError, subject, DiffKindRedecl, (nodeInfo(new_decl)), (nodeInfo(old_def)))) },
                Shadowed, _old_def => { return(()) },
                KeepDef, _old_def => { return(()) },
            }
    };

    let checkVarRedef(def, redecl) = |()| {
        match redecl {
                KindMismatch, old_def => { redefVarErr(old_def, DiffKindRedecl) },
            KeepDef, Right(old_def) => if not((agreeOnLinkage(def, old_def))) { linkageErr(def, old_def) }
otherwise { throwOnLeft(checkCompatibleTypes(new_ty, (declType(old_def)))) },
            Redeclared, Right(old_def) => if not((agreeOnLinkage(def, old_def))) { linkageErr(def, old_def) }
not((canBeOverwritten(old_def))) { redefVarErr(old_def, DuplicateDef) }
otherwise { throwOnLeft(checkCompatibleTypes(new_ty, (declType(old_def)))) },
                _ => { return(()) },
            }
    };

    let concatMapM(f) = |()| {
        (liftM(concat) . mapM(f))
    };

    let createSUERef(_node_info, (Just(ident))) = |()| {
        return(NamedRef(ident))
    };

    fn enterBlockScope() -> m {
        updDefTable((ST.enterBlockScope))
    }

    let enterDecl(decl, cond) = |()| {
        {

            Let([Assign([Span([Ref(Ident("def"))])], Span([Ref(Ident("Declaration")), Ref(Ident("decl"))]))], []);
            let redecl = withDefTable(defineScopedIdentWhen(cond, (declIdent(def)), def));
            checkVarRedef(def, redecl);
            return(def)
        }
    };

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

    let gets(f) = |()| {
        Trav((Lambda((f(s), s))))
    };

    fn hadHardErrors() -> Bool {
        ((not . (null . filter(isHardError))))
    }

    let handleAsmBlock(asm) = |()| {
        handleDecl((AsmEvent(asm)))
    };

    let handleEnumeratorDef(enumerator) = |()| {
        {

            Let([Assign([Span([Ref(Ident("ident"))])], Span([Ref(Ident("declIdent")), Ref(Ident("enumerator"))]))], []);
            let redecl = withDefTable(defineScopedIdent(ident, (EnumeratorDef(enumerator))));
            checkRedef((show(ident)), ident, redecl);
            return(())
        }
    };

    let handleFunDef(ident, fun_def) = |()| {
        {

            Let([Assign([Span([Ref(Ident("def"))])], Span([Ref(Ident("FunctionDef")), Ref(Ident("fun_def"))]))], []);
            let redecl = withDefTable(defineScopedIdentWhen(isDeclaration, ident, def));
            checkVarRedef(def, redecl);
            handleDecl((DeclEvent(def)))
        }
    };

    let handleObjectDef(local, ident, obj_def) = |()| {
        {

            Let([Assign([Span([Ref(Ident("def"))])], Span([Ref(Ident("ObjectDef")), Ref(Ident("obj_def"))]))], []);
            let redecl = withDefTable(defineScopedIdentWhen((Lambda(def, old)), ident, def));
            checkVarRedef(def, redecl);
            handleDecl(((if(local, then, LocalEvent, else, DeclEvent))(def)));
            
        }
    };

    let handleTagDecl(decl) = |()| {
        {

            let redecl = withDefTable(declareTag((sueRef(decl)), decl));
            checkRedef((show(sueRef(decl))), decl, redecl)
        }
    };

    let handleTagDef(def) = |()| {
        {

            let redecl = withDefTable(defineTag((sueRef(def)), def));
            checkRedef((show(sueRef(def))), def, redecl);
            handleDecl((TagEvent(def)))
        }
    };

    let handleTravError(a) = |()| {
        catchTravError(liftM(Just, a), (>>(Lambda(e), return(Nothing))))
    };

    let handleVarDecl(is_local, decl) = |()| {
        {

            let def = enterDecl(decl, (const(False)));
            handleDecl(((if(is_local, then, LocalEvent, else, DeclEvent))(def)))
        }
    };

    let initTravState(userst) = |()| {
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
    };

    let isDeclaration((Declaration(_))) = |()| {
        True
    };

    let isDeclaration(_) = |()| {
        False
    };

    fn leaveBlockScope() -> m {
        updDefTable((ST.leaveBlockScope))
    }

    fn leaveFunctionScope() -> m {
        updDefTable((ST.leaveFunctionScope))
    }

    fn leavePrototypeScope() -> m {
        updDefTable((ST.leaveBlockScope))
    }

    let lookupObject(ident) = |()| {
        {

            let old_decl = liftM((lookupIdent(ident)), getDefTable);
            mapMaybeM(old_decl)(Lambda)
        }
    };

    let lookupTypeDef(ident) = |()| {
        >>=(getDefTable, Lambda)
    };

    let mapMaybeM(m, f) = |()| {
        maybe((return(Nothing)), ((liftM(Just) . f)), m)
    };

    let mapSndM(f, (a, b)) = |()| {
        liftM((Dummy(a)), (f(b)))
    };

    let maybeM(m, f) = |()| {
        maybe((return(())), f, m)
    };

    let mismatchErr(ctx, expect, found) = |()| {
        ++(ctx, ++(": Expected ".to_string(), ++(expect, ++(", but found: ".to_string(), found))))
    };

    let modify(f) = |()| {
        Trav((Lambda(((), f(s)))))
    };

    let modifyOptions(f) = |()| {
        modify(Lambda({
                options: f((options(ts)))
                }))
    };

    let modifyUserState(f) = |()| {
        modify(Lambda({
                userState: f((userState(ts)))
                }))
    };

    let put(s) = |()| {
        Trav((Lambda(((), s))))
    };

    let redefErr(name, lvl, new, old, kind) = |()| {
        throwTravError(redefinition(lvl, (show(name)), kind, (nodeInfo(new)), (nodeInfo(old))))
    };

    let runTrav(state, traversal) = |()| {
        match unTrav(action, (initTravState(state))) {
                Left, trav_err => { Left(vec![trav_err]) },
            Right, (v, ts) => if hadHardErrors((travErrors(ts))) { Left((travErrors(ts))) }
otherwise { Right((v, ts)) },
            }
    };

    let runTrav_(t) = |()| {
        (fmap(fst) . runTrav(())({

                    let r = t;
                    let es = getErrors;
                    return((r, es))
                }))
    };

    let throwOnLeft((Left(err))) = |()| {
        throwTravError(err)
    };

    let throwOnLeft((Right(v))) = |()| {
        return(v)
    };

    fn travErrors() -> TravState {
        (RList.reverse . rerrors)
    }

    let updDefTable(f) = |()| {
        withDefTable((Lambda))
    };

    let warn(err) = |()| {
        recordError((changeErrorLevel(err, LevelWarn)))
    };

    let withExtDeclHandler(action, handler) = |()| {
        {

            modify(Lambda({
        doHandleExtDecl: handler
        }));
            action
        }
    };

}

mod Language_C_Analysis_TypeCheck {
    let @(compositeType(t1), @((DirectType(tn1, q1, a1))(t2), (DirectType(tn2, q2, a2)))) = |()| {
        {

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
        }
    };

    let assignCompatible'(ni, op, t1, t2) = |()| {
        typeErrorOnLeft(ni, (assignCompatible(op, t1, t2)))
    };

    let assignCompatible(CAssignOp, t1, t2) = |()| {
        match (canonicalType(t1), canonicalType(t2)) {
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
            }
    };

    let assignCompatible(op, t1, t2) = |()| {
        >>(binopType((assignBinop(op)), t1, t2), return(()))
    };

    let binopType'(ni, op, t1, t2) = |()| {
        typeErrorOnLeft(ni, (binopType(op, t1, t2)))
    };

    let binopType(op, t1, t2) = |()| {
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
    };

    let castCompatible(t1, t2) = |()| {
        match (canonicalType(t1), canonicalType(t2)) {
                (DirectType(TyVoid, _, _), _) => { return(()) },
                (_, _) => { >>(checkScalar(t1), checkScalar(t2)) },
            }
    };

    let checkIntegral'(ni) = |()| {
        (typeErrorOnLeft(ni) . checkIntegral)
    };

    let checkScalar'(ni) = |()| {
        (typeErrorOnLeft(ni) . checkScalar)
    };

    let checkScalar(t) = |()| {
        match canonicalType(t) {
                DirectType, _, _, _ => { return(()) },
                PtrType, _, _, _ => { return(()) },
                ArrayType, _, _, _, _ => { return(()) },
                t' => { fail(++("expected scalar type, got: ".to_string(), ++(pType(t), ++(" (".to_string(), ++(pType(t'), ")".to_string()))))) },
            }
    };

    let compatible(t1, t2) = |()| {
        >>(compositeType(t1, t2), return(()))
    };

    let compositeDeclAttrs((DeclAttrs(inl, stor, attrs1)), (DeclAttrs(_, _, attrs2))) = |()| {
        DeclAttrs(inl, stor, (mergeAttrs(attrs1, attrs2)))
    };

    let compositeParamDecl'(f, (VarDecl(n1, attrs1, t1)), (VarDecl(n2, attrs2, t2)), dni) = |()| {
        {

            let vd = compositeVarDecl((VarDecl(n1, attrs1, t1')), (VarDecl(n2, attrs2, t2')));
            return(f(vd, dni))
        }
    };

    let compositeParamDecl((AbstractParamDecl(vd1, _)), (ParamDecl(vd2, ni2))) = |()| {
        compositeParamDecl'(ParamDecl, vd1, vd2, ni2)
    };

    let compositeParamDecl((AbstractParamDecl(vd1, ni1)), (AbstractParamDecl(vd2, _))) = |()| {
        compositeParamDecl'(AbstractParamDecl, vd1, vd2, ni1)
    };

    let compositeParamDecl((ParamDecl(vd1, ni1)), (AbstractParamDecl(vd2, _))) = |()| {
        compositeParamDecl'(ParamDecl, vd1, vd2, ni1)
    };

    let compositeParamDecl((ParamDecl(vd1, ni1)), (ParamDecl(vd2, _))) = |()| {
        compositeParamDecl'(ParamDecl, vd1, vd2, ni1)
    };

    let compositeSize((UnknownArraySize(_)), s2) = |()| {
        return(s2)
    };

    let compositeSize(s1, (UnknownArraySize(_))) = |()| {
        return(s1)
    };

    let compositeType((ArrayType(t1, s1, q1, a1)), (ArrayType(t2, s2, q2, a2))) = |()| {
        {

            let t = compositeType(t1, t2);
            let s = compositeSize(s1, s2);
            Let([Assign([Span([Ref(Ident("quals"))])], Span([Ref(Ident("mergeTypeQuals")), Ref(Ident("q1")), Ref(Ident("q2"))])), Assign([Span([Ref(Ident("attrs"))])], Span([Ref(Ident("mergeAttrs")), Ref(Ident("a1")), Ref(Ident("a2"))]))], []);
            return((ArrayType(t, s, quals, attrs)))
        }
    };

    let compositeType((DirectType((TyBuiltin(TyAny)), _, _)), t2) = |()| {
        return(t2)
    };

    let compositeType((FunctionType(ft1, attrs1)), (FunctionType(ft2, attrs2))) = |()| {
        match (ft1, ft2) {
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
            }
    };

    let compositeType((PtrType((DirectType(TyVoid, _, _)), q1, _)), (PtrType(t2, q2, a2))) = |()| {
        return(PtrType(t2, (mergeTypeQuals(q1, q2)), a2))
    };

    let compositeType((PtrType(t1, q1, a1)), (PtrType((DirectType(TyVoid, _, _)), q2, _))) = |()| {
        return(PtrType(t1, (mergeTypeQuals(q1, q2)), a1))
    };

    let compositeType((TypeDefType(tdr1, q1, a1)), (TypeDefType(tdr2, q2, a2))) = |()| {
        match (tdr1, tdr2) {
                (TypeDefRef(i1, Nothing, _), TypeDefRef(i2, _, _)) => { doTypeDef(i1, i2, tdr1) },
                (TypeDefRef(i1, _, _), TypeDefRef(i2, Nothing, _)) => { doTypeDef(i1, i2, tdr2) },
                (TypeDefRef(_, Just(t1), _), TypeDefRef(_, Just(t2), _)) => { compositeType(t1, t2) },
            }
    };

    let compositeType(t1, (DirectType((TyBuiltin(TyAny)), _, _))) = |()| {
        return(t1)
    };

    let compositeType(t1, t2) = |()| {
        fail(++("incompatible types: ".to_string(), ++(pType(t1), ++(", ".to_string(), pType(t2)))))
    };

    let compositeVarDecl((VarDecl(n1, attrs1, t1)), (VarDecl(_, attrs2, t2))) = |()| {
        {

            let t = compositeType(t1, t2);
            return((VarDecl(n1, (compositeDeclAttrs(attrs1, attrs2)), t)))
        }
    };

    let conditionalType'(ni, t1, t2) = |()| {
        typeErrorOnLeft(ni)(conditionalType(t1, t2))
    };

    let conditionalType(t1, t2) = |()| {
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
    };

    let constType((CCharConst((CChar(_, False)), _))) = |()| {
        return(DirectType((TyIntegral(TyChar)), noTypeQuals, noAttributes))
    };

    let constType((CCharConst((CChar(_, True)), _))) = |()| {
        return(DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes))
    };

    let constType((CCharConst((CChars(_, _)), _))) = |()| {
        return(DirectType((TyIntegral(TyInt)), noTypeQuals, noAttributes))
    };

    let constType((CFloatConst((CFloat(fs)), _))) = |()| {
        return(DirectType((TyFloating((getFloatType(fs)))), noTypeQuals, noAttributes))
    };

    let constType((CIntConst((CInteger(_, _, flags)), _))) = |()| {
        return(DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes))
    };

    let constType((CStrConst((CString(chars, wide)), ni))) = |()| {
        {

            let n = genName;
            Let([GuardAssign, Assign([Span([Ref(Ident("ni\'"))])], Span([Ref(Ident("mkNodeInfo")), Parens([Span([Ref(Ident("posOf")), Ref(Ident("ni"))])]), Ref(Ident("n"))])), Assign([Span([Ref(Ident("arraySize"))])], Span([Ref(Ident("ArraySize")), Ref(Ident("True")), Parens([Span([Ref(Ident("CConst")), Parens([Span([Ref(Ident("CIntConst")), Parens([Span([Ref(Ident("cInteger")), Parens([Span([Ref(Ident("toInteger")), Parens([Span([Ref(Ident("length")), Ref(Ident("chars"))])])])])])]), Ref(Ident("ni\'"))])])])])]))], []);
            return(ArrayType((DirectType((TyIntegral(charType)), noTypeQuals, noAttributes)), arraySize, noTypeQuals, vec![]))
        }
    };

    let deepTypeAttrs((ArrayType(t, _, _, attrs))) = |()| {
        liftM((attrs(Operator("++"))), deepTypeAttrs(t))
    };

    let deepTypeAttrs((DirectType((TyComp((CompTypeRef(sue, _, ni)))), _, attrs))) = |()| {
        liftM((attrs(Operator("++"))), sueAttrs(ni, sue))
    };

    let deepTypeAttrs((DirectType((TyEnum((EnumTypeRef(sue, ni)))), _, attrs))) = |()| {
        liftM((attrs(Operator("++"))), sueAttrs(ni, sue))
    };

    let deepTypeAttrs((DirectType(_, _, attrs))) = |()| {
        return(attrs)
    };

    let deepTypeAttrs((FunctionType((FunType(t, _, _)), attrs))) = |()| {
        liftM((attrs(Operator("++"))), deepTypeAttrs(t))
    };

    let deepTypeAttrs((FunctionType((FunTypeIncomplete(t)), attrs))) = |()| {
        liftM((attrs(Operator("++"))), deepTypeAttrs(t))
    };

    let deepTypeAttrs((PtrType(t, _, attrs))) = |()| {
        liftM((attrs(Operator("++"))), deepTypeAttrs(t))
    };

    let deepTypeAttrs((TypeDefType((TypeDefRef(i, _, ni)), _, attrs))) = |()| {
        liftM((attrs(Operator("++"))), typeDefAttrs(ni, i))
    };

    let derefType((ArrayType(t, _, _, _))) = |()| {
        return(t)
    };

    let derefType((PtrType(t, _, _))) = |()| {
        return(t)
    };

    let derefType(t) = |()| {
        match canonicalType(t) {
                PtrType, t', _, _ => { return(t') },
                ArrayType, t', _, _, _ => { return(t') },
                _ => { fail(++("dereferencing non-pointer: ".to_string(), pType(t))) },
            }
    };

    let expandAnonymous(_, (NoName, _)) = |()| {
        return(vec![])
    };

    let expandAnonymous(_, (VarName(n, _), t)) = |()| {
        return(vec![(n, t)])
    };

    let expandAnonymous(ni, (NoName, DirectType((TyComp(ctr)), _, _))) = |()| {
        >>=(lookupSUE(ni, (sueRef(ctr))), tagMembers(ni))
    };

    let fieldType(ni, m, t) = |()| {
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
    };

    let lookupSUE(ni, sue) = |()| {
        {

            let dt = getDefTable;
            match lookupTag(sue, dt) {
        Just, Right(td) => { return(td) },
        _ => { typeError(ni)(++("unknown composite type: ".to_string(), ((render . pretty))(sue))) },
    }
        }
    };

    fn mergeAttrs() -> Attributes {
        (Operator("++"))
    }

    let notFound(i) = |()| {
        fail(++("not found: ".to_string(), identToString(i)))
    };

    fn pType() -> String {
        (render . pretty)
    }

    let sizeEqual((CConst((CIntConst(i1, _)))), (CConst((CIntConst(i2, _))))) = |()| {
        ==(i1, i2)
    };

    let sizeEqual(e1, e2) = |()| {
        ==(nodeInfo(e1), nodeInfo(e2))
    };

    let sueAttrs(ni, sue) = |()| {
        {

            let dt = getDefTable;
            match lookupTag(sue, dt) {
        Nothing => { astError(ni)(++("SUE not found: ".to_string(), render((pretty(sue))))) },
        Just, Left(_) => { return(vec![]) },
        Just, Right(CompDef(CompType(_, _, _, attrs, _))) => { return(attrs) },
        Just, Right(EnumDef(EnumType(_, _, attrs, _))) => { return(attrs) },
    }
        }
    };

    let tagMembers(ni, td) = |()| {
        match td {
                CompDef, CompType(_, _, ms, _, _) => { getMembers(ms) },
                EnumDef, EnumType(_, es, _, _) => { getMembers(es) },
            }
    };

    let typeDefAttrs(ni, i) = |()| {
        {

            let dt = getDefTable;
            match lookupIdent(i, dt) {
        Nothing => { astError(ni)(++("can\'t find typedef name: ".to_string(), identToString(i))) },
        Just, Left(TypeDef(_, t, attrs, _)) => { liftM((attrs(Operator("++"))), deepTypeAttrs(t)) },
        Just, Right(_) => { astError(ni)(++("not a typedef name: ".to_string(), identToString(i))) },
    }
        }
    };

    fn typeError() -> MonadCError {
        astError
    }

    let typeErrorOnLeft(_, (Right(v))) = |()| {
        return(v)
    };

    let typeErrorOnLeft(ni, (Left(err))) = |()| {
        typeError(ni, err)
    };

    let varAddrType(d) = |()| {
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
    };

}

mod Language_C_Analysis_TypeConversions {
    let @(arithmeticConversion((TyIntegral(_)), t2), (TyComplex(_))) = |()| {
        Just(t2)
    };

    let @(arithmeticConversion((TyIntegral(_)), t2), (TyFloating(_))) = |()| {
        Just(t2)
    };

    let @(arithmeticConversion(t1), (TyComplex(_))((TyIntegral(_)))) = |()| {
        Just(t1)
    };

    let @(arithmeticConversion(t1), (TyFloating(_))((TyIntegral(_)))) = |()| {
        Just(t1)
    };

    let arithmeticConversion((TyComplex(t1)), (TyComplex(t2))) = |()| {
        Just(TyComplex(floatConversion(t1, t2)))
    };

    let arithmeticConversion((TyComplex(t1)), (TyFloating(t2))) = |()| {
        Just(TyComplex(floatConversion(t1, t2)))
    };

    let arithmeticConversion((TyEnum(_)), (TyEnum(_))) = |()| {
        Just(TyIntegral(TyInt))
    };

    let arithmeticConversion((TyEnum(_)), t2) = |()| {
        Just(t2)
    };

    let arithmeticConversion((TyFloating(t1)), (TyComplex(t2))) = |()| {
        Just(TyComplex(floatConversion(t1, t2)))
    };

    let arithmeticConversion((TyFloating(t1)), (TyFloating(t2))) = |()| {
        Just(TyFloating(floatConversion(t1, t2)))
    };

    let arithmeticConversion((TyIntegral(t1)), (TyIntegral(t2))) = |()| {
        Just(TyIntegral(intConversion(t1, t2)))
    };

    let arithmeticConversion(_, _) = |()| {
        Nothing
    };

    let arithmeticConversion(t1, (TyEnum(_))) = |()| {
        Just(t1)
    };

    fn floatConversion() -> FloatType {
        max
    }

    let intConversion(t1, t2) = |()| {
        max(TyInt, (max(t1, t2)))
    };

}

mod Language_C_Analysis_TypeUtils {
    let baseType((ArrayType(t, _, _, _))) = |()| {
        t
    };

    let baseType((PtrType(t, _, _))) = |()| {
        t
    };

    let baseType(_) = |()| {
        error("base of non-pointer type".to_string())
    };

    fn boolType() -> Type {
        integral(TyInt)
    }

    let canonicalType(t) = |()| {
        match deepDerefTypeDef(t) {
                FunctionType, ft, attrs => { simplePtr((FunctionType(ft, attrs))) },
                t' => { t' },
            }
    };

    fn charPtr() -> Type {
        simplePtr((integral(TyChar)))
    }

    fn constCharPtr() -> Type {
        constPtr((integral(TyChar)))
    }

    let constPtr(t) = |()| {
        PtrType(t, (TypeQuals(True, False, False)), vec![])
    };

    fn constVoidPtr() -> Type {
        constPtr(voidType)
    }

    let deepDerefTypeDef((ArrayType(t, size, quals, attrs))) = |()| {
        ArrayType((deepDerefTypeDef(t)), size, quals, attrs)
    };

    let deepDerefTypeDef((FunctionType((FunType(rt, params, varargs)), attrs))) = |()| {
        FunctionType((FunType((deepDerefTypeDef(rt)), params, varargs)), attrs)
    };

    let deepDerefTypeDef((FunctionType((FunTypeIncomplete(rt)), attrs))) = |()| {
        FunctionType((FunTypeIncomplete((deepDerefTypeDef(rt)))), attrs)
    };

    let deepDerefTypeDef((PtrType(t, quals, attrs))) = |()| {
        PtrType((deepDerefTypeDef(t)), quals, attrs)
    };

    let deepDerefTypeDef((TypeDefType((TypeDefRef(_, (Just(t)), _)), q, a))) = |()| {
        ((typeAttrsUpd((mergeAttributes(a))) . typeQualsUpd((mergeTypeQuals(q)))))((deepDerefTypeDef(t)))
    };

    let deepDerefTypeDef(t) = |()| {
        t
    };

    let derefTypeDef((TypeDefType((TypeDefRef(_, (Just(t)), _)), q, a))) = |()| {
        ((typeAttrsUpd((mergeAttributes(a))) . typeQualsUpd((mergeTypeQuals(q)))))((derefTypeDef(t)))
    };

    let derefTypeDef(ty) = |()| {
        ty
    };

    let floating(ty) = |()| {
        DirectType((TyFloating(ty)), noTypeQuals, noAttributes)
    };

    let integral(ty) = |()| {
        DirectType((TyIntegral(ty)), noTypeQuals, noAttributes)
    };

    let isFloatingType((DirectType((TyFloating(_)), _, _))) = |()| {
        True
    };

    let isFloatingType(_) = |()| {
        False
    };

    let isFunctionType(ty) = |()| {
        match ty {
                TypeDefType, TypeDefRef(_, Just(actual_ty), _), _, _ => { isFunctionType(actual_ty) },
                TypeDefType, _, _, _ => { error("isFunctionType: unresolved typeDef".to_string()) },
                FunctionType, _, _ => { True },
                _ => { False },
            }
    };

    let isIntegralType((DirectType((TyEnum(_)), _, _))) = |()| {
        True
    };

    let isIntegralType((DirectType((TyIntegral(_)), _, _))) = |()| {
        True
    };

    let isIntegralType(_) = |()| {
        False
    };

    let isPointerType((ArrayType(_, _, _, _))) = |()| {
        True
    };

    let isPointerType((PtrType(_, _, _))) = |()| {
        True
    };

    let isPointerType(_) = |()| {
        False
    };

    let isScalarType(t) = |()| {
        ||(isIntegralType(t), ||(isPointerType(t), isFloatingType(t)))
    };

    fn ptrDiffType() -> Type {
        integral(TyInt)
    }

    let simplePtr(t) = |()| {
        PtrType(t, noTypeQuals, vec![])
    };

    fn size_tType() -> Type {
        integral(TyInt)
    }

    fn stringType() -> Type {
        ArrayType((DirectType((TyIntegral(TyChar)), (TypeQuals(True, False, False)), noAttributes)), (UnknownArraySize(False)), noTypeQuals, vec![])
    }

    let testFlags(flags, fi) = |()| {
        and(map(((flip(testFlag))(fi)), flags))
    };

    let typeAttrs((ArrayType(_, _, _, a))) = |()| {
        a
    };

    let typeAttrs((DirectType(_, _, a))) = |()| {
        a
    };

    let typeAttrs((FunctionType(_, a))) = |()| {
        a
    };

    let typeAttrs((PtrType(_, _, a))) = |()| {
        a
    };

    let typeAttrs((TypeDefType((TypeDefRef(_, (Just(t)), _)), _, a))) = |()| {
        mergeAttributes(a, (typeAttrs(t)))
    };

    let typeAttrs((TypeDefType((TypeDefRef(_, Nothing, _)), _, a))) = |()| {
        a
    };

    let typeAttrsUpd(f, ty) = |()| {
        match ty {
                DirectType, ty_name, ty_quals, ty_attrs => { DirectType(ty_name, ty_quals, (f(ty_attrs))) },
                PtrType, ty_inner, ty_quals, ty_attrs => { PtrType(ty_inner, ty_quals, (f(ty_attrs))) },
                ArrayType, ty_inner, sz, ty_quals, ty_attrs => { ArrayType(ty_inner, sz, ty_quals, (f(ty_attrs))) },
                FunctionType, ty_inner, ty_attrs => { FunctionType(ty_inner, (f(ty_attrs))) },
                TypeDefType, ty_ref, ty_quals, ty_attrs => { TypeDefType(ty_ref, ty_quals, (f(ty_attrs))) },
            }
    };

    let typeQuals((ArrayType(_, _, q, _))) = |()| {
        q
    };

    let typeQuals((DirectType(_, q, _))) = |()| {
        q
    };

    let typeQuals((FunctionType(_, _))) = |()| {
        noTypeQuals
    };

    let typeQuals((PtrType(_, q, _))) = |()| {
        q
    };

    let typeQuals((TypeDefType((TypeDefRef(_, (Just(t)), _)), q, _))) = |()| {
        mergeTypeQuals(q, (typeQuals(t)))
    };

    let typeQuals((TypeDefType((TypeDefRef(_, Nothing, _)), q, _))) = |()| {
        q
    };

    let typeQualsUpd(f, ty) = |()| {
        match ty {
                DirectType, ty_name, ty_quals, ty_attrs => { DirectType(ty_name, (f(ty_quals)), ty_attrs) },
                PtrType, ty_inner, ty_quals, ty_attrs => { PtrType(ty_inner, (f(ty_quals)), ty_attrs) },
                ArrayType, ty_inner, sz, ty_quals, ty_attrs => { ArrayType(ty_inner, sz, (f(ty_quals)), ty_attrs) },
                FunctionType, ty_inner, ty_attrs => { FunctionType(ty_inner, ty_attrs) },
                TypeDefType, ty_ref, ty_quals, ty_attrs => { TypeDefType(ty_ref, (f(ty_quals)), ty_attrs) },
            }
    };

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

    let internalErr(msg) = |()| {
        error((++(internalErrPrefix, ++("\n".to_string(), ++(indentLines(msg), "\n".to_string())))))
    };

    fn internalErrPrefix() -> String {
        unlines(vec!["Language.C : Internal Error".to_string(), ++("This is propably a bug, and should be reported at ".to_string(), "http://www.sivity.net/projects/language.c/newticket".to_string())])
    }

    fn isHardError() -> Bool {
        ((Operator(">")(LevelWarn)) . errorLevel)
    }

    let mkErrorInfo(lvl, msg, node) = |()| {
        ErrorInfo(lvl, (posOfNode(node)), (lines(msg)))
    };

    let showError(short_msg) = |()| {
        (showErrorInfo(short_msg) . errorInfo)
    };

    let showErrorInfo(short_msg, (ErrorInfo(level, pos, msgs))) = |()| {
        ++(header, showMsgLines((if(null, short_msg, then, msgs, else, short_msg:msgs))))
    };

    let unsupportedFeature(msg, a) = |()| {
        UnsupportedFeature(msg, (posOf(a)))
    };

    let unsupportedFeature_(msg) = |()| {
        UnsupportedFeature(msg, internalPos)
    };

    let userErr(msg) = |()| {
        UserError((ErrorInfo(LevelError, internalPos, (lines(msg)))))
    };

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

    let builtinIdent(s) = |()| {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
    };

    let dumpIdent(ide) = |()| {
        ++(identToString(ide), ++(" at ".to_string(), show((nodeInfo(ide)))))
    };

    let identToString((Ident(s, _, _))) = |()| {
        s
    };

    let internalIdent(s) = |()| {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
    };

    let internalIdentAt(pos, s) = |()| {
        Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
    };

    let isAnonymousRef((AnonymousRef(_))) = |()| {
        True
    };

    let isAnonymousRef(_) = |()| {
        False
    };

    let isInternalIdent((Ident(_, _, nodeinfo))) = |()| {
        isInternalPos((posOfNode(nodeinfo)))
    };

    let mkIdent(pos, s, name) = |()| {
        Ident(s, (quad(s)), (mkNodeInfo'(pos, (pos, length(s)), name)))
    };

    let quad((c1:(vec![]))) = |()| {
        ord(c1)
    };

    let quad((c1:c2:(vec![]))) = |()| {
        *(ord(c2), +(bits7, ord(c1)))
    };

    let quad((c1:c2:c3:(vec![]))) = |()| {
        *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))
    };

    let quad((c1:c2:c3:c4:s)) = |()| {
        +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28)))
    };

    let quad((vec![])) = |()| {
        0
    };

}

mod Language_C_Data_InputStream {
    let !(takeChars, n(bstr)) = |()| {
        BSC.unpack(BSC.take(n, bstr))
    };

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

    let takeByte(bs) = |()| {
        seq(BSW.head(bs), (BSW.head(bs), BSW.tail(bs)))
    };

    let takeChar(bs) = |()| {
        match () {
            <todo> => { seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs))) },
            <todo> => { (head(bs), tail(bs)) },
        }
    };

    let takeChars(n, str) = |()| {
        take(n, str)
    };

}

mod Language_C_Data_Name {
    let namesStartingFrom(k) = |()| {
        vec![Name(k..)]
    };

    fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    }

}

mod Language_C_Data_Node {
    #[derive(Clone, Debug)]
    struct NodeInfo(OnlyPos, Position, PosLength, NodeInfo, Position, PosLength, Name);

    let eqByName(obj1, obj2) = |()| {
        ==((nodeInfo(obj1)), (nodeInfo(obj2)))
    };

    fn fileOfNode() -> Maybe {
        (fmap(posFile) . (justIf(isSourcePos) . (posOfNode . nodeInfo)))
    }

    let getLastTokenPos((NodeInfo(_, lastTok, _))) = |()| {
        lastTok
    };

    let getLastTokenPos((OnlyPos(_, lastTok))) = |()| {
        lastTok
    };

    fn internalNode() -> NodeInfo {
        undefNode
    }

    let isUndefNode(_) = |()| {
        False
    };

    let lengthOfNode(ni) = |()| {
        len
    };

    let mkNodeInfo'(pos, lasttok, name) = |()| {
        NodeInfo(pos, lasttok, name)
    };

    let mkNodeInfo(pos, name) = |()| {
        NodeInfo(pos, (nopos, Operator("-")(1)), name)
    };

    let mkNodeInfoOnlyPos(pos) = |()| {
        OnlyPos(pos, (nopos, Operator("-")(1)))
    };

    fn mkNodeInfoPosLen() -> NodeInfo {
        OnlyPos
    }

    let nameOfNode((NodeInfo(_, _, name))) = |()| {
        Just(name)
    };

    let nameOfNode((OnlyPos(_, _))) = |()| {
        Nothing
    };

    let posOfNode(ni) = |()| {
        match ni {
                OnlyPos(pos, _) => { pos },
                NodeInfo(pos, _, _) => { pos },
            }
    };

    fn undefNode() -> NodeInfo {
        OnlyPos(nopos, (nopos, Operator("-")(1)))
    }

}

mod Language_C_Data_Position {
    #[derive(Clone, Debug, Eq, Ord)]
    struct Position(Position, { /* struct def */ }, NoPosition, BuiltinPosition, InternalPosition);

    let adjustPos(_, _, p) = |()| {
        p
    };

    let adjustPos(fname, row, (Position(offs, _, _, _))) = |()| {
        Position(offs, fname, row, 1)
    };

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    let incOffset((Position(o, f, r, c)), n) = |()| {
        Position((+(o, n)), f, r, c)
    };

    let incOffset(p, n) = |()| {
        p
    };

    let incPos((Position(offs, fname, row, col)), n) = |()| {
        Position((+(offs, n)), fname, row, (+(col, n)))
    };

    let incPos(p, _) = |()| {
        p
    };

    let initPos(file) = |()| {
        Position(0, file, 1, 1)
    };

    fn internalPos() -> Position {
        InternalPosition
    }

    let isBuiltinPos(BuiltinPosition) = |()| {
        True
    };

    let isBuiltinPos(_) = |()| {
        False
    };

    let isInternalPos(InternalPosition) = |()| {
        True
    };

    let isInternalPos(_) = |()| {
        False
    };

    let isNoPos(NoPosition) = |()| {
        True
    };

    let isNoPos(_) = |()| {
        False
    };

    let isSourcePos((Position(_, _, _, _))) = |()| {
        True
    };

    let isSourcePos(_) = |()| {
        False
    };

    fn nopos() -> Position {
        NoPosition
    }

    fn position() -> Position {
        Position
    }

    let retPos((Position(offs, fname, row, _))) = |()| {
        Position((+(offs, 1)), fname, (+(row, 1)), 1)
    };

    let retPos(p) = |()| {
        p
    };

}

mod Language_C_Data_RList {
    let appendr(xs, (Reversed(ys))) = |()| {
        Reversed((++(ys, List.reverse(xs))))
    };

    fn empty() -> Reversed {
        Reversed(vec![])
    }

    let rappend((Reversed(xs)), ys) = |()| {
        Reversed((++(List.reverse(ys), xs)))
    };

    let rappendr((Reversed(xs)), (Reversed(ys))) = |()| {
        Reversed((++(ys, xs)))
    };

    let reverse((Reversed(xs))) = |()| {
        List.reverse(xs)
    };

    let rmap(f, (Reversed(xs))) = |()| {
        Reversed((map(f, xs)))
    };

    let singleton(x) = |()| {
        Reversed(vec![x])
    };

    let snoc((Reversed(xs)), x) = |()| {
        Reversed((:(x, xs)))
    };

    let viewr((Reversed((x:xs)))) = |()| {
        (Reversed(xs), x)
    };

    let viewr((Reversed(vec![]))) = |()| {
        error("viewr: empty RList".to_string())
    };

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

    let addTypedef(ident) = |()| {
        (P(Lambda(s, {
                    tyidents: Set.insert(ident, tyids)
                    }, ())))
    };

    fn enterScope() -> P {
        P(Lambda(s, {
                scopes: tyids:ss
                }, ()))
    }

    let execParser((P(parser)), input, pos, builtins, names) = |()| {
        match parser(initialState) {
                PFailed, message, errpos => { Left((ParseError((message, errpos)))) },
                POk, st, result => { Right((result, namesupply(st))) },
            }
    };

    let failP(pos, msg) = |()| {
        P(Lambda(msg, pos))
    };

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

    let isTypeIdent(ident) = |()| {
        P($!(Lambda(s), Set.member(ident, tyids)))
    };

    fn leaveScope() -> P {
        P(Lambda)
    }

    let returnP(a) = |()| {
        P(Lambda(s, a))
    };

    let setInput(i) = |()| {
        P(Lambda(s, {
                curInput: i
                }, ()))
    };

    let setLastToken(CTokEof) = |()| {
        P(Lambda(s, {
                savedToken: (prevToken(s))
                }, ()))
    };

    let setLastToken(tok) = |()| {
        P(Lambda(s, {
                prevToken: tok,
                savedToken: (prevToken(s))
                }, ()))
    };

    let setPos(pos) = |()| {
        P(Lambda(s, {
                curPos: pos
                }, ()))
    };

    let shadowTypedef(ident) = |()| {
        (P(Lambda(s, {
                    tyidents: Set.member(if(ident), Set.delete(tyids(then, ident), tyids(else, tyids)))
                    }, ())))
    };

    let thenP((P(m)), k) = |()| {
        P(Lambda)
    };

}

mod Language_C_Parser_Tokens {
    struct CToken(CTokLParen, PosLength, CTokRParen, PosLength, CTokLBracket, PosLength, CTokRBracket, PosLength, CTokArrow, PosLength, CTokDot, PosLength, CTokExclam, PosLength, CTokTilde, PosLength, CTokInc, PosLength, CTokDec, PosLength, CTokPlus, PosLength, CTokMinus, PosLength, CTokStar, PosLength, CTokSlash, PosLength, CTokPercent, PosLength, CTokAmper, PosLength, CTokShiftL, PosLength, CTokShiftR, PosLength, CTokLess, PosLength, CTokLessEq, PosLength, CTokHigh, PosLength, CTokHighEq, PosLength, CTokEqual, PosLength, CTokUnequal, PosLength, CTokHat, PosLength, CTokBar, PosLength, CTokAnd, PosLength, CTokOr, PosLength, CTokQuest, PosLength, CTokColon, PosLength, CTokAssign, PosLength, CTokPlusAss, PosLength, CTokMinusAss, PosLength, CTokStarAss, PosLength, CTokSlashAss, PosLength, CTokPercAss, PosLength, CTokAmpAss, PosLength, CTokHatAss, PosLength, CTokBarAss, PosLength, CTokSLAss, PosLength, CTokSRAss, PosLength, CTokComma, PosLength, CTokSemic, PosLength, CTokLBrace, PosLength, CTokRBrace, PosLength, CTokEllipsis, PosLength, CTokAlignof, PosLength, CTokAsm, PosLength, CTokAuto, PosLength, CTokBreak, PosLength, CTokBool, PosLength, CTokCase, PosLength, CTokChar, PosLength, CTokConst, PosLength, CTokContinue, PosLength, CTokComplex, PosLength, CTokDefault, PosLength, CTokDo, PosLength, CTokDouble, PosLength, CTokElse, PosLength, CTokEnum, PosLength, CTokExtern, PosLength, CTokFloat, PosLength, CTokFor, PosLength, CTokGoto, PosLength, CTokIf, PosLength, CTokInline, PosLength, CTokInt, PosLength, CTokLong, PosLength, CTokLabel, PosLength, CTokRegister, PosLength, CTokRestrict, PosLength, CTokReturn, PosLength, CTokShort, PosLength, CTokSigned, PosLength, CTokSizeof, PosLength, CTokStatic, PosLength, CTokStruct, PosLength, CTokSwitch, PosLength, CTokTypedef, PosLength, CTokTypeof, PosLength, CTokThread, PosLength, CTokUnion, PosLength, CTokUnsigned, PosLength, CTokVoid, PosLength, CTokVolatile, PosLength, CTokWhile, PosLength, CTokCLit, PosLength, CChar, CTokILit, PosLength, CInteger, CTokFLit, PosLength, CFloat, CTokSLit, PosLength, CString, CTokIdent, PosLength, Ident, CTokTyIdent, PosLength, Ident, CTokGnuC, GnuCTok, PosLength, CTokEof);

    struct GnuCTok(GnuCAttrTok, GnuCExtTok, GnuCVaArg, GnuCOffsetof, GnuCTyCompat, GnuCComplexReal, GnuCComplexImag);

    let posLenOfTok((CTokAlignof(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokAmpAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokAmper(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokAnd(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokArrow(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokAsm(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokAssign(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokAuto(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokBar(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokBarAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokBool(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokBreak(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokCLit(pos, _))) = |()| {
        pos
    };

    let posLenOfTok((CTokCase(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokChar(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokColon(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokComma(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokComplex(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokConst(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokContinue(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokDec(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokDefault(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokDo(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokDot(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokDouble(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokEllipsis(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokElse(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokEnum(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokEqual(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokExclam(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokExtern(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokFLit(pos, _))) = |()| {
        pos
    };

    let posLenOfTok((CTokFloat(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokFor(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokGnuC(_, pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokGoto(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokHat(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokHatAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokHigh(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokHighEq(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokILit(pos, _))) = |()| {
        pos
    };

    let posLenOfTok((CTokIdent(pos, _))) = |()| {
        pos
    };

    let posLenOfTok((CTokIf(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokInc(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokInline(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokInt(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokLBrace(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokLBracket(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokLParen(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokLabel(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokLess(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokLessEq(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokLong(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokMinus(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokMinusAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokOr(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokPercAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokPercent(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokPlus(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokPlusAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokQuest(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokRBrace(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokRBracket(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokRParen(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokRegister(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokRestrict(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokReturn(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokSLAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokSLit(pos, _))) = |()| {
        pos
    };

    let posLenOfTok((CTokSRAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokSemic(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokShiftL(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokShiftR(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokShort(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokSigned(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokSizeof(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokSlash(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokSlashAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokStar(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokStarAss(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokStatic(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokStruct(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokSwitch(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokThread(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokTilde(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokTyIdent(pos, _))) = |()| {
        pos
    };

    let posLenOfTok((CTokTypedef(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokTypeof(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokUnequal(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokUnion(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokUnsigned(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokVoid(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokVolatile(pos))) = |()| {
        pos
    };

    let posLenOfTok((CTokWhile(pos))) = |()| {
        pos
    };

    let posLenOfTok(CTokEof) = |()| {
        error("tokenPos: Eof".to_string())
    };

}

mod Language_C_Parser {
    let execParser_(parser, input, pos) = |()| {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    };

}

mod Language_C_Pretty {
    let attrlistP(attrs) = |()| {
        <>(text("__attribute__".to_string()), parens((parens(((hcat . (punctuate(comma) . map(pretty)(attrs))))))))
    };

    let attrlistP(vec![]) = |()| {
        empty
    };

    let binPrec(CAddOp) = |()| {
        19
    };

    let binPrec(CAndOp) = |()| {
        15
    };

    let binPrec(CDivOp) = |()| {
        20
    };

    let binPrec(CEqOp) = |()| {
        16
    };

    let binPrec(CGeqOp) = |()| {
        17
    };

    let binPrec(CGrOp) = |()| {
        17
    };

    let binPrec(CLeOp) = |()| {
        17
    };

    let binPrec(CLeqOp) = |()| {
        17
    };

    let binPrec(CLndOp) = |()| {
        12
    };

    let binPrec(CLorOp) = |()| {
        11
    };

    let binPrec(CMulOp) = |()| {
        20
    };

    let binPrec(CNeqOp) = |()| {
        16
    };

    let binPrec(COrOp) = |()| {
        13
    };

    let binPrec(CRmdOp) = |()| {
        20
    };

    let binPrec(CShlOp) = |()| {
        18
    };

    let binPrec(CShrOp) = |()| {
        18
    };

    let binPrec(CSubOp) = |()| {
        19
    };

    let binPrec(CXorOp) = |()| {
        14
    };

    fn identP() -> Doc {
        (text . identToString)
    }

    let ifP(flag, doc) = |()| {
        if(flag, then, doc, else, empty)
    };

    fn ii() -> Doc {
        nest(4)
    }

    fn maybeP() -> Maybe {
        maybe(empty)
    }

    let mlistP(pp, xs) = |()| {
        maybeP(pp, (if(null, xs, then, Nothing, else, Just, xs)))
    };

    let parenPrec(prec, prec2, t) = |()| {
        <=(if(prec), prec2(then, t, else, parens, t))
    };

    let prettyDeclr(show_attrs, prec, (CDeclr(name, derived_declrs, asmname, cattrs, _))) = |()| {
        <+>(ppDeclr(prec, (reverse(derived_declrs))), <+>(prettyAsmName(asmname), ifP(show_attrs, (attrlistP(cattrs)))))
    };

    let prettyUsingInclude((CTranslUnit(edecls, _))) = |()| {
        $$(includeWarning(headerFiles), (vcat(map((either(includeHeader, pretty)), mappedDecls))))
    };

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

    let cstringOfLit((CStrLit(cstr, _))) = |()| {
        cstr
    };

    let fmapInitList(_f) = |()| {
        map((Lambda))
    };

    let isSUEDef((CEnumType((CEnum(_, (Just(_)), _, _)), _))) = |()| {
        True
    };

    let isSUEDef((CSUType((CStruct(_, _, (Just(_)), _, _)), _))) = |()| {
        True
    };

    let isSUEDef(_) = |()| {
        False
    };

    let liftStrLit((CStrLit(str, at))) = |()| {
        CStrConst(str, at)
    };

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

    let _showWideFlag(flag) = |()| {
        if(flag, then, showString, "L".to_string(), else, id)
    };

    let cChar(c) = |()| {
        CChar(c, False)
    };

    let cChar_w(c) = |()| {
        CChar(c, True)
    };

    fn cChars() -> CChar {
        CChars
    }

    fn cFloat() -> CFloat {
        (CFloat . show)
    }

    let cInteger(i) = |()| {
        CInteger(i, DecRepr, noFlags)
    };

    let cString(str) = |()| {
        CString(str, False)
    };

    let cString_w(str) = |()| {
        CString(str, True)
    };

    let clearFlag(flag, (Flags(k))) = |()| {
        Flags(clearBit(k, fromEnum(flag)))
    };

    let concatCStrings(cs) = |()| {
        CString((concatMap(getCString, cs)), (any(isWideString, cs)))
    };

    let dQuote(s, t) = |()| {
        ++((:('\"', s)), ++("\"".to_string(), t))
    };

    let escapeCChar('\'') = |()| {
        "\\\'".to_string()
    };

    let escapeChar('\\') = |()| {
        "\\\\".to_string()
    };

    let escapeChar('\n') = |()| {
        "\\n".to_string()
    };

    let escapeChar('\r') = |()| {
        "\\r".to_string()
    };

    let escapeChar('\t') = |()| {
        "\\t".to_string()
    };

    let escapeChar('\u{1b}') = |()| {
        "\\e".to_string()
    };

    let escapeChar('\u{7}') = |()| {
        "\\a".to_string()
    };

    let escapeChar('\u{8}') = |()| {
        "\\b".to_string()
    };

    let escapeChar('\u{b}') = |()| {
        "\\v".to_string()
    };

    let escapeChar('\u{c}') = |()| {
        "\\f".to_string()
    };

    let getCChar((CChar(c, _))) = |()| {
        vec![c]
    };

    let getCChar((CChars(cs, _))) = |()| {
        cs
    };

    let getCCharAsInt((CChar(c, _))) = |()| {
        fromIntegral((fromEnum(c)))
    };

    let getCCharAsInt((CChars(_cs, _))) = |()| {
        error("integer value of multi-character character constants is implementation defined".to_string())
    };

    let getCInteger((CInteger(i, _, _))) = |()| {
        i
    };

    let getCString((CString(str, _))) = |()| {
        str
    };

    let head'(_, (x:_)) = |()| {
        x
    };

    let head'(err, vec![]) = |()| {
        error(err)
    };

    let isAsciiSourceChar(c) = |()| {
        &&(isAscii(c), isPrint(c))
    };

    let isCChar('\'') = |()| {
        False
    };

    let isCChar('\\') = |()| {
        False
    };

    let isCChar('\n') = |()| {
        False
    };

    let isCChar(c) = |()| {
        isAsciiSourceChar(c)
    };

    let isSChar('\"') = |()| {
        False
    };

    let isSChar('\\') = |()| {
        False
    };

    let isSChar('\n') = |()| {
        False
    };

    let isSChar(c) = |()| {
        isAsciiSourceChar(c)
    };

    let isWideChar((CChar(_, wideFlag))) = |()| {
        wideFlag
    };

    let isWideChar((CChars(_, wideFlag))) = |()| {
        wideFlag
    };

    let isWideString((CString(_, wideflag))) = |()| {
        wideflag
    };

    fn noFlags() -> Flags {
        Flags(0)
    }

    fn readCFloat() -> CFloat {
        CFloat
    }

    let readCInteger(repr, str) = |()| {
        match readNum(str) {
                [(n, suffix)] => { mkCInt(n, suffix) },
                parseFailed => { Left(++("Bad Integer literal: ".to_string(), show(parseFailed))) },
            }
    };

    let sQuote(s, t) = |()| {
        ++("\'".to_string(), ++(s, ++("\'".to_string(), t)))
    };

    let setFlag(flag, (Flags(k))) = |()| {
        Flags(setBit(k, fromEnum(flag)))
    };

    let showCharConst(c) = |()| {
        sQuote(escapeCChar(c))
    };

    fn showStringLit() -> ShowS {
        (dQuote . concatMap(showStringChar))
    }

    let testFlag(flag, (Flags(k))) = |()| {
        testBit(k, fromEnum(flag))
    };

    let unescapeChar((:('\\', c:cs))) = |()| {
        match c {
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
            }
    };

    let unescapeChar((:(c, cs))) = |()| {
        (c, cs)
    };

    let unescapeChar(vec![]) = |()| {
        error("unescape char: empty string".to_string())
    };

    let unescapeString(cs) = |()| {
        match unescapeChar(cs) {
                (c, cs') => { :(c, unescapeString(cs')) },
            }
    };

    let unescapeString(vec![]) = |()| {
        vec![]
    };

}

mod Language_C_Syntax_Ops {
    #[derive(Clone, Debug, Eq, Ord)]
    struct CAssignOp(CAssignOp, CMulAssOp, CDivAssOp, CRmdAssOp, CAddAssOp, CSubAssOp, CShlAssOp, CShrAssOp, CAndAssOp, CXorAssOp, COrAssOp);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CBinaryOp(CMulOp, CDivOp, CRmdOp, CAddOp, CSubOp, CShlOp, CShrOp, CLeOp, CGrOp, CLeqOp, CGeqOp, CEqOp, CNeqOp, CAndOp, CXorOp, COrOp, CLndOp, CLorOp);

    #[derive(Clone, Debug, Eq, Ord)]
    struct CUnaryOp(CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp, CAdrOp, CIndOp, CPlusOp, CMinOp, CCompOp, CNegOp);

    let assignBinop(CAddAssOp) = |()| {
        CAddOp
    };

    let assignBinop(CAndAssOp) = |()| {
        CAndOp
    };

    let assignBinop(CAssignOp) = |()| {
        error("direct assignment has no binary operator".to_string())
    };

    let assignBinop(CDivAssOp) = |()| {
        CDivOp
    };

    let assignBinop(CMulAssOp) = |()| {
        CMulOp
    };

    let assignBinop(COrAssOp) = |()| {
        COrOp
    };

    let assignBinop(CRmdAssOp) = |()| {
        CRmdOp
    };

    let assignBinop(CShlAssOp) = |()| {
        CShlOp
    };

    let assignBinop(CShrAssOp) = |()| {
        CShrOp
    };

    let assignBinop(CSubAssOp) = |()| {
        CSubOp
    };

    let assignBinop(CXorAssOp) = |()| {
        CXorOp
    };

    let isBitOp(op) = |()| {
        elem(op, vec![CShlOp, CShrOp, CAndOp, COrOp, CXorOp])
    };

    let isCmpOp(op) = |()| {
        elem(op, vec![CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp])
    };

    let isEffectfulOp(op) = |()| {
        elem(op, vec![CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp])
    };

    let isLogicOp(op) = |()| {
        elem(op, vec![CLndOp, CLorOp])
    };

    let isPtrOp(op) = |()| {
        elem(op, vec![CAddOp, CSubOp])
    };

}

mod Language_C_Syntax_Utils {
    let compoundSubStmts((CBlockDecl(_))) = |()| {
        vec![]
    };

    let compoundSubStmts((CBlockStmt(s))) = |()| {
        vec![s]
    };

    let compoundSubStmts((CNestedFunDef(_))) = |()| {
        vec![]
    };

    let getLabels((CCompound(ls, body, _))) = |()| {
        \\(concatMap(((concatMap(getLabels) . compoundSubStmts)), body), ls)
    };

    let getLabels((CLabel(l, s, _, _))) = |()| {
        :(l, getLabels(s))
    };

    let getLabels(stmt) = |()| {
        concatMap(getLabels, (getSubStmts(stmt)))
    };

    let getSubStmts((CAsm(_, _))) = |()| {
        vec![]
    };

    let getSubStmts((CBreak(_))) = |()| {
        vec![]
    };

    let getSubStmts((CCase(_, s, _))) = |()| {
        vec![s]
    };

    let getSubStmts((CCases(_, _, s, _))) = |()| {
        vec![s]
    };

    let getSubStmts((CCompound(_, body, _))) = |()| {
        concatMap(compoundSubStmts, body)
    };

    let getSubStmts((CCont(_))) = |()| {
        vec![]
    };

    let getSubStmts((CDefault(s, _))) = |()| {
        vec![s]
    };

    let getSubStmts((CExpr(_, _))) = |()| {
        vec![]
    };

    let getSubStmts((CFor(_, _, _, s, _))) = |()| {
        vec![s]
    };

    let getSubStmts((CGoto(_, _))) = |()| {
        vec![]
    };

    let getSubStmts((CGotoPtr(_, _))) = |()| {
        vec![]
    };

    let getSubStmts((CIf(_, sthen, selse, _))) = |()| {
        maybe(vec![sthen], (Lambda), selse)
    };

    let getSubStmts((CLabel(_, s, _, _))) = |()| {
        vec![s]
    };

    let getSubStmts((CReturn(_, _))) = |()| {
        vec![]
    };

    let getSubStmts((CSwitch(_, s, _))) = |()| {
        vec![s]
    };

    let getSubStmts((CWhile(_, s, _, _))) = |()| {
        vec![s]
    };

    let mapBlockItemStmts(_, _, bi) = |()| {
        bi
    };

    let mapBlockItemStmts(stop, f, (CBlockStmt(s))) = |()| {
        CBlockStmt((mapSubStmts(stop, f, s)))
    };

    let mapSubStmts(_, f, s) = |()| {
        f(s)
    };

    let mapSubStmts(stop, f, (CCase(e, s, ni))) = |()| {
        f((CCase(e, (mapSubStmts(stop, f, s)), ni)))
    };

    let mapSubStmts(stop, f, (CCases(e1, e2, s, ni))) = |()| {
        f((CCases(e1, e2, (mapSubStmts(stop, f, s)), ni)))
    };

    let mapSubStmts(stop, f, (CCompound(ls, body, ni))) = |()| {
        f((CCompound(ls, (map((mapBlockItemStmts(stop, f)), body)), ni)))
    };

    let mapSubStmts(stop, f, (CDefault(s, ni))) = |()| {
        f((CDefault((mapSubStmts(stop, f, s)), ni)))
    };

    let mapSubStmts(stop, f, (CFor(i, t, a, s, ni))) = |()| {
        f((CFor(i, t, a, (mapSubStmts(stop, f, s)), ni)))
    };

    let mapSubStmts(stop, f, (CIf(e, sthen, selse, ni))) = |()| {
        f((CIf(e, (mapSubStmts(stop, f, sthen)), (maybe(Nothing, ((Just . mapSubStmts(stop, f))), selse)), ni)))
    };

    let mapSubStmts(stop, f, (CLabel(i, s, attrs, ni))) = |()| {
        f((CLabel(i, (mapSubStmts(stop, f, s)), attrs, ni)))
    };

    let mapSubStmts(stop, f, (CSwitch(e, s, ni))) = |()| {
        f((CSwitch(e, (mapSubStmts(stop, f, s)), ni)))
    };

    let mapSubStmts(stop, f, (CWhile(e, s, isdo, ni))) = |()| {
        f((CWhile(e, (mapSubStmts(stop, f, s)), isdo, ni)))
    };

}

mod Language_C_Syntax {

}

mod Language_C_System_GCC {
    let buildCppArgs((CppArgs(options, extra_args, _tmpdir, input_file, output_file_opt))) = |()| {
        ++({

                (concatMap(tOption, options))
            }, ++(outputFileOpt, ++(vec!["-E".to_string(), input_file], extra_args)))
    };

    let gccParseCPPArgs(args) = |()| {
        match mungeArgs(((Nothing, Nothing, RList.empty), (RList.empty, RList.empty)), args) {
                Left, err => { Left(err) },
                Right, ((Nothing, _, _), _) => { Left("No .c / .hc / .h source file given".to_string()) },
                Right, ((Just(input_file), output_file_opt, cpp_opts), (extra_args, other_args)) => { Right(((rawCppArgs((RList.reverse(extra_args)), input_file))({
                        outputFile: output_file_opt,
                        cppOptions: RList.reverse(cpp_opts)
                        }), RList.reverse(other_args))) },
            }
    };

    fn newGCC() -> GCC {
        GCC
    }

}

mod Language_C_System_Preprocess {
    struct CppOption(IncludeDir, FilePath, Define, String, String, Undefine, String, IncludeFile, FilePath);

    struct CppArgs(CppArgs, { /* struct def */ });

    let addCppOption(cpp_args, opt) = |()| {
        cpp_args({
            cppOptions: :(opt, (cppOptions(cpp_args)))
            })
    };

    let addExtraOption(cpp_args, extra) = |()| {
        cpp_args({
            extraOptions: :(extra, (extraOptions(cpp_args)))
            })
    };

    let cppFile(input_file) = |()| {
        CppArgs({
            cppOptions: vec![],
            extraOptions: vec![],
            cppTmpDir: Nothing,
            inputFile: input_file,
            outputFile: Nothing
            })
    };

    fn isPreprocessed() -> Bool {
        (".i".to_string()(Operator("isSuffixOf")))
    }

    let mkOutputFile(tmp_dir_opt, input_file) = |()| {
        {

            let tmpDir = getTempDir(tmp_dir_opt);
            mkTmpFile(tmpDir, (getOutputFileName(input_file)))
        }
    };

    let mkTmpFile(tmp_dir, file_templ) = |()| {
        {

            let (path, file_handle) = openTempFile(tmp_dir, file_templ);
            hClose(file_handle);
            return(path)
        }
    };

    fn preprocessedExt() -> String {
        ".i".to_string()
    }

    let rawCppArgs(opts, input_file) = |()| {
        CppArgs({
            inputFile: input_file,
            cppOptions: vec![],
            extraOptions: opts,
            outputFile: Nothing,
            cppTmpDir: Nothing
            })
    };

    let runPreprocessor(cpp, cpp_args) = |()| {
        {

            bracket(getActualOutFile, removeTmpOutFile, invokeCpp);
            
        }
    };

}



fn main() { /* demo */ }
