use crate::mangle_variable_name;
use typed_ast::ConcreteIdentifierExpression;

pub fn print_identifier(node: &ConcreteIdentifierExpression) -> String {
    mangle_variable_name(&node.name)
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
            is_disregarded: false,
        };
        assert_eq!(print_identifier(&node), "Bfoo");
    }
}
