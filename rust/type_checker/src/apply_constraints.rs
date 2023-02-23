use crate::{
    generic_nodes::{
        GenericDeclarationExpression, GenericDocument, GenericTypeDeclarationExpression,
    },
    parsed_expression_to_generic_expression::{translate_declaration, translate_type_declaration},
    type_schema::TypeSchema,
};
use ast::{DeclarationValue, DocumentNode, ParsedNode, TopLevelDeclaration, TypeDeclarationNode};

fn translate_top_level_variable_declaration<'a>(
    schema: &mut TypeSchema,
    input: TopLevelDeclaration<ParsedNode<'a, DeclarationValue<'a>>>,
) -> Result<TopLevelDeclaration<GenericDeclarationExpression<'a>>, String> {
    Ok(TopLevelDeclaration {
        declaration: translate_declaration(schema, input.declaration)?,
        is_exported: input.is_exported,
    })
}

fn translate_top_level_type_declaration<'a>(
    schema: &mut TypeSchema,
    input: TopLevelDeclaration<TypeDeclarationNode<'a>>,
) -> Result<TopLevelDeclaration<GenericTypeDeclarationExpression<'a>>, String> {
    Ok(TopLevelDeclaration {
        declaration: translate_type_declaration(schema, input.declaration)?,
        is_exported: input.is_exported,
    })
}

pub fn apply_constraints(input: DocumentNode) -> Result<(GenericDocument, TypeSchema), String> {
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
    let mut type_declarations: Vec<TopLevelDeclaration<GenericTypeDeclarationExpression>> =
        Vec::new();
    type_declarations.reserve_exact(input.value.type_declarations.len());
    for type_declaration in input.value.type_declarations {
        type_declarations.push(translate_top_level_type_declaration(
            &mut schema,
            type_declaration,
        )?);
    }
    Ok((
        GenericDocument {
            imports: input.value.imports,
            type_declarations,
            variable_declarations,
            // We don't need to check top-level expressions since they are
            // inconsequential to the program.
            expressions: vec![],
        },
        schema,
    ))
}
