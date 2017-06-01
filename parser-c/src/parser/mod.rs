use corollary_support::*;

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



