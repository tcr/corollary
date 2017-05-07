use std::fmt::{Debug, Formatter, Error};

#[derive(Clone, Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Ref(Ident),
    Do(Vec<Expr>, Option<Vec<Statement>>),
    Parens(Vec<Expr>),
    Case,
    Let,
    Span(Vec<Expr>),
    Vector(Vec<Expr>),
    Operator(String),
    Record(Vec<(Ident, Expr)>),
    Lambda,
    Str(String),
    Error,
    Dummy,
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
    Data(Ident, Vec<Ident>, Vec<Ident>),
    Dummy,
    Class,
    Instance,
    Import,
    Newtype,
    Pipelist,
    GuardAssign,
    Assign(Ident, Vec<Ident>, Expr),
    Typedef(Ident),
    Prototype(Ident, Vec<Ty>),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Ty {
    Span(Vec<Ty>),
    Where(Box<Ty>, Box<Ty>),
    Pair(Box<Ty>, Box<Ty>),
    Not(Box<Ty>),
    Ref(Ident),
    Tuple(Vec<Ty>),
    Brackets(Vec<Ty>),
    EmptyParen,
    Dummy,
}

#[derive(Clone, Debug)]
pub struct Ident(pub String);
