module Test()
where

data FunctionContext = FunctionContext
    { functionReturnType :: Maybe CType
    , functionName :: Maybe String
    , itemRewrites :: ItemRewrites
    }

castTo :: String
castTo target (Result { resultType = IsArray mut _ el, result = source }) =
    castTo target Result
        { resultType = IsPtr mut el
        , resultMutable = Rust.Immutable
        , result = Rust.MethodCall source (Rust.VarName method) []
        }