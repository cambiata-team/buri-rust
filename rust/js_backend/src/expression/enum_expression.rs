use crate::expression::print_expression;
use typed_ast::{ConcreteEnumExpression, ConcreteType};

pub fn print_enum(enum_expression: &ConcreteEnumExpression) -> String {
    let variants = match &enum_expression.expression_type {
        ConcreteType::Enum(concrete_enum_type) => &concrete_enum_type.variants,
        _ => unreachable!("Expected enum type"),
    };
    let mut enum_has_payload = false;
    let mut variant_names = Vec::new();
    for (variant_name, payload_types) in variants {
        if !payload_types.is_empty() {
            enum_has_payload = true;
        }
        variant_names.push(variant_name.clone());
    }
    variant_names.sort();
    let mut variant_index: usize = 0;
    for (index, variant_name) in variant_names.iter().enumerate() {
        if variant_name == &enum_expression.name {
            variant_index = index;
            break;
        }
    }
    let mut output = String::new();
    if enum_has_payload {
        output.push('[');
        output.push_str(variant_index.to_string().as_str());
        for payload_element in &enum_expression.payload {
            output.push(',');
            output.push_str(&print_expression(payload_element));
        }
        output.push(']');
    } else {
        output.push_str(variant_index.to_string().as_str());
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use typed_ast::{ConcreteEnumType, ConcreteExpression, PrimitiveType};

    #[test]
    fn enums_without_payload_are_integers() {
        let enum_expression = ConcreteEnumExpression {
            name: "foo".to_string(),
            expression_type: ConcreteType::default_enum_for_test(),
            payload: vec![],
        };
        assert_eq!(print_enum(&enum_expression), "0");
    }

    #[test]
    fn enums_with_payload_are_arrays() {
        let mut foo_enum_variants = HashMap::new();
        foo_enum_variants.insert(
            "foo".to_string(),
            vec![ConcreteType::Primitive(PrimitiveType::Int)],
        );
        let enum_expression = ConcreteEnumExpression {
            name: "foo".to_string(),
            expression_type: ConcreteType::Enum(Box::new(ConcreteEnumType {
                variants: foo_enum_variants,
            })),
            payload: vec![ConcreteExpression::integer_for_test(42)],
        };
        assert_eq!(print_enum(&enum_expression), "[0,42]");
    }

    #[test]
    fn alphabetically_first_of_two_variants_has_value_zero() {
        let mut ab_enum_variants = HashMap::new();
        ab_enum_variants.insert("a".to_string(), vec![]);
        ab_enum_variants.insert("b".to_string(), vec![]);
        let enum_expression = ConcreteEnumExpression {
            name: "a".to_string(),
            expression_type: ConcreteType::Enum(Box::new(ConcreteEnumType {
                variants: ab_enum_variants,
            })),
            payload: vec![],
        };
        assert_eq!(print_enum(&enum_expression), "0");
    }

    #[test]
    fn alphabetically_second_of_two_variants_has_value_one() {
        let mut ab_enum_variants = HashMap::new();
        ab_enum_variants.insert("a".to_string(), vec![]);
        ab_enum_variants.insert("b".to_string(), vec![]);
        let enum_expression = ConcreteEnumExpression {
            name: "b".to_string(),
            expression_type: ConcreteType::Enum(Box::new(ConcreteEnumType {
                variants: ab_enum_variants,
            })),
            payload: vec![],
        };
        assert_eq!(print_enum(&enum_expression), "1");
    }
}
