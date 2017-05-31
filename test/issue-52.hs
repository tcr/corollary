module Test()
where

mergeCrateMaps :: [(String, CrateMap)] -> Map.Map String CrateMap
mergeCrateMaps = Map.fromListWith (Map.unionWith (++))
