mod basic_expression;
mod binary_operator_expression;
mod binary_operator_or_if;
mod block;
mod document;
mod expression_context;
mod file;
mod function;
mod function_argument;
mod function_type;
mod identifier;
mod if_statement;
mod import;
mod indent;
mod integer;
mod intra_expression_whitespace;
mod is_keyword;
mod list;
mod list_type;
mod newline;
mod parentheses;
mod record;
mod record_assignment;
mod record_type;
mod string_literal;
mod tag;
mod tag_group_type;
mod tag_identifier;
mod tag_type;
mod type_declaration;
mod type_expression;
mod type_identifier;
mod unary_operator;
mod variable_declaration;

use binary_operator_or_if::binary_operator_or_if as expression;
use expression_context::ExpressionContext;
pub use file::parse_buri_file;

/// Parses an expression for use in unit tests.
///
/// # Panics
///
/// Panics if the input is not a valid expression.
pub fn parse_test_expression(input: &str) -> ast::Expression {
    use crate::newline::newline;
    use nom::{
        combinator::{eof, map, opt},
        sequence::tuple,
    };

    let input = ast::ParserInput::new(input);
    #[allow(clippy::unwrap_used)] // because this should only be used in tests.
    let (_, expression) = map(
        tuple((
            expression(ExpressionContext::new().allow_newlines_in_expressions()),
            opt(newline),
            eof,
        )),
        |(expression, _, _)| expression,
    )(input)
    .unwrap();
    expression
}
