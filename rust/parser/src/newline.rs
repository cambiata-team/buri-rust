use ast::{IResult, ParserInput};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, none_of},
    combinator::{eof, recognize, verify},
    multi::many0_count,
    sequence::tuple,
};

fn comment(input: ParserInput) -> IResult<ParserInput> {
    recognize(tuple((
        tag("--"),
        many0_count(verify(none_of(""), |character| match character {
            // forbid control characters
            c if c.is_control() => false,
            // forbid text direction marks
            '\u{061C}'
            | '\u{2066}'..='\u{2069}'
            | '\u{200E}'..='\u{200F}'
            | '\u{202A}'..='\u{202E}' => false,
            _ => true,
        })),
    )))(input)
}

/// Parses a newline and any trailing comment.
pub fn newline(input: ParserInput) -> IResult<ParserInput> {
    alt((
        recognize(tuple((comment, alt((line_ending, eof))))),
        line_ending,
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string_errors() {
        let input = ParserInput::new("");
        let result = newline(input);
        assert!(result.is_err());
    }

    #[test]
    fn parses_a_newline() {
        let input = ParserInput::new("\n");
        let (remainder, _) = newline(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn parses_a_carriage_return() {
        let input = ParserInput::new("\r\n");
        let (remainder, _) = newline(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn parses_a_comment_ending_in_a_newline() {
        let input = ParserInput::new("-- Hello\n");
        let (remainder, _) = newline(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn parses_a_comment_ending_in_a_carriage_return() {
        let input = ParserInput::new("-- Hello\r\n");
        let (remainder, _) = newline(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn parses_a_comment_ending_in_an_eof() {
        let input = ParserInput::new("-- Hello");
        let (remainder, _) = newline(input).unwrap();
        assert!(remainder.is_empty());
    }
}
