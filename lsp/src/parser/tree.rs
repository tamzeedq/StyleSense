use tree_sitter::*;
use crate::definitions::types::*;

/// Retrieves the tree of provided source code.
/// 
/// # Arguments
/// 
/// * 'source_code'     The source code to retrieve a tree from. 
/// * 'language'        The language that the source code is in.
/// 
/// # Returns
/// 
/// An AST (Abstract Syntax Tree) of the provided source code.
pub fn get_syntax_tree(source_code: &str, language: SupportedLanguageE) -> Result<Tree, String> {
    let mut parser = Parser::new();
    let parser_language = language.to_parser_language();

    parser.set_language(&parser_language)
        .expect("Failed to load parser language object.");

    let tree = parser.parse(source_code, None)
        .ok_or("Failed to parse source code.")?;

    Ok(tree)
}