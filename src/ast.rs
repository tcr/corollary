use std::fmt::{Debug, Formatter, Error};

#[derive(Clone)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Ref(Ident),
    Do(Vec<Vec<Expr>>, Option<Vec<Statement>>),
    Parens(Vec<Expr>),
    Error,
    Dummy,
}

#[derive(Copy, Clone)]
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
    Assign(Ident, Vec<Expr>),
    Typedef(Ident),
    Prototype(Ident, Vec<Ty>),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Ty {
    Dummy,
}

#[derive(Clone, Debug)]
pub struct Ident(pub String);

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Ref(ref i) => write!(fmt, "{:?}", i),
            Do(ref e, ref w) => {
                for item in e {
                    writeln!(fmt, "        {:?}", e);
                }
                writeln!(fmt, "where");
                if let &Some(ref w) = w {
                    for item in w {
                        writeln!(fmt, "        {:?}", w);
                    }
                }
                Ok(())
            }
            Parens(ref e) => {
                for item in e {
                    writeln!(fmt, "        {:?}", e);
                }
                Ok(())
            }
            Error => write!(fmt, "error"),
            _ => write!(fmt, "..."),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
