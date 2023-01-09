use std::ops::Deref;
use std::ops::Range;

type ParserInputContents<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ParserInput<'a>(ParserInputContents<'a>);
pub type IResult<'a, T> = nom::IResult<ParserInput<'a>, T>;
impl<'a> Deref for ParserInput<'a> {
    type Target = ParserInputContents<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ParserInput<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        ParserInput(ParserInputContents::new(input))
    }

    #[must_use]
    pub fn to_range(&self) -> Range<usize> {
        let start = self.location_offset();
        let end = start + self.value().len();
        start..end
    }

    #[must_use]
    pub fn value(&self) -> &'a str {
        self.fragment()
    }

    #[must_use]
    pub const fn get_located_span(&self) -> &ParserInputContents<'a> {
        &self.0
    }
}

impl<'a> PartialEq<&str> for ParserInput<'a> {
    fn eq(&self, string: &&str) -> bool {
        self.value() == *string
    }
}

impl<'a> PartialEq<String> for ParserInput<'a> {
    fn eq(&self, string: &String) -> bool {
        self.value() == string.as_str()
    }
}

impl<'a> nom::InputLength for ParserInput<'a> {
    #[must_use]
    fn input_len(&self) -> usize {
        self.0.input_len()
    }
}

impl<'a> nom::InputTake for ParserInput<'a> {
    fn take(&self, count: usize) -> Self {
        ParserInput(self.0.take(count))
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (left, right) = self.0.take_split(count);
        (ParserInput(left), ParserInput(right))
    }
}

impl<'a> nom::InputIter for ParserInput<'a> {
    type Item = char;
    type Iter = std::str::CharIndices<'a>;
    type IterElem = std::str::Chars<'a>;

    #[must_use]
    fn iter_indices(&self) -> Self::Iter {
        self.0.iter_indices()
    }

    #[must_use]
    fn iter_elements(&self) -> Self::IterElem {
        self.0.iter_elements()
    }

    #[must_use]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.0.slice_index(count)
    }
}

impl<'a> nom::ExtendInto for ParserInput<'a> {
    type Item = char;
    type Extender = String;
    #[inline]
    #[must_use]
    fn new_builder(&self) -> String {
        String::new()
    }
    #[inline]
    fn extend_into(&self, acc: &mut String) {
        acc.push_str(self.value());
    }
}

impl<'a> nom::Offset for ParserInput<'a> {
    #[must_use]
    fn offset(&self, second: &Self) -> usize {
        self.0.offset(&second.0)
    }
}

impl<'a> nom::Compare<&str> for ParserInput<'a> {
    #[must_use]
    fn compare(&self, t: &str) -> nom::CompareResult {
        self.0.compare(t)
    }

    #[must_use]
    fn compare_no_case(&self, t: &str) -> nom::CompareResult {
        self.0.compare_no_case(t)
    }
}

impl<'a> nom::FindSubstring<&str> for ParserInput<'a> {
    #[must_use]
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.0.find_substring(substr)
    }
}

impl<'a> nom::FindToken<char> for ParserInput<'a> {
    #[must_use]
    fn find_token(&self, token: char) -> bool {
        self.0.find_token(token)
    }
}

impl<'a> nom::Slice<std::ops::Range<usize>> for ParserInput<'a> {
    #[must_use]
    fn slice(&self, range: std::ops::Range<usize>) -> Self {
        ParserInput(self.0.slice(range))
    }
}

impl<'a> nom::Slice<std::ops::RangeTo<usize>> for ParserInput<'a> {
    #[must_use]
    fn slice(&self, range: std::ops::RangeTo<usize>) -> Self {
        ParserInput(self.0.slice(range))
    }
}

impl<'a> nom::Slice<std::ops::RangeFrom<usize>> for ParserInput<'a> {
    #[must_use]
    fn slice(&self, range: std::ops::RangeFrom<usize>) -> Self {
        ParserInput(self.0.slice(range))
    }
}

impl<'a> nom::Slice<std::ops::RangeFull> for ParserInput<'a> {
    #[must_use]
    fn slice(&self, range: std::ops::RangeFull) -> Self {
        ParserInput(self.0.slice(range))
    }
}

impl<'a> nom::UnspecializedInput for ParserInput<'a> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser_input_value_is_actual_value() {
        let input = ParserInput::new("hello world");
        assert_eq!(input.value(), "hello world");
    }

    #[test]
    fn parser_input_is_partially_equal_to_a_str() {
        let input = ParserInput::new("hello world");
        assert_eq!(input, "hello world");
    }

    #[test]
    fn parser_input_is_partially_equal_to_a_string() {
        let input = ParserInput::new("hello world");
        assert_eq!(input, "hello world".to_string());
    }

    #[test]
    fn parser_input_partial_eq_can_be_checked_inside_a_vector() {
        let input = ParserInput::new("hello world");
        assert_eq!(vec![input], vec!["hello world"]);
    }
}
