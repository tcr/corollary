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


pub mod Language_C_Data_Name {
    use haskell_support::*;

use Data::Ix;
use Data::IntMap;
use IntMap;
use Data::IntMap;
use Data::Generics;

    pub fn namesStartingFrom(k: isize) -> Vec<Name> {
        vec![Name(k::::)]
    }

    pub fn newNameSupply() -> Vec<Name> {
        namesStartingFrom(0)
    }

}




