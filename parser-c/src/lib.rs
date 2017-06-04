// Original file: "C.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data;
// use Language::C::Syntax;
// use Language::C::Pretty;
// use Language::C::Parser;
// use Language::C::System::Preprocess;

pub fn parseCFile(cpp: cpp, tmp_dir_opt: Option<FilePath>, args: Vec<String>, input_file: FilePath) -> IO<Either<ParseError, CTranslUnit>> {

    let handleCppError = |_0| {
        match (_0) {
            Left(exitCode) => {
                fail(__op_addadd("Preprocessor failed with ".to_string(), show(exitCode)))
            },
            Right(ok) => {
                fail(__op_addadd("Preprocessor failed with ".to_string(), show(exitCode)))
            },
        }
    };

    /*do*/ {
        let input_stream = if not((isPreprocessed(input_file))) {             
{
                let cpp_args = __assign!((rawCppArgs(args, input_file)), {
                        cppTmpDir: tmp_dir_opt
                    });

            __op_bind(runPreprocessor(cpp, cpp_args), handleCppError)            }} else {
readInputStream(input_file)
            };

        parseC(input_stream, (initPos(input_file)))
    }
}

pub fn parseCFilePre(file: FilePath) -> IO<Either<ParseError, CTranslUnit>> {
    /*do*/ {
        let input_stream = readInputStream(file);

        parseC(input_stream, (initPos(file)))
    }
}



