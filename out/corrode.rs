// ERROR: cannot yet convert file ""./corrode/src/Language/Rust/AST.hs""

// ERROR: cannot yet convert file ""./corrode/src/Language/Rust/Corrode/C.lhs""

// ERROR: cannot yet convert file ""./corrode/src/Language/Rust/Corrode/CFG.lhs""

// ERROR: cannot yet convert file ""./corrode/src/Language/Rust/Corrode/CrateMap.hs""

mod Language_Rust_Idiomatic {
    fn itemIdioms(__0: Rust.Item) -> Rust.Item {
        match (__0) {
            $$$ => Rust.Item(attrs, vis, (Rust.Function(fattrs, name, formals, ret, (tailBlock(b))))),
            i => i,
        }
    }

    fn tailBlock(__0: Rust.Block) -> Rust.Block {
        match (__0) {
            $$$ => Rust.Block(b, e),
            $$$ => Rust.Block(b, e),
            b => b,
        }
    }

    fn tailExpr(__0: Rust.Expr) -> Maybe<Maybe<Rust.Expr>> {
        match (__0) {
            $$$ => Just(e),
            $$$ => Just((Just((Rust.BlockExpr((tailBlock(b))))))),
            $$$ => Just((Just((Rust.IfThenElse(c, (tailBlock(t)), (tailBlock(f))))))),
            _ => Nothing,
        }
    }

    fn unsnoc(__0: Vec<a>) -> Maybe<(Vec<a>, a)> {
        match (__0) {
            $$$ => Nothing,
            $$$ => match unsnoc(xs) {
                    Just $$$ => Just((:(x, a), b)),
                    Nothing => Just((vec![], x)),
                },
        }
    }

}

mod Language_Rust {

}

