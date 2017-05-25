use haskell_support::*;

pub fn globalDeclStats(file_filter: fn(FilePath) -> bool, gmap: GlobalDecls) -> Vec<(String, isize)> {
    vec![
        ("Enumeration Constants".to_string(), Map::size(enumerators)),
        ("Total Object/Function Declarations".to_string(), Map::size(all_decls)),
        ("Object definitions".to_string(), Map::size(objDefs)),
        ("Function Definitions".to_string(), Map::size(funDefs)),
        ("Tag definitions".to_string(), Map::size(tagDefs)),
        ("TypeDefs".to_string(), Map::size(typeDefs)),
    ]
}

pub fn joinComma() -> Doc {
    hsep(punctuate(comma, map(pretty)))
}

pub fn prettyAssocs(label: String) -> Doc {
    prettyAssocsWith(label, pretty, pretty)
}

pub fn prettyAssocsWith(label: String, prettyKey: fn(k) -> Doc, prettyVal: fn(v) -> Doc, theMap: Vec<(k, v)>) -> Doc {
    $$(text(label), (nest(8))((vcat(map(prettyEntry, theMap)))))
}

pub fn terminateSemi() -> Doc {
    terminateSemi_(map(pretty))
}

pub fn terminateSemi_() -> Doc {
    hsep(map((<>(semi))))
}

