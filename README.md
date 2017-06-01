# Porting Corrode to Rust

It'd be cool if there existed a port of [Corrode, the C to Rust translator written in Haskell](https://github.com/jameysharp/corrode), into Rust. Corrode depends on `language-c`, an extensive C parser written in Haskell, as well. This project has the goal of translating both libraries into idiomatic Rust using a mix of automated translation and manual cleanup. (Much like Corrode itself!)

```
git clone http://github.com/tcr/corrode-but-in-rust --recursive
```

These are the crates contained in this repo:

* **`parser-haskell/`**, an original Haskell Parser written in LALRPOP.
* **`corollary/`**, an experimental Haskell to Rust compiler. This is used to generate (part of) the `parser-c` and `rust-corrode` crates.
* **`parser-c/`**, a largely automatically cross-compiled version of the `language-c` Haskell module for parsing C code.
* **`rust-corrode/`**, an experimental Rust port of the Corrode program for converting C into Rust code.

Sound nutty? Here is the plan:

This project contains a proof-of-concept cross-compiler from Haskell to Rust which is not designed to be either correct or generalizable. Instead, it's tailored for these two libraries.

**The current result is in the `rust-corrode/` and `rust-language-c/` folders.**. These are crates generated using automated source conversion, generated using these commands:

You'll note that the converted source code doesn't compile yet; fixes to the compiler are very appreciated! **[Or follow along in the tracking issue.](https://github.com/tcr/corrode-but-in-rust/issues/1)**

## References

* [Ten Things You Should Know About Haskell Syntax](https://www.fpcomplete.com/blog/2012/09/ten-things-you-should-know-about-haskell-syntax)
* [Haskell: The Confusing Parts](http://echo.rsmw.net/n00bfaq.html)

## License

This project licensed as MIT or Apache-2, at your option.

Language-C licensed as BSD-3.

Corrode licensed as GPLv2.
