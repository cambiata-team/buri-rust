use super::print_expression;
use typed_ast::ConcreteExpression;

pub fn print_function_arguments(arguments: &Vec<ConcreteExpression>) -> String {
    let mut result = String::new();
    result.push('(');
    for (index, item) in arguments.iter().enumerate() {
        result.push_str(&print_expression(item));
        if index < arguments.len() - 1 {
            result.push(',');
        }
    }
    result.push(')');
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use typed_ast::ConcreteExpression;

    #[test]
    fn can_print_integer_arguments() {
        let arguments = vec![
            ConcreteExpression::integer_for_test(42),
            ConcreteExpression::integer_for_test(43),
        ];
        assert_eq!(print_function_arguments(&arguments), "(42,43)");
    }

    #[test]
    fn does_not_include_comma_with_one_item() {
        let arguments = vec![ConcreteExpression::integer_for_test(42)];
        assert_eq!(print_function_arguments(&arguments), "(42)");
    }

    #[test]
    fn can_print_string_arguments() {
        let arguments = vec![
            ConcreteExpression::string_for_test("foo"),
            ConcreteExpression::string_for_test("bar"),
        ];
        assert_eq!(print_function_arguments(&arguments), "(\"foo\",\"bar\")");
    }
}
