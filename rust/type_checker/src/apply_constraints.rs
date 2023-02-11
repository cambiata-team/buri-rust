use crate::{
    generic_nodes::{
        get_generic_type_id, GenericDocument, GenericIdentifierExpression, GenericSourcedType,
        GenericTopLevelDeclarationExpression,
    },
    parsed_expression_to_generic_expression::translate_parsed_expression_to_generic_expression,
    type_schema::TypeSchema,
    type_schema_substitutions::TypeSchemaSubstitutions,
};
use ast::{DeclarationValue, DocumentNode, ParsedNode, TopLevelDeclaration};
use typed_ast::TypedDeclarationExpression;

fn translate_top_level_variable_declaration<'a>(
    input: TopLevelDeclaration<ParsedNode<'a, DeclarationValue<'a>>>,
) -> Result<TopLevelDeclaration<GenericTopLevelDeclarationExpression<'a>>, ()> {
    let identifier_name = input.declaration.value.identifier.value.name;
    let mut schema = TypeSchema::new();
    let mut substitutions = TypeSchemaSubstitutions::new();
    let expression = translate_parsed_expression_to_generic_expression(
        &mut schema,
        &mut substitutions,
        *input.declaration.value.expression,
    )?;
    let type_id = get_generic_type_id(&expression);
    Ok(TopLevelDeclaration {
        declaration: GenericTopLevelDeclarationExpression {
            declaration: TypedDeclarationExpression {
                declaration_type: GenericSourcedType {
                    type_id,
                    source_of_type: input.declaration.source.clone(),
                },
                expression_type: GenericSourcedType {
                    type_id,
                    source_of_type: input.declaration.source,
                },
                identifier: GenericIdentifierExpression {
                    expression_type: GenericSourcedType {
                        type_id,
                        source_of_type: input.declaration.value.identifier.source,
                    },
                    is_disregarded: input.declaration.value.identifier.value.is_disregarded,
                    name: identifier_name,
                },
                value: expression,
            },
            schema,
            substitutions,
        },
        is_exported: input.is_exported,
    })
}

pub fn apply_constraints(input: DocumentNode) -> Result<GenericDocument, ()> {
    let mut variable_declarations: Vec<TopLevelDeclaration<GenericTopLevelDeclarationExpression>> =
        Vec::new();
    variable_declarations.reserve_exact(input.value.variable_declarations.len());
    for variable_declaration in input.value.variable_declarations {
        variable_declarations.push(translate_top_level_variable_declaration(
            variable_declaration,
        )?);
    }
    Ok(GenericDocument {
        imports: input.value.imports,
        // TODO(aaron) add type declarations to return value
        type_declarations: vec![],
        variable_declarations,
        // TODO(aaron) add top level expressions to return value
        expressions: vec![],
    })
}
