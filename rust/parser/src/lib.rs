mod basic_expression;
mod binary_operator_expression;
mod binary_operator_or_if;
mod block;
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
mod line;
mod list;
mod list_type;
mod newline;
mod parentheses;
mod record;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpressionContext {
    pub indentation: usize,
    pub allow_newlines_in_expressions: bool,
}

impl ExpressionContext {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            indentation: 0,
            allow_newlines_in_expressions: false,
        }
    }

    #[must_use]
    /// Creates a new `ExpressionContext` whose expected level of indentation is one greater
    /// than the object on which this method is invoked.
    pub const fn increment_indentation(&self) -> Self {
        Self {
            indentation: self.indentation + 1,
            allow_newlines_in_expressions: self.allow_newlines_in_expressions,
        }
    }

    #[must_use]
    /// Creates a copy of `self` that allows newlines in expressions.
    pub const fn allow_newlines_in_expressions(&self) -> Self {
        Self {
            indentation: self.indentation,
            allow_newlines_in_expressions: true,
        }
    }

    #[must_use]
    /// Creates a copy of `self` that disallows newlines in expressions.
    pub const fn disallow_newlines_in_expressions(&self) -> Self {
        Self {
            indentation: self.indentation,
            allow_newlines_in_expressions: false,
        }
    }
}
