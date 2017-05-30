use haskell_support::*;

use language_.c._data::ident;
use language_.c._data::node;
use language_.c._analysis::def_table;
use language_.c._analysis::sem_rep;
use language_.c._analysis::type_utils;

pub fn builtins() -> DefTable {
    foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
}

