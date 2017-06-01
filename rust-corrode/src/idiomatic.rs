mod haskell_support {
    pub trait Addable {
        fn add(self, right: Self) -> Self;
    }

    impl Addable for String {
        fn add(self, right: Self) -> Self {
            format!("{}{}", self, right)
        }
    }

    pub fn __op_addadd<A: Addable>(left: A, right: A) -> A {
        Addable::add(left, right)
    }
}


pub mod Language_Rust_Idiomatic {
    use haskell_support::*;

use Language::Rust::AST;

    pub fn itemIdioms(__0: Rust::Item) -> Rust::Item {
        match (__0) {
            Rust::Item(attrs, vis, Rust::Function(fattrs, name, formals, ret, b)) => {
                Rust::Item(attrs, vis, (Rust::Function(fattrs, name, formals, ret, (tailBlock(b)))))
            },
            i => {
                i
            },
        }
    }

    pub fn tailBlock(__0: Rust::Block) -> Rust::Block {
        match (__0) {
            Rust::Block(b, Some(/* TODO ViewPattern */ tailExpr)) => {
                Rust::Block(b, e)
            },
            Rust::Block(/* TODO ViewPattern */ unsnoc, None) => {
                Rust::Block(b, e)
            },
            b => {
                b
            },
        }
    }

    pub fn tailExpr(__0: Rust::Expr) -> Option<Option<Rust::Expr>> {
        match (__0) {
            Rust::Return(e) => {
                Some(e)
            },
            Rust::BlockExpr(b) => {
                Some((Some((Rust::BlockExpr((tailBlock(b)))))))
            },
            Rust::IfThenElse(c, t, f) => {
                Some((Some((Rust::IfThenElse(c, (tailBlock(t)), (tailBlock(f)))))))
            },
            _ => {
                None
            },
        }
    }

    pub fn unsnoc(__0: Vec<a>) -> Option<(Vec<a>, a)> {
        match (__0) {
            [] => {
                None
            },
            [x, xs] => {
                match unsnoc(xs) {
                    Some((a, b)) => {
                        Some((__op_concat(x, a), b))
                    },
                    None => {
                        Some((vec![], x))
                    },
                }
            },
        }
    }

}




