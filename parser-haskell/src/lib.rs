#[macro_use] extern crate errln;

extern crate lalrpop_util;
extern crate regex;
extern crate base64;

pub mod ast;
pub mod haskell;
pub mod util;
pub mod trans;
pub mod whitespace;

use whitespace::commify;

/// Preprocess code to remove comments and convert whitepsace to brace blocks.
/// TODO: merge this into parse() below once result lifetimes can be worked out
pub fn preprocess(input: &str) -> String {
    commify(input)
}

/// Entry point for parsing modules
pub fn parse<'input, 'err>(
    errors: &'err mut Vec<lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str
) -> Result<ast::Module, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    haskell::parse_Module(errors, &input)
}
