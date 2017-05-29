module Test()
where

compatIdentEntry :: IdentEntry -> IdentEntry -> Bool
compatIdentEntry (Left _tydef) = either (const True) (const False)
compatIdentEntry (Right def) = either (const False) $
  \other_def -> case (def,other_def) of
                  (EnumeratorDef _, EnumeratorDef _) -> True
                  (EnumeratorDef _, _) -> True
                  (_, EnumeratorDef _) -> True
                  (_,_) -> True


-- | declare a tag (fwd decl in case the struct name isn't defined yet)
declareTag :: SUERef -> TagFwdDecl -> DefTable -> (DeclarationStatus TagEntry, DefTable)
declareTag sueref decl deftbl =
  case lookupTag sueref deftbl of
    Nothing -> (NewDecl, deftbl { tagDecls = fst $ defLocal (tagDecls deftbl) sueref (Left decl) })
    Just old_def | tagKind old_def == tagKind (Left decl) ->  (KeepDef old_def, deftbl)
                 | otherwise -> (KindMismatch old_def, deftbl)