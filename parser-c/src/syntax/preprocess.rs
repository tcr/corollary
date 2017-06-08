// Original file: "Preprocess.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::InputStream;
// use System::Exit;
// use System::Directory;
// use System::FilePath;
// use System::IO;
// use Control::Exception;
// use Control::Monad;
// use Data::List;

use data::input_stream::*;

// 'Preprocessor' encapsulates the abstract interface for invoking C preprocessors
pub trait Preprocessor {
    // parse the given command line arguments, and return a pair of parsed and ignored arguments
    fn parseCPPArgs(&self, Vec<String>, Either<String, String>) -> (CppArgs, Vec<String>);
    // run the preprocessor
    fn runCPP(&self, CppArgs) -> ExitCode;
}

pub fn preprocessedExt() -> String {
    ".i".to_string()
}

#[derive(Clone)]
pub enum CppOption {
    IncludeDir(FilePath),
    Define(String, String),
    Undefine(String),
    IncludeFile(FilePath),
}
pub use self::CppOption::*;

#[derive(Clone)]
pub struct CppArgs {
    pub cppOptions: Vec<CppOption>,
    pub extraOptions: Vec<String>,
    pub cppTmpDir: Option<FilePath>,
    pub inputFile: FilePath,
    pub outputFile: Option<FilePath>,
}
fn cppOptions(a: CppArgs) -> Vec<CppOption> {
    a.cppOptions
}
fn extraOptions(a: CppArgs) -> Vec<String> {
    a.extraOptions
}
fn cppTmpDir(a: CppArgs) -> Option<FilePath> {
    a.cppTmpDir
}
fn inputFile(a: CppArgs) -> FilePath {
    a.inputFile
}
fn outputFile(a: CppArgs) -> Option<FilePath> {
    a.outputFile
}

pub fn cppFile(input_file: FilePath) -> CppArgs {
    CppArgs {
        cppOptions: vec![],
        extraOptions: vec![],
        cppTmpDir: None,
        inputFile: input_file,
        outputFile: None,
    }
}

pub fn rawCppArgs(opts: Vec<String>, input_file: FilePath) -> CppArgs {
    CppArgs {
        inputFile: input_file,
        cppOptions: vec![],
        extraOptions: opts,
        outputFile: None,
        cppTmpDir: None,
    }
}

pub fn addCppOption(cpp_args: CppArgs, opt: CppOption) -> CppArgs {
    __assign!(cpp_args, {
        cppOptions: __op_concat(opt, cppOptions(cpp_args))
    })
}

pub fn addExtraOption(cpp_args: CppArgs, extra: String) -> CppArgs {
    __assign!(cpp_args, {
        extraOptions: __op_concat(extra, extraOptions(cpp_args))
    })
}

pub fn runPreprocessor<P: Preprocessor>(cpp: P,
                                        cpp_args: CppArgs)
                                        -> Either<ExitCode, InputStream> {

    pub fn getActualOutFile(cpp_args: CppArgs) -> FilePath {
        outputFile(cpp_args.clone())
            .unwrap_or_else(|| {
                mkOutputFile((cppTmpDir(cpp_args.clone())), (inputFile(cpp_args.clone())))
            })
    }

    let cpp_args2 = cpp_args.clone();
    let invokeCpp = |actual_out_file| {
        /*do*/
        {
            let exit_code = cpp.runCPP(__assign!(cpp_args2, {
                outputFile: Some(actual_out_file)
            }));

            match exit_code {
                ExitSuccess => Right(readInputStream(actual_out_file)),
                ExitFailure(_) => Left(exit_code),
            }
        }
    };

    let cpp_args2 = cpp_args.clone();
    let removeTmpOutFile =
        |out_file| maybe((removeFile(out_file)), (|_| ()), (outputFile(cpp_args2)));

    let path = getActualOutFile(cpp_args);
    let ret = invokeCpp(path.clone());
    removeTmpOutFile(path.clone());
    ret
}

pub fn mkOutputFile(tmp_dir_opt: Option<FilePath>, input_file: FilePath) -> FilePath {

    let getTempDir = |_0| match (_0) {
        Some(tmpdir) => tmpdir,
        None => getTemporaryDirectory(),
    };

    /*do*/
    {
        let tmpDir = getTempDir(tmp_dir_opt);

        mkTmpFile(tmpDir, (getOutputFileName(input_file)))
    }
}

// compute output file name from input file name
pub fn getOutputFileName(fp: FilePath) -> FilePath {
    let filename = takeFileName(fp.clone());
    if hasExtension(fp) {
        replaceExtension(filename, preprocessedExt())
    } else {
        addExtension(filename, preprocessedExt())
    }
}

pub fn mkTmpFile(tmp_dir: FilePath, file_templ: FilePath) -> FilePath {
    /*do*/
    {
        let (path, file_handle) = openTempFile(tmp_dir, file_templ);

        hClose(file_handle);
        path
    }
}

pub fn isPreprocessed(x: String) -> bool {
    isSuffixOf(".i".to_string(), x)
}
