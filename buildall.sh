#!/bin/bash

# TODO move this into a real build step or something
cargo run --bin "corollary" -- "./src-corrode/src/" -R -o ./rust-corrode/src
cargo run --bin "corollary" -- "./src-language-c/src/" -R -o ./rust-language-c/src \
  --alias ./src-language-c/src/Language/C/Parser/Lexer.hs=./gen/Lexer.hs \
  --alias ./src-language-c/src/Language/C/Parser/Parser.hs=./gen/Parser.hs
