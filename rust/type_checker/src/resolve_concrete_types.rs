use crate::{
    constraints::Constraint,
    generic_nodes::{
        GenericBinaryOperatorExpression, GenericBlockExpression, GenericBooleanExpression,
        GenericDeclarationExpression, GenericDocument, GenericExpression,
        GenericFunctionExpression, GenericIdentifierExpression, GenericIntegerLiteralExpression,
        GenericListExpression, GenericStringLiteralExpression, GenericVariableDeclaration,
    },
    type_schema::TypeSchema,
    type_schema_substitutions::TypeSchemaSubstitutions,
    GenericTypeId,
};
use ast::TopLevelDeclaration;
use std::collections::HashMap;
use typed_ast::{
    ConcreteBinaryOperatorExpression, ConcreteBlockExpression, ConcreteBooleanExpression,
    ConcreteDeclarationExpression, ConcreteDocument, ConcreteExpression,
    ConcreteFunctionExpression, ConcreteFunctionType, ConcreteIdentifierExpression,
    ConcreteIntegerLiteralExpression, ConcreteListExpression, ConcreteListType, ConcreteRecordType,
    ConcreteStringLiteralExpression, ConcreteTagUnionType, ConcreteType,
    ConcreteVariableDeclaration, PrimitiveType, TypedVariableDeclaration,
};

// TODO(aaron) return correct tag for non-boolean
fn resolve_tag_union_type(constraint_vec: &Vec<Constraint>) -> ConcreteType {
    for constraint in constraint_vec {
        match constraint {
            Constraint::HasTag(tag) => {
                if (tag.tag_name != "true" && tag.tag_name != "false")
                    || tag.tag_content_types.len() > 0
                {
                    return ConcreteType::TagUnion(Box::new(ConcreteTagUnionType {
                        tag_types: HashMap::new(),
                    }));
                }
            }
            _ => {}
        }
    }
    ConcreteType::Primitive(PrimitiveType::CompilerBoolean)
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Indicates the category of a data type without necessarily specifying the type itself.
/// For primitives, the type is specified exactly.
enum TypeCategory {
    /// The category of the type is not known.
    Unknown,
    /// The type is Num.
    Num,
    /// The type is Str.
    Str,
    /// The type is a function type.
    /// The argument types and return types are not specified.
    Function,
    /// The type is a tag union type.
    /// The names and types of the tags are not specified.
    TagUnion,
    /// The type is a list type.
    /// The type of the list's elements is not specified.
    List,
    /// The type is a record type.
    /// The names and types of the fields are not specified.
    Record,
}

fn compute_broad_type(constraint_vec: &Vec<Constraint>) -> Result<TypeCategory, ()> {
    let mut broad_type = TypeCategory::Unknown;
    for constraint in constraint_vec {
        let predicted_type = match constraint {
            Constraint::EqualToPrimitive(primitive) => match primitive {
                PrimitiveType::CompilerBoolean => return Err(()),
                PrimitiveType::Num => TypeCategory::Num,
                PrimitiveType::Str => TypeCategory::Str,
            },
            Constraint::ListOfType(_) => TypeCategory::List,
            Constraint::HasTag(_) | Constraint::TagAtMost(_) => TypeCategory::TagUnion,
            Constraint::HasField(_) | Constraint::HasMethod(_) => TypeCategory::Record,
            Constraint::HasReturnType(_) | Constraint::HasArgumentTypes(_) => {
                TypeCategory::Function
            }
        };
        match broad_type {
            TypeCategory::Unknown => {
                broad_type = predicted_type;
            }
            _ => {
                if predicted_type != broad_type {
                    return Err(());
                }
            }
        };
    }
    Ok(broad_type)
}

fn resolve_generic_type(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    type_id: GenericTypeId,
) -> Result<ConcreteType, ()> {
    let Some(constraint_vec) = simplified_schema
        .constraints
        .get(&substitutions.get_canonical_id(type_id)) else {
        return Err(());
    };
    let broad_type = compute_broad_type(constraint_vec)?;
    match broad_type {
        // If a type does not have constraints, then it does not matter what the type is.
        TypeCategory::Unknown => Ok(ConcreteType::Primitive(PrimitiveType::CompilerBoolean)),
        TypeCategory::Num => Ok(ConcreteType::Primitive(PrimitiveType::Num)),
        TypeCategory::Str => Ok(ConcreteType::Primitive(PrimitiveType::Str)),
        // TODO(aaron) add specific function types to return value
        TypeCategory::Function => Ok(ConcreteType::Function(Box::new(ConcreteFunctionType {
            argument_types: vec![],
            return_type: None,
        }))),
        TypeCategory::TagUnion => Ok(resolve_tag_union_type(constraint_vec)),
        // TODO(aaron) add specific element type to return value
        TypeCategory::List => Ok(ConcreteType::List(Box::new(ConcreteListType {
            element_type: ConcreteType::Primitive(PrimitiveType::CompilerBoolean),
        }))),
        // TODO(aaron) add field names and types to return value
        TypeCategory::Record => Ok(ConcreteType::Record(Box::new(ConcreteRecordType {
            field_types: HashMap::new(),
        }))),
    }
}

fn resolve_binary_operator(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_binary_operator: GenericBinaryOperatorExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::BinaryOperator(Box::new(
        ConcreteBinaryOperatorExpression {
            expression_type: resolve_generic_type(
                simplified_schema,
                substitutions,
                generic_binary_operator.expression_type.type_id,
            )?,
            symbol: generic_binary_operator.symbol,
            left_child: resolve_expression(
                simplified_schema,
                substitutions,
                generic_binary_operator.left_child,
            )?,
            right_child: resolve_expression(
                simplified_schema,
                substitutions,
                generic_binary_operator.right_child,
            )?,
        },
    )))
}

