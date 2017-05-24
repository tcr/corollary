{-# LANGUAGE ViewPatterns #-}

module Language.Rust.Idiomatic (
    itemIdioms
) where

import qualified Language.Rust.AST as Rust

mergeCrateMaps :: [(String, CrateMap)] -> Map.Map String CrateMap
mergeCrateMaps = Map.fromListWith (Map.unionWith (++))
