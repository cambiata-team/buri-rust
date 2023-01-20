use crate::concrete_types::ConcreteType;
use ast::{BinaryOperatorSymbol, UnaryOperatorSymbol};

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteBinaryOperatorExpression {
    pub concrete_type: ConcreteType,
    pub symbol: BinaryOperatorSymbol,
    pub left_child: ConcreteExpression,
    pub right_child: ConcreteExpression,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteBlockExpression {
    pub concrete_type: ConcreteType,
    pub contents: Vec<ConcreteExpression>,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteFunctionExpression {
    pub concrete_type: ConcreteType,
    pub argument_names: Vec<String>,
    pub body: ConcreteExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteIdentifierExpression {
    pub name: String,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteIfExpression {
    pub concrete_type: ConcreteType,
    pub condition: ConcreteExpression,
    pub path_if_true: ConcreteExpression,
    pub path_if_false: Option<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteIntegerLiteralExpression {
    pub value: u64,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteListExpression {
    pub concrete_type: ConcreteType,
    pub contents: Vec<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteRecordExpression {
    pub contents: Vec<(String, ConcreteExpression)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteStringLiteralExpression {
    pub value: String,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteTagExpression {
    pub concrete_type: ConcreteType,
    pub name: String,
    pub contents: Vec<ConcreteExpression>,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteUnaryOperatorExpression {
    pub concrete_type: ConcreteType,
    pub symbol: UnaryOperatorSymbol,
    pub child: ConcreteExpression,
}

// TODO(nick): add this to JS backend
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcreteExpression {
    BinaryOperator(Box<ConcreteBinaryOperatorExpression>),
    Block(Box<ConcreteBlockExpression>),
    Function(Box<ConcreteFunctionExpression>),
    Identifier(Box<ConcreteIdentifierExpression>),
    If(Box<ConcreteIfExpression>),
    Integer(Box<ConcreteIntegerLiteralExpression>),
    List(Box<ConcreteListExpression>),
    Record(Box<ConcreteRecordExpression>),
    StringLiteral(Box<ConcreteStringLiteralExpression>),
    Tag(Box<ConcreteTagExpression>),
    UnaryOperator(Box<ConcreteUnaryOperatorExpression>),
}
