use crate::{
    constraints::{Constraint, HasMethodConstraint},
    parsed_constraint::ParsedConstraint,
    scope::Scope,
    type_checking_call_stack::CheckedTypes,
    TypeId,
};
use std::collections::HashMap;
use type_checker_errors::generate_backtrace_error;
use typed_ast::{ConcreteType, PrimitiveType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CanonicalIds(Vec<TypeId>);

pub const INT_TYPE_ID: usize = 0;
pub const STR_TYPE_ID: usize = 1;

impl CanonicalIds {
    const fn new() -> Self {
        Self(Vec::new())
    }

    /// Return an id which is unique in this `TypeSchema`.
    fn make_id(&mut self) -> TypeId {
        let id = self.0.len();
        self.0.push(id);
        id
    }

    #[must_use]
    pub fn get_canonical_id(&self, mut type_id: TypeId) -> TypeId {
        loop {
            let parent_id = self.0[type_id];
            if parent_id == type_id {
                return type_id;
            }
            type_id = parent_id;
        }
    }
    fn count_ids(&self) -> usize {
        self.0.len()
    }

    fn get_total_canonical_ids(&mut self) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter(|(index, canonical_id)| index == *canonical_id)
            .count()
    }

    fn set_types_equal(&mut self, type_a: TypeId, type_b: TypeId) {
        let canonical_a = self.get_canonical_id(type_a);
        let canonical_b = self.get_canonical_id(type_b);
        self.0[canonical_b] = canonical_a;
        // Makes future canonical id lookups faster.
        self.0[type_a] = canonical_a;
        self.0[type_b] = canonical_a;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeSchema {
    pub types: CanonicalIds,
    constraints: HashMap<TypeId, ParsedConstraint>,
    pub scope: Scope,
}

impl TypeSchema {
    #[must_use]
    #[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
    pub fn new() -> Self {
        let mut schema = Self {
            types: CanonicalIds::new(),
            constraints: HashMap::new(),
            scope: Scope::new(),
        };
        // if-change: update type id constants at the top of the file
        schema
            .declare_identifier_with_constraint(
                String::from("Int"),
                Constraint::EqualToPrimitive(PrimitiveType::Int),
                &mut CheckedTypes::new(),
            )
            .unwrap();
        schema
            .declare_identifier_with_constraint(
                String::from("Str"),
                Constraint::EqualToPrimitive(PrimitiveType::Str),
                &mut CheckedTypes::new(),
            )
            .unwrap();
        // end-if-change
        schema
    }

    fn declare_identifier_with_constraint(
        &mut self,
        identifier_name: String,
        constraint: Constraint,
        checked_types: &mut CheckedTypes,
    ) -> Result<(), String> {
        let type_id = self.types.make_id();
        self.scope
            .declare_identifier(identifier_name, type_id)
            .map_err(generate_backtrace_error)?;
        self.add_constraint(type_id, constraint, checked_types)
            .map_err(generate_backtrace_error)?;
        Ok(())
    }

    pub fn make_id(&mut self) -> TypeId {
        self.types.make_id()
    }
    /// Insert a new constraint for a given type.
    pub fn add_constraint(
        &mut self,
        type_id: TypeId,
        constraint: Constraint,
        checked_types: &mut CheckedTypes,
    ) -> Result<(), String> {
        let canonical_id = self.get_canonical_id(type_id);
        // Get the existing parsed constraint with an immutable reference so we can still
        // use the type schema.
        let new_constraint = ParsedConstraint::new(canonical_id, constraint, self)?;
        if let Some(parsed_constraint) = self.constraints.get(&canonical_id) {
            if parsed_constraint.is_compatible_with(&new_constraint, self, checked_types) {
                // Getting the parsed constraint again so we can mutate it.
                if let Some(parsed_constraint) = self.constraints.get_mut(&canonical_id) {
                    parsed_constraint.add_constraints(new_constraint, &self.types);
                }
            } else {
                return Err(generate_backtrace_error(format!(
                    "ConstraintsNotCompatible\nbase constraint: {parsed_constraint:?}\nnew constraint: {new_constraint:?}\n"
                )));
            }
        } else {
            self.constraints.insert(canonical_id, new_constraint);
        };
        Ok(())
    }
    #[must_use]
    pub fn get_concrete_type_from_id(&self, type_id: TypeId) -> ConcreteType {
        let canonical_id = self.get_canonical_id(type_id);
        self.constraints.get(&canonical_id).map_or_else(
            || ConcreteType::Primitive(PrimitiveType::CompilerBoolean),
            |parsed_constraint| parsed_constraint.to_concrete_type(self),
        )
    }
    pub fn set_equal_to_function_result(
        &mut self,
        expression_type_id: TypeId,
        function_type_id: TypeId,
        checked_types: &mut CheckedTypes,
    ) -> Result<(), String> {
        let function_type_canonical_id = self.get_canonical_id(function_type_id);
        #[allow(clippy::option_if_let_else)] // Making this change violates the borrow checker.
        match self.constraints.get(&function_type_canonical_id) {
            Some(parsed_constraint) => parsed_constraint.get_function_return_type().map_or_else(
                || Err(generate_backtrace_error("NotAFunction".to_owned())),
                |return_type_id| {
                    self.set_equal_to_canonical_type(
                        return_type_id,
                        expression_type_id,
                        checked_types,
                    )
                },
            ),
            _ => Err(generate_backtrace_error("NotAFunction".to_owned())),
        }
    }
    #[must_use]
    pub fn get_function_argument_types(&self, function_type_id: TypeId) -> Option<Vec<TypeId>> {
        let function_type_canonical_id = self.get_canonical_id(function_type_id);
        self.constraints
            .get(&function_type_canonical_id)
            .and_then(ParsedConstraint::get_function_argument_types)
    }
    #[must_use]
    pub fn get_canonical_id(&self, type_id: TypeId) -> TypeId {
        self.types.get_canonical_id(type_id)
    }
    #[must_use]
    pub fn count_ids(&self) -> usize {
        self.types.count_ids()
    }
    pub fn get_total_canonical_ids(&mut self) -> usize {
        self.types.get_total_canonical_ids()
    }
    pub fn set_equal_to_canonical_type(
        &mut self,
        canonical_type_id: TypeId,
        other_type_id: TypeId,
        checked_types: &mut CheckedTypes,
    ) -> Result<(), String> {
        if !self.types_are_compatible(canonical_type_id, other_type_id, checked_types) {
            return Err(generate_backtrace_error("TypesAreNotCompatible".to_owned()));
        }
        match self.constraints.remove(&other_type_id) {
            None => {}
            Some(merged_constraint) => match self.constraints.get_mut(&canonical_type_id) {
                None => {
                    self.constraints
                        .insert(canonical_type_id, merged_constraint);
                }
                Some(existing_constraint) => {
                    existing_constraint.add_constraints(merged_constraint, &self.types);
                }
            },
        };
        self.types.set_types_equal(canonical_type_id, other_type_id);
        Ok(())
    }
    #[must_use]
    pub fn types_are_compatible(
        &self,
        base_type: TypeId,
        other_type: TypeId,
        checked_types: &mut CheckedTypes,
    ) -> bool {
        let base_canonical_id = self.get_canonical_id(base_type);
        let other_canonical_id = self.get_canonical_id(other_type);
        if base_canonical_id == other_canonical_id
            || checked_types.contains(base_canonical_id, other_canonical_id)
        {
            return true;
        }
        checked_types.add(base_canonical_id, other_canonical_id);
        match (
            self.constraints.get(&base_canonical_id),
            self.constraints.get(&other_canonical_id),
        ) {
            (Some(base_constraint), Some(other_constraint)) => {
                base_constraint.is_compatible_with(other_constraint, self, checked_types)
            }
            _ => true,
        }
    }

    pub fn declare_method_on_type(
        &mut self,
        base_type: TypeId,
        method_name: &str,
        method_type_id: TypeId,
        checked_types: &mut CheckedTypes,
    ) -> Result<(), String> {
        let canonical_type_id = self.get_canonical_id(base_type);

        if let Some(parsed_constraint) = self.constraints.get(&canonical_type_id) {
            if let Some(base_method_type) = parsed_constraint.get_same_method_type(
                self,
                method_name,
                method_type_id,
                checked_types,
            )? {
                self.set_equal_to_canonical_type(base_method_type, method_type_id, checked_types)?;
                return Ok(());
            }
        }
        self.add_constraint(
            base_type,
            Constraint::HasMethod(HasMethodConstraint {
                method_name: method_name.to_string(),
                method_type: method_type_id,
            }),
            checked_types,
        )?;
        Ok(())
    }

    pub fn set_equal_to_tag_contents(
        &mut self,
        tag_type_id: TypeId,
        tag_name: &String,
        tag_content_types: &Vec<TypeId>,
    ) -> Result<(), String> {
        let tag_type_canonical_id = self.get_canonical_id(tag_type_id);
        match self.constraints.get(&tag_type_canonical_id) {
            Some(parsed_constraint) => {
                let existing_contents = parsed_constraint.get_tag_content_types(tag_name)?;
                if existing_contents.len() != tag_content_types.len() {
                    return Err(generate_backtrace_error(format!(
                        "TagContentsHaveDifferentLengths: {tag_name}"
                    )));
                }
                for (existing_content, new_content) in
                    existing_contents.iter().zip(tag_content_types.iter())
                {
                    self.set_equal_to_canonical_type(
                        *existing_content,
                        *new_content,
                        &mut CheckedTypes::new(),
                    )?;
                }
                Ok(())
            }
            _ => Err(generate_backtrace_error(format!(
                "NoConstraintForType: {tag_type_id}"
            ))),
        }
    }

    // TODO(aaron) B-279
    // #[cfg(test)]
    pub fn make_identifier_for_test<S: Into<String>>(
        &mut self,
        identifier_name: S,
    ) -> Result<TypeId, String> {
        let id = self.make_id();
        self.scope.declare_identifier(identifier_name.into(), id)?;
        Ok(id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_id_increments_by_one() {
        let mut type_schema = TypeSchema::new();
        let first_id = type_schema.make_id();
        assert_eq!(type_schema.make_id(), first_id + 1);
        assert_eq!(type_schema.make_id(), first_id + 2);
        assert_eq!(type_schema.make_id(), first_id + 3);
    }

    #[test]
    fn each_id_is_its_own_canonical_id_by_default() {
        let mut type_schema = TypeSchema::new();
        let id = type_schema.make_id();
        assert_eq!(type_schema.get_canonical_id(id), id);
    }

    #[test]
    fn set_types_equal_sets_the_canonical_id_of_the_first_type_to_the_canonical_id_of_the_second() {
        let mut type_schema = TypeSchema::new();
        let id_a = type_schema.make_id();
        let id_b = type_schema.make_id();
        type_schema
            .set_equal_to_canonical_type(id_a, id_b, &mut CheckedTypes::new())
            .unwrap();
        assert_eq!(type_schema.get_canonical_id(id_a), id_a);
        assert_eq!(type_schema.get_canonical_id(id_b), id_a);
    }

    #[test]
    fn set_equal_types_sets_the_canonical_ids_even_if_theres_a_chain_of_ids() {
        let mut type_schema = TypeSchema::new();
        let id_a = type_schema.make_id();
        let id_b = type_schema.make_id();
        let id_c = type_schema.make_id();
        type_schema
            .set_equal_to_canonical_type(id_a, id_b, &mut CheckedTypes::new())
            .unwrap();
        type_schema
            .set_equal_to_canonical_type(id_b, id_c, &mut CheckedTypes::new())
            .unwrap();
        assert_eq!(type_schema.get_canonical_id(id_c), id_a);
    }

    #[test]
    fn count_ids_counts_the_total_number_of_ids() {
        let mut type_schema = TypeSchema::new();
        let initial_id = type_schema.make_id();
        type_schema.make_id();
        type_schema.make_id();
        assert_eq!(type_schema.count_ids(), initial_id + 3);
    }

    #[test]
    fn count_ids_ignores_canonical_ids() {
        let mut type_schema = TypeSchema::new();
        let id_a = type_schema.make_id();
        let id_b = type_schema.make_id();
        let id_c = type_schema.make_id();
        type_schema
            .set_equal_to_canonical_type(id_a, id_b, &mut CheckedTypes::new())
            .unwrap();
        type_schema
            .set_equal_to_canonical_type(id_b, id_c, &mut CheckedTypes::new())
            .unwrap();
        assert_eq!(type_schema.count_ids(), id_a + 3);
    }

    #[test]
    fn count_canonical_ids_counts_the_total_number_of_canonical_ids() {
        let mut type_schema = TypeSchema::new();
        let initial_id = type_schema.make_id();
        type_schema.make_id();
        type_schema.make_id();
        assert_eq!(type_schema.get_total_canonical_ids(), initial_id + 3);
    }

    #[test]
    fn set_types_equal_decreases_number_of_canonical_ids() {
        let mut type_schema = TypeSchema::new();
        let id_a = type_schema.make_id();
        let id_b = type_schema.make_id();
        let id_c = type_schema.make_id();
        type_schema
            .set_equal_to_canonical_type(id_a, id_b, &mut CheckedTypes::new())
            .unwrap();
        type_schema
            .set_equal_to_canonical_type(id_b, id_c, &mut CheckedTypes::new())
            .unwrap();
        assert_eq!(type_schema.get_total_canonical_ids(), id_a + 1);
    }

    #[test]
    fn int_is_a_default_type_that_resolves_to_primitive_int() {
        let type_schema = TypeSchema::new();
        let type_id = type_schema
            .scope
            .get_variable_declaration_type("Int")
            .unwrap();
        let concrete_type = type_schema.get_concrete_type_from_id(type_id);
        assert_eq!(concrete_type, ConcreteType::Primitive(PrimitiveType::Int));
    }

    #[test]
    fn str_is_a_default_type_that_resolves_to_primitive_str() {
        let type_schema = TypeSchema::new();
        let type_id = type_schema
            .scope
            .get_variable_declaration_type("Str")
            .unwrap();
        let concrete_type = type_schema.get_concrete_type_from_id(type_id);
        assert_eq!(concrete_type, ConcreteType::Primitive(PrimitiveType::Str));
    }
}
