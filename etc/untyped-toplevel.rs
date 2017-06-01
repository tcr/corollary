// cargo-deps: command-macros="0.1.10" error-chain="*" regex="*"

//! Dumps out which top-level fns are not typed.

#[macro_use] extern crate command_macros;
#[macro_use] extern crate error_chain;
extern crate regex;

use std::fs;
use std::io::prelude::*;
use regex::Regex;

// Error chain
mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Regex(::regex::Error);
        }
    }
}
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    // let stat = cmd!(
    //     ls
    // ).output().unwrap();
    // println!("ls {:?}", stat.stdout);

    let mut contents = String::new();
    // TODO change this to a different file from command line
    fs::File::open("../gen/Parser.hs")?.read_to_string(&mut contents)?;

    let re = Regex::new(r"(?m)^([a-zA-Z_]+)\s+(.*?)=")?;
    for item in re.captures_iter(&contents) {
        let keyword = item[1].to_string();
        match keyword.as_str() {
            "newtype" |
            "type" |
            "instance" |
            "data" => continue,
            _ => { },
        }

        let re_keyword = Regex::new(&format!(r"(?m)^{}\s+(.*?)::", keyword))?;
        if !re_keyword.is_match(&contents) {
            println!("item {:?}", &item[1]);
        }
    }

    // println!("ok {:?}", contents);

    Ok(())
}