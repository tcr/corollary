extern crate lalrpop_util;
extern crate regex;
extern crate walkdir;

pub mod ast;
pub mod calculator;

use regex::Regex;
use std::io::prelude::*;
use std::fs::{File};
use std::fmt::Debug;

use lalrpop_util::{ParseError};
use walkdir::WalkDir;

fn strip_comments(text: &str) -> String {
    let re = Regex::new(r"--[^\n\r]*").unwrap();
    let text = re.replace_all(&text, "").to_string();

    let re = Regex::new(r"\{-[\s\S]*?-\}").unwrap();
    let text = re.replace_all(&text, "").to_string();

    let re = Regex::new(r"(?m);+\s*$").unwrap();
    let text = re.replace_all(&text, "").to_string();

    let re = Regex::new(r"(?m)^#(if|ifn?def|endif|else).*").unwrap();
    let text = re.replace_all(&text, "").to_string();

    let re = Regex::new(r#"'(\\.|[^']|\\ESC)'"#).unwrap();
    let text = re.replace_all(&text, r#"'0'"#).to_string();

    let re = Regex::new(r#""([^"\\]|\\.)*?""#).unwrap();
    let text = re.replace_all(&text, r#""1""#).to_string();

    text
}

pub fn codelist(code: &str) {
    for (i, line) in code.lines().enumerate() {
        println!("{:>3} | {}", i+1, line);
    }
}

pub fn code_error(code: &str, tok_pos: usize) {
    let code = format!("\n\n{}", code);
    let code = code.lines().collect::<Vec<_>>();
    let mut pos: isize = 0;
    for (i, lines) in (&code[..]).windows(3).enumerate() {
        if pos + lines[2].len() as isize >= tok_pos as isize {
            if i > 1 {
                println!("{:>3} | {}", i - 1, lines[0]);
            }
            if i > 0 {
                println!("{:>3} | {}", i, lines[1]);
            }
            println!("{:>3} | {}", i + 1, lines[2]);

            println!("{}^", (0..(tok_pos as isize) - (pos - 6)).map(|_| "~").collect::<String>());
            return;
        }
        pos += (lines[2].len() as isize) + 1;
    }
}

pub fn parse_results<C,T,E>(code: &str, res: Result<C, ParseError<usize,T,E>>) -> C
where C: Debug, T: Debug, E: Debug {
    match res {
        Ok(value) => {
            return value;
        }
        Err(ParseError::InvalidToken {
            location: loc
        }) => {
            println!("Error: Invalid token:");
            code_error(code, loc);
            panic!("{:?}", res);
        }
        Err(ParseError::UnrecognizedToken {
            token: Some((loc, _, _)),
            ..
        }) => {
            println!("Error: Unrecognized token:");
            code_error(code, loc);
            panic!("{:?}", res);
        }
        err => {
            panic!("{:?}", err);
        }
    }
}


#[test]
fn calculator() {
    let a = "./corrode/src/Language/Rust/Idiomatic.hs";
    println!("file: {}", a);
    let mut file = File::open(a).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let input = commify(&contents);
    let mut errors = Vec::new();
    {
        let okay = parse_results(&input, calculator::parse_Module(&mut errors, &input));
        println!("{:?}", okay);
    }
}

#[cfg(not(test))]
fn main() {
    for entry in WalkDir::new("./language-c/src/Language/C") {
    //for entry in WalkDir::new("./corrode/src/Language") {
        let e = entry.unwrap();
        let p = e.path();
        let mut file = File::open(p).unwrap();
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(..) => (),
            _ => continue,
        };

        let input = commify(&contents);
        let mut errors = Vec::new();
        if let Ok(v) = calculator::parse_Module(&mut errors, &input) {
            println!("SUCCESS - {:?}", p);

            println!("v {:?}", v);
        } else {
            println!("ERROR   - {:?}", p);
        }
    }
}

fn commify(val: &str) -> String {
    let re_space = Regex::new(r#"^[ \t]+"#).unwrap();
    let re_nl = Regex::new(r#"^\r?\n"#).unwrap();
    let re_word = Regex::new(r#"[^ \t\r\n]+"#).unwrap();

    let mut out = String::new();

    let mut stash = vec![];
    let mut trigger = false;
    let mut indent = 0;
    let mut first = true;

    let commentless = strip_comments(val);
    let mut v: &str = &commentless;
    while v.len() > 0 {
        if let Some(cap) = re_space.captures(v) {
            let word = &cap[0];
            out.push_str(word);
            v = &v[word.len()..];

            indent += word.len();
        } else if let Some(cap) = re_nl.captures(v) {
            let word = &cap[0];
            out.push_str(word);
            v = &v[word.len()..];

            indent = 0;
            first = true;
            if stash.len() > 1 {
                for _ in &stash[1..] {
                    out.push_str(" ");
                }
            }
        } else if let Some(cap) = re_word.captures(v) {
            let word = &cap[0];

            if first {
                while {
                    if let Some(i) = stash.last() {
                        *i > indent
                    } else {
                        false
                    }
                } {
                    stash.pop();
                    out.push_str("}");
                }

                if let Some(i) = stash.last() {
                    if *i == indent {
                        out.push_str(";");
                    }
                }
            }
            first = false;

            if trigger {
                out.push_str("{");
            }
            out.push_str(word);
            v = &v[word.len()..];

            if trigger {
                stash.push(indent);
            }

            indent += word.len();

            if word == "do" || word == "where" || word == "of" || word == "let" {
                trigger = true;
            } else {
                trigger = false;
            }
        } else {
            panic!("unknown prop {:?}", v);
        }
    }
    for _ in 0..stash.len() {
        out.push_str("}");
    }


    let re = Regex::new(r#"where\s+;"#).unwrap();
    let out = re.replace_all(&out, r#"where "#).to_string();

    out
}
