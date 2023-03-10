use ast::{
    BinaryOperatorNode, BinaryOperatorSymbol, BlockNode, DeclarationNode, Expression, FunctionNode,
    FunctionTypeNode, IdentifierNode, IfNode, IntegerNode, ListNode, ListTypeNode,
    RecordAssignmentNode, RecordNode, RecordTypeNode, StringLiteralNode, TagGroupTypeNode, TagNode,
    TypeDeclarationNode, TypeExpression, TypeIdentifierNode, UnaryOperatorNode,
    UnaryOperatorSymbol,
};
use std::collections::HashMap;
use type_checker_errors::generate_backtrace_error;
use type_checker_types::{
    constraints::{
        Constraint, HasExactFieldsConstraint, HasFieldConstraint, HasFunctionShape,
        HasTagConstraint, TagAtMostConstraint,
    },
    generic_nodes::{
        get_generic_type_id, GenericBinaryOperatorExpression, GenericBlockExpression,
        GenericDeclarationExpression, GenericExpression, GenericFunctionExpression,
        GenericIdentifierExpression, GenericIfExpression, GenericIntegerLiteralExpression,
        GenericListExpression, GenericRecordAssignmentExpression, GenericRecordExpression,
        GenericSourcedType, GenericStringLiteralExpression, GenericTagExpression,
        GenericTypeDeclarationExpression, GenericTypeIdentifierExpression,
        GenericUnaryOperatorExpression,
    },
    type_checking_call_stack::CheckedTypes,
    type_schema::TypeSchema,
    TypeId,
};
use typed_ast::PrimitiveType;

const fn constrain_equal_to_num() -> Constraint {
    Constraint::EqualToPrimitive(PrimitiveType::Int)
}

const fn constrain_equal_to_str() -> Constraint {
    Constraint::EqualToPrimitive(PrimitiveType::Str)
}

fn constrain_at_least_true() -> Constraint {
    Constraint::HasTag(HasTagConstraint {
        tag_name: "true".to_owned(),
        tag_content_types: vec![],
    })
}

fn constrain_at_least_false() -> Constraint {
    Constraint::HasTag(HasTagConstraint {
        tag_name: "false".to_owned(),
        tag_content_types: vec![],
    })
}

fn constrain_at_most_boolean_tag() -> Constraint {
    Constraint::TagAtMost(TagAtMostConstraint {
        tags: HashMap::from([("true".to_owned(), vec![]), ("false".to_owned(), vec![])]),
    })
}

fn constrain_at_most_none_tag() -> Constraint {
    Constraint::TagAtMost(TagAtMostConstraint {
        tags: HashMap::from([("none".to_owned(), vec![])]),
    })
}

#[derive(Debug)]
struct TranslateBinaryOperatorIdCollection {
    pub type_id: TypeId,
    pub left_child_id: TypeId,
    pub right_child_id: TypeId,
}

