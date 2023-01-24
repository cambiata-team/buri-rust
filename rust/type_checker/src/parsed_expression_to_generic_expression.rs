use crate::{
    constraints::Constraint,
    generic_nodes::{
        get_generic_type_id, GenericExpression, GenericIntegerLiteralExpression,
        GenericListExpression, GenericSourcedType, GenericStringLiteralExpression,
    },
    type_schema::TypeSchema,
    type_schema_substitutions::TypeSchemaSubstitutions,
};
use ast::{Expression, IntegerNode, ListNode, StringLiteralNode};
use typed_ast::{ConcreteType, PrimitiveType};

fn translate_integer<'a>(
    schema: &mut TypeSchema,
    substitutions: &mut TypeSchemaSubstitutions,
    node: IntegerNode<'a>,
) -> GenericIntegerLiteralExpression<'a> {
    let type_id = schema.make_id();
    substitutions.insert_new_id(type_id);
    schema.constraints.insert(
        type_id,
        vec![Constraint::EqualToConcrete(ConcreteType::Primitive(
            PrimitiveType::Num,
        ))],
    );
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
    schema
        .constraints
        .insert(list_type_id, vec![Constraint::ListOfType(element_type_id)]);
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
    schema.constraints.insert(
        type_id,
        vec![Constraint::EqualToConcrete(ConcreteType::Primitive(
            PrimitiveType::Str,
        ))],
    );
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
        // TODO(aaron): Expression::Block(node) => translate_block(schema, node),
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
        FunctionApplicationArgumentsNode, FunctionApplicationArgumentsValue, ListNode, ParsedNode,
        ParserInput,
    };

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
        match result.unwrap() {
            GenericExpression::Integer(integer_expression) => {
                assert_eq!((*integer_expression).value, 314);
            }
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn list_input_increments_id_counter_by_two_more_than_total_number_of_ids_in_the_contents() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::List(ParsedNode {
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
        let expression = Expression::List(ParsedNode {
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
        let expression = Expression::List(ParsedNode {
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
        match result.unwrap() {
            GenericExpression::List(list_node) => {
                assert_eq!((*list_node).contents.len(), 3);
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn list_input_with_primitive_elements_has_only_two_canonical_ids() {
        let mut schema = TypeSchema::new();
        let mut substitutions = TypeSchemaSubstitutions::new();
        let expression = Expression::List(ParsedNode {
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
        match result.unwrap() {
            GenericExpression::StringLiteral(string_literal_expression) => {
                assert_eq!((*string_literal_expression).value, "hello");
            }
            _ => panic!("Expected StringLiteral"),
        }
    }
}
