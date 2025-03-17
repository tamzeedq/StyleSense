fn main() {
    println!("cargo:rustc-link-lib=static=tree-sitter-c");
    println!("cargo:rustc-link-lib=static=tree-sitter-cpp");
}
