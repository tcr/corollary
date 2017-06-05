// Original file: "CFG.lhs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Control::Monad;
// use Control::Monad::Trans::State;
// use Data::Foldable;
// use Data::IntMap::Lazy;
// use Data::IntSet;
// use Data::Maybe;
// use Data::Traversable;
// use Text::PrettyPrint::HughesPJClass;

pub struct BasicBlock<s, c>(s, Terminator<c>);


pub type Label = isize;

#[derive(Debug)]
pub enum Terminator_q<c, l> {
    Unreachable,
    Branch(l),
    CondBranch(c, l, l)
}
pub use self::Terminator_q::*;

pub type Terminator<c> = Terminator_q<c, Label>;

pub struct Unordered;


pub struct DepthFirst;


pub struct CFG<k, s, c>(Label, IntMap::IntMap<BasicBlock<s, c>>);


pub fn prettyCFG(fmtS: fn(s) -> Doc, fmtC: fn(c) -> Doc, CFG(entry, blocks): CFG<k, s, c>) -> Doc {

    let blocks_q = /*do*/ {
            let (label, BasicBlock(stmts, term)) = IntMap::toList(blocks);

            let blockHead = __op_ne(text((show(label))), text(":".to_string()));

            let blockBody = fmtS(stmts);

            let blockTail = match term {
                    Unreachable => {
                        text("// unreachable".to_string())
                    },
                    Branch(to) => {
                        text((__op_addadd("goto ".to_string(), __op_addadd(show(to), ";".to_string()))))
                    },
                    CondBranch(cond, true, false) => {
                        __op_ne(text("if(".to_string()), __op_ne(fmtC(cond), __op_ne(text(") goto ".to_string()), __op_ne(text((show(true))), __op_ne(text("; else goto ".to_string()), __op_ne(text((show(false))), text(";".to_string())))))))
                    },
                };

            __op_concat(blockHead, __op_addadd(__map!((nest(4)), vec![blockBody, blockTail]), vec![text("".to_string())]))
        };

    vcat(__op_concat((__op_ne(text("start @".to_string()), text((show(entry))))), blocks_q))
}

pub type BuildCFGT<m, s, c> = StateT<BuildState<s, c>, m>;

pub fn mapBuildCFGT<b>() -> BuildCFGT<n, s, c, b> {
    mapStateT
}

pub struct BuildState<s, c>{
    buildLabel: Label,
    buildBlocks: IntMap::IntMap<BasicBlock<s, c>>
}
fn buildLabel(a: BuildState) -> Label { a.buildLabel }
fn buildBlocks(a: BuildState) -> IntMap::IntMap<BasicBlock<s, c>> { a.buildBlocks }

pub fn newLabel() -> BuildCFGT<m, s, c, Label> {
    /*do*/ {
        let old = get;

        put(old {
                buildLabel: (buildLabel(old) + 1)
            });
        (buildLabel(old))
    }
}

pub fn addBlock(label: Label, stmt: s, terminator: Terminator<c>) -> BuildCFGT<m, s, c, ()> {
    /*do*/ {
        modify(|st| { st {
                    buildBlocks: IntMap::insert(label, (BasicBlock(stmt, terminator)), (buildBlocks(st)))
                } })
    }
}

pub fn buildCFG(root: BuildCFGT<m, s, c, Label>) -> m<CFG<Unordered, s, c>> {
    /*do*/ {
        let (label, __final) = runStateT(root, (BuildState(0, IntMap::empty)));

        (CFG(label, (buildBlocks(__final))))
    }
}

pub fn removeEmptyBlocks(CFG(start, blocks): CFG<k, f<s>, c>) -> CFG<Unordered, f<s>, c> {

    let go = /*do*/ {
            let (empties, done) = get;

            match IntMap::minViewWithKey(empties) {
                None => {
                    ()
                },
                Some(((from, to), empties_q)) => {
                    /*do*/ {
                        put((empties_q, done));
                        step(from, to);
                        go
                    }
                },
            }
        };

    let step = |from, to| {
        /*do*/ {
            let (empties, done) = get;

            match IntMap::splitLookup(to, empties) {
                (_, None, _) => {
                    ()
                },
                (e1, Some(to_q), e2) => {
                    /*do*/ {
                        put((IntMap::union(e1, e2), done));
                        step(to, to_q)
                    }
                },
            };
            let (empties_q, done_q) = get;

            let to_q = IntMap::findWithDefault(to, to, done_q);

            put((empties_q, IntMap::insert(from, to_q, done_q)))
        }
    };

    let rewrites = snd(execState(go, (IntMap::mapMaybe(isBlockEmpty, blocks), IntMap::empty)));

    let rewrite = |to| {
        IntMap::findWithDefault(to, to, rewrites)
    };

    let discards = IntMap::keysSet((IntMap::filterWithKey((__op_assign_div), rewrites)));

    let blocks_q = IntMap::mapMaybeWithKey(rewriteBlock, blocks);

    CFG((rewrite(start)), blocks_q)
}

