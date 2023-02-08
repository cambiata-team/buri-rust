use crate::GenericTypeId;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scope {
    pub parent_scope: Option<Box<Scope>>,
    pub identifiers: HashMap<String, GenericTypeId>,
}

impl Scope {
    pub fn new_head() -> Box<Self> {
        Box::new(Self {
            parent_scope: None,
            identifiers: HashMap::new(),
        })
    }
    pub fn new_sub_scope(input: Box<Self>) -> Box<Self> {
        Box::new(Self {
            parent_scope: Some(input),
            identifiers: HashMap::new(),
        })
    }
    pub fn get_variable_declaration_type(&self, identifier_name: &str) -> Option<GenericTypeId> {
        self.identifiers.get(identifier_name).map_or_else(
            || {
                self.parent_scope
                    .as_ref()
                    .and_then(|parent| parent.get_variable_declaration_type(identifier_name))
            },
            |x| Some(*x),
        )
    }
    pub fn add_variable_declaration(
        &mut self,
        identifier_name: String,
        identifier_type: GenericTypeId,
    ) {
        self.identifiers.insert(identifier_name, identifier_type);
    }
}
