use super::print_expression;
use typed_ast::ConcreteListExpression;

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
    use typed_ast::{ConcreteExpression, ConcreteStringLiteralExpression, ConcreteType};

    use super::*;

    #[test]
    fn can_print_list_of_integers() {
        let list = ConcreteListExpression {
            expression_type: ConcreteType::default_list_for_test(),
            contents: vec![
                ConcreteExpression::integer_for_test(42),
                ConcreteExpression::integer_for_test(43),
            ],
        };
        assert_eq!(print_list(&list), "[(42),(43)]");
    }

    #[test]
    fn does_not_include_comma_with_one_item() {
        let list = ConcreteListExpression {
            expression_type: ConcreteType::default_list_for_test(),
            contents: vec![ConcreteExpression::integer_for_test(42)],
        };
        assert_eq!(print_list(&list), "[(42)]");
    }

    #[test]
    fn can_print_list_of_strings() {
        let list = ConcreteListExpression {
            expression_type: ConcreteType::default_list_for_test(),
            contents: vec![
                ConcreteExpression::string_for_test("foo"),
                ConcreteExpression::string_for_test("bar"),
            ],
        };
        assert_eq!(print_list(&list), "[\"foo\",\"bar\"]");
    }
}
