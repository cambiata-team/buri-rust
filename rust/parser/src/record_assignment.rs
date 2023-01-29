use crate::{
    expression_context::ExpressionContext,
    identifier::identifier,
    intra_expression_whitespace::intra_expression_whitespace,
    record::{record_closing_delimiter, record_opening_delimiter, record_values},
};
use ast::{IResult, ParserInput, RecordAssignmentNode, RecordAssignmentValue};
use nom::{
    character::complete::char,
    combinator::{consumed, map, opt},
    sequence::{delimited, separated_pair, tuple},
};

pub fn record_assignment(
    context: ExpressionContext,
    input: ParserInput,
) -> IResult<RecordAssignmentNode> {
    map(
        consumed(delimited(
            record_opening_delimiter,
            separated_pair(
                identifier,
                tuple((
                    opt(intra_expression_whitespace(
                        context.allow_newlines_in_expressions(),
                    )),
                    char('|'),
                    opt(intra_expression_whitespace(
                        context.allow_newlines_in_expressions(),
                    )),
                )),
                record_values,
            ),
            record_closing_delimiter,
        )),
        |(consumed, (identifier, new_values))| RecordAssignmentNode {
            source: consumed,
            value: RecordAssignmentValue {
                identifier,
                new_values,
            },
        },
    )(input)
}

#[cfg(test)]
mod test {
    use ast::Expression;

    use super::*;

    #[test]
    fn assignment_can_parse() {
        let input = ParserInput::new("{hello|name:\"world\"}");
        let result = record_assignment(ExpressionContext::new(), input);
        assert!(result.is_ok());
    }

    #[test]
    fn parses_identifier_correctly() {
        let input = ParserInput::new("{hello|name:\"world\"}");
        let (_, parsed) = record_assignment(ExpressionContext::new(), input).unwrap();
        assert_eq!(parsed.value.identifier.value.name, "hello");
    }

    #[test]
    fn identifier_cannot_be_an_expression() {
        let input = ParserInput::new("{hello()|name:\"world\"}");
        let result = record_assignment(ExpressionContext::new(), input);
        assert!(result.is_err());
    }

    #[test]
    fn identifier_cannot_be_type() {
        let input = ParserInput::new("{Hello|name:\"world\"}");
        let result = record_assignment(ExpressionContext::new(), input);
        assert!(result.is_err());
    }

    #[test]
    fn parses_one_value() {
        let input = ParserInput::new("{hello|name:\"world\"}");
        let (_, parsed) = record_assignment(ExpressionContext::new(), input).unwrap();
        assert_eq!(parsed.value.new_values.len(), 1);
    }

    #[test]
    fn parses_two_values_separated_by_a_comma() {
        let input = ParserInput::new("{hello|name:\"world\",age:24}");
        let (_, parsed) = record_assignment(ExpressionContext::new(), input).unwrap();
        assert_eq!(parsed.value.new_values.len(), 2);
    }

    #[test]
    fn trailing_commas_do_not_parse_into_an_additional_item() {
        let input = ParserInput::new("{hello|name:\"world\",}");
        let (_, parsed) = record_assignment(ExpressionContext::new(), input).unwrap();
        assert_eq!(parsed.value.new_values.len(), 1);
    }

    #[test]
    fn parses_value_identifier() {
        let input = ParserInput::new("{hello|name:\"world\"}");
        let (_, parsed) = record_assignment(ExpressionContext::new(), input).unwrap();
        assert_eq!(
            parsed
                .value
                .new_values
                .get(0)
                .unwrap()
                .identifier
                .value
                .name,
            "name"
        );
    }

    #[test]
    fn parses_value_expression() {
        let input = ParserInput::new("{hello|name:\"world\"}");
        let (_, parsed) = record_assignment(ExpressionContext::new(), input).unwrap();
        assert!(matches!(
            parsed.value.new_values.get(0).unwrap().value,
            Expression::StringLiteral(_)
        ));
    }

    #[test]
    fn can_have_spaces_anywhere() {
        let input = ParserInput::new("{  hello  |  name  :  \"world\"  ,  }");
        let result = record_assignment(ExpressionContext::new(), input);
        assert!(result.is_ok());
    }

    #[test]
    fn can_have_newlines_anywhere() {
        let input = ParserInput::new("{\n\nhello\n\n|\n\nname\n\n:\n\n\"world\"\n\n,\n\n}");
        let result = record_assignment(ExpressionContext::new(), input);
        assert!(result.is_ok());
    }
}
