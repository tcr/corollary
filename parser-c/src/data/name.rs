// Original file: "Name.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Ix;
// use Data::Generics;

#[derive(Clone, Debug, Eq, Ord)]
pub struct Name(pub isize);

pub fn newNameSupply() -> Vec<Name> {
    namesStartingFrom(0)
}

pub fn namesStartingFrom(k: isize) -> Vec<Name> {
    // TODO fix this to be an infinite iterator
    vec![Name(k)]
}
