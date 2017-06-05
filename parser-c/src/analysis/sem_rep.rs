// Original file: "SemRep.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data;
// use Language::C::Syntax;
// use Data::Map;
// use Map;
// use Data::Map;
// use Data::Maybe;
// use Data::Generics;

#[derive(Clone, Debug)]
pub enum TagDef {
    CompDef(CompType),
    EnumDef(EnumType)
}
pub use self::TagDef::*;

pub fn typeOfTagDef(_0: TagDef) -> TypeName {
    match (_0) {
        CompDef(comptype) => {
            typeOfCompDef(comptype)
        },
        EnumDef(enumtype) => {
            typeOfEnumDef(enumtype)
        },
    }
}

pub fn declOfDef(def: n) -> Decl {
    {
        let vd = getVarDecl(def);

    Decl(vd, (nodeInfo(def)))    }
}

pub fn declIdent() -> Ident {
    identOfVarName(declName)
}

pub fn declName() -> VarName {
    (|VarDecl(n, _, _)| { n }(getVarDecl))
}

pub fn declType() -> Type {
    (|VarDecl(_, _, ty)| { ty }(getVarDecl))
}

pub fn declAttrs() -> DeclAttrs {
    (|VarDecl(_, specs, _)| { specs }(getVarDecl))
}

#[derive(Clone, Debug)]
pub enum IdentDecl {
    Declaration(Decl),
    ObjectDef(ObjDef),
    FunctionDef(FunDef),
    EnumeratorDef(Enumerator)
}
pub use self::IdentDecl::*;

pub fn objKindDescr(_0: IdentDecl) -> String {
    match (_0) {
        Declaration(_) => {
            "declaration".to_string()
        },
        ObjectDef(_) => {
            "object definition".to_string()
        },
        FunctionDef(_) => {
            "function definition".to_string()
        },
        EnumeratorDef(_) => {
            "enumerator definition".to_string()
        },
    }
}

pub fn splitIdentDecls(include_all: bool) -> (Map<Ident, Decl>, (Map<Ident, Enumerator>, Map<Ident, ObjDef>, Map<Ident, FunDef>)) {

    let deal = |ident, entry, (decls, defs)| {
        (Map::insert(ident, (declOfDef(entry)), decls), addDef(ident, entry, defs))
    };

    let deal_q = |_0, _1, _2| {
        match (_0, _1, _2) {
            (ident, Declaration(d), (decls, defs)) => {
                (Map::insert(ident, d, decls), defs)
            },
            (ident, def, (decls, defs)) => {
                (decls, addDef(ident, def, defs))
            },
        }
    };

    let addDef = |ident, entry, (es, os, fs)| {
        match entry {
            Declaration(_) => {
                (es, os, fs)
            },
            EnumeratorDef(e) => {
                (Map::insert(ident, e, es), os, fs)
            },
            ObjectDef(o) => {
                (es, Map::insert(ident, o, os), fs)
            },
            FunctionDef(f) => {
                (es, os, Map::insert(ident, f, fs))
            },
        }
    };

    Map::foldWithKey((if include_all {         
deal} else {
deal_q
        }), (Map::empty, (Map::empty, Map::empty, Map::empty)))
}

pub struct GlobalDecls{
    gObjs: Map<Ident, IdentDecl>,
    gTags: Map<SUERef, TagDef>,
    gTypeDefs: Map<Ident, TypeDef>
}
fn gObjs(a: GlobalDecls) -> Map<Ident, IdentDecl> { a.gObjs }
fn gTags(a: GlobalDecls) -> Map<SUERef, TagDef> { a.gTags }
fn gTypeDefs(a: GlobalDecls) -> Map<Ident, TypeDef> { a.gTypeDefs }

pub fn emptyGlobalDecls() -> GlobalDecls {
    GlobalDecls(Map::empty, Map::empty, Map::empty)
}

pub fn filterGlobalDecls(decl_filter: fn(DeclEvent) -> bool, gmap: GlobalDecls) -> GlobalDecls {
    GlobalDecls {
        gObjs: Map::filter((decl_filter(DeclEvent)), (gObjs(gmap))),
        gTags: Map::filter((decl_filter(TagEvent)), (gTags(gmap))),
        gTypeDefs: Map::filter((decl_filter(TypeDefEvent)), (gTypeDefs(gmap)))
    }
}

