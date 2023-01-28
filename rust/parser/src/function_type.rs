use crate::{
    intra_expression_whitespace::intra_expression_whitespace, type_expression::type_expression,
    ExpressionContext,
};
use ast::{FunctionTypeNode, FunctionTypeValue};
use ast::{IResult, ParserInput};
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{consumed, map, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
};

pub fn function_type(input: ParserInput) -> IResult<FunctionTypeNode> {
    map(
        consumed(tuple((
            delimited(
                tuple((
                    char('('),
                    opt(intra_expression_whitespace(
                        ExpressionContext::new().allow_newlines_in_expressions(),
                    )),
                )),
                separated_list0(
                    tuple((
                        opt(intra_expression_whitespace(
                            ExpressionContext::new().allow_newlines_in_expressions(),
                        )),
                        char(','),
                        opt(intra_expression_whitespace(
                            ExpressionContext::new().allow_newlines_in_expressions(),
                        )),
                    )),
                    type_expression,
                ),
                tuple((
                    opt(tuple((
                        opt(intra_expression_whitespace(
                            ExpressionContext::new().allow_newlines_in_expressions(),
                        )),
                        char(','),
                    ))),
                    opt(intra_expression_whitespace(
                        ExpressionContext::new().allow_newlines_in_expressions(),
                    )),
                    char(')'),
                    opt(intra_expression_whitespace(
                        ExpressionContext::new().allow_newlines_in_expressions(),
                    )),
                    tag("=>"),
                    opt(intra_expression_whitespace(
                        ExpressionContext::new().allow_newlines_in_expressions(),
                    )),
                )),
            ),
            type_expression,
        ))),
        |(source, (arguments, return_type))| FunctionTypeNode {
            source,
            value: FunctionTypeValue {
                return_type: Box::new(return_type),
                arguments,
            },
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::{FunctionTypeNode, FunctionTypeValue, TypeExpression};

    #[test]
    fn parses_return_type() {
        let input = ParserInput::new("() => Str");
        let (_, function) = function_type(input).unwrap();
        let FunctionTypeNode {
            value: FunctionTypeValue { return_type, .. },
            ..
        } = function;
        assert!(matches!(*return_type, TypeExpression::Identifier(_)));
    }

    #[test]
    fn parses_a_single_argument() {
        let input = ParserInput::new("([Int]) => Str");
        let (_, function) = function_type(input).unwrap();
        let FunctionTypeNode {
            value: FunctionTypeValue { arguments, .. },
            ..
        } = function;
        assert_eq!(arguments.len(), 1);
        assert!(matches!(arguments[0], TypeExpression::List(_)));
    }

    #[test]
    fn parses_multiple_arguments() {
        let input = ParserInput::new("([Int], #true) => Str");
        let (_, function) = function_type(input).unwrap();
        let FunctionTypeNode {
            value: FunctionTypeValue { arguments, .. },
            ..
        } = function;
        assert_eq!(arguments.len(), 2);
        assert!(matches!(arguments[0], TypeExpression::List(_)));
        assert!(matches!(arguments[1], TypeExpression::TagGroup(_)));
    }

    #[test]
    fn arguments_can_have_trailing_comma() {
        let input = ParserInput::new("([Int], #true,) => Str");
        let (_, function) = function_type(input).unwrap();
        let FunctionTypeNode {
            value: FunctionTypeValue { arguments, .. },
            ..
        } = function;
        assert_eq!(arguments.len(), 2);
        assert!(matches!(arguments[0], TypeExpression::List(_)));
        assert!(matches!(arguments[1], TypeExpression::TagGroup(_)));
    }

    #[test]
    fn can_have_spaces_anywhere() {
        let input = ParserInput::new("(  [Int]  ,  #true  )  =>  Str");
        let result = function_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_have_newlines_anywhere() {
        let input = ParserInput::new("(\n[Int]\n,\n#true\n)\n=>\nStr");
        let result = function_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn spaces_are_not_required() {
        let input = ParserInput::new("([Int],#true)=>Str");
        let result = function_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_have_spaces_anywhere_with_trailing_comma() {
        let input = ParserInput::new("(  [Int]  ,  #true  ,  )  =>  Str");
        let result = function_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_have_newlines_anywhere_with_trailing_comma() {
        let input = ParserInput::new("(\n[Int]\n,\n#true\n,\n)\n=>\nStr");
        let result = function_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn spaces_are_not_required_with_trailing_comma() {
        let input = ParserInput::new("([Int],#true,)=>Str");
        let result = function_type(input);
        assert!(result.is_ok());
    }
}
