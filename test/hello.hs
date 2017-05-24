{-# LANGUAGE ViewPatterns #-}

module Language.Rust.Idiomatic (
    itemIdioms
) where

import qualified Language.Rust.AST as Rust

parseCrateMap :: String -> Either String CrateMap
parseCrateMap = fmap root . foldrM parseLine (Map.empty, []) . filter (not . null) . map cleanLine . lines

sumEuler :: Int -> Int
sumEuler = sum . (map euler) . mkList
