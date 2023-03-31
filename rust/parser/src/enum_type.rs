use crate::{
    enum_literal::variant_name, intra_expression_whitespace::intra_expression_whitespace,
    type_expression::type_expression, ExpressionContext,
};
use ast::{
    EnumTypeNode, EnumTypeValue, EnumVariantTypeNode, EnumVariantTypeValue, IResult, ParserInput,
    TypeExpression,
};

use nom::{
    bytes::complete::tag,
    combinator::{consumed, map, opt, verify},
    multi::separated_list1,
    sequence::{delimited, tuple},
};

fn payload<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, Vec<TypeExpression>> {
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
            verify(type_expression, |type_expression| {
                !matches!(type_expression, TypeExpression::Enum(_))
            }),
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

fn enum_variant<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, EnumVariantTypeNode<'a>> {
    move |input| {
        map(
            consumed(tuple((tag("."), variant_name, opt(payload(context))))),
            |(input, (_, name, payload))| EnumVariantTypeNode {
                source: input,
                value: EnumVariantTypeValue {
                    variant_name: name.value().to_owned(),
                    payload: payload.map_or_else(Vec::new, |payload| payload),
                },
            },
        )(input)
    }
}

pub fn enum_type<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, EnumTypeNode<'a>> {
    map(
        consumed(separated_list1(
            tuple((
                opt(intra_expression_whitespace(
                    context.allow_newlines_in_expressions(),
                )),
                tag("|"),
                opt(intra_expression_whitespace(
                    context.allow_newlines_in_expressions(),
                )),
            )),
            enum_variant(context),
        )),
        |(input, variants)| EnumTypeNode {
            source: input,
            value: EnumTypeValue { variants },
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_input_is_not_an_enum_type() {
        let input = ParserInput::new("");
        let result = enum_type(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn identifier_is_not_an_enum_type() {
        let input = ParserInput::new("a");
        let result = enum_type(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn period_is_not_an_enum_type() {
        let input = ParserInput::new(".");
        let result = enum_type(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn period_followed_by_identifier_is_enum_type() {
        let input = ParserInput::new(".a");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn identifier_followed_by_a_period_is_not_an_enum_type() {
        let input = ParserInput::new("a.");
        let result = enum_type(ExpressionContext::new())(input);
        assert!(result.is_err());
    }

    #[test]
    fn variant_name_is_preserved() {
        let input = ParserInput::new(".a");
        let result = enum_type(ExpressionContext::new())(input);
        let (_, node) = result.unwrap();
        assert_eq!(node.value.variants.get(0).unwrap().value.variant_name, "a");
    }

    #[test]
    fn empty_payload_stops_parsing_before_payload() {
        let input = ParserInput::new(".a()");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "()");
    }

    #[test]
    fn empty_payload_with_comma_stops_parsing_before_payload() {
        let input = ParserInput::new(".a(,)");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "(,)");
    }

    #[test]
    fn one_payload_parses() {
        let input = ParserInput::new(".a(Int)");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn two_payloads_parse() {
        let input = ParserInput::new(".a(Int,Int)");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_trailing_comma_with_one_payload() {
        let input = ParserInput::new(".a(Int,)");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_trailing_comma_with_two_payloads() {
        let input = ParserInput::new(".a(Int,Int,)");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_spaces_within_payload() {
        let input = ParserInput::new(".a(   Int   ,   Int   )");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_newlines_within_payload() {
        let input = ParserInput::new(".a(\nInt\n,\nInt\n)");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn payload_value_is_preserved() {
        let input = ParserInput::new(".a(Int)");
        let result = enum_type(ExpressionContext::new())(input);
        let (_, node) = result.unwrap();
        match node
            .value
            .variants
            .get(0)
            .unwrap()
            .value
            .payload
            .get(0)
            .unwrap()
        {
            TypeExpression::Identifier(identifier_node) => {
                assert_eq!(identifier_node.value, "Int");
            }
            _ => panic!("Payload value not preserved"),
        };
    }

    #[test]
    fn can_have_two_variants() {
        let input = ParserInput::new(".a | .b");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn can_have_two_variants_without_spaces() {
        let input = ParserInput::new(".a|.b");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn enum_payload_stops_parsing_before_payload() {
        let input = ParserInput::new(".a(.b)");
        let result = enum_type(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "(.b)");
    }

    #[test]
    fn enum_variant_name_may_not_contain_non_ascii_characters() {
        let input = ParserInput::new(".π");
        let result = enum_type(ExpressionContext::new())(input);
        assert!(result.is_err());
    }
}
