use rnix::ast::{AttrSet, AttrpathValue, HasEntry};
use rnix::{Root, SyntaxNode, WalkEvent};
use rowan::ast::AstNode;
use std::fs;
use std::io::{Error, ErrorKind};

use crate::templates::FLAKE_FILE;

/**
 * @brief Safely adds a new input to the `flake.nix` file.
 *
 * This function parses the `flake.nix` file, locates the `inputs` attribute set,
 * checks if the specified input already exists, and if not, inserts the new input.
 *
 * @param key The key/name of the flake input to add.
 * @param url The URL of the flake input.
 * @return Result<(), Error> Returns Ok(()) on success, or an Error if the operation fails.
 */
pub fn add_flake_input(key: &str, url: &str) -> Result<(), Error> {
    let content = fs::read_to_string(FLAKE_FILE)?;
    let ast = Root::parse(&content);

    // Find the `inputs` attribute set
    let inputs_set_node = find_node(ast.syntax(), |node| {
        if let Some(attr) = AttrpathValue::cast(node.clone()) {
            // Use attrpath() instead of key()
            if attr.attrpath()?.to_string().trim() == "inputs" {
                // Unwrap Expr, then cast its syntax
                if let Some(expr) = attr.value() {
                    return AttrSet::cast(expr.syntax().clone());
                }
            }
        }
        None
    })
    .ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            "Could not find `inputs` set in flake.nix",
        )
    })?;

    // Check if the input already exists
    if inputs_set_node.entries().any(|entry| match entry {
        rnix::ast::Entry::AttrpathValue(attr) => attr
            .attrpath()
            .map_or(false, |p| p.to_string().trim() == key),
        _ => false,
    }) {
        println!(
            "Input '{}' already exists in {}. Skipping.",
            key, FLAKE_FILE
        );
        return Ok(());
    }

    // Find closing brace '}' of the inputs set
    let closing_brace = inputs_set_node
        .syntax()
        .children_with_tokens()
        .filter_map(|el| el.into_token())
        .find(|token| token.text() == "}")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not find closing brace"))?;

    // Position to insert the new input
    let insert_pos: usize = closing_brace.text_range().start().into();
    let new_input_text = format!("\n    {}.url = \"{}\";\n  ", key, url);

    let mut new_content = content;
    new_content.insert_str(insert_pos, &new_input_text);
    fs::write(FLAKE_FILE, new_content)?;

    Ok(())
}

/**
 * @brief Helper function to find the first matching AST node.
 *
 * This function traverses the syntax tree in preorder and applies the provided closure
 * to each node, returning the first non-None result.
 *
 * @tparam T The type returned by the closure.
 * @tparam F The closure type, which takes a SyntaxNode and returns `Option<T>`.
 * @param node The root SyntaxNode to start traversal from.
 * @param f The closure to apply to each node.
 * @return `Option<T>` The first non-None result from the closure, or None if not found.
 */
fn find_node<T, F>(node: SyntaxNode, mut f: F) -> Option<T>
where
    F: FnMut(SyntaxNode) -> Option<T>,
{
    node.preorder()
        .filter_map(|event| match event {
            WalkEvent::Enter(node) => f(node),
            _ => None,
        })
        .next()
}
