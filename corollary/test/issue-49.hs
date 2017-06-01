module Test()
where

interpretStatement :: String
interpretStatement next = do
    let isDefault (Just condition) = Left condition
        isDefault Nothing = Right ()