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

{-# LANGUAGE DeriveDataTypeable #-}
-----------------------------------------------------------------------------
-- |
-- Module      :  Language.C.Syntax.Ops
-- Copyright   :  (c) 2008 Benedikt Huber
-- License     :  BSD-style
-- Maintainer  : benedikt.huber@gmail.com
-- Stability   : experimental
-- Portability : ghc
--
-- Unary, binary and asssignment operators. Exported via AST.
-----------------------------------------------------------------------------
module Language.C.Syntax.Ops (
-- * Assignment operators
CAssignOp(..),
assignBinop,
-- * Binary operators
CBinaryOp(..),
isCmpOp,
isPtrOp,
isBitOp,
isLogicOp,
-- * Unary operators
CUnaryOp(..),
isEffectfulOp
)
where
import Data.Generics

-- | C assignment operators (K&R A7.17)
data CAssignOp = CAssignOp
               | CMulAssOp
               | CDivAssOp
               | CRmdAssOp              -- ^ remainder and assignment
               | CAddAssOp
               | CSubAssOp
               | CShlAssOp
               | CShrAssOp
               | CAndAssOp
               | CXorAssOp
               | COrAssOp
               deriving (Eq,Ord,Show,Data,Typeable)

assignBinop :: CAssignOp -> CBinaryOp
assignBinop CAssignOp = error "direct assignment has no binary operator"
assignBinop CMulAssOp = CMulOp
assignBinop CDivAssOp = CDivOp
assignBinop CRmdAssOp = CRmdOp
assignBinop CAddAssOp = CAddOp
assignBinop CSubAssOp = CSubOp
assignBinop CShlAssOp = CShlOp
assignBinop CShrAssOp = CShrOp
assignBinop CAndAssOp = CAndOp
assignBinop CXorAssOp = CXorOp
assignBinop COrAssOp  = COrOp

-- | C binary operators (K&R A7.6-15)
--
data CBinaryOp = CMulOp
               | CDivOp
               | CRmdOp                 -- ^ remainder of division
               | CAddOp
               | CSubOp
               | CShlOp                 -- ^ shift left
               | CShrOp                 -- ^ shift right
               | CLeOp                  -- ^ less
               | CGrOp                  -- ^ greater
               | CLeqOp                 -- ^ less or equal
               | CGeqOp                 -- ^ greater or equal
               | CEqOp                  -- ^ equal
               | CNeqOp                 -- ^ not equal
               | CAndOp                 -- ^ bitwise and
               | CXorOp                 -- ^ exclusive bitwise or
               | COrOp                  -- ^ inclusive bitwise or
               | CLndOp                 -- ^ logical and
               | CLorOp                 -- ^ logical or
               deriving (Eq,Ord,Show,Data,Typeable)

isCmpOp :: CBinaryOp -> Bool
isCmpOp op = op `elem` [ CLeqOp, CGeqOp, CLeOp, CGrOp, CEqOp, CNeqOp ]

isPtrOp :: CBinaryOp -> Bool
isPtrOp op = op `elem` [ CAddOp, CSubOp ]

isBitOp :: CBinaryOp -> Bool
isBitOp op = op `elem` [ CShlOp, CShrOp, CAndOp, COrOp, CXorOp ]

isLogicOp :: CBinaryOp -> Bool
isLogicOp op = op `elem` [ CLndOp, CLorOp ]

-- | C unary operator (K&R A7.3-4)
--
data CUnaryOp = CPreIncOp               -- ^ prefix increment operator
              | CPreDecOp               -- ^ prefix decrement operator
              | CPostIncOp              -- ^ postfix increment operator
              | CPostDecOp              -- ^ postfix decrement operator
              | CAdrOp                  -- ^ address operator
              | CIndOp                  -- ^ indirection operator
              | CPlusOp                 -- ^ prefix plus
              | CMinOp                  -- ^ prefix minus
              | CCompOp                 -- ^ one's complement
              | CNegOp                  -- ^ logical negation
              deriving (Eq,Ord,Show,Data,Typeable)

isEffectfulOp :: CUnaryOp -> Bool
isEffectfulOp op = op `elem` [ CPreIncOp, CPreDecOp, CPostIncOp, CPostDecOp ]

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
