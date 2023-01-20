use super::print_expression;
use concrete_ast::ConcreteRecordExpression;

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
    use super::*;
    use concrete_ast::{
        ConcreteExpression, ConcreteIntegerLiteralExpression, ConcreteStringLiteralExpression,
    };

    #[test]
    fn can_print_record() {
        let record = ConcreteRecordExpression {
            contents: vec![
                (
                    "foo".to_string(),
                    ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                        value: 42,
                    })),
                ),
                (
                    "bar".to_string(),
                    ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
                        value: "baz".to_string(),
                    })),
                ),
            ],
        };
        assert_eq!(print_record(&record), "{foo: 42, bar: \"baz\"}");
    }

    #[test]
    fn does_not_include_comma_with_one_item() {
        let record = ConcreteRecordExpression {
            contents: vec![(
                "foo".to_string(),
                ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                    value: 42,
                })),
            )],
        };
        assert_eq!(print_record(&record), "{foo: 42}");
    }
}
