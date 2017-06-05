// Original file: "Idiomatic.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::Rust::AST;

pub fn unsnoc<a>(_0: Vec<a>) -> Option<(Vec<a>, a)> {
    match (_0) {
        [] => {
            None
        },
        [x, xs] => {
            None
        },
    }
}

pub fn tailExpr(_0: Rust::Expr) -> Option<Option<Rust::Expr>> {
    match (_0) {
        Rust::Return(e) => {
            Some(e)
        },
        Rust::BlockExpr(b) => {
            Some(e)
        },
        Rust::IfThenElse(c, t, f) => {
            Some(e)
        },
        _ => {
            Some(e)
        },
    }
}

pub fn tailBlock(_0: Rust::Block) -> Rust::Block {
    match (_0) {
        Rust::Block(b, Some(/* TODO ViewPattern */ tailExpr)) => {
            Rust::Block(b, e)
        },
        Rust::Block(/* TODO ViewPattern */ unsnoc, None) => {
            Rust::Block(b, e)
        },
        b => {
            Rust::Block(b, e)
        },
    }
}

pub fn itemIdioms(_0: Rust::Item) -> Rust::Item {
    match (_0) {
        Rust::Item(attrs, vis, Rust::Function(fattrs, name, formals, ret, b)) => {
            Rust::Item(attrs, vis, (Rust::Function(fattrs, name, formals, ret, (tailBlock(b)))))
        },
        i => {
            Rust::Item(attrs, vis, (Rust::Function(fattrs, name, formals, ret, (tailBlock(b)))))
        },
    }
}



