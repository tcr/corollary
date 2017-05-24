pub mod Language_C_Parser_Builtin {
    use haskell_support::*;
    pub fn builtinTypeNames() -> Vec<Ident> {
        vec![builtinIdent("__builtin_va_list".to_string())]
    }

}