#[derive(Debug)]
pub enum StructureLabel<s, c> {
    GoTo{
        structureLabel: Label
    },
    ExitTo{
        structureLabel: Label
    },
    Nested(Vec<Structure<s, c>>)
}
pub use self::StructureLabel::*;

pub type StructureTerminator<s, c> = Terminator_q<c, StructureLabel<s, c>>;

pub type StructureBlock<s, c> = (s, StructureTerminator<s, c>);

#[derive(Debug)]
pub enum Structure_q<s, c, a> {
    Simple(s, StructureTerminator<s, c>),
    Loop(a),
    Multiple(IntMap::IntMap<a>, a)
}
pub use self::Structure_q::*;

#[derive(Debug)]
pub struct Structure<s, c>{
    structureEntries: IntSet::IntSet,
    structureBody: Structure_q<s, c, Vec<Structure<s, c>>>
}
fn structureEntries(a: Structure) -> IntSet::IntSet { a.structureEntries }
fn structureBody(a: Structure) -> Structure_q<s, c, Vec<Structure<s, c>>> { a.structureBody }

pub fn prettyStructure() -> Doc {

    let go = |_0| {
        match (_0) {
            Structure(_, Simple(s, term)) => {
                __op_line_concat(text((__op_addadd(show(s), ";".to_string()))), text((show(term))))
            },
            Structure(entries, Loop(body)) => {
                __op_line_concat(text((__op_addadd(show(s), ";".to_string()))), text((show(term))))
            },
            Structure(entries, Multiple(handlers, unhandled)) => {
                __op_line_concat(text((__op_addadd(show(s), ";".to_string()))), text((show(term))))
            },
        }
    };

    let prettyGroup = |entries, kind, body| {
        __op_ne(text("{".to_string()), __op_ne(hsep((punctuate((text(",".to_string())), (__map!((text(show)), (IntSet::toList(entries))))))), __op_line_concat(text((__op_addadd("} ".to_string(), kind))), nest(2, body))))
    };

    vcat(__map!(go))
}

pub fn relooperRoot(CFG(entry, blocks): CFG<k, s, c>) -> Vec<Structure<s, c>> {
    relooper((IntSet::singleton(entry)), IntMap::map((|BasicBlock(s, term)| { (s, fmap(GoTo, term)) }), blocks))
}

pub fn relooper(entries: IntSet::IntSet, blocks: IntMap::IntMap<StructureBlock<s, c>>) -> Vec<Structure<s, c>> {

    let strictReachableFrom = flipEdges((go((IntMap::map(successors, blocks)))));

    {
        let (returns, noreturns) = partitionMembers(entries, IntSet::unions(__map!(successors, IntMap::elems(blocks))));

        let (present, absent) = partitionMembers(entries, (IntMap::keysSet(blocks)));

    match (IntSet::toList(noreturns), IntSet::toList(returns)) {
            ([], []) => {
                vec![]
            },
            ([entry], []) => {
                match IntMap::updateLookupWithKey((|_, _| { None }), entry, blocks) {
                    (Some((s, term)), blocks_q) => {
                        __op_concat(Structure {
                            structureEntries: entries,
                            structureBody: Simple(s, term)
                        }, relooper((successors((s, term))), blocks_q))
                    },
                    (None, _) => {
                        __op_concat(Structure {
                            structureEntries: entries,
                            structureBody: Simple(mempty, (Branch((GoTo(entry)))))
                        }, vec![])
                    },
                }
            },
            _ if not((IntSet::null(absent))) => { __op_concat(if IntSet::null(present) {             
vec![]} else {
Structure {
                structureEntries: entries,
                structureBody: Multiple((IntMap::fromSet((__TODO_const(vec![])), absent)), (relooper(present, blocks)))
            }
            }, vec![]) }
            ([], _) => {
                __op_concat(Structure {
                    structureEntries: entries,
                    structureBody: Loop((relooper(entries, blocks_q)))
                }, relooper(followEntries, followBlocks))
            },
            _ => {
                __op_concat(Structure {
                    structureEntries: entries,
                    structureBody: Multiple(handlers, unhandled)
                }, relooper(followEntries, followBlocks))
            },
        }    }
}

