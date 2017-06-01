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
