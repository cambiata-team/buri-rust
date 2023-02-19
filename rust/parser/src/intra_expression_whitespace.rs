use crate::{indent::indent_any, newline::newline, ExpressionContext};
use ast::{IResult, ParserInput};
use nom::{
    branch::alt,
    character::complete::space1,
    combinator::{opt, value},
    multi::many1_count,
    sequence::tuple,
};

fn multiple_newlines(input: ParserInput) -> IResult<usize> {
    many1_count(tuple((newline, indent_any)))(input)
}

pub fn intra_expression_whitespace<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, ()> {
    if context.allow_newlines_in_expressions {
        |input| {
            alt((
                value((), tuple((space1, opt(multiple_newlines)))),
                value((), multiple_newlines),
            ))(input)
        }
    } else {
        |input| value((), space1)(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string_errors() {
        let input = ParserInput::new("");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        assert!(result.is_err());
    }

    #[test]
    fn one_space_is_consumed() {
        let input = ParserInput::new(" ");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn two_spaces_are_consumed() {
        let input = ParserInput::new("  ");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn one_newline_is_consumed() {
        let input = ParserInput::new("\n");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn two_newlines_are_consumed() {
        let input = ParserInput::new("\n\n");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn spaces_and_newlines_can_be_used_together() {
        let input = ParserInput::new(" \n");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn comment_can_appear_before_newline() {
        let input = ParserInput::new("--hello\n");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn comment_can_follow_newline() {
        let input = ParserInput::new("\n--hello");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn comment_can_follow_newline_after_indent() {
        let input = ParserInput::new("\n    --hello");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn word_is_not_whitespace() {
        let input = ParserInput::new("hello");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        assert!(result.is_err());
    }

    #[test]
    fn spaces_before_word_are_parsed() {
        let input = ParserInput::new("  hello");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "hello");
    }

    #[test]
    fn newlines_before_word_are_parsed() {
        let input = ParserInput::new("\n\nhello");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "hello");
    }

    #[test]
    fn spaces_and_comments_and_indents_can_be_used_together_before_a_word() {
        let input = ParserInput::new("  -- comment \n    -- comment \n    hello");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "hello");
    }

    #[test]
    fn newline_errors_when_allow_multiline_is_disallowed() {
        let input = ParserInput::new("\n");
        let result = intra_expression_whitespace(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn parses_comment_that_ends_file() {
        let input = ParserInput::new("-- comment");
        let result = intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert!(remainder.is_empty());
    }
}
