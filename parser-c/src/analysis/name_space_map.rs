// Original file: "NameSpaceMap.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Prelude;
// use Prelude;
// use Data::Map;
// use Data::List;
// use Data::Map;
// use Map;

pub struct NsMap<k, v>(Map<k, v>, Vec<Vec<(k, v)>>);

pub type NameSpaceMap<k, v> = NsMap<k, v>;


pub fn globalNames<k, v>(NsMap(g, _): NameSpaceMap<k, v>) -> Map<k, v> {
    g
}

pub fn hasLocalNames<k, v>(NsMap(_, l): NameSpaceMap<k, v>) -> bool {
    not((null(l)))
}

pub fn localNames<k, v>(NsMap(_, l): NameSpaceMap<k, v>) -> Vec<Vec<(k, v)>> {
    l
}

pub fn nameSpaceMap<k, v>() -> NameSpaceMap<k, v> {
    NsMap(Map::empty(), vec![])
}

pub fn defGlobal<a, k>(NsMap(gs, lss): NameSpaceMap<k, a>,
                    ident: k,
                    def: a)
                    -> (NameSpaceMap<k, a>, Option<a>) {
    (NsMap((Map::insert(ident, def, gs)), lss), Map::lookup(ident, gs))
}

pub fn enterNewScope<a, k>(NsMap(gs, lss): NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
    NsMap(gs, (__op_concat(vec![], lss)))
}

pub fn leaveScope<a, k>(_0: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
    match (_0) {
        NsMap(_, []) => __error!("NsMaps.leaveScope: No local scope!".to_string()),
        NsMap(gs, [ls, lss]) => (NsMap(gs, lss), ls),
    }
}

pub fn defLocal<a, k>(_0: NameSpaceMap<k, a>,
                   _1: k,
                   _2: a,
                   _3: (NameSpaceMap<k, a>, Option<a>))
                   -> (NameSpaceMap<k, a>, Option<a>) {
    match (_0, _1, _2, _3, _4) {
        (ns, __OP__, NsMap(_, []), ident, def) => defGlobal(ns, ident, def),
        (NsMap(gs, [ls, lss]), ident, def) => {
            (NsMap(gs, (__op_concat((__op_concat((ident, def), ls)), lss))),
             Prelude::lookup(ident, ls))
        }
    }
}

pub fn lookupName<a, k>(ns: NameSpaceMap<k, a>,
                     __OP__: k,
                     NsMap(_, localDefs): Option<a>)
                     -> Option<a> {

    let lookupLocal = |_0| match (_0) {
        [] => None,
        [ls, lss] => {
            match Prelude::lookup(ident, ls) {
                None => lookupLocal(lss),
                Some(def) => Some(def),
            }
        }
    };

    match lookupLocal(localDefs) {
        None => lookupGlobal(ns, ident),
        Some(def) => Some(def),
    }
}

pub fn lookupGlobal<a, k>(NsMap(gs, _): NameSpaceMap<k, a>, ident: k) -> Option<a> {
    Map::lookup(ident, gs)
}

pub fn lookupInnermostScope<a>(nsm: NameSpaceMap<k, a>,
                               __OP__: k,
                               NsMap(_gs, localDefs): Option<a>)
                               -> Option<a> {
    match localDefs {
        [ls, _lss] => Prelude::lookup(ident, ls),
        [] => lookupGlobal(nsm, ident),
    }
}

pub fn nsMapToList<a, k>(NsMap(gs, lss): NameSpaceMap<k, a>) -> Vec<(k, a)> {
    __op_addadd(concat(lss), Map::toList(gs))
}

pub fn mergeNameSpace<a, k>(NsMap(global1, local1): NameSpaceMap<k, a>,
                         NsMap(global2, local2): NameSpaceMap<k, a>)
                         -> NameSpaceMap<k, a> {

    let localUnion = |_0, _1| match (_0, _1) {
        ([l1, ls1], [l2, ls2]) => {
            __op_concat(List::unionBy((|p1, p2| (fst(p1) == fst(p2))), l1, l2),
                        localUnion(ls1, ls2))
        }
        ([], ls2) => ls2,
        (ls1, []) => ls1,
    };

    NsMap((Map::union(global1, global2)), (localUnion(local1, local2)))
}
