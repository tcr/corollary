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


pub mod Language_C_Parser {
    use haskell_support::*;

use Language::C::Parser::Parser;
use parseC;
use Language::C::Parser::ParserMonad;
use execParser;
use Language::C::Parser::Builtin;
use builtinTypeNames;
use Language::C::Data;

    pub fn execParser_(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}




