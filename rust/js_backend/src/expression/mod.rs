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
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use typed_ast::{
        ConcreteIdentifierExpression, ConcreteIntegerLiteralExpression,
        ConcreteStringLiteralExpression, ConcreteType,
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
}
