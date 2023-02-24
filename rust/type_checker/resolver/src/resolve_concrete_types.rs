use ast::TopLevelDeclaration;
use type_checker_types::{
    generic_nodes::{
        GenericBinaryOperatorExpression, GenericBlockExpression, GenericBooleanExpression,
        GenericDeclarationExpression, GenericDocument, GenericExpression,
        GenericFunctionExpression, GenericIdentifierExpression, GenericIfExpression,
        GenericIntegerLiteralExpression, GenericListExpression, GenericRecordAssignmentExpression,
        GenericRecordExpression, GenericStringLiteralExpression, GenericTagExpression,
        GenericUnaryOperatorExpression,
    },
    type_schema::TypeSchema,
    TypeId,
};
use typed_ast::{
    ConcreteBinaryOperatorExpression, ConcreteBlockExpression, ConcreteBooleanExpression,
    ConcreteDeclarationExpression, ConcreteDocument, ConcreteExpression,
    ConcreteFunctionExpression, ConcreteIdentifierExpression, ConcreteIfExpression,
    ConcreteIntegerLiteralExpression, ConcreteListExpression, ConcreteRecordAssignmentExpression,
    ConcreteRecordExpression, ConcreteStringLiteralExpression, ConcreteTagExpression, ConcreteType,
    ConcreteUnaryOperatorExpression, PrimitiveType, TypedDeclarationExpression,
};

fn resolve_generic_type(schema: &mut TypeSchema, type_id: TypeId) -> ConcreteType {
    schema.get_concrete_type_from_id(type_id)
}

fn resolve_binary_operator(
    simplified_schema: &mut TypeSchema,
    generic_binary_operator: GenericBinaryOperatorExpression,
) -> ConcreteExpression {
    ConcreteExpression::BinaryOperator(Box::new(ConcreteBinaryOperatorExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_binary_operator.expression_type.type_id,
        ),
        symbol: generic_binary_operator.symbol,
        left_child: resolve_expression(simplified_schema, generic_binary_operator.left_child),
        right_child: resolve_expression(simplified_schema, generic_binary_operator.right_child),
    }))
}

fn resolve_block(
    simplified_schema: &mut TypeSchema,
    generic_block: GenericBlockExpression,
) -> ConcreteExpression {
    ConcreteExpression::Block(Box::new(ConcreteBlockExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_block.expression_type.type_id,
        ),
        contents: generic_block
            .contents
            .into_iter()
            .map(|x| resolve_expression(simplified_schema, x))
            .collect(),
    }))
}

fn resolve_boolean(
    simplified_schema: &mut TypeSchema,
    generic_boolean: &GenericBooleanExpression,
) -> ConcreteExpression {
    ConcreteExpression::Boolean(Box::new(ConcreteBooleanExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_boolean.expression_type.type_id,
        ),
        value: generic_boolean.value,
    }))
}

fn resolve_declaration(
    simplified_schema: &mut TypeSchema,
    generic_declaration: GenericDeclarationExpression,
) -> ConcreteExpression {
    let generic_type = resolve_generic_type(
        simplified_schema,
        generic_declaration.expression_type.type_id,
    );
    ConcreteExpression::Declaration(Box::new(ConcreteDeclarationExpression {
        declaration_type: generic_type.clone(),
        expression_type: generic_type,
        identifier: match resolve_expression(
            simplified_schema,
            GenericExpression::Identifier(Box::new(generic_declaration.identifier)),
        ) {
            ConcreteExpression::Identifier(x) => *x,
            _ => unreachable!(),
        },
        value: resolve_expression(simplified_schema, generic_declaration.value),
    }))
}

fn resolve_function(
    simplified_schema: &mut TypeSchema,
    generic_function: GenericFunctionExpression,
) -> ConcreteExpression {
    ConcreteExpression::Function(Box::new(ConcreteFunctionExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_function.expression_type.type_id,
        ),
        argument_names: generic_function.argument_names,
        body: resolve_expression(simplified_schema, generic_function.body),
    }))
}

