// Original file: "Utils.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::List;
// use Language::C::Data::Ident;
// use Language::C::Syntax::AST;

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
            vec![]
        },
        CCompound(_, body, _) => {
            concatMap(compoundSubStmts, body)
        },
        CIf(_, sthen, selse, _) => {
            maybe(vec![sthen], (|s| { vec![sthen, s] }), selse)
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
            vec![]
        },
        CGotoPtr(_, _) => {
            vec![]
        },
        CCont(_) => {
            vec![]
        },
        CBreak(_) => {
            vec![]
        },
        CReturn(_, _) => {
            vec![]
        },
        CAsm(_, _) => {
            vec![]
        },
    }
}

pub fn mapBlockItemStmts(_0: fn(CStat) -> bool, _1: fn(CStat) -> CStat, _2: CBlockItem) -> CBlockItem {
    match (_0, _1, _2) {
        (stop, f, CBlockStmt(s)) => {
            CBlockStmt((mapSubStmts(stop, f, s)))
        },
        (_, _, bi) => {
            bi
        },
    }
}

pub fn compoundSubStmts(_0: CBlockItem) -> Vec<CStat> {
    match (_0) {
        CBlockStmt(s) => {
            vec![s]
        },
        CBlockDecl(_) => {
            vec![]
        },
        CNestedFunDef(_) => {
            vec![]
        },
    }
}

pub fn getLabels(_0: CStat) -> Vec<Ident> {
    match (_0) {
        CLabel(l, s, _, _) => {
            __op_concat(l, getLabels(s))
        },
        CCompound(ls, body, _) => {
            __op_forwardslash(concatMap((concatMap(getLabels, compoundSubStmts)), body), ls)
        },
        stmt => {
            concatMap(getLabels, (getSubStmts(stmt)))
        },
    }
}



