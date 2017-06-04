// Original file: "Position.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Generics;

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

pub type PosLength = (Position, isize);

pub fn adjustPos(_0: FilePath, _1: isize, _2: Position) -> Position {
    match (_0, _1, _2) {
        (fname, row, Position(offs, _, _, _)) => {
            Position(offs, fname, row, 1)
        },
        (_, _, p) => {
            Position(offs, fname, row, 1)
        },
    }
}

pub fn builtinPos() -> Position {
    BuiltinPosition
}

pub fn incOffset(_0: Position, _1: isize) -> Position {
    match (_0, _1) {
        (Position(o, f, r, c), n) => {
            Position(((o + n)), f, r, c)
        },
        (p, _) => {
            Position(((o + n)), f, r, c)
        },
    }
}

pub fn incPos(_0: Position, _1: isize) -> Position {
    match (_0, _1) {
        (Position(offs, fname, row, col), n) => {
            Position(((offs + n)), fname, row, ((col + n)))
        },
        (p, _) => {
            Position(((offs + n)), fname, row, ((col + n)))
        },
    }
}

pub fn initPos(file: FilePath) -> Position {
    Position(0, file, 1, 1)
}

pub fn internalPos() -> Position {
    InternalPosition
}

pub fn isBuiltinPos(_0: Position) -> bool {
    match (_0) {
        BuiltinPosition => {
            true
        },
        _ => {
            true
        },
    }
}

pub fn isInternalPos(_0: Position) -> bool {
    match (_0) {
        InternalPosition => {
            true
        },
        _ => {
            true
        },
    }
}

pub fn isNoPos(_0: Position) -> bool {
    match (_0) {
        NoPosition => {
            true
        },
        _ => {
            true
        },
    }
}

pub fn isSourcePos(_0: Position) -> bool {
    match (_0) {
        Position(_, _, _, _) => {
            true
        },
        _ => {
            true
        },
    }
}

pub fn nopos() -> Position {
    NoPosition
}

pub fn position() -> Position {
    Position
}

pub fn retPos(_0: Position) -> Position {
    match (_0) {
        Position(offs, fname, row, _) => {
            Position(((offs + 1)), fname, ((row + 1)), 1)
        },
        p => {
            Position(((offs + 1)), fname, ((row + 1)), 1)
        },
    }
}



