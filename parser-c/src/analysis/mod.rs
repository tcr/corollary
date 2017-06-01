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


pub mod Language_C_Analysis {
    use haskell_support::*;

use Language::C::Analysis::SemError;
use Language::C::Analysis::SemRep;
use Language::C::Analysis::DefTable;
use Language::C::Analysis::TravMonad;
use Language::C::Analysis::AstAnalysis;
use Language::C::Analysis::DeclAnalysis;
use Language::C::Analysis::Debug;

}




