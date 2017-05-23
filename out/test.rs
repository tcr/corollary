mod Test_Code {
    enum AlexReturn<a> {
        AlexEOF,
        AlexError(AlexInput),
        AlexSkip(AlexInput, isize),
        AlexToken(AlexInput, isize, a)
    }

    fn addExternIdent(ident: Ident, deferred: EnvMonad) -> EnvMonad {
        /* do */ {
            let action = runOnce(/* do */ {
                let itype = deferred;
                let rewrites = lift(asks(itemRewrites));
                let path = match Map_lookup((Symbol, identToString(ident)), rewrites) {
                    Some(renamed) => {
                        (__op_concat("".to_string(), renamed))
                    },
                    None => {
                        vec![name]
                    },
                }
            });
            addSymbolIdentAction(ident, action)
        }
    }

    fn addSymbolIdent(ident: Ident, (__mut, ty): (Rust_Mutable, CType)) -> EnvMonad {
        /* do */ {
            {
                let name = applyRenames(ident);
            };
            addSymbolIdentAction(ident)(Result);
            name
        }
    }

    fn bitWidth(__0: isize, __1: IntWidth) -> isize {
        match (__0, __1) {
            (wordWidth, WordWidth) => {
                wordWidth
            },
            (_, BitWidth(w)) => {
                w
            },
        }
    }

    fn blockToStatements((Rust_Block(stmts, mexpr)): Rust_Block) -> Vec<Rust_Stmt> {
        match mexpr {
            Some(expr) => {
                __op_addadd(stmts, exprToStatements(expr))
            },
            None => {
                stmts
            },
        }
    }

    fn sumEuler() -> isize {
        sum(map(euler, mkList))
    }

}



fn main() { /* demo */ }