fn resolve_function_arguments(
    simplified_schema: &mut TypeSchema,
    generic_function_arguments: Vec<GenericExpression>,
) -> ConcreteExpression {
    ConcreteExpression::FunctionArguments(
        generic_function_arguments
            .into_iter()
            .map(|x| resolve_expression(simplified_schema, x))
            .collect(),
    )
}

fn resolve_identifier(
    simplified_schema: &mut TypeSchema,
    generic_identifier: GenericIdentifierExpression,
) -> ConcreteIdentifierExpression {
    ConcreteIdentifierExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_identifier.expression_type.type_id,
        ),
        name: generic_identifier.name,
        is_disregarded: generic_identifier.is_disregarded,
    }
}

fn resolve_if(
    simplified_schema: &mut TypeSchema,
    generic_if: GenericIfExpression,
) -> ConcreteExpression {
    ConcreteExpression::If(Box::new(ConcreteIfExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_if.expression_type.type_id,
        ),
        condition: resolve_expression(simplified_schema, generic_if.condition),
        path_if_true: resolve_expression(simplified_schema, generic_if.path_if_true),
        path_if_false: generic_if
            .path_if_false
            .map(|expression| resolve_expression(simplified_schema, expression)),
    }))
}

fn resolve_integer(
    simplified_schema: &mut TypeSchema,
    generic_integer: &GenericIntegerLiteralExpression,
) -> ConcreteExpression {
    ConcreteExpression::Integer(Box::new(ConcreteIntegerLiteralExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_integer.expression_type.type_id,
        ),
        value: generic_integer.value,
    }))
}

fn resolve_record_assignment(
    simplified_schema: &mut TypeSchema,
    generic_record_assignment: GenericRecordAssignmentExpression,
) -> ConcreteExpression {
    ConcreteExpression::RecordAssignment(Box::new(ConcreteRecordAssignmentExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_record_assignment.expression_type.type_id,
        ),
        identifier: resolve_identifier(simplified_schema, generic_record_assignment.identifier),
        contents: resolve_record(simplified_schema, generic_record_assignment.contents),
    }))
}

fn resolve_list(
    simplified_schema: &mut TypeSchema,
    generic_list: GenericListExpression,
) -> ConcreteExpression {
    ConcreteExpression::List(Box::new(ConcreteListExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_list.expression_type.type_id,
        ),
        contents: generic_list
            .contents
            .into_iter()
            .map(|item| resolve_expression(simplified_schema, item))
            .collect(),
    }))
}

fn resolve_record(
    simplified_schema: &mut TypeSchema,
    generic_record: GenericRecordExpression,
) -> ConcreteRecordExpression {
    ConcreteRecordExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_record.expression_type.type_id,
        ),
        contents: generic_record
            .contents
            .into_iter()
            .map(|(key, value)| (key, resolve_expression(simplified_schema, value)))
            .collect(),
    }
}

fn resolve_string_literal(
    simplified_schema: &mut TypeSchema,
    generic_string_literal: GenericStringLiteralExpression,
) -> ConcreteExpression {
    ConcreteExpression::StringLiteral(Box::new(ConcreteStringLiteralExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_string_literal.expression_type.type_id,
        ),
        value: generic_string_literal.value,
    }))
}

fn resolve_tag(
    simplified_schema: &mut TypeSchema,
    generic_tag: GenericTagExpression,
) -> ConcreteExpression {
    let expression_type =
        resolve_generic_type(simplified_schema, generic_tag.expression_type.type_id);
    if expression_type == ConcreteType::Primitive(PrimitiveType::CompilerBoolean) {
        return ConcreteExpression::Boolean(Box::new(ConcreteBooleanExpression {
            expression_type,
            value: generic_tag.name == "true",
        }));
    }
    ConcreteExpression::Tag(Box::new(ConcreteTagExpression {
        expression_type,
        name: generic_tag.name,
        contents: generic_tag
            .contents
            .into_iter()
            .map(|x| resolve_expression(simplified_schema, x))
            .collect(),
    }))
}

