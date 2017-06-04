// Original file: "Idiomatic.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::Rust::AST;

pub fn itemIdioms(_0: Rust::Item) -> Rust::Item {
    match (_0) {
        _0 => {
            Rust::Item(attrs, vis, (Rust::Function(fattrs, name, formals, ret, (tailBlock(b)))))
        },
        _0 => {
            Rust::Item(attrs, vis, (Rust::Function(fattrs, name, formals, ret, (tailBlock(b)))))
        },
    }
}

pub fn tailBlock(_0: Rust::Block) -> Rust::Block {
    match (_0) {
        _0 => {
            Rust::Block(b, e)
        },
        _0 => {
            Rust::Block(b, e)
        },
        _0 => {
            Rust::Block(b, e)
        },
    }
}

pub fn tailExpr(_0: Rust::Expr) -> Option<Option<Rust::Expr>> {
    match (_0) {
        _0 => {
            Some(e)
        },
        _0 => {
            Some(e)
        },
        _0 => {
            Some(e)
        },
        _0 => {
            Some(e)
        },
    }
}

pub fn unsnoc<a>(_0: Vec<a>) -> Option<(Vec<a>, a)> {
    match (_0) {
        _0 => {
            None
        },
        _0 => {
            None
        },
    }
}