fn translate_binary_operator_add_arithmetic_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) -> Result<(), String> {
    schema.add_constraint(
        id_collection.type_id,
        constrain_equal_to_num(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.left_child_id,
        constrain_equal_to_num(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.right_child_id,
        constrain_equal_to_num(),
        &mut CheckedTypes::new(),
    )?;
    Ok(())
}

fn translate_binary_operator_add_concatenate_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) -> Result<(), String> {
    schema.add_constraint(
        id_collection.type_id,
        constrain_equal_to_str(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.left_child_id,
        constrain_equal_to_str(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.right_child_id,
        constrain_equal_to_str(),
        &mut CheckedTypes::new(),
    )?;
    Ok(())
}

fn translate_binary_operator_add_logic_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) -> Result<(), String> {
    schema.add_constraint(
        id_collection.type_id,
        constrain_at_most_boolean_tag(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.left_child_id,
        constrain_at_most_boolean_tag(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.right_child_id,
        constrain_at_most_boolean_tag(),
        &mut CheckedTypes::new(),
    )?;
    Ok(())
}

fn translate_binary_operator_add_equality_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) -> Result<(), String> {
    schema.add_constraint(
        id_collection.type_id,
        constrain_at_most_boolean_tag(),
        &mut CheckedTypes::new(),
    )?;
    schema.set_equal_to_canonical_type(
        id_collection.left_child_id,
        id_collection.right_child_id,
        &mut CheckedTypes::new(),
    )?;
    Ok(())
}

fn translate_binary_operator_add_comparison_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
) -> Result<(), String> {
    schema.add_constraint(
        id_collection.type_id,
        constrain_at_least_true(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.type_id,
        constrain_at_least_false(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.left_child_id,
        constrain_equal_to_num(),
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.right_child_id,
        constrain_equal_to_num(),
        &mut CheckedTypes::new(),
    )?;
    Ok(())
}

fn translate_binary_operator_add_function_application_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
    right_child: &GenericExpression,
) -> Result<(), String> {
    let argument_types: Vec<TypeId> = match &right_child {
        GenericExpression::FunctionArguments(arguments) => {
            arguments.iter().map(get_generic_type_id).collect()
        }
        _ => {
            return Err(generate_backtrace_error(
                "FunctionApplicationDoesNotUseFunctionArguments".to_owned(),
            ))
        }
    };
    schema.add_constraint(
        id_collection.left_child_id,
        Constraint::HasFunctionShape(HasFunctionShape {
            argument_types,
            return_type: id_collection.type_id,
        }),
        &mut CheckedTypes::new(),
    )?;
    Ok(())
}

fn translate_binary_operator_add_method_lookup_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
    right_child: &GenericExpression,
) -> Result<(), String> {
    let method_name = match right_child {
        GenericExpression::Identifier(identifier_expression) => identifier_expression.name.clone(),
        _ => {
            return Err(generate_backtrace_error(
                "MethodLookupDoesNotUseIdentifier".to_owned(),
            ))
        }
    };
    schema.declare_method_on_type(
        id_collection.left_child_id,
        &method_name,
        id_collection.right_child_id,
        &mut CheckedTypes::new(),
    )?;
    schema.set_equal_to_canonical_type(
        id_collection.right_child_id,
        id_collection.type_id,
        &mut CheckedTypes::new(),
    )?;
    Ok(())
}

fn translate_binary_operator_add_field_lookup_constraints(
    schema: &mut TypeSchema,
    id_collection: &TranslateBinaryOperatorIdCollection,
    right_child: &GenericExpression,
) -> Result<(), String> {
    let field_name = match right_child {
        GenericExpression::Identifier(identifier_expression) => identifier_expression.name.clone(),
        _ => {
            return Err(generate_backtrace_error(
                "FieldLookupDoesNotUseIdentifier".to_owned(),
            ))
        }
    };
    schema.set_equal_to_canonical_type(
        id_collection.right_child_id,
        id_collection.type_id,
        &mut CheckedTypes::new(),
    )?;
    schema.add_constraint(
        id_collection.left_child_id,
        Constraint::HasField(HasFieldConstraint {
            field_name,
            field_type: id_collection.right_child_id,
        }),
        &mut CheckedTypes::new(),
    )?;
    Ok(())
}

fn translate_binary_operator<'a>(
    schema: &mut TypeSchema,
    node: BinaryOperatorNode<'a>,
) -> Result<GenericBinaryOperatorExpression<'a>, String> {
    let type_id = schema.make_id();
    let translated_left_child =
        translate_parsed_expression_to_generic_expression(schema, *node.value.left_child)?;
    let should_declare_unknown_identifier = node.value.symbol == BinaryOperatorSymbol::FieldLookup
        || node.value.symbol == BinaryOperatorSymbol::MethodLookup;
    let translated_right_child = match *node.value.right_child {
        Expression::FunctionApplicationArguments(arguments) => {
            let function_arguments: Result<Vec<GenericExpression>, String> = arguments
                .value
                .arguments
                .into_iter()
                .map(|expression| {
                    translate_parsed_expression_to_generic_expression(schema, expression)
                })
                .collect();
            GenericExpression::FunctionArguments(function_arguments?)
        }
        Expression::Identifier(identifier) if should_declare_unknown_identifier => {
            GenericExpression::Identifier(Box::new(GenericIdentifierExpression {
                expression_type: GenericSourcedType {
                    type_id: schema.make_id(),
                    source_of_type: identifier.source,
                },
                name: identifier.value.name,
                is_disregarded: identifier.value.is_disregarded,
            }))
        }
        _ => translate_parsed_expression_to_generic_expression(schema, *node.value.right_child)?,
    };
    let id_collection = TranslateBinaryOperatorIdCollection {
        type_id,
        left_child_id: get_generic_type_id(&translated_left_child),
        right_child_id: match &translated_right_child {
            GenericExpression::FunctionArguments(_) => 0,
            _ => get_generic_type_id(&translated_right_child),
        },
    };
    match node.value.symbol {
        BinaryOperatorSymbol::Add
        | BinaryOperatorSymbol::Subtract
        | BinaryOperatorSymbol::Multiply
        | BinaryOperatorSymbol::Divide
        | BinaryOperatorSymbol::Modulus
        | BinaryOperatorSymbol::Power => {
            translate_binary_operator_add_arithmetic_constraints(schema, &id_collection)?;
        }
        BinaryOperatorSymbol::Concatenate => {
            translate_binary_operator_add_concatenate_constraints(schema, &id_collection)?;
        }
        BinaryOperatorSymbol::And | BinaryOperatorSymbol::Or => {
            translate_binary_operator_add_logic_constraints(schema, &id_collection)?;
        }
        BinaryOperatorSymbol::EqualTo | BinaryOperatorSymbol::NotEqualTo => {
            translate_binary_operator_add_equality_constraints(schema, &id_collection)?;
        }
        BinaryOperatorSymbol::LessThan
        | BinaryOperatorSymbol::LessThanOrEqualTo
        | BinaryOperatorSymbol::GreaterThan
        | BinaryOperatorSymbol::GreaterThanOrEqualTo => {
            translate_binary_operator_add_comparison_constraints(schema, &id_collection)?;
        }
        BinaryOperatorSymbol::FunctionApplication => {
            translate_binary_operator_add_function_application_constraints(
                schema,
                &id_collection,
                &translated_right_child,
            )?;
            let function_type_id = get_generic_type_id(&translated_left_child);
            schema.set_equal_to_function_result(
                type_id,
                function_type_id,
                &mut CheckedTypes::new(),
            )?;
        }
        BinaryOperatorSymbol::MethodLookup => {
            translate_binary_operator_add_method_lookup_constraints(
                schema,
                &id_collection,
                &translated_right_child,
            )?;
        }
        BinaryOperatorSymbol::FieldLookup => {
            translate_binary_operator_add_field_lookup_constraints(
                schema,
                &id_collection,
                &translated_right_child,
            )?;
        }
    };
    Ok(GenericBinaryOperatorExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        symbol: node.value.symbol,
        left_child: translated_left_child,
        right_child: translated_right_child,
    })
}

fn translate_block<'a>(
    schema: &mut TypeSchema,
    node: BlockNode<'a>,
) -> Result<GenericBlockExpression<'a>, String> {
    let type_id = schema.make_id();
    schema.scope.start_sub_scope();
    let mut element_translations = Vec::new();
    element_translations.reserve_exact(node.value.len());
    for element in node.value {
        let element_translation =
            translate_parsed_expression_to_generic_expression(schema, element)?;
        element_translations.push(element_translation);
    }
    match element_translations.last_mut() {
        None => {
            return Err(generate_backtrace_error(
                "UnreachableBlockFinalExpression".to_owned(),
            ))
        }
        Some(last_element) => {
            schema.set_equal_to_canonical_type(
                get_generic_type_id(last_element),
                type_id,
                &mut CheckedTypes::new(),
            )?;
        }
    }
    schema.scope.end_sub_scope();
    Ok(GenericBlockExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        contents: element_translations,
    })
}