fn resolve_unary_operator(
    simplified_schema: &mut TypeSchema,
    generic_unary_operator: GenericUnaryOperatorExpression,
) -> ConcreteExpression {
    ConcreteExpression::UnaryOperator(Box::new(ConcreteUnaryOperatorExpression {
        expression_type: resolve_generic_type(
            simplified_schema,
            generic_unary_operator.expression_type.type_id,
        ),
        symbol: generic_unary_operator.symbol,
        child: resolve_expression(simplified_schema, generic_unary_operator.child),
    }))
}

fn resolve_expression(
    simplified_schema: &mut TypeSchema,
    expression: GenericExpression,
) -> ConcreteExpression {
    match expression {
        GenericExpression::BinaryOperator(generic_binary_operator) => {
            resolve_binary_operator(simplified_schema, *generic_binary_operator)
        }
        GenericExpression::Block(generic_block) => resolve_block(simplified_schema, *generic_block),
        GenericExpression::Boolean(generic_boolean) => {
            resolve_boolean(simplified_schema, &generic_boolean)
        }
        GenericExpression::Declaration(generic_declaration) => {
            resolve_declaration(simplified_schema, *generic_declaration)
        }
        GenericExpression::Function(generic_function) => {
            resolve_function(simplified_schema, *generic_function)
        }
        GenericExpression::FunctionArguments(generic_function_arguments) => {
            resolve_function_arguments(simplified_schema, generic_function_arguments)
        }
        GenericExpression::Identifier(generic_identifier) => ConcreteExpression::Identifier(
            Box::new(resolve_identifier(simplified_schema, *generic_identifier)),
        ),
        GenericExpression::If(generic_if) => resolve_if(simplified_schema, *generic_if),
        GenericExpression::Integer(generic_integer) => {
            resolve_integer(simplified_schema, &generic_integer)
        }
        GenericExpression::RecordAssignment(generic_record_assignment) => {
            resolve_record_assignment(simplified_schema, *generic_record_assignment)
        }
        GenericExpression::List(list) => resolve_list(simplified_schema, *list),
        GenericExpression::Record(record) => {
            ConcreteExpression::Record(Box::new(resolve_record(simplified_schema, *record)))
        }
        GenericExpression::StringLiteral(generic_string_literal) => {
            resolve_string_literal(simplified_schema, *generic_string_literal)
        }
        GenericExpression::Tag(tag) => resolve_tag(simplified_schema, *tag),
        GenericExpression::UnaryOperator(generic_unary_operator) => {
            resolve_unary_operator(simplified_schema, *generic_unary_operator)
        }
        _ => unimplemented!(),
    }
}

fn resolve_variable_declaration_types(
    schema: &mut TypeSchema,
    input: TopLevelDeclaration<GenericDeclarationExpression>,
) -> TopLevelDeclaration<ConcreteDeclarationExpression> {
    let resolved_type = resolve_generic_type(schema, input.declaration.declaration_type.type_id);
    TopLevelDeclaration {
        declaration: ConcreteDeclarationExpression {
            declaration_type: resolved_type.clone(),
            expression_type: resolved_type,
            identifier: resolve_identifier(schema, input.declaration.identifier),
            value: resolve_expression(schema, input.declaration.value),
        },
        is_exported: input.is_exported,
    }
}

#[must_use]
pub fn resolve_concrete_types(mut schema: TypeSchema, input: GenericDocument) -> ConcreteDocument {
    let variable_declarations: Vec<TopLevelDeclaration<TypedDeclarationExpression<ConcreteType>>> =
        input
            .variable_declarations
            .into_iter()
            .map(|input| resolve_variable_declaration_types(&mut schema, input))
            .collect();
    ConcreteDocument {
        imports: input.imports,
        // We don't need to resolve type declarations since they are always
        // generic and never printed to the JS output.
        type_declarations: vec![],
        variable_declarations,
        // We don't need to resolve top-level expressions since they are
        // printed to the JS output.
        expressions: vec![],
    }
}
