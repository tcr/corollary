#![allow(non_snake_case)]

pub trait OpAddable {
    fn add(self, right: Self) -> Self;
}

pub fn __op_addadd<A: OpAddable>(left: A, right: A) -> A {
    OpAddable::add(left, right)
}

impl OpAddable for String {
    fn add(mut self, right: Self) -> Self {
        self.push_str(&right);
        self
    }
}

impl<A> OpAddable for Vec<A> {
    fn add(mut self, right: Self) -> Self {
        self.extend(right);
        self
    }
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

pub mod List {
    pub fn reverse<A>(mut input: Vec<A>) -> Vec<A> {
        input.reverse();
        input
    }
}

#[macro_export]
macro_rules! __map {
    ($fn: expr, $target: expr) => {
        $target.into_iter()
            .map($fn)
            .collect::<Vec<_>>()
    }
}

pub fn __op_index<F, T: ::std::ops::Index<F>>(a: T, pos: F) -> (<T as std::ops::Index<F>>::Output)
where <T as std::ops::Index<F>>::Output: std::marker::Sized + Clone {
    a[pos].clone()
}