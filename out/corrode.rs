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

            let mty = getTagIdent(ident);
            fromMaybe((return(orig)), mty)
        }
    };

    let @(derivedDeferredTypeOf(deferred, declr), (CDeclr(_, derived, _, _, _))(argtypes)) = |()| {
        {

            let derived' = mapM(derive, derived);
            return({

        let basetype = deferred;
        foldrM((Operator("$")), basetype, derived')
    });
            
        }
    };

    let @(interpretDeclarations((fromItem, makeBinding), declaration), (CDecl(specs, decls, _))) = |()| {
        {

            let (storagespecs, baseTy) = baseTypeOf(specs);
            let mbinds = forM(decls)(Lambda);
            return((catMaybes(mbinds)))
        }
    };

    let @(interpretDeclarations(_, node), (CStaticAssert({

            }))) = |()| {
        unimplemented(node)
    };

    let @(interpretExpr(_, expr), (CBinary(op, lhs, rhs, _))) = |()| {
        {

            let lhs' = interpretExpr(True, lhs);
            let rhs' = interpretExpr(True, rhs);
            binop(expr, op, lhs', rhs')
        }
    };

    let @(interpretExpr(_, expr), (CCall(func, args, _))) = |()| {
        {

            let func' = interpretExpr(True, func);
            match resultType(func') {
        IsFunc, retTy, argTys, variadic => { {

            let args' = castArgs(variadic, (map(snd, argTys)), args);
            return(Result, {
    resultType: retTy,
    resultMutable: Rust.Immutable,
    result: Rust.Call((result(func')), args')
    })
        } },
        _ => { badSource(expr, "function call to non-function".to_string()) },
    };
            
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

            let lhs' = interpretExpr(True, lhs);
            let rhs' = interpretExpr(True, rhs);
            match (resultType(lhs'), resultType(rhs')) {
        (IsArray(mut, _, el), _) => { return((subscript(mut, el, (result(lhs')), rhs'))) },
        (_, IsArray(mut, _, el)) => { return((subscript(mut, el, (result(rhs')), lhs'))) },
        _ => { {

            let ptr = binop(expr, CAddOp, lhs', rhs');
            match resultType(ptr) {
        IsPtr, mut, ty => { return(Result, {
            resultType: ty,
            resultMutable: mut,
            result: Rust.Deref((result(ptr)))
            }) },
        _ => { badSource(expr, "array subscript of non-pointer".to_string()) },
    }
        } },
    };
            
        }
    };

    let @(interpretExpr(_, expr), (CMember(obj, ident, deref, node))) = |()| {
        {

            let obj' = interpretExpr(True)(if(deref, then, CUnary, CIndOp, obj, node, else, obj));
            let objTy = completeType((resultType(obj')));
            let fields = match objTy {
            IsStruct, _, fields => { return(fields) },
            _ => { badSource(expr, "member access of non-struct".to_string()) },
        };
            Let([Assign([Span([Ref(Ident("field"))])], Span([Ref(Ident("applyRenames")), Ref(Ident("ident"))]))], []);
            let ty = match lookup(field, fields) {
            Just, ty => { return(ty) },
            Nothing => { badSource(expr, "request for non-existent field".to_string()) },
        };
            return(Result, {
    resultType: ty,
    resultMutable: resultMutable(obj'),
    result: Rust.Member((result(obj')), (Rust.VarName(field)))
    })
        }
    };

    let @(interpretExpr(_, expr), (CVar(ident, _))) = |()| {
        {

            let sym = getSymbolIdent(ident);
            maybe((badSource(expr, "undefined variable".to_string())), return, sym)
        }
    };

    let @(interpretExpr(demand, expr), (CAssign(op, lhs, rhs, _))) = |()| {
        {

            let lhs' = interpretExpr(True, lhs);
            let rhs' = interpretExpr(True, rhs);
            compound(expr, False, demand, op, lhs', rhs')
        }
    };

    let @(interpretExpr(demand, expr), (CCond(c, (Just(t)), f, _))) = |()| {
        {

            let c' = fmap(toBool, (interpretExpr(True, c)));
            let t' = interpretExpr(demand, t);
            let f' = interpretExpr(demand, f);
            if(demand, then, promotePtr, expr, (mkIf(c')), t', f', else, return, Result, {
    resultType: IsVoid,
    resultMutable: Rust.Immutable,
    result: mkIf(c', (result(t')), (result(f')))
    });
            
        }
    };

    let @(interpretExpr(demand, node), (CUnary(op, expr, _))) = |()| {
        match op {
                CPreIncOp => { incdec(False, CAddAssOp) },
                CPreDecOp => { incdec(False, CSubAssOp) },
                CPostIncOp => { incdec(True, CAddAssOp) },
                CPostDecOp => { incdec(True, CSubAssOp) },
                CAdrOp => { {

                    let expr' = interpretExpr(True, expr);
                    Let([Assign([Span([Ref(Ident("ty\'"))])], Span([Ref(Ident("IsPtr")), Parens([Span([Ref(Ident("resultMutable")), Ref(Ident("expr\'"))])]), Parens([Span([Ref(Ident("resultType")), Ref(Ident("expr\'"))])])]))], []);
                    return(Result, {
    resultType: ty',
    resultMutable: Rust.Immutable,
    result: Rust.Cast((Rust.Borrow((resultMutable(expr')), (result(expr')))), (toRustType(ty')))
    })
                } },
                CIndOp => { {

                    let expr' = interpretExpr(True, expr);
                    match resultType(expr') {
        IsPtr, mut', ty' => { return(Result, {
            resultType: ty',
            resultMutable: mut',
            result: Rust.Deref((result(expr')))
            }) },
        IsFunc, { .. } => { return(expr') },
        _ => { badSource(node, "dereference of non-pointer".to_string()) },
    }
                } },
                CPlusOp => { {

                    let expr' = interpretExpr(demand, expr);
                    Let([Assign([Span([Ref(Ident("ty\'"))])], Span([Ref(Ident("intPromote")), Parens([Span([Ref(Ident("resultType")), Ref(Ident("expr\'"))])])]))], []);
                    return(Result, {
    resultType: ty',
    resultMutable: Rust.Immutable,
    result: castTo(ty', expr')
    })
                } },
                CMinOp => { fmap(wrapping)(simple(Rust.Neg)) },
                CCompOp => { simple(Rust.Not) },
                CNegOp => { {

                    let expr' = interpretExpr(True, expr);
                    return(Result, {
    resultType: IsBool,
    resultMutable: Rust.Immutable,
    result: toNotBool(expr')
    })
                } },
            }
    };

    let @(interpretExpr(demand, stat), (CStatExpr((CCompound(vec![], stmts, _)), _))) = |()| {
        scope({

                Let([Assign([Span([Parens([Span([Ref(Ident("effects"))]), Span([Ref(Ident("final"))])])])], Span([Case(Span([Ref(Ident("last")), Ref(Ident("stmts"))]), [Matching([Ref(Ident("CBlockStmt")), Span([Ref(Ident("CExpr")), Ref(Ident("expr")), Ref(Ident("_"))])], [([Span([Ref(Ident("demand"))])], Span([Parens([Span([Ref(Ident("init")), Ref(Ident("stmts"))]), Span([Ref(Ident("expr"))])])]))]), Direct([Ref(Ident("_"))], [Span([Parens([Span([Ref(Ident("stmts"))]), Span([Ref(Ident("Nothing"))])])])])])]))], []);
                let effects' = cfgToRust(stat, (foldr(interpretBlockItem, (return((vec![], Unreachable))), effects)));
                let final' = mapM((interpretExpr(True)), final);
                return(Result, {
    resultType: maybe(IsVoid, resultType, final'),
    resultMutable: maybe(Rust.Immutable, resultMutable, final'),
    result: Rust.BlockExpr((Rust.Block(effects', (fmap(result, final')))))
    })
            })
    };

    let @(interpretStatement(stmt), (CBreak(_))(next)) = |()| {
        {

            let _ = next;
            let val = lift((asks(onBreak)));
            match val {
        Just, label => { return((vec![], Branch(label))) },
        Nothing => { lift(lift(badSource(stmt, "break outside loop".to_string()))) },
    }
        }
    };

    let @(interpretStatement(stmt), (CCase(expr, body, node))(next)) = |()| {
        {

            let selector = getSwitchExpression(stmt);
            Let([Assign([Span([Ref(Ident("condition"))])], Span([Ref(Ident("CBinary")), Ref(Ident("CEqOp")), Ref(Ident("selector")), Ref(Ident("expr")), Ref(Ident("node"))]))], []);
            addSwitchCase((Just(condition)), body, next)
        }
    };

    let @(interpretStatement(stmt), (CCases(lower, upper, body, node))(next)) = |()| {
        {

            let selector = getSwitchExpression(stmt);
            Let([Assign([Span([Ref(Ident("condition"))])], Span([Ref(Ident("CBinary")), Ref(Ident("CLndOp")), Parens([Span([Ref(Ident("CBinary")), Ref(Ident("CGeqOp")), Ref(Ident("selector")), Ref(Ident("lower")), Ref(Ident("node"))])]), Parens([Span([Ref(Ident("CBinary")), Ref(Ident("CLeqOp")), Ref(Ident("selector")), Ref(Ident("upper")), Ref(Ident("node"))])]), Ref(Ident("node"))]))], []);
            addSwitchCase((Just(condition)), body, next)
        }
    };

    let @(interpretStatement(stmt), (CCont(_))(next)) = |()| {
        {

            let _ = next;
            let val = lift((asks(onContinue)));
            match val {
        Just, label => { return((vec![], Branch(label))) },
        Nothing => { lift(lift(badSource(stmt, "continue outside loop".to_string()))) },
    }
        }
    };

    let @(interpretStatement(stmt), (CReturn(expr, _))(next)) = |()| {
        {

            let _ = next;
            lift(lift({

            let val = lift((asks(functionReturnType)));
            match val {
        Nothing => { badSource(stmt, "return statement outside function".to_string()) },
        Just, retTy => { {

            let expr' = mapM(((fmap((castTo(retTy))) . interpretExpr(True))), expr);
            return((exprToStatements((Rust.Return(expr'))), Unreachable))
        } },
    }
        }))
        }
    };

    let @(interpretStatement(stmt), (CSwitch(expr, body, node))(next)) = |()| {
        {

            let (bindings, expr') = match expr {
            CVar, { .. } => { return((vec![], expr)) },
            _ => { lift(lift({

                        let ident = fmap(internalIdent, (uniqueName("switch".to_string())));
                        let rhs = interpretExpr(True, expr);
                        let var = addSymbolIdent(ident, (Rust.Immutable, resultType(rhs)));
                        return((vec![Rust.Let(Rust.Immutable, (Rust.VarName(var)), Nothing, (Just((result(rhs)))))], CVar(ident, node)))
                    })) },
        };
            let after = newLabel;
            let (_, SwitchCases(cases)) = getSwitchCases(expr')(setBreak(after)(interpretStatement(body, (return((vec![], Branch(after)))))));
            Let([Assign([Span([Ref(Ident("isDefault")), Parens([Span([Ref(Ident("Just")), Ref(Ident("condition"))])])])], Span([Ref(Ident("Left")), Ref(Ident("condition"))])), Assign([Span([Ref(Ident("isDefault")), Ref(Ident("Nothing"))])], Span([Ref(Ident("Right")), Parens([])]))], []);
            Let([Assign([Span([Parens([Span([Ref(Ident("conditions"))]), Span([Ref(Ident("defaults"))])])])], Span([Ref(Ident("IntMap.mapEither")), Ref(Ident("isDefault")), Ref(Ident("cases"))]))], []);
            let defaultCase = match IntMap.keys(defaults) {
            [] => { return(after) },
            [defaultCase] => { return(defaultCase) },
            _ => { lift(lift(badSource(stmt, "duplicate default cases".to_string()))) },
        };
            let entry = foldrM(conditionBlock, defaultCase, (IntMap.toList(conditions)));
            let (rest, end) = next;
            addBlock(after, rest, end);
            return((bindings, Branch(entry)));
            
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

            let (storage, base) = baseTypeOf(spec);
            match storage {
        Just, s => { badSource(s, "storage class specifier in type name".to_string()) },
        Nothing => { return(()) },
    };
            let itype = match declarators {
            [] => { base },
            [(Just(declr, @, CDeclr(Nothing, _, _, _, _)), Nothing, Nothing)] => { derivedTypeOf(base, declr) },
            _ => { badSource(decl, "type name".to_string()) },
        };
            when((typeIsFunc(itype)), (badSource(decl, "use of function type".to_string())));
            return((typeMutable(itype), typeRep(itype)))
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

            let action = runOnce({

            let itype = deferred;
            let rewrites = lift(asks(itemRewrites));
            let path = match Map.lookup((Symbol, identToString(ident)), rewrites) {
            Just, renamed => { return((:("".to_string(), renamed))) },
            Nothing => { {

                Let([Assign([Span([Ref(Ident("name"))])], Span([Ref(Ident("applyRenames")), Ref(Ident("ident"))]))], []);
                Let([Assign([Span([Ref(Ident("ty"))])], Span([Parens([Span([Ref(Ident("typeMutable")), Ref(Ident("itype"))]), Span([Ref(Ident("typeRep")), Ref(Ident("itype"))])])]))], []);
                lift(tell(mempty, {
        outputExterns: Map.singleton(name, (mkItem(name, ty)))
        }));
                return(vec![name])
            } },
        };
            return((typeToResult(itype, (Rust.Path((Rust.PathSegments(path)))))))
        });
            addSymbolIdentAction(ident, action)
        }
    };

    let addSwitchCase(condition, body, next) = |()| {
        {

            let condition' = lift(lift(mapM((interpretExpr(True)), condition)));
            let next' = interpretStatement(body, next);
            let label = match next' {
            ([], Branch(to)) => { return(to) },
            (rest, end) => { {

                let label = newLabel;
                addBlock(label, rest, end);
                return(label)
            } },
        };
            lift(tell(SwitchCases(IntMap.singleton(label, condition'))));
            return((vec![], Branch(label)))
        }
    };

    let addSymbolIdent(ident, (mut, ty)) = |()| {
        {

            Let([Assign([Span([Ref(Ident("name"))])], Span([Ref(Ident("applyRenames")), Ref(Ident("ident"))]))], []);
            addSymbolIdentAction(ident)(return(Result, {
        resultType: ty,
        resultMutable: mut,
        result: Rust.Path((Rust.PathSegments(vec![name])))
        }));
            return(name)
        }
    };

    let addSymbolIdentAction(ident, action) = |()| {
        lift({

                modify(Lambda({
        symbolEnvironment: :((ident, action), symbolEnvironment(st))
        }))
            })
    };

    let addTagIdent(ident, ty) = |()| {
        lift({

                modify(Lambda({
        tagEnvironment: :((ident, ty), tagEnvironment(st))
        }))
            })
    };

    let addTypedefIdent(ident, ty) = |()| {
        lift({

                modify(Lambda({
        typedefEnvironment: :((ident, ty), typedefEnvironment(st))
        }))
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

            Let([Assign([Span([Parens([Span([Ref(Ident("storage"))]), Span([Ref(Ident("_attributes"))]), Span([Ref(Ident("basequals"))]), Span([Ref(Ident("basespecs"))]), Span([Ref(Ident("_inlineNoReturn"))]), Span([Ref(Ident("_align"))])])])], Span([Ref(Ident("partitionDeclSpecs")), Ref(Ident("specs"))]))], []);
            let mstorage = match storage {
            [] => { return(Nothing) },
            [spec] => { return((Just(spec))) },
            _, :, excess, :, _ => { badSource(excess, "extra storage class specifier".to_string()) },
        };
            let base = typedef((mutable(basequals)), basespecs);
            return((mstorage, base));
            
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

                                let ptrTo = match compatiblePtr((resultType(lhs')), (resultType(rhs'))) {
            IsPtr, _, ptrTo => { return(ptrTo) },
            _ => { badSource(expr, "pointer subtraction of incompatible pointers".to_string()) },
        };
                                Let([Assign([Span([Ref(Ident("ty"))])], Span([Ref(Ident("IsInt")), Ref(Ident("Signed")), Ref(Ident("WordWidth"))]))], []);
                                Let([Assign([Span([Ref(Ident("size"))])], Span([Ref(Ident("rustSizeOfType")), Parens([Span([Ref(Ident("toRustType")), Ref(Ident("ptrTo"))])])]))], []);
                                return(Result, {
    resultType: ty,
    resultMutable: Rust.Immutable,
    result: /((Rust.MethodCall((castTo(ty, lhs')), (Rust.VarName("wrapping_sub".to_string())), vec![castTo(ty, rhs')])), castTo(ty, size))
    })
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

            Let([Assign([Span([Ref(Ident("builder"))])], Span([Ref(Ident("buildCFG")), Operator("$"), Do([Expression(Span([Parens([Span([Ref(Ident("early"))]), Span([Ref(Ident("term"))])]), Operator("<-"), Ref(Ident("build"))]), []), Expression(Span([Ref(Ident("entry")), Operator("<-"), Ref(Ident("newLabel"))]), []), Expression(Span([Ref(Ident("addBlock")), Ref(Ident("entry")), Ref(Ident("early")), Ref(Ident("term"))]), []), Expression(Span([Ref(Ident("return")), Ref(Ident("entry"))]), [])], [])]))], []);
            let (rawCFG, _) = evalRWST(builder, (OuterLabels(Nothing, Nothing, Nothing)), Map.empty);
            Let([Assign([Span([Ref(Ident("cfg"))])], Span([Ref(Ident("depthFirstOrder")), Parens([Span([Ref(Ident("removeEmptyBlocks")), Ref(Ident("rawCFG"))])])]))], []);
            Let([Assign([Span([Parens([Span([Ref(Ident("hasGoto"))]), Span([Ref(Ident("structured"))])])])], Span([Ref(Ident("structureCFG")), Ref(Ident("mkBreak")), Ref(Ident("mkContinue")), Ref(Ident("mkLoop")), Ref(Ident("mkIf")), Ref(Ident("mkGoto")), Ref(Ident("mkMatch")), Ref(Ident("cfg"))]))], []);
            return(:(if(hasGoto, then, declCurrent), structured(else, structured)));
            
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

            Let([Assign([Span([Ref(Ident("op\'"))])], Span([Case(Span([Ref(Ident("op"))]), [Direct([Ref(Ident("CAssignOp"))], [Span([Ref(Ident("Nothing"))])]), Direct([Ref(Ident("CMulAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CMulOp"))])]), Direct([Ref(Ident("CDivAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CDivOp"))])]), Direct([Ref(Ident("CRmdAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CRmdOp"))])]), Direct([Ref(Ident("CAddAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CAddOp"))])]), Direct([Ref(Ident("CSubAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CSubOp"))])]), Direct([Ref(Ident("CShlAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CShlOp"))])]), Direct([Ref(Ident("CShrAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CShrOp"))])]), Direct([Ref(Ident("CAndAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CAndOp"))])]), Direct([Ref(Ident("CXorAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("CXorOp"))])]), Direct([Ref(Ident("COrAssOp"))], [Span([Ref(Ident("Just")), Ref(Ident("COrOp"))])])])]))], []);
            Let([Assign([Span([Ref(Ident("duplicateLHS"))])], Span([Ref(Ident("isJust")), Ref(Ident("op\'")), Operator("||"), Ref(Ident("demand"))]))], []);
            Let([Assign([Span([Parens([Span([Ref(Ident("bindings1"))]), Span([Ref(Ident("dereflhs"))]), Span([Ref(Ident("boundrhs"))])])])], Span([Ref(Ident("if")), Ref(Ident("not")), Ref(Ident("duplicateLHS")), Operator("||"), Ref(Ident("hasNoSideEffects")), Parens([Span([Ref(Ident("result")), Ref(Ident("lhs"))])]), Ref(Ident("then")), Parens([Span([Vector([])]), Span([Ref(Ident("lhs"))]), Span([Ref(Ident("rhs"))])]), Ref(Ident("else")), Let([Assign([Span([Ref(Ident("lhsvar"))])], Span([Ref(Ident("Rust.VarName")), Str("_lhs")])), Assign([Span([Ref(Ident("rhsvar"))])], Span([Ref(Ident("Rust.VarName")), Str("_rhs")]))], []), Ref(Ident("in")), Parens([Span([Vector([Span([Ref(Ident("Rust.Let")), Ref(Ident("Rust.Immutable")), Ref(Ident("rhsvar")), Ref(Ident("Nothing")), Parens([Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("result")), Ref(Ident("rhs"))])])])])]), Span([Ref(Ident("Rust.Let")), Ref(Ident("Rust.Immutable")), Ref(Ident("lhsvar")), Ref(Ident("Nothing")), Parens([Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("Rust.Borrow")), Ref(Ident("Rust.Mutable")), Parens([Span([Ref(Ident("result")), Ref(Ident("lhs"))])])])])])])])])]), Span([Ref(Ident("lhs")), Record([(Ident("result"), Span([Ref(Ident("Rust.Deref")), Parens([Span([Ref(Ident("Rust.Var")), Ref(Ident("lhsvar"))])])]))])]), Span([Ref(Ident("rhs")), Record([(Ident("result"), Span([Ref(Ident("Rust.Var")), Ref(Ident("rhsvar"))]))])])])]))], []);
            let rhs' = match op' {
            Just, o => { binop(expr, o, dereflhs, boundrhs) },
            Nothing => { return(boundrhs) },
        };
            Let([Assign([Span([Ref(Ident("assignment"))])], Span([Ref(Ident("Rust.Assign")), Parens([Span([Ref(Ident("result")), Ref(Ident("dereflhs"))])]), Parens([Span([Ref(Ident("Rust.:="))])]), Parens([Span([Ref(Ident("castTo")), Parens([Span([Ref(Ident("resultType")), Ref(Ident("lhs"))])]), Ref(Ident("rhs\'"))])])]))], []);
            Let([Assign([Span([Parens([Span([Ref(Ident("bindings2"))]), Span([Ref(Ident("ret"))])])])], Span([Ref(Ident("if")), Ref(Ident("not")), Ref(Ident("demand")), Ref(Ident("then")), Parens([Span([Vector([])]), Span([Ref(Ident("Nothing"))])]), Ref(Ident("else")), Ref(Ident("if")), Ref(Ident("not")), Ref(Ident("returnOld")), Ref(Ident("then")), Parens([Span([Vector([])]), Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("result")), Ref(Ident("dereflhs"))])])])]), Ref(Ident("else")), Let([Assign([Span([Ref(Ident("oldvar"))])], Span([Ref(Ident("Rust.VarName")), Str("_old")]))], []), Ref(Ident("in")), Parens([Span([Vector([Span([Ref(Ident("Rust.Let")), Ref(Ident("Rust.Immutable")), Ref(Ident("oldvar")), Ref(Ident("Nothing")), Parens([Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("result")), Ref(Ident("dereflhs"))])])])])])])]), Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("Rust.Var")), Ref(Ident("oldvar"))])])])])]))], []);
            return(match Rust.Block((++(bindings1, ++(bindings2, exprToStatements(assignment)))), ret) {
            b, @, Rust.Block(body, Nothing) => { Result({
                resultType: IsVoid,
                resultMutable: Rust.Immutable,
                result: match body {
                                [Rust.Stmt(e)] => { e },
                                _ => { Rust.BlockExpr(b) },
                            }
                }) },
            b => { lhs({
                result: Rust.BlockExpr(b)
                }) },
        });
            
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

            let rewrites = lift((asks(itemRewrites)));
            unless((Map.member((kind, identToString(ident)), rewrites)))(lift(tell(mempty, {
            outputIncomplete: Set.singleton((identToString(ident)))
            })));
            return((IsIncomplete(ident)))
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

            let mexpr = lift(asks(switchExpression));
            match mexpr {
        Nothing => { lift(lift(badSource(stmt, "case outside switch".to_string()))) },
        Just, expr => { return(expr) },
    }
        }
    };

    let getSymbolIdent(ident) = |()| {
        {

            let env = lift(get);
            match lookup(ident, (symbolEnvironment(env))) {
        Just, symbol => { fmap(Just, symbol) },
        Nothing => { match identToString(ident) {
                "__func__" => { getFunctionName("".to_string()) },
                "__FUNCTION__" => { getFunctionName("".to_string()) },
                "__PRETTY_FUNCTION__" => { getFunctionName("top level".to_string()) },
                name => { return(lookup(name, builtinSymbols)) },
            } },
    };
            
        }
    };

    let getTagIdent(ident) = |()| {
        lift({

                let env = gets(tagEnvironment);
                return(lookup(ident, env))
            })
    };

    let getTypedefIdent(ident) = |()| {
        lift({

                let env = gets(typedefEnvironment);
                return((identToString(ident), lookup(ident, env)))
            })
    };

    let gotoLabel(ident) = |()| {
        {

            let labels = lift(get);
            match Map.lookup(ident, labels) {
        Nothing => { {

            let label = newLabel;
            lift((put((Map.insert(ident, label, labels)))));
            return(label)
        } },
        Just, label => { return(label) },
    }
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

            let decl' = lift(lift((interpretDeclarations(makeLetBinding, decl))));
            let (rest, end) = next;
            return((++(decl', rest), end))
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

            let e' = interpretExpr(True, e);
            return((rustAlignOfType((toRustType((resultType(e')))))))
        }
    };

    let interpretExpr(_, (CAlignofType(decl, _))) = |()| {
        {

            let (_mut, ty) = typeName(decl);
            return((rustAlignOfType((toRustType(ty)))))
        }
    };

    let interpretExpr(_, (CCast(decl, expr, _))) = |()| {
        {

            let (_mut, ty) = typeName(decl);
            let expr' = interpretExpr((/=(ty, IsVoid)), expr);
            return(Result, {
    resultType: ty,
    resultMutable: Rust.Immutable,
    result: (==(if(ty), IsVoid(then, result, else, castTo, ty)))(expr')
    })
        }
    };

    let interpretExpr(_, (CCompoundLit(decl, initials, info))) = |()| {
        {

            let (mut, ty) = typeName(decl);
            let final = interpretInitializer(ty, (CInitList(initials, info)));
            return(Result, {
    resultType: ty,
    resultMutable: mut,
    result: final
    })
        }
    };

    let interpretExpr(_, (CSizeofExpr(e, _))) = |()| {
        {

            let e' = interpretExpr(True, e);
            return((rustSizeOfType((toRustType((resultType(e')))))))
        }
    };

    let interpretExpr(_, (CSizeofType(decl, _))) = |()| {
        {

            let (_mut, ty) = typeName(decl);
            return((rustSizeOfType((toRustType(ty)))))
        }
    };

    let interpretExpr(_, expr) = |()| {
        unimplemented(expr)
    };

    let interpretExpr(demand, (CComma(exprs, _))) = |()| {
        {

            Let([Assign([Span([Parens([Span([Ref(Ident("effects"))]), Span([Ref(Ident("mfinal"))])])])], Span([Ref(Ident("if")), Ref(Ident("demand")), Ref(Ident("then")), Parens([Span([Ref(Ident("init")), Ref(Ident("exprs"))]), Span([Ref(Ident("Just")), Parens([Span([Ref(Ident("last")), Ref(Ident("exprs"))])])])]), Ref(Ident("else")), Parens([Span([Ref(Ident("exprs"))]), Span([Ref(Ident("Nothing"))])])]))], []);
            let effects' = mapM(((fmap(resultToStatements) . interpretExpr(False))), effects);
            let mfinal' = mapM((interpretExpr(True)), mfinal);
            return(Result, {
    resultType: maybe(IsVoid, resultType, mfinal'),
    resultMutable: maybe(Rust.Immutable, resultMutable, mfinal'),
    result: Rust.BlockExpr((Rust.Block((concat(effects')), (fmap(result, mfinal')))))
    })
        }
    };

    let interpretFunction((@(CFunDef(specs, declr), (CDeclr(mident, _, _, _, _))(argtypes, body, _)))) = |()| {
        {

            let (storage, baseTy) = baseTypeOf(specs);
            let (attrs, vis) = match storage {
            Nothing => { return((vec![Rust.Attribute("no_mangle".to_string())], Rust.Public)) },
            Just, CStatic(_) => { return((vec![], Rust.Private)) },
            Just, s => { badSource(s, "storage class specifier for function".to_string()) },
        };
            Let([Assign([Span([Ref(Ident("go")), Ref(Ident("name")), Ref(Ident("funTy"))])], Span([Do([Expression(Span([Parens([Span([Ref(Ident("retTy"))]), Span([Ref(Ident("args"))])]), Operator("<-"), Case(Span([Ref(Ident("funTy"))]), [Direct([Ref(Ident("IsFunc")), Ref(Ident("_")), Ref(Ident("_")), Ref(Ident("True"))], [Span([Ref(Ident("unimplemented")), Ref(Ident("declr"))])]), Direct([Ref(Ident("IsFunc")), Ref(Ident("retTy")), Ref(Ident("args")), Ref(Ident("False"))], [Span([Ref(Ident("return")), Parens([Span([Ref(Ident("retTy"))]), Span([Ref(Ident("args"))])])])]), Direct([Ref(Ident("_"))], [Span([Ref(Ident("badSource")), Ref(Ident("declr")), Str("function definition")])])])]), []), Expression(Span([Ref(Ident("when")), Parens([Span([Ref(Ident("name")), Operator("=="), Str("_c_main")])]), Parens([Span([Ref(Ident("wrapMain")), Ref(Ident("declr")), Ref(Ident("name")), Parens([Span([Ref(Ident("map")), Ref(Ident("snd")), Ref(Ident("args"))])])])])]), []), Expression(Span([Let([Assign([Span([Ref(Ident("setRetTy")), Ref(Ident("flow"))])], Span([Ref(Ident("flow")), Record([(Ident("functionReturnType"), Span([Ref(Ident("Just")), Ref(Ident("retTy"))])), (Ident("functionName"), Span([Ref(Ident("Just")), Ref(Ident("name"))]))])]))], [])]), []), Expression(Span([Ref(Ident("f\'")), Operator("<-"), Ref(Ident("mapExceptT")), Parens([Span([Ref(Ident("local")), Ref(Ident("setRetTy"))])]), Operator("$"), Ref(Ident("scope")), Operator("$"), Do([Expression(Span([Ref(Ident("formals")), Operator("<-"), Ref(Ident("sequence")), Dummy]), []), Expression(Span([Let([Assign([Span([Ref(Ident("returnValue"))])], Span([Ref(Ident("if")), Ref(Ident("name")), Operator("=="), Str("_c_main"), Ref(Ident("then")), Ref(Ident("Just")), Number(0), Ref(Ident("else")), Ref(Ident("Nothing"))])), Assign([Span([Ref(Ident("returnStatement"))])], Span([Ref(Ident("Rust.Stmt")), Parens([Span([Ref(Ident("Rust.Return")), Ref(Ident("returnValue"))])])]))], [])]), []), Expression(Span([Ref(Ident("body\'")), Operator("<-"), Ref(Ident("cfgToRust")), Ref(Ident("declr")), Parens([Span([Ref(Ident("interpretStatement")), Ref(Ident("body")), Parens([Span([Ref(Ident("return")), Parens([Span([Vector([Span([Ref(Ident("returnStatement"))])])]), Span([Ref(Ident("Unreachable"))])])])])])])]), []), Expression(Span([Ref(Ident("return")), Parens([Span([Ref(Ident("Rust.Item")), Ref(Ident("attrs")), Ref(Ident("vis")), Parens([Span([Ref(Ident("Rust.Function")), Vector([Span([Ref(Ident("Rust.UnsafeFn"))]), Span([Ref(Ident("Rust.ExternABI")), Ref(Ident("Nothing"))])]), Ref(Ident("name")), Ref(Ident("formals")), Parens([Span([Ref(Ident("toRustRetType")), Ref(Ident("retTy"))])]), Parens([Span([Ref(Ident("statementsToBlock")), Ref(Ident("body\'"))])])])])])])]), [])], [])]), []), Expression(Span([Ref(Ident("emitItems")), Vector([Span([Ref(Ident("f\'"))])])]), [])], [])]))], []);
            let ident = match mident {
            Nothing => { badSource(declr, "anonymous function definition".to_string()) },
            Just, ident => { return(ident) },
        };
            Let([Assign([Span([Ref(Ident("name"))])], Span([Ref(Ident("applyRenames")), Ref(Ident("ident"))]))], []);
            Let([Assign([Span([Ref(Ident("funTy")), Ref(Ident("itype"))])], Span([Ref(Ident("typeToResult")), Ref(Ident("itype")), Parens([Span([Ref(Ident("Rust.Path")), Parens([Span([Ref(Ident("Rust.PathSegments")), Vector([Span([Ref(Ident("name"))])])])])])])]))], []);
            let deferred = fmap((fmap(funTy)), (derivedDeferredTypeOf(baseTy, declr, argtypes)));
            let alreadyUsed = lift(gets(((usedForwardRefs . globalState))));
            match vis {
    Rust.Private => if Set.notMember(ident, alreadyUsed) { {

        let action = runOnce({

            let ty = deferred;
            go(name, (resultType(ty)));
            return(ty)
        });
        addSymbolIdentAction(ident, action)
    } },
        _ => { {

            let ty = deferred;
            addSymbolIdentAction(ident)(return(ty));
            go(name, (resultType(ty)))
        } },
    }
        }
    };

    let interpretInitializer(ty, initial) = |()| {
        {

            let initial' = match initial {
            CInitExpr, expr, _ => { {

                let expr' = interpretExpr(True, expr);
                compatibleInitializer(if(resultType, expr'), ty(then, pure)(scalar((castTo(ty, expr')), else, badSource, initial, "initializer for incompatible type".to_string())))
            } },
            CInitList, list, _ => { translateInitList(ty, list) },
        };
            let zeroed = zeroInitialize(initial', ty);
            helper(ty, zeroed);
            
        }
    };

    let interpretStatement((CCompound(vec![], items, _)), next) = |()| {
        mapBuildCFGT((mapRWST(scope)))({

                foldr(interpretBlockItem, next, items)
            })
    };

    let interpretStatement((CDefault(body, _)), next) = |()| {
        addSwitchCase(Nothing, body, next)
    };

    let interpretStatement((CExpr((Just(expr)), _)), next) = |()| {
        {

            let expr' = lift(lift(interpretExpr(False, expr)));
            let (rest, end) = next;
            return((++(resultToStatements(expr'), rest), end))
        }
    };

    let interpretStatement((CExpr(Nothing, _)), next) = |()| {
        next
    };

    let interpretStatement((CFor(initial, mcond, mincr, body, _)), next) = |()| {
        {

            let after = newLabel;
            let ret = mapBuildCFGT((mapRWST(scope)))({

            let prefix = match initial {
            Left, Nothing => { return(vec![]) },
            Left, Just(expr) => { {

                let expr' = lift(lift(interpretExpr(False, expr)));
                return((resultToStatements(expr')))
            } },
            Right, decls => { lift(lift(interpretDeclarations(makeLetBinding, decls))) },
        };
            let headerLabel = newLabel;
            let incrLabel = match mincr {
            Nothing => { return(headerLabel) },
            Just, incr => { {

                let incr' = lift(lift(interpretExpr(False, incr)));
                let incrLabel = newLabel;
                addBlock(incrLabel, (resultToStatements(incr')), (Branch(headerLabel)));
                return(incrLabel)
            } },
        };
            let (bodyEntry, bodyTerm) = setBreak(after)(setContinue(incrLabel)(interpretStatement(body, (return((vec![], Branch(incrLabel)))))));
            let bodyLabel = newLabel;
            addBlock(bodyLabel, bodyEntry, bodyTerm);
            let cond = match mcond {
            Just, cond => { {

                let cond' = lift(lift(interpretExpr(True, cond)));
                return((CondBranch(cond', bodyLabel, after)))
            } },
            Nothing => { return((Branch(bodyLabel))) },
        };
            addBlock(headerLabel, vec![], cond);
            return((prefix, Branch(headerLabel)))
        });
            let (rest, end) = next;
            addBlock(after, rest, end);
            return(ret)
        }
    };

    let interpretStatement((CGoto(ident, _)), next) = |()| {
        {

            let _ = next;
            let label = gotoLabel(ident);
            return((vec![], Branch(label)))
        }
    };

    let interpretStatement((CIf(c, t, mf, _)), next) = |()| {
        {

            let c' = lift(lift(interpretExpr(True, c)));
            let after = newLabel;
            let falseLabel = match mf {
            Nothing => { return(after) },
            Just, f => { {

                let (falseEntry, falseTerm) = interpretStatement(f, (return((vec![], Branch(after)))));
                let falseLabel = newLabel;
                addBlock(falseLabel, falseEntry, falseTerm);
                return(falseLabel)
            } },
        };
            let (trueEntry, trueTerm) = interpretStatement(t, (return((vec![], Branch(after)))));
            let trueLabel = newLabel;
            addBlock(trueLabel, trueEntry, trueTerm);
            let (rest, end) = next;
            addBlock(after, rest, end);
            return((vec![], CondBranch(c', trueLabel, falseLabel)))
        }
    };

    let interpretStatement((CLabel(ident, body, _, _)), next) = |()| {
        {

            let label = gotoLabel(ident);
            let (rest, end) = interpretStatement(body, next);
            addBlock(label, rest, end);
            return((vec![], Branch(label)))
        }
    };

    let interpretStatement((CWhile(c, body, doWhile, _)), next) = |()| {
        {

            let c' = lift(lift(interpretExpr(True, c)));
            let after = newLabel;
            let headerLabel = newLabel;
            let (bodyEntry, bodyTerm) = setBreak(after)(setContinue(headerLabel)(interpretStatement(body, (return((vec![], Branch(headerLabel)))))));
            let bodyLabel = newLabel;
            addBlock(bodyLabel, bodyEntry, bodyTerm);
            addBlock(headerLabel, vec![])(match toBool(c') {
        Rust.Lit, Rust.LitBool(cont) => if /=(cont, doWhile) { Branch((if(cont, then, bodyLabel, else, after))) },
            _ => { CondBranch(c', bodyLabel, after) },
        });
            let (rest, end) = next;
            addBlock(after, rest, end);
            return((vec![], Branch((if(doWhile, then, bodyLabel, else, headerLabel)))))
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

                let st = get;
                Let([Assign([Span([Parens([Span([Ref(Ident("global\'"))]), Span([Ref(Ident("a"))])])])], Span([Ref(Ident("f")), Parens([Span([Ref(Ident("globalState")), Ref(Ident("st"))])])]))], []);
                put(st, {
    globalState: global'
    });
                return(a)
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

                    let (obj', initial) = match cinitial {
            CInitList, list', _ => { {

                let initial = translateInitList((designatorType(obj)), list');
                return((obj, initial))
            } },
            CInitExpr, expr, _ => { {

                let expr' = interpretExpr(True, expr);
                match nestedObject((resultType(expr')), obj) {
        Nothing => { badSource(cinitial, "type in initializer".to_string()) },
        Just, obj' => { {

            Let([Assign([Span([Ref(Ident("s"))])], Span([Ref(Ident("castTo")), Parens([Span([Ref(Ident("designatorType")), Ref(Ident("obj\'"))])]), Ref(Ident("expr\'"))]))], []);
            return((obj', scalar(s)))
        } },
    }
            } },
        };
                    Let([Assign([Span([Ref(Ident("indices"))])], Span([Ref(Ident("unfoldr")), Parens([Span([Lambda])]), Ref(Ident("obj\'"))]))], []);
                    Let([Assign([Span([Ref(Ident("initializer"))])], Span([Ref(Ident("foldl")), Parens([Span([Lambda, Ref(Ident("Nothing")), Parens([Span([Ref(Ident("IntMap.singleton")), Ref(Ident("j")), Ref(Ident("a"))])])])]), Ref(Ident("initial")), Ref(Ident("indices"))]))], []);
                    return((nextObject(obj'), mappend(prior, initializer)))
                } },
            }
    };

    fn resultToStatements() -> Vec<Rust.Stmt> {
        (exprToStatements . result)
    }

    let runOnce(action) = |()| {
        {

            let cacheRef = lift(lift(newSTRef((Left(action)))));
            return({

        let cache = lift(lift(readSTRef(cacheRef)));
        match cache {
        Left, todo => { {

            lift(lift(writeSTRef(cacheRef)(Left(fail("internal error: runOnce action depends on itself, leading to an infinite loop".to_string())))));
            let val = todo;
            lift(lift(writeSTRef(cacheRef, (Right(val)))));
            return(val)
        } },
        Right, val => { return(val) },
    }
    })
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

            let old = lift(get);
            let a = m;
            lift((modify((Lambda({
            globalState: globalState(st)
            })))));
            return(a)
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

            let objectsAndInitializers = forM(list)(Lambda);
            Let([Assign([Span([Ref(Ident("base"))])], Span([Case(Span([Ref(Ident("ty"))]), [Direct([Ref(Ident("IsArray")), Ref(Ident("_")), Ref(Ident("size")), Ref(Ident("el"))], [Span([Ref(Ident("From")), Ref(Ident("el")), Number(0), Parens([Span([Ref(Ident("replicate")), Parens([Span([Ref(Ident("size")), Operator("-"), Number(1)])]), Ref(Ident("el"))])]), Parens([Span([Ref(Ident("Base")), Ref(Ident("ty"))])])])]), Direct([Ref(Ident("IsStruct")), Ref(Ident("_")), Span([Tuple([Span([Ref(Ident("_"))]), Span([Ref(Ident("ty\'"))])]), Ref(Ident(":")), Ref(Ident("fields"))])], [Span([Ref(Ident("From")), Ref(Ident("ty\'")), Number(0), Parens([Span([Ref(Ident("map")), Ref(Ident("snd")), Ref(Ident("fields"))])]), Parens([Span([Ref(Ident("Base")), Ref(Ident("ty"))])])])]), Direct([Ref(Ident("_"))], [Span([Ref(Ident("Base")), Ref(Ident("ty"))])])])]))], []);
            let (_, initializer) = foldM(resolveCurrentObject, (Just(base), mempty), objectsAndInitializers);
            return(initializer)
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

                    let rank = integerConversionRank(aw, bw);
                    Just((IsInt(as, (==(if(rank), GT(then, aw, else, bw))))))
                } },
                _ => { Nothing },
            }
    };

    let wrapMain(declr, realName, argTypes) = |()| {
        {

            let (setup, args) = wrapArgv(argTypes);
            Let([Assign([Span([Ref(Ident("ret"))])], Span([Ref(Ident("Rust.VarName")), Str("ret")]))], []);
            emitItems(vec![Rust.Item(vec![], Rust.Private, (Rust.Function(vec![], "main".to_string(), vec![], (Rust.TypeName("()".to_string())), (statementsToBlock((++(setup, ++(vec![bind(Rust.Immutable, ret)(Rust.UnsafeExpr(Rust.Block(vec![])(Just(call(realName, args)))))], exprToStatements((call("::std::process::exit".to_string(), vec![Rust.Var(ret)])))))))))))]);
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

            modify(Lambda({
        buildBlocks: IntMap.insert(label, (BasicBlock(stmt, terminator)), (buildBlocks(st)))
        }))
        }
    };

    let buildCFG(root) = |()| {
        {

            let (label, final) = runStateT(root, (BuildState(0, IntMap.empty)));
            return((CFG(label, (buildBlocks(final)))))
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

            let old = get;
            put(old, {
    buildLabel: +(buildLabel(old), 1)
    });
            return((buildLabel(old)))
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

                let thisCrate = Map.lookup("".to_string(), crates);
                let thisModule = Map.lookup(modName, thisCrate);
                Let([Assign([Span([Ref(Ident("thisCrate\'"))])], Span([Ref(Ident("Map.delete")), Ref(Ident("modName")), Ref(Ident("thisCrate"))]))], []);
                Let([Assign([Span([Ref(Ident("crates\'"))])], Span([Ref(Ident("Map.insert")), Str(""), Ref(Ident("thisCrate\'")), Ref(Ident("crates"))]))], []);
                return((thisModule, crates'))
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
