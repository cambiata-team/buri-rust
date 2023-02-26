use ast::BinaryOperatorSymbol;
use typed_ast::{ConcreteBinaryOperatorExpression, ConcreteExpression};

enum OperatorFormat {
    /// Do no formatting to the operator. Simply {left}{operator}{right}
    Naked,
    /// Add parenthesis around the entire expression. ({left}{operator}{right})
    Parenthesized,
    /// Call the operator like a method. ({left}).{operator}({right})
    Method,
}

fn print_operator(operator: &BinaryOperatorSymbol) -> String {
    match operator {
        BinaryOperatorSymbol::Add => "add".to_string(),
        BinaryOperatorSymbol::Subtract => "subtract".to_string(),
        BinaryOperatorSymbol::Multiply => "multiply".to_string(),
        BinaryOperatorSymbol::Divide => "divide".to_string(),
        BinaryOperatorSymbol::Power => "power".to_string(),
        BinaryOperatorSymbol::Modulus => "modulo".to_string(),
        BinaryOperatorSymbol::EqualTo => "equals".to_string(),
        BinaryOperatorSymbol::NotEqualTo => "notEquals".to_string(),
        BinaryOperatorSymbol::LessThan => "lessThan".to_string(),
        BinaryOperatorSymbol::LessThanOrEqualTo => "lessThanOrEquals".to_string(),
        BinaryOperatorSymbol::GreaterThan => "greaterThan".to_string(),
        BinaryOperatorSymbol::GreaterThanOrEqualTo => "greaterThanOrEquals".to_string(),
        BinaryOperatorSymbol::And => "&&".to_string(),
        BinaryOperatorSymbol::Or => "||".to_string(),
        BinaryOperatorSymbol::Concatenate => "+".to_string(),
        BinaryOperatorSymbol::MethodLookup | BinaryOperatorSymbol::FieldLookup => ".".to_string(),
        BinaryOperatorSymbol::FunctionApplication => String::new(),
    }
}

const fn get_format(operator: &BinaryOperatorSymbol) -> OperatorFormat {
    match operator {
        BinaryOperatorSymbol::Add
        | BinaryOperatorSymbol::Subtract
        | BinaryOperatorSymbol::Multiply
        | BinaryOperatorSymbol::Divide
        | BinaryOperatorSymbol::Power
        | BinaryOperatorSymbol::Modulus
        | BinaryOperatorSymbol::EqualTo
        | BinaryOperatorSymbol::NotEqualTo
        | BinaryOperatorSymbol::LessThan
        | BinaryOperatorSymbol::LessThanOrEqualTo
        | BinaryOperatorSymbol::GreaterThan
        | BinaryOperatorSymbol::GreaterThanOrEqualTo => OperatorFormat::Method,
        BinaryOperatorSymbol::Concatenate
        | BinaryOperatorSymbol::And
        | BinaryOperatorSymbol::Or
        | BinaryOperatorSymbol::FunctionApplication => OperatorFormat::Parenthesized,
        BinaryOperatorSymbol::MethodLookup | BinaryOperatorSymbol::FieldLookup => {
            OperatorFormat::Naked
        }
    }
}

fn maybe_parenthesize_left(string: &str, expression: &ConcreteExpression) -> String {
    match expression {
        ConcreteExpression::Integer(_) | ConcreteExpression::UnaryOperator(_) => {
            format!("({string})")
        }
        _ => string.to_string(),
    }
}

