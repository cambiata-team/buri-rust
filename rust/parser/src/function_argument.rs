use crate::{identifier::identifier, type_identifier::type_identifier};
use ast::{FunctionArgumentNode, FunctionArgumentValue, IResult, ParserInput};
use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::{consumed, map, opt},
    sequence::{preceded, tuple},
};

pub fn function_argument(input: ParserInput) -> IResult<FunctionArgumentNode> {
    map(
        consumed(tuple((
            identifier,
            opt(preceded(tuple((space0, tag(":"), space0)), type_identifier)),
        ))),
        |(source, (name, maybe_type))| FunctionArgumentNode {
            source,
            value: FunctionArgumentValue {
                argument_name: name,
                argument_type: maybe_type,
            },
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string_is_not_function_argument() {
        let input = ParserInput::new("");
        let result = function_argument(input);
        assert!(result.is_err());
    }

    #[test]
    fn identifier_is_function_argument() {
        let input = ParserInput::new("hello");
        let result = function_argument(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn identifier_then_colon_then_type_is_function_argument() {
        let input = ParserInput::new("hello:World");
        let result = function_argument(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn spaces_can_be_added_around_colon() {
        let input = ParserInput::new("hello   :   World");
        let result = function_argument(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn argument_name_is_recorded() {
        let input = ParserInput::new("hello:World");
        let result = function_argument(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.argument_name.value.name, "hello");
    }

    #[test]
    fn argument_type_is_recorded() {
        let input = ParserInput::new("hello:World");
        let result = function_argument(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.argument_type.unwrap().value, "World");
    }

    #[test]
    fn argument_type_is_none_if_not_present() {
        let input = ParserInput::new("hello");
        let result = function_argument(input);
        let (_, parsed) = result.unwrap();
        assert!(parsed.value.argument_type.is_none());
    }
}
