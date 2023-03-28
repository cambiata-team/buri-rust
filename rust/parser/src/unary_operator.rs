use crate::{
    basic_expression::basic_expression, binary_operator_expression::binary_operator_expression,
    intra_expression_whitespace::intra_expression_whitespace, ExpressionContext,
};
use ast::{
    BinaryOperatorSymbol, Expression, IResult, ParsedNode, ParserInput, UnaryOperatorNode,
    UnaryOperatorSymbol, UnaryOperatorValue,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{consumed, map, not, opt, peek, verify},
    sequence::preceded,
};

pub fn unary_operator_expression(
    context: ExpressionContext,
    input: ParserInput,
) -> IResult<UnaryOperatorNode> {
    map(
        consumed(alt((
            map(
                preceded(
                    tag("not"),
                    preceded(
                        intra_expression_whitespace(context),
                        alt((
                            verify(binary_operator_expression(context), |expression| {
                                match expression {
                                    Expression::BinaryOperator(binary_operator) => {
                                        matches!(
                                            binary_operator.value.symbol,
                                            BinaryOperatorSymbol::FunctionApplication
                                                | BinaryOperatorSymbol::MethodLookup
                                                | BinaryOperatorSymbol::FieldLookup
                                        )
                                    }
                                    _ => false,
                                }
                            }),
                            basic_expression(context),
                        )),
                    ),
                ),
                |expr| UnaryOperatorValue {
                    symbol: UnaryOperatorSymbol::Not,
                    child: Box::new(expr),
                },
            ),
            map(
                preceded(
                    preceded(tag("-"), not(peek(tag("-")))),
                    preceded(
                        opt(intra_expression_whitespace(context)),
                        basic_expression(context),
                    ),
                ),
                |expr| UnaryOperatorValue {
                    symbol: UnaryOperatorSymbol::Negative,
                    child: Box::new(expr),
                },
            ),
        ))),
        |(consumed_input, produced_output)| ParsedNode {
            source: consumed_input,
            value: produced_output,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::Expression;
    use std::borrow::Borrow;

    #[test]
    fn empty_string_errors() {
        let input = ParserInput::new("");
        let result = unary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
            input,
        );
        assert!(result.is_err());
    }

    #[test]
    fn not_expression_is_parsed() {
        let input = ParserInput::new("not \"\"");
        let result = unary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
            input,
        );
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(consumed.value.symbol, UnaryOperatorSymbol::Not));
        assert!(matches!(
            consumed.value.child.borrow(),
            Expression::StringLiteral(_)
        ));
    }

    #[test]
    fn not_followed_by_nonexpression_is_not_a_unary_expression() {
        let input = ParserInput::new("not )");
        let result = unary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
            input,
        );
        assert!(result.is_err());
    }

    #[test]
    fn negative_expression_is_parsed() {
        let input = ParserInput::new("-\"\"");
        let result = unary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
            input,
        );
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            consumed.value.symbol,
            UnaryOperatorSymbol::Negative
        ));
        assert!(matches!(
            consumed.value.child.borrow(),
            Expression::StringLiteral(_)
        ));
    }

    #[test]
    fn negative_followed_by_nonexpression_is_not_a_unary_expression() {
        let input = ParserInput::new("-)");
        let result = unary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
            input,
        );
        assert!(result.is_err());
    }

    #[test]
    fn not_followed_by_function_call_is_not_expression() {
        let input = ParserInput::new("not foo()");
        let result = unary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
            input,
        );
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(consumed.value.symbol, UnaryOperatorSymbol::Not));
    }

    #[test]
    fn comment_symbol_is_not_unary_operator() {
        let input = ParserInput::new("--1");
        let result = unary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
            input,
        );
        assert!(result.is_err());
    }
}
