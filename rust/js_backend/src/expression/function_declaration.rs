use crate::mangle_variable_name;
use typed_ast::ConcreteFunctionExpression;

pub fn print_function_declaration(function: &ConcreteFunctionExpression) -> String {
    let mut result = String::new();
    result.push('(');
    for (index, parameter) in function.argument_names.iter().enumerate() {
        if index > 0 {
            result.push(',');
        }
        result.push_str(&mangle_variable_name(parameter.as_str()));
    }
    result.push_str(")=>(");
    result.push_str(super::print_expression(&function.body).as_str());
    result.push(')');
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use typed_ast::{ConcreteExpression, ConcreteType};

    #[test]
    fn prints_a_function_with_no_arguments() {
        let function = ConcreteFunctionExpression {
            expression_type: ConcreteType::default_function_for_test(),
            argument_names: vec![],
            body: ConcreteExpression::integer_for_test(42),
        };
        assert_eq!(print_function_declaration(&function), "()=>(42)");
    }

    #[test]
    fn prints_a_function_with_one_argument() {
        let function = ConcreteFunctionExpression {
            expression_type: ConcreteType::default_function_for_test(),
            argument_names: vec!["x".to_string()],
            body: ConcreteExpression::integer_for_test(42),
        };
        assert_eq!(print_function_declaration(&function), "(Bx)=>(42)");
    }

    #[test]
    fn prints_a_function_with_two_arguments() {
        let function = ConcreteFunctionExpression {
            expression_type: ConcreteType::default_function_for_test(),
            argument_names: vec!["x".to_string(), "y".to_string()],
            body: ConcreteExpression::integer_for_test(42),
        };
        assert_eq!(print_function_declaration(&function), "(Bx,By)=>(42)");
    }

    #[test]
    fn prints_a_function_with_three_arguments() {
        let function = ConcreteFunctionExpression {
            expression_type: ConcreteType::default_function_for_test(),
            argument_names: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            body: ConcreteExpression::integer_for_test(42),
        };
        assert_eq!(print_function_declaration(&function), "(Bx,By,Bz)=>(42)");
    }
}
