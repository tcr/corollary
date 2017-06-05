// cargo-deps: command-macros="0.1" error-chain="*" regex="*" walkdir="*"

//! Description

#[macro_use] extern crate command_macros;
#[macro_use] extern crate error_chain;
extern crate walkdir;
extern crate regex;

use walkdir::WalkDir;
use regex::{Regex, Captures};
use std::io::prelude::*;
use std::fs::File;

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

    let re = Regex::new(r"(pub struct (\S+?)\s*\()(.*)\)").unwrap();

    for item in WalkDir::new("./src") {
        if let Ok(item) = item {
            let p = item.path().display().to_string();
            if p.ends_with(".rs") {
                let mut c = String::new();
                { File::open(&p)?.read_to_string(&mut c); }

                let c_out = re.replace_all(&c, |cap: &Captures| {
                    let l = cap[1].to_string();
                    let r = cap[3].to_string();
                    format!("{}{})", l,
                        if r.len() == 0 || r.find("pub ").is_some() {
                            "".to_string()
                        } else {
                            format!("pub {}", r.split_whitespace().collect::<Vec<_>>().join(" pub "))
                        })
                });

                File::create(&p)?.write_all(c_out.as_bytes())?;
            }
        }
    }

    Ok(())
}