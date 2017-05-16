extern crate lalrpop;

fn main() {
    println!("cargo:rerun-if-changed=src/calculator.lalrpop");
    lalrpop::process_root().unwrap();
}
