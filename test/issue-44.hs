module Test.Hello ()
where

run :: Bool -> Bool -> Bool
run x y = x || y


{-HASKELL-}
main = putStrLn $ show $ run False True
{-/HASKELL-}

{-RUST
fn main() {
    assert_eq!(false, Test_Hello::run(false, false));
    assert_eq!(true, Test_Hello::run(true, false));
    assert_eq!(true, Test_Hello::run(false, true));
    assert_eq!(true, Test_Hello::run(true, true));
    println!("success.");
}
/RUST-}
