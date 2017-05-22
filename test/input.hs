module Test.Code ()
where

bitWidth :: Int -> IntWidth -> Int
bitWidth wordWidth WordWidth = wordWidth
bitWidth _ (BitWidth w) = w

blockToStatements :: Rust.Block -> [Rust.Stmt]
blockToStatements (Rust.Block stmts mexpr) = case mexpr of
    Just expr -> stmts ++ exprToStatements expr
    Nothing -> stmts

addExternIdent
    :: Ident
    -> EnvMonad s IntermediateType
    -> (String -> (Rust.Mutable, CType) -> Rust.ExternItem)
    -> EnvMonad s ()
addExternIdent ident deferred mkItem = do
    action <- runOnce $ do
        itype <- deferred
        rewrites <- lift $ asks itemRewrites
        path <- case Map.lookup (Symbol, identToString ident) rewrites of
            Just renamed -> return ("" : renamed)
            Nothing -> return [name]
    addSymbolIdentAction ident action

addSymbolIdent :: Ident -> (Rust.Mutable, CType) -> EnvMonad s String
addSymbolIdent ident (mut, ty) = do
    let name = applyRenames ident
    addSymbolIdentAction ident $ return Result
        { resultType = ty
        , resultMutable = mut
        , result = Rust.Path (Rust.PathSegments [name])
        }
    return name
