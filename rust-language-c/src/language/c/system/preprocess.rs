use haskell_support::*;

pub enum CppOption {
    IncludeDir(FilePath),
    Define(String, String),
    Undefine(String),
    IncludeFile(FilePath)
}
pub use self::CppOption::*;

struct CppArgs(CppArgs<TypeRecord /* todo */>);

pub fn addCppOption(cpp_args: CppArgs, opt: CppOption) -> CppArgs {
    cpp_args {
        cppOptions: __op_concat(opt, (cppOptions(cpp_args)))
    }
}

pub fn addExtraOption(cpp_args: CppArgs, extra: String) -> CppArgs {
    cpp_args {
        extraOptions: __op_concat(extra, (extraOptions(cpp_args)))
    }
}

pub fn cppFile(input_file: FilePath) -> CppArgs {
    CppArgs {
        cppOptions: vec![],
        extraOptions: vec![],
        cppTmpDir: None,
        inputFile: input_file,
        outputFile: None
    }
}

pub fn getOutputFileName(fp: FilePath) -> FilePath {
    /* Expr::Error */ Error
}

pub fn isPreprocessed() -> bool {
    (".i".to_string()(isSuffixOf))
}

pub fn mkOutputFile(tmp_dir_opt: Option<FilePath>, input_file: FilePath) -> IO<FilePath> {
    /* do */ {
        let tmpDir = getTempDir(tmp_dir_opt);

        mkTmpFile(tmpDir, (getOutputFileName(input_file)))
    }
}

pub fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> IO<FilePath> {
    /* do */ {
        let path(file_handle) = openTempFile(tmp_dir, file_templ);

        hClose(file_handle);
        path
    }
}

pub fn preprocessedExt() -> String {
    ".i".to_string()
}

pub fn rawCppArgs(opts: Vec<String>, input_file: FilePath) -> CppArgs {
    CppArgs {
        inputFile: input_file,
        cppOptions: vec![],
        extraOptions: opts,
        outputFile: None,
        cppTmpDir: None
    }
}

pub fn runPreprocessor(cpp: cpp, cpp_args: CppArgs) -> IO<Either<ExitCode, InputStream>> {
    /* do */ {
        bracket(getActualOutFile, removeTmpOutFile, invokeCpp)
    }
}

