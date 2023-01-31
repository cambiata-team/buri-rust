use crate::{
    constraints::Constraint,
    generic_nodes::{GenericDocument, GenericExpression, GenericVariableDeclaration},
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
    ConcreteIntegerLiteralExpression, ConcreteListType, ConcreteRecordType, ConcreteTagUnionType,
    ConcreteType, ConcreteVariableDeclaration, PrimitiveType, TypedVariableDeclaration,
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
enum BroadType {
    Unknown,
    Num,
    Str,
    Function,
    TagUnion,
    List,
    Record,
}

fn compute_broad_type(constraint_vec: &Vec<Constraint>) -> Result<BroadType, ()> {
    let mut broad_type = BroadType::Unknown;
    for constraint in constraint_vec {
        let predicted_type = match constraint {
            Constraint::EqualToPrimitive(primitive) => match primitive {
                PrimitiveType::CompilerBoolean => return Err(()),
                PrimitiveType::Num => BroadType::Num,
                PrimitiveType::Str => BroadType::Str,
            },
            Constraint::ListOfType(_) => BroadType::List,
            Constraint::HasTag(_) | Constraint::TagAtMost(_) => BroadType::TagUnion,
            Constraint::HasField(_) | Constraint::HasMethod(_) => BroadType::Record,
            Constraint::HasReturnType(_) | Constraint::HasArgumentTypes(_) => BroadType::Function,
        };
        match broad_type {
            BroadType::Unknown => {
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
    let constraint_vec = match simplified_schema
        .constraints
        .get(&substitutions.get_canonical_id(type_id))
    {
        Some(x) => x,
        None => return Err(()),
    };
    let broad_type = compute_broad_type(constraint_vec)?;
    match broad_type {
        // If a type does not have constraints, then it does not matter what the type is.
        BroadType::Unknown => Ok(ConcreteType::Primitive(PrimitiveType::CompilerBoolean)),
        BroadType::Num => Ok(ConcreteType::Primitive(PrimitiveType::Num)),
        BroadType::Str => Ok(ConcreteType::Primitive(PrimitiveType::Str)),
        // TODO(aaron) add specific function types to return value
        BroadType::Function => Ok(ConcreteType::Function(Box::new(ConcreteFunctionType {
            argument_types: vec![],
            return_type: None,
        }))),
        BroadType::TagUnion => Ok(resolve_tag_union_type(constraint_vec)),
        // TODO(aaron) add specific element type to return value
        BroadType::List => Ok(ConcreteType::List(Box::new(ConcreteListType {
            element_type: ConcreteType::Primitive(PrimitiveType::CompilerBoolean),
        }))),
        // TODO(aaron) add field names and types to return value
        BroadType::Record => Ok(ConcreteType::Record(Box::new(ConcreteRecordType {
            field_types: HashMap::new(),
        }))),
    }
}

fn resolve_expression<'a>(
    simplified_schema: &TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    expression: GenericExpression<'a>,
) -> Result<ConcreteExpression, ()> {
    match expression {
        GenericExpression::BinaryOperator(generic_binary_operator) => Ok(
            ConcreteExpression::BinaryOperator(Box::new(ConcreteBinaryOperatorExpression {
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
            })),
        ),
        GenericExpression::Block(generic_block) => Ok(ConcreteExpression::Block(Box::new(
            ConcreteBlockExpression {
                expression_type: resolve_generic_type(
                    simplified_schema,
                    substitutions,
                    generic_block.expression_type.type_id,
                )?,
                // TODO(aaron) add block contents to return value
                contents: vec![],
            },
        ))),
        GenericExpression::Boolean(generic_boolean) => Ok(ConcreteExpression::Boolean(Box::new(
            ConcreteBooleanExpression {
                expression_type: resolve_generic_type(
                    simplified_schema,
                    substitutions,
                    generic_boolean.expression_type.type_id,
                )?,
                value: generic_boolean.value,
            },
        ))),
        GenericExpression::Declaration(generic_declaration) => Ok(ConcreteExpression::Declaration(
            Box::new(ConcreteDeclarationExpression {
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
                value: resolve_expression(
                    simplified_schema,
                    substitutions,
                    generic_declaration.value,
                )?,
            }),
        )),
        GenericExpression::Function(generic_function) => Ok(ConcreteExpression::Function(
            Box::new(ConcreteFunctionExpression {
                expression_type: resolve_generic_type(
                    simplified_schema,
                    substitutions,
                    generic_function.expression_type.type_id,
                )?,
                // TODO(aaron) add argument names to return value
                argument_names: vec![],
                body: resolve_expression(simplified_schema, substitutions, generic_function.body)?,
            }),
        )),
        // TODO(aaron) GenericExpression::FunctionArguments
        GenericExpression::Identifier(generic_identifier) => Ok(ConcreteExpression::Identifier(
            Box::new(ConcreteIdentifierExpression {
                expression_type: resolve_generic_type(
                    simplified_schema,
                    substitutions,
                    generic_identifier.expression_type.type_id,
                )?,
                name: generic_identifier.name,
                is_disregarded: generic_identifier.is_disregarded,
            }),
        )),
        // TODO(aaron) GenericExpression::If
        GenericExpression::Integer(generic_integer) => Ok(ConcreteExpression::Integer(Box::new(
            ConcreteIntegerLiteralExpression {
                expression_type: resolve_generic_type(
                    simplified_schema,
                    substitutions,
                    generic_integer.expression_type.type_id,
                )?,
                value: generic_integer.value,
            },
        ))),
        // TODO(aaron) GenericExpression::List
        // TODO(aaron) GenericExpression::Record
        // TODO(aaron) GenericExpression::RecordAssignment
        // TODO(aaron) GenericExpression::StringLiteral
        // TODO(aaron) GenericExpression::Tag
        // TODO(aaron) GenericExpression::UnaryOperator
        _ => unimplemented!(),
    }
}

fn resolve_variable_declaration_types(
    mut input: TopLevelDeclaration<GenericVariableDeclaration>,
) -> Result<TopLevelDeclaration<ConcreteVariableDeclaration>, ()> {
    let mut simplified_schema = input
        .declaration
        .substitutions
        .apply_to_type_schema(input.declaration.schema);
    Ok(TopLevelDeclaration {
        declaration: ConcreteVariableDeclaration {
            declaration_type: resolve_generic_type(
                &mut simplified_schema,
                &mut input.declaration.substitutions,
                input.declaration.declaration.declaration_type.type_id,
            )?,
            identifier_name: input.declaration.declaration.identifier_name,
            expression: resolve_expression(
                &mut simplified_schema,
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
