use crate::ParsedNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperatorSymbol {
    /// `+`
    Add,
    /// `-`
    Subtract,
    /// `*`
    Multiply,
    /// `/`
    Divide,
    /// `%`
    Modulus,
    /// `**`
    Power,
    /// ++
    Concatenate,
    /// `and`
    And,
    /// `or`
    Or,
    /// `==`
    EqualTo,
    /// `!=`
    NotEqualTo,
    /// `<`
    LessThan,
    /// `<=`
    LessThanOrEqualTo,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanOrEqualTo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOperatorValue<'a> {
    /// Which binary operator to apply
    pub symbol: BinaryOperatorSymbol,
    /// The expression to the before the operator symbol.
    pub left_child: Box<Expression<'a>>,
    /// The expression to the right of the operator symbol.
    pub right_child: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionValue<'a> {
    pub arguments: Vec<FunctionArgumentNode<'a>>,
    pub return_type: Option<TypeIdentifierNode<'a>>,
    pub body: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionArgumentValue<'a> {
    pub argument_name: IdentifierNode<'a>,
    pub argument_type: Option<TypeIdentifierNode<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionTypeValue<'a> {
    pub arguments: Vec<TypeExpression<'a>>,
    pub return_type: Box<TypeExpression<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierValue {
    /// The name given by the user.
    pub name: String,
    /// Whether or not the identifier is disregarded. All identifiers that
    /// start with an underscore are considered disregarded.
    pub is_disregarded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportValue<'a> {
    pub path: &'a str,
    pub identifiers: Vec<ImportedIdentifier<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordValue<'a> {
    /// The name of the record.
    pub identifier: IdentifierNode<'a>,
    /// The fields of the record.
    pub value: Expression<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordTypeValue<'a> {
    /// The name of the record.
    pub identifier: IdentifierNode<'a>,
    /// The fields of the record.
    pub value: TypeExpression<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagValue<'a> {
    pub name: TagIdentifierNode<'a>,
    pub contents: Vec<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagTypeValue<'a> {
    pub name: TagIdentifierNode<'a>,
    pub contents: Vec<TypeExpression<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclarationValue<'a> {
    pub identifier: TypeIdentifierNode<'a>,
    pub type_expression: TypeExpression<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfValue<'a> {
    pub condition: Box<Expression<'a>>,
    pub path_if_true: Box<Expression<'a>>,
    pub path_if_false: Option<Box<Expression<'a>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperatorSymbol {
    /// `not`
    Not,
    /// `-`
    Negative,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOperatorValue<'a> {
    /// Which unary operator to apply.
    pub symbol: UnaryOperatorSymbol,
    /// The expression to which the operator is applied.
    pub child: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclarationValue<'a> {
    pub identifier: IdentifierNode<'a>,
    pub type_expression: Option<TypeExpression<'a>>,
    pub expression: Expression<'a>,
}

pub type BinaryOperatorNode<'a> = ParsedNode<'a, BinaryOperatorValue<'a>>;
pub type BlockNode<'a> = ParsedNode<'a, Vec<Expression<'a>>>;
pub type FunctionNode<'a> = ParsedNode<'a, FunctionValue<'a>>;
pub type FunctionArgumentNode<'a> = ParsedNode<'a, FunctionArgumentValue<'a>>;
pub type FunctionTypeNode<'a> = ParsedNode<'a, FunctionTypeValue<'a>>;
pub type IdentifierNode<'a> = ParsedNode<'a, IdentifierValue>;
pub type IfNode<'a> = ParsedNode<'a, IfValue<'a>>;
pub type ImportNode<'a> = ParsedNode<'a, ImportValue<'a>>;
// negative numbers should be proceeded with the - unary operator
pub type IntegerNode<'a> = ParsedNode<'a, u64>;
pub type ListNode<'a> = ParsedNode<'a, Vec<Expression<'a>>>;
pub type ListTypeNode<'a> = ParsedNode<'a, TypeExpression<'a>>;
pub type RecordNode<'a> = ParsedNode<'a, Vec<RecordValue<'a>>>;
pub type RecordTypeNode<'a> = ParsedNode<'a, Vec<RecordTypeValue<'a>>>;
pub type StringLiteralNode<'a> = ParsedNode<'a, String>;
pub type TagIdentifierNode<'a> = ParsedNode<'a, String>;
pub type TagGroupTypeNode<'a> = ParsedNode<'a, Vec<TagTypeNode<'a>>>;
pub type TagNode<'a> = ParsedNode<'a, TagValue<'a>>;
pub type TagTypeNode<'a> = ParsedNode<'a, TagTypeValue<'a>>;
pub type TypeIdentifierNode<'a> = ParsedNode<'a, String>;
pub type TypeDeclarationNode<'a> = ParsedNode<'a, TypeDeclarationValue<'a>>;
pub type UnaryOperatorNode<'a> = ParsedNode<'a, UnaryOperatorValue<'a>>;
pub type VariableDeclarationNode<'a> = ParsedNode<'a, VariableDeclarationValue<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    BinaryOperator(BinaryOperatorNode<'a>),
    Block(BlockNode<'a>),
    Function(FunctionNode<'a>),
    Identifier(IdentifierNode<'a>),
    If(IfNode<'a>),
    Integer(IntegerNode<'a>),
    List(ListNode<'a>),
    Record(RecordNode<'a>),
    StringLiteral(StringLiteralNode<'a>),
    Tag(TagNode<'a>),
    UnaryOperator(UnaryOperatorNode<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpression<'a> {
    Function(FunctionTypeNode<'a>),
    Identifier(TypeIdentifierNode<'a>),
    List(Box<ListTypeNode<'a>>),
    Record(RecordTypeNode<'a>),
    Tag(TagTypeNode<'a>),
    TagGroup(TagGroupTypeNode<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportedIdentifier<'a> {
    Identifier(IdentifierNode<'a>),
    TypeIdentifier(TypeIdentifierNode<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuriAstNodeValue<'a> {
    Expression(Expression<'a>),
    Identifier(IdentifierNode<'a>),
}
