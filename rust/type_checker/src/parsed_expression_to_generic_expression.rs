use crate::{
    constraints::{Constraint, HasFieldConstraint, HasTagConstraint, TagAtMostConstraint},
    generic_nodes::{
        get_generic_type_id, GenericBinaryOperatorExpression, GenericBlockExpression,
        GenericExpression, GenericIdentifierExpression, GenericIntegerLiteralExpression,
        GenericListExpression, GenericSourcedType, GenericStringLiteralExpression,
        GenericTagExpression, GenericUnaryOperatorExpression,
    },
    type_schema::TypeSchema,
    type_schema_substitutions::TypeSchemaSubstitutions,
    GenericTypeId,
};
use ast::{
    BinaryOperatorNode, BinaryOperatorSymbol, BlockNode, Expression, IdentifierNode, IntegerNode,
    ListNode, StringLiteralNode, TagNode, UnaryOperatorNode, UnaryOperatorSymbol,
};
use std::collections::HashMap;
use typed_ast::{ConcreteType, PrimitiveType};

const fn constrain_equal_to_num() -> Constraint {
    Constraint::EqualToConcrete(ConcreteType::Primitive(PrimitiveType::Num))
}

const fn constrain_equal_to_str() -> Constraint {
    Constraint::EqualToConcrete(ConcreteType::Primitive(PrimitiveType::Str))
}

fn constrain_at_least_true() -> Constraint {
    Constraint::HasTag(HasTagConstraint {
        tag_name: "true".to_owned(),
        tag_content_types: vec![],
    })
}

fn constrain_at_least_false() -> Constraint {
    Constraint::HasTag(HasTagConstraint {
        tag_name: "false".to_owned(),
        tag_content_types: vec![],
    })
}

fn constrain_at_most_boolean_tag() -> Constraint {
    Constraint::TagAtMost(TagAtMostConstraint {
        tags: HashMap::from([("true".to_owned(), vec![]), ("false".to_owned(), vec![])]),
    })
}

struct TranslateBinaryOperatorIdCollection {
    pub type_id: GenericTypeId,
    pub left_child_id: GenericTypeId,
    pub right_child_id: GenericTypeId,
}

fn translate_binary_operator_add_arithmetic_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) {
    schema.insert(id_collection.type_id, constrain_equal_to_num());
    schema.insert(id_collection.left_child_id, constrain_equal_to_num());
    schema.insert(id_collection.right_child_id, constrain_equal_to_num());
}

fn translate_binary_operator_add_concatenate_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) {
    schema.insert(id_collection.type_id, constrain_equal_to_str());
    schema.insert(id_collection.left_child_id, constrain_equal_to_str());
    schema.insert(id_collection.right_child_id, constrain_equal_to_str());
}

fn translate_binary_operator_add_logic_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) {
    schema.insert(id_collection.type_id, constrain_at_least_true());
    schema.insert(id_collection.type_id, constrain_at_least_false());
    schema.insert(id_collection.left_child_id, constrain_at_most_boolean_tag());
    schema.insert(
        id_collection.right_child_id,
        constrain_at_most_boolean_tag(),
    );
}

fn translate_binary_operator_add_equality_constraints(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    id_collection: &TranslateBinaryOperatorIdCollection,
) {
    schema.insert(id_collection.type_id, constrain_at_least_true());
    schema.insert(id_collection.type_id, constrain_at_least_false());
    substitutions.set_types_equal(id_collection.left_child_id, id_collection.right_child_id);
}

fn translate_binary_operator_add_comparison_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) {
    schema.insert(id_collection.type_id, constrain_at_least_true());
    schema.insert(id_collection.type_id, constrain_at_least_false());
    schema.insert(id_collection.left_child_id, constrain_equal_to_num());
    schema.insert(id_collection.right_child_id, constrain_equal_to_num());
}

fn translate_binary_operator_add_function_application_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
    right_child: &GenericExpression,
) -> Result<(), ()> {
    let argument_types: Vec<GenericTypeId> = match &right_child {
        GenericExpression::FunctionArguments(arguments) => {
            arguments.iter().map(get_generic_type_id).collect()
        }
        _ => return Err(()),
    };
    schema.insert(
        id_collection.left_child_id,
        Constraint::HasArgumentTypes(argument_types),
    );
    schema.insert(
        id_collection.left_child_id,
        Constraint::HasReturnType(id_collection.type_id),
    );
    Ok(())
}

