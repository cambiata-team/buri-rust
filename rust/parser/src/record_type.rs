use crate::{
    identifier::identifier, intra_expression_whitespace::intra_expression_whitespace,
    type_expression::type_expression, ExpressionContext,
};
use ast::{IResult, ParserInput};
use ast::{RecordTypeNode, RecordTypeValue};
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{consumed, map, opt},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
};

pub fn record_type(input: ParserInput) -> IResult<RecordTypeNode> {
    map(
        consumed(delimited(
            tuple((
                char('{'),
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
                map(
                    separated_pair(
                        identifier,
                        tuple((
                            opt(intra_expression_whitespace(
                                ExpressionContext::new().allow_newlines_in_expressions(),
                            )),
                            tag(":"),
                            opt(intra_expression_whitespace(
                                ExpressionContext::new().allow_newlines_in_expressions(),
                            )),
                        )),
                        type_expression,
                    ),
                    |(key, value)| RecordTypeValue {
                        identifier: key,
                        value,
                    },
                ),
            ),
            tuple((
                opt(intra_expression_whitespace(
                    ExpressionContext::new().allow_newlines_in_expressions(),
                )),
                opt(char(',')),
                opt(intra_expression_whitespace(
                    ExpressionContext::new().allow_newlines_in_expressions(),
                )),
                char('}'),
            )),
        )),
        |(consumed, kv_pairs)| RecordTypeNode {
            value: kv_pairs,
            source: consumed,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::{Expression, TypeExpression};

    #[test]
    fn empty_record_produces_empty_output() {
        let input = ParserInput::new("{}");
        let (_, value) = record_type(input).unwrap();
        assert_eq!(value.value, Vec::new());
    }

    #[test]
    fn empty_record_is_fully_consumed() {
        let input = ParserInput::new("{}");
        let (consumed, _) = record_type(input).unwrap();
        assert!(consumed.is_empty());
    }

    #[test]
    fn consumed_record_keeps_original_input() {
        let input = ParserInput::new("{}");
        let (_, value) = record_type(input.clone()).unwrap();
        assert_eq!(value.source, input);
    }

    #[test]
    fn empty_record_can_contain_spaces() {
        let input = ParserInput::new("{   }");
        let result = record_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn empty_record_can_contain_line_breaks() {
        let input = ParserInput::new("{\n}");
        let result = record_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn empty_record_can_contain_comments() {
        let input = ParserInput::new("{\n-- I'm a comment\n-- I'm another comment\n}");
        let result = record_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn correctly_parses_identifiers_for_a_single_field() {
        let input = ParserInput::new("{ foo: Str }");
        let (_, value) = record_type(input).unwrap();
        assert_eq!(value.value.len(), 1);
        assert_eq!(value.value[0].identifier.value.name, "foo");
    }

    #[test]
    fn correctly_parses_identifiers_for_multiple_fields() {
        let input = ParserInput::new("{ foo: Str, bar: Str }");
        let (_, value) = record_type(input).unwrap();
        assert_eq!(value.value.len(), 2);
        assert_eq!(value.value[0].identifier.value.name, "foo");
        assert_eq!(value.value[1].identifier.value.name, "bar");
    }

    #[test]
    fn correctly_parses_values_for_a_single_field() {
        let input = ParserInput::new("{ foo: Str }");
        let (_, value) = record_type(input).unwrap();
        assert_eq!(value.value.len(), 1);
        assert!(matches!(
            value.value[0].value,
            TypeExpression::Identifier(_)
        ));
    }

    #[test]
    fn correctly_parses_values_for_multiple_fields() {
        let input = ParserInput::new("{ foo: Str, bar: [Nat] }");
        let (_, value) = record_type(input).unwrap();
        assert_eq!(value.value.len(), 2);
        assert!(matches!(
            value.value[0].value,
            TypeExpression::Identifier(_)
        ));
        assert!(matches!(value.value[1].value, TypeExpression::List(_)));
    }

    #[test]
    fn does_not_need_spaces() {
        let input = ParserInput::new("{foo:Str,bar:[Nat]}");
        let result = record_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_have_spaces_anywhere() {
        let input = ParserInput::new("{  foo  :  Str  ,  bar  :  [Nat]  }");
        let result = record_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_have_newlines_anywhere() {
        let input = ParserInput::new("{\nfoo\n:\nStr\n,\nbar\n:\n[Nat]\n}");
        let result = record_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_have_trailing_comma() {
        let input = ParserInput::new("{ foo: Str, bar: [Nat], }");
        let result = record_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn trailing_comma_does_not_add_extra_kv_pair() {
        let input = ParserInput::new("{ foo: Str, bar: [Nat], }");
        let (_, value) = record_type(input).unwrap();
        assert_eq!(value.value.len(), 2);
    }

    #[test]
    fn can_have_comments_anywhere() {
        let input = ParserInput::new(
            "{-- hello\n--hello\nfoo--hello\n:--hello\nStr--hello\n,--hello\nbar--hello\n:--hello\n[Nat]--hello\n-- I'm another comment\n}",
        );
        let result = record_type(input);
        assert!(result.is_ok());
    }

    #[test]
    fn key_identifiers_do_not_include_comments() {
        let input = ParserInput::new("{foo--hello\n: Str}");
        let (_, value) = record_type(input).unwrap();
        assert_eq!(value.value.len(), 1);
        assert_eq!(value.value[0].identifier.value.name, "foo");
    }
}