pub fn mergeGlobalDecls(gmap1: GlobalDecls, gmap2: GlobalDecls) -> GlobalDecls {
    GlobalDecls {
        gObjs: Map::union((gObjs(gmap1)), (gObjs(gmap2))),
        gTags: Map::union((gTags(gmap1)), (gTags(gmap2))),
        gTypeDefs: Map::union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
    }
}

pub enum DeclEvent {
    TagEvent(TagDef),
    DeclEvent(IdentDecl),
    ParamEvent(ParamDecl),
    LocalEvent(IdentDecl),
    TypeDefEvent(TypeDef),
    AsmEvent(AsmBlock)
}
pub use self::DeclEvent::*;

#[derive(Clone, Debug)]
pub struct Decl(VarDecl, NodeInfo);


#[derive(Clone, Debug)]
pub struct ObjDef(VarDecl, Option<Initializer>, NodeInfo);


#[derive(Clone, Debug)]
pub struct FunDef(VarDecl, Stmt, NodeInfo);


#[derive(Clone, Debug)]
pub enum ParamDecl {
    ParamDecl(VarDecl, NodeInfo),
    AbstractParamDecl(VarDecl, NodeInfo)
}
pub use self::ParamDecl::*;

#[derive(Clone, Debug)]
pub enum MemberDecl {
    MemberDecl(VarDecl, Option<Expr>, NodeInfo),
    AnonBitField(Type, Expr, NodeInfo)
}
pub use self::MemberDecl::*;

#[derive(Clone, Debug)]
pub struct TypeDef(Ident, Type, Attributes, NodeInfo);


pub fn identOfTypeDef(TypeDef(ide, _, _, _): TypeDef) -> Ident {
    ide
}

#[derive(Clone, Debug)]
pub struct VarDecl(pub VarName, pub DeclAttrs, pub Type);


pub fn isExtDecl() -> bool {
    hasLinkage(declStorage)
}

#[derive(Clone, Debug)]
pub struct DeclAttrs(FunctionAttrs, Storage, Attributes);


pub fn declStorage(d: d) -> Storage {
    match declAttrs(d) {
        DeclAttrs(_, st, _) => {
            st
        },
    }
}

pub fn functionAttrs(d: d) -> FunctionAttrs {
    match declAttrs(d) {
        DeclAttrs(fa, _, _) => {
            fa
        },
    }
}

#[derive(Clone, Debug, Eq, Ord)]
pub struct FunctionAttrs{
    isInline: bool,
    isNoreturn: bool
}
fn isInline(a: FunctionAttrs) -> bool { a.isInline }
fn isNoreturn(a: FunctionAttrs) -> bool { a.isNoreturn }

pub fn noFunctionAttrs() -> FunctionAttrs {
    FunctionAttrs {
        isInline: false,
        isNoreturn: false
    }
}

#[derive(Clone, Debug, Eq, Ord)]
pub enum Storage {
    NoStorage,
    Auto(Register),
    Static(Linkage, ThreadLocal),
    FunLinkage(Linkage)
}
pub use self::Storage::*;

pub type ThreadLocal = bool;

pub type Register = bool;

#[derive(Clone, Debug, Eq, Ord)]
pub enum Linkage {
    NoLinkage,
    InternalLinkage,
    ExternalLinkage
}
pub use self::Linkage::*;

pub fn hasLinkage(_0: Storage) -> bool {
    match (_0) {
        Auto(_) => {
            false
        },
        Static(NoLinkage, _) => {
            false
        },
        _ => {
            true
        },
    }
}

