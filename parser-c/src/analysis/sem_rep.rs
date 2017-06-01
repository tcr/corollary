mod haskell_support {
    pub trait Addable {
        fn add(self, right: Self) -> Self;
    }

    impl Addable for String {
        fn add(self, right: Self) -> Self {
            format!("{}{}", self, right)
        }
    }

    pub fn __op_addadd<A: Addable>(left: A, right: A) -> A {
        Addable::add(left, right)
    }
}


pub mod Language_C_Analysis_SemRep {
    use haskell_support::*;

use Language::C::Data;
use Language::C::Syntax;
use Language::C::Syntax::Constants;
use Data::Map;
use Map;
use Data::Map;
use Data::Generics;
use Text::PrettyPrint::HughesPJ;

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

    struct GlobalDecls{
        gObjs: Map<Ident, IdentDecl>,
        gTags: Map<SUERef, TagDef>,
        gTypeDefs: Map<Ident, TypeDef>
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
    struct Decl(VarDecl, NodeInfo);

    #[derive(Clone, Debug)]
    struct ObjDef(VarDecl, Option<Initializer>, NodeInfo);

    #[derive(Clone, Debug)]
    struct FunDef(VarDecl, Stmt, NodeInfo);

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
    struct TypeDef(Ident, Type, Attributes, NodeInfo);

    #[derive(Clone, Debug)]
    struct VarDecl(VarName, DeclAttrs, Type);

    #[derive(Clone, Debug)]
    struct DeclAttrs(bool, Storage, Attributes);

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum Storage {
        NoStorage,
        Auto(Register),
        Static(Linkage, ThreadLocal),
        FunLinkage(Linkage)
    }
    pub use self::Storage::*;

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
    struct TypeDefRef(Ident, Option<Type>, NodeInfo);

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
    struct CompTypeRef(SUERef, CompTyKind, NodeInfo);

    #[derive(Clone, Debug)]
    struct EnumTypeRef(SUERef, NodeInfo);

    #[derive(Clone, Debug)]
    struct CompType(SUERef, CompTyKind, Vec<MemberDecl>, Attributes, NodeInfo);

    #[derive(Clone, Debug, Eq, Ord)]
    pub enum CompTyKind {
        StructTag,
        UnionTag
    }
    pub use self::CompTyKind::*;

    #[derive(Clone, Debug)]
    struct EnumType(SUERef, Vec<Enumerator>, Attributes, NodeInfo);

    #[derive(Clone, Debug)]
    struct Enumerator(Ident, Expr, EnumType, NodeInfo);

    #[derive(Clone, Debug)]
    struct TypeQuals{
        constant: bool,
        volatile: bool,
        restrict: bool
    }

    #[derive(Clone, Debug)]
    pub enum VarName {
        VarName(Ident, Option<AsmName>),
        NoName
    }
    pub use self::VarName::*;

    #[derive(Clone, Debug)]
    struct Attr(Ident, Vec<Expr>, NodeInfo);

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

        Decl(vd, (nodeInfo(def)))        }
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

    pub fn hasLinkage(__0: Storage) -> bool {
        match (__0) {
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

    pub fn identOfTypeDef(TypeDef(ide, _, _, _): TypeDef) -> Ident {
        ide
    }

    pub fn identOfVarName(__0: VarName) -> Ident {
        match (__0) {
            NoName => {
                __error!("identOfVarName: NoName".to_string())
            },
            VarName(ident, _) => {
                ident
            },
        }
    }

    pub fn isExtDecl() -> bool {
        hasLinkage(declStorage)
    }

    pub fn isNoName(__0: VarName) -> bool {
        match (__0) {
            NoName => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn isTentative(ObjDef(decl, init_opt, _): ObjDef) -> bool {
        /* Expr::Error */ Error
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

    pub fn mergeTypeQuals(TypeQuals(c1, v1, r1): TypeQuals, TypeQuals(c2, v2, r2): TypeQuals) -> TypeQuals {
        TypeQuals(((c1 && c2)), ((v1 && v2)), ((r1 && r2)))
    }

    pub fn noAttributes() -> Attributes {
        vec![]
    }

    pub fn noTypeQuals() -> TypeQuals {
        TypeQuals(false, false, false)
    }

    pub fn objKindDescr(__0: IdentDecl) -> String {
        match (__0) {
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
        Map::foldWithKey((__TODO_if(include_all, then, deal, __TODO_else, deal_q)), (Map::empty, (Map::empty, Map::empty, Map::empty)))
    }

    pub fn typeOfCompDef(CompType(__ref, tag, _, _, _): CompType) -> TypeName {
        TyComp((CompTypeRef(__ref, tag, undefNode)))
    }

    pub fn typeOfEnumDef(EnumType(__ref, _, _, _): EnumType) -> TypeName {
        TyEnum((EnumTypeRef(__ref, undefNode)))
    }

    pub fn typeOfTagDef(__0: TagDef) -> TypeName {
        match (__0) {
            CompDef(comptype) => {
                typeOfCompDef(comptype)
            },
            EnumDef(enumtype) => {
                typeOfEnumDef(enumtype)
            },
        }
    }

}




