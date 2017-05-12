module test_module (

) where

-- | silly one
data NodeInfo = Int  -- pos, last token and unique name
           deriving (Data,Typeable)


-- | C identifiers
data Ident = String       -- lexeme
                   {-# UNPACK #-}   !Int     -- hash to speed up equality check
                   NodeInfo                   -- attributes of this ident. incl. position
             deriving (Data,Typeable)


-- | string of an identifier
identToString               :: Ident -> String
identToString (Ident s _ _)  = s


applyRenames :: Ident -> String
applyRenames ident = case identToString ident of
    "final" -> "final_"
    "fn" -> "fn_"
    "in" -> "in_"
    "let" -> "let_"
    "main" -> "_c_main"
    "match" -> "match_"
    "mod" -> "mod_"
    "proc" -> "proc_"
    "type" -> "type_"
    "where" -> "where_"
    name -> name
