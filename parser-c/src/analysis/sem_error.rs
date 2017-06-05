// Original file: "SemError.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Typeable;
// use Language::C::Analysis::SemRep;
// use Language::C::Data::Error;
// use Language::C::Data::Node;

#[derive(Debug)]
pub struct InvalidASTError(pub ErrorInfo);


#[derive(Debug)]
pub struct BadSpecifierError(pub ErrorInfo);


#[derive(Debug)]
pub struct RedefError(pub ErrorLevel, pub RedefInfo);


pub struct RedefInfo(pub String, pub RedefKind, pub NodeInfo, pub NodeInfo);


pub enum RedefKind {
    DuplicateDef,
    DiffKindRedecl,
    ShadowedDef,
    DisagreeLinkage,
    NoLinkageOld
}
pub use self::RedefKind::*;

#[derive(Debug)]
pub struct TypeMismatch(pub String, pub (NodeInfo, pub Type), pub (NodeInfo, pub Type));


pub fn invalidAST(node_info: NodeInfo, msg: String) -> InvalidASTError {
    InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
}

pub fn badSpecifierError(node_info: NodeInfo, msg: String) -> BadSpecifierError {
    BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
}

pub fn typeMismatch() -> TypeMismatch {
    TypeMismatch
}

pub fn typeMismatchInfo(TypeMismatch(reason, (node1, _ty2), _t2): TypeMismatch) -> ErrorInfo {
    ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
}

pub fn redefErrLabel(RedefInfo(ident, _, _, _): RedefInfo) -> String {
    __op_addadd(ident, " redefined".to_string())
}

pub fn redefErrorInfo(lvl: ErrorLevel, info: RedefInfo, __OP__: ErrorInfo) -> ErrorInfo {
    ErrorInfo(lvl, (posOfNode(node)), (__op_addadd(vec![redefErrReason(info)], prevDeclMsg(old_node))))
}

pub fn redefErrReason(_0: RedefInfo) -> String {
    match (_0) {
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

pub fn prevDeclMsg(old_node: NodeInfo) -> Vec<String> {
    vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
}

pub fn redefinition(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
    RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
}



