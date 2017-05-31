use haskell_support::*;

use Language::C::Data::Ident;
use Ident;

pub fn builtinTypeNames() -> Vec<Ident> {
    vec![builtinIdent("__builtin_va_list".to_string())]
}

