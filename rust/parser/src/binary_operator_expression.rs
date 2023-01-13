use crate::{
    basic_expression::basic_expression, expression, identifier::identifier,
    intra_expression_whitespace::intra_expression_whitespace, ExpressionContext,
};
use ast::{
    BinaryOperatorNode, BinaryOperatorSymbol, BinaryOperatorValue, Expression,
    FunctionApplicationArgumentsNode, FunctionApplicationArgumentsValue,
};
use ast::{IResult, ParserInput};
use nom::sequence::delimited;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{consumed, map, opt, value},
    multi::{many0, separated_list0},
    sequence::{preceded, tuple},
};

struct BinaryOperatorSegment<'a> {
    source: ParserInput<'a>,
    symbol: BinaryOperatorSymbol,
    expression: Expression<'a>,
}

enum BinaryOperatorTreeNode<'a> {
    Internal(InternalNode<'a>),
    Leaf(Expression<'a>),
}

struct InternalNode<'a> {
    source: ParserInput<'a>,
    symbol: BinaryOperatorSymbol,
    left_child: Box<BinaryOperatorTreeNode<'a>>,
    right_child: Box<BinaryOperatorTreeNode<'a>>,
}

fn binary_operator_segment_requiring_spaces<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<BinaryOperatorSegment<'a>> {
    map(
        tuple((
            consumed(alt((
                value(BinaryOperatorSymbol::And, tag("and")),
                value(BinaryOperatorSymbol::Or, tag("or")),
            ))),
            intra_expression_whitespace(context),
            basic_expression(context),
        )),
        |((source, symbol), _, expression)| BinaryOperatorSegment {
            source,
            symbol,
            expression,
        },
    )
}

fn binary_operator_segment_not_requiring_spaces<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<BinaryOperatorSegment<'a>> {
    map(
        tuple((
            consumed(alt((
                // for correct parsing, check 2-character symbols before 1-character symbols
                value(BinaryOperatorSymbol::EqualTo, tag("==")),
                value(BinaryOperatorSymbol::NotEqualTo, tag("!=")),
                value(BinaryOperatorSymbol::LessThanOrEqualTo, tag("<=")),
                value(BinaryOperatorSymbol::GreaterThanOrEqualTo, tag(">=")),
                value(BinaryOperatorSymbol::Power, tag("**")),
                value(BinaryOperatorSymbol::Concatenate, tag("++")),
                value(BinaryOperatorSymbol::LessThan, tag("<")),
                value(BinaryOperatorSymbol::GreaterThan, tag(">")),
                value(BinaryOperatorSymbol::Add, tag("+")),
                value(BinaryOperatorSymbol::Subtract, tag("-")),
                value(BinaryOperatorSymbol::Multiply, tag("*")),
                value(BinaryOperatorSymbol::Divide, tag("/")),
                value(BinaryOperatorSymbol::Modulus, tag("%")),
            ))),
            opt(intra_expression_whitespace(context)),
            basic_expression(context),
        )),
        |((source, symbol), _, expression)| BinaryOperatorSegment {
            source,
            symbol,
            expression,
        },
    )
}

fn binary_operator_segment_not_allowing_spaces(
    input: ParserInput,
) -> IResult<BinaryOperatorSegment> {
    map(
        tuple((
            consumed(alt((
                // for correct parsing, check 2-character symbols before 1-character symbols
                value(BinaryOperatorSymbol::MethodLookup, tag(":")),
                value(BinaryOperatorSymbol::FieldLookup, tag(".")),
            ))),
            identifier,
        )),
        |((source, symbol), identifier_node)| BinaryOperatorSegment {
            source,
            symbol,
            expression: Expression::Identifier(identifier_node),
        },
    )(input)
}

