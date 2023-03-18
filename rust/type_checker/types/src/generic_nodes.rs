use crate::TypeId;
use ast::{ImportNode, ParserInput, TopLevelDeclaration};
use typed_ast::{
    TypedBinaryOperatorExpression, TypedBlockExpression, TypedBooleanLiteralExpression,
    TypedDeclarationExpression, TypedExpression, TypedFunctionExpression,
    TypedIdentifierExpression, TypedIfExpression, TypedIntegerLiteralExpression,
    TypedListExpression, TypedRecordAssignmentExpression, TypedRecordExpression,
    TypedStringLiteralExpression, TypedTagExpression, TypedTypeDeclarationExpression,
    TypedTypeIdentifierExpression, TypedUnaryOperatorExpression, TypedWhenCase, TypedWhenCaseName,
    TypedWhenExpression,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericSourcedType<'a> {
    /// The derived type of an expression.
    pub type_id: TypeId,
    /// Source code of the expression generating this type.
    pub source_of_type: ParserInput<'a>,
}

pub type GenericBinaryOperatorExpression<'a> =
    TypedBinaryOperatorExpression<GenericSourcedType<'a>>;
pub type GenericBlockExpression<'a> = TypedBlockExpression<GenericSourcedType<'a>>;
pub type GenericBooleanExpression<'a> = TypedBooleanLiteralExpression<GenericSourcedType<'a>>;
pub type GenericDeclarationExpression<'a> = TypedDeclarationExpression<GenericSourcedType<'a>>;
pub type GenericFunctionExpression<'a> = TypedFunctionExpression<GenericSourcedType<'a>>;
pub type GenericIdentifierExpression<'a> = TypedIdentifierExpression<GenericSourcedType<'a>>;
pub type GenericIfExpression<'a> = TypedIfExpression<GenericSourcedType<'a>>;
pub type GenericIntegerLiteralExpression<'a> =
    TypedIntegerLiteralExpression<GenericSourcedType<'a>>;
pub type GenericListExpression<'a> = TypedListExpression<GenericSourcedType<'a>>;
pub type GenericRecordAssignmentExpression<'a> =
    TypedRecordAssignmentExpression<GenericSourcedType<'a>>;
pub type GenericRecordExpression<'a> = TypedRecordExpression<GenericSourcedType<'a>>;
pub type GenericStringLiteralExpression<'a> = TypedStringLiteralExpression<GenericSourcedType<'a>>;
pub type GenericTagExpression<'a> = TypedTagExpression<GenericSourcedType<'a>>;
pub type GenericTypeDeclarationExpression<'a> =
    TypedTypeDeclarationExpression<GenericSourcedType<'a>>;
pub type GenericTypeIdentifierExpression<'a> =
    TypedTypeIdentifierExpression<GenericSourcedType<'a>>;
pub type GenericUnaryOperatorExpression<'a> = TypedUnaryOperatorExpression<GenericSourcedType<'a>>;
pub type GenericWhenExpression<'a> = TypedWhenExpression<GenericSourcedType<'a>>;
pub type GenericWhenCase<'a> = TypedWhenCase<GenericSourcedType<'a>>;
pub type GenericWhenCaseName<'a> = TypedWhenCaseName;

pub type GenericExpression<'a> = TypedExpression<GenericSourcedType<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericDocument<'a> {
    pub imports: Vec<ImportNode<'a>>,
    pub type_declarations:
        Vec<TopLevelDeclaration<TypedTypeDeclarationExpression<GenericSourcedType<'a>>>>,
    pub variable_declarations: Vec<TopLevelDeclaration<GenericDeclarationExpression<'a>>>,
    pub expressions: Vec<TypedExpression<GenericSourcedType<'a>>>,
}

#[must_use]
pub const fn get_generic_type_id(input: &GenericExpression) -> TypeId {
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
        GenericExpression::TypeDeclaration(node) => node.expression_type.type_id,
        GenericExpression::TypeIdentifier(node) => node.expression_type.type_id,
        GenericExpression::UnaryOperator(node) => node.expression_type.type_id,
        GenericExpression::When(node) => node.expression_type.type_id,
    }
}
