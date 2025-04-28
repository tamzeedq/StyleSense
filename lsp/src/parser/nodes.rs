///
/// MODULE DETAILS
/// 
/// # Purpose
/// 
/// * Retrieve key nodes from source code.
/// * Retrieve parsing-critical info on said key nodes.

use tree_sitter::Node;

/// Retrieves all nodes of the provided kind.
/// 
/// # Arguments
/// 
/// * 'root' -              The root node. 
/// * 'requested_kind' -    The kind of node to retrieve all instances of.      
/// 
/// # Returns
/// 
/// A vector containing all instances of the specified kind.
pub fn retrieve_nodes_by_kind<'a>(root: Node<'a>, requested_kind: &str) -> Vec<Node<'a>> {
    let mut nodes = Vec::new();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if child.kind() == requested_kind {
            nodes.push(child);
        }
    }

    nodes
}

/// Retrieves the function definitions within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all function definition nodes.
pub fn retrieve_function_definitions<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "function_definition")
}

/// Retrieves the variables within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all variable nodes.
pub fn retrieve_variables<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "declaration")
}

/// Retrieves the if statements within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all if statement nodes.
pub fn retrieve_if_statements<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "if_statement")
}

/// Retrieves the while statements within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all while loop nodes.
pub fn retrieve_while_statements<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "while_statement")
}

/// Retrieves the for statements within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all for loop nodes.
pub fn retrieve_for_statements<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "for_statement")
}

/// Retrieves the assignment expressions within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all assignment expression nodes.
pub fn retrieve_assignment_expressions<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "assignment_expression")
}

/// Retrieves the return statements within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all return statement nodes.
pub fn retrieve_return_statements<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "return_statement")
}

/// Retrieves the binary expressions within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all binary expression nodes.
pub fn retrieve_binary_expressions<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "binary_expression")
}

/// Retrieves the unary expressions within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all unary expression nodes.
pub fn retrieve_unary_expressions<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "unary_expression")
}

/// Retrieves the function call expressions within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all call expression nodes.
pub fn retrieve_function_calls<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "call_expression")
}

/// Retrieves the struct definitions within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all struct specifier nodes.
pub fn retrieve_struct_definitions<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "struct_specifier")
}

/// Retrieves the enum definitions within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all enum specifier nodes.
pub fn retrieve_enum_definitions<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "enum_specifier")
}

/// Retrieves all comments within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all comment nodes.
pub fn retrieve_comments<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "comment")
}

/// Retrieves preprocessor defines within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all preprocessor definition nodes.
pub fn retrieve_preprocessor_defines<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "preproc_def")
}

/// Retrieves preprocessor includes within the source code.
///
/// # Arguments
///
/// * `root` - The root node.
///
/// # Returns
///
/// A vector containing all preprocessor include nodes.
pub fn retrieve_preprocessor_includes<'a>(root: Node<'a>) -> Vec<Node<'a>> {
    retrieve_nodes_by_kind(root, "preproc_include")
}
