use concrete_ast::ConcreteIdentifierExpression;

pub fn print_identifier(node: &ConcreteIdentifierExpression) -> String {
    node.name.clone()
}

#[cfg(test)]
mod test {
    use super::*;
    use concrete_ast::ConcreteIdentifierExpression;

    #[test]
    fn test_print_identifier() {
        let node = ConcreteIdentifierExpression {
            name: "foo".to_string(),
        };
        assert_eq!(print_identifier(&node), "foo");
    }
}
