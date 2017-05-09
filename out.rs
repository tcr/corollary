mod Language_C_Analysis_AstAnalysis {
    fn advanceDesigList(ds: Vec<CDesignator>, d: CDesignator) -> Vec<CDesignator> {
        drop(1)(dropWhile(((not . matchDesignator(d))), ds))
    }

    fn analyseAST($$$: CTranslUnit) -> m<GlobalDecls> {
        {
            let mapRecoverM_ = |f| {
                mapM_(((handleTravError . f)))
            };

            mapRecoverM_(analyseExt, decls);
            >>=(getDefTable, Lambda((not((inFileScope(dt)))))(error("Internal Error: Not in filescope after analysis".to_string())));
            liftM(globalDefs, getDefTable)
        }
    }

    fn analyseExt(__0: CExtDecl) -> m<EmptyParen> {
        match (__0) {
            $$$ => handleAsmBlock(asm),
            $$$ => analyseFunDef(fundef),
            $$$ => analyseDecl(False, decl),
        }
    }

    fn analyseFunDef($$$: CFunDef) -> m<EmptyParen> {
        {
            let improveFunDefType = |__0| {
                match (__0) {
                    $$$ => return(FunctionType((FunType(return_ty, vec![], False)), attrs)),
                    ty => return(ty),
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

    fn analyseFunctionBody(__0: NodeInfo, __1: VarDecl, __2: CStat, __3: m<Stmt>) -> m<Stmt> {
        match (__0, __1, __2, __3, __4) {
            node_info decl s $$$ $$$ => {
                enterFunctionScope;
                mapM_(((withDefTable . defineLabel)), (++(localLabels, getLabels(s))));
                defineParams(node_info, decl);
                mapM_((tBlockItem(vec![FunCtx(decl)])), items);
                leaveFunctionScope;
                return(s)
            },
            _ _ s => astError((nodeInfo(s)), "Function body is no compound statement".to_string()),
        }
    }

    fn analyseTypeDef(handle_sue_def: Bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, node_info: NodeInfo) -> m<EmptyParen> {
        {
            let checkValidTypeDef = |__0, __1, __2| {
                match (__0, __1, __2) {
                    True _ _ => astError(node_info, "inline specifier for typeDef".to_string()),
                    _ NoStorageSpec _ => return(()),
                    _ bad_storage _ => astError(node_info)(++("storage specified for typeDef: ".to_string(), show(bad_storage))),
                }
            };

            let (VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node)) = analyseVarDecl'(handle_sue_def, declspecs, declr, vec![], Nothing);
            checkValidTypeDef(is_inline, storage_spec, attrs);
            when((isNoName(name)))(astError(node_info, "NoName in analyseTypeDef".to_string()));
            Let;
            handleTypeDef((TypeDef(ident, ty, attrs, node_info)))
        }
    }

    fn builtinType(__0: CBuiltin) -> m<Type> {
        match (__0) {
            $$$ => analyseTypeDecl(d),
            $$$ => return(size_tType),
            $$$ => return(boolType),
        }
    }

    fn checkGuard(c: Vec<StmtCtx>, e: CExpr) -> m<EmptyParen> {
        >>=(tExpr(c, RValue, e), checkScalar'((nodeInfo(e))))
    }

    fn checkInits(__0: Type, __1: Vec<CDesignator>, __2: CInitList) -> m<EmptyParen> {
        match (__0, __1, __2) {
            _ _ $$$ => return(()),
            t dds $$$ => {
                let (dds', ds') = match (dds, ds) {
                                $$$ => typeError((nodeInfo(i)), "excess elements in initializer".to_string()),
                                $$$ => return((rest, vec![dd'])),
                                $$$ => return((advanceDesigList(dds, d), ds)),
                            };
                let t' = tDesignator(t, ds');
                tInit(t', i);
                checkInits(t, dds', is)
            },
        }
    }

    fn complexBaseType(ni: NodeInfo, c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m<Type> {
        {
            let t = tExpr(c, side, e);
            match canonicalType(t) {
                        DirectType $$$ quals attrs => return(DirectType((TyFloating(ft)), quals, attrs)),
                        _ => typeError(ni)(++("expected complex type, got: ".to_string(), pType(t))),
                    }
        }
    }

    fn computeFunDefStorage(__0: Ident, __1: StorageSpec) -> m<Storage> {
        match (__0, __1) {
            _ $$$ => return(FunLinkage(InternalLinkage)),
            ident other_spec => {
                let obj_opt = lookupObject(ident);
                Let;
                match other_spec {
                            NoStorageSpec => return(maybe(defaultSpec, declStorage, obj_opt)),
                            $$$ => return(maybe(defaultSpec, declStorage, obj_opt)),
                            bad_spec => throwTravError(badSpecifierError((nodeInfo(ident)))(++("unexpected function storage specifier (only static or extern is allowed)".to_string(), show(bad_spec)))),
                        }
            },
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

    fn defineParams(ni: NodeInfo, decl: VarDecl) -> m<EmptyParen> {
        match (getParams(declType(decl))) {
                Nothing => astError(ni, "expecting complete function type in function definition".to_string()),
                Just params => mapM_(handleParamDecl, params),
            }
    }

    fn enclosingFunctionType(__0: Vec<StmtCtx>) -> Maybe<Type> {
        match (__0) {
            $$$ => Nothing,
            $$$ => Just(declType(vd)),
            $$$ => enclosingFunctionType(cs),
        }
    }

    fn extFunProto($$$: VarDeclInfo) -> m<EmptyParen> {
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

    fn extVarDecl($$$: VarDeclInfo, init_opt: Maybe<Initializer>) -> m<EmptyParen> {
        {
            when((isNoName(var_name)))(astError(node_info, "NoName in extVarDecl".to_string()));
            let (storage, is_def) = globalStorage(storage_spec);
            Let;
            if(is_def, then, handleObjectDef, False, ident)(ObjDef(vardecl, init_opt, node_info, else, handleVarDecl, False)(Decl(vardecl, node_info)))
        }
    }

    fn getParams(__0: Type) -> Maybe<Vec<ParamDecl>> {
        match (__0) {
            $$$ => Just(params),
            _ => Nothing,
        }
    }

    fn hasTypeDef(declspecs: Vec<CDeclSpec>) -> Maybe<Vec<CDeclSpec>> {
        match foldr(hasTypeDefSpec, (False, vec![]), declspecs) {
                $$$ => Just(specs'),
                $$$ => Nothing,
            }
    }

    fn inLoop(c: Vec<StmtCtx>) -> Bool {
        any(isLoop, c)
    }

    fn inSwitch(c: Vec<StmtCtx>) -> Bool {
        any(isSwitch, c)
    }

    fn localVarDecl($$$: VarDeclInfo, init_opt: Maybe<Initializer>) -> m<EmptyParen> {
        {
            when((isNoName(var_name)))(astError(node_info, "NoName in localVarDecl".to_string()));
            let (storage, is_def) = localStorage(storage_spec);
            Let;
            if(is_def, then, handleObjectDef, True, ident, (ObjDef(vardecl, init_opt, node_info)), else, handleVarDecl, True, (Decl(vardecl, node_info)))
        }
    }

    fn matchDesignator(__0: CDesignator, __1: CDesignator) -> Bool {
        match (__0, __1) {
            $$$ $$$ => ==(m1, m2),
            _ _ => True,
        }
    }

    fn tBlockItem(__0: Vec<StmtCtx>, __1: CBlockItem) -> m<Type> {
        match (__0, __1) {
            c $$$ => tStmt(c, s),
            _ $$$ => >(analyseDecl(True, d), >((), return(voidType))),
            _ $$$ => >(analyseFunDef(fd), >((), return(voidType))),
        }
    }

    fn tDesignator(__0: Type, __1: Vec<CDesignator>) -> m<Type> {
        match (__0, __1) {
            $$$ $$$ => {
                >>=(tExpr(vec![], RValue, e), checkIntegral'(ni));
                tDesignator(bt, ds)
            },
            $$$ $$$ => {
                >>=(tExpr(vec![], RValue, e1), checkIntegral'(ni));
                >>=(tExpr(vec![], RValue, e2), checkIntegral'(ni));
                tDesignator(bt, ds)
            },
            $$$ $$$ => typeError((nodeInfo(d)), "member designator in array initializer".to_string()),
            t $$$ $$$ $$$ => {
                let mt = fieldType(ni, m, t);
                tDesignator((canonicalType(mt)), ds)
            },
            t $$$ $$$ $$$ => typeError((nodeInfo(d)), "array designator in compound initializer".to_string()),
            t $$$ => return(t),
        }
    }

    fn tExpr(c: Vec<StmtCtx>, side: ExprSide, e: CExpr) -> m<Type> {
        match nameOfNode((nodeInfo(e))) {
                Just n => {
                    let dt = getDefTable;
                    match lookupType(dt, n) {
                                Just t => return(t),
                                Nothing => {
                                    let t = tExpr'(c, side, e);
                                    withDefTable((Lambda))
                                },
                            }
                },
                Nothing => tExpr'(c, side, e),
            }
    }

    fn tExpr'(__0: Vec<StmtCtx>, __1: ExprSide, __2: CExpr) -> m<Type> {
        match (__0, __1, __2) {
            c side $$$ => {
                when((==(side, LValue)))(typeError(ni, "binary operator as lvalue".to_string()));
                let lt = tExpr(c, RValue, le);
                let rt = tExpr(c, RValue, re);
                binopType'(ni, op, lt, rt)
            },
            c side $$$ => {
                when((==(side, LValue)))(typeError(ni, "address-of operator as lvalue".to_string()));
                match e {
                            CCompoundLit _ _ _ => liftM(simplePtr, tExpr(c, RValue, e)),
                            CVar i _ => >>=(lookupObject(i), (typeErrorOnLeft(ni) . maybe((notFound(i)), varAddrType))),
                            _ => liftM(simplePtr, tExpr(c, LValue, e)),
                        }
            },
            c _ $$$ => >>=(tExpr(c, RValue, e), ((typeErrorOnLeft(ni) . derefType))),
            c _ $$$ => {
                let t = tExpr(c, RValue, e);
                checkIntegral'(ni, t);
                return(t)
            },
            c side $$$ => {
                when((==(side, LValue)))(typeError(ni, "logical negation used as lvalue".to_string()));
                >>=(tExpr(c, RValue, e), checkScalar'(ni));
                return(boolType)
            },
            c side $$$ => tExpr(c, (if(isEffectfulOp, op, then, LValue, else, side)), e),
            c _ $$$ => {
                let bt = tExpr(c, RValue, b);
                let it = tExpr(c, RValue, i);
                let addrTy = binopType'(ni, CAddOp, bt, it);
                typeErrorOnLeft(ni)(derefType(addrTy))
            },
            c side $$$ => {
                let t1 = tExpr(c, RValue, e1);
                checkScalar'((nodeInfo(e1)), t1);
                let t3 = tExpr(c, side, e3);
                match me2 {
                            Just e2 => {
                                let t2 = tExpr(c, side, e2);
                                conditionalType'(ni, t2, t3)
                            },
                            Nothing => conditionalType'(ni, t1, t3),
                        }
            },
            c side $$$ => {
                let t = tExpr(c, RValue, e);
                let bt = if(deref, then, typeErrorOnLeft, ni, (derefType(t)), else, return, t);
                fieldType(ni, m, bt)
            },
            c side $$$ => >>=(mapM((tExpr(c, side)), es), (return . last)),
            c side $$$ => {
                let dt = analyseTypeDecl(d);
                let et = tExpr(c, side, e);
                typeErrorOnLeft(ni)(castCompatible(dt, et));
                return(dt)
            },
            c side $$$ => {
                when((==(side, LValue)))(typeError(ni, "sizeof as lvalue".to_string()));
                tExpr(c, RValue, e);
                return(size_tType)
            },
            c side $$$ => {
                when((==(side, LValue)))(typeError(ni, "alignof as lvalue".to_string()));
                tExpr(c, RValue, e);
                return(size_tType)
            },
            c side $$$ => complexBaseType(ni, c, side, e),
            c side $$$ => complexBaseType(ni, c, side, e),
            _ side $$$ => {
                when((==(side, LValue)))(typeError(ni, "label address as lvalue".to_string()));
                return(PtrType(voidType, noTypeQuals, vec![]))
            },
            _ side $$$ => {
                when((==(side, LValue)))(typeError(ni, "compound literal as lvalue".to_string()));
                let lt = analyseTypeDecl(d);
                tInitList(ni, (canonicalType(lt)), initList);
                return(lt)
            },
            _ RValue $$$ => return(size_tType),
            _ RValue $$$ => return(size_tType),
            _ LValue $$$ => typeError(ni, "alignoftype as lvalue".to_string()),
            _ LValue $$$ => typeError(ni, "sizeoftype as lvalue".to_string()),
            _ side $$$ => >>=(lookupObject(i), maybe((typeErrorOnLeft(ni)(notFound(i))), ((return . declType)))),
            _ _ $$$ => constType(c),
            _ _ $$$ => builtinType(b),
            c _ $$$ => {
                Let;
                let t = match fe {
                                CVar i _ => >>=(lookupObject(i), maybe((fallback(i)), (const(tExpr(c, RValue, fe))))),
                                _ => tExpr(c, RValue, fe),
                            };
                let atys = mapM((tExpr(c, RValue)), args);
                match canonicalType(t) {
                            PtrType $$$ _ _ => {
                                Let;
                                mapM_(checkArg)(zip3(ptys, atys, args));
                                unless(varargs)(when((/=(length(atys), length(ptys))))(typeError(ni, "incorrect number of arguments".to_string())));
                                return(canonicalType(rt))
                            },
                            PtrType $$$ _ _ => {
                                return(canonicalType(rt))
                            },
                            _ => typeError(ni)(++("attempt to call non-function of type ".to_string(), pType(t))),
                        }
            },
            c _ $$$ => {
                let lt = tExpr(c, LValue, le);
                let rt = tExpr(c, RValue, re);
                when((constant(typeQuals(lt))))(typeError(ni)(++("assignment to lvalue with `constant\' qualifier: ".to_string(), ((render . pretty))(le))));
                match (canonicalType(lt), re) {
                        $$$ => if &&(isPointerType(lt'), ==(getCInteger(i), 0)) { return(()) },
                            $$$ => assignCompatible'(ni, op, lt, rt),
                        };
                return(lt)
            },
            c _ $$$ => {
                enterBlockScope;
                mapM_(((withDefTable . defineLabel)), (getLabels(s)));
                let t = tStmt(c, s);
                leaveBlockScope;
                return(t)
            },
        }
    }

    fn tInit(__0: Type, __1: CInit, __2: m<Initializer>) -> m<Initializer> {
        match (__0, __1, __2, __3) {
            t i $$$ $$$ => {
                let it = tExpr(vec![], RValue, e);
                assignCompatible'(ni, CAssignOp, t, it);
                return(i)
            },
            t i $$$ $$$ => >(tInitList(ni, (canonicalType(t)), initList), >((), return(i))),
        }
    }

    fn tInitList(__0: NodeInfo, __1: Type, __2: CInitList, __3: m<EmptyParen>) -> m<EmptyParen> {
        match (__0, __1, __2, __3, __4) {
            ni t $$$ $$$ $$$ => >(tExpr(vec![], RValue, e), >((), return(()))),
            ni t $$$ $$$ initList => {
                Let;
                checkInits(t, default_ds, initList)
            },
            ni t $$$ $$$ initList => {
                let td = lookupSUE(ni, (sueRef(ctr)));
                let ms = tagMembers(ni, td);
                Let;
                checkInits(t, default_ds, initList)
            },
            ni $$$ _ => return(()),
            _ t $$$ => >(tInit(t, i), >((), return(()))),
            ni t _ => typeError(ni)(++("initializer list for type: ".to_string(), pType(t))),
        }
    }

    fn tStmt(__0: Vec<StmtCtx>, __1: CStat) -> m<Type> {
        match (__0, __1) {
            c $$$ => tStmt(c, s),
            c $$$ => maybe((return(voidType)), (tExpr(c, RValue)), e),
            c $$$ => {
                enterBlockScope;
                mapM_(((withDefTable . defineLabel)), ls);
                let t = foldM((const(tBlockItem(c))), voidType, body);
                leaveBlockScope;
                return(t)
            },
            c $$$ => >(checkGuard(c, e), >((), >(tStmt(c, sthen), >((), >(maybe((return(())), (>(Lambda(c, s), >((), return(())))), selse), >((), return(voidType))))))),
            c $$$ => >>=(tExpr(c, RValue, e), >(checkIntegral'(ni), >((), tStmt((:(SwitchCtx, c)), s)))),
            c $$$ => >(checkGuard(c, e), >((), tStmt((:(LoopCtx, c)), s))),
            _ $$$ => {
                let dt = getDefTable;
                match lookupLabel(l, dt) {
                            Just _ => return(voidType),
                            Nothing => typeError(ni)(++("undefined label in goto: ".to_string(), identToString(l))),
                        }
            },
            c $$$ => {
                unless((inLoop(c)))(astError(ni, "continue statement outside of loop".to_string()));
                return(voidType)
            },
            c $$$ => {
                unless((||(inLoop(c), inSwitch(c))))(astError(ni, "break statement outside of loop or switch statement".to_string()));
                return(voidType)
            },
            c $$$ => {
                let t = tExpr(c, RValue, e);
                let rt = match enclosingFunctionType(c) {
                                Just $$$ => return(rt),
                                Just $$$ => return(rt),
                                Just ft => astError(ni)(++("bad function type: ".to_string(), pType(ft))),
                                Nothing => astError(ni, "return statement outside function".to_string()),
                            };
                match (rt, t) {
                            $$$ => return(()),
                            _ => assignCompatible'(ni, CAssignOp, rt, t),
                        };
                return(voidType)
            },
            _ $$$ => return(voidType),
            _ $$$ => return(voidType),
            c $$$ => {
                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                >>=(tExpr(c, RValue, e), checkIntegral'(ni));
                tStmt(c, s)
            },
            c $$$ => {
                unless((inSwitch(c)))(astError(ni, "case statement outside of switch statement".to_string()));
                >>=(tExpr(c, RValue, e1), checkIntegral'(ni));
                >>=(tExpr(c, RValue, e2), checkIntegral'(ni));
                tStmt(c, s)
            },
            c $$$ => {
                unless((inSwitch(c)))(astError(ni, "default statement outside of switch statement".to_string()));
                tStmt(c, s)
            },
            c $$$ => {
                enterBlockScope;
                either((maybe((return(())), checkExpr)), (analyseDecl(True)), i);
                maybe((return(())), (checkGuard(c)), g);
                maybe((return(())), checkExpr, inc);
                tStmt((:(LoopCtx, c)), s);
                leaveBlockScope;
                return(voidType)
            },
            c $$$ => {
                let t = tExpr(c, RValue, e);
                match t {
                            $$$ => return(voidType),
                            _ => typeError(ni, "can\'t goto non-pointer".to_string()),
                        }
            },
        }
    }

}

mod Language_C_Analysis_AstAnalysis {

}

mod Language_C_Analysis_Builtins {
    fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}

// ERROR: can't output "./language-c/src/Language/C/Analysis/ConstEval.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/Debug.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/DeclAnalysis.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/DefTable.hs"

mod Language_C_Analysis_Export {
    fn exportArraySize(__0: ArraySize) -> CArrSize {
        match (__0) {
            $$$ => CArrSize(static, e),
            $$$ => CNoArrSize(complete),
        }
    }

    fn exportAttrs() -> Vec<CAttr> {
        map(exportAttr)
    }

    fn exportCompType($$$: CompType) -> Vec<CTypeSpec> {
        vec![CSUType(comp, ni)]
    }

    fn exportCompTypeDecl(ty: CompTypeRef) -> Vec<CTypeSpec> {
        vec![CSUType((exportComp(ty)), ni)]
    }

    fn exportCompTypeRef($$$: CompType) -> Vec<CTypeSpec> {
        exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
    }

    fn exportComplexType(ty: FloatType) -> Vec<CTypeSpec> {
        :((CComplexType(ni)), exportFloatType(ty))
    }

    fn exportDeclAttrs($$$: DeclAttrs) -> Vec<CDeclSpec> {
        ++((if(inline, then, vec![CTypeQual((CInlineQual(ni)))], else, vec![])), ++(map((CStorageSpec), (exportStorage(storage))), map(((CTypeQual . CAttrQual)), (exportAttrs(attrs)))))
    }

    fn exportDeclr(other_specs: Vec<CDeclSpec>, ty: Type, attrs: Attributes, name: VarName) -> (Vec<CDeclSpec>, CDeclr) {
        (++(other_specs, specs), CDeclr(ident, derived, asmname, (exportAttrs(attrs)), ni))
    }

    fn exportEnumType($$$: EnumType) -> Vec<CTypeSpec> {
        vec![CEnumType(enum, ni)]
    }

    fn exportEnumTypeDecl(ty: EnumTypeRef) -> Vec<CTypeSpec> {
        vec![CEnumType((exportEnum(ty)), ni)]
    }

    fn exportEnumTypeRef($$$: EnumType) -> Vec<CTypeSpec> {
        exportEnumTypeDecl((EnumTypeRef(sue_ref, node_info)))
    }

    fn exportFloatType(ty: FloatType) -> Vec<CTypeSpec> {
        match ty {
                TyFloat => vec![CFloatType(ni)],
                TyDouble => vec![CDoubleType(ni)],
                TyLDouble => vec![CLongType(ni), CDoubleType(ni)],
            }
    }

    fn exportIntType(ty: IntType) -> Vec<CTypeSpec> {
        match ty {
                TyBool => vec![CBoolType(ni)],
                TyChar => vec![CCharType(ni)],
                TySChar => vec![CSignedType(ni), CCharType(ni)],
                TyUChar => vec![CUnsigType(ni), CCharType(ni)],
                TyShort => vec![CShortType(ni)],
                TyUShort => vec![CUnsigType(ni), CShortType(ni)],
                TyInt => vec![CIntType(ni)],
                TyUInt => vec![CUnsigType(ni), CIntType(ni)],
                TyLong => vec![CLongType(ni)],
                TyULong => vec![CUnsigType(ni), CLongType(ni)],
                TyLLong => vec![CLongType(ni), CLongType(ni)],
                TyULLong => vec![CUnsigType(ni), CLongType(ni), CLongType(ni)],
            }
    }

    fn exportMemberDecl(__0: MemberDecl) -> CDecl {
        match (__0) {
            $$$ => CDecl((map(CTypeSpec)(exportTypeSpec(fromDirectType(ty)))), vec![(Nothing, Nothing, Just(expr))], node_info),
            $$$ => Let(in, CDecl, specs, vec![(Just(declarator), Nothing, bitfieldsz)], node_info),
        }
    }

    fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
        Let(in, CDecl, specs, vec![(Just(declr), Nothing, Nothing)], (nodeInfo(paramdecl)))
    }

    fn exportSUERef() -> Maybe<Ident> {
        (Just . (internalIdent . show))
    }

    fn exportStorage(__0: Storage) -> Vec<CStorageSpec> {
        match (__0) {
            NoStorage => vec![],
            $$$ => if(reg, then, vec![CRegister(ni)], else, vec![]),
            $$$ => threadLocal(thread_local, vec![CStatic(ni)]),
            $$$ => threadLocal(thread_local, vec![CExtern(ni)]),
            $$$ => error("impossible storage: static without linkage".to_string()),
            $$$ => vec![CStatic(ni)],
            $$$ => vec![],
            $$$ => error("impossible storage: function without linkage".to_string()),
        }
    }

    fn exportType(ty: Type) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {
        exportTy(vec![], ty)
    }

    fn exportTypeDecl(ty: Type) -> CDecl {
        CDecl(declspecs, declrs, ni)
    }

    fn exportTypeDef($$$: TypeDef) -> CDecl {
        CDecl((:(CStorageSpec((CTypedef(ni))), declspecs)), vec![declr], node_info)
    }

    fn exportTypeQuals(quals: TypeQuals) -> Vec<CTypeQual> {
        mapMaybe(select, vec![(constant, CConstQual(ni)), (volatile, CVolatQual(ni)), (restrict, CRestrQual(ni))])
    }

    fn exportTypeQualsAttrs(tyqs: TypeQuals, attrs: Attributes) -> Vec<CTypeQual> {
        (++(exportTypeQuals(tyqs), map(CAttrQual, (exportAttrs(attrs)))))
    }

    fn exportTypeSpec(tyname: TypeName) -> Vec<CTypeSpec> {
        match tyname {
                TyVoid => vec![CVoidType(ni)],
                TyIntegral ity => exportIntType(ity),
                TyFloating fty => exportFloatType(fty),
                TyComplex fty => exportComplexType(fty),
                TyComp comp => exportCompTypeDecl(comp),
                TyEnum enum => exportEnumTypeDecl(enum),
                TyBuiltin TyVaList => vec![CTypeDef((internalIdent("va_list".to_string())), ni)],
                TyBuiltin TyAny => vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)],
            }
    }

    fn exportVarDecl($$$: VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
        exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
    }

    fn fromDirectType(__0: Type) -> TypeName {
        match (__0) {
            $$$ => ty,
            $$$ => maybe((error("undefined typeDef".to_string())), fromDirectType, ref),
            _ => error("fromDirectType".to_string()),
        }
    }

    fn ni() -> NodeInfo {
        undefNode
    }

    fn threadLocal(__0: Bool) -> Vec<CStorageSpec> {
        match (__0) {
            False => id,
            True => ((CThread(ni))(Operator(":"))),
        }
    }

}

mod Language_C_Analysis_NameSpaceMap {
    fn defGlobal($$$: NameSpaceMap<k, a>, ident: k, def: a) -> (NameSpaceMap<k, a>, Maybe<a>) {
        (NsMap((Map.insert(ident, def, gs)), lss), Map.lookup(ident, gs))
    }

    fn defLocal(__0: NameSpaceMap<k, a>, __1: k, __2: a, __3: (NameSpaceMap<k, a>, Maybe<a>)) -> (NameSpaceMap<k, a>, Maybe<a>) {
        match (__0, __1, __2, __3, __4) {
            ns $$$ $$$ ident def => defGlobal(ns, ident, def),
            $$$ ident def => (NsMap(gs, (:((:((ident, def), ls)), lss))), Prelude.lookup(ident, ls)),
        }
    }

    fn enterNewScope($$$: NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
        NsMap(gs, (:(vec![], lss)))
    }

    fn globalNames($$$: NameSpaceMap<k, v>) -> Map<k, v> {
        g
    }

    fn hasLocalNames($$$: NameSpaceMap<k, v>) -> Bool {
        not((null(l)))
    }

    fn leaveScope(__0: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
        match (__0) {
            $$$ => error("NsMaps.leaveScope: No local scope!".to_string()),
            $$$ => (NsMap(gs, lss), ls),
        }
    }

    fn localNames($$$: NameSpaceMap<k, v>) -> Vec<Vec<(k, v)>> {
        l
    }

    fn lookupGlobal($$$: NameSpaceMap<k, a>, ident: k) -> Maybe<a> {
        Map.lookup(ident, gs)
    }

    fn lookupInnermostScope(nsm: NameSpaceMap<k, a>, $$$: k, $$$: Maybe<a>) -> Maybe<a> {
        match localDefs {
                $$$ => Prelude.lookup(ident, ls),
                $$$ => lookupGlobal(nsm, ident),
            }
    }

    fn lookupName(ns: NameSpaceMap<k, a>, $$$: k, $$$: Maybe<a>) -> Maybe<a> {
        match (lookupLocal(localDefs)) {
                Nothing => lookupGlobal(ns, ident),
                Just def => Just(def),
            }
    }

    fn mergeNameSpace($$$: NameSpaceMap<k, a>, $$$: NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
        NsMap((Map.union(global1, global2)), (localUnion(local1, local2)))
    }

    fn nameSpaceMap() -> NameSpaceMap<k, v> {
        NsMap(Map.empty, vec![])
    }

    fn nsMapToList($$$: NameSpaceMap<k, a>) -> Vec<(k, a)> {
        ++(concat(lss), Map.toList(gs))
    }

}

mod Language_C_Analysis_SemError {
    fn badSpecifierError(node_info: NodeInfo, msg: String) -> BadSpecifierError {
        BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn invalidAST(node_info: NodeInfo, msg: String) -> InvalidASTError {
        InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
    }

    fn prevDeclMsg(old_node: NodeInfo) -> Vec<String> {
        vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
    }

    fn redefErrLabel($$$: RedefInfo) -> String {
        ++(ident, " redefined".to_string())
    }

    fn redefErrReason(__0: RedefInfo) -> String {
        match (__0) {
            $$$ => ++("duplicate definition of ".to_string(), ident),
            $$$ => ++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string())),
            $$$ => ++(ident, " previously declared as a different kind of symbol".to_string()),
            $$$ => ++(ident, " previously declared with different linkage".to_string()),
            $$$ => ++(ident, " previously declared without linkage".to_string()),
        }
    }

    fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, $$$: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (++(vec![redefErrReason(info)], prevDeclMsg(old_node))))
    }

    fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
        RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
    }

    fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    fn typeMismatchInfo($$$: TypeMismatch) -> ErrorInfo {
        ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
    }

}

// ERROR: can't output "./language-c/src/Language/C/Analysis/SemRep.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/TravMonad.hs"

// ERROR: can't output "./language-c/src/Language/C/Analysis/TypeCheck.hs"

mod Language_C_Analysis_TypeConversions {
    fn arithmeticConversion(__0: TypeName, __1: TypeName) -> Maybe<TypeName> {
        match (__0, __1) {
            $$$ $$$ => Just(TyComplex(floatConversion(t1, t2))),
            $$$ $$$ => Just(TyComplex(floatConversion(t1, t2))),
            $$$ $$$ => Just(TyComplex(floatConversion(t1, t2))),
            t1 $$$ $$$ $$$ => Just(t1),
            $$$ t2 $$$ $$$ => Just(t2),
            $$$ $$$ => Just(TyFloating(floatConversion(t1, t2))),
            t1 $$$ $$$ $$$ => Just(t1),
            $$$ t2 $$$ $$$ => Just(t2),
            $$$ $$$ => Just(TyIntegral(intConversion(t1, t2))),
            $$$ $$$ => Just(TyIntegral(TyInt)),
            $$$ t2 => Just(t2),
            t1 $$$ => Just(t1),
            _ _ => Nothing,
        }
    }

    fn floatConversion() -> FloatType {
        max
    }

    fn intConversion(t1: IntType, t2: IntType) -> IntType {
        max(TyInt, (max(t1, t2)))
    }

}

// ERROR: can't output "./language-c/src/Language/C/Analysis/TypeUtils.hs"

mod Language_C_Analysis {

}

// ERROR: can't output "./language-c/src/Language/C/Data/Error.hs"

mod Language_C_Data_Ident {
    fn bits14() -> Int {
        ^(2, (::(14, Int)))
    }

    fn bits21() -> Int {
        ^(2, (::(21, Int)))
    }

    fn bits28() -> Int {
        ^(2, (::(28, Int)))
    }

    fn bits7() -> Int {
        ^(2, (::(7, Int)))
    }

    fn builtinIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
    }

    fn dumpIdent(ide: Ident) -> String {
        ++(identToString(ide), ++(" at ".to_string(), show((nodeInfo(ide)))))
    }

    fn identToString($$$: Ident) -> String {
        s
    }

    fn internalIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
    }

    fn internalIdentAt(pos: Position, s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
    }

    fn isAnonymousRef(__0: SUERef) -> Bool {
        match (__0) {
            $$$ => True,
            _ => False,
        }
    }

    fn isInternalIdent($$$: Ident) -> Bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo'(pos, (pos, length(s)), name)))
    }

    fn quad(__0: String) -> Int {
        match (__0) {
            $$$ => +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28))),
            $$$ => *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1))))),
            $$$ => *(ord(c2), +(bits7, ord(c1))),
            $$$ => ord(c1),
            $$$ => 0,
        }
    }

}

mod Language_C_Data_InputStream {
    fn countLines() -> Int {
        match () {
             => (length . BSC.lines),
             => (length . lines),
        }
    }

    fn inputStreamEmpty() -> Bool {
        match () {
             => BSW.null,
             => null,
        }
    }

    fn inputStreamFromString() -> InputStream {
        match () {
             => BSC.pack,
             => id,
        }
    }

    fn inputStreamToString() -> String {
        match () {
             => BSC.unpack,
             => id,
        }
    }

    fn readInputStream() -> IO<InputStream> {
        match () {
             => BSW.readFile,
             => readFile,
        }
    }

    fn takeByte(bs: InputStream) -> (Word8, InputStream) {
        seq(BSW.head(bs), (BSW.head(bs), BSW.tail(bs)))
    }

    fn takeChar(__0: InputStream) -> (Char, InputStream) {
        match (__0) {
            bs => seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs))),
            bs => (head(bs), tail(bs)),
        }
    }

    fn takeChars(__0: Int, __1: InputStream) -> Vec<Char> {
        match (__0, __1) {
            $$$ bstr => BSC.unpack(BSC.take(n, bstr)),
            n str => take(n, str),
        }
    }

}

mod Language_C_Data_Name {
    fn namesStartingFrom(k: Int) -> Vec<Name> {
        vec![Name(k..)]
    }

    fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    }

}

// ERROR: can't output "./language-c/src/Language/C/Data/Node.hs"

mod Language_C_Data_Position {
    fn adjustPos(__0: FilePath, __1: Int, __2: Position) -> Position {
        match (__0, __1, __2) {
            fname row $$$ => Position(offs, fname, row, 1),
            _ _ p => p,
        }
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset(__0: Position, __1: Int) -> Position {
        match (__0, __1) {
            $$$ n => Position((+(o, n)), f, r, c),
            p n => p,
        }
    }

    fn incPos(__0: Position, __1: Int) -> Position {
        match (__0, __1) {
            $$$ n => Position((+(offs, n)), fname, row, (+(col, n))),
            p _ => p,
        }
    }

    fn initPos(file: FilePath) -> Position {
        Position(0, file, 1, 1)
    }

    fn internalPos() -> Position {
        InternalPosition
    }

    fn isBuiltinPos(__0: Position) -> Bool {
        match (__0) {
            BuiltinPosition => True,
            _ => False,
        }
    }

    fn isInternalPos(__0: Position) -> Bool {
        match (__0) {
            InternalPosition => True,
            _ => False,
        }
    }

    fn isNoPos(__0: Position) -> Bool {
        match (__0) {
            NoPosition => True,
            _ => False,
        }
    }

    fn isSourcePos(__0: Position) -> Bool {
        match (__0) {
            $$$ => True,
            _ => False,
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
            $$$ => Position((+(offs, 1)), fname, (+(row, 1)), 1),
            p => p,
        }
    }

}

// ERROR: can't output "./language-c/src/Language/C/Data/RList.hs"

mod Language_C_Data {

}

mod Language_C_Parser_Builtin {
    fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}

// ERROR: can't output "./language-c/src/Language/C/Parser/Lexer.x"

// ERROR: can't output "./language-c/src/Language/C/Parser/Parser.y"

// ERROR: can't output "./language-c/src/Language/C/Parser/ParserMonad.hs"

// ERROR: can't output "./language-c/src/Language/C/Parser/Tokens.hs"

mod Language_C_Parser {
    fn execParser_(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}

// ERROR: can't output "./language-c/src/Language/C/Pretty.hs"

// ERROR: can't output "./language-c/src/Language/C/Syntax/AST.hs"

mod Language_C_Syntax_Constants {
    fn _showWideFlag(flag: Bool) -> ShowS {
        if(flag, then, showString, "L".to_string(), else, id)
    }

    fn cChar(c: Char) -> CChar {
        CChar(c, False)
    }

    fn cChar_w(c: Char) -> CChar {
        CChar(c, True)
    }

    fn cChars() -> CChar {
        CChars
    }

    fn cFloat() -> CFloat {
        (CFloat . show)
    }

    fn cInteger(i: Integer) -> CInteger {
        CInteger(i, DecRepr, noFlags)
    }

    fn cString(str: String) -> CString {
        CString(str, False)
    }

    fn cString_w(str: String) -> CString {
        CString(str, True)
    }

    fn clearFlag(flag: f, $$$: Flags<f>) -> Flags<f> {
        Flags(clearBit(k, fromEnum(flag)))
    }

    fn concatCStrings(cs: Vec<CString>) -> CString {
        CString((concatMap(getCString, cs)), (any(isWideString, cs)))
    }

    fn dQuote(s: String, t: ShowS) -> ShowS {
        ++((:("\"\"".to_string(), s)), ++("\\\"".to_string(), t))
    }

    fn escapeCChar($$$: Char) -> String {
        "\\\\\'".to_string()
    }

    fn escapeChar(__0: Char) -> String {
        match (__0) {
            $$$ => "\\\\\\\\".to_string(),
            $$$ => "\\\\a".to_string(),
            $$$ => "\\\\b".to_string(),
            $$$ => "\\\\e".to_string(),
            $$$ => "\\\\f".to_string(),
            $$$ => "\\\\n".to_string(),
            $$$ => "\\\\r".to_string(),
            $$$ => "\\\\t".to_string(),
            $$$ => "\\\\v".to_string(),
        }
    }

    fn getCChar(__0: CChar) -> Vec<Char> {
        match (__0) {
            $$$ => vec![c],
            $$$ => cs,
        }
    }

    fn getCCharAsInt(__0: CChar) -> Integer {
        match (__0) {
            $$$ => fromIntegral((fromEnum(c))),
            $$$ => error("integer value of multi-character character constants is implementation defined".to_string()),
        }
    }

    fn getCInteger($$$: CInteger) -> Integer {
        i
    }

    fn getCString($$$: CString) -> String {
        str
    }

    fn head'(__0: String, __1: Vec<a>) -> a {
        match (__0, __1) {
            err $$$ => error(err),
            _ $$$ => x,
        }
    }

    fn isAsciiSourceChar(c: Char) -> Bool {
        &&(isAscii(c), isPrint(c))
    }

    fn isCChar(__0: Char) -> Bool {
        match (__0) {
            $$$ => False,
            $$$ => False,
            $$$ => False,
            c => isAsciiSourceChar(c),
        }
    }

    fn isSChar(__0: Char) -> Bool {
        match (__0) {
            $$$ => False,
            $$$ => False,
            $$$ => False,
            c => isAsciiSourceChar(c),
        }
    }

    fn isWideChar(__0: CChar) -> Bool {
        match (__0) {
            $$$ => wideFlag,
            $$$ => wideFlag,
        }
    }

    fn isWideString($$$: CString) -> Bool {
        wideflag
    }

    fn noFlags() -> Flags<f> {
        Flags(0)
    }

    fn readCFloat() -> CFloat {
        CFloat
    }

    fn readCInteger(repr: CIntRepr, str: String) -> Either<String, CInteger> {
        match readNum(str) {
                $$$ => mkCInt(n, suffix),
                parseFailed => Left(++("Bad Integer literal: ".to_string(), show(parseFailed))),
            }
    }

    fn sQuote(s: String, t: ShowS) -> ShowS {
        ++("\'".to_string(), ++(s, ++("\'".to_string(), t)))
    }

    fn setFlag(flag: f, $$$: Flags<f>) -> Flags<f> {
        Flags(setBit(k, fromEnum(flag)))
    }

    fn showCharConst(c: Char) -> ShowS {
        sQuote(escapeCChar(c))
    }

    fn showStringLit() -> ShowS {
        (dQuote . concatMap(showStringChar))
    }

    fn testFlag(flag: f, $$$: Flags<f>) -> Bool {
        testBit(k, fromEnum(flag))
    }

    fn unescapeChar(__0: String) -> (Char, String) {
        match (__0) {
            $$$ => match c {
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => ("\"\"".to_string(), cs),
                    $$$ => match head'("bad escape sequence".to_string(), (readHex(cs))) {
                            $$$ => (toEnum(i), cs'),
                        },
                    _ => match head'("bad escape sequence".to_string(), (readOct((:(c, cs))))) {
                            $$$ => (toEnum(i), cs'),
                        },
                },
            $$$ => (c, cs),
            $$$ => error("unescape char: empty string".to_string()),
        }
    }

    fn unescapeString(__0: String) -> String {
        match (__0) {
            $$$ => vec![],
            cs => match unescapeChar(cs) {
                    $$$ => :(c, unescapeString(cs')),
                },
        }
    }

}

mod Language_C_Syntax_Ops {
    fn assignBinop(__0: CAssignOp) -> CBinaryOp {
        match (__0) {
            CAssignOp => error("direct assignment has no binary operator".to_string()),
            CMulAssOp => CMulOp,
            CDivAssOp => CDivOp,
            CRmdAssOp => CRmdOp,
            CAddAssOp => CAddOp,
            CSubAssOp => CSubOp,
            CShlAssOp => CShlOp,
            CShrAssOp => CShrOp,
            CAndAssOp => CAndOp,
            CXorAssOp => CXorOp,
            COrAssOp => COrOp,
        }
    }

    fn isBitOp(op: CBinaryOp) -> Bool {
        elem(op, vec![CShlOp, CShrOp, CAndOp, COrOp, CXorOp])
    }

    fn isCmpOp(op: CBinaryOp) -> Bool {
        elem(op, vec![CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp])
    }

    fn isEffectfulOp(op: CUnaryOp) -> Bool {
        elem(op, vec![CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp])
    }

    fn isLogicOp(op: CBinaryOp) -> Bool {
        elem(op, vec![CLndOp, CLorOp])
    }

    fn isPtrOp(op: CBinaryOp) -> Bool {
        elem(op, vec![CAddOp, CSubOp])
    }

}

// ERROR: can't output "./language-c/src/Language/C/Syntax/Utils.hs"

mod Language_C_Syntax {

}

mod Language_C_System_GCC {
    fn buildCppArgs($$$: CppArgs) -> Vec<String> {
        ++({
                (concatMap(tOption, options))
            }, ++(outputFileOpt, ++(vec!["-E".to_string(), input_file], extra_args)))
    }

    fn gccParseCPPArgs(args: Vec<String>) -> Either<String, (CppArgs, Vec<String>)> {
        match mungeArgs(((Nothing, Nothing, RList.empty), (RList.empty, RList.empty)), args) {
                Left err => Left(err),
                Right $$$ => Left("No .c / .hc / .h source file given".to_string()),
                Right $$$ => Right(((rawCppArgs((RList.reverse(extra_args)), input_file))(hashmap! {
                            "outputFile" => output_file_opt,
                            "cppOptions" => RList.reverse(cpp_opts)
                        }), RList.reverse(other_args))),
            }
    }

    fn newGCC() -> GCC {
        GCC
    }

}

mod Language_C_System_Preprocess {
    fn addCppOption(cpp_args: CppArgs, opt: CppOption) -> CppArgs {
        cpp_args(hashmap! {
                "cppOptions" => :(opt, (cppOptions(cpp_args)))
            })
    }

    fn addExtraOption(cpp_args: CppArgs, extra: String) -> CppArgs {
        cpp_args(hashmap! {
                "extraOptions" => :(extra, (extraOptions(cpp_args)))
            })
    }

    fn cppFile(input_file: FilePath) -> CppArgs {
        CppArgs(hashmap! {
                "cppOptions" => vec![],
                "extraOptions" => vec![],
                "cppTmpDir" => Nothing,
                "inputFile" => input_file,
                "outputFile" => Nothing
            })
    }

    fn isPreprocessed() -> Bool {
        (".i".to_string()(Operator("isSuffixOf")))
    }

    fn mkOutputFile(tmp_dir_opt: Maybe<FilePath>, input_file: FilePath) -> IO<FilePath> {
        {
            let tmpDir = getTempDir(tmp_dir_opt);
            mkTmpFile(tmpDir, (getOutputFileName(input_file)))
        }
    }

    fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> IO<FilePath> {
        {
            let (path, file_handle) = openTempFile(tmp_dir, file_templ);
            hClose(file_handle);
            return(path)
        }
    }

    fn preprocessedExt() -> String {
        ".i".to_string()
    }

    fn rawCppArgs(opts: Vec<String>, input_file: FilePath) -> CppArgs {
        CppArgs(hashmap! {
                "inputFile" => input_file,
                "cppOptions" => vec![],
                "extraOptions" => opts,
                "outputFile" => Nothing,
                "cppTmpDir" => Nothing
            })
    }

    fn runPreprocessor(cpp: cpp, cpp_args: CppArgs) -> IO<Either<ExitCode, InputStream>> {
        {
            fn getActualOutFile() -> IO<FilePath> {
                maybe((mkOutputFile((cppTmpDir(cpp_args)), (inputFile(cpp_args)))), return, (outputFile(cpp_args)))
            }

            let invokeCpp = |actual_out_file| {
                {
                    let exit_code = runCPP(cpp, (cpp_args(hashmap! {
                                        "outputFile" => Just(actual_out_file)
                                    })));
                    match exit_code {
                                ExitSuccess => liftM(Right, (readInputStream(actual_out_file))),
                                ExitFailure _ => return(Left(exit_code)),
                            }
                }
            };

            let removeTmpOutFile = |out_file| {
                maybe((removeFile(out_file)), (Lambda(())), (outputFile(cpp_args)))
            };

            bracket(getActualOutFile, removeTmpOutFile, invokeCpp)
        }
    }

}

