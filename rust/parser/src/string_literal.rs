use ast::StringLiteralNode;
use ast::{IResult, ParsedNode, ParserInput};
use nom::{
    branch::alt,
    bytes::complete::escaped_transform,
    character::complete::{char, none_of},
    combinator::{consumed, map, success, value, verify},
    sequence::delimited,
};

/// Parse a string literal, bounded by double quotes.
/// Return a String with the contents of the string literal,
/// replacing all escape codes with their literal values.
pub fn string_literal(input: ParserInput) -> IResult<StringLiteralNode> {
    map(
        consumed(delimited(
            char('\"'),
            alt((
                escaped_transform(
                    verify(none_of("\\\""), |character| match character {
                        // forbid unescaped control characters
                        c if c.is_control() => false,
                        // forbid unescaped text direction marks
                        '\u{061C}'
                        | '\u{2066}'..='\u{2069}'
                        | '\u{200E}'..='\u{200F}'
                        | '\u{202A}'..='\u{202E}' => false,
                        _ => true,
                    }),
                    '\\',
                    alt((
                        char('\\'),
                        char('\"'),
                        value('\t', char('t')),
                        value('\n', char('n')),
                        value('\r', char('r')),
                    )),
                ),
                success(String::new()),
            )),
            char('\"'),
        )),
        |(consumed_input, produced_output)| ParsedNode {
            source: consumed_input,
            value: produced_output,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_input_is_not_a_string_literal() {
        let input = ParserInput::new("");
        let result = string_literal(input);
        assert!(result.is_err());
    }

    #[test]
    fn unpaired_double_quote_is_not_a_string_literal() {
        let input = ParserInput::new("\"");
        let result = string_literal(input);
        assert!(result.is_err());
    }

    #[test]
    fn paired_double_quote_returns_empty_string() {
        let input = ParserInput::new("\"\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed.source.value(), "\"\"");
        assert_eq!(consumed.value, "");
    }

    #[test]
    fn word_is_not_a_string_literal() {
        let input = ParserInput::new("hello");
        let result = string_literal(input);
        assert!(result.is_err());
    }

    #[test]
    fn unpaired_double_quote_followed_by_word_is_not_a_string_literal() {
        let input = ParserInput::new("\"hello");
        let result = string_literal(input);
        assert!(result.is_err());
    }

    #[test]
    fn word_within_paired_double_quotes_returns_that_word() {
        let input = ParserInput::new("\"hello\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed.source.value(), "\"hello\"");
        assert_eq!(consumed.value, "hello");
    }

    #[test]
    fn escaped_backslash_is_recognized() {
        let input = ParserInput::new("\"\\\\\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed.source.value(), "\"\\\\\"");
        assert_eq!(consumed.value, "\\");
    }

    #[test]
    fn escaped_double_quote_is_recognized() {
        let input = ParserInput::new("\"\\\"\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed.source.value(), "\"\\\"\"");
        assert_eq!(consumed.value, "\"");
    }

    #[test]
    fn escaped_tab_is_recognized() {
        let input = ParserInput::new("\"\\t\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed.source.value(), "\"\\t\"");
        assert_eq!(consumed.value, "\t");
    }

    #[test]
    fn escaped_lf_is_recognized() {
        let input = ParserInput::new("\"\\n\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed.source.value(), "\"\\n\"");
        assert_eq!(consumed.value, "\n");
    }

    #[test]
    fn escaped_cr_is_recognized() {
        let input = ParserInput::new("\"\\r\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed.source.value(), "\"\\r\"");
        assert_eq!(consumed.value, "\r");
    }

    #[test]
    fn backslash_with_no_escape_code_errors() {
        let input = ParserInput::new("\"\\\"");
        let result = string_literal(input);
        assert!(result.is_err());
    }

    #[test]
    fn backslash_with_invalid_escape_code_errors() {
        let input = ParserInput::new("\"\\y\"");
        let result = string_literal(input);
        assert!(result.is_err());
    }

    #[test]
    fn double_quote_without_backslash_prefix_stops_parsing_early() {
        let input = ParserInput::new("\"\"\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "\"");
        assert_eq!(consumed.source.value(), "\"\"");
        assert_eq!(consumed.value, "");
    }

    #[test]
    fn string_can_contain_non_ascii_characters() {
        let input = ParserInput::new("\"π\"");
        let result = string_literal(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed.source.value(), "\"π\"");
        assert_eq!(consumed.value, "π");
    }

    #[test]
    fn string_may_not_contain_unescaped_ascii_control_characters() {
        let input = ParserInput::new("\"\n\"");
        let result = string_literal(input);
        assert!(result.is_err());
    }

    #[test]
    fn string_may_not_contain_unescaped_non_ascii_control_characters() {
        // U+0085 encodes the control character NEL
        let input = ParserInput::new("\"\u{0085}\"");
        let result = string_literal(input);
        assert!(result.is_err());
    }

    #[test]
    fn string_may_not_contain_unescaped_text_direction_marks() {
        // U+202E encodes the right-to-left override mark
        let input = ParserInput::new("\"\u{202E}\"");
        let result = string_literal(input);
        assert!(result.is_err());
    }
}
