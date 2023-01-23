mod list;
mod record;

use crate::{
    identifier::print_identifier,
    literals::{print_integer_literal, print_string_literal},
};
use typed_ast::ConcreteExpression;

pub fn print_expression(expression: &ConcreteExpression) -> String {
    match expression {
        ConcreteExpression::Identifier(identifier) => print_identifier(identifier),
        ConcreteExpression::Integer(integer) => print_integer_literal(integer),
        ConcreteExpression::StringLiteral(string) => print_string_literal(string),
        ConcreteExpression::Record(record) => record::print_record(record),
        ConcreteExpression::List(list) => list::print_list(list),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use typed_ast::{
        ConcreteIdentifierExpression, ConcreteIntegerLiteralExpression, ConcreteListExpression,
        ConcreteRecordExpression, ConcreteStringLiteralExpression, ConcreteType,
    };

    #[test]
    fn can_print_identifier() {
        let expression = ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
            expression_type: ConcreteType::default_for_test(),
            name: "foo".to_string(),
        }));
        assert_eq!(print_expression(&expression), "foo");
    }

    #[test]
    fn can_print_integer_literal() {
        let expression = ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
            expression_type: ConcreteType::default_for_test(),
            value: 42,
        }));
        assert_eq!(print_expression(&expression), "42");
    }

    #[test]
    fn can_print_string_literal() {
        let expression =
            ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
                expression_type: ConcreteType::default_for_test(),
                value: "foo".to_string(),
            }));
        assert_eq!(print_expression(&expression), "\"foo\"");
    }

    #[test]
    fn can_print_record() {
        let expression = ConcreteExpression::Record(Box::new(ConcreteRecordExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: HashMap::from([
                (
                    "foo".to_string(),
                    ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                        expression_type: ConcreteType::default_integer_for_test(),
                        value: 42,
                    })),
                ),
                (
                    "bar".to_string(),
                    ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
                        expression_type: ConcreteType::default_string_for_test(),
                        value: "baz".to_string(),
                    })),
                ),
            ]),
        }));
        // Because of the HashMap, the order of the keys is not guaranteed.
        // However, the order doesn't matter so we can accept either one.
        assert!(
            print_expression(&expression) == "{bar: \"baz\", foo: 42}"
                || print_expression(&expression) == "{foo: 42, bar: \"baz\"}"
        );
    }

    #[test]
    fn can_print_list() {
        let list = ConcreteExpression::List(Box::new(ConcreteListExpression {
            expression_type: ConcreteType::default_list_for_test(),
            contents: vec![ConcreteExpression::Integer(Box::new(
                ConcreteIntegerLiteralExpression {
                    expression_type: ConcreteType::default_integer_for_test(),
                    value: 42,
                },
            ))],
        }));
        assert_eq!(print_expression(&list), "[42]");
    }
}
