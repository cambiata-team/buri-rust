use crate::TypeId;
use std::collections::HashMap;

/// An item in the scope stack.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ScopeItem {
    /// A delimiter that separates different scopes.
    Delimiter,
    /// An type or variable identifier that is declared in the current scope.
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scope {
    stack: Vec<ScopeItem>,
    pub identifiers: HashMap<String, TypeId>,
}

impl Scope {
    #[must_use]
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            identifiers: HashMap::new(),
        }
    }
    pub fn start_sub_scope(&mut self) {
        self.stack.push(ScopeItem::Delimiter);
    }
    pub fn end_sub_scope(&mut self) {
        while let Some(ScopeItem::Identifier(identifier)) = self.stack.pop() {
            self.identifiers.remove(&identifier);
        }
    }
    #[must_use]
    pub fn get_variable_declaration_type(&self, identifier_name: &str) -> Option<TypeId> {
        let answer = self.identifiers.get(identifier_name).copied();
        answer
    }
    pub fn declare_identifier(&mut self, identifier_name: String, identifier_type: TypeId) {
        self.identifiers
            .insert(identifier_name.clone(), identifier_type);
        self.stack.push(ScopeItem::Identifier(identifier_name));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scope_is_initially_empty() {
        let scope = Scope::new();
        assert_eq!(scope.stack.len(), 0);
        assert_eq!(scope.identifiers.len(), 0);
    }

    #[test]
    fn starting_a_sub_scope_adds_delimiter() {
        let mut scope = Scope::new();
        scope.start_sub_scope();
        assert_eq!(scope.stack, vec![ScopeItem::Delimiter]);
    }

    #[test]
    fn starting_a_sub_scope_does_not_affect_identifiers() {
        let mut scope = Scope::new();
        scope.start_sub_scope();
        assert_eq!(scope.identifiers.len(), 0);
    }

    #[test]
    fn adding_identifiers_to_scope_updates_the_stack() {
        let mut scope = Scope::new();
        scope.declare_identifier("foo".to_string(), 0);
        scope.declare_identifier("bar".to_string(), 1);
        scope.declare_identifier("baz".to_string(), 2);
        assert_eq!(
            scope.stack,
            vec![
                ScopeItem::Identifier("foo".to_string()),
                ScopeItem::Identifier("bar".to_string()),
                ScopeItem::Identifier("baz".to_string())
            ]
        );
    }

    #[test]
    fn adding_identifiers_to_scope_updates_the_identifiers() {
        let mut scope = Scope::new();
        scope.declare_identifier("foo".to_string(), 0);
        scope.declare_identifier("bar".to_string(), 1);
        scope.declare_identifier("baz".to_string(), 2);
        assert_eq!(
            scope.identifiers,
            vec![
                ("foo".to_string(), 0),
                ("bar".to_string(), 1),
                ("baz".to_string(), 2)
            ]
            .into_iter()
            .collect::<HashMap<_, _>>()
        );
    }

    #[test]
    fn ending_sub_scope_pops_all_identifiers_from_stack() {
        let mut scope = Scope::new();
        scope.declare_identifier("foo".to_string(), 0);
        scope.declare_identifier("bar".to_string(), 1);
        scope.declare_identifier("baz".to_string(), 2);
        scope.end_sub_scope();
        assert_eq!(scope.stack, Vec::new());
    }

    #[test]
    fn ending_sub_scope_only_pops_identifiers_from_current_scope_in_the_stack() {
        let mut scope = Scope::new();
        scope.declare_identifier("foo".to_string(), 0);
        scope.declare_identifier("bar".to_string(), 1);
        scope.declare_identifier("baz".to_string(), 2);
        scope.start_sub_scope();
        scope.declare_identifier("qux".to_string(), 3);
        scope.declare_identifier("quux".to_string(), 4);
        scope.declare_identifier("quuz".to_string(), 5);
        scope.end_sub_scope();
        assert_eq!(
            scope.stack,
            vec![
                ScopeItem::Identifier("foo".to_string()),
                ScopeItem::Identifier("bar".to_string()),
                ScopeItem::Identifier("baz".to_string())
            ]
        );
    }

    #[test]
    fn ending_sub_scope_removes_identifiers_from_hash_map() {
        let mut scope = Scope::new();
        scope.declare_identifier("foo".to_string(), 0);
        scope.declare_identifier("bar".to_string(), 1);
        scope.declare_identifier("baz".to_string(), 2);
        scope.end_sub_scope();
        assert_eq!(scope.identifiers, HashMap::new());
    }

    #[test]
    fn ending_sub_scope_removes_identifiers_from_current_scope_in_hash_map() {
        let mut scope = Scope::new();
        scope.declare_identifier("foo".to_string(), 0);
        scope.declare_identifier("bar".to_string(), 1);
        scope.declare_identifier("baz".to_string(), 2);
        scope.start_sub_scope();
        scope.declare_identifier("qux".to_string(), 3);
        scope.declare_identifier("quux".to_string(), 4);
        scope.declare_identifier("quuz".to_string(), 5);
        scope.end_sub_scope();
        assert_eq!(
            scope.identifiers,
            vec![
                ("foo".to_string(), 0),
                ("bar".to_string(), 1),
                ("baz".to_string(), 2)
            ]
            .into_iter()
            .collect::<HashMap<_, _>>()
        );
    }
}
