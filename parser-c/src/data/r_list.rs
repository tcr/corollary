//! Original file: "RList.hs"
//! File auto-generated using Corollary.

use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Prelude;
// use Data::List;

pub fn appendr(xs: Vec<a>, Reversed(ys): Reversed<Vec<a>>) -> Reversed<Vec<a>> {
    Reversed((__op_addadd(ys, List::reverse(xs))))
}

pub fn empty() -> Reversed<Vec<a>> {
    Reversed(vec![])
}

pub fn rappend(Reversed(xs): Reversed<Vec<a>>, ys: Vec<a>) -> Reversed<Vec<a>> {
    Reversed((__op_addadd(List::reverse(ys), xs)))
}

pub fn rappendr(Reversed(xs): Reversed<Vec<a>>, Reversed(ys): Reversed<Vec<a>>) -> Reversed<Vec<a>> {
    Reversed((__op_addadd(ys, xs)))
}

pub fn reverse(Reversed(xs): Reversed<Vec<a>>) -> Vec<a> {
    List::reverse(xs)
}

pub fn rmap(f: fn(a) -> b, Reversed(xs): Reversed<Vec<a>>) -> Reversed<Vec<b>> {
    Reversed((map(f, xs)))
}

pub fn singleton(x: a) -> Reversed<Vec<a>> {
    Reversed(vec![x])
}

pub fn snoc(Reversed(xs): Reversed<Vec<a>>, x: a) -> Reversed<Vec<a>> {
    Reversed((__op_concat(x, xs)))
}

pub fn viewr(_0: Reversed<Vec<a>>) -> (Reversed<Vec<a>>, a) {
    match (_0) {
        Reversed([]) => {
            __error!("viewr: empty RList".to_string())
        },
        Reversed([x, xs]) => {
            (Reversed(xs), x)
        },
    }
}



