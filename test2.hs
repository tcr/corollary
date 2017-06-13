module Test()
where

parseCFile :: bool
parseCFile cpp = do
    input_stream <- if not (isPreprocessed input_file)
                        then  let cpp_args = a
                              in runPreprocessor
                        else  readInputStream

interpretExpr :: Bool -> CExpr -> EnvMonad s Result
interpretExpr _ expr@(CConst c) = case c of
    CIntConst (CInteger v repr flags) _ ->
        let allow_signed = not (testFlag FlagUnsigned flags)
            allow_unsigned = not allow_signed || repr /= DecRepr
            widths =
                [ (32 :: Int,
                    if any (`testFlag` flags) [FlagLongLong, FlagLong]
                    then WordWidth else BitWidth 32)
                , (64, BitWidth 64)
                ]
        in case allowed_types of
        [] -> badSource expr "integer (too big)"
        ty : _ -> return (literalNumber ty (Rust.LitInt v repr'))
    CFloatConst (CFloat str) _ -> case span (`notElem` "fF") str of
        _ -> badSource expr "float"


relooper :: Monoid s => IntSet.IntSet -> IntMap.IntMap (StructureBlock s c) -> [Structure s c]
relooper entries blocks =
    let (returns, noreturns) = partitionMembers entries $ IntSet.unions $ map successors $ IntMap.elems blocks
        (present, absent) = partitionMembers entries (IntMap.keysSet blocks)
    in case (IntSet.toList noreturns, IntSet.toList returns) of
    ([], []) -> []

    where
    strictReachableFrom = flipEdges (go (IntMap.map successors blocks))
        where
        grow r = IntMap.map (\ seen -> IntSet.unions $ seen : IntMap.elems (r `restrictKeys` seen)) r
        go r = let r' = grow r in if r /= r' then go r' else r'


mapSubStmts :: (CStat -> Bool) -> (CStat -> CStat) -> CStat -> CStat
-- mapSubStmts stop _ s | stop s = s
mapSubStmts stop f (CLabel i s attrs ni) =
  f (CLabel i (mapSubStmts stop f s) attrs ni)
mapSubStmts stop f (CCase e s ni) =
  f (CCase e (mapSubStmts stop f s) ni)
mapSubStmts stop f (CCases e1 e2 s ni) =
  f (CCases e1 e2 (mapSubStmts stop f s) ni)
mapSubStmts stop f (CDefault s ni) =
  f (CDefault (mapSubStmts stop f s) ni)
mapSubStmts stop f (CCompound ls body ni) =
  f (CCompound ls (map (mapBlockItemStmts stop f) body) ni)
mapSubStmts stop f (CIf e sthen selse ni) =
  f (CIf e
     (mapSubStmts stop f sthen)
     (fmap (mapSubStmts stop f) selse)
     ni)
mapSubStmts stop f (CSwitch e s ni) =
  f (CSwitch e (mapSubStmts stop f s) ni)
mapSubStmts stop f (CWhile e s isdo ni) =
  f (CWhile e (mapSubStmts stop f s) isdo ni)
mapSubStmts stop f (CFor i t a s ni) =
  f (CFor i t a (mapSubStmts stop f s) ni)
mapSubStmts _ f s  = f s