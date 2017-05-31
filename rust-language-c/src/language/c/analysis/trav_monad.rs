use haskell_support::*;

use Language::C::Data;
use Language::C::Data::Ident;
use Language::C::Data::RList;
use Language::C::Syntax;
use Language::C::Analysis::Builtins;
use Language::C::Analysis::SemError;
use Language::C::Analysis::SemRep;
use Language::C::Analysis::DefTable;
use Language::C::Analysis::DefTable;
use Data::IntMap;
use insert;
use Data::Maybe;
use Control::Monad;
use liftM;
use Prelude;

pub enum CLanguage {
    C89,
    C99,
    GNU89,
    GNU99
}
pub use self::CLanguage::*;

struct TravOptions{
    language: CLanguage
}

struct TravState<s>{
    symbolTable: DefTable,
    rerrors: RList<CError>,
    nameGenerator: Vec<Name>,
    doHandleExtDecl: fn(DeclEvent) -> Trav<s, ()>,
    userState: s,
    options: TravOptions
}

pub fn addRef(__use: u, def: d) -> m<()> {
    match (nodeInfo(__use), nodeInfo(def)) {
        (NodeInfo(_, _, useName), NodeInfo(_, _, defName)) => {
            withDefTable((|dt| { ((), dt {
                        refTable: insert((nameId(useName)), defName, (refTable(dt)))
                    }) }))
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
        KeepDef(Right(old_def)) if not((agreeOnLinkage(def, old_def))) => { linkageErr(def, old_def) }
        KeepDef(Right(old_def)) => { throwOnLeft(checkCompatibleTypes(new_ty, (declType(old_def)))) }
        Redeclared(Right(old_def)) if not((agreeOnLinkage(def, old_def))) => { linkageErr(def, old_def) }
        Redeclared(Right(old_def)) if not((canBeOverwritten(old_def))) => { redefVarErr(old_def, DuplicateDef) }
        Redeclared(Right(old_def)) => { throwOnLeft(checkCompatibleTypes(new_ty, (declType(old_def)))) }
        _ => {
            ()
        },
    }
}

pub fn concatMapM(f: fn(a) -> m<Vec<b>>) -> m<Vec<b>> {
    liftM(concat, mapM(f))
}

pub fn createSUERef(__0: NodeInfo, __1: Option<Ident>) -> m<SUERef> {
    match (__0, __1) {
        (_node_info, Some(ident)) => {
            return(NamedRef(ident))
        },
        (node_info, None) => {
            /* Expr::Error */ Error
        },
    }
}

pub fn enterBlockScope() -> m<()> {
    updDefTable((ST::enterBlockScope))
}

pub fn enterDecl(decl: Decl, cond: fn(IdentDecl) -> bool) -> m<IdentDecl> {
    /* do */ {
        let def = Declaration(decl);

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
            let [new_name, gen_q] = nameGenerator(ts);

            put(ts {
                nameGenerator: gen_q
            });
            new_name
        } })
}

pub fn get() -> Trav<s, TravState<s>> {
    Trav((|s| { Right((s, s)) }))
}

pub fn getUserState() -> Trav<s, s> {
    liftM(userState, get)
}

pub fn gets(f: fn(TravState<s>) -> a) -> Trav<s, a> {
    Trav((|s| { Right((f(s), s)) }))
}

pub fn hadHardErrors() -> bool {
    (not(null(filter(isHardError))))
}

pub fn handleAsmBlock(asm: AsmBlock) -> m<()> {
    handleDecl((AsmEvent(asm)))
}

pub fn handleEnumeratorDef(enumerator: Enumerator) -> m<()> {
    /* do */ {
        let ident = declIdent(enumerator);

        let redecl = withDefTable(defineScopedIdent(ident, (EnumeratorDef(enumerator))));

        checkRedef((show(ident)), ident, redecl);
        ()
    }
}

