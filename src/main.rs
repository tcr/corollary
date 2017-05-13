#[macro_use] extern crate maplit;
extern crate base64;
extern crate corroder_parser;
extern crate lalrpop_util;
extern crate regex;
extern crate walkdir;

use corroder_parser::ast;
use corroder_parser::ast::{Expr, Pat, Ty};
use corroder_parser::calculator;
use corroder_parser::util::{print_parse_error, simplify_parse_error};

use regex::{Regex, Captures};
use std::borrow::Borrow;
use std::io::prelude::*;
use std::fs::{File};
use std::env;

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

/// Convert indentation to something else.
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














#[derive(Clone, Copy)]
struct PrintState {
    pub level: i32,
}

impl PrintState {
    fn new() -> PrintState {
        PrintState {
            level: 0,
        }
    }

    fn tab(&self) -> PrintState {
        PrintState {
            level: self.level + 1
        }
    }

    fn untab(&self) -> PrintState {
        PrintState {
            level: if self.level == 0 { 0 } else { self.level - 1 }
        }
    }

    fn indent(&self) -> String {
        let mut out = String::new();
        for _ in 0..self.level {
            out.push_str("    ");
        }
        out
    }
}

fn expr_explode(span: Vec<Expr>) -> Vec<Expr> {
    if span.len() < 3 {
        return span;
    }
    for i in 0..span.len() {
        if let &ast::Expr::Operator(ref op) = &span[i] {
            return vec![ast::Expr::Op(
                Box::new(Expr::Span(expr_explode(span[0..i].to_vec().clone()))),
                op.to_string(),
                Box::new(Expr::Span(expr_explode(span[i+1..].to_vec().clone()))),
            )];
        }
    }
    span
}


