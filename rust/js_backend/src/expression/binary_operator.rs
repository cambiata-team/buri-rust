use ast::BinaryOperatorSymbol;
use concrete_ast::ConcreteBinaryOperatorExpression;

fn print_operator(operator: &BinaryOperatorSymbol) -> String {
    match operator {
        BinaryOperatorSymbol::Add | BinaryOperatorSymbol::Concatenate => "+".to_string(),
        BinaryOperatorSymbol::Subtract => "-".to_string(),
        BinaryOperatorSymbol::Multiply => "*".to_string(),
        BinaryOperatorSymbol::Divide => "/".to_string(),
        BinaryOperatorSymbol::Power => "**".to_string(),
        BinaryOperatorSymbol::Modulus => "%".to_string(),
        BinaryOperatorSymbol::EqualTo => "==".to_string(),
        BinaryOperatorSymbol::NotEqualTo => "!=".to_string(),
        BinaryOperatorSymbol::LessThan => "<".to_string(),
        BinaryOperatorSymbol::LessThanOrEqualTo => "<=".to_string(),
        BinaryOperatorSymbol::GreaterThan => ">".to_string(),
        BinaryOperatorSymbol::GreaterThanOrEqualTo => ">=".to_string(),
        BinaryOperatorSymbol::And => "&&".to_string(),
        BinaryOperatorSymbol::Or => "||".to_string(),
        BinaryOperatorSymbol::MethodLookup | BinaryOperatorSymbol::FieldLookup => ".".to_string(),
        BinaryOperatorSymbol::FunctionApplication => unreachable!(),
    }
}

fn should_parenthesize(operator: &BinaryOperatorSymbol) -> bool {
    match operator {
        BinaryOperatorSymbol::Add
        | BinaryOperatorSymbol::Concatenate
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
        | BinaryOperatorSymbol::GreaterThanOrEqualTo
        | BinaryOperatorSymbol::And
        | BinaryOperatorSymbol::Or => true,
        BinaryOperatorSymbol::MethodLookup | BinaryOperatorSymbol::FieldLookup => false,
        BinaryOperatorSymbol::FunctionApplication => unreachable!(),
    }
}

pub fn print_binary_operator(expression: &ConcreteBinaryOperatorExpression) -> String {
    let operator = print_operator(&expression.symbol);
    let left = super::print_expression(&expression.left_child);
    let right = super::print_expression(&expression.right_child);
    if should_parenthesize(&expression.symbol) {
        format!("({left}{operator}{right})")
    } else {
        format!("{left}{operator}{right}")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use concrete_ast::{
        ConcreteExpression, ConcreteIdentifierExpression, ConcreteIntegerLiteralExpression,
    };

    #[test]
    fn addition() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::Add,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1+2)");
    }

    #[test]
    fn concatenate() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::Concatenate,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1+2)");
    }

    #[test]
    fn subtraction() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::Subtract,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1-2)");
    }

    #[test]
    fn multiplication() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::Multiply,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1*2)");
    }

    #[test]
    fn division() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::Divide,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1/2)");
    }

    #[test]
    fn power() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::Power,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1**2)");
    }

    #[test]
    fn modulus() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::Modulus,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1%2)");
    }

    #[test]
    fn equal_to() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::EqualTo,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1==2)");
    }

    #[test]
    fn not_equal_to() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::NotEqualTo,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1!=2)");
    }

    #[test]
    fn less_than() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::LessThan,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1<2)");
    }

    #[test]
    fn less_than_or_equal_to() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::LessThanOrEqualTo,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1<=2)");
    }

    #[test]
    fn greater_than() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::GreaterThan,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1>2)");
    }

    #[test]
    fn greater_than_or_equal_to() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::GreaterThanOrEqualTo,
            left_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 1,
            })),
            right_child: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 2,
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(1>=2)");
    }

    #[test]
    fn and() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::And,
            left_child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "foo".to_string(),
            })),
            right_child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "bar".to_string(),
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(foo&&bar)");
    }

    #[test]
    fn or() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::Or,
            left_child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "foo".to_string(),
            })),
            right_child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "bar".to_string(),
            })),
        };
        assert_eq!(print_binary_operator(&expression), "(foo||bar)");
    }

    #[test]
    fn method_lookup() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::MethodLookup,
            left_child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "foo".to_string(),
            })),
            right_child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "bar".to_string(),
            })),
        };
        assert_eq!(print_binary_operator(&expression), "foo.bar");
    }

    #[test]
    fn field_lookup() {
        let expression = ConcreteBinaryOperatorExpression {
            symbol: BinaryOperatorSymbol::FieldLookup,
            left_child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "foo".to_string(),
            })),
            right_child: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "bar".to_string(),
            })),
        };
        assert_eq!(print_binary_operator(&expression), "foo.bar");
    }
}
