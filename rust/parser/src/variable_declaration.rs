use crate::{
    binary_operator_expression::binary_operator_expression, identifier::identifier,
    intra_expression_whitespace::intra_expression_whitespace, type_expression::type_expression,
    ExpressionContext,
};
use ast::{DeclarationNode, DeclarationValue};
use ast::{IResult, ParserInput};
use nom::{
    character::complete::{char, space0},
    combinator::{consumed, map, opt},
    sequence::{preceded, separated_pair, tuple},
};

pub fn variable_declaration(input: ParserInput) -> IResult<DeclarationNode> {
    map(
        consumed(separated_pair(
            tuple((
                identifier,
                opt(preceded(
                    tuple((space0, char(':'), space0)),
                    type_expression,
                )),
            )),
            tuple((
                space0,
                char('='),
                opt(intra_expression_whitespace(
                    ExpressionContext::new().allow_newlines_in_expressions(),
                )),
            )),
            binary_operator_expression(ExpressionContext::new().allow_newlines_in_expressions()),
        )),
        |(consumed, ((identifier, type_expression), expression))| DeclarationNode {
            value: DeclarationValue {
                identifier,
                type_expression,
                expression: Box::new(expression),
            },
            source: consumed,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_variable_name() {
        let input = "foo = 1";
        let input = ParserInput::new(input);
        let (_, node) = variable_declaration(input).unwrap();
        assert_eq!(node.value.identifier.value.name, "foo");
    }

    #[test]
    fn missing_type_expression_becomes_none() {
        let input = "foo = 1";
        let input = ParserInput::new(input);
        let (_, node) = variable_declaration(input).unwrap();
        assert_eq!(node.value.type_expression, None);
    }

    #[test]
    fn parses_variable_value() {
        let input = "foo = 1";
        let input = ParserInput::new(input);
        let (_, node) = variable_declaration(input).unwrap();
        assert!(matches!(
            *node.value.expression,
            ast::Expression::Integer(_)
        ));
    }

    #[test]
    fn parses_variable_type() {
        let input = "foo: Int = 1";
        let input = ParserInput::new(input);
        let (_, node) = variable_declaration(input).unwrap();
        assert!(matches!(
            node.value.type_expression,
            Some(ast::TypeExpression::Identifier(_))
        ));
    }

    #[test]
    fn parses_with_spaces_anywhere() {
        let input = "foo : Int = 1";
        let input = ParserInput::new(input);
        let (remainder, _) = variable_declaration(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn parses_with_newline_after_equal_sign() {
        let input = "foo: Int =\n1";
        let input = ParserInput::new(input);
        let (remainder, _) = variable_declaration(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn errors_with_newline_before_colon() {
        let input = "foo\n: Int = 1";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_with_newline_after_colon() {
        let input = "foo: \nInt = 1";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_with_newline_before_equal_sign() {
        let input = "foo: Int\n= 1";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_with_newline_before_equal_sign_and_no_type() {
        let input = "foo\n= 1";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        assert!(result.is_err());
    }

    #[test]
    fn does_not_require_spaces() {
        let input = "foo:Int=1";
        let input = ParserInput::new(input);
        let (remainder, _) = variable_declaration(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn does_not_require_spaces_when_untyped() {
        let input = "foo=1";
        let input = ParserInput::new(input);
        let (remainder, _) = variable_declaration(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn can_have_comment_after_equal_sign() {
        let input = "foo: Int = -- comment\n1";
        let input = ParserInput::new(input);
        let (remainder, _) = variable_declaration(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn errors_without_value() {
        let input = "foo: Int =";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_without_identifier() {
        let input = ": Int = 1";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_without_identifier_or_type() {
        let input = "= 1";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_without_equal_sign() {
        let input = "foo: Int 1";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        assert!(result.is_err());
    }

    #[test]
    fn right_hand_side_can_be_identifier() {
        let input = "foo = bar";
        let input = ParserInput::new(input);
        let result = variable_declaration(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }
}
