module Test()
where


binop :: CExpr -> CBinaryOp -> Result -> Result -> EnvMonad s Result
binop expr op lhs rhs = fmap wrapping $ case op of
    CLndOp -> return Result { resultType = IsBool, resultMutable = Rust.Immutable, result = Rust.LAnd (toBool lhs) (toBool rhs) }