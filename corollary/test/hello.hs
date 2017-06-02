{-# LANGUAGE ViewPatterns #-}

module Hello ()
where

import Test.HUnit.Base (assertEqual)

helloWorld :: String
helloWorld = "hello, world!"

main :: IO ()
main = do
    assertEqual "" "hello, world!" (helloWorld)
    putStrLn "success."
