use ast::{IResult, ParserInput};
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{not, value},
    multi::{count, many0_count},
    sequence::terminated,
};

fn one_indent(input: ParserInput) -> IResult<()> {
    value((), tag("    "))(input)
}

/// Return a parser which parses an indent with an indentation level of `expected_indentation`.
/// The returned parser will error if fractional indentation is discovered or if the actual indentation is less than or greater than the expected indentation.
/// When successful, the returned parser will return the level of indentation, which will always be equal to `expected_indentation`.
pub fn indent_exact<'a>(
    expected_indentation: usize,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, usize> {
    value(
        expected_indentation,
        terminated(count(one_indent, expected_indentation), not(char(' '))),
    )
}

/// Parse an indent with any level of indentation, including zero indentation.
/// Error if fractional indentation is discovered.
/// On success, return the level of indentation.
pub fn indent_any(input: ParserInput) -> IResult<usize> {
    terminated(many0_count(one_indent), not(char(' ')))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn indent_exact_parses_empty_string_when_level_is_zero() {
        let input = ParserInput::new("");
        let result = indent_exact(0)(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed, 0);
    }

    #[test]
    fn indent_exact_errors_on_empty_string_when_level_is_nonzero() {
        let input = ParserInput::new("");
        let result = indent_exact(1)(input);
        assert!(result.is_err());
    }

    #[test]
    fn indent_exact_consumes_space_string_when_actual_indentation_matches_expected_indentation() {
        let indentation_level_2 = " ".repeat(8);
        let input = ParserInput::new(&indentation_level_2);
        let result = indent_exact(2)(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed, 2);
    }

    #[test]
    fn indent_exact_errors_when_space_string_input_is_one_space_too_short() {
        let indentation_level_2_missing_a_space = " ".repeat(7);
        let input = ParserInput::new(&indentation_level_2_missing_a_space);
        let result = indent_exact(2)(input);
        assert!(result.is_err());
    }

    #[test]
    fn indent_exact_errors_when_space_string_input_is_one_space_too_long() {
        let indentation_level_2_with_extra_space = " ".repeat(9);
        let input = ParserInput::new(&indentation_level_2_with_extra_space);
        let result = indent_exact(2)(input);
        assert!(result.is_err());
    }

    #[test]
    fn indent_exact_errors_when_space_string_input_is_one_level_too_short() {
        let indentation_level_1 = " ".repeat(4);
        let input = ParserInput::new(&indentation_level_1);
        let result = indent_exact(2)(input);
        assert!(result.is_err());
    }

    #[test]
    fn indent_exact_errors_when_space_string_input_is_one_level_too_long() {
        let indentation_level_3 = " ".repeat(12);
        let input = ParserInput::new(&indentation_level_3);
        let result = indent_exact(2)(input);
        assert!(result.is_err());
    }

    #[test]
    fn indent_exact_errors_on_unexpected_word() {
        let input = ParserInput::new("hello");
        let result = indent_exact(1)(input);
        assert!(result.is_err());
    }

    #[test]
    fn indent_exact_stops_before_expected_word() {
        let input = ParserInput::new("    hello");
        let result = indent_exact(1)(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "hello");
        assert_eq!(consumed, 1);
    }

    #[test]
    fn indent_any_parses_empty_string() {
        let input = ParserInput::new("");
        let result = indent_any(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed, 0);
    }

    #[test]
    fn indent_any_parses_indent_level_one() {
        let indentation_level_1 = " ".repeat(4);
        let input = ParserInput::new(&indentation_level_1);
        let result = indent_any(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed, 1);
    }

    #[test]
    fn indent_any_parses_indent_level_two() {
        let indentation_level_2 = " ".repeat(8);
        let input = ParserInput::new(&indentation_level_2);
        let result = indent_any(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(consumed, 2);
    }

    #[test]
    fn indent_any_errors_on_fractional_indentation() {
        let two_spaces = " ".repeat(2);
        let input = ParserInput::new(&two_spaces);
        let result = indent_any(input);
        assert!(result.is_err());
    }

    #[test]
    fn indent_any_stops_when_it_reaches_a_word() {
        let input = ParserInput::new("    hello");
        let result = indent_any(input);
        let (remainder, consumed) = result.unwrap();
        assert_eq!(remainder, "hello");
        assert_eq!(consumed, 1);
    }
}
