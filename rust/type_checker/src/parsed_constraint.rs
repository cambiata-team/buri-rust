use crate::{
    constraints::{Constraint, HasMethodConstraint},
    type_schema::{CanonicalIds, TypeSchema},
    TypeId,
};
use std::collections::HashMap;
use typed_ast::{
    ConcreteFunctionType, ConcreteListType, ConcreteRecordType, ConcreteTagUnionType, ConcreteType,
    PrimitiveType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TagGroupConstraints {
    /// For tag unions that need to have at least these tags.
    OpenTags(HashMap<String, Vec<TypeId>>),
    /// For tag unions that can accept at most these tags.
    ClosedTags(HashMap<String, Vec<TypeId>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RecordConstraints {
    /// For records that need to have at least these fields.
    OpenFields(HashMap<String, TypeId>),
    /// For records that can accept at most these fields.
    ClosedFields(HashMap<String, TypeId>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FunctionConstraints {
    pub argument_types: Vec<TypeId>,
    pub return_type: TypeId,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Constraints that apply to a single data type.
enum CategoryConstraints {
    #[default]
    Unknown,
    Primitive(PrimitiveType),
    List(TypeId),
    TagGroup(TagGroupConstraints),
    Record(RecordConstraints),
    Function(FunctionConstraints),
}

impl CategoryConstraints {
    #[allow(clippy::too_many_lines)]
    pub fn is_compatible_with(&self, other: &Self, schema: &TypeSchema) -> bool {
        match (self, other) {
            (Self::Unknown, _) | (_, Self::Unknown) => true,
            (Self::Primitive(self_primitive), Self::Primitive(constraint_primitive)) => {
                self_primitive == constraint_primitive
            }
            (Self::List(self_type), Self::List(constraint_type)) => {
                schema.types_are_compatible(*self_type, *constraint_type)
            }
            (
                Self::TagGroup(TagGroupConstraints::ClosedTags(self_tags)),
                Self::TagGroup(TagGroupConstraints::ClosedTags(other_tags)),
            ) => other_tags.iter().all(|(name, types)| {
                self_tags.get(name).map_or(false, |self_types| {
                    types.iter().all(|type_id| {
                        self_types.iter().any(|self_type_id| {
                            schema.types_are_compatible(*self_type_id, *type_id)
                        })
                    })
                })
            }),
            (
                Self::TagGroup(TagGroupConstraints::OpenTags(self_tags)),
                Self::TagGroup(TagGroupConstraints::ClosedTags(other_tags)),
            ) => other_tags.iter().all(|(name, types)| {
                self_tags.get(name).map_or(true, |self_types| {
                    types.iter().all(|type_id| {
                        self_types.iter().any(|self_type_id| {
                            schema.types_are_compatible(*self_type_id, *type_id)
                        })
                    })
                })
            }),
            (
                Self::TagGroup(TagGroupConstraints::OpenTags(self_tags)),
                Self::TagGroup(TagGroupConstraints::OpenTags(other_tags)),
            ) => other_tags.iter().all(|(name, types)| {
                self_tags.get(name).map_or(true, |self_types| {
                    types.iter().all(|type_id| {
                        self_types.iter().any(|self_type_id| {
                            schema.types_are_compatible(*self_type_id, *type_id)
                        })
                    })
                })
            }),
            (
                Self::TagGroup(TagGroupConstraints::ClosedTags(self_tags)),
                Self::TagGroup(TagGroupConstraints::OpenTags(other_tags)),
            ) => other_tags.iter().all(|(name, other_types)| {
                self_tags.get(name).map_or(false, |self_types| {
                    other_types.iter().all(|other_type_id| {
                        self_types
                            .iter()
                            .any(|type_id| schema.types_are_compatible(*type_id, *other_type_id))
                    })
                })
            }),
            (
                Self::Record(RecordConstraints::ClosedFields(self_items)),
                Self::Record(RecordConstraints::ClosedFields(other_items)),
            ) => other_items.iter().all(|(name, type_id)| {
                self_items.get(name).map_or(false, |self_type_id| {
                    schema.types_are_compatible(*self_type_id, *type_id)
                })
            }),
            (
                Self::Record(RecordConstraints::OpenFields(self_items)),
                Self::Record(RecordConstraints::ClosedFields(other_items)),
            ) => other_items.iter().all(|(name, type_id)| {
                self_items.get(name).map_or(true, |self_type_id| {
                    schema.types_are_compatible(*self_type_id, *type_id)
                })
            }),
            (
                Self::Record(RecordConstraints::ClosedFields(self_items)),
                Self::Record(RecordConstraints::OpenFields(other_items)),
            ) => self_items.iter().all(|(name, type_id)| {
                other_items.get(name).map_or(false, |other_type_id| {
                    schema.types_are_compatible(*type_id, *other_type_id)
                })
            }),
            (
                Self::Record(RecordConstraints::OpenFields(self_items)),
                Self::Record(RecordConstraints::OpenFields(other_items)),
            ) => other_items.iter().all(|(name, type_id)| {
                self_items.get(name).map_or(true, |self_type_id| {
                    schema.types_are_compatible(*self_type_id, *type_id)
                })
            }),
            (
                Self::Function(FunctionConstraints {
                    argument_types: self_argument_types,
                    return_type: self_return_type,
                }),
                Self::Function(FunctionConstraints {
                    argument_types: other_argument_types,
                    return_type: other_return_type,
                }),
            ) => {
                self_argument_types.len() == other_argument_types.len()
                    && self_argument_types
                        .iter()
                        .zip(other_argument_types.iter())
                        .all(|(self_type, other_type)| {
                            schema.types_are_compatible(*self_type, *other_type)
                        })
                    && schema.types_are_compatible(*self_return_type, *other_return_type)
            }
            _ => false,
        }
    }

    pub fn update(&mut self, other: Self, ids: &CanonicalIds) {
        match (self, other) {
            (
                Self::TagGroup(TagGroupConstraints::ClosedTags(self_tags)),
                Self::TagGroup(TagGroupConstraints::ClosedTags(other_tags)),
            ) => {
                let mut new_tags = self_tags.clone();
                for (k, _) in self_tags.iter() {
                    if other_tags.get(k).is_none() {
                        new_tags.remove(k);
                    }
                }
                *self_tags = new_tags;
            }
            (
                Self::TagGroup(TagGroupConstraints::OpenTags(self_tags)),
                Self::TagGroup(TagGroupConstraints::OpenTags(other_tags)),
            ) => {
                for (k, v) in other_tags {
                    self_tags.insert(k, v.iter().map(|id| ids.get_canonical_id(*id)).collect());
                }
            }
            (
                Self::Record(RecordConstraints::ClosedFields(self_fields)),
                Self::Record(RecordConstraints::ClosedFields(other_fields)),
            ) => {
                let mut new_tags = self_fields.clone();
                for (k, _) in self_fields.iter() {
                    if other_fields.get(k).is_none() {
                        new_tags.remove(k);
                    }
                }
                *self_fields = new_tags;
            }
            (
                Self::Record(RecordConstraints::OpenFields(self_fields)),
                Self::Record(RecordConstraints::OpenFields(other_fields)),
            ) => {
                for (k, v) in other_fields {
                    self_fields.insert(k, ids.get_canonical_id(v));
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNameConstraint(Option<String>);

impl ParsedNameConstraint {
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn set(&mut self, name: String) {
        self.0 = Some(name);
    }

    pub fn update(&mut self, other: Self) {
        if other.0.is_some() {
            self.0 = other.0;
        }
    }

    pub fn is_compatible_with(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Some(self_name), Some(other_name)) => self_name == other_name,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedMethodsConstraint(HashMap<String, TypeId>);

impl ParsedMethodsConstraint {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, name: String, type_id: TypeId, ids: &CanonicalIds) {
        let canonical_id = ids.get_canonical_id(type_id);
        self.0.insert(name, canonical_id);
    }

    pub fn update(&mut self, other: Self, ids: &CanonicalIds) {
        for (name, type_id) in other.0 {
            self.add(name, type_id, ids);
        }
    }

    pub fn is_compatible_with_method(
        &self,
        method: &HasMethodConstraint,
        schema: &TypeSchema,
    ) -> bool {
        self.0.get(&method.method_name).map_or(true, |type_id| {
            schema.types_are_compatible(*type_id, method.method_type)
        })
    }

    pub fn is_compatible_with(&self, other: &Self, schema: &TypeSchema) -> bool {
        other.0.iter().all(|(name, type_id)| {
            self.is_compatible_with_method(
                &HasMethodConstraint {
                    method_name: name.clone(),
                    method_type: *type_id,
                },
                schema,
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedConstraint {
    category: CategoryConstraints,
    name: ParsedNameConstraint,
    methods: ParsedMethodsConstraint,
}

impl ParsedConstraint {
    pub fn new(constraint: Constraint, schema: &TypeSchema) -> Self {
        let mut name = ParsedNameConstraint::new();
        let mut methods = ParsedMethodsConstraint::new();
        let category = match constraint {
            Constraint::EqualToPrimitive(p) => CategoryConstraints::Primitive(p),
            Constraint::ListOfType(t) => CategoryConstraints::List(t),
            Constraint::HasTag(t) => CategoryConstraints::TagGroup(TagGroupConstraints::OpenTags(
                vec![(t.tag_name, t.tag_content_types)]
                    .into_iter()
                    .collect(),
            )),
            Constraint::TagAtMost(t) => {
                CategoryConstraints::TagGroup(TagGroupConstraints::ClosedTags(t.tags))
            }
            Constraint::HasField(f) => CategoryConstraints::Record(RecordConstraints::OpenFields(
                vec![(f.field_name, f.field_type)].into_iter().collect(),
            )),
            Constraint::FieldAtMost(f) => {
                CategoryConstraints::Record(RecordConstraints::ClosedFields(f.fields))
            }
            Constraint::HasFunctionShape(f) => CategoryConstraints::Function(FunctionConstraints {
                argument_types: f.argument_types,
                return_type: f.return_type,
            }),
            Constraint::HasMethod(m) => {
                methods.add(m.method_name, m.method_type, &schema.types);
                CategoryConstraints::Unknown
            }
            Constraint::HasName(n) => {
                name.set(n);
                CategoryConstraints::Unknown
            }
        };
        Self {
            category,
            name,
            methods,
        }
    }

    /// Use `ParsedConstraint::is_satisfied_by` before adding a constraint.
    pub fn add_constraints(&mut self, constraint: Self, ids: &CanonicalIds) {
        self.name.update(constraint.name);
        self.methods.update(constraint.methods, ids);
        if self.category == CategoryConstraints::Unknown {
            self.category = constraint.category;
        } else {
            self.category.update(constraint.category, ids);
        }
    }

    pub fn is_compatible_with(&self, other: &Self, schema: &TypeSchema) -> bool {
        self.name.is_compatible_with(&other.name)
            && self.methods.is_compatible_with(&other.methods, schema)
            && self.category.is_compatible_with(&other.category, schema)
    }

    pub fn to_concrete_type(&self, schema: &TypeSchema) -> ConcreteType {
        match &self.category {
            CategoryConstraints::Unknown => ConcreteType::Primitive(PrimitiveType::CompilerBoolean),
            CategoryConstraints::Primitive(p) => ConcreteType::Primitive(*p),
            CategoryConstraints::List(t) => ConcreteType::List(Box::new(ConcreteListType {
                element_type: schema.get_concrete_type_from_id(*t),
            })),
            CategoryConstraints::Function(f) => {
                ConcreteType::Function(Box::new(ConcreteFunctionType {
                    argument_types: f
                        .argument_types
                        .iter()
                        .map(|t| schema.get_concrete_type_from_id(*t))
                        .collect(),
                    return_type: schema.get_concrete_type_from_id(f.return_type),
                }))
            }
            CategoryConstraints::Record(
                RecordConstraints::ClosedFields(r) | RecordConstraints::OpenFields(r),
            ) => ConcreteType::Record(Box::new(ConcreteRecordType {
                field_types: r
                    .iter()
                    .map(|(name, type_id)| {
                        (name.clone(), schema.get_concrete_type_from_id(*type_id))
                    })
                    .collect(),
            })),
            CategoryConstraints::TagGroup(TagGroupConstraints::ClosedTags(t)) => {
                // Check if this is a boolean tag union
                if t.len() <= 2 {
                    let true_is_boolean = t.get("true").map_or(false, std::vec::Vec::is_empty);
                    let false_is_boolean = t.get("false").map_or(false, std::vec::Vec::is_empty);
                    if (t.len() == 2 && true_is_boolean && false_is_boolean)
                        || (t.len() == 1 && (true_is_boolean || false_is_boolean))
                    {
                        return ConcreteType::Primitive(PrimitiveType::CompilerBoolean);
                    }
                }
                ConcreteType::TagUnion(Box::new(ConcreteTagUnionType {
                    tag_types: t
                        .iter()
                        .map(|(name, type_ids)| {
                            (
                                name.clone(),
                                type_ids
                                    .iter()
                                    .map(|type_id| schema.get_concrete_type_from_id(*type_id))
                                    .collect(),
                            )
                        })
                        .collect(),
                }))
            }
            CategoryConstraints::TagGroup(TagGroupConstraints::OpenTags(t)) => {
                ConcreteType::TagUnion(Box::new(ConcreteTagUnionType {
                    tag_types: t
                        .iter()
                        .map(|(name, type_ids)| {
                            (
                                name.clone(),
                                type_ids
                                    .iter()
                                    .map(|type_id| schema.get_concrete_type_from_id(*type_id))
                                    .collect(),
                            )
                        })
                        .collect(),
                }))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constraints::{
        FieldAtMostConstraint, HasFieldConstraint, HasFunctionShape, HasMethodConstraint,
        HasTagConstraint, TagAtMostConstraint,
    };

    //
    // ParsedConstraint::new
    //

    #[test]
    fn new_parsed_constraint_with_name_constraint_sets_name() {
        let parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &TypeSchema::new());
        assert_eq!(parsed_constraint.name.0, Some("foo".to_string()));
    }

    #[test]
    fn new_parsed_constraint_with_name_constraint_has_unknown_type() {
        let parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &TypeSchema::new());
        assert_eq!(parsed_constraint.category, CategoryConstraints::Unknown);
    }

    #[test]
    fn new_parsed_constraint_with_non_name_constraint_sets_name_to_none() {
        let parsed_constraint = ParsedConstraint::new(
            Constraint::EqualToPrimitive(PrimitiveType::Num),
            &TypeSchema::new(),
        );
        assert_eq!(parsed_constraint.name.0, None);
    }

    #[test]
    fn new_parsed_constraint_with_method_constraint_sets_method() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: type_id,
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.methods.0,
            HashMap::from([("foo".to_string(), type_id)])
        );
    }

    #[test]
    fn new_parsed_constraint_with_method_constraint_has_unknown_type() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: type_id,
            }),
            &schema,
        );
        assert_eq!(parsed_constraint.category, CategoryConstraints::Unknown);
    }

    #[test]
    fn new_parsed_constraint_with_primitive_constraint_sets_primitive() {
        let parsed_constraint = ParsedConstraint::new(
            Constraint::EqualToPrimitive(PrimitiveType::Num),
            &TypeSchema::new(),
        );
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Primitive(PrimitiveType::Num)
        );
    }

    #[test]
    fn new_parsed_constraint_with_list_constraint_sets_list() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(Constraint::ListOfType(type_id), &schema);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::List(type_id)
        );
    }

    #[test]
    fn new_parsed_constraint_with_open_tag_constraint_sets_tag() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: "foo".to_string(),
                tag_content_types: vec![type_id],
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::TagGroup(TagGroupConstraints::OpenTags(
                vec![("foo".to_string(), vec![type_id])]
                    .into_iter()
                    .collect()
            ))
        );
    }

    #[test]
    fn new_parsed_constraint_with_closed_tag_constraint_sets_tag() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: vec![("foo".to_string(), vec![type_id])]
                    .into_iter()
                    .collect(),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::TagGroup(TagGroupConstraints::ClosedTags(
                vec![("foo".to_string(), vec![type_id])]
                    .into_iter()
                    .collect()
            ))
        );
    }

    #[test]
    fn new_parsed_constraint_with_open_field_constraint_sets_field() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: "foo".to_string(),
                field_type: type_id,
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Record(RecordConstraints::OpenFields(
                vec![("foo".to_string(), type_id)].into_iter().collect()
            ))
        );
    }

    #[test]
    fn new_parsed_constraint_with_closed_field_constraint_sets_field() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: vec![("foo".to_string(), type_id)].into_iter().collect(),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Record(RecordConstraints::ClosedFields(
                vec![("foo".to_string(), type_id)].into_iter().collect()
            ))
        );
    }

    #[test]
    fn new_parsed_constraint_with_open_record_constraint_sets_record() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Record(RecordConstraints::OpenFields(
                vec![("foo".to_string(), type_id)].into_iter().collect()
            ))
        );
    }

    #[test]
    fn new_parsed_constraint_with_closed_record_constraint_sets_record() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: vec![("foo".to_string(), type_id)].into_iter().collect(),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Record(RecordConstraints::ClosedFields(
                vec![("foo".to_string(), type_id)].into_iter().collect()
            ))
        );
    }

    #[test]
    fn new_parsed_constraint_with_function_constraint_sets_function() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![type_id],
                return_type: type_id,
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Function(FunctionConstraints {
                argument_types: vec![type_id],
                return_type: type_id,
            })
        );
    }

    //
    // add_constraint
    //

    #[test]
    fn add_name_constraint_sets_name() {
        let schema = TypeSchema::new();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        let new_constraint = ParsedConstraint::new(Constraint::HasName("bar".to_string()), &schema);
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(parsed_constraint.name.0, Some("bar".to_string()));
    }

    #[test]
    fn add_name_constraint_does_not_change_category() {
        let schema = TypeSchema::new();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        let new_constraint = ParsedConstraint::new(Constraint::HasName("bar".to_string()), &schema);
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert!(matches!(
            parsed_constraint.category,
            CategoryConstraints::Primitive(_)
        ));
    }

    #[test]
    fn add_method_constraint_adds_method() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        let new_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: type_id,
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.methods.0.get(&"foo".to_string()),
            Some(&type_id)
        );
    }

    #[test]
    fn adding_method_constraint_saves_canonical_id() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let canonical_id = schema.make_id();
        schema.set_types_equal(type_id, canonical_id).unwrap();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        let new_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: type_id,
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.methods.0.get(&"foo".to_string()),
            Some(&canonical_id)
        );
    }

    #[test]
    fn adding_method_constraint_does_not_change_category() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        let new_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: type_id,
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert!(matches!(
            parsed_constraint.category,
            CategoryConstraints::Primitive(_)
        ));
    }

    #[test]
    fn adding_primitive_constraint_saves_constraint() {
        let schema = TypeSchema::new();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let new_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Primitive(PrimitiveType::Num)
        );
    }

    #[test]
    fn adding_list_constraint_saves_constraint() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let new_constraint = ParsedConstraint::new(Constraint::ListOfType(type_id), &schema);
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::List(type_id)
        );
    }

    #[test]
    fn adding_tag_at_most_constraint_saves_constraint() {
        let schema = TypeSchema::new();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let new_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("bar".to_string(), Vec::new())]),
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::TagGroup(TagGroupConstraints::ClosedTags(HashMap::from([(
                "bar".to_string(),
                Vec::new()
            )])))
        );
    }

    #[test]
    fn adding_tag_at_most_constraint_when_tag_at_most_constraint_already_exists_saves_the_intersection(
    ) {
        let schema = TypeSchema::new();
        let mut parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([
                    ("foo".to_string(), Vec::new()),
                    ("bar".to_string(), Vec::new()),
                ]),
            }),
            &schema,
        );
        let new_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([
                    ("bar".to_string(), Vec::new()),
                    ("baz".to_string(), Vec::new()),
                ]),
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::TagGroup(TagGroupConstraints::ClosedTags(HashMap::from([(
                "bar".to_string(),
                Vec::new()
            )])))
        );
    }

    #[test]
    fn adding_has_tag_constraint_saves_tag() {
        let schema = TypeSchema::new();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let new_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: "bar".to_string(),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::TagGroup(TagGroupConstraints::OpenTags(HashMap::from([(
                "bar".to_string(),
                Vec::new()
            )])))
        );
    }

    #[test]
    fn adding_has_tag_constraint_when_has_tag_constraint_already_exists_saves_the_union() {
        let schema = TypeSchema::new();
        let mut parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: "foo".to_string(),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let new_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: "bar".to_string(),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::TagGroup(TagGroupConstraints::OpenTags(HashMap::from([
                ("foo".to_string(), vec![]),
                ("bar".to_string(), vec![])
            ])))
        );
    }

    #[test]
    fn adding_at_most_field_constraint_saves_constraint() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let new_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([("bar".to_string(), type_id)]),
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Record(RecordConstraints::ClosedFields(HashMap::from([(
                "bar".to_string(),
                type_id
            )])))
        );
    }

    #[test]
    fn adding_at_most_field_constraint_when_at_most_field_constraint_already_exists_saves_the_intersection(
    ) {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([("foo".to_string(), type_id), ("bar".to_string(), type_id)]),
            }),
            &schema,
        );
        let new_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([("bar".to_string(), type_id), ("baz".to_string(), type_id)]),
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Record(RecordConstraints::ClosedFields(HashMap::from([(
                "bar".to_string(),
                type_id
            )])))
        );
    }

    #[test]
    fn adding_has_field_constraint_saves_field() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let new_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: "bar".to_string(),
                field_type: type_id,
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Record(RecordConstraints::OpenFields(HashMap::from([(
                "bar".to_string(),
                type_id
            )])))
        );
    }

    #[test]
    fn adding_has_field_constraint_when_has_field_constraint_already_exists_saves_the_union() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: "foo".to_string(),
                field_type: type_id,
            }),
            &schema,
        );
        let new_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: "bar".to_string(),
                field_type: type_id,
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Record(RecordConstraints::OpenFields(HashMap::from([
                ("foo".to_string(), type_id),
                ("bar".to_string(), type_id)
            ])))
        );
    }

    #[test]
    fn adding_function_shape_constraint_saves_constraint() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let new_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type: type_id,
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        assert_eq!(
            parsed_constraint.category,
            CategoryConstraints::Function(FunctionConstraints {
                argument_types: Vec::new(),
                return_type: type_id
            })
        );
    }

    //
    // is_compatible_with
    //

    #[test]
    fn is_compatible_with_name_constraint_when_it_does_not_have_a_name() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        let other_constraint =
            ParsedConstraint::new(Constraint::HasName("bar".to_string()), &schema);
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_name_constraint_if_it_matches_current_name() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let other_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_name_if_it_does_not_match_current_name() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let other_constraint =
            ParsedConstraint::new(Constraint::HasName("bar".to_string()), &schema);
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_method_constraint_of_same_name_and_type() {
        let mut schema = TypeSchema::new();
        let method_type = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_method_constraint_of_same_name_and_canonical_type_id() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: type_b,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_method_constraint_of_different_name() {
        let mut schema = TypeSchema::new();
        let foo_type = schema.make_id();
        let bar_type = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: foo_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "bar".to_string(),
                method_type: bar_type,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_method_constraint_whose_types_are_compatible() {
        let mut schema = TypeSchema::new();
        let foo_type = schema.make_id();
        let bar_type = schema.make_id();
        schema
            .add_constraint(foo_type, Constraint::HasName("foo".to_string()))
            .unwrap();
        schema
            .add_constraint(bar_type, Constraint::HasName("foo".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: foo_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: bar_type,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_method_constraint_whose_types_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let foo_type = schema.make_id();
        let bar_type = schema.make_id();
        schema
            .add_constraint(foo_type, Constraint::HasName("foo".to_string()))
            .unwrap();
        schema
            .add_constraint(bar_type, Constraint::HasName("bar".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: foo_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasMethod(HasMethodConstraint {
                method_name: "foo".to_string(),
                method_type: bar_type,
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_primitive_constraint_if_it_matches_current_primitive() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        let other_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_primitive_constraint_if_it_does_not_match_current_primitive() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        let other_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Str), &schema);
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_primitive_constraint_when_current_category_is_unknown() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::HasName("foo".to_string()), &schema);
        let other_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_primitive_constraint_when_current_category_is_not_primitive() {
        let mut schema = TypeSchema::new();
        let list_type = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(Constraint::ListOfType(list_type), &schema);
        let other_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_list_constraint_of_same_type() {
        let mut schema = TypeSchema::new();
        let list_type = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(Constraint::ListOfType(list_type), &schema);
        let other_constraint = ParsedConstraint::new(Constraint::ListOfType(list_type), &schema);
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_list_constraint_of_same_canonical_type_id() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(Constraint::ListOfType(type_a), &schema);
        let other_constraint = ParsedConstraint::new(Constraint::ListOfType(type_b), &schema);
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_list_constraint_whose_type_is_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(Constraint::ListOfType(type_a), &schema);
        let other_constraint = ParsedConstraint::new(Constraint::ListOfType(type_b), &schema);
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_list_constraint_of_different_type() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(Constraint::ListOfType(type_a), &schema);
        let other_constraint = ParsedConstraint::new(Constraint::ListOfType(type_b), &schema);
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_with_same_tags() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), Vec::new())]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), Vec::new())]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_that_is_a_subset_of_current_tags() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([
                    ("foo".to_string(), Vec::new()),
                    ("bar".to_string(), Vec::new()),
                ]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), Vec::new())]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_tag_at_most_constraint_that_does_not_overlap_with_current_tags() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), Vec::new())]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("bar".to_string(), Vec::new())]),
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_tag_at_most_constraint_when_tag_has_different_contents() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), Vec::new())]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_id])]),
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_when_contents_has_same_type_id() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_id])]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_id])]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_when_contents_has_same_canonical_id() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_a])]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_b])]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_when_contents_are_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_a])]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_b])]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_tag_at_most_constraint_when_contents_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_a])]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([("foo".to_string(), vec![type_b])]),
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_with_same_open_tags() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_that_is_a_subset_of_current_open_tags() {
        let schema = TypeSchema::new();
        let mut parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let new_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("bar"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_that_does_not_overlap_with_current_open_tags() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("bar"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_tag_at_most_constraint_when_open_tag_has_different_contents() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_id],
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_when_open_tag_contents_has_same_type_id() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_id],
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_id],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_when_open_tag_contents_has_same_canonical_id() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_a],
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_tag_at_most_constraint_when_open_tag_contents_are_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_a],
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_tag_at_most_constraint_when_open_tag_contents_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_a],
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_tags_are_the_same() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_tag_is_in_tag_group() {
        let schema = TypeSchema::new();
        let mut parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let new_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("bar"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("bar"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_tag_is_not_in_tag_group() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("bar"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_tag_constraint_when_contents_are_different() {
        let mut schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![schema.make_id()],
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_contents_have_same_type_id() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_id],
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_id],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_contents_have_same_canonical_id() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_a],
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_contents_are_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_a],
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_tag_constraint_when_contents_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_a],
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_closed_tags_are_the_same() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("foo"), Vec::new())]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_tag_is_in_closed_tag_group() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([
                    (String::from("foo"), Vec::new()),
                    (String::from("bar"), Vec::new()),
                ]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_tag_constraint_when_tag_is_not_in_closed_tag_group() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("foo"), Vec::new())]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("bar"),
                tag_content_types: Vec::new(),
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_tag_constraint_when_closed_group_contents_are_different() {
        let mut schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("foo"), Vec::new())]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![schema.make_id()],
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_closed_group_contents_have_same_type_id() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("foo"), vec![type_id])]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_id],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_closed_group_contents_have_same_canonical_id() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("foo"), vec![type_a])]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_tag_constraint_when_closed_group_contents_are_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("foo"), vec![type_a])]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_tag_constraint_when_closed_group_contents_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("foo"), vec![type_a])]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![type_b],
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_fields_are_the_same() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_is_subset_of_current_fields() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([
                    (String::from("foo"), type_a),
                    (String::from("bar"), type_b),
                ]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_field_at_most_constraint_when_field_not_in_current_fields() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("bar"), type_b)]),
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_type_ids_are_the_same() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_canonical_ids_are_the_same() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_b)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_types_are_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_b)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_field_at_most_constraint_when_types_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_b)]),
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_open_fields_are_the_same() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_is_subset_of_open_fields() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let mut parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_a,
            }),
            &schema,
        );
        let new_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("bar"),
                field_type: type_b,
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_field_not_in_open_fields() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("bar"), type_b)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_open_field_type_ids_are_the_same() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_open_field_canonical_ids_are_the_same() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_b)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_field_at_most_constraint_when_open_field_types_are_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_b)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_field_at_most_constraint_when_open_field_types_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_b)]),
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_types_are_the_same() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_field_is_a_subset_of_fields() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let mut parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        let new_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("bar"),
                field_type: type_id,
            }),
            &schema,
        );
        parsed_constraint.add_constraints(new_constraint, &schema.types);
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_field_is_not_in_fields() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("bar"),
                field_type: type_id,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_fields_have_same_type_id() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_fields_have_same_canonical_id() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_b,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_fields_types_are_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_b,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_field_constraint_when_field_types_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_b,
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_closed_fields_are_the_same() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_field_is_a_subset_of_closed_fields() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([
                    (String::from("foo"), type_id),
                    (String::from("bar"), type_id),
                ]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_field_constraint_when_field_is_not_in_closed_fields() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("bar"), type_id)]),
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_closed_fields_have_same_type_id() {
        let mut schema = TypeSchema::new();
        let type_id = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_id)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_id,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_closed_fields_have_same_canonical_id() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema.set_types_equal(type_a, type_b).unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_b,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_field_constraint_when_closed_fields_types_are_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_b,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_field_constraint_when_closed_field_types_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        schema
            .add_constraint(type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), type_a)]),
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type: type_b,
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_function_shape_constraint_when_return_types_are_the_same() {
        let mut schema = TypeSchema::new();
        let return_type = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_function_shape_constraint_when_return_types_have_same_canonical_id() {
        let mut schema = TypeSchema::new();
        let return_type_a = schema.make_id();
        let return_type_b = schema.make_id();
        schema
            .set_types_equal(return_type_a, return_type_b)
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type: return_type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type: return_type_b,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_function_shape_constraint_when_return_types_are_compatible() {
        let mut schema = TypeSchema::new();
        let return_type_a = schema.make_id();
        let return_type_b = schema.make_id();
        schema
            .add_constraint(return_type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(return_type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type: return_type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type: return_type_b,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_function_shape_constraint_when_return_types_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let return_type_a = schema.make_id();
        let return_type_b = schema.make_id();
        schema
            .add_constraint(return_type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(return_type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type: return_type_a,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type: return_type_b,
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_function_shape_constraint_when_arguments_have_different_lengths()
    {
        let mut schema = TypeSchema::new();
        let return_type = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: Vec::new(),
                return_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![schema.make_id()],
                return_type,
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_function_shape_constraint_when_arguments_have_same_types() {
        let mut schema = TypeSchema::new();
        let return_type = schema.make_id();
        let argument_type = schema.make_id();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type],
                return_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type],
                return_type,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_function_shape_constraint_when_arguments_have_same_canonical_ids() {
        let mut schema = TypeSchema::new();
        let return_type = schema.make_id();
        let argument_type_a = schema.make_id();
        let argument_type_b = schema.make_id();
        schema
            .set_types_equal(argument_type_a, argument_type_b)
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type_a],
                return_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type_b],
                return_type,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_compatible_with_has_function_shape_constraint_when_arguments_are_compatible() {
        let mut schema = TypeSchema::new();
        let return_type = schema.make_id();
        let argument_type_a = schema.make_id();
        let argument_type_b = schema.make_id();
        schema
            .add_constraint(argument_type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(argument_type_b, Constraint::HasName("a".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type_a],
                return_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type_b],
                return_type,
            }),
            &schema,
        );
        assert!(parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    #[test]
    fn is_not_compatible_with_has_function_shape_constraint_when_arguments_are_not_compatible() {
        let mut schema = TypeSchema::new();
        let return_type = schema.make_id();
        let argument_type_a = schema.make_id();
        let argument_type_b = schema.make_id();
        schema
            .add_constraint(argument_type_a, Constraint::HasName("a".to_string()))
            .unwrap();
        schema
            .add_constraint(argument_type_b, Constraint::HasName("b".to_string()))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type_a],
                return_type,
            }),
            &schema,
        );
        let other_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type_b],
                return_type,
            }),
            &schema,
        );
        assert!(!parsed_constraint.is_compatible_with(&other_constraint, &schema));
    }

    //
    // to_concrete_type
    //

    #[test]
    fn unknown_category_becomes_compiler_boolean() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::HasName(String::from("foo")), &schema);
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Primitive(PrimitiveType::CompilerBoolean)
        );
    }

    #[test]
    fn number_to_concrete_type() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Num), &schema);
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Primitive(PrimitiveType::Num)
        );
    }

    #[test]
    fn string_to_concrete_type() {
        let schema = TypeSchema::new();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::EqualToPrimitive(PrimitiveType::Str), &schema);
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Primitive(PrimitiveType::Str)
        );
    }

    #[test]
    fn list_to_concrete_type() {
        let mut schema = TypeSchema::new();
        let element_type = schema.make_id();
        schema
            .add_constraint(
                element_type,
                Constraint::EqualToPrimitive(PrimitiveType::Num),
            )
            .unwrap();
        let parsed_constraint =
            ParsedConstraint::new(Constraint::ListOfType(element_type), &schema);
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::List(Box::new(ConcreteListType {
                element_type: ConcreteType::Primitive(PrimitiveType::Num)
            }))
        );
    }

    #[test]
    fn function_to_concrete_type() {
        let mut schema = TypeSchema::new();
        let return_type = schema.make_id();
        let argument_type = schema.make_id();
        schema
            .add_constraint(
                return_type,
                Constraint::EqualToPrimitive(PrimitiveType::Num),
            )
            .unwrap();
        schema
            .add_constraint(
                argument_type,
                Constraint::EqualToPrimitive(PrimitiveType::Str),
            )
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: vec![argument_type],
                return_type,
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Function(Box::new(ConcreteFunctionType {
                argument_types: vec![ConcreteType::Primitive(PrimitiveType::Str)],
                return_type: ConcreteType::Primitive(PrimitiveType::Num)
            }))
        );
    }

    #[test]
    fn closed_record_to_concrete_type() {
        let mut schema = TypeSchema::new();
        let field_type = schema.make_id();
        schema
            .add_constraint(field_type, Constraint::EqualToPrimitive(PrimitiveType::Num))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::FieldAtMost(FieldAtMostConstraint {
                fields: HashMap::from([(String::from("foo"), field_type)]),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Record(Box::new(ConcreteRecordType {
                field_types: HashMap::from([(
                    String::from("foo"),
                    ConcreteType::Primitive(PrimitiveType::Num)
                )])
            }))
        );
    }

    #[test]
    fn open_record_to_concrete_type() {
        let mut schema = TypeSchema::new();
        let field_type = schema.make_id();
        schema
            .add_constraint(field_type, Constraint::EqualToPrimitive(PrimitiveType::Num))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasField(HasFieldConstraint {
                field_name: String::from("foo"),
                field_type,
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Record(Box::new(ConcreteRecordType {
                field_types: HashMap::from([(
                    String::from("foo"),
                    ConcreteType::Primitive(PrimitiveType::Num)
                )])
            }))
        );
    }

    #[test]
    fn closed_tag_union_to_concrete_type() {
        let mut schema = TypeSchema::new();
        let tag_type = schema.make_id();
        schema
            .add_constraint(tag_type, Constraint::EqualToPrimitive(PrimitiveType::Num))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("foo"), vec![tag_type])]),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::TagUnion(Box::new(ConcreteTagUnionType {
                tag_types: HashMap::from([(
                    String::from("foo"),
                    vec![ConcreteType::Primitive(PrimitiveType::Num)]
                )])
            }))
        );
    }

    #[test]
    fn open_tag_union_to_concrete_type() {
        let mut schema = TypeSchema::new();
        let tag_type = schema.make_id();
        schema
            .add_constraint(tag_type, Constraint::EqualToPrimitive(PrimitiveType::Num))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::HasTag(HasTagConstraint {
                tag_name: String::from("foo"),
                tag_content_types: vec![tag_type],
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::TagUnion(Box::new(ConcreteTagUnionType {
                tag_types: HashMap::from([(
                    String::from("foo"),
                    vec![ConcreteType::Primitive(PrimitiveType::Num)]
                )])
            }))
        );
    }

    #[test]
    fn close_tag_to_compiler_boolean_for_true() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("true"), Vec::new())]),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Primitive(PrimitiveType::CompilerBoolean)
        );
    }

    #[test]
    fn close_tag_to_compiler_boolean_for_false() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("false"), Vec::new())]),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Primitive(PrimitiveType::CompilerBoolean)
        );
    }

    #[test]
    fn close_tag_to_compiler_boolean_for_true_and_false() {
        let schema = TypeSchema::new();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([
                    (String::from("true"), Vec::new()),
                    (String::from("false"), Vec::new()),
                ]),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::Primitive(PrimitiveType::CompilerBoolean)
        );
    }

    #[test]
    fn close_tag_to_concrete_tag_union_if_true_has_contents() {
        let mut schema = TypeSchema::new();
        let tag_type = schema.make_id();
        schema
            .add_constraint(tag_type, Constraint::EqualToPrimitive(PrimitiveType::Num))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("true"), vec![tag_type])]),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::TagUnion(Box::new(ConcreteTagUnionType {
                tag_types: HashMap::from([(
                    String::from("true"),
                    vec![ConcreteType::Primitive(PrimitiveType::Num)]
                )])
            }))
        );
    }

    #[test]
    fn close_tag_to_concrete_tag_union_if_false_has_contents() {
        let mut schema = TypeSchema::new();
        let tag_type = schema.make_id();
        schema
            .add_constraint(tag_type, Constraint::EqualToPrimitive(PrimitiveType::Num))
            .unwrap();
        let parsed_constraint = ParsedConstraint::new(
            Constraint::TagAtMost(TagAtMostConstraint {
                tags: HashMap::from([(String::from("false"), vec![tag_type])]),
            }),
            &schema,
        );
        assert_eq!(
            parsed_constraint.to_concrete_type(&schema),
            ConcreteType::TagUnion(Box::new(ConcreteTagUnionType {
                tag_types: HashMap::from([(
                    String::from("false"),
                    vec![ConcreteType::Primitive(PrimitiveType::Num)]
                )])
            }))
        );
    }
}
