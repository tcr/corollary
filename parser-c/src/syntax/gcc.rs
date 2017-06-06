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

pub fn newGCC(gccPath: FilePath) -> GCC {
    GCC { gccPath }
}

pub fn gccParseCPPArgs(args: Vec<String>) -> Either<String, (CppArgs, Vec<String>)> {


    fn getDefine(opt: String) -> CppOption {
        let (key, val) = __break(|x| { '=' == x}, opt);

        Define(key, (if val.is_empty() { "".to_string() } else { tail(val) }))
    }

    fn getArgOpt (cpp_opt: String, mut rest: Vec<String>) -> Option<(CppOption, Vec<String>)> {
        if isPrefixOf("-I".to_string(), cpp_opt) {
            Some((IncludeDir(drop(2, cpp_opt)), rest))
        } else if isPrefixOf("-U".to_string(), cpp_opt) {
            Some((Undefine(drop(2, cpp_opt)), rest))
        } else if isPrefixOf("-D".to_string(), cpp_opt) {
            Some((getDefine(drop(2, cpp_opt), rest)))
        } else if cpp_opt == "-include" {
            let f = rest.remove(0);
            Some((IncludeFile(f), rest))
        } else {
            None
        }
    }
    
    fn mungeArgs(_0: ParseArgsState, unparsed_args: Vec<String>) -> Either<String, ParseArgsState> {
        match _0 {
            parsed @ (cpp_args @ (inp, out, cpp_opts),
                unparsed @ (extra, other)) => {
                match __boxed_slice(unparsed_args) {
                    box ["-E", rest..] => mungeArgs(parsed, rest),
                    box [flag, flagArg, rest..] if (flag == "-MF".to_string()) ||
                                                (flag == "-MT".to_string()) ||
                                                (flag == "-MQ".to_string()) => {
                        mungeArgs((cpp_args, (extra, snoc(other, snoc(flag, flagArg)))), rest)
                    }
                    box [flag, rest..] if (flag == "-c".to_string()) ||
                                    (flag == "-S".to_string()) ||
                                    isPrefixOf("-M".to_string(), flag) => {
                        mungeArgs((cpp_args, (extra, snoc(other, flag))), rest)
                    }
                    box ["-o", file, rest..] if isJust(out) => Left("two output files given".to_string()),
                    box ["-o", file, rest..] => mungeArgs(((inp, Some(file), cpp_opts), unparsed), rest),
                    box [cpp_opt, rest..] if getArgOpt(cpp_opt, rest.to_vec()).is_some() => {
                        let (opt, rest_q) = getArgOpt(cpp_opt, rest.to_vec()).unwrap();
                        mungeArgs(((inp, out, snoc(cpp_opts, opt)), unparsed), rest_q)
                    }
                    box [cfile, rest..] if any(|x| { isSuffixOf(cfile, x.clone()) }, (words(".c .hc .h".to_string()))) => {
                        if isJust(inp) {
                            Left("two input files given".to_string())
                        } else {
                            mungeArgs(((Some(cfile), out, cpp_opts), unparsed), rest)
                        }
                    }
                    box [unknown, rest..] => mungeArgs((cpp_args, (snoc(extra, unknown), other)), rest),
                    box [] => Right(parsed),
                }
            }
        }
    }

    match mungeArgs(((None, None, RList::empty), (RList::empty, RList::empty)),
                    args) {
        Left(err) => Left(err),
        Right(((None, _, _), _)) => Left("No .c / .hc / .h source file given".to_string()),
        Right(((Some(input_file), output_file_opt, cpp_opts), (extra_args, other_args))) => {
            Right((__assign!((rawCppArgs((RList::reverse(extra_args)), input_file)),
                             {
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
        IncludeDir(incl) => vec!["-I".to_string(), incl.to_string()],
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
        IncludeFile(f) => vec!["-include".to_string(), f.to_string()],
    };
    
    let outputFileOpt = output_file_opt.map(|x| vec!["-o".to_string(), x.to_string()]).unwrap_or(vec![]);

    __op_addadd((__concatMap!(tOption, options)),
                __op_addadd(outputFileOpt,
                            __op_addadd(vec!["-E".to_string(), input_file.to_string()],
                            extra_args)))
}
