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

#[derive(Clone, Debug)]
pub enum IdentDecl {
    Declaration(Decl),
    ObjectDef(ObjDef),
    FunctionDef(FunDef),
    EnumeratorDef(Enumerator)
}
pub use self::IdentDecl::*;

pub struct GlobalDecls{
    gObjs: Map<Ident, IdentDecl>,
    gTags: Map<SUERef, TagDef>,
    gTypeDefs: Map<Ident, TypeDef>
}
fn gObjs(a: GlobalDecls) -> Map<Ident, IdentDecl> { a.gObjs }
fn gTags(a: GlobalDecls) -> Map<SUERef, TagDef> { a.gTags }
fn gTypeDefs(a: GlobalDecls) -> Map<Ident, TypeDef> { a.gTypeDefs }

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


#[derive(Clone, Debug)]
pub struct VarDecl(VarName, DeclAttrs, Type);


#[derive(Clone, Debug)]
pub struct DeclAttrs(FunctionAttrs, Storage, Attributes);


#[derive(Clone, Debug, Eq, Ord)]
pub struct FunctionAttrs{
    isInline: bool,
    isNoreturn: bool
}
fn isInline(a: FunctionAttrs) -> bool { a.isInline }
fn isNoreturn(a: FunctionAttrs) -> bool { a.isNoreturn }

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


#[derive(Clone, Debug, Eq, Ord)]
pub enum CompTyKind {
    StructTag,
    UnionTag
}
pub use self::CompTyKind::*;

#[derive(Clone, Debug)]
pub struct EnumType(SUERef, Vec<Enumerator>, Attributes, NodeInfo);


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

pub type Initializer = CInit;

#[derive(Clone, Debug)]
pub enum VarName {
    VarName(Ident, Option<AsmName>),
    NoName
}
pub use self::VarName::*;

pub type AsmBlock = CStrLit;

pub type AsmName = CStrLit;

#[derive(Clone, Debug)]
pub struct Attr(Ident, Vec<Expr>, NodeInfo);


pub type Attributes = Vec<Attr>;

pub type Stmt = CStat;

pub type Expr = CExpr;

pub fn declAttrs() -> DeclAttrs {
    (|VarDecl(_, specs, _)| { specs }(getVarDecl))
}

pub fn declIdent() -> Ident {
    identOfVarName(declName)
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

pub fn declName() -> VarName {
    (|VarDecl(n, _, _)| { n }(getVarDecl))
}

pub fn declOfDef(def: n) -> Decl {
    {
        let vd = getVarDecl(def);

    Decl(vd, (nodeInfo(def)))    }
}

pub fn declStorage(d: d) -> Storage {
    match declAttrs(d) {
        DeclAttrs(_, st, _) => {
            st
        },
    }
}

pub fn declType() -> Type {
    (|VarDecl(_, _, ty)| { ty }(getVarDecl))
}

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

pub fn functionAttrs(d: d) -> FunctionAttrs {
    match declAttrs(d) {
        DeclAttrs(fa, _, _) => {
            fa
        },
    }
}

pub fn hasLinkage(_0: Storage) -> bool {
    match (_0) {
        _0 => {
            false
        },
        _0 => {
            false
        },
        _0 => {
            false
        },
    }
}

pub fn identOfTypeDef(TypeDef(ide, _, _, _): TypeDef) -> Ident {
    ide
}

pub fn identOfVarName(_0: VarName) -> Ident {
    match (_0) {
        _0 => {
            __error!("identOfVarName: NoName".to_string())
        },
        _0 => {
            __error!("identOfVarName: NoName".to_string())
        },
    }
}

pub fn isExtDecl() -> bool {
    hasLinkage(declStorage)
}

pub fn isNoName(_0: VarName) -> bool {
    match (_0) {
        _0 => {
            true
        },
        _0 => {
            true
        },
    }
}

pub fn mergeAttributes() -> Attributes {
    (__op_addadd)
}

pub fn mergeGlobalDecls(gmap1: GlobalDecls, gmap2: GlobalDecls) -> GlobalDecls {
    GlobalDecls {
        gObjs: Map::union((gObjs(gmap1)), (gObjs(gmap2))),
        gTags: Map::union((gTags(gmap1)), (gTags(gmap2))),
        gTypeDefs: Map::union((gTypeDefs(gmap1)), (gTypeDefs(gmap2)))
    }
}

pub fn mergeTypeQuals(TypeQuals(c1, v1, r1, a1, n1, nn1): TypeQuals, TypeQuals(c2, v2, r2, a2, n2, nn2): TypeQuals) -> TypeQuals {
    TypeQuals(((c1 && c2)), ((v1 && v2)), ((r1 && r2)), ((a1 && a2)), ((n1 && n2)), ((nn1 && nn2)))
}

pub fn noAttributes() -> Attributes {
    vec![]
}

pub fn noFunctionAttrs() -> FunctionAttrs {
    FunctionAttrs {
        isInline: false,
        isNoreturn: false
    }
}

pub fn noTypeQuals() -> TypeQuals {
    TypeQuals(false, false, false, false, false, false)
}

pub fn objKindDescr(_0: IdentDecl) -> String {
    match (_0) {
        _0 => {
            "declaration".to_string()
        },
        _0 => {
            "declaration".to_string()
        },
        _0 => {
            "declaration".to_string()
        },
        _0 => {
            "declaration".to_string()
        },
    }
}

pub fn splitIdentDecls(include_all: bool) -> (Map<Ident, Decl>, (Map<Ident, Enumerator>, Map<Ident, ObjDef>, Map<Ident, FunDef>)) {
    Map::foldWithKey((if include_all {         
deal} else {
deal_q
        }), (Map::empty, (Map::empty, Map::empty, Map::empty)))
}

pub fn typeOfCompDef(CompType(__ref, tag, _, _, _): CompType) -> TypeName {
    TyComp((CompTypeRef(__ref, tag, undefNode)))
}

pub fn typeOfEnumDef(EnumType(__ref, _, _, _): EnumType) -> TypeName {
    TyEnum((EnumTypeRef(__ref, undefNode)))
}

pub fn typeOfTagDef(_0: TagDef) -> TypeName {
    match (_0) {
        _0 => {
            typeOfCompDef(comptype)
        },
        _0 => {
            typeOfCompDef(comptype)
        },
    }
}



