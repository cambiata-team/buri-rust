use crate::{
    ConcreteType, TypedBinaryOperatorExpression, TypedBlockExpression,
    TypedBooleanLiteralExpression, TypedExpression, TypedFunctionExpression,
    TypedIdentifierExpression, TypedIfExpression, TypedIntegerLiteralExpression,
    TypedListExpression, TypedRecordExpression, TypedStringLiteralExpression, TypedTagExpression,
    TypedUnaryOperatorExpression,
};

pub type ConcreteBinaryOperatorExpression = TypedBinaryOperatorExpression<ConcreteType>;
pub type ConcreteBlockExpression = TypedBlockExpression<ConcreteType>;
pub type ConcreteBooleanExpression = TypedBooleanLiteralExpression<ConcreteType>;
pub type ConcreteFunctionExpression = TypedFunctionExpression<ConcreteType>;
pub type ConcreteIdentifierExpression = TypedIdentifierExpression<ConcreteType>;
pub type ConcreteIfExpression = TypedIfExpression<ConcreteType>;
pub type ConcreteIntegerLiteralExpression = TypedIntegerLiteralExpression<ConcreteType>;
pub type ConcreteListExpression = TypedListExpression<ConcreteType>;
pub type ConcreteRecordExpression = TypedRecordExpression<ConcreteType>;
pub type ConcreteStringLiteralExpression = TypedStringLiteralExpression<ConcreteType>;
pub type ConcreteTagExpression = TypedTagExpression<ConcreteType>;
pub type ConcreteUnaryOperatorExpression = TypedUnaryOperatorExpression<ConcreteType>;

pub type ConcreteExpression = TypedExpression<ConcreteType>;

impl ConcreteExpression {
    #[must_use]
    pub fn identifier_for_test(name: &str) -> Self {
        Self::Identifier(Box::new(ConcreteIdentifierExpression {
            expression_type: ConcreteType::default_for_test(),
            name: name.to_string(),
            is_disregarded: false,
        }))
    }

    #[must_use]
    pub fn string_for_test(string: &str) -> Self {
        Self::StringLiteral(Box::new(ConcreteStringLiteralExpression {
            expression_type: ConcreteType::default_string_for_test(),
            value: string.to_string(),
        }))
    }

    #[must_use]
    pub fn integer_for_test(int: u64) -> Self {
        Self::Integer(Box::new(ConcreteIntegerLiteralExpression {
            expression_type: ConcreteType::default_integer_for_test(),
            value: int,
        }))
    }
}
