use crate::{
    expression, intra_expression_whitespace::intra_expression_whitespace, ExpressionContext,
};
use ast::{Expression, ListNode};
use ast::{IResult, ParsedNode, ParserInput};
use nom::{
    bytes::complete::tag,
    combinator::{consumed, map, opt},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
};

fn padded_expression(input: ParserInput) -> IResult<Expression> {
    preceded(
        opt(intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )),
        expression(ExpressionContext::new().allow_newlines_in_expressions()),
    )(input)
}

fn padded_comma(input: ParserInput) -> IResult<ParserInput> {
    preceded(
        opt(intra_expression_whitespace(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )),
        tag(","),
    )(input)
}

fn list_contents(input: ParserInput) -> IResult<Vec<Expression>> {
    map(
        tuple((
            padded_expression,
            many0(preceded(padded_comma, padded_expression)),
            opt(padded_comma),
        )),
        |mut items| {
            let mut accumulator = Vec::new();
            accumulator.push(items.0);
            accumulator.append(&mut items.1);
            accumulator
        },
    )(input)
}

pub fn list(input: ParserInput) -> IResult<ListNode> {
    map(
        consumed(delimited(
            tag("["),
            terminated(
                opt(list_contents),
                opt(intra_expression_whitespace(
                    ExpressionContext::new().allow_newlines_in_expressions(),
                )),
            ),
            tag("]"),
        )),
        |(consumed_input, produced_output)| ParsedNode {
            source: consumed_input,
            value: produced_output.unwrap_or_default(),
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string_is_not_a_list() {
        let input = ParserInput::new("");
        let result = list(input);
        assert!(result.is_err());
    }

    #[test]
    fn unpaired_open_square_bracket_is_not_a_list() {
        let input = ParserInput::new("[");
        let result = list(input);
        assert!(result.is_err());
    }

    #[test]
    fn list_contents_is_not_a_list() {
        let input = ParserInput::new("\"\"");
        let result = list(input);
        assert!(result.is_err());
    }

    #[test]
    fn paired_square_brackets_is_an_empty_list() {
        let input = ParserInput::new("[]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 0);
    }

    #[test]
    fn list_with_one_element_is_parsed() {
        let input = ParserInput::new("[ \"\" ]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 1);
    }

    #[test]
    fn list_with_one_element_and_trailing_comma_is_parsed() {
        let input = ParserInput::new("[ \"\", ]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 1);
    }

    #[test]
    fn list_with_two_elements_is_parsed() {
        let input = ParserInput::new("[ \"\", \"\" ]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 2);
    }

    #[test]
    fn list_with_two_elements_and_trailing_comma_is_parsed() {
        let input = ParserInput::new("[ \"\", \"\", ]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 2);
    }

    #[test]
    fn list_can_span_multiple_lines() {
        let input = ParserInput::new("[\n    \"\",\n    \"\",\n]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 2);
    }

    #[test]
    fn list_can_have_empty_lines() {
        let input = ParserInput::new("[\n    \"\",\n\n\n    \"\",\n]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 2);
    }

    #[test]
    fn comments_can_be_inserted_into_the_list() {
        let input = ParserInput::new("[-- hello\n    \"\", -- description of element 1\n\n\n    \"\", -- description of element 2\n]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 2);
    }

    #[test]
    fn a_list_can_contain_sublists() {
        let input = ParserInput::new("[[],[],[]]");
        let result = list(input);
        let (remainder, consumed) = result.unwrap();
        assert!(remainder.is_empty());
        assert_eq!(consumed.value.len(), 3);
    }
}
