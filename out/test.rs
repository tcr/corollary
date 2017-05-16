mod test_module {
    #[derive(Clone, Debug)]
    struct NodeInfo(isize);

    #[derive(Clone, Debug)]
    struct Ident(String, isize, NodeInfo);

    fn applyRenames(ident: Ident) -> String {
        match identToString(ident) {
                "final" => { "final_".to_string() },
                "fn" => { "fn_".to_string() },
                "in" => { "in_".to_string() },
                "let" => { "let_".to_string() },
                "main" => { "_c_main".to_string() },
                "match" => { "match_".to_string() },
                "mod" => { "mod_".to_string() },
                "proc" => { "proc_".to_string() },
                "type" => { "type_".to_string() },
                "where" => { "where_".to_string() },
                name => { name },
            }
    }

    fn identToString(Ident(s, _, _): Ident) -> String {
        s
    }

}



fn main() { /* demo */ }
