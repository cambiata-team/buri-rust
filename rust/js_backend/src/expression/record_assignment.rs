use crate::expression::record::print_record;
use crate::identifier::print_identifier;
use typed_ast::ConcreteRecordAssignmentExpression;

pub fn print_record_assignment(assignment: &ConcreteRecordAssignmentExpression) -> String {
    let identifier = print_identifier(&assignment.identifier);
    let new_values = print_record(&assignment.contents);
    format!("{identifier}.$set({new_values})")
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use typed_ast::{ConcreteExpression, ConcreteRecordExpression, ConcreteType};

    #[test]
    fn assigns_a_single_value() {
        let record = ConcreteRecordExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: HashMap::from([(
                "meaningOfLife".to_string(),
                ConcreteExpression::integer_for_test(42),
            )]),
        };
        let identifier = ConcreteExpression::raw_identifier_for_test("hello");
        let assignment = ConcreteRecordAssignmentExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: record,
            identifier,
        };
        let result = print_record_assignment(&assignment);
        assert_eq!(result, "Bhello.$set({meaningOfLife: 42})");
    }

    #[test]
    fn assigns_multiple_values() {
        let record = ConcreteRecordExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: HashMap::from([
                (
                    "meaningOfLife".to_string(),
                    ConcreteExpression::integer_for_test(42),
                ),
                ("foo".to_string(), ConcreteExpression::integer_for_test(0)),
            ]),
        };
        let identifier = ConcreteExpression::raw_identifier_for_test("hello");
        let assignment = ConcreteRecordAssignmentExpression {
            expression_type: ConcreteType::default_record_for_test(),
            contents: record,
            identifier,
        };
        let result = print_record_assignment(&assignment);
        // Because of the HashMap, the order of the keys is not guaranteed.
        // However, the order doesn't matter so we can accept either one.
        assert!(
            result == "Bhello.$set({meaningOfLife: 42, foo: 0})"
                || result == "Bhello.$set({foo: 0, meaningOfLife: 42})"
        );
    }
}
