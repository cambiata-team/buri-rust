use crate::{constraints::Constraint, scope::Scope, GenericTypeId};
use std::{collections::HashMap, mem::swap};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeSchema {
    pub next_id: GenericTypeId,
    pub constraints: HashMap<GenericTypeId, Vec<Constraint>>,
    pub imports: HashMap<GenericTypeId, String>,
    pub scope: Option<Box<Scope>>,
}

impl TypeSchema {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            constraints: HashMap::new(),
            imports: HashMap::new(),
            scope: Some(Scope::new_head()),
        }
    }
    /// Return an id which is unique in this `TypeSchema`.
    pub fn make_id(&mut self) -> GenericTypeId {
        let return_value = self.next_id;
        self.next_id += 1;
        return_value
    }
    /// Insert a new constraint for a given type.
    pub fn insert(&mut self, typ: GenericTypeId, constraint: Constraint) {
        match self.constraints.get_mut(&typ) {
            Some(constraint_vec) => {
                constraint_vec.push(constraint);
            }
            None => {
                self.constraints.insert(typ, vec![constraint]);
            }
        }
    }
    /// Return the total number of constraints in the system.
    pub fn number_of_constraints(&self) -> usize {
        let mut constraint_count: usize = 0;
        for constraint_vec in self.constraints.values() {
            constraint_count += constraint_vec.len();
        }
        constraint_count
    }
    /// Return the generic type id for an imported identifier, creating the id if necessary.
    pub fn register_import(&mut self, identifier_name: String) -> GenericTypeId {
        let new_id = self.make_id();
        self.imports.insert(new_id, identifier_name);
        new_id
    }
    /// Replaces the scope with a new value, returning the old value.
    fn update_scope(&mut self, mut value: Option<Box<Scope>>) -> Option<Box<Scope>> {
        swap(&mut self.scope, &mut value);
        value
    }
    /// Create a new sub scope within the current scope and navigate to it.
    pub fn move_to_sub_scope(&mut self) {
        let current_scope = self.update_scope(None).map_or_else(Scope::new_head, |x| x);
        self.update_scope(Some(Scope::new_sub_scope(current_scope)));
    }
    /// Delete the current scope and navigate to the current scope's parent scope.
    pub fn move_to_parent_scope(&mut self) {
        let current_scope = self.update_scope(None).map_or_else(Scope::new_head, |x| x);
        self.update_scope(current_scope.parent_scope);
    }
}
