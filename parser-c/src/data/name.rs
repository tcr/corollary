//! Original file: "Name.hs"
//! File auto-generated using Corollary.

use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Ix;
// use Data::IntMap;
// use IntMap;
// use Data::IntMap;
// use Data::Generics;

pub fn namesStartingFrom(k: isize) -> Vec<Name> {
    vec![Name(k::::)]
}

pub fn newNameSupply() -> Vec<Name> {
    namesStartingFrom(0)
}



