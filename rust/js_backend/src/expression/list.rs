use super::print_expression;
use concrete_ast::ConcreteListExpression;

pub fn print_list(list: &ConcreteListExpression) -> String {
    let mut result = String::new();
    result.push('[');
    for (index, item) in list.contents.iter().enumerate() {
        result.push_str(&print_expression(item));
        if index < list.contents.len() - 1 {
            result.push(',');
        }
    }
    result.push(']');
    result
}

#[cfg(test)]
mod test {
    use concrete_ast::{
        ConcreteExpression, ConcreteIntegerLiteralExpression, ConcreteStringLiteralExpression,
    };

    use super::*;

    #[test]
    fn can_print_list_of_integers() {
        let list = ConcreteListExpression {
            contents: vec![
                ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                    value: 42,
                })),
                ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                    value: 43,
                })),
            ],
        };
        assert_eq!(print_list(&list), "[42,43]");
    }

    #[test]
    fn does_not_include_comma_with_one_item() {
        let list = ConcreteListExpression {
            contents: vec![ConcreteExpression::Integer(Box::new(
                ConcreteIntegerLiteralExpression { value: 42 },
            ))],
        };
        assert_eq!(print_list(&list), "[42]");
    }

    #[test]
    fn can_print_list_of_strings() {
        let list = ConcreteListExpression {
            contents: vec![
                ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
                    value: "foo".to_string(),
                })),
                ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
                    value: "bar".to_string(),
                })),
            ],
        };
        assert_eq!(print_list(&list), "[\"foo\",\"bar\"]");
    }
}
