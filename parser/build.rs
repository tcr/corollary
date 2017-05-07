extern crate lalrpop;

use std::fs;

fn main() {
    match (fs::metadata("./src/calculator.lalrpop"), fs::metadata("./src/calculator.rs")) {
        (Ok(a), Ok(b)) => {
            if a.modified().unwrap() < b.modified().unwrap() {
                return;
            }
        }
        _ => {}
    }

    lalrpop::process_root().unwrap();
}
