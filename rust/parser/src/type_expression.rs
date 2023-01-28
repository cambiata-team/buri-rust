use crate::tag_group_type::tag_group_type;
use crate::{
    function_type::function_type, list_type::list_type, record_type::record_type,
    type_identifier::type_identifier,
};
use ast::TypeExpression;
use ast::{IResult, ParserInput};
use nom::{branch::alt, combinator::map};

pub fn type_expression(input: ParserInput) -> IResult<TypeExpression> {
    alt((
        map(type_identifier, TypeExpression::Identifier),
        map(list_type, |list| TypeExpression::List(Box::new(list))),
        map(tag_group_type, TypeExpression::TagGroup),
        map(record_type, TypeExpression::Record),
        map(function_type, TypeExpression::Function),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_function_type_is_a_type_expression() {
        let input = ParserInput::new("(Str) => Str");
        let (_, expression) = type_expression(input.clone()).unwrap();
        assert!(matches!(expression, TypeExpression::Function(_)));
    }

    #[test]
    fn a_type_identifier_is_a_type_expression() {
        let input = ParserInput::new("Hello");
        let (_, expression) = type_expression(input.clone()).unwrap();
        assert!(matches!(expression, TypeExpression::Identifier(_)));
    }

    #[test]
    fn a_list_type_is_a_type_expression() {
        let input = ParserInput::new("[Str]");
        let (_, expression) = type_expression(input.clone()).unwrap();
        assert!(matches!(expression, TypeExpression::List(_)));
    }

    #[test]
    fn a_single_tag_type_is_a_type_expression() {
        let input = ParserInput::new("#hello");
        let (_, expression) = type_expression(input.clone()).unwrap();
        assert!(matches!(expression, TypeExpression::TagGroup(_)));
    }

    #[test]
    fn a_tag_group_type_is_a_type_expression() {
        let input = ParserInput::new("#hello | #world");
        let (_, expression) = type_expression(input.clone()).unwrap();
        assert!(matches!(expression, TypeExpression::TagGroup(_)));
    }

    #[test]
    fn a_record_type_is_a_type_expression() {
        let input = ParserInput::new("{ hello: Str }");
        let (_, expression) = type_expression(input.clone()).unwrap();
        assert!(matches!(expression, TypeExpression::Record(_)));
    }
}
