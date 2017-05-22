{-# LANGUAGE ScopedTypeVariables, PatternGuards #-}
-----------------------------------------------------------------------------
-- |
-- Module      :  Language.C.Parser.Translation
-- Copyright   :  (c) 2008 Benedikt Huber
-- License     :  BSD-style
-- Maintainer  :  benedikt.huber@gmail.com
-- Stability   :  alpha
-- Portability :  ghc
--
-- Analyse the parse tree
--
-- Traverses the AST, analyses declarations and invokes handlers.
-----------------------------------------------------------------------------
module Language.C.Analysis.AstAnalysis (
    -- * Top-level analysis
    analyseAST,
    analyseExt,analyseFunDef,analyseDecl,
    -- * Building blocks for additional analyses
    analyseFunctionBody,
    defineParams,
    -- * Type checking
    tExpr, ExprSide(..),
    tStmt, StmtCtx(..),
    tDesignator,
    defaultMD
)
where
import Language.C.Analysis.SemError
import Language.C.Analysis.SemRep
import Language.C.Analysis.TravMonad
import Language.C.Analysis.ConstEval
import Language.C.Analysis.Debug
import Language.C.Analysis.DefTable (DefTable, globalDefs, defineScopedIdent,
                                     defineLabel, inFileScope, lookupTag,
                                     lookupLabel, insertType, lookupType)
import Language.C.Analysis.DeclAnalysis
import Language.C.Analysis.TypeUtils
import Language.C.Analysis.TypeCheck
import Language.C.Analysis.TypeConversions

import Language.C.Data
import Language.C.Pretty
import Language.C.Syntax.AST
import Language.C.Syntax.Constants
import Language.C.Syntax.Ops
import Language.C.Syntax.Utils
import Text.PrettyPrint.HughesPJ

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
