module Test()
where
    
compatibleInitializer :: CType -> CType -> Bool
compatibleInitializer (IsStruct name1 _) (IsStruct name2 _) = name1 == name2
compatibleInitializer IsStruct{} _ = False
compatibleInitializer _ IsStruct{} = False
compatibleInitializer _ _ = True
