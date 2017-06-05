// Original file: "Utils.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::List;
// use Language::C::Data::Ident;
// use Language::C::Syntax::AST;

use syntax::ast::*;

pub fn getSubStmts(_0: CStat) -> Vec<CStat> {
    match (_0) {
        CLabel(_, s, _, _) => {
            vec![s]
        },
        CCase(_, s, _) => {
            vec![s]
        },
        CCases(_, _, s, _) => {
            vec![s]
        },
        CDefault(s, _) => {
            vec![s]
        },
        CExpr(_, _) => {
            vec![s]
        },
        CCompound(_, body, _) => {
            vec![s]
        },
        CIf(_, sthen, selse, _) => {
            vec![s]
        },
        CSwitch(_, s, _) => {
            vec![s]
        },
        CWhile(_, s, _, _) => {
            vec![s]
        },
        CFor(_, _, _, s, _) => {
            vec![s]
        },
        CGoto(_, _) => {
            vec![s]
        },
        CGotoPtr(_, _) => {
            vec![s]
        },
        CCont(_) => {
            vec![s]
        },
        CBreak(_) => {
            vec![s]
        },
        CReturn(_, _) => {
            vec![s]
        },
        CAsm(_, _) => {
            vec![s]
        },
    }
}

pub fn mapBlockItemStmts(_0: fn(CStat) -> bool, _1: fn(CStat) -> CStat, _2: CBlockItem) -> CBlockItem {
    match (_0, _1, _2) {
        (stop, f, CBlockStmt(s)) => {
            CBlockStmt((mapSubStmts(stop, f, s)))
        },
        (_, _, bi) => {
            CBlockStmt((mapSubStmts(stop, f, s)))
        },
    }
}

pub fn compoundSubStmts(_0: CBlockItem) -> Vec<CStat> {
    match (_0) {
        CBlockStmt(s) => {
            vec![s]
        },
        CBlockDecl(_) => {
            vec![s]
        },
        CNestedFunDef(_) => {
            vec![s]
        },
    }
}

pub fn getLabels(_0: CStat) -> Vec<Ident> {
    match (_0) {
        CLabel(l, s, _, _) => {
            __op_concat(l, getLabels(s))
        },
        CCompound(ls, body, _) => {
            __op_concat(l, getLabels(s))
        },
        stmt => {
            __op_concat(l, getLabels(s))
        },
    }
}



