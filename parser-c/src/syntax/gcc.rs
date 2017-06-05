// Original file: "GCC.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::RList;
// use Language::C::System::Preprocess;
// use Data::Maybe;
// use System::Process;
// use System::Directory;
// use Data::List;

use syntax::preprocess::CppArgs;
use data::r_list::{RList, snoc};
use syntax::preprocess::CppOption::*;
use syntax::preprocess::*;

pub struct GCC {
    gccPath: FilePath,
}
fn gccPath(a: GCC) -> FilePath {
    a.gccPath
}

pub fn newGCC() -> GCC {
    GCC
}

pub fn gccParseCPPArgs(args: Vec<String>) -> Either<String, (CppArgs, Vec<String>)> {

pub fn mungeArgs(parsed: ParseArgsState, __OP__: Vec<String>, (cpp_args(__OP__, (inp, out, cpp_opts)), unparsed(__OP__, (extra, other))): Either<String, ParseArgsState>) -> Either<String, ParseArgsState>{
        match unparsed_args {
            ["-E", rest..] => mungeArgs(parsed, rest),
            [flag, flagArg, rest..] if (flag ==
                                        ("-MF".to_string() ||
                                         (flag ==
                                          ("-MT".to_string() || (flag == "-MQ".to_string()))))) => {
                mungeArgs((cpp_args, (extra, snoc(other, snoc(flag, flagArg)))), rest)
            }
            [flag, rest..] if (flag ==
                             ("-c".to_string() ||
                              (flag ==
                               ("-S".to_string() || isPrefixOf("-M".to_string(), flag))))) => {
                mungeArgs((cpp_args, (extra, snoc(other, flag))), rest)
            }
            ["-o", file, rest..] if isJust(out) => Left("two output files given".to_string()),
            ["-o", file, rest..] => mungeArgs(((inp, Some(file), cpp_opts), unparsed), rest),
            [cpp_opt, rest..] if Some((opt, rest_q)) => {
                mungeArgs(((inp, out, snoc(cpp_opts, opt)), unparsed), rest_q)
            }
            [cfile, rest..] if any(|x| { isSuffixOf(cfile, x) }, (words(".c .hc .h".to_string()))) => {
                if isJust(inp) {
                    Left("two input files given".to_string())
                } else {
                    mungeArgs(((Some(cfile), out, cpp_opts), unparsed), rest)
                }
            }
            [unknown, rest..] => mungeArgs((cpp_args, (snoc(extra, unknown), other)), rest),
            [] => Right(parsed),
        }
    }

    let getDefine = |opt| {
        let (key, val) = __break(|x| { '=' == x}, opt);

        Define(key, (if val.is_empty() { "".to_string() } else { tail(val) }))
    };

    match mungeArgs(((None, None, RList::empty), (RList::empty, RList::empty)),
                    args) {
        Left(err) => Left(err),
        Right(((None, _, _), _)) => Left("No .c / .hc / .h source file given".to_string()),
        Right(((Some(input_file), output_file_opt, cpp_opts), (extra_args, other_args))) => {
            Right((__assign!((rawCppArgs((RList::reverse(extra_args)), input_file)),
                             REMOVE {
                                 outputFile: output_file_opt,
                                 cppOptions: RList::reverse(cpp_opts),
                             }),
                   RList::reverse(other_args)))
        }
    }
}

pub type ParseArgsState = ((Option<FilePath>, Option<FilePath>, RList<CppOption>),
                           (RList<String>, RList<String>));

pub fn buildCppArgs(CppArgs {
    cppOptions: options,
    extraOptions: extra_args,
    cppTmpDir: _tmpdir,
    inputFile: input_file,
    outputFile: output_file_opt
}: CppArgs)
                    -> Vec<String> {

    let tOption = |_0| match (_0) {
        IncludeDir(incl) => vec!["-I".to_string(), incl],
        Define(key, value) => {
            vec![__op_addadd("-D".to_string(),
                             __op_addadd(key,
                                         (__op_addadd(if value.is_empty() {
                                                          "".to_string()
                                                      } else {
                                                          "=".to_string()
                                                      },
                                                      value))))]
        }
        Undefine(key) => vec![__op_addadd("-U".to_string(), key)],
        IncludeFile(f) => vec!["-include".to_string(), f],
    };
    
    let outputFileOpt = output_file_opt.map(|x| ["-o".to_string(), x]).unwrap_or(vec![]);

    __op_addadd((__concatMap!(tOption, options)),
                __op_addadd(outputFileOpt,
                            __op_addadd(vec!["-E".to_string(), input_file], extra_args)))
}
