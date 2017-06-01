module Hello()
where

import Test.HUnit.Base (assertEqual)

truthy :: Bool
truthy = True

falsy :: Bool
falsy = False

{-HASKELL-}

main :: IO ()
main = do
    assertEqual "" True truthy
    assertEqual "" False falsy
    putStrLn "success."

{-/HASKELL-}

{-RUST
fn main() {
    assert_eq!(true, truthy());
    assert_eq!(false, falsy());
    println!("success.");
}
/RUST-}
