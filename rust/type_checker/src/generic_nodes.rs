use crate::GenericTypeId;
use ast::ParserInput;
use typed_ast::{
    TypedBinaryOperatorExpression, TypedBlockExpression, TypedBooleanLiteralExpression,
    TypedExpression, TypedFunctionExpression, TypedIdentifierExpression, TypedIfExpression,
    TypedIntegerLiteralExpression, TypedListExpression, TypedRecordExpression,
    TypedStringLiteralExpression, TypedTagExpression, TypedUnaryOperatorExpression,
};

pub struct GenericSourcedType<'a> {
    /// The derived type of an expression.
    pub type_id: GenericTypeId,
    /// Source code of the expression generating this type.
    pub source_of_type: ParserInput<'a>,
}

pub type GenericBinaryOperatorExpression<'a> =
    TypedBinaryOperatorExpression<GenericSourcedType<'a>>;
pub type GenericBlockExpression<'a> = TypedBlockExpression<GenericSourcedType<'a>>;
pub type GenericBooleanExpression<'a> = TypedBooleanLiteralExpression<GenericSourcedType<'a>>;
pub type GenericFunctionExpression<'a> = TypedFunctionExpression<GenericSourcedType<'a>>;
pub type GenericIdentifierExpression<'a> = TypedIdentifierExpression<GenericSourcedType<'a>>;
pub type GenericIfExpression<'a> = TypedIfExpression<GenericSourcedType<'a>>;
pub type GenericIntegerLiteralExpression<'a> =
    TypedIntegerLiteralExpression<GenericSourcedType<'a>>;
pub type GenericListExpression<'a> = TypedListExpression<GenericSourcedType<'a>>;
pub type GenericRecordExpression<'a> = TypedRecordExpression<GenericSourcedType<'a>>;
pub type GenericStringLiteralExpression<'a> = TypedStringLiteralExpression<GenericSourcedType<'a>>;
pub type GenericTagExpression<'a> = TypedTagExpression<GenericSourcedType<'a>>;
pub type GenericUnaryOperatorExpression<'a> = TypedUnaryOperatorExpression<GenericSourcedType<'a>>;

pub type GenericExpression<'a> = TypedExpression<GenericSourcedType<'a>>;
