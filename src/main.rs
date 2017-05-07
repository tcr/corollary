#[macro_use] extern crate maplit;
extern crate base64;
extern crate corroder_parser;
extern crate lalrpop_util;
extern crate regex;
extern crate walkdir;

use corroder_parser::ast;
use corroder_parser::ast::Ty;
use corroder_parser::calculator;

use regex::{Regex, Captures};
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
    let text = re.replace_all(&text, |caps: &Captures| {
        let v = &caps[0][1..caps[0].len()-1];
        format!("\"{}\"", base64::encode(v))
    }).to_string();

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
    let a = "./language-c/src/Language/C/System/Preprocess.hs";
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






fn print_expr(expr: &ast::Expr) -> String {
    use ast::Expr::*;

    match *expr {
        Parens(ref r) => {
            let mut out = vec![];
            for item in r {
                out.push(print_expr(item));
            }
            out.join(", ")
        }
        Do(ref exprset, ref w) => {
            // where clause
            let mut out = vec![];
            if let &Some(ref stats) = w {
                out.push(print_statement_list(stats));
            }

            for expr in exprset {
                out.push(format!("{};", print_expr(expr)));
            }
            format!("{{\n{}\n}}", out.join("\n"))
        }
        Ref(ast::Ident(ref i)) => {
            format!("{}", i)
        }
        Number(n) => {
            format!("{}", n)
        }
        Op(ref l, op, ref r) => {
            format!("({:?} {:?} {:?})", l, op, r)
        }
        Record(ref items) => {
            let mut out = vec![];
            for &(ast::Ident(ref i), ref v) in items {
                out.push(format!("{:?} => {}", i, print_expr(v)));
            }
            format!("hashmap! {{\n{}\n}}", out.join(", "))
        }
        Str(ref s) => {
            format!("{:?}.to_string()", String::from_utf8_lossy(&base64::decode(s).unwrap_or(b"\"\"".to_vec())))
        }
        Span(ref span) => {
            if span.len() == 1 {
                print_expr(&span[0])
            } else {
                // TODO
                let mut span = span.clone();
                let start = print_expr(&span.remove(0));
                let mut end = "".to_string();
                if span.len() > 0 {
                    let mut out = vec![];
                    for item in &span {
                        out.push(print_expr(item));
                    }
                    end = format!("({})", out.join(", "));
                }
                format!("{}{}", start, end)
            }
        }
        ref expr => {
            format!("{:?}", expr)
        }
    }
}

fn print_stat(stat: &ast::Statement) -> String {
    use ast::Statement::*;
    match stat {
        &Assign(ast::Ident(ref i), ref args, ref e) => {
            format!("        let {} = {}\n", i, print_expr(e))
        }
        _ => {
            "???".to_string()
        }
    }
}

fn unpack_fndef(t: Ty) -> Vec<Ty> {
    match t {
        Ty::Pair(a, b) => {
            let mut v = vec![*a];
            v.extend(unpack_fndef(*b));
            v
        }
        _ => {
            vec![t]
        }
    }
}

fn print_type(t: Ty) -> String {
    match t {
        Ty::Ref(ast::Ident(ref s)) => {
            s.to_string()
        }
        Ty::Span(mut span) => {
            let mut out_span = print_type(span.remove(0));
            if span.len() > 0 {
                let mut type_span = vec![];
                for item in span {
                    type_span.push(print_type(item));
                }
                out_span.push_str(&format!("<{}>", type_span.join(", ")))
            }
            out_span
        }
        Ty::Tuple(spans) => {
            if spans.len() == 1 {
                print_type(spans[0].clone())
            } else {
                format!("({})", spans.into_iter()
                    .map(print_type)
                    .collect::<Vec<_>>()
                    .join(", "))
            }
        }
        Ty::Brackets(spans) => {
            format!("Vec<{}>", spans.into_iter()
                .map(print_type)
                .collect::<Vec<_>>()
                .join(", "))
        }
        t => {
            format!("{:?}", t)
        }
    }
}

fn print_statement_list(stats: &[ast::Statement]) -> String {
    let mut types = btreemap![];
    for item in stats {
        // println!("well {:?}", item);
        if let ast::Statement::Prototype(ast::Ident(s), d) = item.clone() {
            if types.contains_key(&s) {
                panic!("that shouldn't happen {:?}", s);
            }
            types.insert(s, d);
        }
    }

    // Print out assignments as fns
    let mut cache = btreemap![];
    for item in stats {
        if let ast::Statement::Assign(ast::Ident(s), args, expr) = item.clone() {
            //if !types.contains_key(&s) {
            //    println!("this shouldn't happen {:?}", s);
            //}
            //if cache.contains_key(&s) {
            //    panic!("this shouldn't happen {:?}", s);
            //}
            cache.entry(s).or_insert(vec![]).push((args, expr));
        }
    }

    let mut out = vec![];
    for (key, fnset) in cache {
        for (args, expr) in fnset {
            if !types.contains_key(&key) {
                // fallback to printing a lambda
                out.push(
                    format!("    let {} = |{}| {{\n        {}\n    }};",
                        key,
                        args.iter().map(|x| x.0.to_string()).collect::<Vec<_>>().join(", "),
                        print_expr(&expr)));
                continue;
            }

            let d = types[&key].clone();
            assert!(d.len() == 1);
            let t = unpack_fndef(d[0].clone());
            assert!(t.len() >= 1);

            //println!("hm {:?}", types[&key]);
            //println!("hm {:?}", t);
            let mut args_span = vec![];
            for (&ast::Ident(ref arg), ty) in args.iter().zip(t.iter()) {
                args_span.push(format!("{}: {}", arg, print_type(ty.clone())));
            }
            out.push(
                format!("    fn {}({}) -> {} {{\n        {}\n    }};\n",
                    key,
                    args_span.join(", "),
                    print_type(t.last().unwrap().clone()),
                    print_expr(&expr)));
        }
    }

    out.join("\n")
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
            //continue;
            println!("mod {:?} {{", p);
            println!("{}", print_statement_list(&v.statements));
            println!("}}\n\n\n");
        } else {
            println!("ERROR   - {:?}\n\n\n", p);
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