fn translate_binary_operator_add_field_lookup_constraints(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    id_collection: &TranslateBinaryOperatorIdCollection,
    right_child: &GenericExpression,
) -> Result<(), ()> {
    let field_name = match right_child {
        GenericExpression::Identifier(identifier_expression) => identifier_expression.name.clone(),
        _ => return Err(()),
    };
    schema.insert(
        id_collection.left_child_id,
        Constraint::HasField(HasFieldConstraint {
            field_name,
            field_type: id_collection.right_child_id,
        }),
    );
    substitutions.set_types_equal(id_collection.type_id, id_collection.right_child_id);
    Ok(())
}

fn translate_binary_operator<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: BinaryOperatorNode<'a>,
) -> Result<GenericBinaryOperatorExpression<'a>, ()> {
    let type_id = schema.make_id();
    substitutions.insert_new_id(type_id);
    let translated_left_child = translate_parsed_expression_to_generic_expression(
        schema,
        substitutions,
        *node.value.left_child,
    )?;
    let translated_right_child = match *node.value.right_child {
        Expression::FunctionApplicationArguments(arguments) => {
            let function_arguments: Result<Vec<GenericExpression>, ()> = arguments
                .value
                .arguments
                .into_iter()
                .map(|expression| {
                    translate_parsed_expression_to_generic_expression(
                        schema,
                        substitutions,
                        expression,
                    )
                })
                .collect();
            GenericExpression::FunctionArguments(function_arguments?)
        }
        _ => translate_parsed_expression_to_generic_expression(
            schema,
            substitutions,
            *node.value.right_child,
        )?,
    };
    let id_collection = TranslateBinaryOperatorIdCollection {
        type_id,
        left_child_id: get_generic_type_id(&translated_left_child),
        right_child_id: match &translated_right_child {
            GenericExpression::FunctionArguments(_) => 0,
            _ => get_generic_type_id(&translated_right_child),
        },
    };
    match node.value.symbol {
        BinaryOperatorSymbol::Add
        | BinaryOperatorSymbol::Subtract
        | BinaryOperatorSymbol::Multiply
        | BinaryOperatorSymbol::Divide
        | BinaryOperatorSymbol::Modulus
        | BinaryOperatorSymbol::Power => {
            translate_binary_operator_add_arithmetic_constraints(schema, &id_collection);
        }
        BinaryOperatorSymbol::Concatenate => {
            translate_binary_operator_add_concatenate_constraints(schema, &id_collection);
        }
        BinaryOperatorSymbol::And | BinaryOperatorSymbol::Or => {
            translate_binary_operator_add_logic_constraints(schema, &id_collection);
        }
        BinaryOperatorSymbol::EqualTo | BinaryOperatorSymbol::NotEqualTo => {
            translate_binary_operator_add_equality_constraints(
                schema,
                substitutions,
                &id_collection,
            );
        }
        BinaryOperatorSymbol::LessThan
        | BinaryOperatorSymbol::LessThanOrEqualTo
        | BinaryOperatorSymbol::GreaterThan
        | BinaryOperatorSymbol::GreaterThanOrEqualTo => {
            translate_binary_operator_add_comparison_constraints(schema, &id_collection);
        }
        BinaryOperatorSymbol::FunctionApplication => {
            translate_binary_operator_add_function_application_constraints(
                schema,
                &id_collection,
                &translated_right_child,
            )?;
        }
        BinaryOperatorSymbol::MethodLookup => unimplemented!(),
        BinaryOperatorSymbol::FieldLookup => {
            translate_binary_operator_add_field_lookup_constraints(
                schema,
                substitutions,
                &id_collection,
                &translated_right_child,
            );
        }
    };
    Ok(GenericBinaryOperatorExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        symbol: node.value.symbol,
        left_child: translated_left_child,
        right_child: translated_right_child,
    })
}

fn translate_block<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: BlockNode<'a>,
) -> Result<GenericBlockExpression<'a>, ()> {
    let type_id = schema.make_id();
    substitutions.insert_new_id(type_id);
    let mut element_translations = Vec::new();
    element_translations.reserve_exact(node.value.len());
    for element in node.value {
        let element_translation =
            translate_parsed_expression_to_generic_expression(schema, substitutions, element)?;
        element_translations.push(element_translation);
    }
    match element_translations.last_mut() {
        None => return Err(()),
        Some(last_element) => {
            substitutions.set_types_equal(get_generic_type_id(&last_element), type_id);
        }
    }
    Ok(GenericBlockExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        contents: element_translations,
    })
}

