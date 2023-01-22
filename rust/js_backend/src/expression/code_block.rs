use concrete_ast::ConcreteBlockExpression;

pub fn print_code_block(block: &ConcreteBlockExpression) -> String {
    if block.contents.is_empty() {
        return String::new();
    }
    if block.contents.len() == 1 {
        return super::print_expression(&block.contents[0]);
    }
    let mut result = String::new();
    result.push_str("(()=>{");
    for (index, expression) in block.contents.iter().enumerate() {
        if index == &block.contents.len() - 1 {
            result.push_str("return ");
        }
        result.push_str(super::print_expression(expression).as_str());
        result.push(';');
    }
    result.push_str("})()");
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use concrete_ast::{ConcreteExpression, ConcreteIntegerLiteralExpression};

    #[test]
    fn an_empty_code_block_produces_an_empty_string() {
        let block = ConcreteBlockExpression { contents: vec![] };
        assert_eq!(print_code_block(&block), "");
    }

    #[test]
    fn a_code_block_with_one_expression_produces_the_expression() {
        let block = ConcreteBlockExpression {
            contents: vec![ConcreteExpression::Integer(Box::new(
                ConcreteIntegerLiteralExpression { value: 42 },
            ))],
        };
        assert_eq!(print_code_block(&block), "42");
    }

    #[test]
    fn a_code_block_with_two_or_more_expressions_produces_an_immediately_invoked_function() {
        let block = ConcreteBlockExpression {
            contents: vec![
                ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                    value: 42,
                })),
                ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
                    value: 43,
                })),
            ],
        };
        assert_eq!(print_code_block(&block), "(()=>{42;return 43;})()");
    }
}
