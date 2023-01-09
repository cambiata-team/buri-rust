use crate::{
    intra_expression_whitespace::intra_expression_whitespace, type_expression::type_expression,
    ExpressionContext,
};
use ast::ListTypeNode;
use ast::{IResult, ParserInput};
use nom::{
    character::complete::char,
    combinator::{consumed, map, opt},
    sequence::delimited,
};

pub fn list_type(input: ParserInput) -> IResult<ListTypeNode> {
    map(
        consumed(delimited(
            char('['),
            delimited(
                opt(intra_expression_whitespace(
                    ExpressionContext::new().allow_newlines_in_expressions(),
                )),
                type_expression,
                opt(intra_expression_whitespace(
                    ExpressionContext::new().allow_newlines_in_expressions(),
                )),
            ),
            char(']'),
        )),
        |(source, expression)| ListTypeNode {
            source,
            value: expression,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::TypeExpression;

    #[test]
    fn empty_string_is_not_a_list() {
        let input = ParserInput::new("[Str]");
        let (_, parsed) = list_type(input).unwrap();
        assert!(matches!(parsed.value, TypeExpression::Identifier(_)));
    }

    #[test]
    fn can_include_space_before_inner_expression() {
        let input = ParserInput::new("[ Str]");
        let result = list_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_include_space_after_inner_expression() {
        let input = ParserInput::new("[Str ]");
        let result = list_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_include_newline_before_inner_expression() {
        let input = ParserInput::new("[\nStr]");
        let result = list_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_include_newline_after_inner_expression() {
        let input = ParserInput::new("[Str\n]");
        let result = list_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn missing_opening_bracket_errors() {
        let input = ParserInput::new("Str]");
        let result = list_type(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_ending_bracket_errors() {
        let input = ParserInput::new("[Str");
        let result = list_type(input);
        assert!(result.is_err());
    }
}
