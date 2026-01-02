fn main() {
    // The tree-sitter-c and tree-sitter-cpp crates should handle C library compilation
    println!("cargo:rerun-if-changed=build.rs");
}
