use crate::{expression, indent::indent_exact, newline::newline, ExpressionContext};
use ast::{BlockNode, Expression};
use ast::{IResult, ParserInput};
use nom::{
    branch::alt,
    character::complete::space0,
    combinator::{consumed, map, map_res, opt, success},
    sequence::{delimited, preceded, tuple},
};

fn line<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Option<Expression<'a>>> {
    alt((
        delimited(
            indent_exact(context.indentation),
            map(expression(context.disallow_newlines_in_expressions()), Some),
            space0,
        ),
        preceded(space0, success(None)),
    ))
}

/// Parse a block by recursively examining each line. Return the expressions contained in the non-empty lines of the block in REVERSE ORDER.
/// The first expression in the returned `Vec` corresponds to the last nonempty line in the block.
///
/// Consume a whitespace-only line if and only if a non-whitespace line exists later in the block.
/// Consume trailing spaces, but do not consume trailing newlines.
fn block_recursive<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Vec<Expression<'a>>> {
    move |input| {
        map_res(
            tuple((
                line(context),
                opt(tuple((newline, block_recursive(context)))),
            )),
            |(first_line_option, subsequent_lines_option)| match subsequent_lines_option {
                None => {
                    first_line_option.map_or_else(|| Err(()), |first_line| Ok(vec![first_line]))
                }
                Some((_, mut subsequent_lines)) => match first_line_option {
                    None => Ok(subsequent_lines),
                    Some(first_line) => {
                        subsequent_lines.push(first_line);
                        Ok(subsequent_lines)
                    }
                },
            },
        )(input)
    }
}

pub fn block<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, BlockNode<'a>> {
    map(
        consumed(block_recursive(context)),
        |(source, lines_in_reserve_order)| BlockNode {
            source,
            value: lines_in_reserve_order.into_iter().rev().collect(),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_input_is_not_a_block() {
        let input = ParserInput::new("");
        let result = block(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn whitespace_is_not_a_block() {
        let input = ParserInput::new("  ");
        let result = block(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn one_line_is_a_block() {
        let input = ParserInput::new("1");
        let result = block(ExpressionContext::new())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn two_lines_are_a_block() {
        let input = ParserInput::new("1\n2");
        let result = block(ExpressionContext::new())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 2);
    }

    #[test]
    fn one_line_with_expected_indentation_is_a_block() {
        let input = ParserInput::new("    1");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn two_lines_with_expected_indentation_are_a_block() {
        let input = ParserInput::new("    1\n    2");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 2);
    }

    #[test]
    fn if_the_second_of_two_lines_has_too_little_indentation_then_the_block_contains_only_the_first_line(
    ) {
        let input = ParserInput::new("    1\n2");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "\n2");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn if_the_second_of_two_lines_has_too_much_indentation_then_the_block_contains_only_the_first_line(
    ) {
        let input = ParserInput::new("    1\n        2");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "\n        2");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn block_can_contain_empty_lines_with_same_level_of_indentation_as_the_block() {
        let input = ParserInput::new("    1\n    \n    2");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 2);
    }

    #[test]
    fn block_can_contain_empty_lines_with_less_indentation_than_the_block() {
        let input = ParserInput::new("    1\n\n    2");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 2);
    }

    #[test]
    fn block_can_contain_empty_lines_with_more_indentation_than_the_block() {
        let input = ParserInput::new("    1\n        \n    2");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 2);
    }

    #[test]
    fn block_can_start_with_empty_line() {
        let input = ParserInput::new("\n1");
        let result = block(ExpressionContext::new())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn block_can_start_with_empty_line_when_nonzero_indentation_is_expected() {
        let input = ParserInput::new("\n    1");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn block_cannot_end_with_empty_line() {
        let input = ParserInput::new("1\n\n");
        let result = block(ExpressionContext::new())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "\n\n");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn block_cannot_end_with_empty_line_when_nonzero_indentation_is_expected() {
        let input = ParserInput::new("    1\n\n");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "\n\n");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn block_can_include_a_declaration() {
        let input = ParserInput::new("    x = 1");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn block_can_include_two_declarations() {
        let input = ParserInput::new("    x = 1\n    y = 2");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 2);
    }

    #[test]
    fn comment_can_appear_in_block() {
        let input = ParserInput::new("    x = 1 + 1\n    -- this is a comment\n    x");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 2);
    }
}
