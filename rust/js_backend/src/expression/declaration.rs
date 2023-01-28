use crate::{expression::print_expression, identifier::print_identifier};
use typed_ast::ConcreteDeclarationExpression;

pub fn print_declaration(declaration: &ConcreteDeclarationExpression) -> String {
    let export = if declaration.is_exported {
        "export "
    } else {
        ""
    };
    let identifier = print_identifier(&declaration.identifier);
    let value = print_expression(&declaration.value);
    format!("{export}const {identifier}={value}")
}

#[cfg(test)]
mod test {
    use typed_ast::{ConcreteExpression, ConcreteType};

    use super::*;

    #[test]
    fn declare_an_integer() {
        let declaration = ConcreteDeclarationExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            identifier: ConcreteExpression::raw_identifier_for_test("foo"),
            value: ConcreteExpression::integer_for_test(42),
            is_exported: false,
        };
        assert_eq!(print_declaration(&declaration), "const foo=42");
    }

    #[test]
    fn declare_a_string() {
        let declaration = ConcreteDeclarationExpression {
            expression_type: ConcreteType::default_string_for_test(),
            identifier: ConcreteExpression::raw_identifier_for_test("hello"),
            value: ConcreteExpression::string_for_test("world"),
            is_exported: false,
        };
        assert_eq!(print_declaration(&declaration), "const hello=\"world\"");
    }

    #[test]
    fn can_export_declarations() {
        let declaration = ConcreteDeclarationExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            identifier: ConcreteExpression::raw_identifier_for_test("foo"),
            value: ConcreteExpression::integer_for_test(42),
            is_exported: true,
        };
        assert_eq!(print_declaration(&declaration), "export const foo=42");
    }
}