fn function_application<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<BinaryOperatorSegment<'a>> {
    move |input| {
        map(
            consumed(map(
                consumed(delimited(
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
                        expression(context.allow_newlines_in_expressions()),
                    ),
                    tuple((
                        opt(intra_expression_whitespace(
                            context.allow_newlines_in_expressions(),
                        )),
                        opt(tuple((
                            tag(","),
                            opt(intra_expression_whitespace(
                                context.allow_newlines_in_expressions(),
                            )),
                        ))),
                        tag(")"),
                    )),
                )),
                |(source, arguments)| {
                    Expression::FunctionApplicationArguments(FunctionApplicationArgumentsNode {
                        source,
                        value: FunctionApplicationArgumentsValue { arguments },
                    })
                },
            )),
            |(source, expression)| BinaryOperatorSegment {
                source,
                symbol: BinaryOperatorSymbol::FunctionApplication,
                expression,
            },
        )(input)
    }
}

const fn order_of_operations(symbol: &BinaryOperatorSymbol) -> u8 {
    match symbol {
        BinaryOperatorSymbol::FunctionApplication
        | BinaryOperatorSymbol::MethodLookup
        | BinaryOperatorSymbol::FieldLookup => 1,
        BinaryOperatorSymbol::Power => 2,
        BinaryOperatorSymbol::Multiply
        | BinaryOperatorSymbol::Divide
        | BinaryOperatorSymbol::Modulus => 3,
        BinaryOperatorSymbol::Add | BinaryOperatorSymbol::Subtract => 4,
        BinaryOperatorSymbol::Concatenate => 5,
        BinaryOperatorSymbol::EqualTo
        | BinaryOperatorSymbol::NotEqualTo
        | BinaryOperatorSymbol::LessThan
        | BinaryOperatorSymbol::LessThanOrEqualTo
        | BinaryOperatorSymbol::GreaterThan
        | BinaryOperatorSymbol::GreaterThanOrEqualTo => 6,
        BinaryOperatorSymbol::And | BinaryOperatorSymbol::Or => 7,
    }
}

fn push_bin_op_segment<'a>(
    node: BinaryOperatorTreeNode<'a>,
    segment: BinaryOperatorSegment<'a>,
) -> BinaryOperatorTreeNode<'a> {
    if let BinaryOperatorTreeNode::Internal(ref internal_node_reference) = node {
        if order_of_operations(&segment.symbol)
            < order_of_operations(&internal_node_reference.symbol)
        {
            if let BinaryOperatorTreeNode::Internal(mut internal_node) = node {
                internal_node.right_child =
                    Box::new(push_bin_op_segment(*internal_node.right_child, segment));
                return BinaryOperatorTreeNode::Internal(internal_node);
            };
        };
    };
    BinaryOperatorTreeNode::Internal(InternalNode {
        source: segment.source,
        symbol: segment.symbol,
        left_child: Box::new(node),
        right_child: Box::new(BinaryOperatorTreeNode::Leaf(segment.expression)),
    })
}

fn transform_to_ast(node: BinaryOperatorTreeNode) -> Expression {
    match node {
        BinaryOperatorTreeNode::Internal(internal_node) => {
            Expression::BinaryOperator(BinaryOperatorNode {
                source: internal_node.source,
                value: BinaryOperatorValue {
                    symbol: internal_node.symbol,
                    left_child: Box::new(transform_to_ast(*internal_node.left_child)),
                    right_child: Box::new(transform_to_ast(*internal_node.right_child)),
                },
            })
        }
        BinaryOperatorTreeNode::Leaf(expression) => expression,
    }
}

