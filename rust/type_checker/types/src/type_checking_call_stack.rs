use crate::TypeId;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct CheckedTypes(HashSet<(TypeId, TypeId)>);

const fn generate_id_tuple(base_id: TypeId, other_id: TypeId) -> (TypeId, TypeId) {
    if base_id < other_id {
        (base_id, other_id)
    } else {
        (other_id, base_id)
    }
}

impl CheckedTypes {
    #[must_use]
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn add(&mut self, base_id: TypeId, other_id: TypeId) -> bool {
        self.0.insert(generate_id_tuple(base_id, other_id))
    }

    pub fn remove(&mut self, base_id: TypeId, other_id: TypeId) -> bool {
        self.0.remove(&generate_id_tuple(base_id, other_id))
    }

    #[must_use]
    pub fn contains(&self, base_id: TypeId, other_id: TypeId) -> bool {
        self.0.contains(&generate_id_tuple(base_id, other_id))
    }
}
