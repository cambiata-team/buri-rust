use crate::GenericTypeId;
use ast::ParserInput;
use typed_ast::{
    TypedBinaryOperatorExpression, TypedBlockExpression, TypedBooleanLiteralExpression,
    TypedDocument, TypedExpression, TypedFunctionExpression, TypedIdentifierExpression,
    TypedIfExpression, TypedIntegerLiteralExpression, TypedListExpression, TypedRecordExpression,
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
pub type GenericDocument<'a> = TypedDocument<'a, GenericSourcedType<'a>>;

pub const fn get_generic_type_id<'a>(input: &GenericExpression<'a>) -> GenericTypeId {
    match input {
        GenericExpression::BinaryOperator(node) => node.expression_type.type_id,
        GenericExpression::Block(node) => node.expression_type.type_id,
        GenericExpression::Boolean(node) => node.expression_type.type_id,
        GenericExpression::Declaration(node) => node.expression_type.type_id,
        GenericExpression::Function(node) => node.expression_type.type_id,
        GenericExpression::FunctionArguments(_) => unreachable!(),
        GenericExpression::Identifier(node) => node.expression_type.type_id,
        GenericExpression::If(node) => node.expression_type.type_id,
        GenericExpression::Integer(node) => node.expression_type.type_id,
        GenericExpression::List(node) => node.expression_type.type_id,
        GenericExpression::Record(node) => node.expression_type.type_id,
        GenericExpression::RecordAssignment(node) => node.expression_type.type_id,
        GenericExpression::StringLiteral(node) => node.expression_type.type_id,
        GenericExpression::Tag(node) => node.expression_type.type_id,
        GenericExpression::UnaryOperator(node) => node.expression_type.type_id,
    }
}