fn translate_declaration_expression<'a>(
    schema: &mut TypeSchema,
    name_type_id: TypeId,
    expression: Expression<'a>,
    maybe_type_expression: Option<TypeExpression<'a>>,
) -> Result<GenericExpression<'a>, String> {
    if let Expression::Function(function) = &expression {
        let maybe_function_type_id = match &maybe_type_expression {
            Some(TypeExpression::Function(function_type)) => {
                Some(translate_function_type(schema, function_type)?)
            }
            Some(TypeExpression::Identifier(identifier_node)) => Some(
                translate_type_identifier(schema, identifier_node.clone())?
                    .expression_type
                    .type_id,
            ),
            _ => None,
        };
        if let Some(function_type_id) = maybe_function_type_id {
            schema.set_equal_to_canonical_type(
                function_type_id,
                name_type_id,
                &mut CheckedTypes::new(),
            )?;
            let translated_function =
                translate_function(schema, function.clone(), Some(function_type_id))?;
            return Ok(GenericExpression::Function(Box::new(translated_function)));
        }
    }

    let expression = translate_parsed_expression_to_generic_expression(schema, expression)?;
    let expression_id = get_generic_type_id(&expression);

    if let Some(type_expression) = maybe_type_expression {
        let type_expression_id = translate_parsed_type_expression(schema, &type_expression)?;
        schema.set_equal_to_canonical_type(
            type_expression_id,
            expression_id,
            &mut CheckedTypes::new(),
        )?;
    }
    schema.set_equal_to_canonical_type(expression_id, name_type_id, &mut CheckedTypes::new())?;
    Ok(expression)
}

pub fn translate_declaration<'a>(
    schema: &mut TypeSchema,
    node: DeclarationNode<'a>,
) -> Result<GenericDeclarationExpression<'a>, String> {
    let declaration_type_id = schema.make_id();
    let declaration_type = constrain_at_most_none_tag();
    schema.add_constraint(
        declaration_type_id,
        declaration_type,
        &mut CheckedTypes::new(),
    )?;

    let name_type_id = schema.make_id();
    schema
        .scope
        .declare_identifier(node.value.identifier.value.name.clone(), name_type_id)?;
    let identifier = translate_identifier(schema, node.value.identifier.clone())?;

    let expression = translate_declaration_expression(
        schema,
        name_type_id,
        *node.value.expression,
        node.value.type_expression,
    )?;

    Ok(GenericDeclarationExpression {
        declaration_type: GenericSourcedType {
            type_id: name_type_id,
            source_of_type: node.source.clone(),
        },
        expression_type: GenericSourcedType {
            type_id: declaration_type_id,
            source_of_type: node.source,
        },
        identifier,
        value: expression,
    })
}

fn translate_function<'a>(
    schema: &mut TypeSchema,
    node: FunctionNode<'a>,
    declaration_type: Option<TypeId>,
) -> Result<GenericFunctionExpression<'a>, String> {
    let function_type = schema.make_id();
    schema.scope.start_sub_scope();
    let mut argument_names = Vec::new();
    let mut argument_types = Vec::new();
    argument_names.reserve_exact(node.value.arguments.len());
    argument_types.reserve_exact(node.value.arguments.len());
    let declaration_argument_types = match declaration_type {
        Some(declaration_type) => {
            let Some(declaration_function_type) = schema.get_function_argument_types(declaration_type) else {
                return Err(generate_backtrace_error("Declaration type is not for a function".to_owned()))
            };
            declaration_function_type.into_iter().map(Some).collect()
        }
        None => std::vec::from_elem(None, node.value.arguments.len()),
    };
    if declaration_argument_types.len() != node.value.arguments.len() {
        return Err(generate_backtrace_error(
            "Function does not have the same number of arguments as the declaration".to_owned(),
        ));
    }
    for (argument, declaration_argument_type) in node
        .value
        .arguments
        .into_iter()
        .zip(declaration_argument_types)
    {
        let identifier_type = schema.make_id();
        schema.scope.declare_identifier(
            argument.value.argument_name.value.name.clone(),
            identifier_type,
        )?;
        if let Some(argument_type_expression) = argument.value.argument_type {
            let Some(argument_type_id) = schema.scope.get_variable_declaration_type(&argument_type_expression.value) else {
                return Err(generate_backtrace_error("IdentifierNotFound".to_owned()))
            };
            if let Some(declaration_argument_type) = declaration_argument_type {
                if !schema.types_are_compatible(
                    argument_type_id,
                    declaration_argument_type,
                    &mut CheckedTypes::new(),
                ) {
                    return Err(generate_backtrace_error(
                        "Argument type does not match declaration".to_owned(),
                    ));
                }
            };
            schema.set_equal_to_canonical_type(
                argument_type_id,
                identifier_type,
                &mut CheckedTypes::new(),
            )?;
        } else if let Some(declaration_argument_type) = declaration_argument_type {
            schema.set_equal_to_canonical_type(
                declaration_argument_type,
                identifier_type,
                &mut CheckedTypes::new(),
            )?;
        }
        argument_types.push(identifier_type);
        argument_names.push(argument.value.argument_name.value.name.clone());
    }
    let body = translate_parsed_expression_to_generic_expression(schema, *node.value.body)?;
    let body_id = get_generic_type_id(&body);
    let return_type = schema.make_id();
    schema.set_equal_to_canonical_type(body_id, return_type, &mut CheckedTypes::new())?;
    if let Some(declaration_type) = declaration_type {
        schema.set_equal_to_function_result(body_id, declaration_type, &mut CheckedTypes::new())?;
    }
    schema.add_constraint(
        function_type,
        Constraint::HasFunctionShape(HasFunctionShape {
            argument_types,
            return_type,
        }),
        &mut CheckedTypes::new(),
    )?;
    schema.scope.end_sub_scope();
    Ok(GenericFunctionExpression {
        expression_type: GenericSourcedType {
            type_id: function_type,
            source_of_type: node.source,
        },
        argument_names,
        body,
    })
}

