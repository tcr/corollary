module Test.Hello ()
where

data Term
  = Hello
  | World

printer :: Term -> String
printer Hello = "Hello"
printer World = "World"

helloworld :: String
helloworld = ((printer Hello) ++ " " ++ (printer World))

{-HASKELL-}
main = putStrLn helloworld

{-RUST

fn main() {
    assert_eq!("Hello World", Test_Hello::helloworld());
}

-}
