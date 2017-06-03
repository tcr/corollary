#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::List;
// use Language::C::Data::Ident;
// use Language::C::Syntax::AST;

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

pub fn mapSubStmts(_0: fn(CStat) -> bool, _1: fn(CStat) -> CStat, _2: CStat) -> CStat {
    match (_0, _1, _2) {
        (stop, _, s) => {
            /* Expr::Error */ Error
        },
        (stop, f, CLabel(i, s, attrs, ni)) => {
            f((CLabel(i, (mapSubStmts(stop, f, s)), attrs, ni)))
        },
        (stop, f, CCase(e, s, ni)) => {
            f((CCase(e, (mapSubStmts(stop, f, s)), ni)))
        },
        (stop, f, CCases(e1, e2, s, ni)) => {
            f((CCases(e1, e2, (mapSubStmts(stop, f, s)), ni)))
        },
        (stop, f, CDefault(s, ni)) => {
            f((CDefault((mapSubStmts(stop, f, s)), ni)))
        },
        (stop, f, CCompound(ls, body, ni)) => {
            f((CCompound(ls, (__map!((mapBlockItemStmts(stop, f)), body)), ni)))
        },
        (stop, f, CIf(e, sthen, selse, ni)) => {
            f((CIf(e, (mapSubStmts(stop, f, sthen)), (maybe(None, (Some(mapSubStmts(stop, f))), selse)), ni)))
        },
        (stop, f, CSwitch(e, s, ni)) => {
            f((CSwitch(e, (mapSubStmts(stop, f, s)), ni)))
        },
        (stop, f, CWhile(e, s, isdo, ni)) => {
            f((CWhile(e, (mapSubStmts(stop, f, s)), isdo, ni)))
        },
        (stop, f, CFor(i, t, a, s, ni)) => {
            f((CFor(i, t, a, (mapSubStmts(stop, f, s)), ni)))
        },
        (_, f, s) => {
            f(s)
        },
    }
}



