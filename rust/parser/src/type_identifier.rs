use ast::TypeIdentifierNode;
use ast::{IResult, ParsedNode, ParserInput};
use nom::{
    bytes::complete::{take, take_while},
    combinator::{map, recognize, verify},
    sequence::tuple,
};

pub fn type_identifier(input: ParserInput) -> IResult<TypeIdentifierNode> {
    map(
        recognize(tuple((
            verify(take(1_usize), |consumed: &ParserInput| {
                consumed
                    .value()
                    .chars()
                    .next()
                    .map_or(false, char::is_uppercase)
            }),
            take_while(|char: char| char == '_' || char.is_alphanumeric()),
        ))),
        |consumed: ParserInput| ParsedNode {
            value: consumed.value().to_string(),
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
        let result = type_identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_first_character_is_lowercase() {
        let input = ParserInput::new("hello");
        let result = type_identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_first_character_is_an_underscore() {
        let input = ParserInput::new("_Hello");
        let result = type_identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_first_character_is_a_number() {
        let input = ParserInput::new("1Hello");
        let result = type_identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn succeeds_when_first_character_is_uppercase() {
        let input = ParserInput::new("Hello");
        let (_, parsed_node) = type_identifier(input).unwrap();
        assert_eq!(parsed_node.value, "Hello");
    }

    #[test]
    fn can_include_numbers() {
        let input = ParserInput::new("Hello123");
        let (_, parsed_node) = type_identifier(input).unwrap();
        assert_eq!(parsed_node.value, "Hello123");
    }

    #[test]
    fn can_include_underscores() {
        let input = ParserInput::new("Hello_World");
        let (_, parsed_node) = type_identifier(input).unwrap();
        assert_eq!(parsed_node.value, "Hello_World");
    }
}
