module Test()
where

data FunctionContext = FunctionContext
    { functionReturnType :: Maybe CType
    , functionName :: Maybe String
    , itemRewrites :: ItemRewrites
    }