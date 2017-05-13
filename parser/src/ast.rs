#[derive(Clone, Debug)]
pub enum Expr {
    Number(isize),
    Op(Box<Expr>, String, Box<Expr>),
    Ref(Ident),
    Do(Vec<Expr>, Option<Vec<Statement>>),
    Parens(Vec<Expr>),
    Case(Box<Expr>, Vec<CaseCond>),
    Let,
    Span(Vec<Expr>),
    Vector(Vec<Expr>),
    Operator(String),
    Record(Vec<(Expr, Expr)>),
    Lambda,
    Str(String),
    Error,
    Dummy,
}

#[derive(Clone, Debug)]
pub enum CaseCond {
    Matching(Vec<Pat>, Vec<(Expr, Expr)>),
    Direct(Vec<Pat>, Expr),
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Data(Ident, Vec<Vec<Ty>>, Option<Vec<Ident>>),
    Dummy,
    Class,
    Instance,
    Import,
    Newtype,
    Pipelist,
    GuardAssign,
    Assign(Pat, Vec<Pat>, Expr),
    Typedef(Ident),
    Prototype(Ident, Vec<Ty>),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: Ident,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Ty {
    Span(Vec<Ty>),
    Where(Box<Ty>, Box<Ty>),
    Pair(Box<Ty>, Box<Ty>),
    Not(Box<Ty>),
    Ident(Ident),
    Tuple(Vec<Ty>),
    Brackets(Box<Ty>),
    EmptyParen,
}

#[derive(Clone, Debug)]
pub enum Pat {
    Span(Vec<Pat>),
    Arrow(Ident, Box<Pat>),
    Ref(Ident),
    Tuple(Vec<Pat>),
    Brackets(Vec<Pat>),
    Str(String),
    Num(isize),
    RecordTODO,
    EmptyParen,
    Dummy,
}

#[derive(Clone, Debug)]
pub struct Ident(pub String);
