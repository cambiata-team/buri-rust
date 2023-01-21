use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    CompilerBoolean,
    Num,
    Str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteFunctionType {
    /// Types of the arguments of a function, in order.
    /// If a function does not take any arguments, then the vec is empty.
    pub argument_types: Vec<ConcreteType>,
    /// return_type = None means that the function does not return a value.
    pub return_type: Option<ConcreteType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConcreteTagUnionType {
    /// Map the name of a tag to an array of the types of its contained values.
    /// Tag with no contents maps to empty vec.
    pub tag_types: HashMap<String, Vec<ConcreteType>>,
    /// Signifies of any tags in the tag union have content. If this is true,
    /// then at least one tag in the tag union has content. If this is false,
    /// then no tags in the tag union have content.
    pub some_tags_have_content: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteListType {
    pub element_type: ConcreteType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteRecordType {
    /// Map field name to type of that field.
    pub field_types: HashMap<String, ConcreteType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcreteType {
    Primitive(PrimitiveType),
    Function(Box<ConcreteFunctionType>),
    TagUnion(Box<ConcreteTagUnionType>),
    List(Box<ConcreteListType>),
    Record(Box<ConcreteRecordType>),
}
