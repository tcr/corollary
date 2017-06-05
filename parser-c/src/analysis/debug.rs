// Original file: "Debug.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::Export;
// use Language::C::Analysis::DefTable;
// use Language::C::Analysis::NameSpaceMap;
// use Language::C::Data;
// use Language::C::Pretty;
// use Text::PrettyPrint::HughesPJ;
// use Data::Map;
// use Map;
// use Data::Map;

pub fn prettyAssocs(label: String) -> Doc {
    prettyAssocsWith(label, pretty, pretty)
}

pub fn prettyAssocsWith(label: String, prettyKey: fn(k) -> Doc, prettyVal: fn(v) -> Doc, theMap: Vec<(k, v)>) -> Doc {

    let prettyEntry = |(k, v)| {
        __op_doc_conat(prettyKey(k), __op_doc_conat(text(" ~> ".to_string()), prettyVal(v)))
    };

    __op_line_something(text(label), nest(8, (vcat(__map!(prettyEntry, theMap)))))
}

pub fn globalDeclStats(file_filter: fn(FilePath) -> bool, gmap: GlobalDecls) -> Vec<(String, isize)> {

    let gmap_q = filterGlobalDecls(filterFile, gmap);

    pub fn filterFile() -> bool {
        maybe(true, file_filter, fileOfNode(nodeInfo))
    }

    vec![
        ("Enumeration Constants".to_string(), Map::size(enumerators)),
        ("Total Object/Function Declarations".to_string(), Map::size(all_decls)),
        ("Object definitions".to_string(), Map::size(objDefs)),
        ("Function Definitions".to_string(), Map::size(funDefs)),
        ("Tag definitions".to_string(), Map::size(tagDefs)),
        ("TypeDefs".to_string(), Map::size(typeDefs)),
    ]
}

pub fn joinComma<T>(input: Vec<T>) -> Doc {
    hsep(punctuate(comma, __map!(pretty, input)))
}

pub fn terminateSemi<T>(input: Vec<T>) -> Doc {
    terminateSemi_(__map!(pretty, input))
}

pub fn terminateSemi_<T>(input: Vec<T>) -> Doc {
    hsep(__map!(|x| { __op_ne(semi, x) }, input))
}



