#![allow(non_snake_case)]

pub trait Addable {
    fn add(self, right: Self) -> Self;
}

impl Addable for String {
    fn add(self, right: Self) -> Self {
        format!("{}{}", self, right)
    }
}

pub fn __op_addadd<A: Addable>(left: A, right: A) -> A {
    Addable::add(left, right)
}

pub struct IO<A: Sized>(A);

pub fn assertEqual<A: Eq + Sized>(desc: String, left: A, right: A) -> IO<()> {
    if left != right {
        panic!("{}", desc);
    }
    IO(())
}

pub fn putStrLn(line: String) -> IO<()> {
    println!("{}", line);
    IO(())
}