/// Parse an expression optionally containing binary operators at the top level.
pub fn binary_operator_expression<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    map(
        tuple((
            basic_expression(context),
            many0(alt((
                preceded(
                    intra_expression_whitespace(context),
                    alt((
                        binary_operator_segment_requiring_spaces(context),
                        binary_operator_segment_not_requiring_spaces(context),
                        function_application(context),
                    )),
                ),
                binary_operator_segment_not_requiring_spaces(context),
                binary_operator_segment_not_allowing_spaces,
                function_application(context),
            ))),
        )),
        |(expression, segments)| {
            let mut node = BinaryOperatorTreeNode::Leaf(expression);
            for segment in segments {
                node = push_bin_op_segment(node, segment);
            }
            transform_to_ast(node)
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::IdentifierValue;

    #[test]
    fn empty_input_is_not_binary_operator_expression() {
        let input = ParserInput::new("");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        assert!(result.is_err());
    }

    #[test]
    fn binary_operator_not_requiring_spaces_preserves_child_expressions() {
        let input = ParserInput::new("1+2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        match expression {
            Expression::BinaryOperator(node) => {
                assert!(matches!(*node.value.left_child, Expression::Integer(_)));
                assert!(matches!(*node.value.right_child, Expression::Integer(_)));
            }
            _ => panic!("Expected BinaryOperator"),
        }
    }

    #[test]
    fn binary_operator_requiring_spaces_preserves_child_expressions() {
        let input = ParserInput::new("1 and 2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        match expression {
            Expression::BinaryOperator(node) => {
                assert!(matches!(*node.value.left_child, Expression::Integer(_)));
                assert!(matches!(*node.value.right_child, Expression::Integer(_)));
            }
            _ => panic!("Expected BinaryOperator"),
        }
    }

    #[test]
    fn binary_operator_not_requiring_spaces_can_be_padded_by_spaces() {
        let input = ParserInput::new("1   + 2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn binary_operator_requiring_spaces_without_leading_space_stops_parsing_after_last_valid_expression(
    ) {
        let input = ParserInput::new("1and 2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "and 2");
        assert!(matches!(expression, Expression::Integer(_)));
    }

    #[test]
    fn binary_operator_requiring_spaces_without_trailing_space_stops_parsing_after_last_valid_expression(
    ) {
        let input = ParserInput::new("1 and2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, " and2");
        assert!(matches!(expression, Expression::Integer(_)));
    }

    #[test]
    fn recognize_add() {
        let input = ParserInput::new("1+2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Add,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_subtract() {
        let input = ParserInput::new("1-2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Subtract,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_multiply() {
        let input = ParserInput::new("1*2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Multiply,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_divide() {
        let input = ParserInput::new("1/2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Divide,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_modulus() {
        let input = ParserInput::new("1%2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Modulus,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_power() {
        let input = ParserInput::new("1**2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Power,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_concatenate() {
        let input = ParserInput::new("\"Hello\" ++ \"World\"");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Concatenate,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_and() {
        let input = ParserInput::new("1 and 2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::And,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_or() {
        let input = ParserInput::new("1 or 2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Or,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_equal_to() {
        let input = ParserInput::new("1==2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
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
    fn recognize_not_equal_to() {
        let input = ParserInput::new("1!=2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::NotEqualTo,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_less_than() {
        let input = ParserInput::new("1<2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::LessThan,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_less_than_or_equal_to() {
        let input = ParserInput::new("1<=2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::LessThanOrEqualTo,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_greater_than() {
        let input = ParserInput::new("1>2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::GreaterThan,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_greater_than_or_equal_to() {
        let input = ParserInput::new("1>=2");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::GreaterThanOrEqualTo,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn recognize_comparison_operator_when_using_string_inputs() {
        let input = ParserInput::new("\"hello\"==\"world\"");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
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
    fn comparison_operator_preserves_child_expressions_when_using_string_inputs() {
        let input = ParserInput::new("\"hello\"==\"world\"");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        match expression {
            Expression::BinaryOperator(node) => {
                assert!(matches!(
                    *node.value.left_child,
                    Expression::StringLiteral(_)
                ));
                assert!(matches!(
                    *node.value.right_child,
                    Expression::StringLiteral(_)
                ));
            }
            _ => panic!("Expected BinaryOperator"),
        }
    }

    #[test]
    fn recognize_comparison_operator_when_using_list_inputs() {
        let input = ParserInput::new("[1]==[2,3]");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
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
    fn comparison_operator_preserves_child_expressions_when_using_list_inputs() {
        let input = ParserInput::new("[1]==[2,3]");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        match expression {
            Expression::BinaryOperator(node) => {
                assert!(matches!(*node.value.left_child, Expression::List(_)));
                assert!(matches!(*node.value.right_child, Expression::List(_)));
            }
            _ => panic!("Expected BinaryOperator"),
        }
    }

    #[test]
    fn recognize_function_application() {
        let input = ParserInput::new("a()");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::FunctionApplication,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn function_application_can_have_one_argument() {
        let input = ParserInput::new("a(1)");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_application_can_have_two_arguments() {
        let input = ParserInput::new("a(1,2)");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_application_can_be_separated_by_spaces() {
        let input = ParserInput::new("a  (  1  ,  2  )");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_application_arguments_can_be_separated_by_newlines_when_newlines_are_disallowed_in_expressions(
    ) {
        let input = ParserInput::new("a(\n1\n,\n2\n)");
        let result = binary_operator_expression(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_application_argument_can_be_another_function_application() {
        let input = ParserInput::new("a(b())");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_application_argument_can_be_a_binary_operator_expression() {
        let input = ParserInput::new("a(1+2)");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_application_argument_can_be_an_if_expression() {
        let input = ParserInput::new("a(if 1 == 2 do 3 else 4)");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_application_can_be_applied_to_another_function_application() {
        let input = ParserInput::new("a()()");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_application_preserves_function_expression() {
        let input = ParserInput::new("a(314)");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (_, expression) = result.unwrap();
        match expression {
            Expression::BinaryOperator(binary_node) => match *binary_node.value.left_child {
                Expression::Identifier(identifier_node) => {
                    assert_eq!(identifier_node.value.name, "a");
                }
                _ => panic!("Expected Identifier"),
            },
            _ => panic!("Expected BinaryOperator"),
        }
    }

    #[test]
    fn function_application_preserves_argument_value() {
        let input = ParserInput::new("a(314)");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (_, expression) = result.unwrap();
        match expression {
            Expression::BinaryOperator(binary_node) => match *binary_node.value.right_child {
                Expression::FunctionApplicationArguments(arguments_node) => {
                    assert_eq!(arguments_node.value.arguments.len(), 1);
                    match arguments_node.value.arguments.get(0).unwrap() {
                        Expression::Integer(integer_node) => {
                            assert_eq!(integer_node.value, 314);
                        }
                        _ => panic!("Expected Integer"),
                    }
                }
                _ => panic!("Expected FunctionArguments"),
            },
            _ => panic!("Expected BinaryOperator"),
        }
    }

    #[test]
    fn recognize_method_lookup() {
        let input = ParserInput::new("a:b");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::MethodLookup,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn function_application_on_method_lookup_parses() {
        let input = ParserInput::new("a:b()");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn recognize_field_lookup() {
        let input = ParserInput::new("a.b");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::FieldLookup,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn with_two_binary_operators_of_equal_order_the_second_is_the_root_of_the_expression() {
        let input = ParserInput::new("1+2-3");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Subtract,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn with_two_binary_operators_of_increasing_order_the_second_is_the_root_of_the_expression() {
        let input = ParserInput::new("1*2-3");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Subtract,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn with_two_binary_operators_of_decreasing_order_the_first_is_the_root_of_the_expression() {
        let input = ParserInput::new("1+2*3");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Add,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn when_two_binary_operators_are_parsed_the_non_root_operator_is_also_parsed() {
        let input = ParserInput::new("1+2*3");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        match expression {
            Expression::BinaryOperator(node) => {
                assert!(matches!(
                    *node.value.right_child,
                    Expression::BinaryOperator(BinaryOperatorNode {
                        value: BinaryOperatorValue {
                            symbol: BinaryOperatorSymbol::Multiply,
                            ..
                        },
                        ..
                    })
                ));
            }
            _ => panic!("Expected BinaryOperator"),
        }
    }

    #[test]
    fn parentheses_on_left_side_of_operator_supersede_order_of_operations() {
        let input = ParserInput::new("(1+2)*3");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Multiply,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn parentheses_on_right_side_of_operator_supersede_order_of_operations() {
        let input = ParserInput::new("1*(2+3)");
        let result = binary_operator_expression(
            ExpressionContext::new().allow_newlines_in_expressions(),
        )(input);
        let (remainder, expression) = result.unwrap();
        assert_eq!(remainder, "");
        assert!(matches!(
            expression,
            Expression::BinaryOperator(BinaryOperatorNode {
                value: BinaryOperatorValue {
                    symbol: BinaryOperatorSymbol::Multiply,
                    ..
                },
                ..
            })
        ));
    }
}
