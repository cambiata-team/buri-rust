use crate::{
    block::block, expression, identifier::identifier, indent::indent_exact,
    intra_expression_whitespace::intra_expression_whitespace, newline::newline,
    tag_identifier::tag_identifier, type_expression::type_expression, ExpressionContext,
};
use ast::{Expression, IResult, ParserInput, WhenCase, WhenCaseArgumentValue, WhenNode, WhenValue};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::{consumed, map, opt, success, verify},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
};
use std::collections::HashSet;

fn when_condition<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    delimited(
        tuple((tag("when"), space1)),
        expression(context),
        tuple((space1, tag("is"), newline)),
    )
}

fn case_argument<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, WhenCaseArgumentValue<'a>> {
    map(
        tuple((
            identifier,
            opt(preceded(
                tuple((
                    opt(intra_expression_whitespace(
                        context.allow_newlines_in_expressions(),
                    )),
                    tag(":"),
                    opt(intra_expression_whitespace(
                        context.allow_newlines_in_expressions(),
                    )),
                )),
                type_expression,
            )),
        )),
        |(identifier, type_expression)| WhenCaseArgumentValue {
            identifier,
            type_expression,
        },
    )
}

fn case_arguments_vec<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Vec<WhenCaseArgumentValue<'a>>> {
    alt((
        delimited(
            tuple((
                tag("("),
                opt(intra_expression_whitespace(
                    context.allow_newlines_in_expressions(),
                )),
            )),
            separated_list0(
                tuple((
                    opt(intra_expression_whitespace(
                        context.allow_newlines_in_expressions(),
                    )),
                    tag(","),
                    opt(intra_expression_whitespace(
                        context.allow_newlines_in_expressions(),
                    )),
                )),
                case_argument(context),
            ),
            tuple((
                opt(intra_expression_whitespace(
                    context.allow_newlines_in_expressions(),
                )),
                opt(tag(",")),
                opt(intra_expression_whitespace(
                    context.allow_newlines_in_expressions(),
                )),
                tag(")"),
            )),
        ),
        success(vec![]),
    ))
}

fn case_expression<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    alt((
        delimited(space1, expression(context), space0),
        preceded(
            tuple((space0, newline)),
            map(block(context.increment_indentation()), Expression::Block),
        ),
    ))
}

fn when_case<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, WhenCase<'a>> {
    map(
        tuple((
            indent_exact(context.indentation),
            tag_identifier,
            case_arguments_vec(context),
            space1,
            tag("do"),
            case_expression(context),
        )),
        |(_, case_name, case_arguments, _, _, expression)| WhenCase {
            case_name,
            case_arguments,
            expression,
        },
    )
}

fn when_default_case<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    preceded(
        tuple((
            indent_exact(context.indentation),
            tag("_"),
            space1,
            tag("do"),
        )),
        case_expression(context),
    )
}

pub fn when_statement<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, WhenNode<'a>> {
    move |input| {
        verify(
            map(
                consumed(tuple((
                    when_condition(context),
                    tuple((
                        separated_list1(newline, when_case(context.increment_indentation())),
                        opt(preceded(
                            newline,
                            when_default_case(context.increment_indentation()),
                        )),
                    )),
                ))),
                |(source, (condition, (cases, default_case)))| WhenNode {
                    source,
                    value: WhenValue {
                        condition: Box::new(condition),
                        cases,
                        default_case: default_case.map(Box::new),
                    },
                },
            ),
            |node| {
                let mut case_name = HashSet::new();
                for case in &node.value.cases {
                    if !case_name.insert(case.case_name.value.clone()) {
                        return false;
                    }
                }
                true
            },
        )(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string_is_not_when_statement() {
        let input = ParserInput::new("");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn when_keyword_is_not_when_statement() {
        let input = ParserInput::new("when");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn when_keyword_with_expression_and_is_keyword_is_not_when_statement() {
        let input = ParserInput::new("when x is");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn one_simple_case_parses() {
        let input = ParserInput::new("when x is\n    #red do 1");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn two_simple_cases_parse() {
        let input = ParserInput::new("when x is\n    #red do 1\n    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn parse_a_default_case() {
        let input = ParserInput::new("when x is\n    #red do 1\n    _ do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn stop_parsing_after_default_case() {
        let input = ParserInput::new("when x is\n    #red do 1\n    _ do 2\n    #blue do 3");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "\n    #blue do 3");
    }

    #[test]
    fn error_on_only_default_case() {
        let input = ParserInput::new("when x is\n    _ do 1");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn allow_only_one_default_case() {
        let input = ParserInput::new("when x is\n    #red do 1\n    _ do 2\n    _ do 3");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "\n    _ do 3");
    }

    #[test]
    fn missing_when_keyword_errors() {
        let input = ParserInput::new("x is\n    #red do 1\n    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_is_keyword_errors() {
        let input = ParserInput::new("when x\n    #red do 1\n    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_do_keyword_errors() {
        let input = ParserInput::new("when x is\n    #red 1\n    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_expression_errors() {
        let input = ParserInput::new("when x is\n    #red do\n    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn newline_after_when_keyword_errors() {
        let input = ParserInput::new("when\nx is\n    #red do 1\n    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_newline_after_is_keyword_errors() {
        let input = ParserInput::new("when x is 1 #red do 1\n    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_newline_after_case_does_not_consume_following_cases() {
        let input = ParserInput::new("when x is\n    #red do 1    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "#blue do 2");
    }

    #[test]
    fn condition_can_be_tag_literal() {
        let input = ParserInput::new("when #red is\n    #red do 1\n    #blue do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn cases_can_be_blocks() {
        let input = ParserInput::new("when x is\n    #red do\n        1\n    #blue do\n        2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn cases_can_be_multiline_blocks() {
        let input = ParserInput::new(
            "when x is\n    #red do\n        1\n        2\n    #blue do\n        3\n        4",
        );
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn default_case_can_be_block() {
        let input = ParserInput::new("when x is\n    #red do 1\n    _ do\n        2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn cases_can_have_payloads() {
        let input = ParserInput::new("when x is\n    #red(value) do 1\n    #blue(value) do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn cases_can_have_payloads_with_multiple_arguments() {
        let input = ParserInput::new("when x is\n    #red(a, b) do 1\n    #blue(c, d) do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn cases_can_have_payloads_with_different_numbers_of_arguments() {
        let input = ParserInput::new("when x is\n    #red(a, b) do 1\n    #blue(c) do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn simple_tags_can_be_mixed_with_tags_with_payloads() {
        let input = ParserInput::new("when x is\n    #red do 1\n    #blue(value) do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn default_case_cannot_have_a_payload() {
        let input = ParserInput::new("when x is\n    #red do 1\n    _(value) do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "\n    _(value) do 2");
    }

    #[test]
    fn payload_can_have_an_explicit_type_annotation() {
        let input =
            ParserInput::new("when x is\n    #red(value: Int) do 1\n    #blue(value: Int) do 2");
        let result = when_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }
}
