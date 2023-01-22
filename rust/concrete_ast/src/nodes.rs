use crate::concrete_types::ConcreteTagUnionType;
use ast::{BinaryOperatorSymbol, UnaryOperatorSymbol};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteBinaryOperatorExpression {
    pub symbol: BinaryOperatorSymbol,
    pub left_child: ConcreteExpression,
    pub right_child: ConcreteExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteBlockExpression {
    pub contents: Vec<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteFunctionExpression {
    pub argument_names: Vec<String>,
    pub body: ConcreteExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteIdentifierExpression {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteIfExpression {
    pub condition: ConcreteExpression,
    pub path_if_true: ConcreteExpression,
    pub path_if_false: Option<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteIntegerLiteralExpression {
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteListExpression {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteTagExpression {
    pub concrete_type: ConcreteTagUnionType,
    pub name: String,
    pub contents: Vec<ConcreteExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteUnaryOperatorExpression {
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
