use concrete_ast::ConcreteIfExpression;

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

pub fn print_if(expression: &ConcreteIfExpression) -> String {
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
    use concrete_ast::{ConcreteExpression, ConcreteIdentifierExpression, ConcreteIfExpression};

    use super::*;

    #[test]
    fn prints_if_with_all_paths() {
        let expression = ConcreteIfExpression {
            condition: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "foo".to_string(),
            })),
            path_if_true: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "bar".to_string(),
            })),
            path_if_false: Some(ConcreteExpression::Identifier(Box::new(
                ConcreteIdentifierExpression {
                    name: "baz".to_string(),
                },
            ))),
        };
        assert_eq!(print_if(&expression), "(foo?bar:baz)");
    }

    #[test]
    fn prints_if_with_only_true_path() {
        let expression = ConcreteIfExpression {
            condition: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "foo".to_string(),
            })),
            path_if_true: ConcreteExpression::Identifier(Box::new(ConcreteIdentifierExpression {
                name: "bar".to_string(),
            })),
            path_if_false: None,
        };
        assert_eq!(print_if(&expression), "(foo?[\"some\",bar]:[\"none\"])");
    }
}
