# porting Corrode to Rust

It'd be cool if there existed a port of [Corrode, the C to Rust translator written in Haskell](https://github.com/jameysharp/corrode), into Rust. A large blocker is the lack of an extensive C parsing library like Haskell has. This project has the goal of translating both libraries into idiomatic Rust using a mix of automated translation and manual cleanup. (Much like Corrode itself!)

```
git clone http://github.com/tcr/corrode-but-in-rust --recursive
```

This project contains a proof-of-concept cross-compiler from Haskell to Rust which is not designed to be either correct or generalizable. Instead, it's tailored for these libraries, each written in a conventional programming style. I expect the conversion to go like this:

* [x] Automate bulk cross-compilation as modules (see the `out/` directory for current status)
* [x] Write a proof-of-concept parser for Haskell and compilation to Rust
* [x] Support functions, variables, literals, types
* [x] Support case statements and guards
* [ ] Convert instances and data structures correctly
* [ ] Detect pointfree code and convert it into pointwise
* [ ] Properly convert $ and . operators
* [ ] Convert rest of operators into Rust equivalents or fn wrappers
* [ ] Successfully parse all files (except lexer.x parser.y) (failures currently output as // ERROR)
* [ ] Find a way to parse flex-based lexer.x and parser.y files and cross-compile them
* [ ] Switch to *manual* conversion for all remaining edge cases (&str and String conversion, undoing tricky code segments, etc.)
* [ ] Pass language-c test bench
* [ ] Pass corrode test bench
* [ ] Port literate Haskell comments into Rust
* [ ] Feature-complete

## Status

See the current status by looking at the cross-compiled files in the out/ directory. These are equivalent to:

```
cargo run "./corrode/src/Language" > out/corrode.rs
cargo run "./language-c/src/Language/C" > out/language_c.rs
cargo run "./test" > out/test.rs
```

Look at the `test/input.hs` file and `out/test.rs` for an example of compilation you can (almost!) execute.

## References

* Great Haskell language reference: http://echo.rsmw.net/n00bfaq.html

## License

This project licensed as MIT or Apache-2, at your option.

Language-C licensed as BSD-3.

Corrode licensed as GPLv2.
