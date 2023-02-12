use crate::{
    generic_nodes::{GenericDeclarationExpression, GenericDocument},
    parsed_expression_to_generic_expression::translate_declaration,
    type_schema::TypeSchema,
};
use ast::{DeclarationValue, DocumentNode, ParsedNode, TopLevelDeclaration};

fn translate_top_level_variable_declaration<'a>(
    schema: &mut TypeSchema,
    input: TopLevelDeclaration<ParsedNode<'a, DeclarationValue<'a>>>,
) -> Result<TopLevelDeclaration<GenericDeclarationExpression<'a>>, ()> {
    Ok(TopLevelDeclaration {
        declaration: translate_declaration(schema, input.declaration)?,
        is_exported: input.is_exported,
    })
}

pub fn apply_constraints(input: DocumentNode) -> Result<(GenericDocument, TypeSchema), ()> {
    let mut schema = TypeSchema::new();
    let mut variable_declarations: Vec<TopLevelDeclaration<GenericDeclarationExpression>> =
        Vec::new();
    variable_declarations.reserve_exact(input.value.variable_declarations.len());
    for variable_declaration in input.value.variable_declarations {
        variable_declarations.push(translate_top_level_variable_declaration(
            &mut schema,
            variable_declaration,
        )?);
    }
    Ok((
        GenericDocument {
            imports: input.value.imports,
            // TODO(aaron) add type declarations to return value
            type_declarations: vec![],
            variable_declarations,
            // TODO(aaron) add top level expressions to return value
            expressions: vec![],
        },
        schema,
    ))
}
