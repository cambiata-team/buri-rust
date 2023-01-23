use typed_ast::ConcreteIfExpression;

fn print_true_path(expression: &ConcreteIfExpression) -> String {
    let has_else = expression.path_if_false.is_some();
    let mut result = String::new();
    if !has_else {
        result.push_str("[\"some\",");
    }
    result.push_str(super::print_expression(&expression.path_if_true).as_str());
    if !has_else {
        result.push(']');
    }
    result
}

fn print_false_path(expression: &ConcreteIfExpression) -> String {
    expression
        .path_if_false
        .as_ref()
        .map_or_else(|| "[\"none\"]".to_string(), super::print_expression)
}

pub fn print_if_expression(expression: &ConcreteIfExpression) -> String {
    let mut result = String::new();
    result.push('(');
    result.push_str(super::print_expression(&expression.condition).as_str());
    result.push('?');
    result.push_str(print_true_path(expression).as_str());
    result.push(':');
    result.push_str(print_false_path(expression).as_str());
    result.push(')');
    result
}

#[cfg(test)]
mod test {
    use typed_ast::{ConcreteExpression, ConcreteIfExpression, ConcreteType};

    use super::*;

    #[test]
    fn prints_if_with_all_paths() {
        let expression = ConcreteIfExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            condition: ConcreteExpression::identifier_for_test("foo"),
            path_if_true: ConcreteExpression::identifier_for_test("bar"),
            path_if_false: Some(ConcreteExpression::identifier_for_test("baz")),
        };
        assert_eq!(print_if_expression(&expression), "(foo?bar:baz)");
    }

    #[test]
    fn prints_if_with_only_true_path() {
        let expression = ConcreteIfExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            condition: ConcreteExpression::identifier_for_test("foo"),
            path_if_true: ConcreteExpression::identifier_for_test("bar"),
            path_if_false: None,
        };
        assert_eq!(
            print_if_expression(&expression),
            "(foo?[\"some\",bar]:[\"none\"])"
        );
    }
}
