use tree_sitter::{Parser, Language};

// Load Tree-sitter C/C++ grammars
unsafe extern "C" {
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
}

pub fn parse_code(source_code: &str, language: &str) {
    let mut parser = Parser::new();

    let language = match language {
        "c" => unsafe { tree_sitter_c() },
        "cpp" => unsafe { tree_sitter_cpp() },
        _ => panic!("Unsupported language"),
    };

    parser.set_language(&language).expect("Failed to load parser");

    let tree = parser.parse(source_code, None).expect("Failed to parse code");
    println!("{:#?}", tree.root_node());
}
