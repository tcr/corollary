#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::InputStream;
// use System::Exit;
// use System::Directory;
// use System::FilePath;
// use System::Environment;
// use System::IO;
// use Control::Exception;
// use Control::Monad;
// use Data::List;

pub enum CppOption {
    IncludeDir(FilePath),
    Define(String, String),
    Undefine(String),
    IncludeFile(FilePath)
}
pub use self::CppOption::*;

pub struct CppArgs{
    cppOptions: Vec<CppOption>,
    extraOptions: Vec<String>,
    cppTmpDir: Option<FilePath>,
    inputFile: FilePath,
    outputFile: Option<FilePath>
}
fn cppOptions(a: CppArgs) -> Vec<CppOption> { a.cppOptions }
fn extraOptions(a: CppArgs) -> Vec<String> { a.extraOptions }
fn cppTmpDir(a: CppArgs) -> Option<FilePath> { a.cppTmpDir }
fn inputFile(a: CppArgs) -> FilePath { a.inputFile }
fn outputFile(a: CppArgs) -> Option<FilePath> { a.outputFile }

pub fn addCppOption<a>(cpp_args: CppArgs, opt: CppOption) -> CppArgs {
    cpp_args {
        cppOptions: __op_concat(opt, (cppOptions(cpp_args)))
    }
}

pub fn addExtraOption<a>(cpp_args: CppArgs, extra: String) -> CppArgs {
    cpp_args {
        extraOptions: __op_concat(extra, (extraOptions(cpp_args)))
    }
}

pub fn cppFile<a>(input_file: FilePath) -> CppArgs {
    CppArgs {
        cppOptions: vec![],
        extraOptions: vec![],
        cppTmpDir: None,
        inputFile: input_file,
        outputFile: None
    }
}

pub fn getOutputFileName<a>(fp: FilePath) -> FilePath {
    /* Expr::Error */ Error
}

pub fn isPreprocessed<a>() -> bool {
    (".i".to_string()(isSuffixOf))
}

pub fn mkOutputFile<a>(tmp_dir_opt: Option<FilePath>, input_file: FilePath) -> IO<FilePath> {
    /*do*/ {
        let tmpDir = getTempDir(tmp_dir_opt);

        mkTmpFile(tmpDir, (getOutputFileName(input_file)))
    }
}

pub fn mkTmpFile<a>(tmp_dir: FilePath, file_templ: FilePath) -> IO<FilePath> {
    /*do*/ {
        let (path, file_handle) = openTempFile(tmp_dir, file_templ);

        hClose(file_handle);
        path
    }
}

pub fn preprocessedExt<a>() -> String {
    ".i".to_string()
}

pub fn rawCppArgs<a>(opts: Vec<String>, input_file: FilePath) -> CppArgs {
    CppArgs {
        inputFile: input_file,
        cppOptions: vec![],
        extraOptions: opts,
        outputFile: None,
        cppTmpDir: None
    }
}

pub fn runPreprocessor<a>(cpp: cpp, cpp_args: CppArgs) -> IO<Either<ExitCode, InputStream>> {
    /*do*/ {
        bracket(getActualOutFile, removeTmpOutFile, invokeCpp)
    }
}



