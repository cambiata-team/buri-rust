use crate::{
    expression, intra_expression_whitespace::intra_expression_whitespace,
    type_identifier::type_identifier, ExpressionContext,
};
use ast::{EnumLiteralNode, EnumLiteralValue, Expression, IResult, ParserInput};

use nom::{
    bytes::complete::{tag, take_while1},
    combinator::{consumed, map, opt, recognize, verify},
    multi::separated_list1,
    sequence::{delimited, tuple},
};

fn variant_name(input: ParserInput) -> IResult<ParserInput> {
    verify(
        recognize(take_while1(|character: char| {
            character.is_ascii_alphanumeric() || character == '_'
        })),
        |parser_input: &ParserInput| {
            let string = parser_input.value();
            !(string.starts_with('_') || string.ends_with('_'))
        },
    )(input)
}

fn payload<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Vec<Expression>> {
    delimited(
        tuple((
            opt(intra_expression_whitespace(context)),
            tag("("),
            opt(intra_expression_whitespace(
                context.allow_newlines_in_expressions(),
            )),
        )),
        separated_list1(
            tuple((
                opt(intra_expression_whitespace(
                    context.allow_newlines_in_expressions(),
                )),
                tag(","),
                opt(intra_expression_whitespace(
                    context.allow_newlines_in_expressions(),
                )),
            )),
            expression(context),
        ),
        tuple((
            opt(intra_expression_whitespace(
                context.allow_newlines_in_expressions(),
            )),
            opt(tag(",")),
            opt(intra_expression_whitespace(
                context.allow_newlines_in_expressions(),
            )),
            tag(")"),
        )),
    )
}

pub fn enum_literal<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, EnumLiteralNode<'a>> {
    move |input| {
        map(
            consumed(tuple((
                type_identifier,
                tag("."),
                variant_name,
                opt(payload(context)),
            ))),
            |(input, (qualifier, _, name, payload))| EnumLiteralNode {
                source: input,
                value: EnumLiteralValue {
                    qualifier,
                    variant_name: name.value().to_owned(),
                    payload: payload.map_or_else(Vec::new, |payload| payload),
                },
            },
        )(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_input_is_not_an_enum_literal() {
        let input = ParserInput::new("");
        let result = enum_literal(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn identifier_is_not_an_enum_literal() {
        let input = ParserInput::new("a");
        let result = enum_literal(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn type_identifier_is_not_an_enum_literal() {
        let input = ParserInput::new("A");
        let result = enum_literal(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn period_is_not_an_enum_literal() {
        let input = ParserInput::new(".");
        let result = enum_literal(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn qualified_enum_is_an_enum_literal() {
        let input = ParserInput::new("A.a");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn unqualified_enum_is_not_an_enum_literal() {
        let input = ParserInput::new(".a");
        let result = enum_literal(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn qualifier_is_preserved() {
        let input = ParserInput::new("A.a");
        let result = enum_literal(ExpressionContext::new())(input);
        let (_, enum_literal) = result.unwrap();
        assert_eq!(enum_literal.value.qualifier.value, "A");
    }

    #[test]
    fn variant_name_is_preserved() {
        let input = ParserInput::new("A.a");
        let result = enum_literal(ExpressionContext::new())(input);
        let (_, enum_literal) = result.unwrap();
        assert_eq!(enum_literal.value.variant_name, "a");
    }

    #[test]
    fn empty_payload_stops_parsing_before_payload() {
        let input = ParserInput::new("A.a()");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "()");
    }

    #[test]
    fn empty_payload_with_comma_stops_parsing_before_payload() {
        let input = ParserInput::new("A.a(,)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "(,)");
    }

    #[test]
    fn one_payload_parses() {
        let input = ParserInput::new("A.a(1)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn two_payloads_parse() {
        let input = ParserInput::new("A.a(1,2)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_trailing_comma_with_one_payload() {
        let input = ParserInput::new("A.a(1,)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_trailing_comma_with_two_payloads() {
        let input = ParserInput::new("A.a(1,2,)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_spaces_within_payload() {
        let input = ParserInput::new("A.a(   1   ,   2   )");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_newlines_within_payload() {
        let input = ParserInput::new("A.a(\n1\n,\n2\n)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn payload_value_is_preserved() {
        let input = ParserInput::new("A.a(1)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (_, enum_literal) = result.unwrap();
        assert!(matches!(
            enum_literal.value.payload.get(0),
            Some(Expression::Integer(_))
        ));
    }

    #[test]
    fn can_parse_payloads_other_than_integers() {
        let input = ParserInput::new("A.a(\"HelloWorld\")");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_preserve_payloads_other_than_integers() {
        let input = ParserInput::new("A.a(\"HelloWorld\")");
        let result = enum_literal(ExpressionContext::new())(input);
        let (_, enum_literal) = result.unwrap();
        assert!(matches!(
            enum_literal.value.payload.get(0),
            Some(Expression::StringLiteral(_))
        ));
    }

    #[test]
    fn can_parse_enum_payloads() {
        let input = ParserInput::new("A.a(B.b)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_preserve_enum_payloads() {
        let input = ParserInput::new("A.a(B.b)");
        let result = enum_literal(ExpressionContext::new())(input);
        let (_, enum_literal) = result.unwrap();
        assert!(matches!(
            enum_literal.value.payload.get(0),
            Some(Expression::EnumLiteral(_))
        ));
    }
}
