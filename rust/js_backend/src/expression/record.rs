use super::print_expression;
use typed_ast::ConcreteRecordExpression;

pub fn print_record(record: &ConcreteRecordExpression) -> String {
    let mut result = String::new();
    result.push('{');
    for (index, (key, value)) in record.contents.iter().enumerate() {
        result.push_str(key);
        result.push_str(": ");
        result.push_str(&print_expression(value));
        if index < record.contents.len() - 1 {
            result.push_str(", ");
        }
    }
    result.push('}');
    result
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use typed_ast::{
        ConcreteExpression, ConcreteIntegerLiteralExpression, ConcreteStringLiteralExpression,
        ConcreteType,
    };

    #[test]
    fn can_print_record() {
        let record = ConcreteRecordExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: HashMap::from([
                (
                    "foo".to_string(),
                    ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                        expression_type: ConcreteType::default_integer_for_test(),
                        value: 42,
                    })),
                ),
                (
                    "bar".to_string(),
                    ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
                        expression_type: ConcreteType::default_string_for_test(),
                        value: "baz".to_string(),
                    })),
                ),
            ]),
        };
        // Because of the HashMap, the order of the keys is not guaranteed.
        // However, the order doesn't matter so we can accept either one.
        assert!(
            print_record(&record) == "{bar: \"baz\", foo: 42}"
                || print_record(&record) == "{foo: 42, bar: \"baz\"}"
        );
    }

    #[test]
    fn does_not_include_comma_with_one_item() {
        let record = ConcreteRecordExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: HashMap::from([(
                "foo".to_string(),
                ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                    expression_type: ConcreteType::default_integer_for_test(),
                    value: 42,
                })),
            )]),
        };
        assert_eq!(print_record(&record), "{foo: 42}");
    }
}
