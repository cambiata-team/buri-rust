use crate::{
    binary_operator_expression::binary_operator_expression, block::block, expression,
    indent::indent_exact, intra_expression_whitespace::intra_expression_whitespace,
    newline::newline, ExpressionContext,
};
use ast::{Expression, IfNode, IfValue};
use ast::{IResult, ParserInput};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::{cond, consumed, eof, map, opt},
    sequence::{delimited, preceded, terminated, tuple},
};

fn if_condition<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    delimited(
        tuple((
            tag("if"),
            intra_expression_whitespace(context.disallow_newlines_in_expressions()),
        )),
        binary_operator_expression(context.disallow_newlines_in_expressions()),
        tuple((
            intra_expression_whitespace(context.disallow_newlines_in_expressions()),
            tag("do"),
        )),
    )
}

struct IfElsePair<'a> {
    pub path_if_true: Expression<'a>,
    pub path_if_false: Option<Expression<'a>>,
}

fn blockless_if<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, IfElsePair> {
    map(
        tuple((
            intra_expression_whitespace(context.disallow_newlines_in_expressions()),
            binary_operator_expression(context.disallow_newlines_in_expressions()),
            opt(preceded(
                tuple((
                    intra_expression_whitespace(context.disallow_newlines_in_expressions()),
                    tag("else"),
                    intra_expression_whitespace(context.disallow_newlines_in_expressions()),
                )),
                binary_operator_expression(context.disallow_newlines_in_expressions()),
            )),
            cond(
                !(context.allow_newlines_in_expressions),
                tuple((space0, alt((newline, eof)))),
            ),
        )),
        |(_, expression_if_true, maybe_expression_if_false, _)| IfElsePair {
            path_if_true: expression_if_true,
            path_if_false: maybe_expression_if_false,
        },
    )
}

fn block_if<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, IfElsePair> {
    map(
        tuple((
            space0,
            newline,
            map(block(context.increment_indentation()), Expression::Block),
            opt(preceded(
                tuple((indent_exact(context.indentation), tag("else"))),
                alt((
                    preceded(
                        tuple((space0, newline)),
                        map(block(context.increment_indentation()), Expression::Block),
                    ),
                    preceded(
                        space1,
                        alt((
                            map(
                                if_statement(context.disallow_newlines_in_expressions()),
                                Expression::If,
                            ),
                            terminated(
                                expression(context.disallow_newlines_in_expressions()),
                                tuple((space0, newline)),
                            ),
                        )),
                    ),
                )),
            )),
        )),
        |(_, _, path_if_true, path_if_false)| IfElsePair {
            path_if_true,
            path_if_false,
        },
    )
}

