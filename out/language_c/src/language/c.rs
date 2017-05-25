use haskell_support::*;

pub fn parseCFile(cpp: cpp, tmp_dir_opt: Option<FilePath>, args: Vec<String>, input_file: FilePath) -> IO<Either<ParseError, CTranslUnit>> {
    /* do */ {
        let input_stream = __op_bind(if(not, (isPreprocessed(input_file)), then, {
                let cpp_args = (rawCppArgs(args, input_file))({
                        cppTmpDir: tmp_dir_opt
                    });
            }, in, runPreprocessor, cpp, cpp_args), handleCppError(else, readInputStream, input_file));
        return(parseC(input_stream, (initPos(input_file))));

    }
}

pub fn parseCFilePre(file: FilePath) -> IO<Either<ParseError, CTranslUnit>> {
    /* do */ {
        let input_stream = readInputStream(file);
        return(parseC(input_stream, (initPos(file))))
    }
}

