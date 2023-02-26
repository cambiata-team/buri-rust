use ast::UnaryOperatorSymbol;
use typed_ast::ConcreteUnaryOperatorExpression;

fn print_unary_operator_symbol(operator: &UnaryOperatorSymbol) -> String {
    match operator {
        UnaryOperatorSymbol::Negative => "-".to_string(),
        UnaryOperatorSymbol::Not => "!".to_string(),
    }
}

pub fn print_unary_operator(expression: &ConcreteUnaryOperatorExpression) -> String {
    format!(
        "{}{}",
        print_unary_operator_symbol(&expression.symbol),
        super::print_expression(&expression.child)
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::UnaryOperatorSymbol;
    use typed_ast::{ConcreteExpression, ConcreteType};

    #[test]
    fn can_print_negative_integer() {
        let expression = ConcreteUnaryOperatorExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            symbol: UnaryOperatorSymbol::Negative,
            child: ConcreteExpression::integer_for_test(42),
        };
        assert_eq!(print_unary_operator(&expression), "-42");
    }

    #[test]
    fn can_print_not_boolean() {
        let expression = ConcreteUnaryOperatorExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            symbol: UnaryOperatorSymbol::Not,
            child: ConcreteExpression::identifier_for_test("foo"),
        };
        assert_eq!(print_unary_operator(&expression), "!Bfoo");
    }
}
