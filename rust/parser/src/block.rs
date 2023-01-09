use crate::{line::line, ExpressionContext};
use ast::BlockNode;
use ast::{IResult, ParserInput};
use nom::{
    combinator::{consumed, map, verify},
    multi::many1,
};

pub fn block<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, BlockNode<'a>> {
    verify(
        map(
            consumed(many1(line(context))),
            |(source, maybe_expressions)| {
                let mut nonempty_lines = Vec::new();
                for maybe_expression in maybe_expressions {
                    maybe_expression.map_or((), |line_contents| nonempty_lines.push(line_contents));
                }
                BlockNode {
                    source,
                    value: nonempty_lines,
                }
            },
        ),
        |node| !node.value.is_empty(),
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
        assert_eq!(remainder, "2");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn if_the_second_of_two_lines_has_too_much_indentation_then_the_block_contains_only_the_first_line(
    ) {
        let input = ParserInput::new("    1\n        2");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "        2");
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
    fn block_can_end_with_empty_line() {
        let input = ParserInput::new("1\n\n");
        let result = block(ExpressionContext::new())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 1);
    }

    #[test]
    fn block_can_end_with_empty_line_when_nonzero_indentation_is_expected() {
        let input = ParserInput::new("    1\n\n");
        let result = block(ExpressionContext::new().increment_indentation())(input);
        let (remainder, lines) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(lines.value.len(), 1);
    }
}
