#[derive(Clone, Debug)]
pub enum Expr {
    Number(isize),
    Op(Box<Expr>, String, Box<Expr>),
    Ref(Ident),
    Do(Vec<DoStatement>),
    Parens(Vec<Expr>),
    Case(Box<Expr>, Vec<CaseCond>),
    /// `let` a = 2; b = 3 `in` ...
    Let(Vec<Assignment>, Box<Expr>),
    Span(Vec<Expr>),
    Vector(Vec<Expr>),
    Operator(String),
    Record(Vec<(Ident, Expr)>),
    Lambda(Vec<Pat>, Box<Expr>),
    Str(String),
    Char(String),
    Error,
    Dummy,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub pats: Vec<Pat>,
    pub expr: Expr,
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
pub enum Statement {
    Import,

    // Name, Inner Types, Deriving IDs
    Type,
    Data(Ident, Vec<Vec<Ty>>, Vec<Ident>, Option<Vec<Ty>>),
    Newtype(Ident, Ty, Vec<Ident>),
    Class,
    Instance,

    Prototype(Ident, Vec<Ty>),
    Assign(Box<Assignment>, Where),
    GuardAssign,

    // TODO remove this
    Dummy,
}

pub type Where = Vec<Statement>;

#[derive(Clone, Debug)]
pub enum DoStatement {
    Let(Vec<Assignment>),
    Expression(Box<Expr>),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: Ident,
    pub statements: Vec<Statement>,
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
    Dummy,
}

#[derive(Clone, Debug)]
pub enum Pat {
    Span(Vec<Pat>),
    Arrow(Ident, Box<Pat>),
    Not(Box<Pat>),
    Ref(Ident),
    Infix(Ident),
    Tuple(Vec<Pat>),
    Brackets(Vec<Pat>),
    Record(Vec<(Ident, Pat)>),
    Operator(String),
    Str(String),
    Char(String),
    Num(isize),
    Concat(Box<Pat>, Box<Pat>),
    EmptyParen,
    Dummy,
}

#[derive(Clone, Debug)]
pub struct Ident(pub String);

// maybe move this to a new mod
/// De-infixes a `Pat::Infix`.
pub fn rearrange_infix_pat(mut pats: Vec<Pat>) -> Vec<Pat> {
    let mut index = None;
    for (i, pat) in pats.iter().enumerate() {
        if match pat { &Pat::Infix(_) => true, _ => false } {
            assert!(index.is_none(), "Multiple infix patterns: {:?}", pats);
            index = Some(i);
        }
    }

    if let Some(i) = index {
        let ident = match pats.remove(i) {
            Pat::Infix(ident) => ident,
            _ => panic!(),
        };
        pats.insert(0, Pat::Ref(ident));
    }

    pats
}
