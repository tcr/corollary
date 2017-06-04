#[derive(Clone, Debug)]
pub enum Expr {
    Number(isize),
    Op(Box<Expr>, String, Box<Expr>),
    Ref(Ident),
    Do(Vec<DoItem>,  Where),
    Parens(Vec<Expr>),
    Case(Box<Expr>, Vec<CaseCond>),
    Generator(Vec<Expr>, Vec<()>), //TODO listgenerator body
    /// `let` a = 2; b = 3 `in` ...
    Let(Vec<Assignment>, Box<Expr>),
    Span(Vec<Expr>),
    Vector(Vec<Expr>),
    Operator(String),
    Record(Box<Expr>, Vec<(Ident, Expr)>),
    Lambda(Vec<Pat>, Box<Expr>),
    Str(String),
    Char(String),

    RecordArgs(Vec<(Ident, Expr)>), // Should be preprocessed out
    Error,
}

#[derive(Clone, Debug)]
pub enum Assignment {
    Assign {
        pats: Vec<Pat>,
        expr: Expr,
    },
    Case {
        pats: Vec<Pat>,
        sets: Vec<(Vec<(Expr, Option<Expr>)>, Expr)>,
    }
}

#[derive(Clone, Debug)]
pub enum CaseCond {
    Matching(Vec<Pat>, Vec<(Vec<Expr>, Expr)>),
    Direct(Vec<Pat>, Vec<Expr>),
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Clone, Debug)]
pub enum Item {
    Import(Vec<Vec<Ident>>),

    // Name, Inner Types, Deriving IDs, Type Parameters
    Data(Ident, Vec<Vec<Ty>>, Vec<Ident>, Vec<Ty>),
    // Name, Wrapped Type, Deriving IDs, Type Parameters
    Newtype(Ident, Ty, Vec<Ident>, Vec<Ty>),
    // Name, Wrapped Type, Type Parameters
    Type(Ident, Vec<Ty>, Vec<Ty>),
    Class,
    Instance,

    Prototype(Vec<Ident>, Vec<Ty>),
    Assign(Box<Assignment>, Where),
    GuardAssign,

    Infixr(isize, Ident),
    Infixl(isize, Ident),
}

pub type Where = Vec<Item>;

#[derive(Clone, Debug)]
pub enum DoItem {
    Let(Vec<Assignment>),
    Bind(Vec<Pat>, Box<Expr>),
    Expression(Box<Expr>),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: Ident,
    pub items: Where,
}

#[derive(Clone, Debug)]
pub enum Ty {
    Span(Vec<Ty>),
    Pair(Box<Ty>, Box<Ty>),
    Not(Box<Ty>),
    Ref(Ident),
    Tuple(Vec<Ty>),
    Brackets(Box<Ty>),
    Record(Vec<(Ident, Ty)>),
    EmptyParen,
    RangeOp,

    Dummy,
}

#[derive(Clone, Debug)]
pub enum Pat {
    Span(Vec<Pat>),
    ViewPattern(Ident, Box<Pat>),
    Not(Box<Pat>),
    Ref(Ident),
    Infix(Ident),
    Tuple(Vec<Pat>),
    Brackets(Vec<Pat>),
    Record(Ident, Vec<(Ident, Pat)>),
    Operator(String),
    Str(String),
    Char(String),
    Num(isize),
    Concat(Box<Pat>, Box<Pat>),
    EmptyParen,
}

#[derive(Clone, Debug)]
pub struct Ident(pub String);
