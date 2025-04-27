use crate::definitions::*;
use tree_sitter::*;

/*
 * @brief       Retrieves the tree of provided source code.
 * 
 * @param[in]   source_code     The source code to retrieve nodes from.
 * @param[in]   language        The language of the source code.
 *    
 */
pub fn get_syntax_tree(source_code: &str, language: SupportedLanguageE) -> Result<Tree, String> {
    let mut parser = Parser::new();
    let parser_language = language.to_parser_language();

    parser.set_language(&parser_language)
        .expect("Failed to load parser language object.");

    let tree = parser.parse(source_code, None)
        .ok_or("Failed to parse source code.")?;

    Ok(tree)
}