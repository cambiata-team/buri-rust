use crate::{
    expression, intra_expression_whitespace::intra_expression_whitespace, ExpressionContext,
};
use ast::Expression;
use ast::{IResult, ParserInput};
use nom::{
    character::complete::char,
    combinator::opt,
    sequence::{delimited, tuple},
};

pub fn parentheses(input: ParserInput) -> IResult<Expression> {
    delimited(
        tuple((
            char('('),
            opt(intra_expression_whitespace(
                ExpressionContext::new().allow_newlines_in_expressions(),
            )),
        )),
        expression(ExpressionContext::new().allow_newlines_in_expressions()),
        tuple((
            opt(intra_expression_whitespace(
                ExpressionContext::new().allow_newlines_in_expressions(),
            )),
            char(')'),
        )),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_input_errors() {
        let input = ParserInput::new("");
        let result = parentheses(input);
        assert!(result.is_err());
    }

    #[test]
    fn unclosed_open_parenthesis_errors() {
        let input = ParserInput::new("(");
        let result = parentheses(input);
        assert!(result.is_err());
    }

    #[test]
    fn empty_paired_parentheses_errors() {
        let input = ParserInput::new("()");
        let result = parentheses(input);
        assert!(result.is_err());
    }

    #[test]
    fn basic_expression_enclosed_by_parentheses_returns_that_expression() {
        let input = ParserInput::new("(0)");
        let result = parentheses(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(parsed, Expression::Integer(_)));
    }

    #[test]
    fn missing_closing_parenthesis_after_expression_errors() {
        let input = ParserInput::new("(0");
        let result = parentheses(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_opening_parenthesis_before_expression_errors() {
        let input = ParserInput::new("0)");
        let result = parentheses(input);
        assert!(result.is_err());
    }

    #[test]
    fn spaces_can_be_added_around_expression() {
        let input = ParserInput::new("(  0  )");
        let result = parentheses(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn newlines_can_be_added_around_expression() {
        let input = ParserInput::new("(\n\n0\n\n)");
        let result = parentheses(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn binary_operator_expression_enclosed_by_parentheses_returns_that_expression() {
        let input = ParserInput::new("(1+2)");
        let result = parentheses(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(parsed, Expression::BinaryOperator(_)));
    }

    #[test]
    fn binary_operator_expression_may_be_split_across_multiple_lines_and_indented_as_a_block() {
        let input = ParserInput::new("(\n    1\n    +\n    2\n)");
        let result = parentheses(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn parentheses_can_contain_if_statement() {
        let input = ParserInput::new("(if 1 == 2 do 3)");
        let result = parentheses(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }
}
