//! Build script to compile the tree-sitter parser for bracket detection.

fn main() {
    let src_dir = std::path::Path::new("src");

    // Configure the compiler to build the parser.c file
    let mut c_config = cc::Build::new();
    c_config.include(src_dir.join("tree_sitter"));
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        // Use lower target macOS version to ensure compatibility
        .flag_if_supported("-mmacosx-version-min=11.0");

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);

    // Make sure parser.c is visible to the main compilation unit
    println!("cargo:rustc-link-search=native={}", src_dir.display());

    // The library name must match the function exported by parser.c
    // tree_sitter_bracket_parser is the function name we need to expose
    c_config.compile("tree_sitter_bracket_parser");

    // Tell Cargo to rerun the build script if any of these files change
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
    println!("cargo:rerun-if-changed=src/tree_sitter/parser.h");
    println!("cargo:rerun-if-changed=grammar.js");
}