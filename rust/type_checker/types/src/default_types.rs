use crate::{
    constraints::{Constraint, HasFunctionShape, HasMethodConstraint, TagAtMostConstraint},
    type_checking_call_stack::CheckedTypes,
    type_schema::{TypeSchema, INT_TYPE_ID},
    TypeId,
};
use std::collections::HashMap;

pub fn create_list_default_methods(
    schema: &mut TypeSchema,
    list_type_id: TypeId,
    list_contents_type: TypeId,
) -> Result<(TypeId, Vec<HasMethodConstraint>), String> {
    create_parsed_constraint_from_methods(
        vec![
            Method {
                name: "get",
                arguments: vec![INT_TYPE_ID],
                return_type: create_option_type(list_contents_type, schema)?,
            },
            Method {
                name: "append",
                arguments: vec![list_contents_type],
                return_type: list_type_id,
            },
            Method {
                name: "size",
                arguments: vec![],
                return_type: INT_TYPE_ID,
            },
        ],
        schema,
    )
}

pub fn create_string_default_methods(
    schema: &mut TypeSchema,
) -> Result<(TypeId, Vec<HasMethodConstraint>), String> {
    create_parsed_constraint_from_methods(
        vec![Method {
            name: "size",
            arguments: vec![],
            return_type: INT_TYPE_ID,
        }],
        schema,
    )
}

struct Method {
    name: &'static str,
    arguments: Vec<TypeId>,
    return_type: TypeId,
}

fn create_parsed_constraint_from_methods(
    methods: Vec<Method>,
    schema: &mut TypeSchema,
) -> Result<(TypeId, Vec<HasMethodConstraint>), String> {
    let type_id = schema.make_id();
    let mut method_constraints = Vec::new();
    method_constraints.reserve(methods.len());
    for method in methods {
        let function_shape_id = schema.make_id();
        schema.add_constraint(
            function_shape_id,
            Constraint::HasFunctionShape(HasFunctionShape {
                argument_types: method.arguments,
                return_type: method.return_type,
            }),
            &mut CheckedTypes::new(),
        )?;
        let method_constraint = HasMethodConstraint {
            method_name: method.name.into(),
            method_type: function_shape_id,
        };
        method_constraints.push(method_constraint.clone());
        schema.add_constraint(
            type_id,
            Constraint::HasMethod(method_constraint),
            &mut CheckedTypes::new(),
        )?;
    }
    Ok((type_id, method_constraints))
}

fn create_option_type(content_type_id: TypeId, schema: &mut TypeSchema) -> Result<TypeId, String> {
    let type_id = schema.make_id();
    schema.add_constraint(
        type_id,
        Constraint::TagAtMost(TagAtMostConstraint {
            tags: HashMap::from([
                ("some".to_string(), vec![content_type_id]),
                ("none".to_string(), vec![]),
            ]),
        }),
        &mut CheckedTypes::new(),
    )?;
    Ok(type_id)
}
