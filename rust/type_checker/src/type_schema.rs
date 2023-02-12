use crate::{constraints::Constraint, scope::Scope, GenericTypeId};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeSchema {
    types: Vec<GenericTypeId>,
    constraints: HashMap<GenericTypeId, Vec<Constraint>>,
    scope: Scope,
}

impl TypeSchema {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            scope: Scope::new(),
            constraints: HashMap::new(),
        }
    }
    /// Return an id which is unique in this `TypeSchema`.
    pub fn make_id(&mut self) -> GenericTypeId {
        let id = self.types.len();
        self.types.push(id);
        id
    }
    /// Insert a new constraint for a given type.
    pub fn add_constraint(&mut self, type_id: GenericTypeId, constraint: Constraint) {
        let canonical_id = self.get_canonical_id(type_id);
        if let Some(constraint_vec) = self.constraints.get_mut(&canonical_id) {
            constraint_vec.push(constraint);
        } else {
            self.constraints.insert(canonical_id, vec![constraint]);
        }
    }
    pub fn get_constraints(&mut self, type_id: GenericTypeId) -> Option<&Vec<Constraint>> {
        let canonical_id = self.get_canonical_id(type_id);
        self.constraints.get(&canonical_id)
    }
    pub fn get_canonical_id(&mut self, mut type_id: GenericTypeId) -> GenericTypeId {
        loop {
            let parent_id = self.types[type_id];
            if parent_id == type_id {
                return type_id;
            }
            self.types[type_id] = self.types[parent_id];
            type_id = parent_id;
        }
    }
    pub fn count_ids(&self) -> usize {
        self.types.len()
    }
    /// Return the total number of constraints in the system.
    pub fn get_total_constraints(&self) -> usize {
        let mut constraint_count: usize = 0;
        for constraint_vec in self.constraints.values() {
            constraint_count += constraint_vec.len();
        }
        constraint_count
    }

    pub fn get_total_canonical_ids(&mut self) -> usize {
        self.types
            .iter()
            .enumerate()
            .filter(|(index, canonical_id)| index == *canonical_id)
            .count()
    }
    pub fn set_types_equal(&mut self, type_a: GenericTypeId, type_b: GenericTypeId) {
        let canonical_a = self.get_canonical_id(type_a);
        let canonical_b = self.get_canonical_id(type_b);
        self.types[canonical_a] = canonical_b;
        let b_constraints = self
            .constraints
            .get(&canonical_b)
            .map_or(Vec::new(), std::clone::Clone::clone);
        let a_constraints = self.constraints.get_mut(&canonical_a);
        match a_constraints {
            Some(a_constraints) => a_constraints.extend(b_constraints),
            None => {
                self.constraints.insert(canonical_a, b_constraints);
            }
        }
    }
    pub fn start_sub_scope(&mut self) {
        self.scope.start_sub_scope();
    }
    pub fn end_sub_scope(&mut self) {
        self.scope.end_sub_scope();
    }
    pub fn get_variable_declaration_type(&self, identifier_name: &str) -> Option<GenericTypeId> {
        self.scope.get_variable_declaration_type(identifier_name)
    }
    pub fn declare_identifier(&mut self, identifier_name: String, identifier_type: GenericTypeId) {
        self.scope
            .declare_identifier(identifier_name, identifier_type);
    }

    #[cfg(test)]
    pub fn make_identifier_for_test<S: Into<String>>(
        &mut self,
        identifier_name: S,
    ) -> GenericTypeId {
        let id = self.make_id();
        self.declare_identifier(identifier_name.into(), id);
        id
    }
}
