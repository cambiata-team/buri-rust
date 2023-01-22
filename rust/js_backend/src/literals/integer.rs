use typed_ast::ConcreteIntegerLiteralExpression;

pub fn print_integer_literal(node: &ConcreteIntegerLiteralExpression) -> String {
    let mut result = String::new();
    result.push_str(&node.value.to_string());
    result
}

#[cfg(test)]
mod test {
    use super::*;

    use typed_ast::{ConcreteType, PrimitiveType};

    #[test]
    fn test_print_integer_literal() {
        let node = ConcreteIntegerLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Num),
            value: 1,
        };
        assert_eq!(print_integer_literal(&node), "1");
    }

    #[test]
    fn test_print_integer_literal_2() {
        let node = ConcreteIntegerLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Num),
            value: 2,
        };
        assert_eq!(print_integer_literal(&node), "2");
    }
}
