#!/bin/sh

set -e

# When a given file is fully ported, you can remove it from this list for recompilation.

COROLLARY="../corollary/target/debug/corollary"

cargo build --bin corollary --manifest-path ../corollary/Cargo.toml

$COROLLARY ../deps/language-c/src/Language/C.hs -o src/lib.rs

$COROLLARY ../deps/language-c/src/Language/C/Analysis.hs -o src/analysis/mod.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/AstAnalysis.hs -o src/analysis/ast_analysis.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/Builtins.hs -o src/analysis/builtins.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/ConstEval.hs -o src/analysis/const_eval.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/Debug.hs -o src/analysis/debug.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/DeclAnalysis.hs -o src/analysis/decl_analysis.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/DefTable.hs -o src/analysis/def_table.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/Export.hs -o src/analysis/export.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/NameSpaceMap.hs -o src/analysis/name_space_map.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/SemError.hs -o src/analysis/sem_error.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/SemRep.hs -o src/analysis/sem_rep.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/TravMonad.hs -o src/analysis/trav_monad.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/TypeCheck.hs -o src/analysis/type_check.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/TypeConversions.hs -o src/analysis/type_conversions.rs
$COROLLARY ../deps/language-c/src/Language/C/Analysis/TypeUtils.hs -o src/analysis/type_utils.rs

$COROLLARY ../deps/language-c/src/Language/C/Data.hs -o src/data/mod.rs
$COROLLARY ../deps/language-c/src/Language/C/Data/Error.hs -o src/data/error.rs
$COROLLARY ../deps/language-c/src/Language/C/Data/Ident.hs -o src/data/ident.rs
$COROLLARY ../deps/language-c/src/Language/C/Data/InputStream.hs -o src/data/input_stream.rs
$COROLLARY ../deps/language-c/src/Language/C/Data/Name.hs -o src/data/name.rs
$COROLLARY ../deps/language-c/src/Language/C/Data/Node.hs -o src/data/node.rs
$COROLLARY ../deps/language-c/src/Language/C/Data/Position.hs -o src/data/position.rs
$COROLLARY ../deps/language-c/src/Language/C/Data/RList.hs -o src/data/r_list.rs

$COROLLARY ../deps/language-c/src/Language/C/Parser.hs -o src/parser/mod.rs
$COROLLARY ../deps/language-c/src/Language/C/Parser/Builtin.hs -o src/parser/builtin.rs
$COROLLARY ../deps/language-c/src/Language/C/Parser/ParserMonad.hs -o src/parser/parser_monad.rs
$COROLLARY ../deps/language-c/src/Language/C/Parser/Tokens.hs -o src/parser/tokens.rs

# Generated files
$COROLLARY ./gen/Parser.hs -o src/parser/parser.rs
$COROLLARY ./gen/Lexer.hs -o src/parser/lexer.rs

$COROLLARY ../deps/language-c/src/Language/C/Syntax.hs -o src/syntax/mod.rs
$COROLLARY ../deps/language-c/src/Language/C/Syntax/AST.hs -o src/syntax/ast.rs
$COROLLARY ../deps/language-c/src/Language/C/Syntax/Constants.hs -o src/syntax/constants.rs
$COROLLARY ../deps/language-c/src/Language/C/Syntax/Ops.hs -o src/syntax/ops.rs
$COROLLARY ../deps/language-c/src/Language/C/Syntax/Utils.hs -o src/syntax/utils.rs

$COROLLARY ../deps/language-c/src/Language/C/System/GCC.hs -o src/syntax/gcc.rs
$COROLLARY ../deps/language-c/src/Language/C/System/Preprocess.hs -o src/syntax/preprocess.rs
