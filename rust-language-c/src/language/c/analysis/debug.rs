use haskell_support::*;

use language_.c._analysis::sem_rep;
use language_.c._analysis::export;
use language_.c._analysis::def_table;
use language_.c._analysis::name_space_map;
use language_.c._data;
use language_.c._pretty;
use language_.c._syntax;
use text::pretty_print::hughes_pj;
use data::map;
use map;
use qualified;
use data::map;
use as;
use map;

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
    __op_line_something(text(label), (nest(8))((vcat(map(prettyEntry, theMap)))))
}

pub fn terminateSemi() -> Doc {
    terminateSemi_(map(pretty))
}

pub fn terminateSemi_() -> Doc {
    hsep(map((__op_ne(semi))))
}