pub fn handleFunDef(ident: Ident, fun_def: FunDef) -> m<()> {
    /* do */ {
        let def = FunctionDef(fun_def);

        let redecl = withDefTable(defineScopedIdentWhen(isDeclaration, ident, def));

        checkVarRedef(def, redecl);
        handleDecl((DeclEvent(def)))
    }
}

pub fn handleObjectDef(local: bool, ident: Ident, obj_def: ObjDef) -> m<()> {
    /* do */ {
        let def = ObjectDef(obj_def);

        let redecl = withDefTable(defineScopedIdentWhen((|old| { shouldOverride(def, old) }), ident, def));

        checkVarRedef(def, redecl);
        handleDecl(((__TODO_if(local, then, LocalEvent, __TODO_else, DeclEvent))(def)))
    }
}

pub fn handleParamDecl(__0: ParamDecl, __1: m<()>) -> m<()> {
    match (__0, __1, __2) {
        (pd, __OP__, AbstractParamDecl(_, _)) => {
            handleDecl((ParamEvent(pd)))
        },
        (pd, __OP__, ParamDecl(vardecl, node)) => {
            /* do */ {
                let def = ObjectDef((ObjDef(vardecl, None, node)));

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
    catchTravError(liftM(Some, a), (|e| { __op_rshift(recordError(e), None) }))
}

pub fn handleTypeDef(typeDef: TypeDef, __OP__: m<()>) -> m<()> {
    /* do */ {
        let redecl = withDefTable(defineTypeDef(ident, typeDef));

        checkRedef((show(ident)), typeDef, redecl);
        handleDecl((TypeDefEvent(typeDef)));
        ()
    }
}

pub fn handleVarDecl(is_local: bool, decl: Decl) -> m<()> {
    /* do */ {
        let def = enterDecl(decl, (__TODO_const(false)));

        handleDecl(((__TODO_if(is_local, then, LocalEvent, __TODO_else, DeclEvent))(def)))
    }
}

pub fn initTravState(userst: s) -> TravState<s> {
    TravState {
        symbolTable: emptyDefTable,
        rerrors: RList::empty,
        nameGenerator: newNameSupply,
        doHandleExtDecl: __TODO_const((())),
        userState: userst,
        options: TravOptions {
                language: C99
            }
    }
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
    liftM((__op_tuple2(a)), (f(b)))
}

pub fn maybeM(m: Option<a>, f: fn(a) -> m<()>) -> m<()> {
    maybe((()), f, m)
}

pub fn mismatchErr(ctx: String, expect: String, found: String) -> String {
    __op_addadd(ctx, __op_addadd(": Expected ".to_string(), __op_addadd(expect, __op_addadd(", but found: ".to_string(), found))))
}

pub fn modify(f: fn(TravState<s>) -> TravState<s>) -> Trav<s, ()> {
    Trav((|s| { Right(((), f(s))) }))
}

pub fn modifyOptions(f: fn(TravOptions) -> TravOptions) -> Trav<s, ()> {
    modify(|ts| { ts {
            options: f((options(ts)))
        } })
}

pub fn modifyUserState(f: fn(s) -> s) -> Trav<s, ()> {
    modify(|ts| { ts {
            userState: f((userState(ts)))
        } })
}

pub fn put(s: TravState<s>) -> Trav<s, ()> {
    Trav((|_| { Right(((), s)) }))
}

pub fn redefErr(name: Ident, lvl: ErrorLevel, new: new, old: old, kind: RedefKind) -> m<()> {
    throwTravError(redefinition(lvl, (show(name)), kind, (nodeInfo(new)), (nodeInfo(old))))
}

pub fn runTrav(state: s, traversal: Trav<s, a>) -> Either<Vec<CError>, (a, TravState<s>)> {
    match unTrav(action, (initTravState(state))) {
        Left(trav_err) => {
            Left(vec![trav_err])
        },
        Right((v, ts)) if hadHardErrors((travErrors(ts))) => { Left((travErrors(ts))) }
        Right((v, ts)) => { Right((v, ts)) }
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
        modify(|st| { st {
                doHandleExtDecl: handler
            } });
        action
    }
}