fn translate_identifier<'a>(
    schema: &mut TypeSchema,
    node: IdentifierNode<'a>,
) -> Result<GenericIdentifierExpression<'a>, String> {
    let Some(type_id) = schema.scope.get_variable_declaration_type(&node.value.name) else {
        return Err(generate_backtrace_error(format!("IdentifierNotFound: {}", node.value.name)))
    };
    Ok(GenericIdentifierExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        name: node.value.name,
        is_disregarded: node.value.is_disregarded,
    })
}

fn translate_if<'a>(
    schema: &mut TypeSchema,
    node: IfNode<'a>,
) -> Result<GenericIfExpression<'a>, String> {
    let type_id = schema.make_id();
    let translated_condition =
        translate_parsed_expression_to_generic_expression(schema, *node.value.condition)?;
    schema.add_constraint(
        get_generic_type_id(&translated_condition),
        constrain_at_most_boolean_tag(),
        &mut CheckedTypes::new(),
    )?;
    schema.scope.start_sub_scope();
    let translated_true_path =
        translate_parsed_expression_to_generic_expression(schema, *node.value.path_if_true)?;
    schema.scope.end_sub_scope();
    schema.scope.start_sub_scope();
    let translated_false_path = if let Some(false_path) = node.value.path_if_false {
        schema.set_equal_to_canonical_type(
            get_generic_type_id(&translated_true_path),
            type_id,
            &mut CheckedTypes::new(),
        )?;
        let translated_false_path =
            translate_parsed_expression_to_generic_expression(schema, *false_path)?;
        schema.set_equal_to_canonical_type(
            get_generic_type_id(&translated_false_path),
            type_id,
            &mut CheckedTypes::new(),
        )?;
        Some(translated_false_path)
    } else {
        schema.add_constraint(
            type_id,
            Constraint::HasTag(HasTagConstraint {
                tag_name: "none".to_owned(),
                tag_content_types: vec![],
            }),
            &mut CheckedTypes::new(),
        )?;
        schema.add_constraint(
            type_id,
            Constraint::HasTag(HasTagConstraint {
                tag_name: "some".to_owned(),
                tag_content_types: vec![get_generic_type_id(&translated_true_path)],
            }),
            &mut CheckedTypes::new(),
        )?;
        None
    };
    schema.scope.end_sub_scope();
    Ok(GenericIfExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        condition: translated_condition,
        path_if_true: translated_true_path,
        path_if_false: translated_false_path,
    })
}

fn translate_integer<'a>(
    schema: &mut TypeSchema,
    node: IntegerNode<'a>,
) -> Result<GenericIntegerLiteralExpression<'a>, String> {
    let type_id = schema.make_id();
    schema.add_constraint(type_id, constrain_equal_to_num(), &mut CheckedTypes::new())?;
    Ok(GenericIntegerLiteralExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        value: node.value,
    })
}

fn translate_list<'a>(
    schema: &mut TypeSchema,
    node: ListNode<'a>,
) -> Result<GenericListExpression<'a>, String> {
    let list_type_id = schema.make_id();
    let element_type_id = schema.make_id();
    schema.add_constraint(
        list_type_id,
        Constraint::ListOfType(element_type_id),
        &mut CheckedTypes::new(),
    )?;
    let mut element_translations = Vec::new();
    element_translations.reserve_exact(node.value.len());
    for element in node.value {
        let element_translation =
            translate_parsed_expression_to_generic_expression(schema, element)?;
        schema.set_equal_to_canonical_type(
            get_generic_type_id(&element_translation),
            element_type_id,
            &mut CheckedTypes::new(),
        )?;
        element_translations.push(element_translation);
    }
    Ok(GenericListExpression {
        expression_type: GenericSourcedType {
            type_id: list_type_id,
            source_of_type: node.source,
        },
        contents: element_translations,
    })
}

fn translate_record<'a>(
    schema: &mut TypeSchema,
    node: RecordNode<'a>,
) -> Result<GenericRecordExpression<'a>, String> {
    let record_type_id = schema.make_id();
    let mut element_translations = HashMap::new();
    element_translations.reserve(node.value.len());
    let mut fields = HashMap::new();
    for element in node.value {
        let field_type_id = schema.make_id();
        let field_name = element.identifier.value.name;
        fields.insert(field_name.clone(), field_type_id);
        let element_translation =
            translate_parsed_expression_to_generic_expression(schema, element.value)?;
        schema.set_equal_to_canonical_type(
            get_generic_type_id(&element_translation),
            field_type_id,
            &mut CheckedTypes::new(),
        )?;
        element_translations.insert(field_name, element_translation);
    }
    schema.add_constraint(
        record_type_id,
        Constraint::HasExactFields(HasExactFieldsConstraint { fields }),
        &mut CheckedTypes::new(),
    )?;
    Ok(GenericRecordExpression {
        expression_type: GenericSourcedType {
            type_id: record_type_id,
            source_of_type: node.source,
        },
        contents: element_translations,
    })
}

fn translate_string<'a>(
    schema: &mut TypeSchema,
    node: StringLiteralNode<'a>,
) -> Result<GenericStringLiteralExpression<'a>, String> {
    let type_id = schema.make_id();
    schema.add_constraint(type_id, constrain_equal_to_str(), &mut CheckedTypes::new())?;
    Ok(GenericStringLiteralExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        value: node.value,
    })
}

fn translate_tag<'a>(
    schema: &mut TypeSchema,
    node: TagNode<'a>,
) -> Result<GenericTagExpression<'a>, String> {
    let type_id = schema.make_id();
    let translated_content_expressions: Vec<GenericExpression> = match node
        .value
        .contents
        .into_iter()
        .map(|expression| translate_parsed_expression_to_generic_expression(schema, expression))
        .collect()
    {
        Ok(x) => x,
        Err(x) => {
            return Err(x);
        }
    };
    let translated_content_types: Vec<TypeId> = translated_content_expressions
        .iter()
        .map(get_generic_type_id)
        .collect();
    schema.add_constraint(
        type_id,
        Constraint::HasTag(HasTagConstraint {
            tag_name: node.value.name.value.clone(),
            tag_content_types: translated_content_types,
        }),
        &mut CheckedTypes::new(),
    )?;
    Ok(GenericTagExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        name: node.value.name.value,
        contents: translated_content_expressions,
    })
}