fn print_expr(state: PrintState, expr: &ast::Expr) -> String {
    use ast::Expr::*;

    match *expr {
        Parens(ref r) => {
            let mut out = vec![];
            for item in r {
                out.push(print_expr(state, item));
            }
            format!("({})", out.join(", "))
        }
        Vector(ref r) => {
            let mut out = vec![];
            for item in r {
                out.push(print_expr(state, item));
            }
            format!("vec![{}]", out.join(", "))
        }
        Do(ref exprset, ref w) => {
            // where clause
            let mut out = vec![];
            if let &Some(ref stats) = w {
                out.push(print_statement_list(state, stats));
            }

            for (i, expr) in exprset.iter().enumerate() {
                let comm = if i == exprset.len() - 1 { "" } else { ";" };
                out.push(format!("{}{}{}", state.indent(), print_expr(state.tab(), expr), comm));
            }
            format!("{{\n{}\n{}}}", out.join("\n"), state.untab().indent())
        }
        Ref(ast::Ident(ref i)) => {
            format!("{}", i)
        }
        Number(n) => {
            format!("{}", n)
        }
        Op(ref l, ref op, ref r) => {
            if op == "$" {
                format!("{}({})", print_expr(state, l), print_expr(state, r))
            } else if op == "." {
                format!("({} . {})", print_expr(state, l), print_expr(state, r))
            } else if op == "<-" {
                format!("let {} = {}", print_expr(state, l), print_expr(state, r))
            } else {
                format!("{}({}, {})", op, print_expr(state, l), print_expr(state, r))
            }
        }
        Record(ref items) => {
            let mut out = vec![];
            for &(ref i, ref v) in items {
                out.push(format!("{}{:?} => {}",
                    state.indent(),
                    print_expr(state.tab().tab(), i),
                    print_expr(state.tab().tab(), v)));
            }
            format!("hashmap! {{\n{}\n{}}}", out.join(",\n"), state.indent())
        }
        Str(ref s) => {
            format!("{:?}.to_string()", String::from_utf8_lossy(&base64::decode(s).unwrap_or(b"\"\"".to_vec())))
        }
        Span(ref span) => {
            let span = expr_explode(span.clone());
            if span.len() == 1 {
                print_expr(state.tab(), &span[0])
            } else {
                if span.len() == 0 {
                    format!("()") //TODO WHAT
                } else {
                    // TODO
                    let mut span = span.clone();
                    let start = print_expr(state, &span.remove(0));
                    let mut end = "".to_string();
                    if span.len() > 0 {
                        let mut out = vec![];
                        for item in &span {
                            out.push(print_expr(state.tab(), item));
                        }
                        end = format!("({})", out.join(", "));
                    }
                    format!("{}{}", start, end)
                }
            }
        }
        Case(ref cond, ref rest) => {
            let mut out = vec![];
            for item in rest {
                match item.clone() {
                    ast::CaseCond::Matching(label, arms) => {
                        let mut inner = vec![];
                        for (cond, arm) in arms {
                            inner.push(format!("{} {{ {} }}",
                                print_expr(state, &cond),
                                print_expr(state, &arm),
                            ));
                        }
                        out.push(format!("{}{} => if {},",
                            state.indent(),
                            print_patterns(state, label),
                            inner.join("\n")));
                    }
                    ast::CaseCond::Direct(label, arm) => {
                        out.push(format!("{}{} => {},",
                            state.tab().indent(),
                            print_patterns(state, label),
                            print_expr(state.tab(), &arm)));
                    }
                }
            }
            format!("match {} {{\n{}\n{}}}", print_expr(state.tab(), cond), out.join("\n"), state.indent())
        }
        ref expr => {
            format!("{:?}", expr)
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

fn print_pattern(state: PrintState, pat: &Pat) -> String {
    match *pat {
        Pat::Ref(ast::Ident(ref s)) => s.to_string(),
        Pat::Span(ref span) => {
            let mut out_span = print_pattern(state.tab(), &span[0]);
            if span.len() > 1 {
                out_span.push_str(&format!("({})", print_patterns(state.tab(), &span[1..])));
            }
            out_span
        }
        Pat::Str(ref s) => {
            let decoded = &base64::decode(s)
                .map(|vec| String::from_utf8_lossy(&vec).into_owned())
                .unwrap_or_else(|_| "@@WEIRD BAD BASE64 STR@@".to_string());
            format!("{:?}", decoded)
        }
        Pat::Num(n) => format!("{}", n),
        Pat::Tuple(ref pats) => {
            if pats.len() == 1 {
                print_pattern(state.tab(), &pats[0])
            } else {
                format!("({})", print_patterns(state.tab(), pats))
            }
        }
        Pat::Brackets(ref pats) => {
            format!("[{}]", print_patterns(state.tab(), pats))
        }
        Pat::Arrow(ast::Ident(ref s), ref p) => {
            format!("({} -> {})", s, print_pattern(state.tab(), &**p))
        }
        Pat::Not(ref s) => print_pattern(state, &**s),
        Pat::EmptyParen => format!("()"),
        Pat::Dummy => format!("<todo>"),
    }
}


fn print_type<T: Borrow<Ty>>(state: PrintState, t: T) -> String {
    match *t.borrow() {
        Ty::Ref(ast::Ident(ref s)) => {
            if s == "Int" {
                format!("isize")
            } else {
                s.to_string()
            }
        }
        Ty::Not(ref s) => {
            print_type(state, &**s)
        }
        Ty::Span(ref span) => {
            let mut out_span = print_type(state.tab(), &span[0]);
            if span.len() > 1 {
                out_span.push_str(&format!("({})", print_types(state.tab(), &span[1..])));
            }
            out_span
        }
        Ty::Tuple(ref spans) => {
            if spans.len() == 1 {
                print_type(state.tab(), &spans[0])
            } else {
                format!("({})", print_types(state.tab(), spans))
            }
        }
        Ty::Brackets(ref t) => {
            format!("Vec<{}>", print_type(state.tab(), &**t))
        }
        Ty::Where(_, ref t) => print_type(state, &**t), // temp
        Ty::Pair(ref a, ref b) => {
            format!("fn({}) -> {}", print_type(state, &**a), print_type(state, &**b))
        }
        Ty::RecordTODO => "{ /* struct def */ }".to_string(),
        Ty::EmptyParen => "()".to_string(),
    }
}

fn print_types<I, T>(state: PrintState, iter: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Borrow<Ty>,
{
    iter.into_iter().map(|t| print_type(state, t)).collect::<Vec<_>>().join(", ")
}

fn print_patterns<I, T>(state: PrintState, iter: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Borrow<Pat>,
{
    iter.into_iter().map(|p| print_pattern(state, p.borrow())).collect::<Vec<_>>().join(", ")
}

fn print_statement_list(state: PrintState, stats: &[ast::Statement]) -> String {
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

    // Print out data structures.
    for item in stats {
        if let ast::Statement::Data(name, data, derives) = item.clone() {
            let derives = if let Some(d) = derives {
                format!("#[derive({})]\n    ", d.iter().map(|x| {
                    if x.0 == "Data" {
                        format!("Clone")
                    } else if x.0 == "Typeable" {
                        format!("Debug")
                    } else {
                        x.0.to_string()
                    }
                }).collect::<Vec<_>>().join(", "))
            } else {
                format!("")
            };
            println!("    {}struct {}({});",
                derives,
                name.0,
                data.iter().map(|tyset| {
                    print_types(state, tyset)
                }).collect::<Vec<_>>().join(", "));
            println!("");
        }
    }

    // Print out assignments as fns
    let mut cache = btreemap![];
    for item in stats {
        if let ast::Statement::Assign(s, args, expr) = item.clone() {
            //if !types.contains_key(&s) {
            //    println!("this shouldn't happen {:?}", s);
            //}
            //if cache.contains_key(&s) {
            //    panic!("this shouldn't happen {:?}", s);
            //}
            cache.entry(print_pattern(PrintState::new(), &s)).or_insert(vec![]).push((args, expr));
        }
    }

    // Comprss guards
    let mut new_cache = btreemap![];
    for (key, fnset) in cache {
        if fnset.len() > 1 {
            let args = (0..fnset[0].0.len()).map(|x| format!("__{}", x)).collect::<Vec<_>>();
            new_cache.insert(key, vec![(
                args.iter()
                    .map(|x| Pat::Ref(ast::Ident(x.to_string())))
                    .collect::<Vec<_>>(),
                ast::Expr::Case(
                    Box::new(ast::Expr::Parens(args.iter()
                        .map(|x| ast::Expr::Ref(ast::Ident(x.to_string())))
                        .collect::<Vec<_>>())),
                    fnset.iter().map(|x| {
                        ast::CaseCond::Direct(x.0.clone(), x.1.clone())
                    }).collect::<Vec<_>>(),
                ),
            )]);
        } else {
            new_cache.insert(key, fnset);
        }
    }

    let mut out = vec![];
    for (key, fnset) in new_cache {
        for (args, expr) in fnset {
            // For type-less functions,
            if !types.contains_key(&key) {
                // fallback to printing a lambda
                out.push(
                    format!("{}let {} = |{}| {{\n{}{}\n{}}};\n",
                        state.indent(),
                        key,
                        print_patterns(state, args),
                        state.tab().indent(),
                        print_expr(state.tab(), &expr),
                        state.indent()));
                continue;
            }

            let d = types[&key].clone();
            assert!(d.len() == 1);
            let t = unpack_fndef(d[0].clone());
            assert!(t.len() >= 1);

            //println!("hm {:?}", types[&key]);
            //println!("hm {:?}", t);
            let mut args_span = vec![];
            for (arg, ty) in args.iter().zip(t.iter()) {
                args_span.push(format!("{}: {}", print_pattern(state, arg), print_type(state.tab(), ty)));
            }
            out.push(
                format!("{}fn {}({}) -> {} {{\n{}{}\n{}}}\n",
                    state.indent(),
                    key,
                    args_span.join(", "),
                    print_type(state.tab(), t.last().unwrap()),
                    state.tab().indent(),
                    print_expr(state.tab(), &expr),
                    state.indent()));
        }
    }

    out.join("\n")
}


#[test]
fn calculator() {
    // let a = "./corrode/src/Language/Rust/Corrode/CrateMap.hs";
    let a = "language-c/src/Language/C/Analysis/AstAnalysis.hs";
    // let a = "./test/input.hs";
    println!("file: {}", a);
    let mut file = File::open(a).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let input = commify(&contents);
    let mut errors = Vec::new();
    match calculator::parse_Module(&mut errors, &input) {
        Ok(okay) => println!("{:#?}", okay),
        Err(e) => {
            let e = simplify_parse_error(e);
            print_parse_error(&input, &e);
            panic!(e);
        }
    }
}

fn fix_lhs(s: &str) -> String {
    let re = Regex::new(r"```haskell([\s\S]*?)```").unwrap();
    let mut out = vec![];
    for cap in re.captures_iter(&s) {
        out.push(cap[1].to_string());
    }

    out.join("\n\n")
}


#[cfg(not(test))]
fn main() {
    let dir = match env::args().nth(1) {
        Some(s) => s,
        _ => {
            panic!("Usage: cargo run <dir>");
        }
    };

    for entry in WalkDir::new(dir) {
        let e = entry.unwrap();
        let p = e.path();
        let mut do_fix_lhs = false;
        if p.display().to_string().ends_with(".lhs") {
            do_fix_lhs = true;
        } else if !p.display().to_string().ends_with(".hs") {
            continue;
        }


        let mut file = File::open(p).unwrap();
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(..) => (),
            _ => continue,
        };

        if do_fix_lhs {
            contents = fix_lhs(&contents);
        }

        let input = commify(&contents);
        let mut errors = Vec::new();
        match calculator::parse_Module(&mut errors, &input) {
            Ok(v) => {
                println!("mod {} {{", v.name.0.replace(".", "_"));
                let state = PrintState::new();
                println!("{}", print_statement_list(state.tab(), &v.statements));
                println!("}}\n");
            }
            Err(e) => {
                println!("/* ERROR: cannot yet convert file {:?}", p);
                print_parse_error(&input, &simplify_parse_error(e));
                println!("*/");
            }
        }
    }
    println!("");
    println!("");
    println!("fn main() {{ /* demo */ }}")
}
