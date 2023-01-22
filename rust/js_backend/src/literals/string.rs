use typed_ast::ConcreteStringLiteralExpression;

pub fn print_string_literal(node: &ConcreteStringLiteralExpression) -> String {
    let mut result = String::new();
    result.push('\"');
    // Assumes all characters are correctly escaped, which is guaranteed
    // by the parser.
    result.push_str(&node.value);
    result.push('\"');
    result
}

#[cfg(test)]
mod test {
    use super::*;

    use typed_ast::{ConcreteType, PrimitiveType};

    #[test]
    fn simple_string_literal() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "hello".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"hello\"");
    }
}
