// Original file: "Parser.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Parser::Parser;
// use parseC;
// use Language::C::Parser::ParserMonad;
// use execParser;
// use Language::C::Parser::Builtin;
// use builtinTypeNames;
// use Language::C::Data;

use parser::parser_monad::P;

pub mod builtin;
pub mod lexer;
pub mod parser_monad;
pub mod parser;
pub mod tokens;

use parser::parser_monad::execParser;
use parser::builtin::*;
use data::name::*;
use parser::parser_monad::ParseError;
use data::position::Position;
use data::input_stream::InputStream;

pub fn execParser_<a>(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
    match execParser(parser, input, pos, builtinTypeNames, newNameSupply) {
        s @ Left(..) => s,
        Right((v, _)) => Right(v)
    }
}
