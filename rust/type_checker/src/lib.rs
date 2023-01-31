mod apply_constraints;
mod constraints;
mod generic_nodes;
mod parsed_expression_to_generic_expression;
mod resolve_concrete_types;
mod type_schema;
mod type_schema_substitutions;

type GenericTypeId = usize;

pub use apply_constraints::apply_constraints;
pub use resolve_concrete_types::resolve_concrete_types;
