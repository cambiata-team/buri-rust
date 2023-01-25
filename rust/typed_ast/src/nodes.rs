use ast::{BinaryOperatorSymbol, UnaryOperatorSymbol};
use std::collections::HashMap;

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedBinaryOperatorExpression<T> {
    pub expression_type: T,
    pub symbol: BinaryOperatorSymbol,
    pub left_child: TypedExpression<T>,
    pub right_child: TypedExpression<T>,
}

// TODO(nick): add this to JS backend
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

// TODO(nick): add this to JS backend
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

// TODO(nick): add this to JS backend
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

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedListExpression<T> {
    pub expression_type: T,
    pub contents: Vec<TypedExpression<T>>,
}

// TODO(nick): add this to JS backend
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

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedTagExpression<T> {
    pub expression_type: T,
    pub name: String,
    pub contents: Vec<TypedExpression<T>>,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedUnaryOperatorExpression<T> {
    pub expression_type: T,
    pub symbol: UnaryOperatorSymbol,
    pub child: TypedExpression<T>,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedExpression<T> {
    BinaryOperator(Box<TypedBinaryOperatorExpression<T>>),
    Block(Box<TypedBlockExpression<T>>),
    Boolean(Box<TypedBooleanLiteralExpression<T>>),
    Function(Box<TypedFunctionExpression<T>>),
    Identifier(Box<TypedIdentifierExpression<T>>),
    If(Box<TypedIfExpression<T>>),
    Integer(Box<TypedIntegerLiteralExpression<T>>),
    List(Box<TypedListExpression<T>>),
    Record(Box<TypedRecordExpression<T>>),
    StringLiteral(Box<TypedStringLiteralExpression<T>>),
    Tag(Box<TypedTagExpression<T>>),
    UnaryOperator(Box<TypedUnaryOperatorExpression<T>>),
}
