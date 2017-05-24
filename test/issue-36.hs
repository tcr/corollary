module Test.Hello ()
where

run :: Int
run y = do
  truthy <- \x -> (x * x)
  truthy(y)

{-RUST
fn main() {
    assert_eq!(4, Test_Hello::run(2));
    assert_eq!(16, Test_Hello::run(4));
    assert_ne!(15, Test_Hello::run(4));
    println!("success.");
}
/RUST-}
