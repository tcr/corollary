module Test()
where
    
parseCrateMap :: String -> Either String CrateMap
parseCrateMap = fmap root . foldrM parseLine (Map.empty, []) . filter (not . null) . map cleanLine . lines
