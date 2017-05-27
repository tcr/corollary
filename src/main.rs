#![allow(unused_imports)]

#[macro_use] extern crate errln;
#[macro_use] extern crate maplit;
extern crate clap;
extern crate hex;
extern crate lalrpop_util;
extern crate parser_haskell;
extern crate regex;
extern crate tempdir;
extern crate walkdir;

mod ir;

use parser_haskell::ast;
use parser_haskell::ast::{Expr, Pat, Ty};
use parser_haskell::util::{print_parse_error, simplify_parse_error};

use clap::{Arg, App, SubCommand};
use hex::*;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write;
use std::fs::{File};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;
use walkdir::WalkDir;

use ir::PrintState;

// Expand a sequence of expression terms into a nested tree of operators
// TODO Operator precedence rules should be applied here
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

fn print_ident(_: PrintState, mut expr: String) -> String {
    // Handle keywords here
    if expr == "mut" {
        return "__mut".to_string()
    } else if expr == "error" {
        return "__error!".to_string()
    } else if expr == "str" {
        return "__str".to_string()
    } else if expr == "const" {
        return "__TODO_const".to_string()
    } else if expr == "else" {
        return "__TODO_else".to_string()
    } else if expr == "if" {
        return "__TODO_if".to_string()
    } else if expr == "@" {
        return "__TODO_at".to_string()
    } else if expr == "ref" {
        return "__ref".to_string()
    } else if expr == "static" {
        return "__static".to_string()
    } else if expr == "enum" {
        return "__enum".to_string()
    } else if expr == "use" {
        return "__use".to_string()
    } else if expr == "mod" {
        return "__mod".to_string()
    }

    if expr.find(":").is_some() {
        // Print ..:XXX sequences as hex
        let pos = expr.find(":").unwrap();
        expr = format!("{}__id_{}", &expr[0..pos], (&expr[pos..]).to_hex())
    }
    expr.replace("'", "_q").replace(".", "::")
}


fn print_type_ident(state: PrintState, s: &str) -> String {
    // Handle common translations for types here
    match s {
        "Int" => format!("isize"),
        "Nothing" => format!("None"),
        "Just" => format!("Some"),
        "Maybe" => format!("Option"),
        "True" => format!("true"),
        "False" => format!("false"),
        "Bool" => format!("bool"),
        _ => print_ident(state, s.to_string()),
    }
}

fn print_op_fn(value: &str) -> String {
    // Here you add new infix operators.
    match value {
        "++" => "__op_addadd".to_string(),
        ":" => "__op_concat".to_string(),
        ">>=" => "__op_bind".to_string(),
        ">>" => "__op_rshift".to_string(),
        "<<" => "__op_lshift".to_string(),
        "<+>" => "__op_doc_conat".to_string(),
        "$+$" => "__op_line_concat".to_string(),
        "$$" => "__op_line_something".to_string(), // TODO
        "$!" => "__op_TODO_dollarnot".to_string(),
        ".&." => "__op_dotted_and".to_string(),
        ".|." => "__op_dotted_or".to_string(),
        "/=" => "__op_assign_div".to_string(),
        "+=" => "__op_assign_add".to_string(),
        "-=" => "__op_assign_sub".to_string(),
        "*=" => "__op_assign_mul".to_string(),
        "^" => "__op_power".to_string(),
        "<>" => "__op_ne".to_string(),
        "\\\\" => "__op_forwardslash".to_string(), // TODO
        _ => value.to_string()
    }
}

/// Backwards compatibility.
fn print_expr(state: PrintState, expr: &ast::Expr) -> String {
    let expr = convert_expr(state, expr);
    format!("{}", ir::Printer { state: state, out: expr })
}

/// Converts several Haskell expresions to a vector of Rust expressions.
fn convert_exprs<'a, I>(state: PrintState, exprs: I) -> Vec<ir::Expr>
where
    I: IntoIterator<Item = &'a ast::Expr>
{
    exprs.into_iter().map(|e| convert_expr(state, e)).collect()
}

