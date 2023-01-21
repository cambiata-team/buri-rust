use ast::UnaryOperatorSymbol;
use concrete_ast::ConcreteUnaryOperatorExpression;

const fn should_include_space(operator: &UnaryOperatorSymbol) -> bool {
    match operator {
        UnaryOperatorSymbol::Negative => false,
        UnaryOperatorSymbol::Not => true,
    }
}

fn print_unary_operator_symbol(operator: &UnaryOperatorSymbol) -> String {
    match operator {
        UnaryOperatorSymbol::Negative => "-".to_string(),
        UnaryOperatorSymbol::Not => "not".to_string(),
    }
}

pub fn print_unary_operator(expression: &ConcreteUnaryOperatorExpression) -> String {
    let mut result = String::new();
    result.push_str(&print_unary_operator_symbol(&expression.symbol));
    if should_include_space(&expression.symbol) {
        result.push(' ');
    }
    result.push_str(&super::print_expression(&expression.child));
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::UnaryOperatorSymbol;
    use concrete_ast::{
        ConcreteExpression, ConcreteIdentifierExpression, ConcreteIntegerLiteralExpression,
    };

    #[test]
    fn can_print_negative_integer() {
        let expression = ConcreteUnaryOperatorExpression {
            symbol: UnaryOperatorSymbol::Negative,
            child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 42,
            })),
        };
        assert_eq!(print_unary_operator(&expression), "-42");
    }

    #[test]
    fn can_print_not_boolean() {
        let expression = ConcreteUnaryOperatorExpression {
            symbol: UnaryOperatorSymbol::Not,
            child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "true".to_string(),
            })),
        };
        assert_eq!(print_unary_operator(&expression), "not true");
    }
}
