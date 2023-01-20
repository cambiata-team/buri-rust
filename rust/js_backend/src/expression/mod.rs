mod list;
mod record;

use self::record::print_record;
use crate::{
    identifier::print_identifier,
    literals::{print_integer_literal, print_string_literal},
};
use concrete_ast::ConcreteExpression;

pub fn print_expression(expression: &ConcreteExpression) -> String {
    match expression {
        ConcreteExpression::Identifier(identifier) => print_identifier(identifier),
        ConcreteExpression::Integer(integer) => print_integer_literal(integer),
        ConcreteExpression::StringLiteral(string) => print_string_literal(string),
        ConcreteExpression::Record(record) => print_record(record),
        ConcreteExpression::List(list) => list::print_list(list),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use concrete_ast::{
        ConcreteIdentifierExpression, ConcreteIntegerLiteralExpression,
        ConcreteStringLiteralExpression,
    };

    #[test]
    fn can_print_identifier() {
        let expression = ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
            name: "foo".to_string(),
        }));
        assert_eq!(print_expression(&expression), "foo");
    }

    #[test]
    fn can_print_integer_literal() {
        let expression =
            ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression { value: 42 }));
        assert_eq!(print_expression(&expression), "42");
    }

    #[test]
    fn can_print_string_literal() {
        let expression =
            ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
                value: "foo".to_string(),
            }));
        assert_eq!(print_expression(&expression), "\"foo\"");
    }

    #[test]
    fn can_print_record() {
        let expression =
            ConcreteExpression::Record(Box::new(concrete_ast::ConcreteRecordExpression {
                contents: vec![
                    (
                        "foo".to_string(),
                        ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                            value: 42,
                        })),
                    ),
                    (
                        "bar".to_string(),
                        ConcreteExpression::StringLiteral(Box::new(
                            ConcreteStringLiteralExpression {
                                value: "baz".to_string(),
                            },
                        )),
                    ),
                ],
            }));
        assert_eq!(print_expression(&expression), "{foo: 42, bar: \"baz\"}");
    }

    #[test]
    fn can_print_list() {
        let expression = ConcreteExpression::List(Box::new(concrete_ast::ConcreteListExpression {
            contents: vec![ConcreteExpression::Integer(Box::new(
                ConcreteIntegerLiteralExpression { value: 42 },
            ))],
        }));
        assert_eq!(print_expression(&expression), "[42]");
    }
}
