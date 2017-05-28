use haskell_support::*;

pub fn namesStartingFrom(k: isize) -> Vec<Name> {
    vec![Name(k::::)]
}

pub fn newNameSupply() -> Vec<Name> {
    namesStartingFrom(0)
}