pub fn restrictKeys<a>(m: IntMap::IntMap<a>, s: IntSet::IntSet) -> IntMap::IntMap<a> {
    IntMap::intersection(m, IntMap::fromSet((__TODO_const(())), s))
}

pub fn outEdges(blocks: IntMap::IntMap<StructureBlock<s, c>>) -> IntSet::IntSet {
    IntSet::difference(IntSet::unions((__map!(successors, IntMap::elems(blocks)))), IntMap::keysSet(blocks))
}

pub fn partitionMembers<a, b>(a: IntSet::IntSet, b: IntSet::IntSet) -> (IntSet::IntSet, IntSet::IntSet) {
    (IntSet::intersection(a, b), IntSet::difference(a, b))
}

pub fn successors((_, term): StructureBlock<s, c>) -> IntSet::IntSet {
    IntSet::fromList(/* Expr::Generator */ Generator)
}

pub fn flipEdges(edges: IntMap::IntMap<IntSet::IntSet>) -> IntMap::IntMap<IntSet::IntSet> {
    IntMap::unionsWith(IntSet::union, /* Expr::Generator */ Generator)
}

pub fn simplifyStructure() -> Vec<Structure<s, c>> {

    let descend = |structure| {
        structure {
            structureBody: match structureBody(structure) {
                    Simple(s, term) => {
                        Simple(s, term)
                    },
                    Multiple(handlers, unhandled) => {
                        Multiple((IntMap::map(simplifyStructure, handlers)), (simplifyStructure(unhandled)))
                    },
                    Loop(body) => {
                        Loop((simplifyStructure(body)))
                    },
                }
        }
    };

    let go = |_0, _1| {
        match (_0, _1) {
            (Structure(entries, Simple(s, term)), [Structure(_, Multiple(handlers, unhandled)), rest]) => {
                __op_concat(Structure(entries, (Simple(s, (fmap(rewrite, term))))), rest)
            },
            (block, rest) => {
                __op_concat(Structure(entries, (Simple(s, (fmap(rewrite, term))))), rest)
            },
        }
    };

    foldr(go, vec![], __map!(descend))
}

pub fn depthFirstOrder(CFG(start, blocks): CFG<k, s, c>) -> CFG<DepthFirst, s, c> {

    let search = |label| {
        /*do*/ {
            let (seen, order) = get;

            if !((IntSet::member(label, seen))) { /*do*/ {
                put((IntSet::insert(label, seen), order));
                match IntMap::lookup(label, blocks) {
                    Some(BasicBlock(_, term)) => {
                        traverse_(search, term)
                    },
                    _ => {
                        ()
                    },
                };
                modify((|(seen_q, order_q)| { (seen_q, __op_concat(label, order_q)) }))
            } }
        }
    };

    let __final = snd((execState((search(start)), (IntSet::empty, vec![]))));

    let start_q = 0;

    let mapping = IntMap::fromList((zip(__final, vec![start_q::::])));

    let rewrite = |label| {
        IntMap::findWithDefault((__error!("basic block disappeared".to_string())), label, mapping)
    };

    let rewriteBlock = |label, BasicBlock(body, term)| {
        (label, BasicBlock(body, (fmap(rewrite, term))))
    };

    let blocks_q = IntMap::fromList((IntMap::elems((IntMap::intersectionWith(rewriteBlock, mapping, blocks)))));

    CFG(start_q, blocks_q)
}

pub fn structureCFG(mkBreak: fn(Option<Label>) -> s, mkContinue: fn(Option<Label>) -> s, mkLoop: fn(Label) -> fn(s) -> s, mkIf: fn(c) -> fn(s) -> fn(s) -> s, mkGoto: fn(Label) -> s, mkMatch: fn(Vec<(Label, s)>) -> fn(s) -> s, cfg: CFG<DepthFirst, s, c>) -> (bool, s) {

    let root = simplifyStructure((relooperRoot(cfg)));

    let foo = |exits, next_q| {
        snd(foldr(go, (next_q, mempty)))
    };

    (hasMultiple(root), foo(vec![], mempty, root))
}

pub fn hasMultiple() -> bool {

    let go = |_0| {
        match (_0) {
            Multiple {

                } => {
                true
            },
            Simple(_, term) => {
                true
            },
            Loop(body) => {
                true
            },
        }
    };

    any((go(structureBody)))
}



