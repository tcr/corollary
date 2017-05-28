module Test()
where

modifyOptions :: (TravOptions -> TravOptions) -> Trav s ()
modifyOptions f = modify $ \ts -> ts { options = f (options ts) }
