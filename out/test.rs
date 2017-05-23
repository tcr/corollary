pub mod Test_Hello {
    pub enum Term {
        Hello,
        World
    }
    pub use Term::*;

    pub fn helloworld() -> String {
        (__op_addadd((printer(Hello)), __op_addadd(" ".to_string(), (printer(World)))))
    }

    let main = putStrLn(helloworld);

    pub fn printer(__0: Term) -> String {
        match (__0) {
            Hello => {
                "Hello".to_string()
            },
            World => {
                "World".to_string()
            },
        }
    }

}

pub mod Test_Code {
    pub enum AlexReturn<a> {
        AlexEOF,
        AlexError(AlexInput),
        AlexSkip(AlexInput, isize),
        AlexToken(AlexInput, isize, a)
    }
    pub use AlexReturn::*;

    pub fn addExternIdent(ident: Ident, deferred: EnvMonad) -> EnvMonad {
        /* do */ {
            let action = runOnce(/* do */ {
                let itype = deferred;
                let rewrites = lift(asks(itemRewrites));
                let path = match Map::lookup((Symbol, identToString(ident)), rewrites) {
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

    pub fn addSymbolIdent(ident: Ident, (__mut, ty): (Rust::Mutable, CType)) -> EnvMonad {
        /* do */ {
            {
                let name = applyRenames(ident);
            };
            addSymbolIdentAction(ident)(Result);
            name
        }
    }

    pub fn bitWidth(__0: isize, __1: IntWidth) -> isize {
        match (__0, __1) {
            (wordWidth, WordWidth) => {
                wordWidth
            },
            (_, BitWidth(w)) => {
                w
            },
        }
    }

    pub fn blockToStatements((Rust::Block(stmts, mexpr)): Rust::Block) -> Vec<Rust::Stmt> {
        match mexpr {
            Some(expr) => {
                __op_addadd(stmts, exprToStatements(expr))
            },
            None => {
                stmts
            },
        }
    }

    pub fn sumEuler() -> isize {
        sum(map(euler, mkList))
    }

}



fn main() { /* demo */ }
