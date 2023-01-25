use crate::{constraints::Constraint, GenericTypeId};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSchema {
    pub next_id: GenericTypeId,
    pub constraints: HashMap<GenericTypeId, Vec<Constraint>>,
    pub imports: HashMap<String, GenericTypeId>,
}

impl TypeSchema {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            constraints: HashMap::new(),
            imports: HashMap::new(),
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
}
