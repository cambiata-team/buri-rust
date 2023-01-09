use crate::{expression, indent::indent_exact, newline::newline, ExpressionContext};
use ast::Expression;
use ast::{IResult, ParserInput};
use nom::{
    branch::alt,
    character::complete::{space0, space1},
    combinator::{eof, map, success},
    sequence::{delimited, preceded, tuple},
};

/// Parse a line of code.
/// Return `None` if the line does not contain any statements.
/// Return `Some(Line)` with the content of the line otherwise.
/// Error if the line contains a statement or malformed statement, and actual indentation does not match expected indentation.
pub fn line<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Option<Expression<'a>>> {
    alt((
        delimited(
            indent_exact(context.indentation),
            map(expression(context.disallow_newlines_in_expressions()), Some),
            tuple((space0, alt((newline, eof)))),
        ),
        preceded(tuple((space1, alt((newline, eof)))), success(None)),
        preceded(newline, success(None)),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn error_if_line_is_empty() {
        let input = ParserInput::new("");
        let result = line(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn return_none_if_line_is_only_spaces() {
        let input = ParserInput::new("  ");
        let result = line(ExpressionContext::new())(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(parsed.is_none());
    }

    #[test]
    fn return_none_if_line_is_only_spaces_for_any_expected_indentation() {
        let input = ParserInput::new("  ");
        let result = line(
            ExpressionContext::new()
                .increment_indentation()
                .increment_indentation()
                .increment_indentation(),
        )(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(parsed.is_none());
    }

    #[test]
    fn line_can_contain_if_statement() {
        let input = ParserInput::new("if 1 == 2 do 3");
        let result = line(ExpressionContext::new())(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(parsed, Some(_)));
    }

    #[test]
    fn line_can_contain_binary_expression() {
        let input = ParserInput::new("1 + 2");
        let result = line(ExpressionContext::new())(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(parsed, Some(_)));
    }

    #[test]
    fn errors_if_actual_indentation_is_smaller_than_expected() {
        let input = ParserInput::new("1 + 2");
        let result = line(ExpressionContext::new().increment_indentation())(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_if_actual_indentation_is_bigger_than_expected() {
        let input = ParserInput::new("    1 + 2");
        let result = line(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn returns_some_if_actual_indentation_is_equal_to_expected() {
        let input = ParserInput::new("    1 + 2");
        let result = line(ExpressionContext::new().increment_indentation())(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(parsed.is_some());
    }

    #[test]
    fn returns_some_if_terminated_by_lf() {
        let input = ParserInput::new("1 + 2\n");
        let result = line(ExpressionContext::new())(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(parsed.is_some());
    }

    #[test]
    fn returns_some_if_terminated_by_crlf() {
        let input = ParserInput::new("1 + 2\r\n");
        let result = line(ExpressionContext::new())(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(parsed.is_some());
    }

    #[test]
    fn returns_some_if_followed_by_spaces() {
        let input = ParserInput::new("1 + 2  ");
        let result = line(ExpressionContext::new())(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(parsed.is_some());
    }

    #[test]
    fn returns_some_if_followed_by_comment() {
        let input = ParserInput::new("1 + 2--hello");
        let result = line(ExpressionContext::new())(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(parsed.is_some());
    }
}
