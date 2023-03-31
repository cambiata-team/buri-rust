use crate::TypeId;
use std::collections::HashMap;
use typed_ast::PrimitiveType;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type is an enum with at least a specific
/// variants with a specific set of types.
pub struct HasEnumVariantConstraint {
    pub name: String,
    pub payload: Vec<TypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type is an enum with a specific
/// set of variants, each of which has a specific set of types.
pub struct EnumExactConstraint {
    /// The keys are the names of the variants in the enum.
    /// The values are the types of the variant payloads.
    pub variants: HashMap<String, Vec<TypeId>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type is a tag union
/// with a tag of given name whose payload has particular types.
pub struct HasTagConstraint {
    pub tag_name: String,
    pub tag_content_types: Vec<TypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constraint that a generic type is a tag union whose tags are a subset
/// of a given set of tags.
pub struct TagAtMostConstraint {
    /// The keys are the names of the tags in a tag union.
    /// The values are the types of the tag payloads.
    pub tags: HashMap<String, Vec<TypeId>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type be a record with a field named `field_name`,
/// the type of which is `field_type`.
pub struct HasFieldConstraint {
    pub field_name: String,
    pub field_type: TypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HasExactFieldsConstraint {
    pub fields: HashMap<String, TypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Constrain that a generic type be a record with a method named `method_name`,
/// the type of which is `method_type`.
pub struct HasMethodConstraint {
    pub method_name: String,
    pub method_type: TypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HasFunctionShape {
    pub argument_types: Vec<TypeId>,
    pub return_type: TypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    /// Constrain that a generic type be equal to some primitive type.
    EqualToPrimitive(PrimitiveType),
    /// Constrain that a generic type is a list whose contents have a particular type.
    ListOfType(TypeId),
    /// Constrain that a generic type is a tag union with at least a given set of tags.
    HasTag(HasTagConstraint),
    /// Constrain that a generic type is a tag union with at most a given set of tags.
    TagAtMost(TagAtMostConstraint),
    /// Constrain that a generic type is an enum with at least a given set of variants.
    HasVariant(HasEnumVariantConstraint),
    /// Constrain that a generic type is an enum with exactly a given set of variants.
    EnumExact(EnumExactConstraint),
    HasField(HasFieldConstraint),
    HasExactFields(HasExactFieldsConstraint),
    HasMethod(HasMethodConstraint),
    /// Constrain that a generic type is a function with a given return type.
    HasFunctionShape(HasFunctionShape),
    // If the type must match a particular type identifier.
    HasName(String),
}
