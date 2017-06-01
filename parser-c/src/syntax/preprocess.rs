mod haskell_support {
    pub trait Addable {
        fn add(self, right: Self) -> Self;
    }

    impl Addable for String {
        fn add(self, right: Self) -> Self {
            format!("{}{}", self, right)
        }
    }

    pub fn __op_addadd<A: Addable>(left: A, right: A) -> A {
        Addable::add(left, right)
    }
}


pub mod Language_C_System_Preprocess {
    use haskell_support::*;

use Language::C::Data::InputStream;
use System::Exit;
use System::Directory;
use System::FilePath;
use System::Environment;
use System::IO;
use Control::Exception;
use Control::Monad;
use Data::List;

    pub enum CppOption {
        IncludeDir(FilePath),
        Define(String, String),
        Undefine(String),
        IncludeFile(FilePath)
    }
    pub use self::CppOption::*;

    struct CppArgs{
        cppOptions: Vec<CppOption>,
        extraOptions: Vec<String>,
        cppTmpDir: Option<FilePath>,
        inputFile: FilePath,
        outputFile: Option<FilePath>
    }

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
            let (path, file_handle) = openTempFile(tmp_dir, file_templ);

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

}




