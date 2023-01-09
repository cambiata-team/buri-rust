use crate::{
    identifier::identifier, integer::integer, list::list, parentheses::parentheses, record::record,
    string_literal::string_literal, tag::tag, unary_operator::unary_operator_expression,
    ExpressionContext,
};
use ast::{Expression, IResult, ParserInput};
use nom::{branch::alt, combinator::map};

pub fn basic_expression<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    alt((
        parentheses,
        map(
            move |input| unary_operator_expression(context, input),
            Expression::UnaryOperator,
        ),
        map(identifier, Expression::Identifier),
        map(integer, Expression::Integer),
        map(string_literal, Expression::StringLiteral),
        map(list, Expression::List),
        map(record, Expression::Record),
        map(tag, Expression::Tag),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expression_can_be_identifier() {
        let input = ParserInput::new("hello");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::Identifier(_)));
    }

    #[test]
    fn expression_can_be_integer() {
        let input = ParserInput::new("0");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::Integer(_)));
    }

    #[test]
    fn expression_can_be_string_literal() {
        let input = ParserInput::new("\"hello\"");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::StringLiteral(_)));
    }

    #[test]
    fn expression_can_be_list() {
        let input = ParserInput::new("[\"hello\"]");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::List(_)));
    }

    #[test]
    fn expression_can_be_a_record() {
        let input = ParserInput::new("{a: 0}");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::Record(_)));
    }

    #[test]
    fn expression_can_be_a_tag() {
        let input = ParserInput::new("#hello");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::Tag(_)));
    }
}
