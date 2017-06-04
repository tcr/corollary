// Original file: "AST.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Char;
// use Numeric;
// use Text::PrettyPrint::HughesPJClass;

#[derive(Debug, Eq)]
pub struct Lifetime(String);


#[derive(Debug, Eq)]
pub struct Type(String);


#[derive(Debug, Eq)]
pub enum LitIntRepr {
    DecRepr,
    OctalRepr,
    HexRepr
}
pub use self::LitIntRepr::*;

#[derive(Debug, Eq)]
pub enum Lit {
    LitByteStr(String),
    LitByteChar(Char),
    LitBool(bool),
    LitInt(Integer, LitIntRepr, Type),
    LitFloat(String, Type)
}
pub use self::Lit::*;

#[derive(Debug, Eq)]
pub struct Var(String);


#[derive(Debug)]
pub struct Path(Vec<String>);


#[derive(Debug, Eq)]
pub enum Visibility {
    Public,
    Private
}
pub use self::Visibility::*;

#[derive(Debug, Eq)]
pub enum Mutable {
    Immutable,
    Mutable
}
pub use self::Mutable::*;

#[derive(Debug)]
pub enum Stmt {
    Stmt(Expr),
    Let(Mutable, Var, Option<Type>, Option<Expr>),
    StmtItem(Vec<Attribute>, ItemKind)
}
pub use self::Stmt::*;

#[derive(Debug)]
pub struct Block(Vec<Stmt>, Option<Expr>);


#[derive(Debug)]
pub struct Attribute(String);


#[derive(Debug)]
pub struct Item(Vec<Attribute>, Visibility, ItemKind);


#[derive(Debug)]
pub enum FunctionAttribute {
    UnsafeFn,
    ExternABI(Option<String>)
}
pub use self::FunctionAttribute::*;

#[derive(Debug)]
pub enum ItemKind {
    Function(Vec<FunctionAttribute>, String, Vec<(Mutable, Var, Type)>, Type, Block),
    Static(Mutable, Var, Type, Expr),
    Struct(String, Vec<(String, Type)>),
    Extern(Vec<ExternItem>),
    Use(String),
    Enum(String, Vec<Enumerator>),
    CloneImpl(Type)
}
pub use self::ItemKind::*;

#[derive(Debug)]
pub enum ExternItem {
    ExternFn(String, Vec<(Var, Type)>, bool, Type),
    ExternStatic(Mutable, Var, Type)
}
pub use self::ExternItem::*;

#[derive(Debug)]
pub enum Enumerator {
    EnumeratorAuto(String),
    EnumeratorExpr(String, Expr)
}
pub use self::Enumerator::*;

#[derive(Debug)]
pub enum Expr {
    Lit(Lit),
    Var(Var),
    Path(Path),
    Index(Expr, Expr),
    ArrayExpr(Vec<Expr>),
    RepeatArray(Expr, Expr),
    StructExpr(String, Vec<(String, Expr)>, Option<Expr>),
    Call(Expr, Vec<Expr>),
    MethodCall(Expr, Var, Vec<Expr>),
    Lambda(Vec<Var>, Expr),
    Member(Expr, Var),
    BlockExpr(Block),
    UnsafeExpr(Block),
    IfThenElse(Expr, Block, Block),
    Loop(Option<Lifetime>, Block),
    While(Option<Lifetime>, Expr, Block),
    For(Option<Lifetime>, Var, Expr, Block),
    Break(Option<Lifetime>),
    Continue(Option<Lifetime>),
    Return(Option<Expr>),
    Neg(Expr),
    Deref(Expr),
    Not(Expr),
    Borrow(Mutable, Expr),
    Cast(Expr, Type),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Mod(Expr, Expr),
    Add(Expr, Expr),
    Sub(Expr, Expr),
    ShiftL(Expr, Expr),
    ShiftR(Expr, Expr),
    And(Expr, Expr),
    Xor(Expr, Expr),
    Or(Expr, Expr),
    CmpLT(Expr, Expr),
    CmpGT(Expr, Expr),
    CmpLE(Expr, Expr),
    CmpGE(Expr, Expr),
    CmpEQ(Expr, Expr),
    CmpNE(Expr, Expr),
    LAnd(Expr, Expr),
    LOr(Expr, Expr),
    Range(Expr, Expr),
    Assign(Expr, AssignOp, Expr)
}
pub use self::Expr::*;

#[derive(Debug)]
pub enum AssignOp {
    __id_3a3d,
    __id_3a2b3d,
    __id_3a2d3d,
    __id_3a2a3d,
    __id_3a2f3d,
    __id_3a253d,
    __id_3a263d,
    __id_3a7c3d,
    __id_3a5e3d,
    __id_3a3c3c3d,
    __id_3a3e3e3d
}
pub use self::AssignOp::*;

#[derive(Debug)]
pub enum ExprPosition {
    TopExpr,
    LeftExpr,
    RightExpr
}
pub use self::ExprPosition::*;

pub fn pPrintBlock(_0: Doc, _1: Block) -> Doc {
    match (_0, _1) {
        (_0, _1) => {
            sep(vec![
                    __op_doc_conat(pre, text("{".to_string())),
                    nest(4, (maybe(empty, pPrint, e))),
                    text("}".to_string()),
                ])
        },
        (_0, _1) => {
            sep(vec![
                    __op_doc_conat(pre, text("{".to_string())),
                    nest(4, (maybe(empty, pPrint, e))),
                    text("}".to_string()),
                ])
        },
    }
}



