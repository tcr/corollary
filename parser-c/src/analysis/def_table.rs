#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data;
// use Language::C::Analysis::NameSpaceMap;
// use Language::C::Analysis::SemRep;
// use Control::Applicative;
// use Data::Map;
// use Map;
// use Data::Map;
// use Data::IntMap;
// use IntMap;
// use Data::IntMap;
// use Data::Generics;

pub type IdentEntry = Either<TypeDef, IdentDecl>;

pub enum TagFwdDecl {
    CompDecl(CompTypeRef),
    EnumDecl(EnumTypeRef)
}
pub use self::TagFwdDecl::*;

pub type TagEntry = Either<TagFwdDecl, TagDef>;

pub struct DefTable{
    identDecls: NameSpaceMap<Ident, IdentEntry>,
    tagDecls: NameSpaceMap<SUERef, TagEntry>,
    labelDefs: NameSpaceMap<Ident, Ident>,
    memberDecls: NameSpaceMap<Ident, MemberDecl>,
    refTable: IntMap<Name>,
    typeTable: IntMap<Type>
}
fn identDecls(a: DefTable) -> NameSpaceMap<Ident, IdentEntry> { a.identDecls }
fn tagDecls(a: DefTable) -> NameSpaceMap<SUERef, TagEntry> { a.tagDecls }
fn labelDefs(a: DefTable) -> NameSpaceMap<Ident, Ident> { a.labelDefs }
fn memberDecls(a: DefTable) -> NameSpaceMap<Ident, MemberDecl> { a.memberDecls }
fn refTable(a: DefTable) -> IntMap<Name> { a.refTable }
fn typeTable(a: DefTable) -> IntMap<Type> { a.typeTable }

#[derive(Clone, Debug)]
pub enum DeclarationStatus<t> {
    NewDecl,
    Redeclared(t),
    KeepDef(t),
    Shadowed(t),
    KindMismatch(t)
}
pub use self::DeclarationStatus::*;

#[derive(Eq, Ord)]
pub enum TagEntryKind {
    CompKind(CompTyKind),
    EnumKind
}
pub use self::TagEntryKind::*;

pub fn compatIdentEntry<a>(_0: IdentEntry) -> bool {
    match (_0) {
        Left(_tydef) => {
            either((__TODO_const(true)), (__TODO_const(false)))
        },
        Right(def) => {
            either((__TODO_const(false)))(|other_def| { match (def, other_def) {
                    (EnumeratorDef(_), EnumeratorDef(_)) => {
                        true
                    },
                    (EnumeratorDef(_), _) => {
                        true
                    },
                    (_, EnumeratorDef(_)) => {
                        true
                    },
                    (_, _) => {
                        true
                    },
                } })
        },
    }
}

pub fn compatTagEntry<a>(te1: TagEntry, te2: TagEntry) -> bool {
    (tagKind(te1) == tagKind(te2))
}

