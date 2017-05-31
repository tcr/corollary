use haskell_support::*;

use Data::Generics;

#[derive(Clone, Debug, Eq, Ord)]
pub enum Position {
    Position{
        posOffset: isize,
        posFile: String,
        posRow: isize,
        posColumn: isize
    },
    NoPosition,
    BuiltinPosition,
    InternalPosition
}
pub use self::Position::*;

pub fn adjustPos(__0: FilePath, __1: isize, __2: Position) -> Position {
    match (__0, __1, __2) {
        (fname, row, Position(offs, _, _, _)) => {
            Position(offs, fname, row, 1)
        },
        (_, _, p) => {
            p
        },
    }
}

pub fn builtinPos() -> Position {
    BuiltinPosition
}

pub fn incOffset(__0: Position, __1: isize) -> Position {
    match (__0, __1) {
        (Position(o, f, r, c), n) => {
            Position(((o + n)), f, r, c)
        },
        (p, n) => {
            p
        },
    }
}

pub fn incPos(__0: Position, __1: isize) -> Position {
    match (__0, __1) {
        (Position(offs, fname, row, col), n) => {
            Position(((offs + n)), fname, row, ((col + n)))
        },
        (p, _) => {
            p
        },
    }
}

pub fn initPos(file: FilePath) -> Position {
    Position(0, file, 1, 1)
}

pub fn internalPos() -> Position {
    InternalPosition
}

pub fn isBuiltinPos(__0: Position) -> bool {
    match (__0) {
        BuiltinPosition => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isInternalPos(__0: Position) -> bool {
    match (__0) {
        InternalPosition => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isNoPos(__0: Position) -> bool {
    match (__0) {
        NoPosition => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isSourcePos(__0: Position) -> bool {
    match (__0) {
        Position(_, _, _, _) => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn nopos() -> Position {
    NoPosition
}

pub fn position() -> Position {
    Position
}

pub fn retPos(__0: Position) -> Position {
    match (__0) {
        Position(offs, fname, row, _) => {
            Position(((offs + 1)), fname, ((row + 1)), 1)
        },
        p => {
            p
        },
    }
}

