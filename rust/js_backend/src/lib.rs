use expression::print_expression;
use imports::print_imports;
use typed_ast::{ConcreteType, TypedDocument};

mod expression;
mod identifier;
mod imports;
mod literals;

#[must_use]
pub fn print_js_document(document: &TypedDocument<ConcreteType>) -> String {
    let mut result = String::from("import '@packages/std/prelude/index.js'\n");
    result.push_str(&print_imports(&document.imports));
    for declaration in &document.variable_declarations {
        result.push('\n');
        if declaration.is_exported {
            result.push_str("export ");
        }
        result.push_str("const ");
        // TODO(B-218): Clean this up to use the print_declaration method instead
        // manually rewriting that code here.
        result.push_str(&declaration.declaration.identifier_name);
        result.push('=');
        result.push_str(&print_expression(&declaration.declaration.expression));
    }
    result
}
