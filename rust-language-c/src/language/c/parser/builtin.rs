use haskell_support::*;

use language_.c._data::ident;
use ident;

pub fn builtinTypeNames() -> Vec<Ident> {
    vec![builtinIdent("__builtin_va_list".to_string())]
}

