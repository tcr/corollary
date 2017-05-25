mod haskell_support {
    pub trait Addable {
        fn add(self, right: Self) -> Self;
    }

<<<<<<< 011d6cd66ada531456a0aa265b31e41dd8989ee8
    impl Addable for String {
        fn add(self, right: Self) -> Self {
            format!("{}{}", self, right)
        }
    }

    pub fn __op_addadd<A: Addable>(left: A, right: A) -> A {
        Addable::add(left, right)
=======
    fn addExternIdent(ident: Ident, deferred: EnvMonad) -> EnvMonad {
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

    fn addSymbolIdent(ident: Ident, (__mut, ty): (Rust::Mutable, CType)) -> EnvMonad {
        /* do */ {
            {
                let name = applyRenames(ident);
            };
            addSymbolIdentAction(ident)(Result);
            name
        }
>>>>>>> Refresh output
    }
}


pub mod Test_Hello {
    use haskell_support::*;
    pub enum Term {
        Hello,
        World
    }
    pub use self::Term::*;

<<<<<<< 011d6cd66ada531456a0aa265b31e41dd8989ee8
    pub fn helloworld() -> String {
        (__op_addadd((printer(Hello)), __op_addadd(" ".to_string(), (printer(World)))))
    }

    pub fn printer(__0: Term) -> String {
        match (__0) {
            Hello => {
                "Hello".to_string()
=======
    fn blockToStatements((Rust::Block(stmts, mexpr)): Rust::Block) -> Vec<Rust::Stmt> {
        match mexpr {
            Some(expr) => {
                __op_addadd(stmts, exprToStatements(expr))
>>>>>>> Refresh output
            },
            World => {
                "World".to_string()
            },
        }
    }

}



/* RUST ... /RUST */

fn main() {
    assert_eq!("Hello World", Test_Hello::helloworld());
    println!("success.");
}

