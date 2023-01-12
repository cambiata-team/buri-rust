use crate::concrete_types::ConcreteType;
use ast::{BinaryOperatorSymbol, UnaryOperatorSymbol};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteBinaryOperatorExpression {
    pub concrete_type: ConcreteType,
    pub symbol: BinaryOperatorSymbol,
    pub left_child: ConcreteExpression,
    pub right_child: ConcreteExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteBlockExpression {
    pub concrete_type: ConcreteType,
    pub contents: Vec<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteFunctionExpression {
    pub concrete_type: ConcreteType,
    pub argument_names: Vec<String>,
    pub body: ConcreteExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteIdentifierExpression {
    pub concrete_type: ConcreteType,
    pub name: String,
    pub is_disregarded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteIfExpression {
    pub concrete_type: ConcreteType,
    pub condition: ConcreteExpression,
    pub path_if_true: ConcreteExpression,
    pub path_if_false: Option<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteIntegerExpression {
    pub concrete_type: ConcreteType,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteListExpression {
    pub concrete_type: ConcreteType,
    pub contents: Vec<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteRecordExpression {
    pub concrete_type: ConcreteType,
    pub contents: HashMap<String, ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteStringLiteralExpression {
    pub concrete_type: ConcreteType,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteTagExpression {
    pub concrete_type: ConcreteType,
    pub name: String,
    pub contents: Vec<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcreteUnaryOperatorExpression {
    pub concrete_type: ConcreteType,
    pub symbol: UnaryOperatorSymbol,
    pub child: ConcreteExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConcreteExpression {
    BinaryOperator(Box<ConcreteBinaryOperatorExpression>),
    Block(Box<ConcreteBlockExpression>),
    Function(Box<ConcreteFunctionExpression>),
    Identifier(Box<ConcreteIdentifierExpression>),
    If(Box<ConcreteIfExpression>),
    Integer(Box<ConcreteIntegerExpression>),
    List(Box<ConcreteListExpression>),
    Record(Box<ConcreteRecordExpression>),
    StringLiteral(Box<ConcreteStringLiteralExpression>),
    Tag(Box<ConcreteTagExpression>),
    UnaryOperator(Box<ConcreteUnaryOperatorExpression>),
}
