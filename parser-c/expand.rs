// cargo-deps: command-macros="0.1" error-chain="*" regex="*" walkdir="*"

//! Description

#[macro_use] extern crate command_macros;
#[macro_use] extern crate error_chain;
extern crate walkdir;

use walkdir::Walkdir;

// Error chain
mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    // let stat = cmd!( ls ).status().unwrap();
    // println!("ls {:?}", stat);

    let re = Regex::new(r"a").unwrap();

    for item in Walkdir::new("./src") {
        println!("{:?}", item);
    }

    Ok(())
}