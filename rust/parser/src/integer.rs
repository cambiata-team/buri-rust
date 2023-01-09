use ast::IntegerNode;
use ast::{IResult, ParserInput};
use nom::{character::complete::digit1, combinator::map};

/// Parses an integer with the digits 0-9. Underscores are not allowed.
/// Negatives are not allowed (use the `-` unary operator instead).
pub fn integer(input: ParserInput) -> IResult<IntegerNode> {
    map(digit1, |consumed: ParserInput| IntegerNode {
        // Will default to 0 if the integer is too large or small to fit in an
        // i64.
        value: consumed.value().parse::<u64>().unwrap_or(std::u64::MAX),
        source: consumed,
    })(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn non_digits_error() {
        let input = ParserInput::new("hello");
        let result = integer(input);
        assert!(result.is_err());
    }

    #[test]
    fn a_negative_sign_errors() {
        let input = ParserInput::new("-1");
        let result = integer(input);
        assert!(result.is_err());
    }

    #[test]
    fn zero_has_a_value_of_zero() {
        let input = ParserInput::new("0");
        let (remainder, integer_node) = integer(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(integer_node.value, 0);
    }

    #[test]
    fn a_positive_integer_has_a_value_of_the_integer() {
        let input = ParserInput::new("1");
        let (remainder, integer_node) = integer(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(integer_node.value, 1);
    }

    #[test]
    fn can_have_multiple_leading_zeros() {
        let input = ParserInput::new("0001");
        let (remainder, integer_node) = integer(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(integer_node.value, 1);
    }

    #[test]
    fn integers_that_cannot_fit_in_an_i64_evaluate_to_zero() {
        let input = ParserInput::new("9999999999999999999999999999");
        let (remainder, integer_node) = integer(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(integer_node.value, std::u64::MAX);
    }
}
