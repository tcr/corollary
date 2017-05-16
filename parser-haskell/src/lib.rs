extern crate lalrpop_util;
extern crate regex;
extern crate base64;

pub mod ast;
pub mod haskell;
pub mod util;

use regex::{Captures, Regex};

// TODO rename "strip comments and other stuff too i guess"
fn strip_comments(text: &str) -> String {
    // Strip comments
    let re = Regex::new(r"--[^\n\r]*").unwrap();
    let text = re.replace_all(&text, "").to_string();
    let re = Regex::new(r"\{-[\s\S]*?-\}").unwrap();
    let text = re.replace_all(&text, "").to_string();

    // Strip trailing semicolons (so we don't have "empty statements")
    let re = Regex::new(r"(?m);+\s*$").unwrap();
    let text = re.replace_all(&text, "").to_string();

    // Strip preprocessor decls
    let re = Regex::new(r"(?m)^#(if|ifn?def|endif|else).*").unwrap();
    let text = re.replace_all(&text, "").to_string();

    // TODO this should be handled in the parser
    let escape_re = Regex::new(r#"\\([abfnrtv'"\\0]|NUL|ESC)"#).unwrap();
    let decode_escapes = |text: &str| {
        let text = escape_re.replace_all(text, |caps: &Captures| {
            match &caps[1] {
                "a" => "\u{0007}",
                "b" => "\u{0008}",
                "f" => "\u{000C}",
                "n" => "\n",
                "r" => "\r",
                "t" => "\t",
                "v" => "\u{000B}",
                "'" => "'",
                "\"" => "\"",
                "\\" => "\\",
                "0" | "NUL" => "\0",
                "ESC" => "\x1b",
                s => panic!("str escape {}", s),
            }.into()
        });
        text.to_string()
    };

    // Char literals.
    let re = Regex::new(r"'([^'\\]|\\[A-Z]{1,3}|\\.)'").unwrap();
    let text = re.replace_all(&text, |caps: &Captures| {
        let v = decode_escapes(&caps[1]);
        assert!(v.len() == 1, "multi char literal {:?}", v);
        format!("'{}'", base64::encode(&v))
    }).to_string();

    // Replace all strings with a base64 encoded version to make the parser simpler.
    // If its possible to get LALRPOP to not complain with proper string regexes, should just use
    // that instead
    let re = Regex::new(r#""(([^"\\]|\\.)*?)""#).unwrap();
    let text = re.replace_all(&text, |caps: &Captures| {
        let v = decode_escapes(&caps[1]);
        format!("\"{}\"", base64::encode(&v))
    }).to_string();

    text
}

fn decode_literal(s: &str) -> String {
    let vec = base64::decode(s).expect(&format!("invalid base64: {:?}", s));
    String::from_utf8(vec).expect("invalid UTF-8")
}

fn word_is_block_word(word: &str) -> bool {
    word == "do" || word == "where" || word == "of" || word == "let"
}

/// Convert indentation to semicolon-delimited brackets, so it can be parsed more easily.
fn commify(val: &str) -> String {
    let re_space = Regex::new(r#"^[ \t]+"#).unwrap();
    let re_nl = Regex::new(r#"^\r?\n"#).unwrap();
    let re_word = Regex::new(r#"([\(\{\[\]\}\)]|[^ \t\r\n\(\{\[\]\}\)]+)"#).unwrap();

    // Strip out all comments from the contents.
    let commentless = strip_comments(val);

    // Previous indentation levels
    let mut stash: Vec<usize> = vec![];
    // Previous brace nesting levels.
    let mut braces: Vec<isize> = vec![];
    // Previous word was a block starting word, option containing its indent level.
    let mut trigger = None;
    // How many spaces to indent.
    let mut indent = 0;
    // Check if this is the first word in the line.
    let mut first = true;

    let mut out = String::new();
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
                    if let Some(last_level) = stash.last().map(|x| *x) {
                        // Check if we decreased our indent level
                        last_level > indent
                    } else {
                        false
                    }
                } {
                    // out.push_str(&format!("[{:?}{:?}]", last_level, stash.last()));
                    stash.pop();
                    braces.pop();
                    out.push_str("}");
                }

                if let Some(i) = stash.last() {
                    if *i == indent && trigger.is_none() {
                        out.push_str(";");
                    }
                }
            }

            if ["]", ")", "}"].contains(&word) {
                if let Some(brace) = braces.last_mut() {
                    *brace -= 1;
                }
            }
            if ["[", "(", "{"].contains(&word) {
                if let Some(brace) = braces.last_mut() {
                    *brace += 1;
                }
            }

            // End braces insertion when meeting an unbalanced ending ), }, or ]
            while {
                if let Some(brace) = braces.last().map(|x| *x) {
                    brace < 0
                } else {
                    false
                }
            } {
                stash.pop();
                braces.pop();
                out.push_str("}");
            }

            out.push_str(word);
            v = &v[word.len()..];

            if trigger.is_some() {
                // The next word after a block word is where the whitespace column begins.
                stash.push(indent);
            }
            first = false;

            trigger = if word_is_block_word(word) { Some(indent) } else { None };
            if trigger.is_some() {
                out.push_str("{");

                // Trace brace indentation level.
                braces.push(0);
            }

            indent += word.len();
        } else {
            unreachable!("unknown prop {:?}", v);
        }
    }
    for _ in 0..stash.len() {
        out.push_str("}");
    }


    // Replace trailing commas after where statements
    // TODO fix this in the parser instead
    let re = Regex::new(r#"where\s+;"#).unwrap();
    let out = re.replace_all(&out, r#"where "#).to_string();
    // let re = Regex::new(r#"\};\s*where\b"#).unwrap();
    // let out = re.replace_all(&out, r#"} where"#).to_string();
    let re = Regex::new(r#"\};\}"#).unwrap();
    let out = re.replace_all(&out, r#"}}"#).to_string();

    out
}

/// Preprocess code to remove comments and convert whitepsace to brace blocks.
/// TODO: merge this into parse() below once result lifetimes can be worked out
pub fn preprocess(input: &str) -> String {
    commify(input)
}

/// Entry point for parsing modules
pub fn parse<'input, 'err>(
    errors: &'err mut Vec<lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str
) -> Result<ast::Module, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    haskell::parse_Module(errors, &input)
}
