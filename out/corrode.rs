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

    let pPrintBlock(pre, (Block(ss, e))) = |()| {
        <+>(pre, $+$(text("{".to_string()), $+$(nest(4, (vcat((++(map(pPrint, ss), vec![maybe(empty, pPrint, e)]))))), text("}".to_string()))))
    };

    let pPrintBlock(pre, (Block(vec![], e))) = |()| {
        sep(vec![<+>(pre, text("{".to_string())), nest(4, (maybe(empty, pPrint, e))), text("}".to_string())])
    };

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

    let @(castTo(target), (IsInt({

        }))((Result({
            result: Rust.Lit((Rust.LitInt(n, repr, _)))
            })))) = |()| {
        Rust.Lit((Rust.LitInt(n, repr, (toRustType(target)))))
    };

    let @(completeType(orig), (IsIncomplete(ident))) = |()| {
        {

            ;
            
        }
    };

    let @(derivedDeferredTypeOf(deferred, declr), (CDeclr(_, derived, _, _, _))(argtypes)) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretDeclarations((fromItem, makeBinding), declaration), (CDecl(specs, decls, _))) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretDeclarations(_, node), (CStaticAssert({

            }))) = |()| {
        unimplemented(node)
    };

    let @(interpretExpr(_, expr), (CBinary(op, lhs, rhs, _))) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretExpr(_, expr), (CCall(func, args, _))) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretExpr(_, expr), (CConst(c))) = |()| {
        match c {
                CIntConst, CInteger(v, repr, flags), _ => { Let([Assign([Span([Ref(Ident("allow_signed"))])], Span([Ref(Ident("not")), Parens([Span([Ref(Ident("testFlag")), Ref(Ident("FlagUnsigned")), Ref(Ident("flags"))])])])), Assign([Span([Ref(Ident("allow_unsigned"))])], Span([Ref(Ident("not")), Ref(Ident("allow_signed")), Operator("||"), Ref(Ident("repr")), Operator("/="), Ref(Ident("DecRepr"))])), Assign([Span([Ref(Ident("widths"))])], Span([Vector([Span([Parens([Span([Number(32)]), Span([Ref(Ident("if")), Ref(Ident("any")), Parens([Span([Operator("testFlag"), Ref(Ident("flags"))])]), Vector([Span([Ref(Ident("FlagLongLong"))]), Span([Ref(Ident("FlagLong"))])]), Ref(Ident("then")), Ref(Ident("WordWidth")), Ref(Ident("else")), Ref(Ident("BitWidth")), Number(32)])])]), Span([Parens([Span([Number(64)]), Span([Ref(Ident("BitWidth")), Number(64)])])])])])), Assign([Span([Ref(Ident("allowed_types"))])], Span([Dummy])), Assign([Span([Ref(Ident("repr\'"))])], Span([Case(Span([Ref(Ident("repr"))]), [Direct([Ref(Ident("DecRepr"))], [Span([Ref(Ident("Rust.DecRepr"))])]), Direct([Ref(Ident("OctalRepr"))], [Span([Ref(Ident("Rust.OctalRepr"))])]), Direct([Ref(Ident("HexRepr"))], [Span([Ref(Ident("Rust.HexRepr"))])])])]))], [])(in, match allowed_types {
                        [] => { badSource(expr, "integer (too big)".to_string()) },
                        ty, :, _ => { return((literalNumber(ty, (Rust.LitInt(v, repr'))))) },
                    }) },
                CFloatConst, CFloat(str), _ => { match span((Operator("notElem")("fF".to_string())), str) {
                        (v, "") => { return((literalNumber((IsFloat(64)), (Rust.LitFloat(v))))) },
                        (v, [_]) => { return((literalNumber((IsFloat(32)), (Rust.LitFloat(v))))) },
                        _ => { badSource(expr, "float".to_string()) },
                    } },
                CCharConst, CChar(ch, False), _ => { return(Result, {
                    resultType: charType,
                    resultMutable: Rust.Immutable,
                    result: Rust.Lit((Rust.LitByteChar(ch)))
                    }) },
                CStrConst, CString(str, False), _ => { return(Result, {
                    resultType: IsArray(Rust.Immutable, (+(length(str), 1)), charType),
                    resultMutable: Rust.Immutable,
                    result: Rust.Deref((Rust.Lit((Rust.LitByteStr((++(str, "\u{0}".to_string())))))))
                    }) },
                _ => { unimplemented(expr) },
            }
    };

    let @(interpretExpr(_, expr), (CIndex(lhs, rhs, _))) = |()| {
        {

            ;
            ;
            ;
            
        }
    };

    let @(interpretExpr(_, expr), (CMember(obj, ident, deref, node))) = |()| {
        {

            ;
            ;
            ;
            ;
            ;
            
        }
    };

    let @(interpretExpr(_, expr), (CVar(ident, _))) = |()| {
        {

            ;
            
        }
    };

    let @(interpretExpr(demand, expr), (CAssign(op, lhs, rhs, _))) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretExpr(demand, expr), (CCond(c, (Just(t)), f, _))) = |()| {
        {

            ;
            ;
            ;
            ;
            
        }
    };

    let @(interpretExpr(demand, node), (CUnary(op, expr, _))) = |()| {
        match op {
                CPreIncOp => { incdec(False, CAddAssOp) },
                CPreDecOp => { incdec(False, CSubAssOp) },
                CPostIncOp => { incdec(True, CAddAssOp) },
                CPostDecOp => { incdec(True, CSubAssOp) },
                CAdrOp => { {

                    ;
                    ;
                    
                } },
                CIndOp => { {

                    ;
                    
                } },
                CPlusOp => { {

                    ;
                    ;
                    
                } },
                CMinOp => { fmap(wrapping)(simple(Rust.Neg)) },
                CCompOp => { simple(Rust.Not) },
                CNegOp => { {

                    ;
                    
                } },
            }
    };

    let @(interpretExpr(demand, stat), (CStatExpr((CCompound(vec![], stmts, _)), _))) = |()| {
        scope({

                ;
                ;
                ;
                
            })
    };

    let @(interpretStatement(stmt), (CBreak(_))(next)) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretStatement(stmt), (CCase(expr, body, node))(next)) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretStatement(stmt), (CCases(lower, upper, body, node))(next)) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretStatement(stmt), (CCont(_))(next)) = |()| {
        {

            ;
            ;
            
        }
    };

    let @(interpretStatement(stmt), (CReturn(expr, _))(next)) = |()| {
        {

            ;
            
        }
    };

    let @(interpretStatement(stmt), (CSwitch(expr, body, node))(next)) = |()| {
        {

            ;
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            
        }
    };

    let @(toPtr(ptr), (Result({
            resultType: IsArray(mut, _, el)
            }))) = |()| {
        Just(ptr, {
            resultType: IsPtr(mut, el),
            result: castTo((IsPtr(mut, el)), ptr)
            })
    };

    let @(toPtr(ptr), (Result({
            resultType: IsPtr({

                        })
            }))) = |()| {
        Just(ptr)
    };

    let @(typeName(decl), (CDecl(spec, declarators, _))) = |()| {
        {

            ;
            ;
            ;
            ;
            
        }
    };

    let @(typeName(decl), (CStaticAssert({

            }))) = |()| {
        badSource(decl, "static assert in type name ".to_string())
    };

    let @(usual(_, b), (IsFloat(_))) = |()| {
        Just(b)
    };

    let @(usual(a), (IsFloat(_))(_)) = |()| {
        Just(a)
    };

    let @(wrapping(r), (Result({
            resultType: IsInt(Unsigned, _)
            }))) = |()| {
        match result(r) {
                Rust.Add, lhs, rhs => { r({
                    result: Rust.MethodCall(lhs, (Rust.VarName("wrapping_add".to_string())), vec![rhs])
                    }) },
                Rust.Sub, lhs, rhs => { r({
                    result: Rust.MethodCall(lhs, (Rust.VarName("wrapping_sub".to_string())), vec![rhs])
                    }) },
                Rust.Mul, lhs, rhs => { r({
                    result: Rust.MethodCall(lhs, (Rust.VarName("wrapping_mul".to_string())), vec![rhs])
                    }) },
                Rust.Div, lhs, rhs => { r({
                    result: Rust.MethodCall(lhs, (Rust.VarName("wrapping_div".to_string())), vec![rhs])
                    }) },
                Rust.Mod, lhs, rhs => { r({
                    result: Rust.MethodCall(lhs, (Rust.VarName("wrapping_rem".to_string())), vec![rhs])
                    }) },
                Rust.Neg, e => { r({
                    result: Rust.MethodCall(e, (Rust.VarName("wrapping_neg".to_string())), vec![])
                    }) },
                _ => { r },
            }
    };

    let addExternIdent(ident, deferred, mkItem) = |()| {
        {

            ;
            
        }
    };

    let addSwitchCase(condition, body, next) = |()| {
        {

            ;
            ;
            ;
            ;
            
        }
    };

    let addSymbolIdent(ident, (mut, ty)) = |()| {
        {

            ;
            ;
            
        }
    };

    let addSymbolIdentAction(ident, action) = |()| {
        lift({

                
            })
    };

    let addTagIdent(ident, ty) = |()| {
        lift({

                
            })
    };

    let addTypedefIdent(ident, ty) = |()| {
        lift({

                
            })
    };

    let applyRenames(ident) = |()| {
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
    };

    let badSource(node, msg) = |()| {
        noTranslation(node, (++("illegal ".to_string(), ++(msg, "; check whether a real C compiler accepts this".to_string()))))
    };

    let baseTypeOf(specs) = |()| {
        {

            ;
            ;
            ;
            ;
            
        }
    };

    let binop(expr, op, lhs, rhs) = |()| {
        fmap(wrapping)(match op {
                    CMulOp => { promote(expr, Rust.Mul, lhs, rhs) },
                    CDivOp => { promote(expr, Rust.Div, lhs, rhs) },
                    CRmdOp => { promote(expr, Rust.Mod, lhs, rhs) },
                    CAddOp => { match (toPtr(lhs), toPtr(rhs)) {
                            (Just(ptr), _) => { return((offset(ptr, rhs))) },
                            (_, Just(ptr)) => { return((offset(ptr, lhs))) },
                            _ => { promote(expr, Rust.Add, lhs, rhs) },
                        } },
                    CSubOp => { match (toPtr(lhs), toPtr(rhs)) {
                            (Just(lhs'), Just(rhs')) => { {

                                ;
                                ;
                                ;
                                
                            } },
                            (Just(ptr), _) => { return(ptr, {
                                result: Rust.MethodCall((result(ptr)), (Rust.VarName("offset".to_string())), vec![Rust.Neg((castTo((IsInt(Signed, WordWidth)), rhs)))])
                                }) },
                            _ => { promote(expr, Rust.Sub, lhs, rhs) },
                        } },
                    CShlOp => { shift(Rust.ShiftL) },
                    CShrOp => { shift(Rust.ShiftR) },
                    CLeOp => { comparison(Rust.CmpLT) },
                    CGrOp => { comparison(Rust.CmpGT) },
                    CLeqOp => { comparison(Rust.CmpLE) },
                    CGeqOp => { comparison(Rust.CmpGE) },
                    CEqOp => { comparison(Rust.CmpEQ) },
                    CNeqOp => { comparison(Rust.CmpNE) },
                    CAndOp => { promote(expr, Rust.And, lhs, rhs) },
                    CXorOp => { promote(expr, Rust.Xor, lhs, rhs) },
                    COrOp => { promote(expr, Rust.Or, lhs, rhs) },
                    CLndOp => { return(Result, {
                        resultType: IsBool,
                        resultMutable: Rust.Immutable,
                        result: Rust.LAnd((toBool(lhs)), (toBool(rhs)))
                        }) },
                    CLorOp => { return(Result, {
                        resultType: IsBool,
                        resultMutable: Rust.Immutable,
                        result: Rust.LOr((toBool(lhs)), (toBool(rhs)))
                        }) },
                })
    };

    let bitWidth(_, (BitWidth(w))) = |()| {
        w
    };

    let bitWidth(wordWidth, WordWidth) = |()| {
        wordWidth
    };

    let blockToStatements((Rust.Block(stmts, mexpr))) = |()| {
        match mexpr {
                Just, expr => { ++(stmts, exprToStatements(expr)) },
                Nothing => { stmts },
            }
    };

    let castTo((IsInt(Signed, w)), (Result({
        result: Rust.Neg((Rust.Lit((Rust.LitInt(n, repr, _)))))
        }))) = |()| {
        Rust.Neg((Rust.Lit((Rust.LitInt(n, repr, (toRustType((IsInt(Signed, w)))))))))
    };

    let castTo(IsBool, source) = |()| {
        toBool(source)
    };

    let castTo(target, (Result({
        resultType: IsArray(mut, _, el),
        result: source
        }))) = |()| {
        castTo(target, Result, {
            resultType: IsPtr(mut, el),
            resultMutable: Rust.Immutable,
            result: Rust.MethodCall(source, (Rust.VarName(method)), vec![])
            })
    };

    let castTo(target, source) = |()| {
        Rust.Cast((result(source)), (toRustType(target)))
    };

    let cfgToRust(_node, build) = |()| {
        {

            ;
            ;
            ;
            ;
            ;
            
        }
    };

    fn charType() -> CType {
        IsInt(Unsigned, (BitWidth(8)))
    }

    let compatibleInitializer((IsStruct(name1, _)), (IsStruct(name2, _))) = |()| {
        ==(name1, name2)
    };

    let compatibleInitializer(IsStruct, {

    }, _) = |()| {
        False
    };

    let compatibleInitializer(_, IsStruct, {

    }) = |()| {
        False
    };

    let compatibleInitializer(_, _) = |()| {
        True
    };

    let compatiblePtr((IsArray(mut, _, el)), b) = |()| {
        compatiblePtr((IsPtr(mut, el)), b)
    };

    let compatiblePtr((IsPtr(_, IsVoid)), b) = |()| {
        b
    };

    let compatiblePtr((IsPtr(m1, a)), (IsPtr(m2, b))) = |()| {
        IsPtr((leastMutable(m1, m2)), (compatiblePtr(a, b)))
    };

    let compatiblePtr(_, _) = |()| {
        IsVoid
    };

    let compatiblePtr(a, (IsArray(mut, _, el))) = |()| {
        compatiblePtr(a, (IsPtr(mut, el)))
    };

    let compatiblePtr(a, (IsPtr(_, IsVoid))) = |()| {
        a
    };

    let completeType(ty) = |()| {
        return(ty)
    };

    let compound(expr, returnOld, demand, op, lhs, rhs) = |()| {
        {

            ;
            ;
            ;
            ;
            ;
            ;
            ;
            
        }
    };

    let derivedTypeOf(deferred, declr) = |()| {
        join((derivedDeferredTypeOf(deferred, declr, vec![])))
    };

    let designatorType((Base(ty))) = |()| {
        ty
    };

    let designatorType((From(ty, _, _, _))) = |()| {
        ty
    };

    let emitIncomplete(kind, ident) = |()| {
        {

            ;
            ;
            
        }
    };

    let emitItems(items) = |()| {
        lift(tell(mempty, {
                outputItems: items
                }))
    };

    fn enumReprType() -> CType {
        IsInt(Signed, (BitWidth(32)))
    }

    let exprToStatements((Rust.BlockExpr(b))) = |()| {
        blockToStatements(b)
    };

    let exprToStatements((Rust.IfThenElse(c, t, f))) = |()| {
        vec![Rust.Stmt((Rust.IfThenElse(c, (extractExpr(t)), (extractExpr(f)))))]
    };

    let exprToStatements(e) = |()| {
        vec![Rust.Stmt(e)]
    };

    let getSwitchCases(expr) = |()| {
        mapBuildCFGT(wrap)
    };

    let getSwitchExpression(stmt) = |()| {
        {

            ;
            
        }
    };

    let getSymbolIdent(ident) = |()| {
        {

            ;
            ;
            
        }
    };

    let getTagIdent(ident) = |()| {
        lift({

                ;
                
            })
    };

    let getTypedefIdent(ident) = |()| {
        lift({

                ;
                
            })
    };

    let gotoLabel(ident) = |()| {
        {

            ;
            
        }
    };

    let intPromote((IsEnum(_))) = |()| {
        enumReprType
    };

    let intPromote(IsBool) = |()| {
        IsInt(Signed, (BitWidth(32)))
    };

    let intPromote(x) = |()| {
        x
    };

    let integerConversionRank((BitWidth(a)), (BitWidth(b))) = |()| {
        Just((compare(a, b)))
    };

    let integerConversionRank(WordWidth, WordWidth) = |()| {
        Just(EQ)
    };

    let integerConversionRank(_, _) = |()| {
        Nothing
    };

    let interpretBlockItem((CBlockDecl(decl)), next) = |()| {
        {

            ;
            ;
            
        }
    };

    let interpretBlockItem((CBlockStmt(stmt)), next) = |()| {
        interpretStatement(stmt, next)
    };

    let interpretBlockItem(item, _) = |()| {
        lift(lift((unimplemented(item))))
    };

    let interpretConstExpr((CConst((CIntConst((CInteger(v, _, _)), _))))) = |()| {
        return(v)
    };

    let interpretConstExpr(expr) = |()| {
        unimplemented(expr)
    };

    let interpretExpr(_, (CAlignofExpr(e, _))) = |()| {
        {

            ;
            
        }
    };

    let interpretExpr(_, (CAlignofType(decl, _))) = |()| {
        {

            ;
            
        }
    };

    let interpretExpr(_, (CCast(decl, expr, _))) = |()| {
        {

            ;
            ;
            
        }
    };

    let interpretExpr(_, (CCompoundLit(decl, initials, info))) = |()| {
        {

            ;
            ;
            
        }
    };

    let interpretExpr(_, (CSizeofExpr(e, _))) = |()| {
        {

            ;
            
        }
    };

    let interpretExpr(_, (CSizeofType(decl, _))) = |()| {
        {

            ;
            
        }
    };

    let interpretExpr(_, expr) = |()| {
        unimplemented(expr)
    };

    let interpretExpr(demand, (CComma(exprs, _))) = |()| {
        {

            ;
            ;
            ;
            
        }
    };

    let interpretFunction((@(CFunDef(specs, declr), (CDeclr(mident, _, _, _, _))(argtypes, body, _)))) = |()| {
        {

            ;
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            
        }
    };

    let interpretInitializer(ty, initial) = |()| {
        {

            ;
            ;
            ;
            
        }
    };

    let interpretStatement((CCompound(vec![], items, _)), next) = |()| {
        mapBuildCFGT((mapRWST(scope)))({

                
            })
    };

    let interpretStatement((CDefault(body, _)), next) = |()| {
        addSwitchCase(Nothing, body, next)
    };

    let interpretStatement((CExpr((Just(expr)), _)), next) = |()| {
        {

            ;
            ;
            
        }
    };

    let interpretStatement((CExpr(Nothing, _)), next) = |()| {
        next
    };

    let interpretStatement((CFor(initial, mcond, mincr, body, _)), next) = |()| {
        {

            ;
            ;
            ;
            ;
            
        }
    };

    let interpretStatement((CGoto(ident, _)), next) = |()| {
        {

            ;
            ;
            
        }
    };

    let interpretStatement((CIf(c, t, mf, _)), next) = |()| {
        {

            ;
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            
        }
    };

    let interpretStatement((CLabel(ident, body, _, _)), next) = |()| {
        {

            ;
            ;
            ;
            
        }
    };

    let interpretStatement((CWhile(c, body, doWhile, _)), next) = |()| {
        {

            ;
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            
        }
    };

    let interpretStatement(stmt, _) = |()| {
        lift(lift(unimplemented(stmt)))
    };

    let interpretTranslationUnit(_thisModule, rewrites, (CTranslUnit(decls, _))) = |()| {
        match err {
                Left, msg => { Left(msg) },
                Right, _ => { Right(items') },
            }
    };

    fn makeLetBinding() -> MakeBinding {
        (Rust.StmtItem(vec![]), makeBinding)
    }

    fn makeStaticBinding() -> MakeBinding {
        (Rust.Item(vec![], Rust.Private), makeBinding)
    }

    let modifyGlobal(f) = |()| {
        lift({

                ;
                ;
                ;
                
            })
    };

    let mutable(quals) = |()| {
        if(any, (Lambda), quals, then, Rust.Immutable, else, Rust.Mutable)
    };

    let nestedObject(ty, desig) = |()| {
        match designatorType(desig) {
                IsArray, _, size, el => { Just((From(el, 0, (replicate((-(size, 1)), el)), desig))) },
            ty' => if compatibleInitializer(ty, ty') { Just(desig) },
                IsStruct, _, (_, ty')(:, fields) => { nestedObject(ty, (From(ty', 0, (map(snd, fields)), desig))) },
                _ => { Nothing },
            }
    };

    let nextObject((From(_, _, vec![], base))) = |()| {
        nextObject(base)
    };

    let nextObject((From(_, i, (:(ty, remaining)), base))) = |()| {
        Just((From(ty, (+(i, 1)), remaining, base)))
    };

    let nextObject(Base, {

    }) = |()| {
        Nothing
    };

    let noTranslation(node, msg) = |()| {
        throwE(concat(vec![show((posOf(node))), ": ".to_string(), msg, ":\n".to_string(), render((nest(4, (pretty(node)))))]))
    };

    let objectFromDesignators(_, vec![]) = |()| {
        pure(Nothing)
    };

    let objectFromDesignators(ty, desigs) = |()| {
        <$>(Just, go(ty, desigs, (Base(ty))))
    };

    let promote(node, op, a, b) = |()| {
        match usual((resultType(a)), (resultType(b))) {
                Just, rt => { return(Result, {
                    resultType: rt,
                    resultMutable: Rust.Immutable,
                    result: op((castTo(rt, a)), (castTo(rt, b)))
                    }) },
                Nothing => { badSource(node)(concat(vec!["arithmetic combination for ".to_string(), show((resultType(a))), " and ".to_string(), show((resultType(b)))])) },
            }
    };

    let promotePtr(node, op, a, b) = |()| {
        match (resultType(a), resultType(b)) {
                (IsArray(_, _, _), _) => { ptrs },
                (IsPtr(_, _), _) => { ptrs },
                (_, IsArray(_, _, _)) => { ptrs },
                (_, IsPtr(_, _)) => { ptrs },
                _ => { promote(node, op, a, b) },
            }
    };

    let resolveCurrentObject((obj0, prior), (obj1, cinitial)) = |()| {
        match mplus(obj1, obj0) {
                Nothing => { return((Nothing, prior)) },
                Just, obj => { {

                    ;
                    ;
                    ;
                    
                } },
            }
    };

    fn resultToStatements() -> Vec<Rust.Stmt> {
        (exprToStatements . result)
    }

    let runOnce(action) = |()| {
        {

            ;
            
        }
    };

    let rustAlignOfType((Rust.TypeName(ty))) = |()| {
        Result({
            resultType: IsInt(Unsigned, WordWidth),
            resultMutable: Rust.Immutable,
            result: Rust.Call((Rust.Var((Rust.VarName((++("::std::mem::align_of::<".to_string(), ++(ty, ">".to_string()))))))), vec![])
            })
    };

    let rustSizeOfType((Rust.TypeName(ty))) = |()| {
        Result({
            resultType: IsInt(Unsigned, WordWidth),
            resultMutable: Rust.Immutable,
            result: Rust.Call((Rust.Var((Rust.VarName((++("::std::mem::size_of::<".to_string(), ++(ty, ">".to_string()))))))), vec![])
            })
    };

    let scalar(expr) = |()| {
        Initializer((Just(expr)), IntMap.empty)
    };

    let scope(m) = |()| {
        {

            ;
            ;
            ;
            
        }
    };

    let setBreak(label) = |()| {
        mapBuildCFGT((local((Lambda({
                    onBreak: Just(label)
                    })))))
    };

    let setContinue(label) = |()| {
        mapBuildCFGT((local((Lambda({
                    onContinue: Just(label)
                    })))))
    };

    let statementsToBlock(stmts) = |()| {
        Rust.Block(stmts, Nothing)
    };

    let statementsToBlock(vec![Rust.Stmt((Rust.BlockExpr(stmts)))]) = |()| {
        stmts
    };

    let toBool((Result({
        result: Rust.Lit((Rust.LitInt(0, _, _)))
        }))) = |()| {
        Rust.Lit((Rust.LitBool(False)))
    };

    let toBool((Result({
        result: Rust.Lit((Rust.LitInt(1, _, _)))
        }))) = |()| {
        Rust.Lit((Rust.LitBool(True)))
    };

    let toBool((Result({
        resultType: t,
        result: v
        }))) = |()| {
        match t {
                IsBool => { v },
                IsPtr, _, _ => { Rust.Not((Rust.MethodCall(v, (Rust.VarName("is_null".to_string())), vec![]))) },
                _ => { Rust.CmpNE(v, 0) },
            }
    };

    let toNotBool((Result({
        result: Rust.Lit((Rust.LitInt(0, _, _)))
        }))) = |()| {
        Rust.Lit((Rust.LitBool(True)))
    };

    let toNotBool((Result({
        result: Rust.Lit((Rust.LitInt(1, _, _)))
        }))) = |()| {
        Rust.Lit((Rust.LitBool(False)))
    };

    let toNotBool((Result({
        resultType: t,
        result: v
        }))) = |()| {
        match t {
                IsBool => { Rust.Not(v) },
                IsPtr, _, _ => { Rust.MethodCall(v, (Rust.VarName("is_null".to_string())), vec![]) },
                _ => { Rust.CmpEQ(v, 0) },
            }
    };

    let toPtr(_) = |()| {
        Nothing
    };

    let toRustRetType(IsVoid) = |()| {
        Rust.TypeName("()".to_string())
    };

    let toRustRetType(ty) = |()| {
        toRustType(ty)
    };

    let toRustType((IsArray(_, size, el))) = |()| {
        Rust.TypeName((++("[".to_string(), ++(typename(el), ++("; ".to_string(), ++(show(size), "]".to_string()))))))
    };

    let toRustType((IsEnum(name))) = |()| {
        Rust.TypeName(name)
    };

    let toRustType((IsFloat(w))) = |()| {
        Rust.TypeName((:('f', show(w))))
    };

    let toRustType((IsFunc(retTy, args, variadic))) = |()| {
        Rust.TypeName(concat(vec!["unsafe extern fn(".to_string(), args', ")".to_string(), /=(if(retTy), ++(IsVoid(then, " -> ".to_string()), typename(retTy, else, "".to_string())))]))
    };

    let toRustType((IsIncomplete(ident))) = |()| {
        Rust.TypeName((identToString(ident)))
    };

    let toRustType((IsInt(s, w))) = |()| {
        Rust.TypeName((:((match s {
                            Signed => { 'i' },
                            Unsigned => { 'u' },
                        }), (match w {
                            BitWidth, b => { show(b) },
                            WordWidth => { "size".to_string() },
                        }))))
    };

    let toRustType((IsPtr(mut, to))) = |()| {
        Let([Assign([Span([Ref(Ident("Rust.TypeName")), Ref(Ident("to\'"))])], Span([Ref(Ident("toRustType")), Ref(Ident("to")), Ref(Ident("in")), Ref(Ident("Rust.TypeName")), Parens([Span([Ref(Ident("rustMut")), Ref(Ident("mut")), Operator("++"), Ref(Ident("to\'"))])])]))], [])
    };

    let toRustType((IsStruct(name, _fields))) = |()| {
        Rust.TypeName(name)
    };

    let toRustType(IsBool) = |()| {
        Rust.TypeName("bool".to_string())
    };

    let toRustType(IsVoid) = |()| {
        Rust.TypeName("::std::os::raw::c_void".to_string())
    };

    let translateInitList(ty, list) = |()| {
        {

            ;
            ;
            ;
            
        }
    };

    let typeToResult(itype, expr) = |()| {
        Result({
            resultType: typeRep(itype),
            resultMutable: typeMutable(itype),
            result: expr
            })
    };

    let unimplemented(node) = |()| {
        noTranslation(node, "Corrode doesn\'t handle this yet".to_string())
    };

    let uniqueName(base) = |()| {
        modifyGlobal(Lambda)
    };

    let useForwardRef(ident) = |()| {
        modifyGlobal(Lambda)
    };

    let usual((IsFloat(aw)), (IsFloat(bw))) = |()| {
        Just((IsFloat((max(aw, bw)))))
    };

    let usual(origA, origB) = |()| {
        match (intPromote(origA), intPromote(origB)) {
            (a, b) => if ==(a, b) { Just(a) },
                (IsInt(Signed, sw), IsInt(Unsigned, uw)) => { mixedSign(sw, uw) },
                (IsInt(Unsigned, uw), IsInt(Signed, sw)) => { mixedSign(sw, uw) },
                (IsInt(as, aw), IsInt(_bs, bw)) => { {

                    ;
                    
                } },
                _ => { Nothing },
            }
    };

    let wrapMain(declr, realName, argTypes) = |()| {
        {

            ;
            ;
            ;
            ;
                            let wrapArgv(vec![]) = |()| {
                    return((vec![], vec![]))
                };
;
            ;
                            let wrapArgv(_) = |()| {
                    unimplemented(declr)
                };
;
                            let wrapEnvp(vec![]) = |()| {
                    return((vec![], vec![]))
                };
;
            ;
                            let wrapEnvp(_) = |()| {
                    unimplemented(declr)
                };

        }
    };

    let wrapping(r) = |()| {
        r
    };

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

    let addBlock(label, stmt, terminator) = |()| {
        {

            
        }
    };

    let buildCFG(root) = |()| {
        {

            ;
            
        }
    };

    let depthFirstOrder((CFG(start, blocks))) = |()| {
        CFG(start', blocks')
    };

    let flipEdges(edges) = |()| {
        IntMap.unionsWith(IntSet.union, Dummy)
    };

    fn hasMultiple() -> Bool {
        any(((go . structureBody)))
    }

    fn mapBuildCFGT() -> BuildCFGT {
        mapStateT
    }

    fn newLabel() -> Monad {
        {

            ;
            ;
            
        }
    }

    let outEdges(blocks) = |()| {
        IntSet.difference(IntSet.unions((map(successors)(IntMap.elems(blocks)))), IntMap.keysSet(blocks))
    };

    let partitionMembers(a, b) = |()| {
        (IntSet.intersection(a, b), IntSet.difference(a, b))
    };

    let prettyCFG(fmtS, fmtC, (CFG(entry, blocks))) = |()| {
        vcat(:((<>(text("start @".to_string()), text((show(entry))))), blocks'))
    };

    fn prettyStructure() -> Doc {
        (vcat . map(go))
    }

    let relooper(entries, blocks) = |()| {
        Let([Assign([Span([Parens([Span([Ref(Ident("returns"))]), Span([Ref(Ident("noreturns"))])])])], Span([Ref(Ident("partitionMembers")), Ref(Ident("entries")), Operator("$"), Ref(Ident("IntSet.unions")), Operator("$"), Ref(Ident("map")), Ref(Ident("successors")), Operator("$"), Ref(Ident("IntMap.elems")), Ref(Ident("blocks"))])), Assign([Span([Parens([Span([Ref(Ident("present"))]), Span([Ref(Ident("absent"))])])])], Span([Ref(Ident("partitionMembers")), Ref(Ident("entries")), Parens([Span([Ref(Ident("IntMap.keysSet")), Ref(Ident("blocks"))])])]))], [])(in, match (IntSet.toList(noreturns), IntSet.toList(returns)) {
                ([], []) => { vec![] },
                ([entry], []) => { match IntMap.updateLookupWithKey((Lambda), entry, blocks) {
                        (Just((s, term)), blocks') => { :(Structure({
                                structureEntries: entries,
                                structureBody: Simple(s, term)
                                }), relooper((successors((s, term))), blocks')) },
                        (Nothing, _) => { :(Structure({
                                structureEntries: entries,
                                structureBody: Simple(mempty, (Branch((GoTo(entry)))))
                                }), vec![]) },
                    } },
            _ => if not((IntSet.null(absent))) { :(if(IntSet.null, present, then, vec![], else, Structure, {
                    structureEntries: entries,
                    structureBody: Multiple((IntMap.fromSet((const(vec![])), absent)), (relooper(present, blocks)))
                    }), vec![]) },
                ([], _) => { :(Structure({
                        structureEntries: entries,
                        structureBody: Loop((relooper(entries, blocks')))
                        }), relooper(followEntries, followBlocks)) },
                _ => { :(Structure({
                        structureEntries: entries,
                        structureBody: Multiple(handlers, unhandled)
                        }), relooper(followEntries, followBlocks)) },
            })
    };

    let relooperRoot((CFG(entry, blocks))) = |()| {
        relooper((IntSet.singleton(entry)))(IntMap.map((Lambda), blocks))
    };

    let removeEmptyBlocks((CFG(start, blocks))) = |()| {
        CFG((rewrite(start)), blocks')
    };

    let restrictKeys(m, s) = |()| {
        IntMap.intersection(m, IntMap.fromSet((const(())), s))
    };

    fn simplifyStructure() -> Monoid {
        (foldr(go, vec![]) . map(descend))
    }

    let structureCFG(mkBreak, mkContinue, mkLoop, mkIf, mkGoto, mkMatch, cfg) = |()| {
        (hasMultiple(root), foo(vec![], mempty, root))
    };

    let successors((_, term)) = |()| {
        IntSet.fromList(Dummy)
    };

}

mod Language_Rust_Corrode_CrateMap {
    #[derive(Debug, Eq, Ord)]
    struct ItemKind(Enum, Struct, Union, Type, Symbol);

    fn mergeCrateMaps() -> Map.Map {
        Map.fromListWith((Map.unionWith((Operator("++")))))
    }

    fn parseCrateMap() -> Either {
        (fmap(root) . (foldrM(parseLine, (Map.empty, vec![])) . (filter(((not . null))) . (map(cleanLine) . lines))))
    }

    let rewritesFromCratesMap(crates) = |()| {
        Map.fromList(Dummy)
    };

    let splitModuleMap(modName, crates) = |()| {
        fromMaybe((vec![], crates))({

                ;
                ;
                ;
                ;
                
            })
    };

}

mod Language_Rust_Idiomatic {
    let itemIdioms((Rust.Item(attrs, vis, (Rust.Function(fattrs, name, formals, ret, b))))) = |()| {
        Rust.Item(attrs, vis, (Rust.Function(fattrs, name, formals, ret, (tailBlock(b)))))
    };

    let itemIdioms(i) = |()| {
        i
    };

    let tailBlock((Rust.Block(Dummy, Nothing))) = |()| {
        Rust.Block(b, e)
    };

    let tailBlock((Rust.Block(b, (Just(Dummy))))) = |()| {
        Rust.Block(b, e)
    };

    let tailBlock(b) = |()| {
        b
    };

    let tailExpr((Rust.BlockExpr(b))) = |()| {
        Just((Just((Rust.BlockExpr((tailBlock(b)))))))
    };

    let tailExpr((Rust.IfThenElse(c, t, f))) = |()| {
        Just((Just((Rust.IfThenElse(c, (tailBlock(t)), (tailBlock(f)))))))
    };

    let tailExpr((Rust.Return(e))) = |()| {
        Just(e)
    };

    let tailExpr(_) = |()| {
        Nothing
    };

    let unsnoc((x:xs)) = |()| {
        match unsnoc(xs) {
                Just, (a, b) => { Just((x:a, b)) },
                Nothing => { Just((vec![], x)) },
            }
    };

    let unsnoc(vec![]) = |()| {
        Nothing
    };

}

mod Language_Rust {

}



fn main() { /* demo */ }
