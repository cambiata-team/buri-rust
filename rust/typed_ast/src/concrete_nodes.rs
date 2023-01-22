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
