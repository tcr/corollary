// Original file: "Utils.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::List;
// use Language::C::Data::Ident;
// use Language::C::Syntax::AST;

pub fn compoundSubStmts(_0: CBlockItem) -> Vec<CStat> {
    match (_0) {
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
    }
}

pub fn getLabels(_0: CStat) -> Vec<Ident> {
    match (_0) {
        _0 => {
            __op_concat(l, getLabels(s))
        },
        _0 => {
            __op_concat(l, getLabels(s))
        },
        _0 => {
            __op_concat(l, getLabels(s))
        },
    }
}

pub fn getSubStmts(_0: CStat) -> Vec<CStat> {
    match (_0) {
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
        _0 => {
            vec![s]
        },
    }
}

pub fn mapBlockItemStmts(_0: fn(CStat) -> bool, _1: fn(CStat) -> CStat, _2: CBlockItem) -> CBlockItem {
    match (_0, _1, _2) {
        (_0, _1, _2) => {
            CBlockStmt((mapSubStmts(stop, f, s)))
        },
        (_0, _1, _2) => {
            CBlockStmt((mapSubStmts(stop, f, s)))
        },
    }
}



