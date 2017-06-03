#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Ix;
// use Data::IntMap;
// use IntMap;
// use Data::IntMap;
// use Data::Generics;

#[derive(Clone, Debug, Eq, Ix, Ord, Read)]
pub struct Name{
    nameId: isize
}
fn nameId(a: Name) -> isize { a.nameId }

pub fn namesStartingFrom(k: isize) -> Vec<Name> {
    vec![Name(k::::)]
}

pub fn newNameSupply() -> Vec<Name> {
    namesStartingFrom(0)
}



