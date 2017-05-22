mod Test_Code {
new_Csh [([Ref(Ident("__0")), Ref(Ident("__1"))], Case(Parens([Ref(Ident("__0")), Ref(Ident("__1"))]), [Direct([Ref(Ident("wordWidth")), Ref(Ident("WordWidth"))], [Span([Ref(Ident("wordWidth"))])]), Direct([Ref(Ident("_")), Span([Span([Ref(Ident("BitWidth")), Ref(Ident("w"))])])], [Span([Ref(Ident("w"))])])]))]
    fn addExternIdent(ident: EnvMonad) -> EnvMonad {
        /* do */ {
            let action = runOnce(/* do */ {
                let itype = deferred;
                let rewrites = lift(asks(itemRewrites));
                let path = match Map_lookup((Symbol, identToString(ident)), rewrites) {
                    Just(renamed) => {
                        return((__op_concat("".to_string(), renamed)))
                    },
                    Nothing => {
                        return(vec![name])
                    },
                }
            });
            addSymbolIdentAction(ident, action)
        }
    }

    fn bitWidth(__0: isize) -> isize {
        match (__0, __1) {
            wordWidth(WordWidth) => {
                wordWidth
            },
            _(BitWidth(w)) => {
                w
            },
        }
    }

    fn blockToStatements((Rust_Block(stmts, mexpr)): Vec<Rust.Stmt>) -> Vec<Rust.Stmt> {
        match mexpr {
            Just(expr) => {
                __op_addadd(stmts, exprToStatements(expr))
            },
            Nothing => {
                stmts
            },
        }
    }

}



fn main() { /* demo */ }