fn translate_unary_operator<'a>(
    schema: &mut TypeSchema,
    node: UnaryOperatorNode<'a>,
) -> Result<GenericUnaryOperatorExpression<'a>, String> {
    let type_id = schema.make_id();
    let new_child = match node.value.symbol {
        UnaryOperatorSymbol::Not => {
            schema.add_constraint(type_id, constrain_at_least_true(), &mut CheckedTypes::new())?;
            schema.add_constraint(
                type_id,
                constrain_at_least_false(),
                &mut CheckedTypes::new(),
            )?;
            let translated_child =
                translate_parsed_expression_to_generic_expression(schema, *node.value.child)?;
            schema.add_constraint(
                get_generic_type_id(&translated_child),
                constrain_at_most_boolean_tag(),
                &mut CheckedTypes::new(),
            )?;
            translated_child
        }
        UnaryOperatorSymbol::Negative => {
            schema.add_constraint(type_id, constrain_equal_to_num(), &mut CheckedTypes::new())?;
            let translated_child =
                translate_parsed_expression_to_generic_expression(schema, *node.value.child)?;
            schema.add_constraint(
                get_generic_type_id(&translated_child),
                constrain_equal_to_num(),
                &mut CheckedTypes::new(),
            )?;
            translated_child
        }
    };
    Ok(GenericUnaryOperatorExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        symbol: node.value.symbol,
        child: new_child,
    })
}

fn translate_record_assignment<'a>(
    schema: &mut TypeSchema,
    node: RecordAssignmentNode<'a>,
) -> Result<GenericRecordAssignmentExpression<'a>, String> {
    let assignment_type_id = schema.make_id();
    let raw_translated_name = translate_identifier(schema, node.value.identifier)?;
    let translated_name = GenericExpression::Identifier(Box::new(raw_translated_name.clone()));
    let name_type_id = get_generic_type_id(&translated_name);
    let mut field_translations = HashMap::new();
    field_translations.reserve(node.value.new_values.len());

    for element in node.value.new_values {
        let field_type_id = schema.make_id();
        let field_name = element.identifier.value.name;
        let field_translation =
            translate_parsed_expression_to_generic_expression(schema, element.value)?;
        schema.set_equal_to_canonical_type(
            get_generic_type_id(&field_translation),
            field_type_id,
            &mut CheckedTypes::new(),
        )?;
        field_translations.insert(field_name.clone(), field_translation);
        schema.add_constraint(
            assignment_type_id,
            Constraint::HasField(HasFieldConstraint {
                field_name,
                field_type: field_type_id,
            }),
            &mut CheckedTypes::new(),
        )?;
    }
    schema.set_equal_to_canonical_type(
        name_type_id,
        assignment_type_id,
        &mut CheckedTypes::new(),
    )?;
    Ok(GenericRecordAssignmentExpression {
        expression_type: GenericSourcedType {
            type_id: assignment_type_id,
            source_of_type: node.source.clone(),
        },
        identifier: raw_translated_name,
        contents: GenericRecordExpression {
            expression_type: GenericSourcedType {
                type_id: assignment_type_id,
                source_of_type: node.source,
            },
            contents: field_translations,
        },
    })
}

pub fn translate_type_declaration<'a>(
    schema: &mut TypeSchema,
    node: TypeDeclarationNode<'a>,
) -> Result<GenericTypeDeclarationExpression<'a>, String> {
    let declaration_type_id = schema.make_id();
    schema.add_constraint(
        declaration_type_id,
        constrain_at_most_none_tag(),
        &mut CheckedTypes::new(),
    )?;

    let name_type_id = schema.make_id();
    schema
        .scope
        .declare_identifier(node.value.identifier.value.clone(), name_type_id)?;

    let identifier_name = translate_type_identifier(schema, node.value.identifier.clone())?;
    let type_expression_id = translate_parsed_type_expression(schema, &node.value.type_expression)?;
    schema.set_equal_to_canonical_type(
        type_expression_id,
        name_type_id,
        &mut CheckedTypes::new(),
    )?;
    Ok(GenericTypeDeclarationExpression {
        declaration_type: GenericSourcedType {
            type_id: declaration_type_id,
            source_of_type: node.source.clone(),
        },
        expression_type: GenericSourcedType {
            type_id: type_expression_id,
            source_of_type: node.source,
        },
        identifier_name,
    })
}

fn translate_type_identifier<'a>(
    schema: &mut TypeSchema,
    node: TypeIdentifierNode<'a>,
) -> Result<GenericTypeIdentifierExpression<'a>, String> {
    let Some(type_id) = schema.scope.get_variable_declaration_type(&node.value) else {
        return Err(generate_backtrace_error("TypeIdentifierNotFound".to_owned()))
    };
    Ok(GenericTypeIdentifierExpression {
        expression_type: GenericSourcedType {
            type_id,
            source_of_type: node.source,
        },
        name: node.value,
    })
}

fn translate_function_type(
    schema: &mut TypeSchema,
    node: &FunctionTypeNode,
) -> Result<TypeId, String> {
    let type_id = schema.make_id();
    let mut argument_types = Vec::new();
    for argument in &node.value.arguments {
        let argument_type_id = translate_parsed_type_expression(schema, argument)?;
        argument_types.push(argument_type_id);
    }
    let return_type_id = translate_parsed_type_expression(schema, &node.value.return_type)?;
    schema.add_constraint(
        type_id,
        Constraint::HasFunctionShape(HasFunctionShape {
            argument_types,
            return_type: return_type_id,
        }),
        &mut CheckedTypes::new(),
    )?;
    Ok(type_id)
}

fn translate_type_identifier_type(
    schema: &mut TypeSchema,
    node: &TypeIdentifierNode,
) -> Result<TypeId, String> {
    schema
        .scope
        .get_variable_declaration_type(&node.value)
        .ok_or_else(|| format!("Type identifier not found: {}", node.value))
}

