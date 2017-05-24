pub mod Language_C_Analysis_Builtins {
    use haskell_support::*;
    pub fn builtins() -> DefTable {
        foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
    }

}

