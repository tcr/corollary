extern crate lalrpop_util;
extern crate regex;

pub mod ast;
pub mod calculator;

use regex::Regex;

fn strip_comments(text: &str) -> String {
    let re = Regex::new(r"--[^\n\r]*").unwrap();
    return re.replace_all(text, "").to_string();
}

#[test]
fn calculator() {

    let input = r#"
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
"#;

    let input = strip_comments(input);

    let mut errors = Vec::new();
    {
        let okay = calculator::parse_Statements(&mut errors, &input).unwrap();
        println!("{:?}", okay);
    }
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
