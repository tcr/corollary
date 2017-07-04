# Porting Corrode to Rust

The goal of this project is to partially automate the translation of [Corrode, the C to Rust translator written in Haskell](https://github.com/jameysharp/corrode), and Language-C, the Haskell library and C AST it depends on, into Rust. It aims to broaden development to a larger and more relevant audience, and bolster the ecosystem around a critical piece of Rust's tooling: its C to Rust conversion tool.

To accomplish this goal, this project has developed a Haskell to Rust compiler, Corollary, than can convert most of these libraries into Rust, letting contributors manually finish the conversion into idiomatic Rust code. Much like the usecase of Corrode itself!

This project has non-functional, but nearly complete, ports of [Language-C](https://github.com/tcr/corrode-but-in-rust/tree/master/parser-c) and [Corrode](https://github.com/tcr/corrode-but-in-rust/tree/master/rust-corrode) into Rust. Read on for how we can get to 100% working crates.

```
git clone http://github.com/tcr/corrode-but-in-rust --recursive
```

These are the crates contained in this repo:

* **`parser-haskell/`**, an original Haskell Parser written in LALRPOP.
* **`corollary/`**, an experimental Haskell to Rust compiler. This is used to generate (part of) the `parser-c` and `rust-corrode` crates.
* **`corollary-support/`**, a support crate for converted Haskell code to use.
* **`parser-c/`**, a largely automatically cross-compiled version of the `language-c` Haskell module for parsing C code.
* **`rust-corrode/`**, a largely automatically cross-compiled Rust port of the Corrode program for converting C into Rust code.

Sound wild? Here is the plan:

- This project contains a proof-of-concept cross-compiler from Haskell to Rust (Corollary) which is not designed to be either correct or generalizable; instead, it's tailored to port these two libraries.
- The crates `rust-corrode/` and `rust-language-c/` both contain a `compile-haskell.sh` script which semi cross-compiles its code using the above crate. As files are ported over, they can be finalized (fully ported) and removed from `compile-haskell.sh` until no files remain.
- If a problem can be fixed in the conversion instead of manually, edit **corollary** to add more support for the Haskell into Rust conversion. [Follow along in this tracking issue!](https://github.com/tcr/corrode-but-in-rust/issues/1)

One big caveat: Parser.x and Lexer.y, two files for lexing and parsing used by Language-C, are written using Haskell tools and in Haskell. Because their converted files are very regularly generated, we for now just opt to have these conversions done as a separate build pipeline for the Language-c port (`parser-c/`). A unique requirement for Corollary: modifications to these Haskell files should convert to Rust without issues, unlike other files (which can be manually edited to cross the cap to correctness.)

### Using Corollary

Corrode can be used as an independent binary (and is used in our build pipelines here). For example:

```
cargo run --bin corrode -- input/file/path.hs -o target/output.rs
```

This converts a file from Haskell into Rust. You can omit the `-o` parameter to write the file to stdout.

Additionally, you can run a file using the `--run` parameter, which will strip any code in a `{-HASKELL-} ... {-/HASKELL-}` block and run any code in a `{-RUST ... /RUST-}` block embedded in a file. This is how tests for Corollary are written; to run more than one file you'll want to convert a Haskell source tree into a Cargo crate.

### Patching or Modifying Corollary

In general, most changes to Corollary should maintain the broadest compatibility possible, with the exception of avoid regressions in able to translate the precompiled `Parser.x` and `Lexer.y` files. New tests for new functionality and not breaking old tests is the easiest way to do this.

## References

* [Ten Things You Should Know About Haskell Syntax](https://www.fpcomplete.com/blog/2012/09/ten-things-you-should-know-about-haskell-syntax)
* [Haskell: The Confusing Parts](http://echo.rsmw.net/n00bfaq.html)

## License

Corollary is licensed as MIT or Apache-2, at your option.

Language-C licensed as BSD-3.

Corrode licensed as GPLv2.
