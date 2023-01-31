use crate::GenericTypeId;
use std::collections::HashMap;
use typed_ast::{ConcreteType, PrimitiveType};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type is a tag union
/// with a tag of given name whose payload has particular types.
pub struct HasTagConstraint {
    pub tag_name: String,
    pub tag_content_types: Vec<GenericTypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constraint that a generic type is a tag union whose tags are a subset
/// of a given set of tags.
pub struct TagAtMostConstraint {
    /// The keys are the names of the tags in a tag union.
    /// The values are the types of the tag payloads.
    pub tags: HashMap<String, Vec<GenericTypeId>>,
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
    /// Constrain that a generic type be equal to some primitive type.
    EqualToPrimitive(PrimitiveType),
    /// Constrain that a generic type is a list whose contents have a particular type.
    ListOfType(GenericTypeId),
    /// Constrain that a generic type is a tag union with at least a given set of tags.
    HasTag(HasTagConstraint),
    /// Constrain that a generic type is a tag union with at most a given set of tags.
    TagAtMost(TagAtMostConstraint),
    HasField(HasFieldConstraint),
    HasMethod(HasMethodConstraint),
    /// Constrain that a generic type is a function with a given return type.
    HasReturnType(GenericTypeId),
    /// Constrain that a generic type is a function whose arguments have given types.
    HasArgumentTypes(Vec<GenericTypeId>),
}
