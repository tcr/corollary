    pub enum Term {
        Hello,
        World
    }
    pub use self::Term::*;

mod haskell_support {
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
}


pub mod Test_Hello {
    use haskell_support::*;
    pub fn helloworld() -> String {
        (__op_addadd((printer(Hello)), __op_addadd(" ".to_string(), (printer(World)))))
    }

    pub fn printer(__0: Term) -> String {
        match (__0) {
            Hello => {
                "Hello".to_string()
            },
            World => {
                "World".to_string()
            },
        }
    }

}




/* RUST ... /RUST */

fn main() {
    assert_eq!("Hello World", Test_Hello::helloworld());
    println!("success.");
}

