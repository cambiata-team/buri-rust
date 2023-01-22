use concrete_ast::ConcreteFunctionExpression;

pub fn print_function_declaration(function: &ConcreteFunctionExpression) -> String {
    let mut result = String::new();
    result.push('(');
    for (index, parameter) in function.argument_names.iter().enumerate() {
        if index > 0 {
            result.push(',');
        }
        result.push_str(parameter.as_str());
    }
    result.push_str(")=>(");
    result.push_str(super::print_expression(&function.body).as_str());
    result.push(')');
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use concrete_ast::{ConcreteExpression, ConcreteIntegerLiteralExpression};

    #[test]
    fn prints_a_function_with_no_arguments() {
        let function = ConcreteFunctionExpression {
            argument_names: vec![],
            body: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 42,
            })),
        };
        assert_eq!(print_function_declaration(&function), "()=>(42)");
    }

    #[test]
    fn prints_a_function_with_one_argument() {
        let function = ConcreteFunctionExpression {
            argument_names: vec!["x".to_string()],
            body: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 42,
            })),
        };
        assert_eq!(print_function_declaration(&function), "(x)=>(42)");
    }

    #[test]
    fn prints_a_function_with_two_arguments() {
        let function = ConcreteFunctionExpression {
            argument_names: vec!["x".to_string(), "y".to_string()],
            body: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 42,
            })),
        };
        assert_eq!(print_function_declaration(&function), "(x,y)=>(42)");
    }

    #[test]
    fn prints_a_function_with_three_arguments() {
        let function = ConcreteFunctionExpression {
            argument_names: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            body: ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                value: 42,
            })),
        };
        assert_eq!(print_function_declaration(&function), "(x,y,z)=>(42)");
    }
}
