use crate::{
    constraints::{
        Constraint, HasFieldConstraint, HasMethodConstraint, HasTagConstraint, TagAtMostConstraint,
    },
    type_schema::TypeSchema,
    GenericTypeId,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSchemaSubstitutions {
    /// Implement a disjoint set / union find data structure.
    /// The mapping from key `x` to value `y` signifies that `y` is the parent of `x`.
    /// If `x` is its own parent, then it is a root node of a set.
    /// The id of a root node in a set is considered the canonical name of the every id in the set.
    disjoint_set: Vec<GenericTypeId>,
}

impl TypeSchemaSubstitutions {
    pub const fn new() -> Self {
        Self {
            disjoint_set: Vec::new(),
        }
    }
    pub fn insert_new_id(&mut self, typ: GenericTypeId) {
        while self.disjoint_set.len() <= typ {
            self.disjoint_set.push(self.disjoint_set.len());
        }
    }
    pub fn get_canonical_id(&mut self, typ: GenericTypeId) -> GenericTypeId {
        match self.disjoint_set.get(typ) {
            None => typ,
            Some(parent_id_ref) => {
                let parent_id = *parent_id_ref;
                if parent_id == typ {
                    typ
                } else {
                    let canonical_id = self.get_canonical_id(parent_id);
                    self.disjoint_set[typ] = canonical_id;
                    canonical_id
                }
            }
        }
    }
    pub fn count_ids(&self) -> usize {
        self.disjoint_set.len()
    }
    pub fn count_canonical_ids(&mut self) -> usize {
        let mut canonical_ids = HashSet::new();
        for typ in 0..self.count_ids() {
            canonical_ids.insert(self.get_canonical_id(typ));
        }
        canonical_ids.len()
    }
    pub fn set_types_equal(&mut self, type_a: GenericTypeId, type_b: GenericTypeId) {
        let canonical_a = self.get_canonical_id(type_a);
        let canonical_b = self.get_canonical_id(type_b);
        self.disjoint_set[canonical_a] = canonical_b;
    }
    fn apply_to_vec(&mut self, input: Vec<GenericTypeId>) -> Vec<GenericTypeId> {
        input
            .into_iter()
            .map(|typ| self.get_canonical_id(typ))
            .collect()
    }
    fn apply_to_constraint(&mut self, constraint: Constraint) -> Constraint {
        match constraint {
            Constraint::EqualToConcrete(x) => Constraint::EqualToConcrete(x),
            Constraint::ListOfType(element_type) => {
                Constraint::ListOfType(self.get_canonical_id(element_type))
            }
            Constraint::HasTag(has_tag_constraint) => Constraint::HasTag(HasTagConstraint {
                tag_name: has_tag_constraint.tag_name,
                tag_content_types: self.apply_to_vec(has_tag_constraint.tag_content_types),
            }),
            Constraint::TagAtMost(tag_at_most_constraint) => {
                let mut new_constraint = TagAtMostConstraint {
                    tags: HashMap::new(),
                };
                for (tag_name, tag_content_types) in tag_at_most_constraint.tags {
                    new_constraint
                        .tags
                        .insert(tag_name, self.apply_to_vec(tag_content_types));
                }
                Constraint::TagAtMost(new_constraint)
            }
            Constraint::HasField(has_field_constraint) => {
                Constraint::HasField(HasFieldConstraint {
                    field_name: has_field_constraint.field_name,
                    field_type: self.get_canonical_id(has_field_constraint.field_type),
                })
            }
            Constraint::HasMethod(has_method_constraint) => {
                Constraint::HasMethod(HasMethodConstraint {
                    method_name: has_method_constraint.method_name,
                    method_type: self.get_canonical_id(has_method_constraint.method_type),
                })
            }
            Constraint::HasReturnType(return_type) => {
                Constraint::HasReturnType(self.get_canonical_id(return_type))
            }
            Constraint::HasArgumentTypes(argument_types) => {
                Constraint::HasArgumentTypes(self.apply_to_vec(argument_types))
            }
        }
    }
    fn apply_to_constraints_map(
        &mut self,
        input: HashMap<GenericTypeId, Vec<Constraint>>,
    ) -> HashMap<GenericTypeId, Vec<Constraint>> {
        let mut output: HashMap<GenericTypeId, Vec<Constraint>> = HashMap::new();
        for (key, value) in input {
            let canonical = self.get_canonical_id(key);
            let mut new_constraints = value
                .into_iter()
                .map(|constraint| self.apply_to_constraint(constraint))
                .collect();
            match output.get_mut(&canonical) {
                Some(output_vec) => output_vec.append(&mut new_constraints),
                None => {
                    output.insert(canonical, new_constraints);
                }
            }
        }
        output
    }
    fn apply_to_imports_map(
        &mut self,
        input: HashMap<String, GenericTypeId>,
    ) -> HashMap<String, GenericTypeId> {
        let mut output = HashMap::new();
        for (key, value) in input {
            output.insert(key, self.get_canonical_id(value));
        }
        output
    }
    pub fn apply_to_type_schema(&mut self, input: TypeSchema) -> TypeSchema {
        TypeSchema {
            next_id: input.next_id,
            constraints: self.apply_to_constraints_map(input.constraints),
            imports: self.apply_to_imports_map(input.imports),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn type_is_its_own_canonical_id() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        substitutions.insert_new_id(3);
        assert_eq!(substitutions.get_canonical_id(3), 3);
    }

    #[test]
    fn two_types_each_have_their_own_canonical_id() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        substitutions.insert_new_id(3);
        substitutions.insert_new_id(5);
        assert_eq!(substitutions.get_canonical_id(3), 3);
        assert_eq!(substitutions.get_canonical_id(5), 5);
    }

    #[test]
    fn when_two_types_are_made_equal_they_have_the_same_canonical_id() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        substitutions.insert_new_id(3);
        substitutions.insert_new_id(5);
        substitutions.set_types_equal(3, 5);
        assert_eq!(
            substitutions.get_canonical_id(3),
            substitutions.get_canonical_id(5)
        );
    }

    #[test]
    fn equality_is_commutative() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        substitutions.insert_new_id(3);
        substitutions.insert_new_id(5);
        substitutions.insert_new_id(7);
        substitutions.set_types_equal(3, 5);
        substitutions.set_types_equal(5, 7);
        assert_eq!(
            substitutions.get_canonical_id(3),
            substitutions.get_canonical_id(7)
        );
    }

    #[test]
    fn when_two_types_are_set_equal_other_types_still_have_a_different_canonical_id() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        substitutions.insert_new_id(3);
        substitutions.insert_new_id(5);
        substitutions.insert_new_id(7);
        substitutions.set_types_equal(3, 5);
        assert_ne!(
            substitutions.get_canonical_id(3),
            substitutions.get_canonical_id(7)
        );
        assert_ne!(
            substitutions.get_canonical_id(5),
            substitutions.get_canonical_id(7)
        );
    }

    #[test]
    fn setting_two_types_equal_decrements_the_number_of_canonical_ids_by_one() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        substitutions.insert_new_id(3);
        substitutions.insert_new_id(5);
        let old_canonical_id_count = substitutions.count_canonical_ids();
        substitutions.set_types_equal(3, 5);
        let new_canonical_id_count = substitutions.count_canonical_ids();
        assert_eq!(old_canonical_id_count, new_canonical_id_count + 1);
    }

    #[test]
    fn apply_to_type_schema_replaces_all_instances_of_a_given_id_in_constraint_keys() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let type_c = schema.make_id();
        substitutions.insert_new_id(type_a);
        substitutions.insert_new_id(type_b);
        substitutions.insert_new_id(type_c);
        schema.constraints.insert(
            type_a,
            vec![Constraint::HasField(HasFieldConstraint {
                field_name: "hello".to_owned(),
                field_type: type_b,
            })],
        );
        schema.constraints.insert(
            type_b,
            vec![Constraint::HasMethod(HasMethodConstraint {
                method_name: "world".to_owned(),
                method_type: type_c,
            })],
        );
        schema
            .constraints
            .insert(type_c, vec![Constraint::HasReturnType(type_a)]);
        substitutions.set_types_equal(type_a, type_b);
        let new_schema = substitutions.apply_to_type_schema(schema);
        assert_eq!(new_schema.constraints.len(), 2);
        assert_eq!(
            new_schema
                .constraints
                .get(&substitutions.get_canonical_id(type_a))
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            new_schema
                .constraints
                .get(&substitutions.get_canonical_id(type_b))
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn apply_to_type_schema_does_not_replace_non_substituted_ids_in_constraint_keys() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let type_c = schema.make_id();
        substitutions.insert_new_id(type_a);
        substitutions.insert_new_id(type_b);
        substitutions.insert_new_id(type_c);
        schema.constraints.insert(
            type_a,
            vec![Constraint::HasField(HasFieldConstraint {
                field_name: "hello".to_owned(),
                field_type: type_b,
            })],
        );
        schema.constraints.insert(
            type_b,
            vec![Constraint::HasMethod(HasMethodConstraint {
                method_name: "world".to_owned(),
                method_type: type_c,
            })],
        );
        schema
            .constraints
            .insert(type_c, vec![Constraint::HasReturnType(type_a)]);
        substitutions.set_types_equal(type_a, type_b);
        let new_schema = substitutions.apply_to_type_schema(schema);
        assert_eq!(
            new_schema
                .constraints
                .get(&substitutions.get_canonical_id(type_c))
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn apply_to_type_schema_replaces_all_instances_of_a_given_id_in_constraint_values() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let type_c = schema.make_id();
        substitutions.insert_new_id(type_a);
        substitutions.insert_new_id(type_b);
        substitutions.insert_new_id(type_c);
        schema.constraints.insert(
            type_a,
            vec![Constraint::HasField(HasFieldConstraint {
                field_name: "hello".to_owned(),
                field_type: type_b,
            })],
        );
        schema.constraints.insert(
            type_b,
            vec![Constraint::HasMethod(HasMethodConstraint {
                method_name: "world".to_owned(),
                method_type: type_c,
            })],
        );
        schema
            .constraints
            .insert(type_c, vec![Constraint::HasReturnType(type_a)]);
        substitutions.set_types_equal(type_a, type_b);
        let new_schema = substitutions.apply_to_type_schema(schema);
        for (_, constraint_vec) in new_schema.constraints {
            for constraint in constraint_vec {
                match constraint {
                    Constraint::HasField(has_field_constraint) => {
                        assert_eq!(
                            substitutions.get_canonical_id(has_field_constraint.field_type),
                            substitutions.get_canonical_id(type_a)
                        );
                        assert_eq!(
                            substitutions.get_canonical_id(has_field_constraint.field_type),
                            substitutions.get_canonical_id(type_b)
                        );
                    }
                    Constraint::HasReturnType(return_type) => {
                        assert_eq!(
                            substitutions.get_canonical_id(return_type),
                            substitutions.get_canonical_id(type_a)
                        );
                        assert_eq!(
                            substitutions.get_canonical_id(return_type),
                            substitutions.get_canonical_id(type_b)
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn apply_to_type_schema_does_not_replace_non_substituted_ids_in_constraint_values() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let type_c = schema.make_id();
        substitutions.insert_new_id(type_a);
        substitutions.insert_new_id(type_b);
        substitutions.insert_new_id(type_c);
        schema.constraints.insert(
            type_a,
            vec![Constraint::HasField(HasFieldConstraint {
                field_name: "hello".to_owned(),
                field_type: type_b,
            })],
        );
        schema.constraints.insert(
            type_b,
            vec![Constraint::HasMethod(HasMethodConstraint {
                method_name: "world".to_owned(),
                method_type: type_c,
            })],
        );
        schema
            .constraints
            .insert(type_c, vec![Constraint::HasReturnType(type_a)]);
        substitutions.set_types_equal(type_a, type_b);
        let new_schema = substitutions.apply_to_type_schema(schema);
        for (_, constraint_vec) in new_schema.constraints {
            for constraint in constraint_vec {
                match constraint {
                    Constraint::HasMethod(has_method_constraint) => {
                        assert_eq!(
                            substitutions.get_canonical_id(has_method_constraint.method_type),
                            substitutions.get_canonical_id(type_c)
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn apply_to_type_schema_replaces_all_instances_of_a_given_id_in_import_values() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let type_c = schema.make_id();
        substitutions.insert_new_id(type_a);
        substitutions.insert_new_id(type_b);
        substitutions.insert_new_id(type_c);
        schema.imports.insert("apple".to_owned(), type_a);
        schema.imports.insert("banana".to_owned(), type_b);
        schema.imports.insert("carrot".to_owned(), type_c);
        substitutions.set_types_equal(type_a, type_b);
        assert_eq!(
            substitutions.get_canonical_id(*schema.imports.get("apple").unwrap()),
            substitutions.get_canonical_id(type_a)
        );
        assert_eq!(
            substitutions.get_canonical_id(*schema.imports.get("apple").unwrap()),
            substitutions.get_canonical_id(type_b)
        );
        assert_eq!(
            substitutions.get_canonical_id(*schema.imports.get("banana").unwrap()),
            substitutions.get_canonical_id(type_a)
        );
        assert_eq!(
            substitutions.get_canonical_id(*schema.imports.get("banana").unwrap()),
            substitutions.get_canonical_id(type_b)
        );
    }

    #[test]
    fn apply_to_type_schema_does_not_replace_non_substituted_ids_in_import_values() {
        let mut substitutions = TypeSchemaSubstitutions::new();
        let mut schema = TypeSchema::new();
        let type_a = schema.make_id();
        let type_b = schema.make_id();
        let type_c = schema.make_id();
        substitutions.insert_new_id(type_a);
        substitutions.insert_new_id(type_b);
        substitutions.insert_new_id(type_c);
        schema.imports.insert("apple".to_owned(), type_a);
        schema.imports.insert("banana".to_owned(), type_b);
        schema.imports.insert("carrot".to_owned(), type_c);
        substitutions.set_types_equal(type_a, type_b);
        assert_eq!(
            substitutions.get_canonical_id(*schema.imports.get("carrot").unwrap()),
            substitutions.get_canonical_id(type_c)
        );
    }
}
