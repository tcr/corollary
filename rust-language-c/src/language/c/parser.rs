use haskell_support::*;

use language_.c._parser::parser;
use parse_c;
use language_.c._parser::parser_monad;
use exec_parser;
use language_.c._parser::builtin;
use builtin_type_names;
use language_.c._data;

pub fn execParser_(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
    fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
}

