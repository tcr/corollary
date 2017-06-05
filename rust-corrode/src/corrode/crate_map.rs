// Original file: "CrateMap.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Foldable;
// use Data::List;
// use Data::Map;
// use Data::Maybe;

#[derive(Debug, Eq, Ord)]
pub enum ItemKind {
    Enum,
    Struct,
    Union,
    Type,
    Symbol
}
pub use self::ItemKind::*;

pub type ModuleMap = Vec<((ItemKind, String), String)>;

pub type CrateMap = Map::Map<String, ModuleMap>;

pub type CratesMap = Map::Map<String, CrateMap>;

pub type ItemRewrites = Map::Map<(ItemKind, String), Vec<String>>;

pub fn parseCrateMap() -> Either<String, CrateMap> {

    let root = |_0| {
        match (_0) {
            (crate, []) => {
                crate
            },
            (crate, unassigned) => {
                crate
            },
        }
    };

    let cleanLine = words(takeWhile((__op_assign_div('#'))));

    let parseLine = |_0, _1| {
        match (_0, _1) {
            (["-", item], (crate, items)) => {
                /*do*/ {
                    let item_q = parseItem(item);

                    (crate, __op_concat(item_q, items))
                }
            },
            ([name], (crate, items)) => {
                /*do*/ {
                    let item_q = parseItem(item);

                    (crate, __op_concat(item_q, items))
                }
            },
            (contents, _) => {
                /*do*/ {
                    let item_q = parseItem(item);

                    (crate, __op_concat(item_q, items))
                }
            },
        }
    };

    let parseItem = |contents| {
        match parseItemKind(contents) {
            (kind, [name]) => {
                ((kind, name), name)
            },
            (kind, [old, "as", new]) => {
                ((kind, old), new)
            },
            _ => {
                Left((unwords((__op_concat("unsupported crate map item:".to_string(), contents)))))
            },
        }
    };

    let parseItemKind = |_0| {
        match (_0) {
            ["enum", rest] => {
                (Enum, rest)
            },
            ["struct", rest] => {
                (Enum, rest)
            },
            ["union", rest] => {
                (Enum, rest)
            },
            ["typedef", rest] => {
                (Enum, rest)
            },
            rest => {
                (Enum, rest)
            },
        }
    };

    fmap(root, foldrM(parseLine, (Map::empty, vec![]), filter((not(null)), __map!(cleanLine, lines))))
}

pub fn mergeCrateMaps() -> Map::Map<String, CrateMap> {
    Map::fromListWith((Map::unionWith((__op_addadd))))
}

pub fn splitModuleMap(modName: String, crates: CratesMap) -> (ModuleMap, CratesMap) {
    fromMaybe((vec![], crates), /*do*/ {
            let thisCrate = Map::lookup("".to_string(), crates);

            let thisModule = Map::lookup(modName, thisCrate);

            let thisCrate_q = Map::delete(modName, thisCrate);

            let crates_q = Map::insert("".to_string(), thisCrate_q, crates);

            (thisModule, crates_q)
        })
}

pub fn rewritesFromCratesMap(crates: CratesMap) -> ItemRewrites {
    Map::fromList(/* Expr::Generator */ Generator)
}



