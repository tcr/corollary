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


pub mod Language_C_Analysis_Builtins {
    use haskell_support::*;

use Language::C::Data::Ident;
use Language::C::Data::Node;
use Language::C::Analysis::DefTable;
use Language::C::Analysis::SemRep;
use Language::C::Analysis::TypeUtils;

    pub fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}




