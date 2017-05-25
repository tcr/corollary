module Test.Hello ()
where

truthy :: Bool
truthy = true

falsy :: Bool
falsy = false

{-RUST
fn main() {
    assert_eq!(true, Test_Hello::truthy());
    assert_eq!(false, Test_Hello::falsy());
    println!("success.");
}
/RUST-}