pub fn declStatusDescr<a>(_0: DeclarationStatus<t>) -> String {
    match (_0) {
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

pub fn declareTag<a>(sueref: SUERef, decl: TagFwdDecl, deftbl: DefTable) -> (DeclarationStatus<TagEntry>, DefTable) {
    match lookupTag(sueref, deftbl) {
        None => {
            (NewDecl, deftbl {
                tagDecls: fst(defLocal((tagDecls(deftbl)), sueref, (Left(decl))))
            })
        },
        Some(old_def) if (tagKind(old_def) == tagKind((Left(decl)))) => { (KeepDef(old_def), deftbl) }
        Some(old_def) => { (KindMismatch(old_def), deftbl) }
    }
}

pub fn defRedeclStatus<a>(sameKind: fn(t) -> fn(t) -> bool, def: t, oldDecl: Option<t>) -> DeclarationStatus<t> {
    match oldDecl {
        Some(def_q) if sameKind(def, def_q) => { Redeclared(def_q) }
        Some(def_q) => { KindMismatch(def_q) }
        None => {
            NewDecl
        },
    }
}

pub fn defRedeclStatusLocal<a>(sameKind: fn(t) -> fn(t) -> bool, ident: k, def: t, oldDecl: Option<t>, nsm: NameSpaceMap<k, t>) -> DeclarationStatus<t> {
    match defRedeclStatus(sameKind, def, oldDecl) {
        NewDecl => {
            match lookupName(nsm, ident) {
                Some(shadowed) => {
                    Shadowed(shadowed)
                },
                None => {
                    NewDecl
                },
            }
        },
        redecl => {
            redecl
        },
    }
}

pub fn defineGlobalIdent<a>(ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {
    (defRedeclStatus(compatIdentEntry, (Right(def)), oldDecl), deftbl {
        identDecls: decls_q
    })
}

pub fn defineLabel<a>(ident: Ident, deftbl: DefTable) -> (DeclarationStatus<Ident>, DefTable) {
    {
        let (labels_q, old_label) = defLocal((labelDefs(deftbl)), ident, ident);

    (maybe(NewDecl, Redeclared, old_label), deftbl {
            labelDefs: labels_q
        })    }
}

pub fn defineScopedIdent<a>() -> (DeclarationStatus<IdentEntry>, DefTable) {
    defineScopedIdentWhen((__TODO_const(true)))
}

pub fn defineScopedIdentWhen<a>(override_def: fn(IdentDecl) -> bool, ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {
    (redecl_status, deftbl {
        identDecls: decls_q
    })
}

pub fn defineTag<a>(sueref: SUERef, def: TagDef, deftbl: DefTable) -> (DeclarationStatus<TagEntry>, DefTable) {
    (redeclStatus, deftbl {
        tagDecls: decls_q
    })
}

pub fn defineTypeDef<a>(ident: Ident, tydef: TypeDef, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {
    (defRedeclStatus(compatIdentEntry, (Left(tydef)), oldDecl), deftbl {
        identDecls: decls_q
    })
}

pub fn emptyDefTable<a>() -> DefTable {
    DefTable(nameSpaceMap, nameSpaceMap, nameSpaceMap, nameSpaceMap, IntMap::empty, IntMap::empty)
}

pub fn enterBlockScope<a>(deftbl: DefTable) -> DefTable {
    enterLocalScope(deftbl {
        labelDefs: enterNewScope((labelDefs(deftbl)))
    })
}

pub fn enterFunctionScope<a>(deftbl: DefTable) -> DefTable {
    enterLocalScope(deftbl {
        labelDefs: enterNewScope((labelDefs(deftbl)))
    })
}

pub fn enterLocalScope<a>(deftbl: DefTable) -> DefTable {
    deftbl {
        identDecls: enterNewScope((identDecls(deftbl))),
        tagDecls: enterNewScope((tagDecls(deftbl)))
    }
}

pub fn enterMemberDecl<a>(deftbl: DefTable) -> DefTable {
    deftbl {
        memberDecls: enterNewScope((memberDecls(deftbl)))
    }
}

pub fn globalDefs<a>(deftbl: DefTable) -> GlobalDecls {
    Map::foldWithKey(insertDecl, (GlobalDecls(e, gtags, e)), (globalNames(identDecls(deftbl))))
}

pub fn identOfTyDecl<a>() -> Ident {
    either(identOfTypeDef, declIdent)
}

pub fn inFileScope<a>(dt: DefTable) -> bool {
    not(((hasLocalNames((identDecls(dt))) || hasLocalNames((labelDefs(dt))))))
}

pub fn insertType<a>(dt: DefTable, n: Name, t: Type) -> DefTable {
    dt {
        typeTable: IntMap::insert((nameId(n)), t, (typeTable(dt)))
    }
}

pub fn leaveBlockScope<a>(deftbl: DefTable) -> DefTable {
    leaveLocalScope(deftbl {
        labelDefs: leaveScope_((labelDefs(deftbl)))
    })
}

pub fn leaveFunctionScope<a>(deftbl: DefTable) -> DefTable {
    leaveLocalScope(deftbl {
        labelDefs: leaveScope_((labelDefs(deftbl)))
    })
}

pub fn leaveLocalScope<a>(deftbl: DefTable) -> DefTable {
    deftbl {
        identDecls: leaveScope_((identDecls(deftbl))),
        tagDecls: leaveScope_((tagDecls(deftbl)))
    }
}

pub fn leaveMemberDecl<a>(deftbl: DefTable) -> (Vec<MemberDecl>, DefTable) {
    {
        let (decls_q, members) = leaveScope((memberDecls(deftbl)));

    __op_tuple2((), (__map!(snd, members))((deftbl {
                memberDecls: decls_q
            })))    }
}

pub fn leaveScope_<a>() -> NameSpaceMap<k, a> {
    fst(leaveScope)
}

pub fn lookupIdent<a>(ident: Ident, deftbl: DefTable) -> Option<IdentEntry> {
    lookupName((identDecls(deftbl)), ident)
}

pub fn lookupIdentInner<a>(ident: Ident, deftbl: DefTable) -> Option<IdentEntry> {
    lookupInnermostScope((identDecls(deftbl)), ident)
}

pub fn lookupLabel<a>(ident: Ident, deftbl: DefTable) -> Option<Ident> {
    lookupName((labelDefs(deftbl)), ident)
}

pub fn lookupTag<a>(sue_ref: SUERef, deftbl: DefTable) -> Option<TagEntry> {
    lookupName((tagDecls(deftbl)), sue_ref)
}

pub fn lookupTagInner<a>(sue_ref: SUERef, deftbl: DefTable) -> Option<TagEntry> {
    lookupInnermostScope((tagDecls(deftbl)), sue_ref)
}

pub fn lookupType<a>(dt: DefTable, n: Name) -> Option<Type> {
    IntMap::lookup((nameId(n)), (typeTable(dt)))
}

pub fn mergeDefTable<a>(DefTable(i1, t1, l1, m1, r1, tt1): DefTable, DefTable(i2, t2, l2, m2, r2, tt2): DefTable) -> DefTable {
    DefTable((mergeNameSpace(i1, i2)), (mergeNameSpace(t1, t2)), (mergeNameSpace(l1, l2)), (mergeNameSpace(m1, m2)), (union(r1, r2)), (union(tt1, tt2)))
}

pub fn tagKind<a>(_0: TagEntry) -> TagEntryKind {
    match (_0) {
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



