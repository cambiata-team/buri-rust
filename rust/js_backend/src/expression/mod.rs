mod binary_operator;
mod block;
mod boolean;
mod declaration;
mod function_arguments;
mod function_declaration;
mod if_expression;
mod list;
mod record;
mod record_assignment;
mod tag;
mod unary_operator;
mod variable_name_mangling;
mod when;

use crate::{
    identifier::print_identifier,
    literals::{print_integer_literal, print_string_literal},
};
use typed_ast::ConcreteExpression;

pub use declaration::print_declaration;
pub use variable_name_mangling::mangle_variable_name;

use self::when::print_when;

fn print_expression(expression: &ConcreteExpression) -> String {
    match expression {
        ConcreteExpression::Identifier(identifier) => print_identifier(identifier),
        ConcreteExpression::Integer(integer) => print_integer_literal(integer),
        ConcreteExpression::StringLiteral(string) => print_string_literal(string),
        ConcreteExpression::Record(record) => record::print_record(record),
        ConcreteExpression::RecordAssignment(assignment) => {
            record_assignment::print_record_assignment(assignment)
        }
        ConcreteExpression::List(list) => list::print_list(list),
        ConcreteExpression::BinaryOperator(operator) => {
            binary_operator::print_binary_operator(operator)
        }
        ConcreteExpression::UnaryOperator(operator) => {
            unary_operator::print_unary_operator(operator)
        }
        ConcreteExpression::Tag(tag) => tag::print_tag(tag),
        ConcreteExpression::If(if_expression) => if_expression::print_if_expression(if_expression),
        ConcreteExpression::Block(block) => block::print_block(block),
        ConcreteExpression::Function(function) => {
            function_declaration::print_function_declaration(function)
        }
        ConcreteExpression::Boolean(boolean) => boolean::print_boolean(boolean),
        ConcreteExpression::Declaration(declaration) => declaration::print_declaration(declaration),
        ConcreteExpression::FunctionArguments(arguments) => {
            function_arguments::print_function_arguments(arguments)
        }
        ConcreteExpression::TypeDeclaration(_) | ConcreteExpression::TypeIdentifier(_) => {
            String::new()
        }
        ConcreteExpression::When(when) => print_when(when),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::{BinaryOperatorSymbol, UnaryOperatorSymbol};
    use std::collections::HashMap;
    use typed_ast::{
        ConcreteBinaryOperatorExpression, ConcreteBlockExpression, ConcreteBooleanExpression,
        ConcreteDeclarationExpression, ConcreteFunctionExpression, ConcreteIfExpression,
        ConcreteListExpression, ConcreteRecordAssignmentExpression, ConcreteRecordExpression,
        ConcreteStringLiteralExpression, ConcreteTagExpression, ConcreteType,
        ConcreteUnaryOperatorExpression, PrimitiveType,
    };

    #[test]
    fn can_print_identifier() {
        let expression = ConcreteExpression::identifier_for_test("foo");
        assert_eq!(print_expression(&expression), "Bfoo");
    }

    #[test]
    fn can_print_integer_literal() {
        let expression = ConcreteExpression::integer_for_test(42);
        assert_eq!(print_expression(&expression), "42");
    }

    #[test]
    fn can_print_string_literal() {
        let expression = ConcreteExpression::string_for_test("foo");
        assert_eq!(print_expression(&expression), "\"foo\"");
    }

    #[test]
    fn can_print_record() {
        let expression = ConcreteExpression::Record(Box::new(ConcreteRecordExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: HashMap::from([
                ("foo".to_string(), ConcreteExpression::integer_for_test(42)),
                (
                    "bar".to_string(),
                    ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
                        expression_type: ConcreteType::default_string_for_test(),
                        value: "baz".to_string(),
                    })),
                ),
            ]),
        }));
        // Because of the HashMap, the order of the keys is not guaranteed.
        // However, the order doesn't matter so we can accept either one.
        assert!(
            print_expression(&expression) == "{bar: \"baz\", foo: 42}"
                || print_expression(&expression) == "{foo: 42, bar: \"baz\"}"
        );
    }

    #[test]
    fn can_print_list() {
        let list = ConcreteExpression::List(Box::new(ConcreteListExpression {
            expression_type: ConcreteType::default_list_for_test(),
            contents: vec![ConcreteExpression::integer_for_test(42)],
        }));
        assert_eq!(print_expression(&list), "[42]");
    }

    #[test]
    fn print_binary_operator() {
        let expression =
            ConcreteExpression::BinaryOperator(Box::new(ConcreteBinaryOperatorExpression {
                expression_type: ConcreteType::default_binary_operator_for_test(),
                symbol: BinaryOperatorSymbol::FieldLookup,
                left_child: ConcreteExpression::identifier_for_test("foo"),
                right_child: ConcreteExpression::identifier_for_test("bar"),
            }));
        assert_eq!(print_expression(&expression), "Bfoo.bar");
    }

    #[test]
    fn can_print_unary_operator() {
        let expression =
            ConcreteExpression::UnaryOperator(Box::new(ConcreteUnaryOperatorExpression {
                expression_type: ConcreteType::default_integer_for_test(),
                symbol: UnaryOperatorSymbol::Negative,
                child: ConcreteExpression::integer_for_test(42),
            }));
        assert_eq!(print_expression(&expression), "-42");
    }

    #[test]
    fn print_tag() {
        let tag = ConcreteExpression::Tag(Box::new(ConcreteTagExpression {
            name: "foo".to_string(),
            expression_type: ConcreteType::default_tag_union_for_test(),
            contents: vec![],
        }));
        assert_eq!(print_expression(&tag), "[\"foo\"]");
    }

    #[test]
    fn print_if_expression() {
        let expression = ConcreteExpression::If(Box::new(ConcreteIfExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            condition: ConcreteExpression::identifier_for_test("foo"),
            path_if_true: ConcreteExpression::identifier_for_test("bar"),
            path_if_false: Some(ConcreteExpression::identifier_for_test("baz")),
        }));
        assert_eq!(print_expression(&expression), "(Bfoo?Bbar:Bbaz)");
    }

    #[test]
    fn print_block() {
        let block = ConcreteExpression::Block(Box::new(ConcreteBlockExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            contents: vec![ConcreteExpression::integer_for_test(42)],
        }));
        assert_eq!(print_expression(&block), "42");
    }

    #[test]
    fn print_function_declaration() {
        let function = ConcreteExpression::Function(Box::new(ConcreteFunctionExpression {
            expression_type: ConcreteType::default_function_for_test(),
            argument_names: vec![],
            body: ConcreteExpression::integer_for_test(42),
        }));
        assert_eq!(print_expression(&function), "()=>(42)");
    }

    #[test]
    fn print_boolean() {
        let boolean = ConcreteExpression::Boolean(Box::new(ConcreteBooleanExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::CompilerBoolean),
            value: true,
        }));
        assert_eq!(print_expression(&boolean), "true");
    }

    #[test]
    fn print_declaration() {
        let declaration =
            ConcreteExpression::Declaration(Box::new(ConcreteDeclarationExpression {
                declaration_type: ConcreteType::default_integer_for_test(),
                expression_type: ConcreteType::default_integer_for_test(),
                identifier: ConcreteExpression::raw_identifier_for_test("foo"),
                value: ConcreteExpression::integer_for_test(42),
            }));
        assert_eq!(print_expression(&declaration), "const Bfoo=42");
    }

    #[test]
    fn print_record_assignment() {
        let record = ConcreteRecordExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: HashMap::from([(
                "meaningOfLife".to_string(),
                ConcreteExpression::integer_for_test(42),
            )]),
        };
        let identifier = ConcreteExpression::raw_identifier_for_test("hello");
        let assignment =
            ConcreteExpression::RecordAssignment(Box::new(ConcreteRecordAssignmentExpression {
                expression_type: ConcreteType::default_record_for_test(),
                contents: record,
                identifier,
            }));
        assert_eq!(
            print_expression(&assignment),
            "Bhello.$set({meaningOfLife: 42})"
        );
    }
}
