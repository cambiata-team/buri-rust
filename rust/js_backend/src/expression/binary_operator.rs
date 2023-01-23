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
        BinaryOperatorSymbol::FunctionApplication => unreachable!(),
    }
}

fn get_format(operator: &BinaryOperatorSymbol) -> OperatorFormat {
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
        | BinaryOperatorSymbol::Or => OperatorFormat::Parenthesized,
        BinaryOperatorSymbol::MethodLookup | BinaryOperatorSymbol::FieldLookup => {
            OperatorFormat::Naked
        }
        BinaryOperatorSymbol::FunctionApplication => unreachable!(),
    }
}

fn maybe_parenthesize_left(string: &str, expression: &ConcreteExpression) -> String {
    if let ConcreteExpression::Integer(_) = expression {
        format!("({string})")
    } else {
        super::print_expression(expression)
    }
}

pub fn print_binary_operator(expression: &ConcreteBinaryOperatorExpression) -> String {
    let operator = print_operator(&expression.symbol);
    let left = super::print_expression(&expression.left_child);
    let right = super::print_expression(&expression.right_child);
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
    fn addition_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Add,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "foo.add(bar)");
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
    fn subtraction_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Subtract,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "foo.subtract(bar)");
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
    fn multiplication_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Multiply,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "foo.multiply(bar)");
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
    fn division_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Divide,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "foo.divide(bar)");
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
    fn power_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Power,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "foo.power(bar)");
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
    fn modulus_without_number_literals_do_not_have_parenthesis() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Modulus,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "foo.modulo(bar)");
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
        assert_eq!(print_binary_operator(&expression), "foo.equals(bar)");
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
        assert_eq!(print_binary_operator(&expression), "foo.notEquals(bar)");
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
        assert_eq!(print_binary_operator(&expression), "foo.lessThan(bar)");
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
            "foo.lessThanOrEquals(bar)"
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
        assert_eq!(print_binary_operator(&expression), "foo.greaterThan(bar)");
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
            "foo.greaterThanOrEquals(bar)"
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
        assert_eq!(print_binary_operator(&expression), "(foo&&bar)");
    }

    #[test]
    fn or() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::Or,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "(foo||bar)");
    }

    #[test]
    fn method_lookup() {
        let expression = ConcreteBinaryOperatorExpression {
            expression_type: ConcreteType::default_binary_operator_for_test(),
            symbol: BinaryOperatorSymbol::MethodLookup,
            left_child: ConcreteExpression::identifier_for_test("foo"),
            right_child: ConcreteExpression::identifier_for_test("bar"),
        };
        assert_eq!(print_binary_operator(&expression), "foo.bar");
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
        assert_eq!(print_binary_operator(&expression), "foo.bar");
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
