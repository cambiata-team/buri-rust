use ast::TagIdentifierNode;
use ast::{IResult, ParserInput};
use nom::{
    bytes::complete::take_while,
    character::complete::char,
    combinator::{consumed, map, verify},
    sequence::preceded,
};

pub fn tag_identifier(input: ParserInput) -> IResult<TagIdentifierNode> {
    map(
        consumed(preceded(
            char('#'),
            verify(
                take_while(|character: char| character.is_ascii_alphanumeric() || character == '_'),
                |name: &ParserInput| !name.value().is_empty(),
            ),
        )),
        |(consumed, identifier): (ParserInput, ParserInput)| TagIdentifierNode {
            value: identifier.value().to_string(),
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
        let (_, parsed) = tag_identifier(input).unwrap();
        assert_eq!(parsed.value, "hello");
    }

    #[test]
    fn space_not_allowed_before_name() {
        let input = ParserInput::new("# hello");
        let result = tag_identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn name_can_start_with_uppercase() {
        let input = ParserInput::new("#Hello");
        let (_, parsed) = tag_identifier(input).unwrap();
        assert_eq!(parsed.value, "Hello".to_string());
    }

    #[test]
    fn name_can_include_underscores() {
        let input = ParserInput::new("#hello_world");
        let (_, parsed) = tag_identifier(input).unwrap();
        assert_eq!(parsed.value, "hello_world".to_string());
    }

    #[test]
    fn name_can_contain_uppercase() {
        let input = ParserInput::new("#helloWorld");
        let (_, parsed) = tag_identifier(input).unwrap();
        assert_eq!(parsed.value, "helloWorld".to_string());
    }

    #[test]
    fn name_can_start_with_number() {
        let input = ParserInput::new("#1hello");
        let (_, parsed) = tag_identifier(input).unwrap();
        assert_eq!(parsed.value, "1hello".to_string());
    }

    #[test]
    fn name_can_include_numbers() {
        let input = ParserInput::new("#404");
        let (_, parsed) = tag_identifier(input).unwrap();
        assert_eq!(parsed.value, "404".to_string());
    }

    #[test]
    fn name_cannot_include_spaces() {
        let input = ParserInput::new("#hello world");
        let (remainder, parsed) = tag_identifier(input).unwrap();
        assert!(!remainder.is_empty());
        assert_eq!(parsed.value, "hello".to_string());
    }

    #[test]
    fn tag_identifier_may_not_contain_non_ascii_characters() {
        let input = ParserInput::new("#π");
        let result = tag_identifier(input);
        assert!(result.is_err());
    }
}
