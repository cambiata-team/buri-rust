use crate::ParserInput;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParsedNode<'a, T> {
    pub source: ParserInput<'a>,
    pub value: T,
}