fn resolve_block(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_block: &GenericBlockExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::Block(Box::new(
        ConcreteBlockExpression {
            expression_type: resolve_generic_type(
                simplified_schema,
                substitutions,
                generic_block.expression_type.type_id,
            )?,
            // TODO(aaron) add block contents to return value
            contents: vec![],
        },
    )))
}

fn resolve_boolean(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_boolean: &GenericBooleanExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::Boolean(Box::new(
        ConcreteBooleanExpression {
            expression_type: resolve_generic_type(
                simplified_schema,
                substitutions,
                generic_boolean.expression_type.type_id,
            )?,
            value: generic_boolean.value,
        },
    )))
}

fn resolve_declaration(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_declaration: GenericDeclarationExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::Declaration(Box::new(
        ConcreteDeclarationExpression {
            expression_type: resolve_generic_type(
                simplified_schema,
                substitutions,
                generic_declaration.expression_type.type_id,
            )?,
            identifier: match resolve_expression(
                simplified_schema,
                substitutions,
                GenericExpression::Identifier(Box::new(generic_declaration.identifier)),
            )? {
                ConcreteExpression::Identifier(x) => *x,
                _ => return Err(()),
            },
            value: resolve_expression(simplified_schema, substitutions, generic_declaration.value)?,
        },
    )))
}

fn resolve_function(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_function: GenericFunctionExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::Function(Box::new(
        ConcreteFunctionExpression {
            expression_type: resolve_generic_type(
                simplified_schema,
                substitutions,
                generic_function.expression_type.type_id,
            )?,
            // TODO(aaron) add argument names to return value
            argument_names: vec![],
            body: resolve_expression(simplified_schema, substitutions, generic_function.body)?,
        },
    )))
}

fn resolve_identifier(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_identifier: GenericIdentifierExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::Identifier(Box::new(
        ConcreteIdentifierExpression {
            expression_type: resolve_generic_type(
                simplified_schema,
                substitutions,
                generic_identifier.expression_type.type_id,
            )?,
            name: generic_identifier.name,
            is_disregarded: generic_identifier.is_disregarded,
        },
    )))
}

fn resolve_integer(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_integer: &GenericIntegerLiteralExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::Integer(Box::new(
        ConcreteIntegerLiteralExpression {
            expression_type: resolve_generic_type(
                simplified_schema,
                substitutions,
                generic_integer.expression_type.type_id,
            )?,
            value: generic_integer.value,
        },
    )))
}