fn translate_identifier<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: IdentifierNode<'a>,
) -> GenericIdentifierExpression<'a> {
    let type_id = schema.make_id();
    substitutions.insert_new_id(type_id);
    GenericIdentifierExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        name: node.value.name,
        is_disregarded: node.value.is_disregarded,
    }
}

fn translate_integer<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: IntegerNode<'a>,
) -> GenericIntegerLiteralExpression<'a> {
    let type_id = schema.make_id();
    substitutions.insert_new_id(type_id);
    schema.insert(type_id, constrain_equal_to_num());
    GenericIntegerLiteralExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        value: node.value,
    }
}

fn translate_list<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: ListNode<'a>,
) -> Result<GenericListExpression<'a>, ()> {
    let list_type_id = schema.make_id();
    substitutions.insert_new_id(list_type_id);
    let element_type_id = schema.make_id();
    substitutions.insert_new_id(element_type_id);
    schema.insert(list_type_id, Constraint::ListOfType(element_type_id));
    let mut element_translations = Vec::new();
    element_translations.reserve_exact(node.value.len());
    for element in node.value {
        let element_translation =
            translate_parsed_expression_to_generic_expression(schema, substitutions, element)?;
        substitutions.set_types_equal(get_generic_type_id(&element_translation), element_type_id);
        element_translations.push(element_translation);
    }
    Ok(GenericListExpression {
        expression_type: GenericSourcedType {
            type_id: list_type_id,
            source_of_type: node.source,
        },
        contents: element_translations,
    })
}

fn translate_string<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: StringLiteralNode<'a>,
) -> GenericStringLiteralExpression<'a> {
    let type_id = schema.make_id();
    substitutions.insert_new_id(type_id);
    schema.insert(type_id, constrain_equal_to_str());
    GenericStringLiteralExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        value: node.value,
    }
}

fn translate_tag<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: TagNode<'a>,
) -> Result<GenericTagExpression<'a>, ()> {
    let type_id = schema.make_id();
    substitutions.insert_new_id(type_id);
    let translated_content_expressions: Vec<GenericExpression> = match node
        .value
        .contents
        .into_iter()
        .map(|expression| {
            translate_parsed_expression_to_generic_expression(schema, substitutions, expression)
        })
        .collect()
    {
        Ok(x) => x,
        Err(x) => {
            return Err(x);
        }
    };
    let translated_content_types: Vec<GenericTypeId> = translated_content_expressions
        .iter()
        .map(get_generic_type_id)
        .collect();
    schema.insert(
        type_id,
        Constraint::HasTag(HasTagConstraint {
            tag_name: node.value.name.value.clone(),
            tag_content_types: translated_content_types,
        }),
    );
    Ok(GenericTagExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        name: node.value.name.value,
        contents: translated_content_expressions,
    })
}

fn translate_unary_operator<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: UnaryOperatorNode<'a>,
) -> Result<GenericUnaryOperatorExpression<'a>, ()> {
    let type_id = schema.make_id();
    substitutions.insert_new_id(type_id);
    let new_child = match node.value.symbol {
        UnaryOperatorSymbol::Not => {
            schema.insert(type_id, constrain_at_least_true());
            schema.insert(type_id, constrain_at_least_false());
            let translated_child = translate_parsed_expression_to_generic_expression(
                schema,
                substitutions,
                *node.value.child,
            )?;
            schema.insert(
                get_generic_type_id(&translated_child),
                constrain_at_most_boolean_tag(),
            );
            translated_child
        }
        UnaryOperatorSymbol::Negative => {
            schema.insert(type_id, constrain_equal_to_num());
            let translated_child = translate_parsed_expression_to_generic_expression(
                schema,
                substitutions,
                *node.value.child,
            )?;
            schema.insert(
                get_generic_type_id(&translated_child),
                constrain_equal_to_num(),
            );
            translated_child
        }
    };
    Ok(GenericUnaryOperatorExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        symbol: node.value.symbol,
        child: new_child,
    })
}

