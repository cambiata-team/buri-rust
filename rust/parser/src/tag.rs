use crate::{
    expression, intra_expression_whitespace::intra_expression_whitespace,
    tag_identifier::tag_identifier, ExpressionContext,
};
use ast::{IResult, ParserInput};
use ast::{TagNode, TagValue};
use nom::{
    character::complete::char,
    combinator::{consumed, map, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
};

pub fn tag(input: ParserInput) -> IResult<TagNode> {
    map(
        consumed(tuple((
            tag_identifier,
            opt(delimited(
                tuple((
                    char('('),
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
                    expression(ExpressionContext::new().allow_newlines_in_expressions()),
                ),
                tuple((
                    opt(intra_expression_whitespace(
                        ExpressionContext::new().allow_newlines_in_expressions(),
                    )),
                    char(')'),
                )),
            )),
        ))),
        |(consumed, (name, contents))| TagNode {
            value: TagValue {
                name,
                contents: contents.unwrap_or_default(),
            },
            source: consumed,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_tag_identifier() {
        let input = ParserInput::new("#hello");
        let (_, parsed) = tag(input).unwrap();
        assert_eq!(parsed.value.name.value, "hello".to_string());
    }

    #[test]
    fn tag_without_parenthesis_has_empty_contents() {
        let input = ParserInput::new("#hello");
        let (_, parsed) = tag(input).unwrap();
        assert!(parsed.value.contents.is_empty());
    }

    #[test]
    fn empty_contents_results_in_empty_array() {
        let input = ParserInput::new("#hello()");
        let (_, parsed) = tag(input).unwrap();
        assert!(parsed.value.contents.is_empty());
    }

    #[test]
    fn contents_can_be_a_single_expression() {
        let input = ParserInput::new("#hello(1)");
        let (_, parsed) = tag(input).unwrap();
        assert_eq!(parsed.value.contents.len(), 1);
        assert!(matches!(
            parsed.value.contents[0],
            ast::Expression::Integer(_)
        ));
    }

    #[test]
    fn contents_can_be_multiple_expressions() {
        let input = ParserInput::new("#hello(1, \"\", [])");
        let (_, parsed) = tag(input).unwrap();
        assert_eq!(parsed.value.contents.len(), 3);
        assert!(matches!(
            parsed.value.contents[0],
            ast::Expression::Integer(_)
        ));
        assert!(matches!(
            parsed.value.contents[1],
            ast::Expression::StringLiteral(_)
        ));
        assert!(matches!(parsed.value.contents[2], ast::Expression::List(_)));
    }

    #[test]
    fn contents_can_be_surrounded_by_spaces() {
        let input = ParserInput::new("#hello( 1 , \"\" )");
        let (_, parsed) = tag(input).unwrap();
        assert_eq!(parsed.value.contents.len(), 2);
    }

    #[test]
    fn contents_can_be_surrounded_by_newlines() {
        let input = ParserInput::new("#hello(\n1\n,\n\"\"\n)");
        let (_, parsed) = tag(input).unwrap();
        assert_eq!(parsed.value.contents.len(), 2);
    }

    #[test]
    fn contents_can_have_comments_anywhere() {
        let input =
            ParserInput::new("#hello(--comment\n1\n--comment\n,\n--comment\n\"\"\n--comment\n)");
        let (_, parsed) = tag(input).unwrap();
        assert_eq!(parsed.value.contents.len(), 2);
    }

    #[test]
    fn cannot_have_a_space_before_the_parenthesis() {
        let input = ParserInput::new("#hello (1)");
        let (remainder, parsed) = tag(input).unwrap();
        assert!(!remainder.is_empty());
        assert!(parsed.value.contents.is_empty());
    }
}
