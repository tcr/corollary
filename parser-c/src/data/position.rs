// Original file: "Position.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Generics;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Position {
    Position {
        posOffset: isize,
        posFile: String,
        posRow: isize,
        posColumn: isize,
    },
    NoPosition,
    BuiltinPosition,
    InternalPosition,
}
pub use self::Position::{NoPosition, BuiltinPosition, InternalPosition};

impl ::std::fmt::Display for Position {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn posOffset(p: Position) -> isize {
    if let Position::Position { posOffset, .. } = p {
        posOffset
    } else {
        panic!("Non Position::Position passed to posOffset")
    }
}

pub fn posFile(p: Position) -> String {
    if let Position::Position { posFile, .. } = p {
        posFile
    } else {
        panic!("Non Position::Position passed to posFile")
    }
}

pub fn posRow(p: Position) -> isize {
    if let Position::Position { posRow, .. } = p {
        posRow
    } else {
        panic!("Non Position::Position passed to posRow")
    }
}

pub fn posColumn(p: Position) -> isize {
    if let Position::Position { posColumn, .. } = p {
        posColumn
    } else {
        panic!("Non Position::Position passed to posColumn")
    }
}

pub type PosLength = (Position, isize);

pub fn position(posOffset: isize, posFile: String, posRow: isize, posColumn: isize) -> Position {
    Position::Position { posOffset, posFile, posRow, posColumn }
}

// class of type which aggregate a source code location
pub trait Pos {
    fn posOf(self) -> Position;
}
pub fn posOf<P: Pos>(input: P) -> Position {
    input.posOf()
}

pub fn initPos(file: FilePath) -> Position {
    position(0, file.into(), 1, 1)
}

pub fn isSourcePos(_0: Position) -> bool {
    match (_0) {
        Position::Position { .. } => true,
        _ => false,
    }
}

pub fn nopos() -> Position {
    NoPosition
}

pub fn isNoPos(_0: Position) -> bool {
    match (_0) {
        NoPosition => true,
        _ => false,
    }
}

pub fn builtinPos() -> Position {
    BuiltinPosition
}

pub fn isBuiltinPos(_0: Position) -> bool {
    match (_0) {
        BuiltinPosition => true,
        _ => false,
    }
}

pub fn internalPos() -> Position {
    InternalPosition
}

pub fn isInternalPos(_0: Position) -> bool {
    match (_0) {
        InternalPosition => true,
        _ => false,
    }
}

pub fn incPos(_0: Position, _1: isize) -> Position {
    match (_0, _1) {
        (Position::Position {posOffset: offs, posFile: fname, posRow: row, posColumn: col }, n) => position(((offs + n)), fname, row, ((col + n))),
        (p, _) => p,
    }
}

pub fn retPos(_0: Position) -> Position {
    match (_0) {
        Position::Position {posOffset: offs, posFile: fname, posRow: row, ..}  => position(((offs + 1)), fname, ((row + 1)), 1),
        p => p,
    }
}

pub fn adjustPos(_0: FilePath, _1: isize, _2: Position) -> Position {
    match (_0, _1, _2) {
        (fname, row, Position::Position {posOffset: offs, .. }) => position(offs, fname.into(), row, 1),
        (_, _, p) => p,
    }
}

pub fn incOffset(_0: Position, _1: isize) -> Position {
    match (_0, _1) {
        (Position::Position {
            posOffset: o,
            posFile: f, 
            posRow: r,
            posColumn: c
        }, n) => position(((o + n)), f, r, c),
        (p, _) => p,
    }
}
