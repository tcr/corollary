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


import Control.Monad
import Prelude hiding (reverse)
import Data.Either (rights)
import Data.Foldable (foldrM)
import Data.List hiding (reverse)
import qualified Data.Map as Map
import Data.Maybe


data StmtCtx = FunCtx VarDecl
             | LoopCtx
             | SwitchCtx


data ExprSide = LValue | RValue
                deriving (Eq, Show)

-- * analysis

-- | Analyse the given AST
--
-- @analyseAST ast@ results in global declaration dictionaries.
-- If you want to perform specific actions on declarations or definitions, you may provide
-- callbacks in the @MonadTrav@ @m@.
--
-- Returns the set of global declarations and definitions which where successfully translated.
-- It is the users responsibility to check whether any hard errors occurred (@runTrav@ does this for you).
analyseAST :: (MonadTrav m) => CTranslUnit -> m GlobalDecls
analyseAST (CTranslUnit decls _file_node) = do
    -- analyse all declarations, but recover from errors
    mapRecoverM_ analyseExt decls
    -- check we are in global scope afterwards
    getDefTable >>= \dt -> when (not (inFileScope dt)) $
        error "Internal Error: Not in filescope after analysis"
    -- get the global definition table (XXX: remove ?)
    liftM globalDefs getDefTable
    where
    mapRecoverM_ f = mapM_ (handleTravError . f)

-- | Analyse an top-level declaration
analyseExt :: (MonadTrav m) => CExtDecl -> m ()
analyseExt (CAsmExt asm _)
    = handleAsmBlock asm
analyseExt (CFDefExt fundef)
    = analyseFunDef fundef
analyseExt (CDeclExt decl)
    = analyseDecl False decl


data FunctionAttribute
    = UnsafeFn
    | ExternABI (Maybe String)
    | ArrayExpr [Expr]
    | ShiftL Expr Expr
    deriving Show
