// Original file: "C.hs"
// File auto-generated using Corollary.

#![feature(proc_macro)]
#![feature(slice_patterns, box_syntax, box_patterns, fnbox)]
#![allow(unused_parens)]

#[macro_use] extern crate corollary_support;
extern crate num;
#[macro_use] extern crate matches;
#[macro_use] extern crate num_derive;
extern crate parser_c_macro;

use corollary_support::*;

// pub mod analysis;
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
        Left(exitCode) => __error!(__op_addadd("Preprocessor failed with ".to_string(), show(exitCode))),
        Right(ok) => ok,
    };

    /*do*/
    {
        let input_stream = if !isPreprocessed(input_file.clone().into()) {
            {
                let cpp_args = __assign!((rawCppArgs(args, input_file.clone())), {
                    cppTmpDir: tmp_dir_opt
                });

                handleCppError(runPreprocessor(cpp, cpp_args))
            }
        } else {
            readInputStream(input_file.clone())
        };

        parseC(input_stream, (initPos(input_file)))
    }
}

pub fn parseCFilePre(file: FilePath) -> Either<ParseError, CTranslUnit> {
    /*do*/
    {
        let input_stream = readInputStream(file.clone());

        parseC(input_stream, (initPos(file)))
    }
}
