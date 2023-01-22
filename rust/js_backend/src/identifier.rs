use typed_ast::ConcreteIdentifierExpression;

pub fn print_identifier(node: &ConcreteIdentifierExpression) -> String {
    node.name.clone()
}

#[cfg(test)]
mod test {
    use super::*;
    use typed_ast::{ConcreteIdentifierExpression, ConcreteType, PrimitiveType};

    #[test]
    fn test_print_identifier() {
        let node = ConcreteIdentifierExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            name: "foo".to_string(),
        };
        assert_eq!(print_identifier(&node), "foo");
    }
}
