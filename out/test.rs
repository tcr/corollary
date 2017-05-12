mod test_module {
    #[derive(Clone, Debug)]
    struct NodeInfo(isize);

    #[derive(Clone, Debug)]
    struct Ident(String, isize, NodeInfo);

    fn applyRenames(ident: Ident) -> String {
        match (identToString(ident)).as_ref() {
                "final" => ("final_".to_string()).into(),
                "fn" => ("fn_".to_string()).into(),
                "in" => ("in_".to_string()).into(),
                "let" => ("let_".to_string()).into(),
                "main" => ("_c_main".to_string()).into(),
                "match" => ("match_".to_string()).into(),
                "mod" => ("mod_".to_string()).into(),
                "proc" => ("proc_".to_string()).into(),
                "type" => ("type_".to_string()).into(),
                "where" => ("where_".to_string()).into(),
                name => (name).into(),
            }
    }

    fn identToString(Ident(s, _, _): Ident) -> String {
        s
    }

}



fn main() { }
