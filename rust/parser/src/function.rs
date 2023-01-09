use crate::{
    block::block, expression, function_argument::function_argument,
    intra_expression_whitespace::intra_expression_whitespace, newline::newline,
    type_identifier::type_identifier, ExpressionContext,
};
use ast::{Expression, FunctionArgumentNode, FunctionNode, FunctionValue, TypeIdentifierNode};
use ast::{IResult, ParserInput};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{consumed, map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
};

fn argument_list<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Vec<FunctionArgumentNode<'a>>> {
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
            function_argument,
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
    )
}

fn function_return_type<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<TypeIdentifierNode<'a>> {
    preceded(
        tuple((
            opt(intra_expression_whitespace(
                context.allow_newlines_in_expressions(),
            )),
            tag(":"),
            opt(intra_expression_whitespace(
                context.allow_newlines_in_expressions(),
            )),
        )),
        type_identifier,
    )
}

fn function_body<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    alt((
        preceded(
            opt(intra_expression_whitespace(context)),
            expression(context),
        ),
        preceded(
            tuple((space0, newline)),
            map(block(context.increment_indentation()), Expression::Block),
        ),
    ))
}

pub fn function<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, FunctionNode<'a>> {
    map(
        consumed(tuple((
            argument_list(context),
            opt(function_return_type(context)),
            preceded(opt(intra_expression_whitespace(context)), tag("=>")),
            function_body(context),
        ))),
        |(source, (arguments, return_type, _, body))| FunctionNode {
            source,
            value: FunctionValue {
                arguments,
                return_type,
                body: Box::new(body),
            },
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::IntegerNode;

    #[test]
    fn empty_string_is_not_function() {
        let input = ParserInput::new("");
        let result = function(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn trivial_constant_function_parses() {
        let input = ParserInput::new("()=>0");
        let result = function(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn missing_argument_list_errors() {
        let input = ParserInput::new("=>0");
        let result = function(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_arrow_errors() {
        let input = ParserInput::new("()0");
        let result = function(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn missing_body_errors() {
        let input = ParserInput::new("()=>");
        let result = function(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn one_argument_parses() {
        let input = ParserInput::new("(hello)=>0");
        let result = function(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn two_arguments_parse() {
        let input = ParserInput::new("(hello,world)=>0");
        let result = function(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn types_can_be_added_to_arguments() {
        let input = ParserInput::new("(hello:A,world:B)=>0");
        let result = function(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn argument_name_is_recorded() {
        let input = ParserInput::new("(hello)=>0");
        let result = function(ExpressionContext::new())(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(
            parsed
                .value
                .arguments
                .get(0)
                .unwrap()
                .value
                .argument_name
                .value
                .name,
            "hello"
        );
    }

    #[test]
    fn function_body_is_recorded() {
        let input = ParserInput::new("(hello)=>0");
        let result = function(ExpressionContext::new())(input);
        let (_, parsed) = result.unwrap();
        assert!(matches!(
            *(parsed.value.body),
            Expression::Integer(IntegerNode { value: 0, .. })
        ));
    }

    #[test]
    fn function_body_can_be_nontrivial() {
        let input = ParserInput::new("(x,y)=>if x < y do x + y else x / y");
        let result = function(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_body_can_be_block() {
        let input = ParserInput::new("(x,y)=>\n    1 + 2\n    x - y\n");
        let result = function(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn spaces_can_be_used_around_arguments_and_arrow() {
        let input = ParserInput::new("(  hello  :  A  ,  world  :  B  )  =>  0");
        let result = function(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }
}
