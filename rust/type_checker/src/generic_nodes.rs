use crate::GenericTypeId;
use typed_ast::{
    TypedBinaryOperatorExpression, TypedBlockExpression, TypedBooleanLiteralExpression,
    TypedExpression, TypedFunctionExpression, TypedIdentifierExpression, TypedIfExpression,
    TypedIntegerLiteralExpression, TypedListExpression, TypedRecordExpression,
    TypedStringLiteralExpression, TypedTagExpression, TypedUnaryOperatorExpression,
};

pub type GenericBinaryOperatorExpression = TypedBinaryOperatorExpression<GenericTypeId>;
pub type GenericBlockExpression = TypedBlockExpression<GenericTypeId>;
pub type GenericBooleanExpression = TypedBooleanLiteralExpression<GenericTypeId>;
pub type GenericFunctionExpression = TypedFunctionExpression<GenericTypeId>;
pub type GenericIdentifierExpression = TypedIdentifierExpression<GenericTypeId>;
pub type GenericIfExpression = TypedIfExpression<GenericTypeId>;
pub type GenericIntegerLiteralExpression = TypedIntegerLiteralExpression<GenericTypeId>;
pub type GenericListExpression = TypedListExpression<GenericTypeId>;
pub type GenericRecordExpression = TypedRecordExpression<GenericTypeId>;
pub type GenericStringLiteralExpression = TypedStringLiteralExpression<GenericTypeId>;
pub type GenericTagExpression = TypedTagExpression<GenericTypeId>;
pub type GenericUnaryOperatorExpression = TypedUnaryOperatorExpression<GenericTypeId>;

pub type GenericExpression = TypedExpression<GenericTypeId>;
