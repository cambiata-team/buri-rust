use crate::{
    generic_nodes::{
        get_generic_type_id, GenericDocument, GenericSourcedType, GenericVariableDeclaration,
    },
    parsed_expression_to_generic_expression::translate_parsed_expression_to_generic_expression,
    type_schema::TypeSchema,
    type_schema_substitutions::TypeSchemaSubstitutions,
};
use ast::{DocumentNode, ParsedNode, TopLevelDeclaration, VariableDeclarationValue};
use typed_ast::TypedVariableDeclaration;

fn translate_variable_declaration<'a>(
    input: TopLevelDeclaration<ParsedNode<'a, VariableDeclarationValue<'a>>>,
) -> Result<TopLevelDeclaration<GenericVariableDeclaration<'a>>, ()> {
    let identifier_name = input.declaration.value.identifier.value.name;
    let mut schema = TypeSchema::new();
    let mut substitutions = TypeSchemaSubstitutions::new();
    let expression = translate_parsed_expression_to_generic_expression(
        &mut schema,
        &mut substitutions,
        input.declaration.value.expression,
    )?;
    let type_id = get_generic_type_id(&expression);
    Ok(TopLevelDeclaration {
        declaration: GenericVariableDeclaration {
            declaration: TypedVariableDeclaration {
                declaration_type: GenericSourcedType {
                    type_id,
                    source_of_type: input.declaration.source,
                },
                identifier_name,
                expression,
            },
            schema,
            substitutions,
        },
        is_exported: input.is_exported,
    })
}

pub fn apply_constraints(input: DocumentNode) -> Result<GenericDocument, ()> {
    let mut variable_declarations: Vec<TopLevelDeclaration<GenericVariableDeclaration>> =
        Vec::new();
    variable_declarations.reserve_exact(input.value.variable_declarations.len());
    for variable_declaration in input.value.variable_declarations {
        variable_declarations.push(translate_variable_declaration(variable_declaration)?);
    }
    Ok(GenericDocument {
        imports: input.value.imports,
        // TODO(aaron) add type declarations to return value
        type_declarations: vec![],
        variable_declarations: variable_declarations,
        // TODO(aaron) add top level expressions to return value
        expressions: vec![],
    })
}