pub fn if_statement<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, IfNode<'a>> {
    move |input| {
        map(
            consumed(tuple((
                if_condition(context),
                alt((blockless_if(context), block_if(context))),
            ))),
            |(source, (condition, blocks))| IfNode {
                source,
                value: IfValue {
                    condition: Box::new(condition),
                    path_if_true: Box::new(blocks.path_if_true),
                    path_if_false: blocks.path_if_false.map(Box::new),
                },
            },
        )(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::{BinaryOperatorNode, BinaryOperatorSymbol, BinaryOperatorValue, IntegerNode};

    #[test]
    fn empty_string_is_not_if_statement() {
        let input = ParserInput::new("");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_err());
    }

    #[test]
    fn if_keyword_is_not_if_statement() {
        let input = ParserInput::new("if");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_err());
    }

    #[test]
    fn if_keyword_and_condition_is_not_if_statement() {
        let input = ParserInput::new("if 1 == 2");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_err());
    }

    #[test]
    fn if_keyword_and_condition_and_do_keyword_is_not_if_statement() {
        let input = ParserInput::new("if 1 == 2 do");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_err());
    }

    #[test]
    fn basic_if_statement_is_parsed() {
        let input = ParserInput::new("if 1 == 2 do 3");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_ok());
    }

    #[test]
    fn else_statement_with_missing_content_is_not_included_in_well_formed_if_statement() {
        let input = ParserInput::new("if 1 == 2 do 3 else ");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, " else ");
    }

    #[test]
    fn if_else_is_parsed() {
        let input = ParserInput::new("if 1 == 2 do 3 else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn content_of_if_else_condition_is_preserved() {
        let input = ParserInput::new("if 1 == 2 do 3 else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, parsed) = result.unwrap();
        assert!(matches!(
            *parsed.value.condition,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::EqualTo,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn content_of_true_branch_is_preserved() {
        let input = ParserInput::new("if 1 == 2 do 3 else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, parsed) = result.unwrap();
        assert!(matches!(
            *(parsed.value.path_if_true),
            Expression::Integer(IntegerNode { value: 3, .. })
        ));
    }

    #[test]
    fn content_of_false_branch_is_preserved() {
        let input = ParserInput::new("if 1 == 2 do 3 else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, parsed) = result.unwrap();
        assert!(matches!(
            *(parsed.value.path_if_false.unwrap()),
            Expression::Integer(IntegerNode { value: 4, .. })
        ));
    }

    #[test]
    fn newlines_cannot_replace_spaces_even_if_newlines_are_allowed_in_expressions() {
        let input = ParserInput::new("if 1 ==\n2 do 3 else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_no_whitespace_after_if_keyword() {
        let input = ParserInput::new("if1 == 2 do 3 else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_no_whitespace_before_do_keyword() {
        let input = ParserInput::new("if 1 == 2do 3 else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_no_whitespace_after_do_keyword() {
        let input = ParserInput::new("if 1 == 2 do3 else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        assert!(result.is_err());
    }

    #[test]
    fn stops_parsing_before_else_when_no_whitespace_before_else_keyword() {
        let input = ParserInput::new("if 1 == 2 do 3else 4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "else 4");
    }

    #[test]
    fn stops_parsing_before_else_when_no_whitespace_after_else_keyword() {
        let input = ParserInput::new("if 1 == 2 do 3 else4");
        let result = if_statement(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, " else4");
    }

    #[test]
    fn basic_if_statement_parses_when_disallowing_multiline_expressions() {
        let input = ParserInput::new("if 1 == 2 do 3\n");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn when_disallowing_multiline_expressions_indentation_is_required_on_true_block() {
        let input = ParserInput::new("if 1 == 2 do\n3");
        let result = if_statement(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn when_disallowing_multiline_expressions_correct_indentation_is_accepted() {
        let input = ParserInput::new("if 1 == 2 do\n    3");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn when_disallowing_multiline_expressions_can_have_else_block() {
        let input = ParserInput::new("if 1 == 2 do\n    3\nelse\n    4");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn when_disallowing_multiline_expressions_parsing_stops_before_misindented_else_block() {
        let input = ParserInput::new("if 1 == 2 do\n    3\nelse\n4");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "else\n4");
    }

    #[test]
    fn when_disallowing_multiline_expressions_parsing_stops_before_misindented_else_keyword() {
        let input = ParserInput::new("if 1 == 2 do\n    3\n    else\n    4");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "    else\n    4");
    }

    #[test]
    fn when_disallowing_multiline_expressions_parsing_stops_before_one_line_else_clause() {
        let input = ParserInput::new("if 1 == 2 do\n    3\nelse 4");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "else 4");
    }

    #[test]
    fn when_disallowing_multiline_expressions_parsing_stops_after_one_line_if_clause() {
        let input = ParserInput::new("if 1 == 2 do 3\nelse\n    4");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "else\n    4");
    }

    #[test]
    fn if_can_follow_else() {
        let input = ParserInput::new("if 1 == 2 do\n    3\nelse if 4 == 5 do\n    6");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn else_can_follow_else_if() {
        let input = ParserInput::new("if 1 == 2 do\n    3\nelse if 4 == 5 do\n    6\nelse\n    7");
        let result = if_statement(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }
}
