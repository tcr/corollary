// cargo-deps: regex="*"

extern crate regex;

use regex::Regex;
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut f = fs::File::open("Rust/Corrode/C.lhs").unwrap();
    let mut f2 = fs::File::create("Rust/Corrode/C.hs").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let re = Regex::new(r"```haskell([\s\S]*?)```").unwrap();
    let mut out = vec![];
    for cap in re.captures_iter(&s) {
        out.push(cap[1].to_string());
    }

    f2.write_all(out.join("\n\n").as_bytes());
}
