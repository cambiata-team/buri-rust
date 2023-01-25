use crate::{
    constraints::{Constraint, HasTagConstraint, TagAtMostConstraint},
    generic_nodes::{
        get_generic_type_id, GenericBlockExpression, GenericExpression,
        GenericIntegerLiteralExpression, GenericListExpression, GenericSourcedType,
        GenericStringLiteralExpression,
    },
    type_schema::TypeSchema,
    type_schema_substitutions::TypeSchemaSubstitutions,
};
use ast::{BlockNode, Expression, IntegerNode, ListNode, StringLiteralNode};
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

pub fn translate_parsed_expression_to_generic_expression<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    expression: Expression<'a>,
) -> Result<GenericExpression<'a>, ()> {
    match expression {
        // TODO(aaron): Expression::BinaryOperator(node) => translate_binary_operator(schema, node),
        Expression::Block(node) => translate_block(schema, substitutions, node)
            .map(Box::new)
            .map(GenericExpression::Block),
        // TODO(aaron): Expression::Function(node) => translate_function(schema, node),
        Expression::FunctionApplicationArguments(node) => Err(()),
        // TODO(aaron): Expression::Identifier(node) => translate_identifier(schema, node),
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
        // TODO(aaron): Expression::Tag(node) => translate_tag(schema, node),
        // TODO(aaron): Expression::UnaryOperator(node) => translate_unary_operator(schema, node),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::{
        FunctionApplicationArgumentsNode, FunctionApplicationArgumentsValue, ListNode, ParserInput,
    };

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
        assert_eq!(schema.constraints.len(), 1);
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
        assert_eq!(schema.constraints.len(), 4);
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
        assert_eq!(schema.constraints.len(), 1);
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
}
