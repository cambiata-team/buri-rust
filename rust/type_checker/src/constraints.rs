use crate::GenericTypeId;
use typed_ast::ConcreteType;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type be a tag union with a tag named `tag_name`,
/// the contents of which have the types of `tag_contents_types`.
pub struct SubTagConstraint {
    pub tag_name: String,
    pub tag_content_types: Vec<GenericTypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type be a record with a field named `field_name`,
/// the type of which is `field_type`.
pub struct HasFieldConstraint {
    pub field_name: String,
    pub field_type: GenericTypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type be a record with a method named `method_name`,
/// the type of which is `method_type`.
pub struct HasMethodConstraint {
    pub method_name: String,
    pub method_type: GenericTypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    /// Constrain that a generic type be equal to some concrete type.
    EqualToConcrete(ConcreteType),
    SubTag(SubTagConstraint),
    HasField(HasFieldConstraint),
    HasMethod(HasMethodConstraint),
    /// Constrain that a generic type is a function with a given return type.
    HasReturnType(GenericTypeId),
    /// Constrain that a generic type is a function whose arguments have given types.
    HasArgumentTypes(Vec<GenericTypeId>),
}