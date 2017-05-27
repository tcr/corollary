use haskell_support::*;

pub fn buildCppArgs(CppArgs(options, extra_args, _tmpdir, input_file, output_file_opt): CppArgs) -> Vec<String> {
    __op_addadd(/* do */ {
        (concatMap(tOption, options))
    }, __op_addadd(outputFileOpt, __op_addadd(vec!["-E".to_string(), input_file], extra_args)))
}

pub fn gccParseCPPArgs(args: Vec<String>) -> Either<String, (CppArgs, Vec<String>)> {
    match mungeArgs(((None, None, RList::empty), (RList::empty, RList::empty)), args) {
        Left | err => {
            Left(err)
        },
        Right | ((None, _, _), _) => {
            Left("No .c / .hc / .h source file given".to_string())
        },
        Right | ((Some(input_file), output_file_opt, cpp_opts), (extra_args, other_args)) => {
            Right(((rawCppArgs((RList::reverse(extra_args)), input_file))({
                    outputFile: output_file_opt,
                    cppOptions: RList::reverse(cpp_opts)
                }), RList::reverse(other_args)))
        },
    }
}

pub fn newGCC() -> GCC {
    GCC
}

