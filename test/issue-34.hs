module Test()
where

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
            Nothing -> do
                let name = applyRenames ident
                let ty = (typeMutable itype, typeRep itype)
                lift $ tell mempty { outputExterns = Map.singleton name (mkItem name ty) }
                return [name]
        return (typeToResult itype (Rust.Path (Rust.PathSegments path)))
    addSymbolIdentAction ident action
