module Test()
where

import Data.Maybe
import Test.HUnit.Base (assertEqual)

data FunctionContext = FunctionContext
    { functionReturnType :: Maybe String
    , functionName :: Maybe String
    , itemRewrites :: String
    }


main :: IO ()
main = do
    let a = FunctionContext {
            functionReturnType = Just "apples",
            functionName = Nothing,
            itemRewrites = "hello, world!"
        }
    assertEqual "" "hello, world!" (itemRewrites a)
    putStrLn "success."
