use crate::{
    intra_expression_whitespace::intra_expression_whitespace, type_expression::type_expression,
    type_identifier::type_identifier, ExpressionContext,
};
use ast::{IResult, ParserInput};
use ast::{TypeDeclarationNode, TypeDeclarationValue};
use nom::{
    character::complete::char,
    combinator::{consumed, map, opt},
    sequence::{separated_pair, tuple},
};

pub fn type_declaration(input: ParserInput) -> IResult<TypeDeclarationNode> {
    map(
        consumed(separated_pair(
            type_identifier,
            tuple((
                opt(intra_expression_whitespace(ExpressionContext::new())),
                char('='),
                opt(intra_expression_whitespace(
                    ExpressionContext::new().allow_newlines_in_expressions(),
                )),
            )),
            type_expression,
        )),
        |(consumed, (identifier, type_expression))| TypeDeclarationNode {
            value: TypeDeclarationValue {
                identifier,
                type_expression: Box::new(type_expression),
            },
            source: consumed,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::{TypeExpression, TypeIdentifierNode};

    #[test]
    fn parses_identifier() {
        let input = ParserInput::new("Hello = World");
        let (_, declaration) = type_declaration(input.clone()).unwrap();
        assert!(matches!(
            declaration.value,
            TypeDeclarationValue {
                identifier: TypeIdentifierNode { .. },
                ..
            }
        ));
    }

    #[test]
    fn parses_type_expression() {
        let input = ParserInput::new("Hello = World");
        let (_, declaration) = type_declaration(input.clone()).unwrap();
        assert!(matches!(
            *declaration.value.type_expression,
            TypeExpression::Identifier(_),
        ));
    }

    #[test]
    fn errors_when_using_invalid_type_identifier() {
        let input = ParserInput::new("hello = World");
        let result = type_declaration(input.clone());
        assert!(result.is_err());
    }

    #[test]
    fn parses_without_space_between_identifier_and_equal_sign() {
        let input = ParserInput::new("Hello= World");
        let result = type_declaration(input.clone());
        assert!(result.is_ok());
    }

    #[test]
    fn parses_without_space_between_equal_sign_and_type_expression() {
        let input = ParserInput::new("Hello =World");
        let result = type_declaration(input.clone());
        assert!(result.is_ok());
    }

    #[test]
    fn newline_before_equal_sign_errors() {
        let input = ParserInput::new("Hello\n= World");
        let result = type_declaration(input.clone());
        assert!(result.is_err());
    }

    #[test]
    fn can_use_newline_after_equal_sign() {
        let input = ParserInput::new("Hello =\nWorld");
        let result = type_declaration(input.clone());
        assert!(result.is_ok());
    }
}
