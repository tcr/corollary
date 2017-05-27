//! Output structures.

use std::cmp;
use std::fmt::{self, Display, Formatter};

pub enum Expr {
    Free(String),
    Number(isize),
    StrLiteral(String),
    VecLiteral { exprs: Vec<Expr>, line_length: LineLength },
}

impl Out for Expr {
    fn fmt(&self, f: &mut Formatter, state: PrintState) -> fmt::Result {
        use self::Expr::*;

        match *self {
            Free(ref s) => f.write_str(s),
            Number(n) => write!(f, "{}", n),
            StrLiteral(ref s) => write!(f, "{:?}.to_string()", s),
            VecLiteral { ref exprs, line_length } => {
                if state.fits_on_line(line_length) {
                    write!(f, "vec![")?;
                    commas(f, state, exprs)?;
                    write!(f, "]")
                } else {
                    writeln!(f, "vec![")?;
                    comma_lines(f, state.tab(), exprs)?;
                    write!(f, "{}]", state.indent())
                }
            }
        }
    }
}

impl Expr {
    /// Approximates this expr's length on one line.
    /// Returns None if the expr is multiline.
    pub fn line_length(&self) -> LineLength {
        use self::Expr::*;
        match *self {
            Free(ref s) => if s.contains("\n") { None } else { Some(s.len()) },
            Number(_) => Some(2),
            StrLiteral(ref s) => Some(s.len() + 3),
            VecLiteral { line_length, .. } => line_length,
        }
    }
}

// HELPERS

/// Output length in `Char`s.
/// If output uses multiple lines, use `None`.
pub type LineLength = Option<usize>;

#[derive(Clone, Copy)]
pub struct PrintState {
    pub level: i32,
}

impl PrintState {
    pub fn new() -> PrintState {
        PrintState {
            level: 0,
        }
    }

    pub fn tab(&self) -> PrintState {
        PrintState {
            level: self.level + 1
        }
    }

    pub fn untab(&self) -> PrintState {
        PrintState {
            level: if self.level == 0 { 0 } else { self.level - 1 }
        }
    }

    pub fn indent(&self) -> String {
        let mut out = String::new();
        for _ in 0..self.level {
            out.push_str("    ");
        }
        out
    }

    pub fn fits_on_line(&self, len: LineLength) -> bool {
        len.map(|n| {
            let indent = self.level * 4;
            let limit = 100 - cmp::min(indent, 80);
            n <= limit as usize
        }).unwrap_or(false)
    }
}

/// Output-able as a Rust expression.
pub trait Out {
    fn fmt(&self, f: &mut Formatter, state: PrintState) -> fmt::Result;
}

impl<'a, O: Out> Out for &'a O {
    fn fmt(&self, f: &mut Formatter, state: PrintState) -> fmt::Result {
        (*self).fmt(f, state)
    }
}

/// Use this to display `Out` types.
pub struct Printer<O> {
    pub out: O,
    pub state: PrintState,
}

impl<O: Out> Display for Printer<O> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.out.fmt(f, self.state)
    }
}

/// Output items joined by commas.
fn commas<I, O>(f: &mut Formatter, state: PrintState, iter: I) -> fmt::Result
where
    I: IntoIterator<Item=O>,
    O: Out,
{
    let mut first = true;
    for o in iter {
        if first {
            first = false;
        } else {
            write!(f, ", ")?;
        }
        o.fmt(f, state)?;
    }
    Ok(())
}

/// Output one line per item, indented and followed by a comma.
fn comma_lines<I, O>(f: &mut Formatter, state: PrintState, iter: I) -> fmt::Result
where
    I: IntoIterator<Item=O>,
    O: Out,
{
    for o in iter {
        write!(f, "{}", state.indent())?;
        o.fmt(f, state)?;
        writeln!(f, ",")?;
    }
    Ok(())
}
