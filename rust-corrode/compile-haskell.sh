#!/bin/sh

# When a given file is fully ported, you can remove it from this list for recompilation.

COROLLARY="../corollary/target/debug/corollary"

cargo build --bin corollary --manifest-path ../corollary/Cargo.toml

$COROLLARY ../deps/corrode/src/Language/Rust.hs -o src/mod.rs
$COROLLARY ../deps/corrode/src/Language/Rust/AST.hs -o src/ast.rs
$COROLLARY ../deps/corrode/src/Language/Rust/Idiomatic.hs -o src/idiomatic.rs

$COROLLARY ../deps/corrode/src/Language/Rust/Corrode/C.lhs -o src/corrode/c.rs
$COROLLARY ../deps/corrode/src/Language/Rust/Corrode/CFG.lhs -o src/corrode/cfg.rs
$COROLLARY ../deps/corrode/src/Language/Rust/Corrode/CrateMap.hs -o src/corrode/crate_map.rs
