#[cfg(test)]
mod tests {
    use lsp::parser::nodes::retrieve_while_statements;
    use lsp::parser::{nodes, tree::get_syntax_tree};
    use lsp::definitions::types::SupportedLanguageE::LangC;

    fn setup_tree_and_bytes() -> (tree_sitter::Tree, Vec<u8>) {
        let source_code = r#"
            #include <stdio.h>
            #define FOO 123

            // Enum defining colours.
            enum colour {
                RED,
                GREEN,
                BLUE
            };

            struct MyStruct { int x; };

            int main() {
                int a = 5;
                if (a > 3) {
                    while (a--) {
                        printf("while loop\n");
                    }
                    for (int i = 0; i < 8; i++) {
                        printf("for loop\n");
                    }
                }
                return 0;
            }
        "#;

        let tree = match get_syntax_tree(source_code, LangC) {
            Ok(tree) => tree,
            Err(e) => panic!("Error parsing source code: {}", e),
        };

        let bytes = source_code.as_bytes().to_vec(); // ensure 'static lifetime

        (tree, bytes)
    }

    #[test]
    fn test_func_def_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_function_definitions(root).len(), 1);
    }

    #[test]
    fn test_if_statement_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_if_statements(root).len(), 1);
    }

    #[test]
    fn test_while_statement_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_while_statements(root).len(), 1);
    }

    #[test]
    fn test_variable_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_variables(root).len(), 2);
    }

    #[test]
    fn test_include_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_preprocessor_includes(root).len(), 1);
    }

    #[test]
    fn test_define_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_preprocessor_defines(root).len(), 1);
    }

    #[test]
    fn test_struct_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_struct_definitions(root).len(), 1);
    }

    #[test]
    fn test_for_statement_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_for_statements(root).len(), 1);
    }

    #[test]
    fn test_return_statements_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_return_statements(root).len(), 1);
    } 

    #[test]
    fn test_enum_statements_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_enum_definitions(root).len(), 1);
    }

    #[test]
    fn test_comments_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_comments(root).len(), 1);
    }

    #[test]
    fn test_function_call_count() {
        let (tree, _) = setup_tree_and_bytes();
        let root = tree.root_node();
        assert_eq!(nodes::retrieve_function_calls(root).len(), 2);
    }

    #[test]
    fn test_space_between_condition_and_body() {
        let (tree, source_code_bytes) = setup_tree_and_bytes();
        let root = tree.root_node();
        let while_vector = retrieve_while_statements(root);
        assert_eq!(nodes::get_space_between_condition_and_body(while_vector[0], &source_code_bytes), Some(1));
    }
}
