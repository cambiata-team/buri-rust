use crate::{
    binary_operator_expression::binary_operator_expression, if_statement::if_statement,
    ExpressionContext,
};
use ast::Expression;
use ast::{IResult, ParserInput};
use nom::{branch::alt, combinator::map};

pub fn binary_operator_or_if<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    alt((
        binary_operator_expression(context),
        map(if_statement(context), Expression::If),
    ))
}