fn resolve_list(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_list: &GenericListExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::List(Box::new(ConcreteListExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            substitutions,
            generic_list.expression_type.type_id,
        )?,
        contents: generic_list
            .contents
            .iter()
            .map(|item| resolve_expression(simplified_schema, substitutions, item.clone()))
            .collect::<Result<Vec<_>, _>>()?,
    })))
}

fn resolve_string_literal(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    generic_string_literal: GenericStringLiteralExpression,
) -> Result<ConcreteExpression, ()> {
    Ok(ConcreteExpression::StringLiteral(Box::new(
        ConcreteStringLiteralExpression {
            expression_type: resolve_generic_type(
                simplified_schema,
                substitutions,
                generic_string_literal.expression_type.type_id,
            )?,
            value: generic_string_literal.value,
        },
    )))
}

fn resolve_expression(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    expression: GenericExpression,
) -> Result<ConcreteExpression, ()> {
    match expression {
        GenericExpression::BinaryOperator(generic_binary_operator) => {
            resolve_binary_operator(simplified_schema, substitutions, *generic_binary_operator)
        }
        GenericExpression::Block(generic_block) => {
            resolve_block(simplified_schema, substitutions, &generic_block)
        }
        GenericExpression::Boolean(generic_boolean) => {
            resolve_boolean(simplified_schema, substitutions, &generic_boolean)
        }
        GenericExpression::Declaration(generic_declaration) => {
            resolve_declaration(simplified_schema, substitutions, *generic_declaration)
        }
        GenericExpression::Function(generic_function) => {
            resolve_function(simplified_schema, substitutions, *generic_function)
        }
        // TODO(aaron) GenericExpression::FunctionArguments
        GenericExpression::Identifier(generic_identifier) => {
            resolve_identifier(simplified_schema, substitutions, *generic_identifier)
        }
        // TODO(aaron) GenericExpression::If
        GenericExpression::Integer(generic_integer) => {
            resolve_integer(simplified_schema, substitutions, &generic_integer)
        }
        // TODO(aaron) GenericExpression::Record
        // TODO(aaron) GenericExpression::RecordAssignment
        GenericExpression::List(list) => resolve_list(simplified_schema, substitutions, &list),
        GenericExpression::StringLiteral(generic_string_literal) => {
            resolve_string_literal(simplified_schema, substitutions, *generic_string_literal)
        }
        // TODO(aaron) GenericExpression::Tag
        // TODO(aaron) GenericExpression::UnaryOperator
        _ => unimplemented!(),
    }
}

fn resolve_variable_declaration_types(
    mut input: TopLevelDeclaration<GenericVariableDeclaration>,
) -> Result<TopLevelDeclaration<ConcreteVariableDeclaration>, ()> {
    let simplified_schema = input
        .declaration
        .substitutions
        .apply_to_type_schema(input.declaration.schema);
    Ok(TopLevelDeclaration {
        declaration: ConcreteVariableDeclaration {
            declaration_type: resolve_generic_type(
                &simplified_schema,
                &mut input.declaration.substitutions,
                input.declaration.declaration.declaration_type.type_id,
            )?,
            identifier_name: input.declaration.declaration.identifier_name,
            expression: resolve_expression(
                &simplified_schema,
                &mut input.declaration.substitutions,
                input.declaration.declaration.expression,
            )?,
        },
        is_exported: input.is_exported,
    })
}

pub fn resolve_concrete_types(input: GenericDocument) -> Result<ConcreteDocument, ()> {
    let variable_declarations: Result<
        Vec<TopLevelDeclaration<TypedVariableDeclaration<ConcreteType>>>,
        (),
    > = input
        .variable_declarations
        .into_iter()
        .map(resolve_variable_declaration_types)
        .collect();
    Ok(ConcreteDocument {
        imports: input.imports,
        // TODO(aaron) add type declarations to return value
        type_declarations: vec![],
        variable_declarations: variable_declarations?,
        // TODO(aaron) add top level expressions to return value
        expressions: vec![],
    })
}
