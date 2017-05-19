mod Language_Rust_AST {
    #[derive(Debug, Eq)]
    struct LitIntRepr(DecRepr, OctalRepr, HexRepr);

    #[derive(Debug, Eq)]
    struct Lit(LitByteStr, String, LitByteChar, Char, LitBool, Bool, LitInt, Integer, LitIntRepr, Type, LitFloat, String, Type);

    #[derive(Debug, Eq)]
    struct Visibility(Public, Private);

    #[derive(Debug, Eq)]
    struct Mutable(Immutable, Mutable);

    #[derive(Debug)]
    struct Stmt(Stmt, Expr, Let, Mutable, Var, Maybe(Type), Maybe(Expr), StmtItem, Vec<Attribute>, ItemKind);

    #[derive(Debug)]
    struct Block(Block, Vec<Stmt>, Maybe(Expr));

    #[derive(Debug)]
    struct Attribute(Attribute, String);

    #[derive(Debug)]
    struct Item(Item, Vec<Attribute>, Visibility, ItemKind);

    #[derive(Debug)]
    struct FunctionAttribute(UnsafeFn, ExternABI, Maybe(String));

    #[derive(Debug)]
    struct ItemKind(Function, Vec<FunctionAttribute>, String, Vec<(Mutable, Var, Type)>, Type, Block, Static, Mutable, Var, Type, Expr, Struct, String, Vec<(String, Type)>, Extern, Vec<ExternItem>, Use, String, Enum, String, Vec<Enumerator>, CloneImpl, Type);

    #[derive(Debug)]
    struct ExternItem(ExternFn, String, Vec<(Var, Type)>, Bool, Type, ExternStatic, Mutable, Var, Type);

    #[derive(Debug)]
    struct Enumerator(EnumeratorAuto, String, EnumeratorExpr, String, Expr);

    #[derive(Debug)]
    struct Expr(Lit, Lit, Var, Var, Path, Path, Index, Expr, Expr, ArrayExpr, Vec<Expr>, RepeatArray, Expr, Expr, StructExpr, String, Vec<(String, Expr)>, Maybe(Expr), Call, Expr, Vec<Expr>, MethodCall, Expr, Var, Vec<Expr>, Lambda, Vec<Var>, Expr, Member, Expr, Var, BlockExpr, Block, UnsafeExpr, Block, IfThenElse, Expr, Block, Block, Loop, Maybe(Lifetime), Block, While, Maybe(Lifetime), Expr, Block, For, Maybe(Lifetime), Var, Expr, Block, Break, Maybe(Lifetime), Continue, Maybe(Lifetime), Return, Maybe(Expr), Neg, Expr, Deref, Expr, Not, Expr, Borrow, Mutable, Expr, Cast, Expr, Type, Mul, Expr, Expr, Div, Expr, Expr, Mod, Expr, Expr, Add, Expr, Expr, Sub, Expr, Expr, ShiftL, Expr, Expr, ShiftR, Expr, Expr, And, Expr, Expr, Xor, Expr, Expr, Or, Expr, Expr, CmpLT, Expr, Expr, CmpGT, Expr, Expr, CmpLE, Expr, Expr, CmpGE, Expr, Expr, CmpEQ, Expr, Expr, CmpNE, Expr, Expr, LAnd, Expr, Expr, LOr, Expr, Expr, Range, Expr, Expr, Assign, Expr, AssignOp, Expr);

    #[derive(Debug)]
    struct AssignOp(:=, :+=, :-=, :*=, :/=, :%=, :&=, :|=, :^=, :<<=, :>>=);

    #[derive(Debug)]
    struct ExprPosition(TopExpr, LeftExpr, RightExpr);


}

mod Language_Rust_Corrode_C {
    struct FunctionContext(FunctionContext, { /* struct def */ });

    struct Output(Output, { /* struct def */ });

    struct GlobalState(GlobalState, { /* struct def */ });

    struct EnvState(EnvState, { /* struct def */ });

    struct Initializer(Initializer, Maybe(Rust.Expr), IntMap.IntMap(Initializer));

    #[derive(Debug)]
    struct Designator(Base, CType, From, CType, isize, Vec<CType>, Designator);

    struct OuterLabels(OuterLabels, { /* struct def */ });

    struct Result(Result, { /* struct def */ });

    #[derive(Debug, Eq)]
    struct Signed(Signed, Unsigned);

    #[derive(Debug, Eq)]
    struct IntWidth(BitWidth, isize, WordWidth);

    #[derive(Debug)]
    struct CType(IsBool, IsInt, Signed, IntWidth, IsFloat, isize, IsVoid, IsFunc, CType, Vec<(Maybe((Rust.Mutable, Ident)), CType)>, Bool, IsPtr, Rust.Mutable, CType, IsArray, Rust.Mutable, isize, CType, IsStruct, String, Vec<(String, CType)>, IsEnum, String, IsIncomplete, Ident);

    struct IntermediateType(IntermediateType, { /* struct def */ });


}

mod Language_Rust_Corrode_CFG {
    struct BasicBlock(BasicBlock, s, Terminator(c));

    #[derive(Debug)]
    struct Terminator'(Unreachable, Branch, l, CondBranch, c, l, l);

    struct Unordered();

    struct DepthFirst();

    struct CFG(CFG, Label, IntMap.IntMap(BasicBlock(s, c)));

    struct BuildState(BuildState, { /* struct def */ });

    #[derive(Debug)]
    struct StructureLabel(GoTo, { /* struct def */ }, ExitTo, { /* struct def */ }, Nested, Vec<Structure(s, c)>);

    #[derive(Debug)]
    struct Structure'(Simple, s, StructureTerminator(s, c), Loop, a, Multiple, IntMap.IntMap(a), a);

    #[derive(Debug)]
    struct Structure(Structure, { /* struct def */ });


}

mod Language_Rust_Corrode_CrateMap {
    #[derive(Debug, Eq, Ord)]
    struct ItemKind(Enum, Struct, Union, Type, Symbol);


}

mod Language_Rust_Idiomatic {

}

mod Language_Rust {

}



fn main() { /* demo */ }
