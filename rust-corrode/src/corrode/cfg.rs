mod haskell_support {
    pub trait Addable {
        fn add(self, right: Self) -> Self;
    }

    impl Addable for String {
        fn add(self, right: Self) -> Self {
            format!("{}{}", self, right)
        }
    }

    pub fn __op_addadd<A: Addable>(left: A, right: A) -> A {
        Addable::add(left, right)
    }
}


pub mod Language_Rust_Corrode_CFG {
    use haskell_support::*;

use Control::Monad;
use Control::Monad::Trans::State;
use Data::Foldable;
use Data::IntMap::Lazy;
use Data::IntSet;
use Data::Maybe;
use Data::Traversable;
use Text::PrettyPrint::HughesPJClass;

    struct BasicBlock<s, c>(s, Terminator<c>);

    #[derive(Debug)]
    pub enum Terminator_q<c, l> {
        Unreachable,
        Branch(l),
        CondBranch(c, l, l)
    }
    pub use self::Terminator_q::*;

    struct Unordered;

    struct DepthFirst;

    struct CFG<k, s, c>(Label, IntMap::IntMap<BasicBlock<s, c>>);

    struct BuildState<s, c>{
        buildLabel: Label,
        buildBlocks: IntMap::IntMap<BasicBlock<s, c>>
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

    #[derive(Debug)]
    pub enum Structure_q<s, c, a> {
        Simple(s, StructureTerminator<s, c>),
        Loop(a),
        Multiple(IntMap::IntMap<a>, a)
    }
    pub use self::Structure_q::*;

    #[derive(Debug)]
    struct Structure<s, c>{
        structureEntries: IntSet::IntSet,
        structureBody: Structure_q<s, c, Vec<Structure<s, c>>>
    }

    pub fn addBlock(label: Label, stmt: s, terminator: Terminator<c>) -> BuildCFGT<m, s, c, ()> {
        /* do */ {
            modify(|st| { st {
                    buildBlocks: IntMap::insert(label, (BasicBlock(stmt, terminator)), (buildBlocks(st)))
                } })
        }
    }

    pub fn buildCFG(root: BuildCFGT<m, s, c, Label>) -> m<CFG<Unordered, s, c>> {
        /* do */ {
            let (label, __final) = runStateT(root, (BuildState(0, IntMap::empty)));

            (CFG(label, (buildBlocks(__final))))
        }
    }

    pub fn depthFirstOrder(CFG(start, blocks): CFG<k, s, c>) -> CFG<DepthFirst, s, c> {
        CFG(start_q, blocks_q)
    }

    pub fn flipEdges(edges: IntMap::IntMap<IntSet::IntSet>) -> IntMap::IntMap<IntSet::IntSet> {
        IntMap::unionsWith(IntSet::union, /* Expr::Generator */ Generator)
    }

    pub fn hasMultiple() -> bool {
        any((go(structureBody)))
    }

    pub fn mapBuildCFGT() -> BuildCFGT<n, s, c, b> {
        mapStateT
    }

    pub fn newLabel() -> BuildCFGT<m, s, c, Label> {
        /* do */ {
            let old = get;

            put(old {
                    buildLabel: (buildLabel(old) + 1)
                });
            (buildLabel(old))
        }
    }

    pub fn outEdges(blocks: IntMap::IntMap<StructureBlock<s, c>>) -> IntSet::IntSet {
        IntSet::difference(IntSet::unions((map(successors)(IntMap::elems(blocks)))), IntMap::keysSet(blocks))
    }

    pub fn partitionMembers(a: IntSet::IntSet, b: IntSet::IntSet) -> (IntSet::IntSet, IntSet::IntSet) {
        (IntSet::intersection(a, b), IntSet::difference(a, b))
    }

    pub fn prettyCFG(fmtS: fn(s) -> Doc, fmtC: fn(c) -> Doc, CFG(entry, blocks): CFG<k, s, c>) -> Doc {
        vcat(__op_concat((__op_ne(text("start @".to_string()), text((show(entry))))), blocks_q))
    }

    pub fn prettyStructure() -> Doc {
        vcat(map(go))
    }

    pub fn relooper(entries: IntSet::IntSet, blocks: IntMap::IntMap<StructureBlock<s, c>>) -> Vec<Structure<s, c>> {
        {
            let (returns, noreturns) = partitionMembers(entries)(IntSet::unions(map(successors)(IntMap::elems(blocks))));

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
                _ if not((IntSet::null(absent))) => { __op_concat(__TODO_if(IntSet::null, present, then, vec![], __TODO_else, Structure {
                        structureEntries: entries,
                        structureBody: Multiple((IntMap::fromSet((__TODO_const(vec![])), absent)), (relooper(present, blocks)))
                    }), vec![]) }
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
            }        }
    }

    pub fn relooperRoot(CFG(entry, blocks): CFG<k, s, c>) -> Vec<Structure<s, c>> {
        relooper((IntSet::singleton(entry)))(IntMap::map((|BasicBlock(s, term)| { (s, fmap(GoTo, term)) }), blocks))
    }

    pub fn removeEmptyBlocks(CFG(start, blocks): CFG<k, f<s>, c>) -> CFG<Unordered, f<s>, c> {
        CFG((rewrite(start)), blocks_q)
    }

    pub fn restrictKeys(m: IntMap::IntMap<a>, s: IntSet::IntSet) -> IntMap::IntMap<a> {
        IntMap::intersection(m, IntMap::fromSet((__TODO_const(())), s))
    }

    pub fn simplifyStructure() -> Vec<Structure<s, c>> {
        foldr(go, vec![], map(descend))
    }

    pub fn structureCFG(mkBreak: fn(Option<Label>) -> s, mkContinue: fn(Option<Label>) -> s, mkLoop: fn(Label) -> fn(s) -> s, mkIf: fn(c) -> fn(s) -> fn(s) -> s, mkGoto: fn(Label) -> s, mkMatch: fn(Vec<(Label, s)>) -> fn(s) -> s, cfg: CFG<DepthFirst, s, c>) -> (bool, s) {
        (hasMultiple(root), foo(vec![], mempty, root))
    }

    pub fn successors((_, term): StructureBlock<s, c>) -> IntSet::IntSet {
        IntSet::fromList(/* Expr::Generator */ Generator)
    }

}




