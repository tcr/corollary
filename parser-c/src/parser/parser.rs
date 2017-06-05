#[macro_use]
use corollary_support::*;

use data::input_stream::*;
use data::position::*;

pub fn parseC(input: InputStream, initialPosition: Position) -> Either<ParseError, CTranslUnit> {
    // TODO
    Left(ParseError("dummy".to_string()))
}
