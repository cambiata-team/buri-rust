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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcreteTagUnionType {
    /// Map the name of a tag to an array of the types of its contained values.
    /// Tag with no contents maps to empty vec.
    pub tag_types: HashMap<String, Vec<ConcreteType>>,
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

impl ConcreteType {
    pub const fn default_for_test() -> Self {
        Self::Primitive(PrimitiveType::Str)
    }
}