fn translate_list_type(
    schema: &mut TypeSchema,
    expression: &ListTypeNode,
) -> Result<TypeId, String> {
    let type_id = schema.make_id();
    let contents_type_id = translate_parsed_type_expression(schema, &expression.value)?;
    schema.add_constraint(
        type_id,
        Constraint::ListOfType(contents_type_id),
        &mut CheckedTypes::new(),
    )?;
    Ok(type_id)
}

fn translate_record_type(
    schema: &mut TypeSchema,
    expression: &RecordTypeNode,
) -> Result<TypeId, String> {
    let type_id = schema.make_id();

    let mut fields = HashMap::new();

    for field in &expression.value {
        let field_type_id = translate_parsed_type_expression(schema, &field.value)?;
        fields.insert(field.identifier.value.name.clone(), field_type_id);
    }

    schema.add_constraint(
        type_id,
        Constraint::HasExactFields(HasExactFieldsConstraint { fields }),
        &mut CheckedTypes::new(),
    )?;

    Ok(type_id)
}

fn translate_tag_group_type(
    schema: &mut TypeSchema,
    expression: &TagGroupTypeNode,
) -> Result<TypeId, String> {
    let type_id = schema.make_id();

    let mut tags: HashMap<String, Vec<TypeId>> = HashMap::new();
    for tag in &expression.value {
        let tag_name = tag.value.name.value.clone();
        if tags.contains_key(&tag_name) {
            return Err(generate_backtrace_error(
                "DuplicateTagNamesInTagGroup".to_owned(),
            ));
        }
        let mut content_item_ids = vec![];
        for content_item in &tag.value.contents {
            let content_item_id = translate_parsed_type_expression(schema, content_item)?;
            content_item_ids.push(content_item_id);
        }
        tags.insert(tag_name, content_item_ids);
    }
    schema.add_constraint(
        type_id,
        Constraint::TagAtMost(TagAtMostConstraint { tags }),
        &mut CheckedTypes::new(),
    )?;
    Ok(type_id)
}

fn translate_parsed_type_expression(
    schema: &mut TypeSchema,
    expression: &TypeExpression,
) -> Result<TypeId, String> {
    match expression {
        TypeExpression::Function(function) => translate_function_type(schema, function),
        TypeExpression::Identifier(identifier) => {
            translate_type_identifier_type(schema, identifier)
        }
        TypeExpression::List(list) => translate_list_type(schema, list),
        TypeExpression::Record(record) => translate_record_type(schema, record),
        TypeExpression::TagGroup(tags) => translate_tag_group_type(schema, tags),
    }
}

