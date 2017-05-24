pub mod Language_C_Parser {
    use haskell_support::*;
    pub fn execParser_(parser: P<a>, input: InputStream, pos: Position) -> Either<ParseError, a> {
        fmap(fst)(execParser(parser, input, pos, builtinTypeNames, newNameSupply))
    }

}

