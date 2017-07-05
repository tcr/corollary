//! Utility functions used during parsing.

use base64;
use ast::*;

pub fn encode_literal(s: &str) -> String {
    base64::encode(s)
}

pub fn decode_literal(s: &str) -> String {
    let vec = base64::decode(s).unwrap_or_else(|_| panic!("invalid base64: {:?}", s));
    String::from_utf8(vec).expect("invalid UTF-8")
}

/// De-infixes a `Pat::Infix`.
pub fn rearrange_infix_pat(mut pats: Vec<Pat>) -> Vec<Pat> {
    let mut index = None;
    for (i, pat) in pats.iter().enumerate() {
        if match pat { &Pat::Infix(_) => true, _ => false } {
            index = Some(i);
            break;
        }
    }

    if let Some(i) = index {
        let left = pats[0..i].to_vec();
        let right = pats[i+1..].to_vec();
        let ident = match pats.remove(i) {
            Pat::Infix(ident) => ident,
            _ => panic!(),
        };
        return vec![Pat::Ref(ident), Pat::Span(left), Pat::Span(rearrange_infix_pat(right))]
    }

    pats
}


pub fn expr_to_pat(expr: &Expr) -> Pat {
    match *expr {
        Expr::Operator(ref s) => Pat::Operator(s.to_string()),
        Expr::Ref(ref id) => Pat::Ref(id.clone()),

        Expr::Parens(ref inner) => {
            Pat::Tuple(inner.iter().map(|x| expr_to_pat(x)).collect::<Vec<_>>())
        },
        Expr::Span(ref inner) => {
            Pat::Span(inner.iter().map(|x| expr_to_pat(x)).collect::<Vec<_>>())
        }
        Expr::Vector(ref inner) => {
            Pat::Brackets(inner.iter().map(|x| expr_to_pat(x)).collect::<Vec<_>>())
        }

        Expr::Record(ref base, ref inner) => Pat::Record(Ident("_TODO_RECORD_".to_string()), {
            inner.iter()
                .map(|&(ref k, ref v)| (k.clone(), expr_to_pat(&v)))
                .collect::<Vec<_>>()
        }),
        Expr::Number(num) => Pat::Num(num),
        Expr::Str(ref s) => Pat::Str(s.clone()),
        Expr::Char(ref s) => Pat::Char(s.clone()),

        Expr::If(..) |
        Expr::Op(..) |
        Expr::Case(..) |
        Expr::Let(..) |
        Expr::Do(..) |
        Expr::Lambda(..) |
        Expr::RecordArgs(..) |
        Expr::Generator(..) |
        Expr::Error => {
            panic!("Invalid expr to pat conversion: {:?}", expr);
        }
    }
}
