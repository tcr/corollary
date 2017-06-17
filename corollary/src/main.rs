#[macro_use] extern crate errln;
#[macro_use] extern crate error_chain;
extern crate clap;
extern crate hex;
extern crate lalrpop_util;
extern crate parser_haskell;
extern crate regex;
extern crate tempdir;
extern crate walkdir;
extern crate corollary;
extern crate inflector;

use parser_haskell::util::{print_parse_error, simplify_parse_error};

use clap::{Arg, App};
use regex::Regex;
use std::fmt::Write;
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempdir::TempDir;

use corollary::print_item_list;
use corollary::ir::PrintState;

// Define error chain.
mod errors {
    error_chain! {
        foreign_links {
            Walkdir(::walkdir::Error);
            Io(::std::io::Error);
            Fmt(::std::fmt::Error);
        }
    }
}
use errors::*;

#[test] #[ignore]
fn test_single_file() {
    let a = "./corrode/src/Language/Rust/Corrode/C.lhs";
    // let a = "./corrode/src/Language/Rust/Corrode/C.hs";
    // let a = "./test/input.hs";
    println!("file: {}", a);
    let mut file = File::open(a).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if a.ends_with(".lhs") {
        contents = strip_lhs(&contents);
    }
    let contents = parser_haskell::preprocess(&contents);

    // let mut a = ::std::fs::File::create("temp.txt").unwrap();
    // a.write_all(contents.as_bytes());

    let mut errors = Vec::new();
    match parser_haskell::parse(&mut errors, &contents) {
        Ok(okay) => println!("{:#?}", okay),
        Err(e) => {
            let e = simplify_parse_error(e);
            print_parse_error(&contents, &e);
            panic!(e);
        }
    }
}

#[test]
fn test_no_regressions() {
    let a = vec![
        "../deps/corrode/src/Language/Rust/AST.hs",
        "../deps/corrode/src/Language/Rust/Corrode/C.lhs",
        "../deps/corrode/src/Language/Rust/Corrode/CFG.lhs",
        "../deps/corrode/src/Language/Rust/Corrode/CrateMap.hs",
        "../deps/corrode/src/Language/Rust/Idiomatic.hs",
        "../deps/corrode/src/Language/Rust.hs",

        "../deps/language-c/src/Language/C/Analysis/AstAnalysis.hs",
        "../deps/language-c/src/Language/C/Analysis/Builtins.hs",
        "../deps/language-c/src/Language/C/Analysis/ConstEval.hs",
        "../deps/language-c/src/Language/C/Analysis/Debug.hs",
        "../deps/language-c/src/Language/C/Analysis/DeclAnalysis.hs",
        "../deps/language-c/src/Language/C/Analysis/DefTable.hs",
        "../deps/language-c/src/Language/C/Analysis/Export.hs",
        "../deps/language-c/src/Language/C/Analysis/NameSpaceMap.hs",
        "../deps/language-c/src/Language/C/Analysis/SemError.hs",
        "../deps/language-c/src/Language/C/Analysis/SemRep.hs",
        "../deps/language-c/src/Language/C/Analysis/TravMonad.hs",
        "../deps/language-c/src/Language/C/Analysis/TypeCheck.hs",
        "../deps/language-c/src/Language/C/Analysis/TypeConversions.hs",
        "../deps/language-c/src/Language/C/Analysis/TypeUtils.hs",
        "../deps/language-c/src/Language/C/Analysis.hs",
        "../deps/language-c/src/Language/C/Data/Error.hs",
        "../deps/language-c/src/Language/C/Data/Ident.hs",
        "../deps/language-c/src/Language/C/Data/InputStream.hs",
        "../deps/language-c/src/Language/C/Data/Name.hs",
        "../deps/language-c/src/Language/C/Data/Node.hs",
        "../deps/language-c/src/Language/C/Data/Position.hs",
        "../deps/language-c/src/Language/C/Data/RList.hs",
        "../deps/language-c/src/Language/C/Data.hs",
        "../deps/language-c/src/Language/C/Parser/Builtin.hs",
        "../deps/language-c/src/Language/C/Parser/ParserMonad.hs",
        "../deps/language-c/src/Language/C/Parser/Tokens.hs",
        "../deps/language-c/src/Language/C/Parser.hs",
        "../deps/language-c/src/Language/C/Pretty.hs",
        "../deps/language-c/src/Language/C/Syntax/AST.hs",
        "../deps/language-c/src/Language/C/Syntax/Constants.hs",
        "../deps/language-c/src/Language/C/Syntax/Ops.hs",
        "../deps/language-c/src/Language/C/Syntax/Utils.hs",
        "../deps/language-c/src/Language/C/Syntax.hs",
        "../deps/language-c/src/Language/C/System/GCC.hs",
        "../deps/language-c/src/Language/C/System/Preprocess.hs",

        "../parser-c/gen/Lexer.hs",
        "../parser-c/gen/Parser.hs",
    ];

    for path in a {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        if path.ends_with(".lhs") {
            contents = strip_lhs(&contents);
        }
        let contents = parser_haskell::preprocess(&contents);

        // Do not output preprocessed data temp.txt
        println!("{:?}", path);
        // use ::std::io::Write;
        // let mut a = ::std::fs::File::create("temp.txt").unwrap();
        // a.write_all(contents.as_bytes());

        let mut errors = Vec::new();
        match parser_haskell::parse(&mut errors, &contents) {
            Ok(_) => {
                // OK
            }
            Err(e) => {
                //TODO print_parse_error return string, feed to panic
                print_parse_error(&contents, &simplify_parse_error(e));
                panic!("cannot convert file {:?}", path);
            }
        }
    }
}

