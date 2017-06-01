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


pub mod Language_C_Data {
    use haskell_support::*;

use Language::C::Data::InputStream;
use Language::C::Data::Ident;
use Language::C::Data::Name;
use Language::C::Data::Position;
use Language::C::Data::Error;
use Language::C::Data::Node;

}




