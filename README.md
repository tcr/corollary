# cross-compiling Corrode to Rust

It'd be cool if Corrode were in Rust, but its C parsing library was originally
written in Haskell so it is to.

Let's automatically convert that C parsing library to Rust and then tidy it up,
just like Corrode does with C. Then we can port Corrode to Rust. That's not
ridiculous! Right!?

**Status:** Parses a lot of the files. No AST generated yet, no working code
either.

Great Haskell reference: http://echo.rsmw.net/n00bfaq.html

Language lib: https://github.com/acowley/language-c/blob/master/src/Language/C/Syntax/Utils.hs