fn strip_lhs(s: &str) -> String {
    let re = Regex::new(r"([ \t]*)```haskell([\s\S]*?)```").unwrap();
    let mut out = vec![];
    for cap in re.captures_iter(&s) {
        let indent = cap[1].to_string().len();
        let group = cap[2].to_string()
            .lines()
            .map(|x| {
                x.chars().skip(indent).collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        out.push(group);
    }

    out.join("\n\n")
}

/// Converts a Haskell file by its path into a Rust module.
fn convert_file(input: &str, p: &Path, inline_mod: bool, dump_ast: bool) -> Result<(String, String)> {
    let mut contents = input.to_string();
    let mut file_out = String::new();
    let mut rust_out = String::new();

    // Parse out HASKELL /HASKELL RUST /RUST sections.
    let re = Regex::new(r#"HASKELL[\s\S]*?/HASKELL"#).unwrap();
    contents = re.replace(&contents, "").to_string();
    let re = Regex::new(r#"RUST([\s\S]*?)/RUST"#).unwrap();
    if let Some(cap) = re.captures(&contents) {
        rust_out.push_str(&cap.get(1).unwrap().as_str().to_string());
    }
    contents = re.replace(&contents, "").to_string();

    // Preprocess the file.
    let contents = parser_haskell::preprocess(&contents);
    // errln!("{}", contents);

    // Parse the file.
    let mut errors = Vec::new();
    match parser_haskell::parse(&mut errors, &contents) {
        Ok(v) => {
            // errln!("{:?}", v);

            if dump_ast {
                println!("{}", format!("{:#?}", v).replace("    ", "  "));
            } else {
                writeln!(file_out, "// Original file: {:?}", p.file_name().unwrap())?;
                writeln!(file_out, "// File auto-generated using Corollary.")?;
                writeln!(file_out, "")?;

                if inline_mod {
                    writeln!(file_out, "pub mod {} {{", v.name.0.replace(".", "_"))?;
                    writeln!(file_out, "    use haskell_support::*;")?;
                    writeln!(file_out, "")?;
                    let state = PrintState::new();
                    writeln!(file_out, "{}", print_item_list(state.tab(), &v.items, true))?;
                    writeln!(file_out, "}}\n")?;
                } else {
                    writeln!(file_out, "#[macro_use] use corollary_support::*;")?;
                    writeln!(file_out, "")?;
                    let state = PrintState::new();
                    writeln!(file_out, "{}", print_item_list(state, &v.items, true))?;
                }
            }
        }
        Err(e) => {
            errln!("/* ERROR: cannot convert file {:?}" ,p);
            // TODO have this write to Format
            print_parse_error(&contents, &simplify_parse_error(e));
            errln!("*/");
            panic!("COULDN'T PARSE");
        }
    }

    Ok((file_out, rust_out))
}

quick_main!(run);

fn run() -> Result<()> {
    use std::io::Write;

    let matches = App::new("corollary")
        .version("0.1")
        .about("Converts Haskell to Rust")
        .arg(Arg::with_name("run")
            .short("r")
            .long("run")
            .help("Runs the file"))
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .help("Output path")
            .takes_value(true))
        .arg(Arg::with_name("ast")
            .short("a")
            .long("ast")
            .help("Dump AST"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let arg_input = matches.value_of("INPUT").unwrap();
    let arg_run = matches.is_present("run");
    let arg_out: Option<_> = matches.value_of("out");
    let arg_ast = matches.is_present("ast");

    if arg_run && arg_out.is_some() {
        bail!("Cannot use --out and --run at the same time.");
    }
    if (arg_run || arg_out.is_some()) && arg_ast {
        bail!("Cannot use --ast and (--run or --out) at the same time.");
    }

    // Starting message.
    if arg_run {
        errln!("running {:?}...", arg_input);
    } else {
        errln!("cross-compiling {:?}...", arg_input);
    }

    // Read file contents.
    let mut file = File::open(arg_input)
        .chain_err(|| format!("Could not open {:?}", arg_input))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Preprocess the file. Translate .lhs.
    if arg_input.ends_with(".lhs") {
        contents = strip_lhs(&contents);
    }
    let (mut file_section, rust_section) = convert_file(&contents, &PathBuf::from(arg_input), false, arg_ast)?;

    if arg_ast {
        return Ok(());
    }

    // Add Rust segments RUST ... /RUST and Haskell support code.
    let _ = writeln!(file_section, "");
    let _ = writeln!(file_section, "");
    if rust_section.len() > 0 {
        let _ = writeln!(file_section, "/* RUST ... /RUST */");
        let _ = writeln!(file_section, "{}", rust_section);
    }

    if let Some(out_path) = arg_out {
        // Create directory.
        let _ = create_dir_all(&Path::new(&arg_out.unwrap()).parent().unwrap());

        // Write file to path.
        errln!("... outputting to {:?}", out_path);
        let mut f = File::create(&out_path)?;
        let _ = f.write_all(file_section.as_bytes());
    } else if !arg_run {
        // Print file to stdout.
        print!("{}", file_section);
    } else if arg_run {
        // Run the file.
        let dir = TempDir::new("corollary")?;
        let file_path = dir.path().join("script.rs");

        let mut f = File::create(&file_path)?;
        let _ = f.write_all(b"// cargo-deps: corollary-support={path=\"/Users/trim/Desktop/corrode-but-in-rust/corollary-support\"}\n\nextern crate corollary_support;\n\n");
        let _ = f.write_all(file_section.as_bytes());
        if rust_section.len() == 0 {
            let _ = f.write_all(b"\n\nfn main() { let _ = __main(); }\n");
        }
        drop(f);

        let output = Command::new("cargo")
                    .args(&["script", &file_path.display().to_string()])
                    .output()
                    .expect("failed to execute process");

        if !output.status.success() {
            err!("{}", String::from_utf8_lossy(&output.stderr));
        }
        err!("{}", String::from_utf8_lossy(&output.stdout));
        ::std::process::exit(output.status.code().unwrap());
    }

    Ok(())
}
