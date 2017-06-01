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


pub mod Language_C_Analysis_SemError {
    use haskell_support::*;

use Data::Typeable;
use Language::C::Analysis::SemRep;
use Language::C::Data::Error;
use Language::C::Data::Node;

    #[derive(Debug)]
    struct RedefError(ErrorLevel, RedefInfo);

    struct RedefInfo(String, RedefKind, NodeInfo, NodeInfo);

    pub enum RedefKind {
        DuplicateDef,
        DiffKindRedecl,
        ShadowedDef,
        DisagreeLinkage,
        NoLinkageOld
    }
    pub use self::RedefKind::*;

    #[derive(Debug)]
    struct TypeMismatch(String, (NodeInfo, Type), (NodeInfo, Type));

    pub fn badSpecifierError(node_info: NodeInfo, msg: String) -> BadSpecifierError {
        BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
    }

    pub fn invalidAST(node_info: NodeInfo, msg: String) -> InvalidASTError {
        InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
    }

    pub fn prevDeclMsg(old_node: NodeInfo) -> Vec<String> {
        vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
    }

    pub fn redefErrLabel(RedefInfo(ident, _, _, _): RedefInfo) -> String {
        __op_addadd(ident, " redefined".to_string())
    }

    pub fn redefErrReason(__0: RedefInfo) -> String {
        match (__0) {
            RedefInfo(ident, DuplicateDef, _, _) => {
                __op_addadd("duplicate definition of ".to_string(), ident)
            },
            RedefInfo(ident, ShadowedDef, _, _) => {
                __op_addadd("this declaration of ".to_string(), __op_addadd(ident, " shadows a previous one".to_string()))
            },
            RedefInfo(ident, DiffKindRedecl, _, _) => {
                __op_addadd(ident, " previously declared as a different kind of symbol".to_string())
            },
            RedefInfo(ident, DisagreeLinkage, _, _) => {
                __op_addadd(ident, " previously declared with different linkage".to_string())
            },
            RedefInfo(ident, NoLinkageOld, _, _) => {
                __op_addadd(ident, " previously declared without linkage".to_string())
            },
        }
    }

    pub fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, __OP__: ErrorInfo) -> ErrorInfo {
        ErrorInfo(lvl, (posOfNode(node)), (__op_addadd(vec![redefErrReason(info)], prevDeclMsg(old_node))))
    }

    pub fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
        RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
    }

    pub fn typeMismatch() -> TypeMismatch {
        TypeMismatch
    }

    pub fn typeMismatchInfo(TypeMismatch(reason, (node1, _ty2), _t2): TypeMismatch) -> ErrorInfo {
        ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
    }

}




