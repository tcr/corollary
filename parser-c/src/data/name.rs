use corollary_support::*;

use Data::Ix;
use Data::IntMap;
use IntMap;
use Data::IntMap;
use Data::Generics;

pub fn namesStartingFrom(k: isize) -> Vec<Name> {
    vec![Name(k::::)]
}

pub fn newNameSupply() -> Vec<Name> {
    namesStartingFrom(0)
}