pub fn translate_parsed_expression_to_generic_expression<'a>(
    schema: &mut TypeSchema,
    expression: Expression<'a>,
) -> Result<GenericExpression<'a>, String> {
    match expression {
        Expression::BinaryOperator(node) => translate_binary_operator(schema, node)
            .map(Box::new)
            .map(GenericExpression::BinaryOperator),
        Expression::Block(node) => translate_block(schema, node)
            .map(Box::new)
            .map(GenericExpression::Block),
        Expression::Declaration(node) => translate_declaration(schema, node)
            .map(Box::new)
            .map(GenericExpression::Declaration),
        Expression::Function(node) => translate_function(schema, node, None)
            .map(Box::new)
            .map(GenericExpression::Function),
        Expression::FunctionApplicationArguments(_) => Err(generate_backtrace_error(
            "UnreachableFunctionApplicationArgumentExpression".to_owned(),
        )),
        Expression::Identifier(node) => Ok(GenericExpression::Identifier(Box::new(
            translate_identifier(schema, node)?,
        ))),
        Expression::If(node) => translate_if(schema, node)
            .map(Box::new)
            .map(GenericExpression::If),
        Expression::Integer(node) => Ok(GenericExpression::Integer(Box::new(translate_integer(
            schema, node,
        )?))),
        Expression::List(node) => translate_list(schema, node)
            .map(Box::new)
            .map(GenericExpression::List),
        Expression::Record(node) => translate_record(schema, node)
            .map(Box::new)
            .map(GenericExpression::Record),
        Expression::RecordAssignment(node) => translate_record_assignment(schema, node)
            .map(Box::new)
            .map(GenericExpression::RecordAssignment),
        Expression::StringLiteral(node) => Ok(GenericExpression::StringLiteral(Box::new(
            translate_string(schema, node)?,
        ))),
        Expression::Tag(node) => translate_tag(schema, node)
            .map(Box::new)
            .map(GenericExpression::Tag),
        Expression::TypeDeclaration(node) => translate_type_declaration(schema, node)
            .map(Box::new)
            .map(GenericExpression::TypeDeclaration),
        Expression::UnaryOperator(node) => translate_unary_operator(schema, node)
            .map(Box::new)
            .map(GenericExpression::UnaryOperator),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::{FunctionApplicationArgumentsNode, FunctionApplicationArgumentsValue, ParserInput};
    use indoc::indoc;
    use parser::parse_test_expression;
    use typed_ast::ConcreteType;

    const INITIAL_CONSTRAINT_COUNT: usize = 2;

    #[test]
    fn binary_operator_increments_id_counter_by_one_more_than_total_number_of_ids_in_children() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314 + 271");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 3);
    }

    #[test]
    fn arithmetic_binary_operator_adds_three_constraints_beyond_those_added_by_its_children() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314 + 271");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn concatenate_binary_operator_adds_three_constraints_beyond_those_added_by_its_children() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("\"Hello\" ++ \"World\"");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn logic_binary_operator_adds_three_constraints_beyond_those_added_by_its_children() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("a and b");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn equality_binary_operator_adds_two_constraints_beyond_those_added_by_its_children() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314 == 271");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 2
        );
    }

    #[test]
    fn equality_binary_operator_only_has_two_canonical_ids_when_children_only_have_one_type_each() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314 == 271");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 2
        );
    }

    #[test]
    fn ordered_comparison_binary_operator_adds_four_constraints_beyond_those_added_by_its_children()
    {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314 < 271");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn function_arguments_binary_operator_has_one_more_canonical_id_than_sum_of_canonical_ids_in_children(
    ) {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        let expression = parse_test_expression("a(\"hello\", 314, 271)");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 5
        );
    }

    #[test]
    fn function_arguments_binary_operator_adds_two_constraints_beyond_those_added_by_its_children()
    {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        let expression = parse_test_expression("a(\"hello\", 314, 271)");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 5
        );
    }

    #[test]
    fn method_lookup_binary_operator_only_has_two_canonical_ids_when_both_left_and_right_are_identifiers(
    ) {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("a:b");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn method_lookup_binary_operator_only_adds_one_constraint_when_both_left_and_right_are_identifiers(
    ) {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("a:b");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn field_lookup_binary_operator_only_has_two_canonical_ids_when_both_left_and_right_are_identifiers(
    ) {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("a.b");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn field_lookup_binary_operator_only_adds_one_constraint_when_both_left_and_right_are_identifiers(
    ) {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("a.b");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn binary_operator_preserves_symbol() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314 + 271");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::BinaryOperator(binary_operator_expression) = result {
            assert_eq!(binary_operator_expression.symbol, BinaryOperatorSymbol::Add);
        } else {
            panic!();
        }
    }

    #[test]
    fn block_input_increments_id_counter_by_two_more_than_total_number_of_ids_in_the_contents() {
        let mut schema = TypeSchema::new();
        let expression = Expression::Block(BlockNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 4);
    }

    #[test]
    fn for_block_input_each_element_in_input_block_has_corresponding_element_in_translated_block() {
        let mut schema = TypeSchema::new();
        let expression = Expression::Block(BlockNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::Block(block_expression) = result {
            assert_eq!(block_expression.contents.len(), 3);
        } else {
            panic!();
        }
    }

    #[test]
    fn block_input_with_primitive_elements_has_as_many_canonical_ids_as_elements() {
        let mut schema = TypeSchema::new();
        let expression = Expression::Block(BlockNode {
            source: ParserInput::new(""),
            value: vec![
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 2,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 3,
                }),
                Expression::Integer(IntegerNode {
                    source: ParserInput::new(""),
                    value: 5,
                }),
            ],
        });
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn function_application_arguments_does_not_increment_id_counter() {
        let mut schema = TypeSchema::new();
        let expression =
            Expression::FunctionApplicationArguments(FunctionApplicationArgumentsNode {
                source: ParserInput::new(""),
                value: FunctionApplicationArgumentsValue { arguments: vec![] },
            });
        let _ = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT);
    }

    #[test]
    fn function_application_arguments_input_errors() {
        let mut schema = TypeSchema::new();
        let expression =
            Expression::FunctionApplicationArguments(FunctionApplicationArgumentsNode {
                source: ParserInput::new(""),
                value: FunctionApplicationArgumentsValue { arguments: vec![] },
            });
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn identifier_input_preserves_name() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("hello").unwrap();
        let expression = parse_test_expression("hello");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::Identifier(identifier_expression) = result {
            assert_eq!(identifier_expression.name, "hello");
        } else {
            panic!();
        }
    }

    #[test]
    fn when_false_branch_is_absent_if_increments_id_counter_by_one_beyond_children() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("if a do b");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 3);
    }

    #[test]
    fn when_false_branch_is_present_if_increments_id_counter_by_one_beyond_children() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        schema.make_identifier_for_test("c").unwrap();
        let expression = parse_test_expression("if a do b else c");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 4);
    }

    #[test]
    fn when_false_branch_is_present_if_only_has_two_canonical_ids_when_condition_and_paths_are_all_identifiers(
    ) {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        schema.make_identifier_for_test("c").unwrap();
        let expression = parse_test_expression("if a do b else c");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 2
        );
    }

    #[test]
    fn when_false_branch_is_absent_if_adds_three_constraints() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("if a do b");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn when_false_branch_is_present_if_adds_one_constraint() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        schema.make_identifier_for_test("c").unwrap();
        let expression = parse_test_expression("if a do b else c");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 2
        );
    }

    #[test]
    fn errors_if_conditional_is_not_a_boolean() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        let expression = parse_test_expression("if #green do a");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn errors_with_the_branches_have_different_types() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("if #true do 1 else \"2\"");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn integer_input_increments_id_counter_by_one() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 1);
    }

    #[test]
    fn integer_input_adds_one_constraint() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 1
        );
    }

    #[test]
    fn integer_input_returns_integer_with_preserved_value() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("314");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::Integer(integer_expression) = result {
            assert_eq!(integer_expression.value, 314);
        } else {
            panic!();
        }
    }

    #[test]
    fn list_input_increments_id_counter_by_two_more_than_total_number_of_ids_in_the_contents() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("[2, 3, 5]");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 9);
    }

    #[test]
    fn list_input_adds_one_constraint_beyond_those_added_by_its_contents() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("[2, 3, 5]");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 6
        );
    }

    #[test]
    fn for_list_input_each_element_in_input_list_has_corresponding_element_in_translated_list() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("[2, 3, 5]");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::List(list_node) = result {
            assert_eq!(list_node.contents.len(), 3);
        } else {
            panic!();
        }
    }

    #[test]
    fn list_input_with_primitive_elements_has_only_two_canonical_ids() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("[2, 3, 5]");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 6
        );
    }

    #[test]
    fn lists_of_mixed_types_errors() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("[1, \"hello\"]");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn record_input_increments_id_counter_by_two_for_each_field_plus_one_for_the_record() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("{ a: 3, b: 4 }");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 5);
    }

    #[test]
    fn record_adds_one_constraint_plus_two_more_for_each_field() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("{ a: 3, b: 4 }");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 5
        );
    }

    #[test]
    fn for_record_input_each_field_in_input_list_has_corresponding_field_in_translated_list() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        schema.make_identifier_for_test("b").unwrap();
        let expression = parse_test_expression("{ a: 3, b: 4 }");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::Record(record_node) = result {
            assert_eq!(record_node.contents.len(), 2);
        } else {
            panic!();
        }
    }

    #[test]
    fn record_input_has_one_canonical_id_plus_one_more_for_each_primitive_field() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("{ a: 3, b: 4 }");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 3
        );
    }

    #[test]
    fn record_assignment_type_checks() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("a = { a: 3 }");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression("{ a | a: 2 }");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_ok());
    }

    #[test]
    fn record_assignment_only_needs_to_update_a_subset_of_fields() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("a = { a: 3, b: 2 }");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression("{ a | a: 2 }");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_ok());
    }

    #[test]
    fn record_assignment_cannot_add_new_fields() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("a = { a: 3 }");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression("{ a | b: 2 }");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn string_input_increments_id_counter_by_one() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("\"hello\"");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 1);
    }

    #[test]
    fn string_input_adds_one_constraint() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("\"hello\"");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 1
        );
    }

    #[test]
    fn string_input_returns_string_with_preserved_value() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("\"hello\"");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::StringLiteral(string_literal_expression) = result {
            assert_eq!(string_literal_expression.value, "hello");
        } else {
            panic!();
        }
    }

    #[test]
    fn tag_increments_id_counter_by_one() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("#a");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 1);
    }

    #[test]
    fn tag_with_no_contents_adds_one_constraint() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("#a");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 1
        );
    }

    #[test]
    fn tag_preserves_name() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("#a");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::Tag(tag_expression) = result {
            assert_eq!(tag_expression.name, "a");
        } else {
            panic!();
        }
    }

    #[test]
    fn unary_operator_input_increments_id_counter_by_one_more_than_added_by_its_child() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("-314");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(schema.count_ids(), INITIAL_CONSTRAINT_COUNT + 2);
    }

    #[test]
    fn unary_operator_negative_input_adds_two_constraints_beyond_those_added_by_the_child() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("-314");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 2
        );
    }

    #[test]
    fn unary_operator_not_input_adds_three_constraints_beyond_those_added_by_the_child() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("hello").unwrap();
        let expression = parse_test_expression("not hello");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        assert_eq!(
            schema.get_total_canonical_ids(),
            INITIAL_CONSTRAINT_COUNT + 2
        );
    }

    #[test]
    fn unary_operator_negative_input_preserves_symbol() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("-314");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::UnaryOperator(unary_operator_expression) = result {
            assert_eq!(
                unary_operator_expression.symbol,
                UnaryOperatorSymbol::Negative
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn unary_operator_not_input_preserves_symbol() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("hello").unwrap();
        let expression = parse_test_expression("not hello");
        let result =
            translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        if let GenericExpression::UnaryOperator(unary_operator_expression) = result {
            assert_eq!(unary_operator_expression.symbol, UnaryOperatorSymbol::Not);
        } else {
            panic!();
        }
    }

    #[test]
    fn functions_can_accept_no_arguments_and_return_a_number() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("(a) => 1");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_ok());
    }

    #[test]
    fn function_can_accept_an_argument_and_return_it() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("(a) => a");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_ok());
    }

    #[test]
    fn function_can_return_an_arguments_field() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("(a) => a.b");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_ok());
    }

    #[test]
    fn cannot_shadow_variables() {
        let mut schema = TypeSchema::new();
        schema.make_identifier_for_test("a").unwrap();
        let expression = parse_test_expression(indoc! {"
            () =>
                a = 1
                a
        "});
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn can_declare_a_type_identifier() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("MyInt = Int");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_ok());
    }

    #[test]
    fn declaring_type_identifier_adds_it_to_the_scope() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("MyInt = Int");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let my_int_type_id = schema.scope.get_variable_declaration_type("MyInt").unwrap();
        assert_eq!(
            schema.get_concrete_type_from_id(my_int_type_id),
            ConcreteType::Primitive(PrimitiveType::Int)
        );
    }

    #[test]
    fn type_in_declaration_must_match_expression_type() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("string: Str = 1");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn type_in_declaration_can_reference_previous_type_declaration() {
        let mut schema = TypeSchema::new();
        let id = schema.make_id();
        schema
            .scope
            .declare_identifier(String::from("MyInt"), id)
            .unwrap();
        let expression = parse_test_expression("int: MyInt = 1");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_ok());
    }

    #[test]
    fn checks_if_function_result_is_assigned_to_wrong_type() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("func: (Int) => Int = (a) => a");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression("hello: Str = func(1)");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn use_declaration_type_annotation_to_infer_function_argument_types() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("func: (Int) => Str = (a) => a");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn use_declaration_type_annotation_to_infer_function_argument_types_with_separate_type_declaration(
    ) {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("Func = (Int) => Str");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression("func: Func = (a) => a");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn argument_type_can_be_more_flexible_than_the_declaration_type() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("Primary = #red | #green | #blue");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression =
            parse_test_expression("Rainbow = #red | #orange | #yellow | #green | #blue | #purple");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression(
            "isBlue: (Primary) => #true | #false = (a: Rainbow) => a == #blue",
        );
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_ok());
    }

    #[test]
    fn argument_type_can_not_be_less_flexible_than_the_declaration_type() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("Primary = #red | #green | #blue");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression =
            parse_test_expression("Rainbow = #red | #orange | #yellow | #green | #blue | #purple");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression(
            "isGreen: (Rainbow) => #true | #false = (a: Primary) => a == #green",
        );
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn functions_with_different_numbers_of_arguments_do_not_type_check() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("func: (Int, Int) => Str = (a) => a");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }

    #[test]
    fn cannot_use_a_function_to_lookup_a_non_existent_field() {
        let mut schema = TypeSchema::new();
        let expression = parse_test_expression("person = { name: \"Derek\" }");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression("getAge = (arg) => arg.age");
        translate_parsed_expression_to_generic_expression(&mut schema, expression).unwrap();
        let expression = parse_test_expression("age = getAge(person)");
        let result = translate_parsed_expression_to_generic_expression(&mut schema, expression);
        assert!(result.is_err());
    }
}
