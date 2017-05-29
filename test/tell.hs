module Test()
where

emitItems :: [Rust.Item] -> EnvMonad s ()
emitItems items = lift $ tell mempty { outputItems = items }
