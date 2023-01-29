use ast::{BinaryOperatorSymbol, ImportNode, TopLevelDeclaration, UnaryOperatorSymbol};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedBinaryOperatorExpression<T> {
    pub expression_type: T,
    pub symbol: BinaryOperatorSymbol,
    pub left_child: TypedExpression<T>,
    pub right_child: TypedExpression<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedBlockExpression<T> {
    pub expression_type: T,
    pub contents: Vec<TypedExpression<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedBooleanLiteralExpression<T> {
    pub expression_type: T,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedDeclarationExpression<T> {
    pub expression_type: T,
    pub identifier: TypedIdentifierExpression<T>,
    pub value: TypedExpression<T>,
    pub is_exported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedFunctionExpression<T> {
    pub expression_type: T,
    pub argument_names: Vec<String>,
    pub body: TypedExpression<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedIdentifierExpression<T> {
    pub expression_type: T,
    pub name: String,
    pub is_disregarded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedIfExpression<T> {
    pub expression_type: T,
    pub condition: TypedExpression<T>,
    pub path_if_true: TypedExpression<T>,
    pub path_if_false: Option<TypedExpression<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedIntegerLiteralExpression<T> {
    pub expression_type: T,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedListExpression<T> {
    pub expression_type: T,
    pub contents: Vec<TypedExpression<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedRecordAssignmentExpression<T> {
    pub expression_type: T,
    pub identifier: TypedIdentifierExpression<T>,
    pub contents: TypedRecordExpression<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedRecordExpression<T> {
    pub expression_type: T,
    pub contents: HashMap<String, TypedExpression<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedStringLiteralExpression<T> {
    pub expression_type: T,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedTagExpression<T> {
    pub expression_type: T,
    pub name: String,
    pub contents: Vec<TypedExpression<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedUnaryOperatorExpression<T> {
    pub expression_type: T,
    pub symbol: UnaryOperatorSymbol,
    pub child: TypedExpression<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedExpression<T> {
    BinaryOperator(Box<TypedBinaryOperatorExpression<T>>),
    Block(Box<TypedBlockExpression<T>>),
    Boolean(Box<TypedBooleanLiteralExpression<T>>),
    Declaration(Box<TypedDeclarationExpression<T>>),
    Function(Box<TypedFunctionExpression<T>>),
    Identifier(Box<TypedIdentifierExpression<T>>),
    If(Box<TypedIfExpression<T>>),
    Integer(Box<TypedIntegerLiteralExpression<T>>),
    List(Box<TypedListExpression<T>>),
    Record(Box<TypedRecordExpression<T>>),
    RecordAssignment(Box<TypedRecordAssignmentExpression<T>>),
    StringLiteral(Box<TypedStringLiteralExpression<T>>),
    Tag(Box<TypedTagExpression<T>>),
    UnaryOperator(Box<TypedUnaryOperatorExpression<T>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedTypeDeclaration<T> {
    pub declaration_type: T,
    pub identifier_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedVariableDeclaration<T> {
    pub declaration_type: T,
    pub identifier_name: String,
    pub expression: TypedExpression<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedDocument<'a, T> {
    pub imports: Vec<ImportNode<'a>>,
    pub type_declarations: Vec<TopLevelDeclaration<TypedTypeDeclaration<T>>>,
    pub variable_declarations: Vec<TopLevelDeclaration<TypedVariableDeclaration<T>>>,
    pub expressions: Vec<TypedExpression<T>>,
}
