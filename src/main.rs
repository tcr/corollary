extern crate lalrpop_util;
extern crate regex;

pub mod ast;
pub mod calculator;

use regex::Regex;

fn strip_comments(text: &str) -> String {
    let re = Regex::new(r"--[^\n\r]*").unwrap();
    let text = re.replace_all(&text, "").to_string();

    let re = Regex::new(r"\{-#.*?#-\}").unwrap();
    let text = re.replace_all(&text, "").to_string();

    text
}

#[test]
fn calculator() {

    let input = r#"
module Language.C.Syntax.Utils (
  -- * Generic operations
  getSubStmts,
  mapSubStmts,
  mapBlockItemStmts,
  -- * Concrete operations
  getLabels
) where

import Data.List
import Language.C.Data.Ident
import Language.C.Syntax.AST

-- XXX: This is should be generalized !!!
--      Data.Generics sounds attractive, but we really need to control the evaluation order
-- XXX: Expression statements (which are somewhat problematic anyway), aren't handled yet
getSubStmts :: CStat -> [CStat]
getSubStmts (CLabel _ s _ _)      = [s]
getSubStmts (CCase _ s _)         = [s]
getSubStmts (CCases _ _ s _)      = [s]
getSubStmts (CDefault s _)        = [s]
getSubStmts (CExpr _ _)           = []
getSubStmts (CCompound _ body _)  = concatMap compoundSubStmts body
getSubStmts (CIf _ sthen selse _) = maybe [sthen] (\s -> [sthen,s]) selse
getSubStmts (CSwitch _ s _)       = [s]
getSubStmts (CWhile _ s _ _)      = [s]
getSubStmts (CFor _ _ _ s _)      = [s]
getSubStmts (CGoto _ _)           = []
getSubStmts (CGotoPtr _ _)        = []
getSubStmts (CCont _)             = []
getSubStmts (CBreak _)            = []
getSubStmts (CReturn _ _)         = []
getSubStmts (CAsm _ _)            = []

"#;

    let input = commify(input);

    let mut errors = Vec::new();
    {
        let okay = calculator::parse_Statements(&mut errors, &input).unwrap();
        println!("{:?}", okay);
    }
}

#[cfg(not(test))]
fn main() {
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

            if word == "do" || word == "where" {
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

    out
}
