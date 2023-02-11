use crate::{expression::print_expression, identifier::print_identifier};
use typed_ast::ConcreteDeclarationExpression;

pub fn print_declaration(declaration: &ConcreteDeclarationExpression) -> String {
    let identifier = print_identifier(&declaration.identifier);
    let value = print_expression(&declaration.value);
    format!("const {identifier}={value}")
}

#[cfg(test)]
mod test {
    use typed_ast::{ConcreteExpression, ConcreteType};

    use super::*;

    #[test]
    fn declare_an_integer() {
        let declaration = ConcreteDeclarationExpression {
            declaration_type: ConcreteType::default_integer_for_test(),
            expression_type: ConcreteType::default_integer_for_test(),
            identifier: ConcreteExpression::raw_identifier_for_test("foo"),
            value: ConcreteExpression::integer_for_test(42),
        };
        assert_eq!(print_declaration(&declaration), "const foo=42");
    }

    #[test]
    fn declare_a_string() {
        let declaration = ConcreteDeclarationExpression {
            declaration_type: ConcreteType::default_integer_for_test(),
            expression_type: ConcreteType::default_string_for_test(),
            identifier: ConcreteExpression::raw_identifier_for_test("hello"),
            value: ConcreteExpression::string_for_test("world"),
        };
        assert_eq!(print_declaration(&declaration), "const hello=\"world\"");
    }
}
