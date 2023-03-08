fn build_parsed_constraint_from_constraints(
    constraints: &Vec<Constraint>,
    type_parameters: &Vec<TypeParameter>,
) -> ParsedConstraint {
    let mut parsed_constraint = ParsedConstraint::new();
    for constraint in constraints {
        let type_parameter = type_parameters
            .iter()
            .find(|type_parameter| type_parameter.name == constraint.type_parameter)
            .unwrap();
        let type_parameter_index = type_parameter.index;
        let type_parameter_constraint = constraint.constraint.clone();
        parsed_constraint.add_constraint(type_parameter_index, type_parameter_constraint);
    }
    parsed_constraint
}
