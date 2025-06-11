//! Build script to compile the Tree-sitter grammar for bracket parsing

use std::path::PathBuf;

fn main() {
    // Notify cargo to rerun the build script if the grammar file changes
    println!("cargo:rerun-if-changed=grammar.js");
    println!("cargo:rerun-if-changed=src/grammar.json");

    // Find the folder that contains grammar.js
    let grammar_dir = PathBuf::from(".");

    // Set up the build configuration for the Tree-sitter parser
    let mut c_config = cc::Build::new();
    c_config.include(&grammar_dir);
    c_config.include(grammar_dir.join("src"));

    // Add the parser.c file
    let parser_path = grammar_dir.join("src/parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.display());

    // Enable C99 standard and optimize for speed
    c_config.opt_level(2);
    c_config.std("c99");

    // Compile the parser as a static library
    c_config.compile("tree_sitter_bracket_parser");

    // Print a success message
    println!("cargo:warning=Successfully compiled the Tree-sitter grammar for bracket parsing");
}
