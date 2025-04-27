/*
 *****************
 * MODULE DETAILS
 *****************
 * 
 * Purpose: Retrieve key nodes that the parser needs for parsing!
 */

use tree_sitter::Node;

/*
 * @brief       Retrieves all nodes of the provided kind.
 * 
 * @param[in]   root            The root node.
 * @param[in]   requestedKind   The kind of node to retrieve all instances of.
 * 
 * @returns     A vector containing all instances of the specified node type.
 */
pub fn retrieve_nodes_by_kind(root: Node, requestedKind: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if child.kind() == requestedKind {
            nodes.push(child);
        }
    }

    nodes;
}

/*
 * @brief       Retrieves the functions definitions within the source code.
 * 
 * @param[in]   root        The root node.        
 * 
 * @returns     A vector containing all function definition nodes.
 */
pub fn retrieve_function_definitions(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "function_definition")
}

/* 
 * @brief       Retrieves the variables within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all variable nodes.
 */
pub fn retrieve_variables(root: Node) {
    retrieve_nodes_by_kind(root, "declaration")
}

/* 
 * @brief       Retrieves the if statements within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all if statement nodes.
 */
pub fn retrieve_if_statements(root: Node) {
    retrieve_nodes_by_kind(root, "if_statement")
}

/*
 * @brief       Retrieves the while statements within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all while loop nodes.
 */
pub fn retrieve_while_statements(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "while_statement")
}

/*
 * @brief       Retrieves the for statements within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all for loop nodes.
 */
pub fn retrieve_for_statements(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "for_statement")
}

/*
 * @brief       Retrieves the assignment expressions within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all assignment expression nodes.
 */
pub fn retrieve_assignment_expressions(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "assignment_expression")
}

/*
 * @brief       Retrieves the return statements within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all return statement nodes.
 */
pub fn retrieve_return_statements(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "return_statement")
}

/*
 * @brief       Retrieves the binary expressions within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all binary expression nodes.
 */
pub fn retrieve_binary_expressions(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "binary_expression")
}

/*
 * @brief       Retrieves the unary expressions within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all unary expression nodes.
 */
pub fn retrieve_unary_expressions(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "unary_expression")
}

/*
 * @brief       Retrieves the function call expressions within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all call expression nodes.
 */
pub fn retrieve_function_calls(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "call_expression")
}

/*
 * @brief       Retrieves the struct definitions within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all struct specifier nodes.
 */
pub fn retrieve_struct_definitions(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "struct_specifier")
}

/*
 * @brief       Retrieves the enum definitions within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all enum specifier nodes.
 */
pub fn retrieve_enum_definitions(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "enum_specifier")
}

/*
 * @brief       Retrieves all comments within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all comment nodes.
 */
pub fn retrieve_comments(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "comment")
}

/*
 * @brief       Retrieves preprocessor defines within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all preprocessor definition nodes.
 */
pub fn retrieve_preprocessor_defines(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "preproc_def")
}

/*
 * @brief       Retrieves preprocessor includes within the source code.
 * 
 * @param[in]   root        The root node.
 * 
 * @returns     A vector containing all preprocessor include nodes.
 */
pub fn retrieve_preprocessor_includes(root: Node) -> Vec<Node> {
    retrieve_nodes_by_kind(root, "preproc_include")
}