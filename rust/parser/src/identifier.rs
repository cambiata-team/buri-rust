use crate::is_keyword::is_keyword;
use ast::{IResult, ParsedNode, ParserInput};
use ast::{IdentifierNode, IdentifierValue};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::{map, recognize, verify},
    multi::{many0, many1},
    sequence::tuple,
};

pub fn identifier(input: ParserInput) -> IResult<IdentifierNode> {
    map(
        verify(
            alt((
                recognize(tuple((
                    many0(tag("_")),
                    take_while(|char: char| char == '_' || char.is_alphanumeric()),
                ))),
                recognize(many1(tag("_"))),
            )),
            |consumed: &ParserInput| {
                if consumed.value().is_empty() {
                    return false;
                }
                if is_keyword(consumed) {
                    return false;
                }
                if !consumed.value().starts_with('_') {
                    return consumed.chars().next().map_or(false, char::is_lowercase);
                }
                consumed
                    .value()
                    .trim_start_matches('_')
                    .chars()
                    .next()
                    .map_or(true, |char| !char.is_uppercase())
            },
        ),
        |consumed: ParserInput| ParsedNode {
            value: IdentifierValue {
                name: consumed.value().to_string(),
                is_disregarded: consumed.value().starts_with('_'),
            },
            source: consumed,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors_with_no_characters() {
        let input = ParserInput::new("");
        let result = identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_first_character_is_uppercase() {
        let input = ParserInput::new("Hello");
        let result = identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_first_character_after_underscores_is_uppercase() {
        let input = ParserInput::new("___Hello");
        let result = identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_first_character_is_a_number() {
        let input = ParserInput::new("1hello");
        let result = identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn first_character_after_underscores_can_be_a_number() {
        let input = ParserInput::new("___1hello");
        let (_, parsed_node) = identifier(input).unwrap();
        assert_eq!(parsed_node.value.name, "___1hello");
    }

    #[test]
    fn underscore_identifiers_are_disregarded() {
        let input = ParserInput::new("_");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(remainder.is_empty());
        assert!(parsed_node.value.is_disregarded);
    }

    #[test]
    fn can_have_multiple_underscores() {
        let input = ParserInput::new("____");
        let (remainder, _) = identifier(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn underscores_followed_by_letters_are_still_disregarded() {
        let input = ParserInput::new("__hello");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(remainder.is_empty());
        assert!(parsed_node.value.is_disregarded);
    }

    #[test]
    fn underscore_has_name_of_underscore() {
        let input = ParserInput::new("_");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(parsed_node.value.name, "_");
    }

    #[test]
    fn underscores_have_name_of_underscores() {
        let input = ParserInput::new("____");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(parsed_node.value.name, "____");
    }

    #[test]
    fn underscore_with_letters_has_name_of_underscores_plus_letters() {
        let input = ParserInput::new("_hello");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(parsed_node.value.name, "_hello");
    }

    #[test]
    fn can_have_underscores_mid_name() {
        let input = ParserInput::new("hello_world");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(parsed_node.value.name, "hello_world");
    }

    #[test]
    fn can_have_uppercase_letters() {
        let input = ParserInput::new("helloWorld");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(parsed_node.value.name, "helloWorld");
    }

    #[test]
    fn can_have_numbers() {
        let input = ParserInput::new("helloWorld123");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(parsed_node.value.name, "helloWorld123");
    }

    #[test]
    fn can_have_characters_after_identifier() {
        let input = ParserInput::new("foo:");
        let (remainder, parsed_node) = identifier(input).unwrap();
        assert!(!remainder.is_empty());
        assert_eq!(parsed_node.value.name, "foo");
    }

    #[test]
    fn keyword_is_not_identifier() {
        let input = ParserInput::new("else");
        let result = identifier(input);
        assert!(result.is_err());
    }
}
