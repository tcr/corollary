# porting Corrode to Rust

It'd be cool if there existed a port of [Corrode, the C to Rust translator written in Haskell](https://github.com/jameysharp/corrode), into Rust. A large blocker is the lack of an extensive C parsing library like Haskell has. This project has the goal of translating both libraries into idiomatic Rust using a mix of automated translation and manual cleanup. (Much like Corrode itself!)

```
git clone http://github.com/tcr/corrode-but-in-rust --recursive
```

This project contains a proof-of-concept cross-compiler from Haskell to Rust which is not designed to be either correct or generalizable. Instead, it's tailored for these two libraries.

**The current result is in the `rust-corrode/` and `rust-language-c/` folders.**. These are crates generated using automated source conversion, generated using these commands:

```
# source code in `src-corrode/` and `src-language-c/` w/ additional pregenerated files in `gen/`
cargo run --bin "corollary" -- "./src-corrode/src/" -R -o ./rust-corrode/src
cargo run --bin "corollary" -- "./src-language-c/src/" -R -o ./rust-language-c/src \
  --alias ./src-language-c/src/Language/C/Parser/Lexer.hs=./gen/Lexer.hs \
  --alias ./src-language-c/src/Language/C/Parser/Parser.hs=./gen/Parser.hs
```

You'll note that the converted source code doesn't compile yet; fixes to the compiler are very appreciated! **[Or follow along in the tracking issue.](https://github.com/tcr/corrode-but-in-rust/issues/1)**

## References

* [Ten Things You Should Know About Haskell Syntax](https://www.fpcomplete.com/blog/2012/09/ten-things-you-should-know-about-haskell-syntax)
* [Haskell: The Confusing Parts](http://echo.rsmw.net/n00bfaq.html)

## License

This project licensed as MIT or Apache-2, at your option.

Language-C licensed as BSD-3.

Corrode licensed as GPLv2.
