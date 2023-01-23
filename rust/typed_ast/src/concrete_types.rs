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

impl ConcreteType {
    #[must_use]
    pub const fn default_for_test() -> Self {
        Self::default_string_for_test()
    }

    #[must_use]
    pub fn default_record_for_test() -> Self {
        Self::Record(Box::new(ConcreteRecordType {
            field_types: HashMap::new(),
        }))
    }

    #[must_use]
    pub const fn default_integer_for_test() -> Self {
        Self::Primitive(PrimitiveType::Num)
    }

    #[must_use]
    pub const fn default_string_for_test() -> Self {
        Self::Primitive(PrimitiveType::Str)
    }

    #[must_use]
    pub fn default_list_for_test() -> Self {
        Self::List(Box::new(ConcreteListType {
            element_type: Self::default_string_for_test(),
        }))
    }

    #[must_use]
    pub fn default_binary_operator_for_test() -> Self {
        Self::Function(Box::new(ConcreteFunctionType {
            argument_types: vec![],
            return_type: None,
        }))
    }

    #[must_use]
    pub fn default_tag_union_for_test(some_tags_have_content: bool) -> Self {
        Self::TagUnion(Box::new(ConcreteTagUnionType {
            some_tags_have_content,
            tag_types: HashMap::new(),
        }))
    }

    #[must_use]
    pub fn default_function_for_test() -> Self {
        Self::Function(Box::new(ConcreteFunctionType {
            argument_types: vec![],
            return_type: None,
        }))
    }
}