pub fn print_binary_operator(expression: &ConcreteBinaryOperatorExpression) -> String {
    let operator = print_operator(&expression.symbol);
    let left = super::print_expression(&expression.left_child);
    let right = {
        let right_child_text = super::print_expression(&expression.right_child);
        match &expression.symbol {
            BinaryOperatorSymbol::MethodLookup | BinaryOperatorSymbol::FieldLookup => {
                right_child_text[1..].to_owned()
            }
            _ => right_child_text,
        }
    };
    match get_format(&expression.symbol) {
        OperatorFormat::Naked => format!(
            "{}{operator}{right}",
            maybe_parenthesize_left(&left, &expression.left_child)
        ),
        OperatorFormat::Parenthesized => format!("({left}{operator}{right})"),
        OperatorFormat::Method => {
            format!(
                "{}.{}({})",
                maybe_parenthesize_left(&left, &expression.left_child),
                operator,
                right
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use typed_ast::{ConcreteExpression, ConcreteType};

    #[test]
    fn addition() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Add,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).add(2)");
    }

    #[test]
    fn addition_with_unary_operator_has_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Add,
            left_child: ConcreteExpression::negative_unary_operator_for_test(
                ConcreteExpression::integer_for_test(1),
            ),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(-1).add(2)");
    }

    #[test]
    fn addition_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Add,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.add(Bbar)");
    }

    #[test]
    fn concatenate() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Concatenate,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1+2)");
    }

    #[test]
    fn subtraction() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Subtract,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).subtract(2)");
    }

    #[test]
    fn subtraction_with_unary_operator_has_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Subtract,
            left_child: ConcreteExpression::negative_unary_operator_for_test(
                ConcreteExpression::integer_for_test(1),
            ),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(-1).subtract(2)");
    }

    #[test]
    fn subtraction_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Subtract,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.subtract(Bbar)");
    }

    #[test]
    fn multiplication() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Multiply,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).multiply(2)");
    }

    #[test]
    fn multiplication_with_unary_operator_has_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Multiply,
            left_child: ConcreteExpression::negative_unary_operator_for_test(
                ConcreteExpression::integer_for_test(1),
            ),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(-1).multiply(2)");
    }

    #[test]
    fn multiplication_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Multiply,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.multiply(Bbar)");
    }

    #[test]
    fn division() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Divide,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).divide(2)");
    }

    #[test]
    fn division_with_unary_operator_has_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Divide,
            left_child: ConcreteExpression::negative_unary_operator_for_test(
                ConcreteExpression::integer_for_test(1),
            ),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(-1).divide(2)");
    }

    #[test]
    fn division_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Divide,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.divide(Bbar)");
    }

    #[test]
    fn power() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Power,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).power(2)");
    }

    #[test]
    fn power_with_unary_operator_has_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Power,
            left_child: ConcreteExpression::negative_unary_operator_for_test(
                ConcreteExpression::integer_for_test(1),
            ),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(-1).power(2)");
    }

    #[test]
    fn power_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Power,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.power(Bbar)");
    }

    #[test]
    fn modulus() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Modulus,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).modulo(2)");
    }

    #[test]
    fn modulus_with_unary_operator_has_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Modulus,
            left_child: ConcreteExpression::negative_unary_operator_for_test(
                ConcreteExpression::integer_for_test(1),
            ),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(-1).modulo(2)");
    }

    #[test]
    fn modulus_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Modulus,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.modulo(Bbar)");
    }

    #[test]
    fn equal_to() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::EqualTo,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).equals(2)");
    }

    #[test]
    fn equal_to_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::EqualTo,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.equals(Bbar)");
    }

    #[test]
    fn not_equal_to() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::NotEqualTo,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).notEquals(2)");
    }

    #[test]
    fn not_equal_to_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::NotEqualTo,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.notEquals(Bbar)");
    }

    #[test]
    fn less_than() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::LessThan,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).lessThan(2)");
    }

    #[test]
    fn less_than_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::LessThan,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.lessThan(Bbar)");
    }

    #[test]
    fn less_than_or_equal_to() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::LessThanOrEqualTo,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(
            print_binary_operator(&expression),
            "(1).lessThanOrEquals(2)"
        );
    }

    #[test]
    fn less_than_or_equal_to_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::LessThanOrEqualTo,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(
            print_binary_operator(&expression),
            "Bfoo.lessThanOrEquals(Bbar)"
        );
    }

    #[test]
    fn greater_than() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::GreaterThan,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(print_binary_operator(&expression), "(1).greaterThan(2)");
    }

    #[test]
    fn greater_than_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::GreaterThan,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.greaterThan(Bbar)");
    }

    #[test]
    fn greater_than_or_equal_to() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::GreaterThanOrEqualTo,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::integer_for_test(2),
        };
        assert_eq!(
            print_binary_operator(&expression),
            "(1).greaterThanOrEquals(2)"
        );
    }

    #[test]
    fn greater_than_or_equal_to_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::GreaterThanOrEqualTo,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(
            print_binary_operator(&expression),
            "Bfoo.greaterThanOrEquals(Bbar)"
        );
    }

    #[test]
    fn and() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::And,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "(Bfoo&&Bbar)");
    }

    #[test]
    fn or() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Or,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "(Bfoo||Bbar)");
    }

    #[test]
    fn method_lookup() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::MethodLookup,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.bar");
    }

    #[test]
    fn wrap_left_in_parenthesis_if_left_is_integer_literal_in_method_lookup() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::MethodLookup,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::identifier_for_test("foo"),
        };
        assert_eq!(print_binary_operator(&expression), "(1).foo");
    }

    #[test]
    fn field_lookup() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::FieldLookup,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "Bfoo.bar");
    }

    #[test]
    fn wrap_left_in_parenthesis_if_left_is_integer_literal_in_field_lookup() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::FieldLookup,
            left_child: ConcreteExpression::integer_for_test(1),
            right_child: ConcreteExpression::identifier_for_test("foo"),
        };
        assert_eq!(print_binary_operator(&expression), "(1).foo");
    }
}