/// Converts a Haskell expression to Rust.
fn convert_expr(state: PrintState, expr: &ast::Expr) -> ir::Expr {
    use ast::Expr::*;

    let freeform = match *expr {
        Parens(ref r) => {
            let mut out = vec![];
            for item in r {
                out.push(print_expr(state, item));
            }
            format!("({})", out.join(", "))
        }
        Vector(ref r) => {
            let exprs = convert_exprs(state, r);
            // memoize line length
            let line_length = exprs.iter().fold(Some(2), |sum, e| {
                sum.and_then(|n| e.line_length().map(|len| n + len + 2))
            });
            return ir::Expr::VecLiteral { exprs, line_length };
        }
        Do(ref stmts) => print_do(state.tab(), stmts),
        Let(ref assigns, ref expr) => {
            let mut out = assigns.iter().map(|a| print_let(state.tab(), a)).collect::<Vec<_>>();
            out.push(format!("{}{}", state.indent(), print_expr(state.tab(), expr)));
            // todo: should be possible to inline this extra {} in most scenarios
            format!("{{\n{}{}}}", out.join("\n"), state.indent())
        }
        Ref(ast::Ident(ref i)) => {
            print_type_ident(state, i)
        }
        Number(n) => return ir::Expr::Number(n),
        Op(ref l, ref op, ref r) => {
            if op == "&&"
                || op == "=="
                || op == "*"
                || op == "+"
                || op == "-"
                || op == "||"
                || op == ">"
                || op == "<"
                || op == ">="
                || op == "<=" {
                format!("({} {} {})", print_expr(state, l), op, print_expr(state, r))
            } else if op == "$" {
                format!("{}({})", print_expr(state, l), print_expr(state, r))
            } else if op == "." {
                let l: Expr = (**l).clone();
                let r: Expr = (**r).clone();

                // Dot operator. (f . g) x = f(g(x)
                //TODO this is conditional on overcomplicated AST of spans and parens but easily
                // might change in the future
                if let &Expr::Span(ref left) = &l {
                    if let &Expr::Parens(ref span) = &left[0] {
                        if let &Expr::Span(ref innerspan) = &span[0] {
                            // --> (a b c) ...
                            let mut innerspan = innerspan.clone();
                            //TODO what about span[1..]
                            innerspan.push(r);
                            format!("{}", print_expr(state, &Expr::Span(vec![Expr::Parens(vec![Expr::Span(innerspan)])])))
                        } else {
                            panic!("WHAT {:?}", l);
                            //format!("({} . {})", print_expr(state, &l), print_expr(state, &r))
                        }
                    } else if let &Expr::Ref(..) = &left[0] {
                        // --> a ...
                        let mut outer = left.clone();
                        outer.push(r);
                        format!("{}", print_expr(state, &Expr::Span(outer)))
                    } else {
                        panic!("WHAT {:?}", l);
                    }
                } else {
                    panic!("WHAT {:?}", l);
                }

            } else if op == "<-" {
                format!("let {} = {}", print_expr(state, l), print_expr(state, r))
            } else {
                let new_op = print_op_fn(op);
                format!("{}({}, {})", new_op, print_expr(state, l), print_expr(state, r))
            }
        }
        Record(ref items) => {
            let mut out = vec![];
            for &(ast::Ident(ref i), ref v) in items {
                out.push(format!("{}{}: {}",
                    state.indent(),
                    i,
                    print_expr(state.tab().tab(), v)));
            }
            format!("{{\n{}\n{}}}", out.join(",\n"), state.untab().indent())
        }
        Str(ref s) => return ir::Expr::StrLiteral(s.clone()),
        Char(ref s) => {
            assert!(s.len() == 1, "char lit {:?}", s);
            format!("{:?}", s.chars().next().unwrap())
        }
        Span(ref span) => {
            let span = expr_explode(span.clone());
            if span.len() == 2 && ({
                if let ast::Expr::Record(..) = span[1] {
                    true
                } else { false }
            }) {
                format!("{} {}",
                    print_expr(state, &span[0].clone()),
                    print_expr(state, &span[1].clone()))
            } else if span.len() == 1 {
                print_expr(state, &span[0])
            } else {
                if span.len() == 0 {
                    format!("()") //TODO not sure what this would be?
                } else {
                    // Check for return() here, for now
                    if print_expr(state, &span[0]) == "return" {
                        //TODO handle return more intelligently
                        print_expr(state, &Expr::Span(span[1..].to_vec()))
                    } else {
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
        }
        Case(ref cond, ref rest) => {
            let mut out = vec![];
            for item in rest {
                match item.clone() {
                    ast::CaseCond::Matching(label, arms) => {
                        let mut inner = vec![];
                        for (cond, arm) in arms {
                            inner.push(format!("{} {{ {} }}",
                                cond.iter().map(|x| print_expr(state, x)).collect::<Vec<_>>().join(" && "),
                                print_expr(state, &arm),
                            ));
                        }
                        out.push(format!("{}{} => if {},",
                            state.tab().indent(),
                            print_case_patterns(state, label),
                            inner.join("\n")));
                    }
                    ast::CaseCond::Direct(labels, arms) => {
                        out.push(format!("{}{} => {{\n{}{}\n{}}},",
                            state.tab().indent(),
                            print_case_patterns(state, labels),
                            state.tab().tab().indent(),
                            arms.iter()
                                .map(|x| print_expr(state.tab().tab(), x))
                                .collect::<Vec<_>>()
                                .join("; "),
                            state.tab().indent(),
                        ));
                    }
                }
            }
            format!("match {} {{\n{}\n{}}}", print_expr(state.tab(), cond), out.join("\n"), state.indent())
        }
        Lambda(ref pats, ref body) => {
            format!("|{}| {{ {} }}",
                print_patterns(state.tab(), pats),
                print_expr(state.tab(), body))
        }
        Operator(ref value) => {
            print_op_fn(value)
        }
        Dummy => {
            format!("<Expr::Dummy>")
        }
        Error => {
            format!("<Expr::Error>")
        }
    };
    ir::Expr::Free(freeform)
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



// Expand a sequence of expression terms into a nested tree of operators
// TODO Operator precedence rules should be applied here
fn pat_explode(span: Vec<Pat>) -> Vec<Pat> {
    if span.len() < 3 {
        return span;
    }
    for i in 0..span.len() {
        if let &ast::Pat::Operator(ref op) = &span[i] {
            if op == ":" {
                return vec![ast::Pat::Concat(
                    Box::new(Pat::Span(pat_explode(span[0..i].to_vec().clone()))),
                    Box::new(Pat::Span(pat_explode(span[i+1..].to_vec().clone()))),
                )];
            } else {
                //TODO exhaustiveness check
                continue;
                //panic!("dont know how to translate {:?}", op);
            }
        }
    }
    span
}

fn print_pattern(state: PrintState, pat: &Pat) -> String {
    match *pat {
        Pat::Ref(ast::Ident(ref s)) =>{
            print_type_ident(state, s)
        }
        Pat::Span(ref span) => {
            let span = pat_explode(span.to_vec());
            let mut out_span = print_pattern(state.tab(), &span[0]);
            if span.len() > 1 {
                out_span.push_str(&format!("({})", print_patterns(state.tab(), &span[1..])));
            }
            out_span
        }
        Pat::Str(ref s) => {
            format!("{:?}", s)
        }
        Pat::Char(ref s) => {
            assert!(s.len() == 1, "char lit {:?}", s);
            format!("{:?}", s.chars().next().unwrap())
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
        Pat::Record(..) => "{ /* pat record */ }".to_string(),
        Pat::Arrow(ast::Ident(ref s), ref p) => {
            format!("({} -> {})", s, print_pattern(state.tab(), &**p))
        }
        Pat::Not(ref s) => print_pattern(state, &**s),
        Pat::EmptyParen => format!("()"),
        Pat::Concat(ref a, ref b) => {
            format!("[{}, {}]", // TODO expand second argument
                print_pattern(state.tab(), &**a),
                print_pattern(state.tab(), &**b),
            )
        }
        Pat::Operator(ref op) => {
            // Should only be @
            format!("__OP__")
        }
        Pat::Infix(ref ident) => {
            panic!("Infix pattern `{}` was not rearranged", ident.0)
        }
        Pat::Dummy => format!("<todo>"),
    }
}


fn print_type<T: Borrow<Ty>>(state: PrintState, t: T) -> String {
    match *t.borrow() {
        Ty::Ref(ast::Ident(ref s)) => {
            print_type_ident(state, s)
        }
        Ty::Not(ref s) => {
            print_type(state, &**s)
        }
        Ty::Span(ref span) => {
            let mut out_span = print_type(state.tab(), &span[0]);
            if span.len() > 1 {
                out_span.push_str(&format!("<{}>", print_types(state.tab(), &span[1..])));
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
        Ty::Pair(ref a, ref b) => {
            format!("fn({}) -> {}", print_type(state, &**a), print_type(state, &**b))
        }
        Ty::Record(..) => "TypeRecord /* todo */".to_string(),
        Ty::EmptyParen => "()".to_string(),
        Ty::Dummy => "()".to_string(),
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

fn print_case_patterns<I, T>(state: PrintState, iter: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Borrow<Pat>,
{
    iter.into_iter().map(|p| print_pattern(state, p.borrow())).collect::<Vec<_>>().join(" | ")
}

fn print_item_list(state: PrintState, stats: &[ast::Item]) -> String {
    let mut types = btreemap![];
    for item in stats {
        //errln!("{:?}", item);

        // println!("well {:?}", item);
        if let &ast::Item::Prototype(ast::Ident(ref s), ref d) = item {
            if types.contains_key(&s) {
                panic!("that shouldn't happen {:?}", s);
            }
            types.insert(s, d.clone());
        }
    }

    // Output
    let mut out = vec![];

    // Print out data structures.
    for item in stats {
        if let ast::Item::Data(name, data, derives, args) = item.clone() {
            let derive_rust = derives.iter()
                .map(|x| {
                    // Convert common Haskell "derive" terms into Rust's
                    if x.0 == "Data" {
                        format!("Clone")
                    } else if x.0 == "Typeable" || x.0 == "Show" {
                        format!("Debug")
                    } else {
                        x.0.to_string()
                    }
                })
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            if data.len() > 1 {
                out.push(format!("{}{}pub enum {} {{\n{}{}\n{}}}\n{}{}",
                    state.indent(),
                    if derive_rust.len() > 0 {
                        format!("#[derive({})]\n{}", derive_rust.join(", "), state.indent())
                    } else {
                        format!("")
                    },
                    print_type(state, Ty::Span({
                        let mut v = vec![Ty::Ref(name.clone())];
                        v.extend(args.unwrap_or(vec![]));
                        v
                    })),
                    state.tab().indent(),
                    data.iter().map(|tyset| {
                        let mut tyset = tyset.clone();
                        if tyset.len() == 1 {
                            if let Ty::Span(values) = tyset[0].clone() {
                                tyset = values;
                            }
                        }
                        format!("{}{}",
                            print_type(state.tab(), tyset[0].clone()),
                            if tyset.len() > 2 {
                                format!("{}", print_type(state.tab(), Ty::Tuple(tyset.clone()[1..].to_vec())))
                            } else if tyset.len() > 1 {
                                format!("({})", print_type(state.tab(), Ty::Tuple(tyset.clone()[1..].to_vec())))
                            } else {
                                "".to_string()
                            }
                        )
                    }).collect::<Vec<_>>().join(&format!(",\n{}", state.tab().indent())),
                    state.indent(),
                    state.indent(),
                    format!("pub use self::{}::*;", print_ident(state, name.0)),
                    ));
            } else {
                let props = data.iter().map(|tyset| {
                    print_types(state, tyset)
                }).collect::<Vec<_>>().join(", ");
                out.push(format!("{}{}struct {}{};",
                    state.indent(),
                    if derive_rust.len() > 0 {
                        format!("#[derive({})]\n{}", derive_rust.join(", "), state.indent())
                    } else {
                        format!("")
                    },
                    print_type(state, Ty::Span({
                        let mut v = vec![Ty::Ref(name)];
                        v.extend(args.unwrap_or(vec![]));
                        v
                    })),
                    if data.len() > 0 { format!("({})", props) } else { "".to_string() }
                ));
            }
            out.push("".to_string())
        }
    }

    // Print out assignments as fns
    let mut cache: BTreeMap<String, Vec<ast::Assignment>> = btreemap![];
    for item in stats {
        if let ast::Item::Assign(assign, where_) = item.clone() {
            if !where_.is_empty() {
                // TODO
                //println!("// push {:?} into fn", where_)
                //out.push(print_item_list(state.tab(), w));
            }

            let mut assign = *assign;

            // If hte AST is refactored to break out the first Ident
            // for the issigned, this whole check should be deleted
            let ident = match assign.pats.remove(0) {
                Pat::Ref(ast::Ident(s)) => s,
                span => panic!("Expected ident, got {:?}\n\nin: {:?}\n", span, item),
            };

            cache.entry(ident)
                .or_insert(vec![])
                .push(assign);
        }
    }

    // Convert guards into basically case statements
    let mut new_cache: BTreeMap<String, Vec<ast::Assignment>> = btreemap![];
    for (key, fnset) in cache {
        if fnset.len() > 1 {
            // There are multiple impls of this function, so expand this into a
            // case statement.
            let args = (0..fnset[0].pats.len())
                .map(|x| format!("__{}", x))
                .collect::<Vec<_>>();

            // Convert args into case options.
            let pats = args
                .iter()
                .map(|x| Pat::Ref(ast::Ident(x.to_string())))
                .collect::<Vec<_>>();

            // Generate case statements.
            let expr = ast::Expr::Case(
                Box::new(ast::Expr::Parens(args.iter()
                    .map(|x| ast::Expr::Ref(ast::Ident(x.to_string())))
                    .collect::<Vec<_>>())),
                fnset.iter().map(|x| {
                    ast::CaseCond::Direct(
                        vec![Pat::Tuple(x.pats.clone())],
                        vec![x.expr.clone()])
                }).collect::<Vec<_>>(),
            );
            let res = vec![ast::Assignment { pats, expr }];
            new_cache.insert(key, res);
        } else {
            //TODO waitaminute
            new_cache.insert(key, fnset);
        }
    }

    for (key, fnset) in new_cache {
        for ast::Assignment { pats: args, expr } in fnset {
            // For type-less functions,
            if !types.contains_key(&key) {
                // With no type signature, we print a lambda.
                // If there are no arguments, we compute it now (non-lazily).
                if args.len() == 0 {
                    out.push(
                        format!("{}let {} = {};\n",
                            state.indent(),
                            key,
                            print_expr(state.tab(), &expr)));
                } else {
                    out.push(
                        format!("{}let {} = |{}| {{\n{}{}\n{}}};\n",
                            state.indent(),
                            key,
                            print_patterns(state, args),
                            state.tab().indent(),
                            print_expr(state.tab(), &expr),
                            state.indent()));
                }
                continue;
            }

            let d = types[&key].clone();
            //assert!(d.len() == 1);
            //TODO what did this assert do
            let t = unpack_fndef(d[0].clone());
            assert!(t.len() >= 1);

            //println!("hm {:?}", types[&key]);
            //println!("hm {:?}", t);
            let mut args_span = vec![];
            for (arg, ty) in args.iter().zip(t.iter()) {
                args_span.push(format!("{}: {}", print_pattern(state, arg), print_type(state.tab(), ty)));
            }
            out.push(
                format!("{}pub fn {}({}) -> {} {{\n{}{}\n{}}}\n",
                    state.indent(),
                    print_type_ident(state, &key),
                    args_span.join(", "),
                    print_type(state.tab(), t.last().unwrap()),
                    state.tab().indent(),
                    print_expr(state.tab(), &expr),
                    state.indent()));
        }
    }

    out.join("\n")
}

fn print_let(state: PrintState, assign: &ast::Assignment) -> String {
    format!(
        "{}let {} = {};",
        state.indent(),
        print_patterns(state, &assign.pats),
        print_expr(state, &assign.expr),
    )
}

fn print_do(state: PrintState, stmts: &[ast::DoItem]) -> String {
    let mut out = vec![];
    for (i, stmt) in stmts.iter().enumerate() {
        match *stmt {
            ast::DoItem::Let(ref assigns) => {
                for assign in assigns {
                    out.push(print_let(state, assign));
                }
            }
            ast::DoItem::Bind(ref pats, ref expr) => {
                // good enough for now
                let assign = ast::Assignment { pats: pats.clone(), expr: *expr.clone() };
                out.push(print_let(state, &assign));
            }
            ast::DoItem::Expression(ref e) => {
                let mut expr = print_expr(state, &*e);
                if i + 1 < stmts.len() {
                    expr.push(';');
                }
                out.push(format!("{}{}", state.indent(), expr));
            }
        }
    }
    format!("/* do */ {{\n{}\n{}}}", out.join("\n"), state.untab().indent())
}

#[test] #[ignore]
fn test_single_file() {
    use std::io::Write;

    let a = "./corrode/src/Language/Rust/Corrode/C.lhs";
    // let a = "./corrode/src/Language/Rust/Corrode/C.hs";
    // let a = "./test/input.hs";
    println!("file: {}", a);
    let mut file = File::open(a).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if a.ends_with(".lhs") {
        contents = strip_lhx(&contents);
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
        "./corrode/src/Language/Rust/AST.hs",
        "./corrode/src/Language/Rust/Corrode/C.lhs",
        "./corrode/src/Language/Rust/Corrode/CFG.lhs",
        "./corrode/src/Language/Rust/Corrode/CrateMap.hs",
        "./corrode/src/Language/Rust/Idiomatic.hs",
        "./corrode/src/Language/Rust.hs",

        "./language-c/src/Language/C/Analysis/AstAnalysis.hs",
        "./language-c/src/Language/C/Analysis/Builtins.hs",
        "./language-c/src/Language/C/Analysis/ConstEval.hs",
        "./language-c/src/Language/C/Analysis/Debug.hs",
        "./language-c/src/Language/C/Analysis/DeclAnalysis.hs",
        "./language-c/src/Language/C/Analysis/DefTable.hs",
        "./language-c/src/Language/C/Analysis/Export.hs",
        "./language-c/src/Language/C/Analysis/NameSpaceMap.hs",
        "./language-c/src/Language/C/Analysis/SemError.hs",
        "./language-c/src/Language/C/Analysis/SemRep.hs",
        "./language-c/src/Language/C/Analysis/TravMonad.hs",
        "./language-c/src/Language/C/Analysis/TypeCheck.hs",
        "./language-c/src/Language/C/Analysis/TypeConversions.hs",
        "./language-c/src/Language/C/Analysis/TypeUtils.hs",
        "./language-c/src/Language/C/Analysis.hs",
        "./language-c/src/Language/C/Data/Error.hs",
        "./language-c/src/Language/C/Data/Ident.hs",
        "./language-c/src/Language/C/Data/InputStream.hs",
        "./language-c/src/Language/C/Data/Name.hs",
        "./language-c/src/Language/C/Data/Node.hs",
        "./language-c/src/Language/C/Data/Position.hs",
        "./language-c/src/Language/C/Data/RList.hs",
        "./language-c/src/Language/C/Data.hs",
        "./language-c/src/Language/C/Parser/Builtin.hs",
        "./language-c/src/Language/C/Parser/ParserMonad.hs",
        "./language-c/src/Language/C/Parser/Tokens.hs",
        "./language-c/src/Language/C/Parser.hs",
        "./language-c/src/Language/C/Pretty.hs",
        "./language-c/src/Language/C/Syntax/AST.hs",
        "./language-c/src/Language/C/Syntax/Constants.hs",
        "./language-c/src/Language/C/Syntax/Ops.hs",
        "./language-c/src/Language/C/Syntax/Utils.hs",
        "./language-c/src/Language/C/Syntax.hs",
        "./language-c/src/Language/C/System/GCC.hs",
        "./language-c/src/Language/C/System/Preprocess.hs",

        "./gen/Lexer.hs",
        "./gen/Parser.hs",

        "./test/input.hs",
    ];

    for path in a {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        if path.ends_with(".lhs") {
            contents = strip_lhx(&contents);
        }
        let contents = parser_haskell::preprocess(&contents);

        // Do not output preprocessed data temp.txt
        //println!("{:?}", path);
        use ::std::io::Write;
        // let mut a = ::std::fs::File::create("temp.txt").unwrap();
        // a.write_all(contents.as_bytes());

        let mut errors = Vec::new();
        match parser_haskell::parse(&mut errors, &contents) {
            Ok(v) => {
            }
            Err(e) => {
                //TODO print_parse_error return string, feed to panic
                print_parse_error(&contents, &simplify_parse_error(e));
                panic!("cannot convert file {:?}", path);
            }
        }
    }
}

fn strip_lhx(s: &str) -> String {
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

fn convert_file(input: &str, p: &Path) -> (String, String) {
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

    // Parse the file.
    let mut errors = Vec::new();
    match parser_haskell::parse(&mut errors, &contents) {
        Ok(v) => {
            //errln!("{:?}", v);

            let expand_mod = false;
            if expand_mod {
                let _ = writeln!(file_out, "pub mod {} {{", v.name.0.replace(".", "_"));
                let _ = writeln!(file_out, "    use haskell_support::*;");
                let _ = writeln!(file_out, "");
                let state = PrintState::new();
                let _ = writeln!(file_out, "{}", print_item_list(state.tab(), &v.items));
                let _ = writeln!(file_out, "}}\n");
            } else {
                let _ = writeln!(file_out, "use haskell_support::*;");
                let _ = writeln!(file_out, "");
                let state = PrintState::new();
                let _ = writeln!(file_out, "{}", print_item_list(state, &v.items));
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

    (file_out, rust_out)
}


#[cfg(not(test))]
fn main() {
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
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let do_run = matches.is_present("run");
    if do_run {
        errln!("running {:?}...", file);
    } else {
        errln!("cross-compiling {:?}...", file);
    }

    // Create target directory.
    let target_dir = matches.value_of("out");
    if let Some(target) = target_dir {
        let _ = ::std::fs::create_dir_all(target);
    }

    let mut rust_section = "".to_string();
    let mut file_section = "".to_string();

    let _ = writeln!(file_section, "{}", include_str!("haskell_support.txt"));
    let _ = writeln!(file_section, "");
    for entry in WalkDir::new(file) {
        let e = entry.unwrap();
        let p = e.path();

        // Check filetype. Allow .lhs and .hs, ignore all else.
        let mut do_strip_lhx = false;
        if p.display().to_string().ends_with(".lhs") {
            do_strip_lhx = true;
        } else if !p.display().to_string().ends_with(".hs") {
            continue;
        }

        // Read file contents.
        let mut file = File::open(p).unwrap();
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(..) => (),
            _ => continue,
        };

        // Preprocess the file.
        if do_strip_lhx {
            contents = strip_lhx(&contents);
        }

        let (file_out, rust_out) = convert_file(&contents, p);

        if let Some(target) = target_dir {
            //let _ = ::std::fs::create_dir_all(target);
            let mut a = p.components();
            a.next();
            a.next();
            a.next();

            let t = format!("{}/{}", target, a.as_path().display()).to_lowercase();
            let _ = ::std::fs::create_dir_all(&Path::new(&t).parent().unwrap());

            let t = t.replace(".lhs", ".rs");
            let t = t.replace(".hs", ".rs");

            let mut f = File::create(&t).unwrap();
            let _ = f.write_all(file_out.as_bytes());
            let _ = f.write_all(rust_out.as_bytes());
            drop(f);
        } else {
            let _ = writeln!(file_section, "{}", file_out);
            rust_section.push_str(&rust_out);
        }
    }

    if let Some(_) = target_dir {
        return;
    }
    let _ = writeln!(file_section, "");
    let _ = writeln!(file_section, "");
    if rust_section.len() > 0 {
        let _ = writeln!(file_section, "{}", include_str!("haskell_support.txt"));
        let _ = writeln!(file_section, "/* RUST ... /RUST */");
        let _ = writeln!(file_section, "{}", rust_section);
    }

    // Evaluate --run
    if !do_run {
        print!("{}", file_section);
    } else {
        let dir = TempDir::new("corollary").unwrap();
        let file_path = dir.path().join("script.rs");

        let mut f = File::create(&file_path).unwrap();
        let _ = f.write_all(file_section.as_bytes());
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
}
