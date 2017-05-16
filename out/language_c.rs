/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/AstAnalysis.hs"
Error: Unrecognized token `->`:
117 |     | null declrs =
118 |         case typedef_spec of{ Just _  -> astError node "YmFkIHR5cGVkZW
119 |                               Nothing -> analyseTypeDecl decl >> retur
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/Builtins.hs"
Error: Unrecognized token `=`:
 10 | ;builtins = foldr doIdent (foldr doTypeDef emptyDefTable typedefs) ide
 11 |   where{ doTypeDef d = snd . defineTypeDef (identOfTypeDef d) d
 12 |          doIdent   d = snd . defineGlobalIdent (declIdent d) d
~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/ConstEval.hs"
Error: Unrecognized token `->`:
 21 | ;data MachineDesc =
 22 |   MachineDesc
 23 |   { iSize        :: IntType -> Integer
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Analysis_Debug {
    fn globalDeclStats(file_filter: fn(FilePath) -> Bool, gmap: GlobalDecls) -> Vec<(String, isize)> {
        vec![("Enumeration Constants".to_string(), Map.size(enumerators)), ("Total Object/Function Declarations".to_string(), Map.size(all_decls)), ("Object definitions".to_string(), Map.size(objDefs)), ("Function Definitions".to_string(), Map.size(funDefs)), ("Tag definitions".to_string(), Map.size(tagDefs)), ("TypeDefs".to_string(), Map.size(typeDefs))]
    }

    fn joinComma() -> Doc {
        (hsep . (punctuate(comma) . map(pretty)))
    }

    fn prettyAssocs(label: String) -> Doc {
        prettyAssocsWith(label, pretty, pretty)
    }

    fn prettyAssocsWith(label: String, prettyKey: fn(k) -> Doc, prettyVal: fn(v) -> Doc, theMap: Vec<(k, v)>) -> Doc {
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
    #[derive(Eq, Ord, Show, Read)]
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

    fn analyseVarDecl(handle_sue_def: Bool, storage_specs: Vec<CStorageSpec>, decl_attrs: Vec<CAttr>, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, inline: Bool, CDeclr(name_opt, derived_declrs, asmname_opt, declr_attrs, node): CDeclr, oldstyle_params: Vec<CDecl>, init_opt: Maybe(CInit)) -> m(VarDeclInfo) {
        {

        }
    }

    fn analyseVarDecl'(handle_sue_def: Bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, oldstyle: Vec<CDecl>, init_opt: Maybe(CInit)) -> m(VarDeclInfo) {
        {

        }
    }

    fn canonicalStorageSpec(storagespecs: Vec<CStorageSpec>) -> m(StorageSpec) {
        liftM(elideAuto)(foldrM(updStorage, NoStorageSpec, storagespecs))
    }

    fn canonicalTypeSpec() -> m(TypeSpecAnalysis) {
        foldrM(go, TSNone)
    }

    fn computeParamStorage(__0: NodeInfo, __1: StorageSpec) -> Either(BadSpecifierError, Storage) {
        match (__0, __1) {
            _, NoStorageSpec => { Right((Auto(False))) },
            _, RegSpec => { Right((Auto(True))) },
            node, spec => { (Left . badSpecifierError(node)(++("Bad storage specified for parameter: ".to_string(), show(spec)))) },
        }
    }

    fn emptyDeclr(node: NodeInfo) -> CDeclr {
        CDeclr(Nothing, vec![], Nothing, vec![], node)
    }

    fn emptyNumTypeSpec() -> NumTypeSpec {
        NumTypeSpec(hashmap! {
            "base" => NoBaseType,
            "signSpec" => NoSignSpec,
            "sizeMod" => NoSizeMod,
            "isComplex" => False
            })
    }

    fn getOnlyDeclr(__0: CDecl) -> m(CDeclr) {
        match (__0) {
            CDecl(_, [(Just(declr), _, _)], _) => { return(declr) },
            CDecl(_, _, node) => { internalErr("getOnlyDeclr: declaration doesn\'t have a unique declarator".to_string()) },
        }
    }

    fn hasThreadLocalSpec(__0: StorageSpec) -> Bool {
        match (__0) {
            ThreadSpec => { True },
            StaticSpec(b) => { b },
            ExternSpec(b) => { b },
            _ => { False },
        }
    }

    fn isTypeDef(declspecs: Vec<CDeclSpec>) -> Bool {
        not(null(Dummy))
    }

    fn mergeOldStyle(__0: NodeInfo, __1: Vec<CDecl>, __2: Vec<CDerivedDeclr>) -> m(Vec<CDerivedDeclr>) {
        match (__0, __1, __2) {
            _node, [], declrs => { return(declrs) },
            node, oldstyle_params, CFunDeclr(params, attrs, fdnode, :, dds) => { match params {
                    Left, list => { {

                    } },
                    Right, _newstyle => { astError(node, "oldstyle parameter list, but newstyle function declaration".to_string()) },
                } },
            node, _, _ => { astError(node, "oldstyle parameter list, but not function type".to_string()) },
        }
    }

    fn mergeTypeAttributes(node_info: NodeInfo, quals: TypeQuals, attrs: Vec<Attr>, typ: Type) -> m(Type) {
        match typ {
                DirectType, ty_name, quals', attrs' => { merge(quals', attrs')(mkDirect(ty_name)) },
                PtrType, ty, quals', attrs' => { merge(quals', attrs')(PtrType(ty)) },
                ArrayType, ty, array_sz, quals', attrs' => { merge(quals', attrs')(ArrayType(ty, array_sz)) },
                FunctionType, FunType(return_ty, params, inline), attrs' => { return(FunctionType((FunType(return_ty, params, inline)), (++(attrs', attrs)))) },
                TypeDefType, tdr, quals', attrs' => { merge(quals', attrs')(TypeDefType(tdr)) },
            }
    }

    fn mkVarName(__0: NodeInfo, __1: Maybe(Ident), __2: Maybe(AsmName)) -> m(VarName) {
        match (__0, __1, __2) {
            node, Nothing, _ => { return(NoName) },
            node, Just(n), asm => { return(VarName(n, asm)) },
        }
    }

    fn nameOfDecl(d: CDecl) -> m(Ident) {
        >>=(getOnlyDeclr(d), Lambda)
    }

    fn splitCDecl(decl: CDecl, <todo>: m(Vec<CDecl>)) -> m(Vec<CDecl>) {
        match declrs {
                [] => { internalErr("splitCDecl applied to empty declaration".to_string()) },
                [declr] => { return(vec![decl]) },
                d1:ds => { Let },
            }
    }

    fn tArraySize(__0: CArrSize) -> m(ArraySize) {
        match (__0) {
            CNoArrSize(False) => { return((UnknownArraySize(False))) },
            CNoArrSize(True) => { return((UnknownArraySize(True))) },
            CArrSize(static, szexpr) => { liftM((ArraySize(static)), (return(szexpr))) },
        }
    }

    fn tAttr(CAttr(name, cexpr, node): CAttr) -> m(Attr) {
        return(Attr(name, cexpr, node))
    }

    fn tCompType(tag: SUERef, sue_ref: CompTyKind, member_decls: Vec<CDecl>, attrs: Attributes, node: NodeInfo) -> m(CompType) {
        ap(return((CompType(tag, sue_ref))), ap((concatMapM(tMemberDecls, member_decls)), ap((return(attrs)), (return(node)))))
    }

    fn tCompTypeDecl(handle_def: Bool, CStruct(tag, ident_opt, member_decls_opt, attrs, node_info): CStructUnion) -> m(CompTypeRef) {
        {

        }
    }

    fn tDirectType(handle_sue_def: Bool, node: NodeInfo, ty_quals: Vec<CTypeQual>, canonTySpec: TypeSpecAnalysis) -> m(Type) {
        {

        }
    }

    fn tEnumType(sue_ref: SUERef, enumerators: Vec<(Ident, Maybe(CExpr))>, attrs: Attributes, node: NodeInfo) -> m(EnumType) {
        {

        }
    }

    fn tMemberDecls(__0: CDecl) -> m(Vec<MemberDecl>) {
        match (__0) {
            CDecl(declspecs, [], node) => { {

            } },
            CDecl(declspecs, declrs, node) => { mapM((uncurry(tMemberDecl)), (zip((True:repeat(False)), declrs))) },
        }
    }

    fn tNumType(NumTypeSpec(basetype, sgn, sz, iscomplex): NumTypeSpec) -> m(Either((FloatType, Bool), IntType)) {
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

    fn tParamDecl(CDecl(declspecs, declrs, node): CDecl) -> m(ParamDecl) {
        {

        }
    }

    fn tTag(__0: CStructTag) -> CompTyKind {
        match (__0) {
            CStructTag => { StructTag },
            CUnionTag => { UnionTag },
        }
    }

    fn tType(handle_sue_def: Bool, top_node: NodeInfo, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, derived_declrs: Vec<CDerivedDeclr>, oldstyle_params: Vec<CDecl>) -> m(Type) {
        >>=(mergeOldStyle(top_node, oldstyle_params, derived_declrs), buildType)
    }

    fn tTypeQuals() -> m((TypeQuals, Attributes)) {
        foldrM(go, (noTypeQuals, vec![]))
    }

    fn typeDefRef(t_node: NodeInfo, name: Ident) -> m(TypeDefRef) {
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

    fn compatIdentEntry(__0: IdentEntry) -> Bool {
        match (__0) {
            Left(_tydef) => { either((const(True)), (const(False))) },
            Right(def) => { either((const(False)))(Lambda) },
        }
    }

    fn compatTagEntry(te1: TagEntry, te2: TagEntry) -> Bool {
        ==(tagKind(te1), tagKind(te2))
    }

    fn declStatusDescr(__0: DeclarationStatus(t)) -> String {
        match (__0) {
            NewDecl => { "new".to_string() },
            Redeclared(_) => { "redeclared".to_string() },
            KeepDef(_) => { "keep old".to_string() },
            Shadowed(_) => { "shadowed".to_string() },
            KindMismatch(_) => { "kind mismatch".to_string() },
        }
    }

    fn declareTag(sueref: SUERef, decl: TagFwdDecl, deftbl: DefTable) -> (DeclarationStatus(TagEntry), DefTable) {
        match lookupTag(sueref, deftbl) {
                Nothing => { (NewDecl, deftbl(hashmap! {
                        "tagDecls" => fst(defLocal((tagDecls(deftbl)), sueref, (Left(decl))))
                        })) },
            Just, old_def => if ==(tagKind(old_def), tagKind((Left(decl)))) { (KeepDef(old_def), deftbl) }
otherwise { (KindMismatch(old_def), deftbl) },
            }
    }

    fn defRedeclStatus(sameKind: fn(t) -> fn(t) -> Bool, def: t, oldDecl: Maybe(t)) -> DeclarationStatus(t) {
        match oldDecl {
            Just, def' => if sameKind(def, def') { Redeclared(def') }
otherwise { KindMismatch(def') },
                Nothing => { NewDecl },
            }
    }

    fn defRedeclStatusLocal(sameKind: fn(t) -> fn(t) -> Bool, ident: k, def: t, oldDecl: Maybe(t), nsm: NameSpaceMap(k, t)) -> DeclarationStatus(t) {
        match defRedeclStatus(sameKind, def, oldDecl) {
                NewDecl => { match lookupName(nsm, ident) {
                        Just, shadowed => { Shadowed(shadowed) },
                        Nothing => { NewDecl },
                    } },
                redecl => { redecl },
            }
    }

    fn defineGlobalIdent(ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus(IdentEntry), DefTable) {
        (defRedeclStatus(compatIdentEntry, (Right(def)), oldDecl), deftbl(hashmap! {
                "identDecls" => decls'
                }))
    }

    fn defineLabel(ident: Ident, deftbl: DefTable) -> (DeclarationStatus(Ident), DefTable) {
        Let
    }

    fn defineScopedIdent() -> (DeclarationStatus(IdentEntry), DefTable) {
        defineScopedIdentWhen((const(True)))
    }

    fn defineScopedIdentWhen(override_def: fn(IdentDecl) -> Bool, ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus(IdentEntry), DefTable) {
        (redecl_status, deftbl(hashmap! {
                "identDecls" => decls'
                }))
    }

    fn defineTag(sueref: SUERef, def: TagDef, deftbl: DefTable) -> (DeclarationStatus(TagEntry), DefTable) {
        (redeclStatus, deftbl(hashmap! {
                "tagDecls" => decls'
                }))
    }

    fn defineTypeDef(ident: Ident, tydef: TypeDef, deftbl: DefTable) -> (DeclarationStatus(IdentEntry), DefTable) {
        (defRedeclStatus(compatIdentEntry, (Left(tydef)), oldDecl), deftbl(hashmap! {
                "identDecls" => decls'
                }))
    }

    fn emptyDefTable() -> DefTable {
        DefTable(nameSpaceMap, nameSpaceMap, nameSpaceMap, nameSpaceMap, IntMap.empty, IntMap.empty)
    }

    fn enterBlockScope(deftbl: DefTable) -> DefTable {
        enterLocalScope(deftbl(hashmap! {
                "labelDefs" => enterNewScope((labelDefs(deftbl)))
                }))
    }

    fn enterFunctionScope(deftbl: DefTable) -> DefTable {
        enterLocalScope(deftbl(hashmap! {
                "labelDefs" => enterNewScope((labelDefs(deftbl)))
                }))
    }

    fn enterLocalScope(deftbl: DefTable) -> DefTable {
        deftbl(hashmap! {
            "identDecls" => enterNewScope((identDecls(deftbl))),
            "tagDecls" => enterNewScope((tagDecls(deftbl)))
            })
    }

    fn enterMemberDecl(deftbl: DefTable) -> DefTable {
        deftbl(hashmap! {
            "memberDecls" => enterNewScope((memberDecls(deftbl)))
            })
    }

    fn globalDefs(deftbl: DefTable) -> GlobalDecls {
        Map.foldWithKey(insertDecl, (GlobalDecls(e, gtags, e)), (globalNames(identDecls(deftbl))))
    }

    fn identOfTyDecl() -> Ident {
        either(identOfTypeDef, declIdent)
    }

    fn inFileScope(dt: DefTable) -> Bool {
        not((||(hasLocalNames((identDecls(dt))), hasLocalNames((labelDefs(dt))))))
    }

    fn insertType(dt: DefTable, n: Name, t: Type) -> DefTable {
        dt(hashmap! {
            "typeTable" => IntMap.insert((nameId(n)), t, (typeTable(dt)))
            })
    }

    fn leaveBlockScope(deftbl: DefTable) -> DefTable {
        leaveLocalScope(deftbl(hashmap! {
                "labelDefs" => leaveScope_((labelDefs(deftbl)))
                }))
    }

    fn leaveFunctionScope(deftbl: DefTable) -> DefTable {
        leaveLocalScope(deftbl(hashmap! {
                "labelDefs" => leaveScope_((labelDefs(deftbl)))
                }))
    }

    fn leaveLocalScope(deftbl: DefTable) -> DefTable {
        deftbl(hashmap! {
            "identDecls" => leaveScope_((identDecls(deftbl))),
            "tagDecls" => leaveScope_((tagDecls(deftbl)))
            })
    }

    fn leaveMemberDecl(deftbl: DefTable) -> (Vec<MemberDecl>, DefTable) {
        Let
    }

    fn leaveScope_() -> NameSpaceMap(k, a) {
        (fst . leaveScope)
    }

    fn lookupIdent(ident: Ident, deftbl: DefTable) -> Maybe(IdentEntry) {
        lookupName((identDecls(deftbl)), ident)
    }

    fn lookupIdentInner(ident: Ident, deftbl: DefTable) -> Maybe(IdentEntry) {
        lookupInnermostScope((identDecls(deftbl)), ident)
    }

    fn lookupLabel(ident: Ident, deftbl: DefTable) -> Maybe(Ident) {
        lookupName((labelDefs(deftbl)), ident)
    }

    fn lookupTag(sue_ref: SUERef, deftbl: DefTable) -> Maybe(TagEntry) {
        lookupName((tagDecls(deftbl)), sue_ref)
    }

    fn lookupTagInner(sue_ref: SUERef, deftbl: DefTable) -> Maybe(TagEntry) {
        lookupInnermostScope((tagDecls(deftbl)), sue_ref)
    }

    fn lookupType(dt: DefTable, n: Name) -> Maybe(Type) {
        IntMap.lookup((nameId(n)), (typeTable(dt)))
    }

    fn mergeDefTable(DefTable(i1, t1, l1, m1, r1, tt1): DefTable, DefTable(i2, t2, l2, m2, r2, tt2): DefTable) -> DefTable {
        DefTable((mergeNameSpace(i1, i2)), (mergeNameSpace(t1, t2)), (mergeNameSpace(l1, l2)), (mergeNameSpace(m1, m2)), (union(r1, r2)), (union(tt1, tt2)))
    }

    fn tagKind(__0: TagEntry) -> TagEntryKind {
        match (__0) {
            Left(CompDecl(cd)) => { CompKind((compTag(cd))) },
            Left(EnumDecl(_)) => { EnumKind },
            Right(CompDef(cd)) => { CompKind((compTag(cd))) },
            Right(EnumDef(_)) => { EnumKind },
        }
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/Export.hs"
Error: Unrecognized token `->`:
 35 |     (specs,derived) = exportType ty
 36 |      ;(ident,asmname) = case name of{ (VarName vident asmname_opt) ->
 37 |                                      _ -> (Nothing,Nothing)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/NameSpaceMap.hs"
Error: Unrecognized token `=`:
152 |   where{ localUnion (l1:ls1) (l2:ls2) =
153 |            List.unionBy (\p1 p2 -> fst p1 == fst p2) l1 l2 : localUnio
154 |          localUnion [] ls2 = ls2
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
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
        match (__0) {
            RedefInfo(ident, DuplicateDef, _, _) => { ++("duplicate definition of ".to_string(), ident) },
            RedefInfo(ident, ShadowedDef, _, _) => { ++("this declaration of ".to_string(), ++(ident, " shadows a previous one".to_string())) },
            RedefInfo(ident, DiffKindRedecl, _, _) => { ++(ident, " previously declared as a different kind of symbol".to_string()) },
            RedefInfo(ident, DisagreeLinkage, _, _) => { ++(ident, " previously declared with different linkage".to_string()) },
            RedefInfo(ident, NoLinkageOld, _, _) => { ++(ident, " previously declared without linkage".to_string()) },
        }
    }

    fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, <todo>: ErrorInfo) -> ErrorInfo {
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

    struct GlobalDecls(GlobalDecls, { /* struct def */ });

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
    struct TypeQuals(TypeQuals, { /* struct def */ });

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

    fn declOfDef(def: n) -> Decl {
        Let
    }

    fn declStorage(d: d) -> Storage {
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

    fn filterGlobalDecls(decl_filter: fn(DeclEvent) -> Bool, gmap: GlobalDecls) -> GlobalDecls {
        GlobalDecls(hashmap! {
            "gObjs" => Map.filter(((decl_filter . DeclEvent)), (gObjs(gmap))),
            "gTags" => Map.filter(((decl_filter . TagEvent)), (gTags(gmap))),
            "gTypeDefs" => Map.filter(((decl_filter . TypeDefEvent)), (gTypeDefs(gmap)))
            })
    }

    fn hasLinkage(__0: Storage) -> Bool {
        match (__0) {
            Auto(_) => { False },
            Static(NoLinkage, _) => { False },
            _ => { True },
        }
    }

    fn identOfTypeDef(TypeDef(ide, _, _, _): TypeDef) -> Ident {
        ide
    }

    fn identOfVarName(__0: VarName) -> Ident {
        match (__0) {
            NoName => { error("identOfVarName: NoName".to_string()) },
            VarName(ident, _) => { ident },
        }
    }

    fn isExtDecl() -> Bool {
        (hasLinkage . declStorage)
    }

    fn isNoName(__0: VarName) -> Bool {
        match (__0) {
            NoName => { True },
            _ => { False },
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
        match (__0) {
            Declaration(_) => { "declaration".to_string() },
            ObjectDef(_) => { "object definition".to_string() },
            FunctionDef(_) => { "function definition".to_string() },
            EnumeratorDef(_) => { "enumerator definition".to_string() },
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
        match (__0) {
            CompDef(comptype) => { typeOfCompDef(comptype) },
            EnumDef(enumtype) => { typeOfEnumDef(enumtype) },
        }
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TravMonad.hs"
Error: Unrecognized token `->`:
372 |
373 |
374 | ;newtype Trav s a = Trav { unTrav :: TravState s -> Either CError (a,
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TypeCheck.hs"
Error: Unrecognized token `|`:
 95 |   do{ n <- genName
 96 |       let{ charType | wide      = TyInt
 97 |                     | otherwise = TyChar
~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Analysis_TypeConversions {
    fn arithmeticConversion(__0: TypeName, __1: TypeName) -> Maybe(TypeName) {
        match (__0, __1) {
            TyComplex(t1), TyComplex(t2) => { Just(TyComplex(floatConversion(t1, t2))) },
            TyComplex(t1), TyFloating(t2) => { Just(TyComplex(floatConversion(t1, t2))) },
            TyFloating(t1), TyComplex(t2) => { Just(TyComplex(floatConversion(t1, t2))) },
            t1, <todo>, TyComplex(_), TyIntegral(_) => { Just(t1) },
            TyIntegral(_), t2, <todo>, TyComplex(_) => { Just(t2) },
            TyFloating(t1), TyFloating(t2) => { Just(TyFloating(floatConversion(t1, t2))) },
            t1, <todo>, TyFloating(_), TyIntegral(_) => { Just(t1) },
            TyIntegral(_), t2, <todo>, TyFloating(_) => { Just(t2) },
            TyIntegral(t1), TyIntegral(t2) => { Just(TyIntegral(intConversion(t1, t2))) },
            TyEnum(_), TyEnum(_) => { Just(TyIntegral(TyInt)) },
            TyEnum(_), t2 => { Just(t2) },
            t1, TyEnum(_) => { Just(t1) },
            _, _ => { Nothing },
        }
    }

    fn floatConversion() -> FloatType {
        max
    }

    fn intConversion(t1: IntType, t2: IntType) -> IntType {
        max(TyInt, (max(t1, t2)))
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Analysis/TypeUtils.hs"
Error: Unrecognized token `->`:
136 | ;isFunctionType ty =
137 |     case ty of{  TypeDefType (TypeDefRef _ (Just actual_ty) _) _ _ ->
138 |                  TypeDefType _ _ _ -> error "aXNGdW5jdGlvblR5cGU6IHVuc
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
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
        ++(header, showMsgLines((if(null, short_msg, then, msgs, else, short_msg:msgs))))
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
        match (__0) {
            AnonymousRef(_) => { True },
            _ => { False },
        }
    }

    fn isInternalIdent(Ident(_, _, nodeinfo): Ident) -> Bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo'(pos, (pos, length(s)), name)))
    }

    fn quad(__0: String) -> isize {
        match (__0) {
            c1:c2:c3:c4:s => { +((mod((*(ord(c4), +(bits21, *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28))) },
            c1:c2:c3:([]) => { *(ord(c3), +(bits14, *(ord(c2), +(bits7, ord(c1))))) },
            c1:c2:([]) => { *(ord(c2), +(bits7, ord(c1))) },
            c1:([]) => { ord(c1) },
            [] => { 0 },
        }
    }

}

mod Language_C_Data_InputStream {
    fn countLines() -> isize {
        match () {
             => { (length . BSC.lines) },
             => { (length . lines) },
        }
    }

    fn inputStreamEmpty() -> Bool {
        match () {
             => { BSW.null },
             => { null },
        }
    }

    fn inputStreamFromString() -> InputStream {
        match () {
             => { BSC.pack },
             => { id },
        }
    }

    fn inputStreamToString() -> String {
        match () {
             => { BSC.unpack },
             => { id },
        }
    }

    fn readInputStream() -> IO(InputStream) {
        match () {
             => { BSW.readFile },
             => { readFile },
        }
    }

    fn takeByte(bs: InputStream) -> (Word8, InputStream) {
        seq(BSW.head(bs), (BSW.head(bs), BSW.tail(bs)))
    }

    fn takeChar(__0: InputStream) -> (Char, InputStream) {
        match (__0) {
            bs => { seq(BSC.head(bs), (BSC.head(bs), BSC.tail(bs))) },
            bs => { (head(bs), tail(bs)) },
        }
    }

    fn takeChars(__0: isize, __1: InputStream) -> Vec<Char> {
        match (__0, __1) {
            n, bstr => { BSC.unpack(BSC.take(n, bstr)) },
            n, str => { take(n, str) },
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

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Data/Node.hs"
Error: Unrecognized token `->`:
 56 |     where{
 57 |     len = case ni of{ NodeInfo firstPos lastTok _ -> computeLength fir
 58 |                        OnlyPos firstPos lastTok -> computeLength first
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Data_Position {
    #[derive(Eq, Ord, Debug, Clone)]
    struct Position(Position, { /* struct def */ }, NoPosition, BuiltinPosition, InternalPosition);

    fn adjustPos(__0: FilePath, __1: isize, __2: Position) -> Position {
        match (__0, __1, __2) {
            fname, row, Position(offs, _, _, _) => { Position(offs, fname, row, 1) },
            _, _, p => { p },
        }
    }

    fn builtinPos() -> Position {
        BuiltinPosition
    }

    fn incOffset(__0: Position, __1: isize) -> Position {
        match (__0, __1) {
            Position(o, f, r, c), n => { Position((+(o, n)), f, r, c) },
            p, n => { p },
        }
    }

    fn incPos(__0: Position, __1: isize) -> Position {
        match (__0, __1) {
            Position(offs, fname, row, col), n => { Position((+(offs, n)), fname, row, (+(col, n))) },
            p, _ => { p },
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
            BuiltinPosition => { True },
            _ => { False },
        }
    }

    fn isInternalPos(__0: Position) -> Bool {
        match (__0) {
            InternalPosition => { True },
            _ => { False },
        }
    }

    fn isNoPos(__0: Position) -> Bool {
        match (__0) {
            NoPosition => { True },
            _ => { False },
        }
    }

    fn isSourcePos(__0: Position) -> Bool {
        match (__0) {
            Position(_, _, _, _) => { True },
            _ => { False },
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
            Position(offs, fname, row, _) => { Position((+(offs, 1)), fname, (+(row, 1)), 1) },
            p => { p },
        }
    }

}

mod Language_C_Data_RList {
    fn appendr(xs: Vec<a>, Reversed(ys): Reversed(Vec<a>)) -> Reversed(Vec<a>) {
        Reversed((++(ys, List.reverse(xs))))
    }

    fn empty() -> Reversed(Vec<a>) {
        Reversed(vec![])
    }

    fn rappend(Reversed(xs): Reversed(Vec<a>), ys: Vec<a>) -> Reversed(Vec<a>) {
        Reversed((++(List.reverse(ys), xs)))
    }

    fn rappendr(Reversed(xs): Reversed(Vec<a>), Reversed(ys): Reversed(Vec<a>)) -> Reversed(Vec<a>) {
        Reversed((++(ys, xs)))
    }

    fn reverse(Reversed(xs): Reversed(Vec<a>)) -> Vec<a> {
        List.reverse(xs)
    }

    fn rmap(f: fn(a) -> b, Reversed(xs): Reversed(Vec<a>)) -> Reversed(Vec<b>) {
        Reversed((map(f, xs)))
    }

    fn singleton(x: a) -> Reversed(Vec<a>) {
        Reversed(vec![x])
    }

    fn snoc(Reversed(xs): Reversed(Vec<a>), x: a) -> Reversed(Vec<a>) {
        Reversed((:(x, xs)))
    }

    fn viewr(__0: Reversed(Vec<a>)) -> (Reversed(Vec<a>), a) {
        match (__0) {
            Reversed([]) => { error("viewr: empty RList".to_string()) },
            Reversed(x:xs) => { (Reversed(xs), x) },
        }
    }

}

mod Language_C_Data {

}

mod Language_C_Parser_Builtin {
    fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Parser/ParserMonad.hs"
Error: Unrecognized token `CTokEof`:
 45 | ;import Language.C.Data.Name    (Name)
 46 | ;import Language.C.Data.Ident    (Ident)
 47 | ;import Language.C.Parser.Tokens (CToken(CTokEof))
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./language-c/src/Language/C/Parser/Tokens.hs"
Error: Unrecognized token `|`:
255 |    ;showsPrec _ (CTokTilde    _  ) = showString "fg=="
256 |    ;showsPrec _ (CTokInc      _  ) = showString "Kys="
257 | QcmVjIF8gKENUb2tCYXIgICAgICBfICApID0gc2hvd1N0cmluZyA="|"CiAgc2hvd3NQcm
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Parser {
    fn execParser_(parser: P(a), input: InputStream, pos: Position) -> Either(ParseError, a) {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Pretty.hs"
Error: Unrecognized token `->`:
384 |          parenPrec p 26 $ prettyPrec 26 expr <> text "Kys="
385 |      ;prettyPrec p (CUnary CPostDecOp expr _) =
386 | gICAgICAgICAgICAgICA8PiB0ZXh0IChpZiBkZXJlZiB0aGVuIA=="->"IGVsc2Ug"."KS
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
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

    fn fmapInitList(_f: fn(a) -> b) -> CInitializerList(b) {
        map((Lambda))
    }

    fn isSUEDef(__0: CTypeSpecifier(a)) -> Bool {
        match (__0) {
            CSUType(CStruct(_, _, Just(_), _, _), _) => { True },
            CEnumType(CEnum(_, Just(_), _, _), _) => { True },
            _ => { False },
        }
    }

    fn liftStrLit(CStrLit(str, at): CStringLiteral(a)) -> CConstant(a) {
        CStrConst(str, at)
    }

    fn partitionDeclSpecs() -> (Vec<CStorageSpecifier(a)>, Vec<CAttribute(a)>, Vec<CTypeQualifier(a)>, Vec<CTypeSpecifier(a)>, Bool) {
        foldr(deals, (vec![], vec![], vec![], vec![], False))
    }

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/Syntax/Constants.hs"
Error: Unrecognized token `->`:
107 |          showIFlag f = if testFlag f flags then show f else []
108 |           ;showInt i = case repr of{ DecRepr -> shows i
109 |                                     OctalRepr -> showString "MA==" . s
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_Syntax_Ops {
    #[derive(Eq, Ord, Show, Clone, Debug)]
    struct CAssignOp(CAssignOp, CMulAssOp, CDivAssOp, CRmdAssOp, CAddAssOp, CSubAssOp, CShlAssOp, CShrAssOp, CAndAssOp, CXorAssOp, COrAssOp);

    #[derive(Eq, Ord, Show, Clone, Debug)]
    struct CBinaryOp(CMulOp, CDivOp, CRmdOp, CAddOp, CSubOp, CShlOp, CShrOp, CLeOp, CGrOp, CLeqOp, CGeqOp, CEqOp, CNeqOp, CAndOp, CXorOp, COrOp, CLndOp, CLorOp);

    #[derive(Eq, Ord, Show, Clone, Debug)]
    struct CUnaryOp(CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp, CAdrOp, CIndOp, CPlusOp, CMinOp, CCompOp, CNegOp);

    fn assignBinop(__0: CAssignOp) -> CBinaryOp {
        match (__0) {
            CAssignOp => { error("direct assignment has no binary operator".to_string()) },
            CMulAssOp => { CMulOp },
            CDivAssOp => { CDivOp },
            CRmdAssOp => { CRmdOp },
            CAddAssOp => { CAddOp },
            CSubAssOp => { CSubOp },
            CShlAssOp => { CShlOp },
            CShrAssOp => { CShrOp },
            CAndAssOp => { CAndOp },
            CXorAssOp => { CXorOp },
            COrAssOp => { COrOp },
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

mod Language_C_Syntax_Utils {
    fn compoundSubStmts(__0: CBlockItem) -> Vec<CStat> {
        match (__0) {
            CBlockStmt(s) => { vec![s] },
            CBlockDecl(_) => { vec![] },
            CNestedFunDef(_) => { vec![] },
        }
    }

    fn getLabels(__0: CStat) -> Vec<Ident> {
        match (__0) {
            CLabel(l, s, _, _) => { :(l, getLabels(s)) },
            CCompound(ls, body, _) => { \\(concatMap(((concatMap(getLabels) . compoundSubStmts)), body), ls) },
            stmt => { concatMap(getLabels, (getSubStmts(stmt))) },
        }
    }

    fn getSubStmts(__0: CStat) -> Vec<CStat> {
        match (__0) {
            CLabel(_, s, _, _) => { vec![s] },
            CCase(_, s, _) => { vec![s] },
            CCases(_, _, s, _) => { vec![s] },
            CDefault(s, _) => { vec![s] },
            CExpr(_, _) => { vec![] },
            CCompound(_, body, _) => { concatMap(compoundSubStmts, body) },
            CIf(_, sthen, selse, _) => { maybe(vec![sthen], (Lambda), selse) },
            CSwitch(_, s, _) => { vec![s] },
            CWhile(_, s, _, _) => { vec![s] },
            CFor(_, _, _, s, _) => { vec![s] },
            CGoto(_, _) => { vec![] },
            CGotoPtr(_, _) => { vec![] },
            CCont(_) => { vec![] },
            CBreak(_) => { vec![] },
            CReturn(_, _) => { vec![] },
            CAsm(_, _) => { vec![] },
        }
    }

    fn mapBlockItemStmts(__0: fn(CStat) -> Bool, __1: fn(CStat) -> CStat, __2: CBlockItem) -> CBlockItem {
        match (__0, __1, __2) {
            stop, f, CBlockStmt(s) => { CBlockStmt((mapSubStmts(stop, f, s))) },
            _, _, bi => { bi },
        }
    }

    fn mapSubStmts(__0: fn(CStat) -> Bool, __1: fn(CStat) -> CStat, __2: CStat) -> CStat {
        match (__0, __1, __2) {
            stop, f, CLabel(i, s, attrs, ni) => { f((CLabel(i, (mapSubStmts(stop, f, s)), attrs, ni))) },
            stop, f, CCase(e, s, ni) => { f((CCase(e, (mapSubStmts(stop, f, s)), ni))) },
            stop, f, CCases(e1, e2, s, ni) => { f((CCases(e1, e2, (mapSubStmts(stop, f, s)), ni))) },
            stop, f, CDefault(s, ni) => { f((CDefault((mapSubStmts(stop, f, s)), ni))) },
            stop, f, CCompound(ls, body, ni) => { f((CCompound(ls, (map((mapBlockItemStmts(stop, f)), body)), ni))) },
            stop, f, CIf(e, sthen, selse, ni) => { f((CIf(e, (mapSubStmts(stop, f, sthen)), (maybe(Nothing, ((Just . mapSubStmts(stop, f))), selse)), ni))) },
            stop, f, CSwitch(e, s, ni) => { f((CSwitch(e, (mapSubStmts(stop, f, s)), ni))) },
            stop, f, CWhile(e, s, isdo, ni) => { f((CWhile(e, (mapSubStmts(stop, f, s)), isdo, ni))) },
            stop, f, CFor(i, t, a, s, ni) => { f((CFor(i, t, a, (mapSubStmts(stop, f, s)), ni))) },
            _, f, s => { f(s) },
        }
    }

}

mod Language_C_Syntax {

}

/* ERROR: cannot yet convert file "./language-c/src/Language/C/System/GCC.hs"
Error: Unrecognized token `}`:
 38 | rce target = do{ copyFile source target
 39 |                  p <- getPermissions target
 40 |                  setPermissions target p{writable=True}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
*/
mod Language_C_System_Preprocess {
    struct CppOption(IncludeDir, FilePath, Define, String, String, Undefine, String, IncludeFile, FilePath);

    struct CppArgs(CppArgs, { /* struct def */ });

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

    fn mkOutputFile(tmp_dir_opt: Maybe(FilePath), input_file: FilePath) -> IO(FilePath) {
        {

        }
    }

    fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> IO(FilePath) {
        {

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

    fn runPreprocessor(cpp: cpp, cpp_args: CppArgs) -> IO(Either(ExitCode, InputStream)) {
        {

        }
    }

}



fn main() { /* demo */ }
