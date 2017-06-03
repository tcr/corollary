#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Parser::Parser;
// use parseC;
// use Language::C::Parser::ParserMonad;
// use execParser;
// use Language::C::Parser::Builtin;
// use builtinTypeNames;
// use Language::C::Data;

pub fn execParser_<a>(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
    fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
}



