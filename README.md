# cross-compiling Corrode to Rust

It'd be cool if Corrode were in Rust, but its C parsing library was originally
written in Haskell so it is to.

Let's automatically convert that C parsing library to Rust and then tidy it up,
just like Corrode does with C. Then we can port Corrode to Rust. That's not
ridiculous! Right???

```
git clone http://github.com/tcr/corrode-but-in-rust --recursive
```

## Status

See the current cross-compiled files in the out/ directory. Equivalent to:

```
cargo run "./corrode/src/Language" > out/corrode.rs
cargo run "./language-c/src/Language/C" > out/language_c.rs
cargo run "./test" > out/test.rs
```

## References

Great Haskell reference: http://echo.rsmw.net/n00bfaq.html

Language lib: https://github.com/acowley/language-c/blob/master/src/Language/C/Syntax/Utils.hs
