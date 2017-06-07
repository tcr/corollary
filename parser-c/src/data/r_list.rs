// Original file: "RList.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Prelude;
// use Data::List;

pub struct Reversed<a>(pub a);

pub fn snoc<a>(Reversed(xs): Reversed<Vec<a>>, x: a) -> Reversed<Vec<a>> {
    Reversed((__op_concat(x, xs)))
}

pub mod RList {
    use super::Reversed;
    use corollary_support::*;

    pub fn empty<a>() -> Reversed<Vec<a>> {
        Reversed(vec![])
    }

    pub fn singleton<a>(x: a) -> Reversed<Vec<a>> {
        Reversed(vec![x])
    }

    pub fn rappend<a>(Reversed(xs): Reversed<Vec<a>>, ys: Vec<a>) -> Reversed<Vec<a>> {
        Reversed((__op_addadd(List::reverse(ys), xs)))
    }

    pub fn appendr<a>(xs: Vec<a>, Reversed(ys): Reversed<Vec<a>>) -> Reversed<Vec<a>> {
        Reversed((__op_addadd(ys, List::reverse(xs))))
    }

    pub fn rappendr<a>(Reversed(xs): Reversed<Vec<a>>,
                    Reversed(ys): Reversed<Vec<a>>)
                    -> Reversed<Vec<a>> {
        Reversed((__op_addadd(ys, xs)))
    }

    pub fn rmap<a, b>(f: fn(a) -> b, Reversed(xs): Reversed<Vec<a>>) -> Reversed<Vec<b>> {
        Reversed((__map!(f, xs)))
    }

    pub fn reverse<a>(Reversed(xs): Reversed<Vec<a>>) -> Vec<a> {
        List::reverse(xs)
    }

    pub fn viewr<a>(_0: Reversed<Vec<a>>) -> (Reversed<Vec<a>>, a) {
        let mut xs = _0.0;
        if xs.is_empty() {
            __error!("viewr: empty RList".to_string());
        } else {
            let x = xs.remove(0);
            (Reversed(xs), x)
        }
    }
}