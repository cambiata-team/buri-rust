use expression::{mangle_variable_name, print_declaration};
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
        result.push_str(&print_declaration(&declaration.declaration));
    }
    result
}
