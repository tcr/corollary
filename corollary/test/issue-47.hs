module Test.Hello ()
where

truthy :: Bool
truthy = true

falsy :: Bool
falsy = false

{-RUST
fn main() {
    assert_eq!(true, truthy());
    assert_eq!(false, falsy());
    println!("success.");
}
/RUST-}