pub fn declLinkage(decl: d) -> Linkage {
    match declStorage(decl) {
        NoStorage => {
            undefined
        },
        Auto(_) => {
            NoLinkage
        },
        Static(linkage, _) => {
            linkage
        },
        FunLinkage(linkage) => {
            linkage
        },
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    DirectType(TypeName, TypeQuals, Attributes),
    PtrType(Type, TypeQuals, Attributes),
    ArrayType(Type, ArraySize, TypeQuals, Attributes),
    FunctionType(FunType, Attributes),
    TypeDefType(TypeDefRef, TypeQuals, Attributes)
}
pub use self::Type::*;

#[derive(Clone, Debug)]
pub enum FunType {
    FunType(Type, Vec<ParamDecl>, bool),
    FunTypeIncomplete(Type)
}
pub use self::FunType::*;

#[derive(Clone, Debug)]
pub enum ArraySize {
    UnknownArraySize(bool),
    ArraySize(bool, Expr)
}
pub use self::ArraySize::*;

#[derive(Clone, Debug)]
pub enum TypeName {
    TyVoid,
    TyIntegral(IntType),
    TyFloating(FloatType),
    TyComplex(FloatType),
    TyComp(CompTypeRef),
    TyEnum(EnumTypeRef),
    TyBuiltin(BuiltinType)
}
pub use self::TypeName::*;

#[derive(Clone, Debug)]
pub enum BuiltinType {
    TyVaList,
    TyAny
}
pub use self::BuiltinType::*;

#[derive(Clone, Debug)]
pub struct TypeDefRef(Ident, Type, NodeInfo);


#[derive(Clone, Debug, Eq, Ord)]
pub enum IntType {
    TyBool,
    TyChar,
    TySChar,
    TyUChar,
    TyShort,
    TyUShort,
    TyInt,
    TyUInt,
    TyInt128,
    TyUInt128,
    TyLong,
    TyULong,
    TyLLong,
    TyULLong
}
pub use self::IntType::*;

#[derive(Clone, Debug, Eq, Ord)]
pub enum FloatType {
    TyFloat,
    TyDouble,
    TyLDouble
}
pub use self::FloatType::*;

#[derive(Clone, Debug)]
pub struct CompTypeRef(SUERef, CompTyKind, NodeInfo);


#[derive(Clone, Debug)]
pub struct EnumTypeRef(SUERef, NodeInfo);


#[derive(Clone, Debug)]
pub struct CompType(SUERef, CompTyKind, Vec<MemberDecl>, Attributes, NodeInfo);


pub fn typeOfCompDef(CompType(__ref, tag, _, _, _): CompType) -> TypeName {
    TyComp((CompTypeRef(__ref, tag, undefNode)))
}

#[derive(Clone, Debug, Eq, Ord)]
pub enum CompTyKind {
    StructTag,
    UnionTag
}
pub use self::CompTyKind::*;

#[derive(Clone, Debug)]
pub struct EnumType(SUERef, Vec<Enumerator>, Attributes, NodeInfo);


pub fn typeOfEnumDef(EnumType(__ref, _, _, _): EnumType) -> TypeName {
    TyEnum((EnumTypeRef(__ref, undefNode)))
}

#[derive(Clone, Debug)]
pub struct Enumerator(Ident, Expr, EnumType, NodeInfo);


#[derive(Clone, Debug)]
pub struct TypeQuals{
    constant: bool,
    volatile: bool,
    restrict: bool,
    atomic: bool,
    nullable: bool,
    nonnull: bool
}
fn constant(a: TypeQuals) -> bool { a.constant }
fn volatile(a: TypeQuals) -> bool { a.volatile }
fn restrict(a: TypeQuals) -> bool { a.restrict }
fn atomic(a: TypeQuals) -> bool { a.atomic }
fn nullable(a: TypeQuals) -> bool { a.nullable }
fn nonnull(a: TypeQuals) -> bool { a.nonnull }

pub fn noTypeQuals() -> TypeQuals {
    TypeQuals(false, false, false, false, false, false)
}

pub fn mergeTypeQuals(TypeQuals(c1, v1, r1, a1, n1, nn1): TypeQuals, TypeQuals(c2, v2, r2, a2, n2, nn2): TypeQuals) -> TypeQuals {
    TypeQuals(((c1 && c2)), ((v1 && v2)), ((r1 && r2)), ((a1 && a2)), ((n1 && n2)), ((nn1 && nn2)))
}

pub type Initializer = CInit;

#[derive(Clone, Debug)]
pub enum VarName {
    VarName(Ident, Option<AsmName>),
    NoName
}
pub use self::VarName::*;

pub fn identOfVarName(_0: VarName) -> Ident {
    match (_0) {
        NoName => {
            __error!("identOfVarName: NoName".to_string())
        },
        VarName(ident, _) => {
            ident
        },
    }
}

pub fn isNoName(_0: VarName) -> bool {
    match (_0) {
        NoName => {
            true
        },
        _ => {
            false
        },
    }
}

pub type AsmBlock = CStrLit;

pub type AsmName = CStrLit;

#[derive(Clone, Debug)]
pub struct Attr(Ident, Vec<Expr>, NodeInfo);


pub type Attributes = Vec<Attr>;

pub fn noAttributes() -> Attributes {
    vec![]
}

pub fn mergeAttributes() -> Attributes {
    (__op_addadd)
}

pub type Stmt = CStat;

pub type Expr = CExpr;



