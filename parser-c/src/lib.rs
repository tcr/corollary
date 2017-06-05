// Original file: "C.hs"
// File auto-generated using Corollary.

#![allow(unused_parens)]

extern crate corollary_support;

#[macro_use]
use corollary_support::*;

pub mod analysis;
pub mod data;
pub mod parser;
pub mod syntax;

use syntax::preprocess::*;
use syntax::ast::*;
use data::input_stream::readInputStream;
use data::position::initPos;
use parser::parser_monad::ParseError;
use parser::parser::parseC;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data;
// use Language::C::Syntax;
// use Language::C::Pretty;
// use Language::C::Parser;
// use Language::C::System::Preprocess;

pub fn parseCFile<C: Preprocessor>(cpp: C,
                  tmp_dir_opt: Option<FilePath>,
                  args: Vec<String>,
                  input_file: FilePath)
                  -> Either<ParseError, CTranslUnit> {

    let handleCppError = |_0| match (_0) {
        Left(exitCode) => {
            Err(__op_addadd("Preprocessor failed with ".to_string(), show(exitCode)))
        }
        Right(ok) => ok,
    };

    /*do*/
    {
        let input_stream = if !isPreprocessed(input_file) {
            {
                let cpp_args = __assign!((rawCppArgs(args, input_file)), {
                    cppTmpDir: tmp_dir_opt
                });

                __op_bind(runPreprocessor(cpp, cpp_args), handleCppError)
            }
        } else {
            readInputStream(input_file)
        };

        parseC(input_stream, (initPos(input_file)))
    }
}

pub fn parseCFilePre(file: FilePath) -> Either<ParseError, CTranslUnit> {
    /*do*/
    {
        let input_stream = readInputStream(file);

        parseC(input_stream, (initPos(file)))
    }
}
