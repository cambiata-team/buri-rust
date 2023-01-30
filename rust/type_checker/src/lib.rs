mod apply_constraints;
mod constraints;
mod generic_nodes;
mod parsed_expression_to_generic_expression;
mod type_schema;
mod type_schema_substitutions;

type GenericTypeId = usize;

pub use apply_constraints::apply_constraints;

use generic_nodes::GenericDocument;
use typed_ast::ConcreteDocument;

#[allow(clippy::needless_pass_by_value)]
// TODO(aaron) clarify error type, and move function definition to separate file.
pub fn resolve_concrete_types(input: GenericDocument) -> Result<ConcreteDocument, ()> {
    unimplemented!();
}