pub fn translate_parsed_expression_to_generic_expression<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    expression: Expression<'a>,
) -> Result<GenericExpression<'a>, ()> {
    match expression {
        Expression::BinaryOperator(node) => translate_binary_operator(schema, substitutions, node)
            .map(Box::new)
            .map(GenericExpression::BinaryOperator),
        Expression::Block(node) => translate_block(schema, substitutions, node)
            .map(Box::new)
            .map(GenericExpression::Block),
        // TODO(aaron): Expression::Function(node) => translate_function(schema, node),
        Expression::FunctionApplicationArguments(node) => Err(()),
        Expression::Identifier(node) => Ok(GenericExpression::Identifier(Box::new(
            translate_identifier(schema, substitutions, node),
        ))),
        // TODO(aaron): Expression::If(node) => translate_if(schema, node),
        Expression::Integer(node) => Ok(GenericExpression::Integer(Box::new(translate_integer(
            schema,
            substitutions,
            node,
        )))),
        Expression::List(node) => translate_list(schema, substitutions, node)
            .map(Box::new)
            .map(GenericExpression::List),
        // TODO(aaron): Expression::Record(node) => translate_record(schema, node),
        Expression::StringLiteral(node) => Ok(GenericExpression::StringLiteral(Box::new(
            translate_string(schema, substitutions, node),
        ))),
        Expression::Tag(node) => translate_tag(schema, substitutions, node)
            .map(Box::new)
            .map(GenericExpression::Tag),
        Expression::UnaryOperator(node) => translate_unary_operator(schema, substitutions, node)
            .map(Box::new)
            .map(GenericExpression::UnaryOperator),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::{
        BinaryOperatorValue, FunctionApplicationArgumentsNode, FunctionApplicationArgumentsValue,
        IdentifierValue, ListNode, ParserInput, TagIdentifierNode, TagValue, UnaryOperatorValue,
    };

    #[test]
    fn binary_operator_increments_id_counter_by_one_more_than_total_number_of_ids_in_children() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::Add,
                left_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
                right_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 271,
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.next_id, 3);
    }

    #[test]
    fn arithmetic_binary_operator_adds_three_constraints_beyond_those_added_by_its_children() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::Add,
                left_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
                right_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 271,
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 5);
    }

    #[test]
    fn concatenate_binary_operator_adds_three_constraints_beyond_those_added_by_its_children() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::Concatenate,
                left_child: Box::new(Expression::StringLiteral(StringLiteralNode {
                    source: ParserInput::new(""),
                    value: "Hello".to_owned(),
                })),
                right_child: Box::new(Expression::StringLiteral(StringLiteralNode {
                    source: ParserInput::new(""),
                    value: "World".to_owned(),
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 5);
    }

    #[test]
    fn logic_binary_operator_adds_four_constraints_beyond_those_added_by_its_children() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::And,
                left_child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "a".to_owned(),
                        is_disregarded: false,
                    },
                })),
                right_child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "b".to_owned(),
                        is_disregarded: false,
                    },
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 4);
    }

    #[test]
    fn equality_binary_operator_adds_two_constraints_beyond_those_added_by_its_children() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::EqualTo,
                left_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
                right_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 271,
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 4);
    }

    #[test]
    fn equality_binary_operator_only_has_two_canonical_ids_when_children_only_have_one_type_each() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::EqualTo,
                left_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
                right_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 271,
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(substitutions.count_canonical_ids(), 2);
    }

    #[test]
    fn ordered_comparison_binary_operator_adds_four_constraints_beyond_those_added_by_its_children()
    {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::LessThan,
                left_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
                right_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 271,
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 6);
    }

    #[test]
    fn function_arguments_binary_operator_has_one_more_canonical_id_than_sum_of_canonical_ids_in_children(
    ) {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::FunctionApplication,
                left_child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "a".to_owned(),
                        is_disregarded: false,
                    },
                })),
                right_child: Box::new(Expression::FunctionApplicationArguments(
                    FunctionApplicationArgumentsNode {
                        source: ParserInput::new(""),
                        value: FunctionApplicationArgumentsValue {
                            arguments: vec![
                                Expression::StringLiteral(StringLiteralNode {
                                    source: ParserInput::new(""),
                                    value: "Hello".to_owned(),
                                }),
                                Expression::Integer(IntegerNode {
                                    source: ParserInput::new(""),
                                    value: 314,
                                }),
                                Expression::Integer(IntegerNode {
                                    source: ParserInput::new(""),
                                    value: 271,
                                }),
                            ],
                        },
                    },
                )),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(substitutions.count_canonical_ids(), 5);
    }

    #[test]
    fn function_arguments_binary_operator_adds_two_constraints_beyond_those_added_by_its_children()
    {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::FunctionApplication,
                left_child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "a".to_owned(),
                        is_disregarded: false,
                    },
                })),
                right_child: Box::new(Expression::FunctionApplicationArguments(
                    FunctionApplicationArgumentsNode {
                        source: ParserInput::new(""),
                        value: FunctionApplicationArgumentsValue {
                            arguments: vec![
                                Expression::StringLiteral(StringLiteralNode {
                                    source: ParserInput::new(""),
                                    value: "Hello".to_owned(),
                                }),
                                Expression::Integer(IntegerNode {
                                    source: ParserInput::new(""),
                                    value: 314,
                                }),
                                Expression::Integer(IntegerNode {
                                    source: ParserInput::new(""),
                                    value: 271,
                                }),
                            ],
                        },
                    },
                )),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 5);
    }

    #[test]
    fn field_lookup_binary_operator_only_has_two_canonical_ids_when_both_left_and_right_are_identifiers(
    ) {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::FieldLookup,
                left_child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "a".to_owned(),
                        is_disregarded: false,
                    },
                })),
                right_child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "b".to_owned(),
                        is_disregarded: false,
                    },
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(substitutions.count_canonical_ids(), 2);
    }

    #[test]
    fn field_lookup_binary_operator_only_adds_one_constraint_when_both_left_and_right_are_identifiers(
    ) {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::FieldLookup,
                left_child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "a".to_owned(),
                        is_disregarded: false,
                    },
                })),
                right_child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "b".to_owned(),
                        is_disregarded: false,
                    },
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 1);
    }

    #[test]
    fn binary_operator_preserves_symbol() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::BinaryOperator(BinaryOperatorNode {
            source: ParserInput::new(""),
            value: BinaryOperatorValue {
                symbol: BinaryOperatorSymbol::Add,
                left_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
                right_child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 271,
                })),
            },
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::BinaryOperator(binary_operator_expression)) = result {
            assert_eq!(
                (*binary_operator_expression).symbol,
                BinaryOperatorSymbol::Add
            )
        } else {
            panic!();
        }
    }

    #[test]
    fn block_input_increments_id_counter_by_two_more_than_total_number_of_ids_in_the_contents() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Block(BlockNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.next_id, 4);
    }

    #[test]
    fn for_block_input_each_element_in_input_block_has_corresponding_element_in_translated_block() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Block(BlockNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::Block(block_expression)) = result {
            assert_eq!((*block_expression).contents.len(), 3);
        } else {
            panic!();
        }
    }

    #[test]
    fn block_input_with_primitive_elements_has_as_many_canonical_ids_as_elements() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Block(BlockNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(substitutions.count_canonical_ids(), 3);
    }

    #[test]
    fn function_application_arguments_does_not_increment_id_counter() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression =
            Expression::FunctionApplicationArguments(FunctionApplicationArgumentsNode {
                source: ParserInput::new(""),
                value: FunctionApplicationArgumentsValue { arguments: vec![] },
            });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.next_id, 0);
    }

    #[test]
    fn function_application_arguments_input_errors() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression =
            Expression::FunctionApplicationArguments(FunctionApplicationArgumentsNode {
                source: ParserInput::new(""),
                value: FunctionApplicationArgumentsValue { arguments: vec![] },
            });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert!(result.is_err());
    }

    #[test]
    fn identifier_input_preserves_name() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Identifier(IdentifierNode {
            source: ParserInput::new(""),
            value: IdentifierValue {
                name: "hello".to_owned(),
                is_disregarded: false,
            },
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::Identifier(identifier_expression)) = result {
            assert_eq!((*identifier_expression).name, "hello");
        } else {
            panic!();
        }
    }

    #[test]
    fn integer_input_increments_id_counter_by_one() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Integer(IntegerNode {
            source: ParserInput::new(""),
            value: 314,
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.next_id, 1);
    }

    #[test]
    fn integer_input_adds_one_constraint() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Integer(IntegerNode {
            source: ParserInput::new(""),
            value: 314,
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 1);
    }

    #[test]
    fn integer_input_returns_integer_with_preserved_value() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Integer(IntegerNode {
            source: ParserInput::new(""),
            value: 314,
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::Integer(integer_expression)) = result {
            assert_eq!((*integer_expression).value, 314);
        } else {
            panic!();
        }
    }

    #[test]
    fn list_input_increments_id_counter_by_two_more_than_total_number_of_ids_in_the_contents() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::List(ListNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.next_id, 5);
    }

    #[test]
    fn list_input_adds_one_constraint_beyond_those_added_by_its_contents() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::List(ListNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 4);
    }

    #[test]
    fn for_list_input_each_element_in_input_list_has_corresponding_element_in_translated_list() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::List(ListNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::List(list_node)) = result {
            assert_eq!((*list_node).contents.len(), 3);
        } else {
            panic!();
        }
    }

    #[test]
    fn list_input_with_primitive_elements_has_only_two_canonical_ids() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::List(ListNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(substitutions.count_canonical_ids(), 2);
    }

    #[test]
    fn string_input_increments_id_counter_by_one() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::StringLiteral(StringLiteralNode {
            source: ParserInput::new(""),
            value: "hello".to_owned(),
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.next_id, 1);
    }

    #[test]
    fn string_input_adds_one_constraint() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::StringLiteral(StringLiteralNode {
            source: ParserInput::new(""),
            value: "hello".to_owned(),
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 1);
    }

    #[test]
    fn string_input_returns_string_with_preserved_value() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::StringLiteral(StringLiteralNode {
            source: ParserInput::new(""),
            value: "hello".to_owned(),
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::StringLiteral(string_literal_expression)) = result {
            assert_eq!((*string_literal_expression).value, "hello");
        } else {
            panic!();
        }
    }

    #[test]
    fn tag_increments_id_counter_by_one() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Tag(TagNode {
            source: ParserInput::new(""),
            value: TagValue {
                name: TagIdentifierNode {
                    source: ParserInput::new(""),
                    value: "a".to_owned(),
                },
                contents: vec![],
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.next_id, 1);
    }

    #[test]
    fn tag_with_no_contents_adds_one_constraint() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Tag(TagNode {
            source: ParserInput::new(""),
            value: TagValue {
                name: TagIdentifierNode {
                    source: ParserInput::new(""),
                    value: "a".to_owned(),
                },
                contents: vec![],
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 1);
    }

    #[test]
    fn tag_preserves_name() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::Tag(TagNode {
            source: ParserInput::new(""),
            value: TagValue {
                name: TagIdentifierNode {
                    source: ParserInput::new(""),
                    value: "a".to_owned(),
                },
                contents: vec![],
            },
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::Tag(tag_expression)) = result {
            assert_eq!((*tag_expression).name, "a");
        } else {
            panic!();
        }
    }

    #[test]
    fn unary_operator_input_increments_id_counter_by_one_more_than_added_by_its_child() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::UnaryOperator(UnaryOperatorNode {
            source: ParserInput::new(""),
            value: UnaryOperatorValue {
                symbol: UnaryOperatorSymbol::Negative,
                child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.next_id, 2);
    }

    #[test]
    fn unary_operator_negative_input_adds_two_constraints_beyond_those_added_by_the_child() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::UnaryOperator(UnaryOperatorNode {
            source: ParserInput::new(""),
            value: UnaryOperatorValue {
                symbol: UnaryOperatorSymbol::Negative,
                child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 3);
    }

    #[test]
    fn unary_operator_not_input_adds_three_constraints_beyond_those_added_by_the_child() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::UnaryOperator(UnaryOperatorNode {
            source: ParserInput::new(""),
            value: UnaryOperatorValue {
                symbol: UnaryOperatorSymbol::Not,
                child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "hello".to_owned(),
                        is_disregarded: false,
                    },
                })),
            },
        });
        let _ = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        assert_eq!(schema.number_of_constraints(), 3);
    }

    #[test]
    fn unary_operator_negative_input_preserves_symbol() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::UnaryOperator(UnaryOperatorNode {
            source: ParserInput::new(""),
            value: UnaryOperatorValue {
                symbol: UnaryOperatorSymbol::Negative,
                child: Box::new(Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 314,
                })),
            },
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::UnaryOperator(unary_operator_expression)) = result {
            assert_eq!(
                (*unary_operator_expression).symbol,
                UnaryOperatorSymbol::Negative
            )
        } else {
            panic!();
        }
    }

    #[test]
    fn unary_operator_not_input_preserves_symbol() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::UnaryOperator(UnaryOperatorNode {
            source: ParserInput::new(""),
            value: UnaryOperatorValue {
                symbol: UnaryOperatorSymbol::Not,
                child: Box::new(Expression::Identifier(IdentifierNode {
                    source: ParserInput::new(""),
                    value: IdentifierValue {
                        name: "hello".to_owned(),
                        is_disregarded: false,
                    },
                })),
            },
        });
        let result = translate_parsed_expression_to_generic_expression(
            &mut schema,
            &mut substitutions,
            expression,
        );
        if let Ok(GenericExpression::UnaryOperator(unary_operator_expression)) = result {
            assert_eq!(
                (*unary_operator_expression).symbol,
                UnaryOperatorSymbol::Not
            )
        } else {
            panic!();
        }
    }
}
