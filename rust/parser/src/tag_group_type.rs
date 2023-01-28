use ast::TagGroupTypeNode;
use ast::{IResult, ParserInput};
use nom::{
    character::complete::char,
    combinator::{consumed, map, opt},
    multi::separated_list1,
    sequence::tuple,
};

use crate::{
    intra_expression_whitespace::intra_expression_whitespace, tag_type::tag_type, ExpressionContext,
};

pub fn tag_group_type(input: ParserInput) -> IResult<TagGroupTypeNode> {
    map(
        consumed(separated_list1(
            tuple((
                opt(intra_expression_whitespace(ExpressionContext::new())),
                char('|'),
                opt(intra_expression_whitespace(
                    ExpressionContext::new().allow_newlines_in_expressions(),
                )),
            )),
            tag_type,
        )),
        |(consumed, tags)| TagGroupTypeNode {
            value: tags,
            source: consumed,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors_when_no_tags() {
        let input = ParserInput::new("");
        let result = tag_group_type(input);
        assert!(result.is_err());
    }

    #[test]
    fn parses_tag_identifier() {
        let input = ParserInput::new("#hello");
        let (_, parsed) = tag_group_type(input).unwrap();
        assert_eq!(parsed.value.len(), 1);
        assert_eq!(parsed.value[0].value.name.value, "hello".to_string());
    }

    #[test]
    fn parses_multiple_tag_identifiers() {
        let input = ParserInput::new("#hello | #world");
        let (_, parsed) = tag_group_type(input).unwrap();
        assert_eq!(parsed.value.len(), 2);
        assert_eq!(parsed.value[0].value.name.value, "hello".to_string());
        assert_eq!(parsed.value[1].value.name.value, "world".to_string());
    }

    #[test]
    fn parses_tag_with_value() {
        let input = ParserInput::new("#hello(Str)");
        let (_, parsed) = tag_group_type(input).unwrap();
        assert_eq!(parsed.value.len(), 1);
    }

    #[test]
    fn parses_multiple_tags_with_values() {
        let input = ParserInput::new("#hello(Str) | #world(Str)");
        let (_, parsed) = tag_group_type(input).unwrap();
        assert_eq!(parsed.value.len(), 2);
    }

    #[test]
    fn does_not_require_spaces_between_tags() {
        let input = ParserInput::new("#hello|#world");
        let (_, parsed) = tag_group_type(input).unwrap();
        assert_eq!(parsed.value.len(), 2);
    }

    #[test]
    fn can_have_newline_after_pipe_character() {
        let input = ParserInput::new("#hello |\n#world");
        let (_, parsed) = tag_group_type(input).unwrap();
        assert_eq!(parsed.value.len(), 2);
    }

    #[test]
    fn can_include_comment_after_pipe_character() {
        let input = ParserInput::new("#hello | --hello world\n#world");
        let (_, parsed) = tag_group_type(input).unwrap();
        assert_eq!(parsed.value.len(), 2);
    }

    #[test]
    fn commas_are_not_parsed() {
        let input = ParserInput::new("#hello,");
        let (remainder, _) = tag_group_type(input).unwrap();
        assert_eq!(remainder, ",");
    }
}
