# Corollary: Haskell to Rust conversion

**WIP: [parser-c (language-c port)](https://github.com/tcr/parser-c) works with simple C programs!**

Corollary is a very experimental Haskell to Rust compiler. The goal is to automate the syntatic conversion of Haskell into Rust, letting users manually finish the conversion into idiomatic Rust code. Along with an (extremely loose) adaptation of Haskell methods in `corollary-support`, this can expediate the process of completing a full port.

**Current status:** Source code translation can parse and translate entire files. Source code specific hacks, along with manual translation, were used for the [language-c](http://github.com/tcr/parser-c) port of Haskell's C parsing library.

Given this project was purpose-built for porting a single library, you'll find source-specific hacks throughout the codebase, though they should ultimately be removed. There are no solutions yet for converting Haskell's module and import system, top-level functions without explicit type declarations, monadic classes, tail recursion, true laziness, or currying (lacking a better way to involve Haskell's type analysis). Want to help? Suggest your ideas in the issue tracker!

## Usage

Corollary can be used as a binary:

```
cargo install corollary
corollary input/file/path.hs -o target/output.rs
```

Thsi converts a single source file from Haskell into Rust. You can omit the `-o` parameter to write the file to stdout. Additionally, you can run a file using the `--run` parameter.

Corollary will strip any code in a `{-HASKELL-} ... {-/HASKELL-}` block and include any code in a `{-RUST ... /RUST-}` block embedded in a file. (See `corollary/test` for examples.) This allows you to `--run` a Haskell file directly, given it is self-contained (does not rely on Haskell's module system).

## Development

Clone this repository including its test dependencies:

```
git clone http://github.com/tcr/corollary --recursive
```

These are the crates contained in this repo:

* **`parser-haskell/`**, an original Haskell Parser written in LALRPOP.
* **`corollary/`**, an experimental Haskell to Rust compiler. This is used to generate (part of) the `parser-c` and `rust-corrode` crates.
* **`corollary-support/`**, a support crate for converted Haskell code to use.

In addition, libraries to test Corollary against exist in the `deps/` directory.

## References

* [Ten Things You Should Know About Haskell Syntax](https://www.fpcomplete.com/blog/2012/09/ten-things-you-should-know-about-haskell-syntax)
* [Haskell: The Confusing Parts](http://echo.rsmw.net/n00bfaq.html)

## License

Corollary and `parser-haskell` are licensed as MIT or Apache-2, at your option.
