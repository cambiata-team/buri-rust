use crate::{
    function::function, identifier::identifier, integer::integer, list::list,
    parentheses::parentheses, record::record, record_assignment::record_assignment,
    string_literal::string_literal, tag::tag, type_declaration::type_declaration,
    unary_operator::unary_operator_expression, variable_declaration::variable_declaration,
    ExpressionContext,
};
use ast::{Expression, IResult, ParserInput};
use nom::{branch::alt, combinator::map};

pub fn basic_expression<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Expression<'a>> {
    alt((
        map(move |input| function(context, input), Expression::Function),
        parentheses,
        map(type_declaration, Expression::TypeDeclaration),
        map(variable_declaration, Expression::Declaration),
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
        map(
            move |input| record_assignment(context, input),
            Expression::RecordAssignment,
        ),
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

    #[test]
    fn expression_can_be_a_function() {
        let input = ParserInput::new("() => 42");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::Function(_)));
    }

    #[test]
    fn expression_can_be_a_function_with_arguments() {
        let input = ParserInput::new("(a) => a");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::Function(_)));
    }

    #[test]
    fn functions_can_access_argument_fields() {
        let input = ParserInput::new("(a) => a.b");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::Function(_)));
    }

    #[test]
    fn expression_can_be_a_record_assignment() {
        let input = ParserInput::new("{hello|name:\"world\"}");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::RecordAssignment(_)));
    }

    #[test]
    fn declarations_take_precedence_over_other_identifiers() {
        let input = ParserInput::new("hello = \"hello\"");
        let result =
            basic_expression(ExpressionContext::new().allow_newlines_in_expressions())(input);
        let (_, consumed) = result.unwrap();
        assert!(matches!(consumed, Expression::Declaration(_)));
    }
}
