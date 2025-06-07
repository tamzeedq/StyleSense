use tree_sitter::{Node, TreeCursor};

/// Retrieves the number of ASCII space characters between the closing
/// parenthesis of a `while` statement and the opening `{` of its body.
///
/// # Arguments
///
/// * `while_node` - The `while_statement` node to inspect.
/// * `source_code` - The full source code as a byte slice.
///
/// # Returns
///
/// `Some(count)` with the number of spaces, or `None` if structure is unexpected.
pub fn get_space_between_condition_and_body<'a>(node: Node<'a>, source_code: &'a [u8],) -> Option<usize> {
    let mut cursor:TreeCursor = node.walk();
    let mut condition_end:Option<usize> = None;
    let mut body_start:Option<usize> = None;

    /* Retrieve the closing parenthesis and opening curly brace. */
    for child in node.children(&mut cursor) {
        if child.kind() == "parenthesized_expression" {
            condition_end = Some(child.end_byte());
        } else if child.kind() == "compound_statement" {
            body_start = Some(child.start_byte());
        }
    }

    /* Count the spaces between the closing parenthesis and opening curly brace. */
    match (condition_end, body_start) {
        (Some(end), Some(start)) if end < start => {

            /* Grab a slice of bytes respresenting the. */
            let gap = &source_code[end..start];

            /* Count the spaces within the gap. */
            let space_count = gap
                .iter()
                .filter(|byte_ref| **byte_ref == b' ')
                .count();

            Some(space_count)
        }
        _ => None,
    }
}

/// Recursively retrieves all nodes of the provided kind.
/// 
/// # Arguments
/// 
/// * 'node' -              The provided starting node. 
/// * 'requested_kind' -    The kind of node to retrieve all instances of.      
/// 
/// # Returns
/// 
/// A vector containing all instances of the specified kind.
pub fn retrieve_nodes_by_kind<'a>(node: Node<'a>, requested_kind: &str) -> Vec<Node<'a>> {
    let mut requested_nodes:Vec<Node<'a>> = Vec::new();
    let mut cursor:TreeCursor<'a> = node.walk();

    /* Push the current node if it is of the requested kind. */
    if node.kind() == requested_kind {
        requested_nodes.push(node);
    }

    /* Recursively check each child node for its child nodes. */
    for child in node.children(&mut cursor) {
        requested_nodes.extend(retrieve_nodes_by_kind(child, requested_kind) );
    }

    requested_nodes
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