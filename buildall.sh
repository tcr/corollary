#!/bin/bash

# TODO move this into a real build step or something

# source code in `src-corrode/` and `src-language-c/` w/ additional pregenerated files in `gen/`
cargo run --bin "corollary" -- "./src-corrode/src" -R -o ./rust-corrode
cargo run --bin "corollary" -- "./src-language-c/src" -R -o ./rust-language-c \
  --alias ./src-language-c/src/Language/C/Parser/Lexer.hs=./gen/Lexer.hs \
  --alias ./src-language-c/src/Language/C/Parser/Parser.hs=./gen/Parser.hs