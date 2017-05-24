{-# LANGUAGE ViewPatterns #-}

module Language.Rust.Idiomatic (
    itemIdioms
) where

import qualified Language.Rust.AST as Rust

promote
    :: (Pretty node, Pos node)
    => node
    -> (Rust.Expr -> Rust.Expr -> Rust.Expr)
    -> Result -> Result -> EnvMonad s Result
promote node op a b = case usual (resultType a) (resultType b) of
    Just rt -> return Result
        { resultType = rt
        , resultMutable = Rust.Immutable
        , result = op (castTo rt a) (castTo rt b)
        }
    Nothing -> badSource node $ concat
        [ "arithmetic combination for "
        , show (resultType a)
        , " and "
        , show (resultType b)
        ]
