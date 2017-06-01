{-# LANGUAGE ViewPatterns #-}

module Hello ()
where

import Test.HUnit.Base (assertEqual)

helloWorld :: String
helloWorld = "hello, world!"

{-HASKELL-}

main :: IO ()
main = do
    assertEqual "" "hello, world!" helloWorld
    putStrLn "success."

{-/HASKELL-}

{-RUST
fn main() {
    assert_eq!("hello, world!", helloWorld());
    println!("success.");
}
/RUST-}
