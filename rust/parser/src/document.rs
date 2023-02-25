use crate::{
    import::import, line::line, newline::newline, type_declaration::type_declaration,
    variable_declaration::variable_declaration, ExpressionContext,
};
use ast::{
    DeclarationNode, DocumentNode, DocumentValue, Expression, IResult, ImportNode, ParserInput,
    TopLevelDeclaration, TypeDeclarationNode,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{consumed, eof, map, opt, value},
    multi::many0,
    sequence::{terminated, tuple},
};

enum DocumentElement<'a> {
    None,
    Import(ImportNode<'a>),
    TypeDeclaration(TypeDeclarationNode<'a>),
    VariableDeclaration(DeclarationNode<'a>),
    Expression(Expression<'a>),
}

fn declaration(input: ParserInput) -> IResult<DocumentElement> {
    alt((
        map(
            terminated(type_declaration, alt((newline, eof))),
            DocumentElement::TypeDeclaration,
        ),
        // Newlines are required by variable_declarations, so we don't
        // need to add them here.
        map(
            move |input| variable_declaration(ExpressionContext::new(), input),
            DocumentElement::VariableDeclaration,
        ),
    ))(input)
}

pub fn document<'a>() -> impl FnMut(ParserInput<'a>) -> IResult<'a, DocumentNode<'a>> {
    map(
        consumed(many0(alt((
            tuple((
                map(
                    opt(terminated(tag("@export"), tuple((space0, newline)))),
                    |maybe_export| maybe_export.is_some(),
                ),
                declaration,
            )),
            tuple((
                value(false, tag("")),
                alt((
                    map(newline, |_| DocumentElement::None),
                    map(
                        terminated(import, alt((newline, eof))),
                        DocumentElement::Import,
                    ),
                    declaration,
                    map(line(ExpressionContext::new()), |maybe_expression| {
                        maybe_expression.map_or(DocumentElement::None, |expression| {
                            DocumentElement::Expression(expression)
                        })
                    }),
                )),
            )),
        )))),
        |(source, document_elements)| {
            let mut document = DocumentValue {
                imports: vec![],
                type_declarations: vec![],
                variable_declarations: vec![],
                expressions: vec![],
            };
            for (is_exported, element) in document_elements {
                match element {
                    DocumentElement::None => {}
                    DocumentElement::Import(elem) => document.imports.push(elem),
                    DocumentElement::TypeDeclaration(elem) => {
                        document.type_declarations.push(TopLevelDeclaration {
                            declaration: elem,
                            is_exported,
                        });
                    }
                    DocumentElement::VariableDeclaration(elem) => {
                        document.variable_declarations.push(TopLevelDeclaration {
                            declaration: elem,
                            is_exported,
                        });
                    }
                    DocumentElement::Expression(elem) => document.expressions.push(elem),
                }
            }
            DocumentNode {
                source,
                value: document,
            }
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    use ast::{Expression, ImportNode, ImportValue, IntegerNode};

    #[test]
    fn blank_document_is_document() {
        let input = ParserInput::new("");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn import_is_document() {
        let input = ParserInput::new("import a from \"file.buri\"");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn import_value_is_preserved() {
        let input = ParserInput::new("import a from \"file.buri\"");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.imports.len(), 1);
        assert!(matches!(
            parsed.value.imports.get(0).unwrap(),
            ImportNode {
                value: ImportValue {
                    path: "file.buri",
                    ..
                },
                ..
            }
        ));
    }

    #[test]
    fn type_declaration_is_document() {
        let input = ParserInput::new("Hello = World");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn type_declaration_value_is_preserved() {
        let input = ParserInput::new("Hello = World");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.type_declarations.len(), 1);
        assert_eq!(
            parsed
                .value
                .type_declarations
                .get(0)
                .unwrap()
                .declaration
                .value
                .identifier
                .value,
            "Hello"
        );
    }

    #[test]
    fn literal_is_document() {
        let input = ParserInput::new("0");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn literal_value_is_preserved() {
        let input = ParserInput::new("314");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.expressions.len(), 1);
        assert!(matches!(
            parsed.value.expressions.get(0).unwrap(),
            Expression::Integer(IntegerNode { value: 314, .. })
        ));
    }

    #[test]
    fn identifier_is_document() {
        let input = ParserInput::new("hello");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn variable_declaration_is_document() {
        let input = ParserInput::new("hello = world");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_call_is_document() {
        let input = ParserInput::new("main()");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn variable_declaration_value_is_preserved() {
        let input = ParserInput::new("hello = world");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.variable_declarations.len(), 1);
        assert_eq!(
            parsed
                .value
                .variable_declarations
                .get(0)
                .unwrap()
                .declaration
                .value
                .identifier
                .value
                .name,
            "hello"
        );
    }

    #[test]
    fn document_can_contain_multiple_lines_of_different_types() {
        let input = ParserInput::new("import a from \"file.buri\"\nHello = World\na");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn document_can_contain_empty_lines() {
        let input = ParserInput::new("import a from \"file.buri\"\n\na");
        let result = document()(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn declaration_without_export_decorator_is_not_exported() {
        let input = ParserInput::new("hello = world");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.variable_declarations.len(), 1);
        assert!(
            !parsed
                .value
                .variable_declarations
                .get(0)
                .unwrap()
                .is_exported
        );
    }

    #[test]
    fn type_declaration_without_export_decorator_is_not_exported() {
        let input = ParserInput::new("Hello = World");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.type_declarations.len(), 1);
        assert!(!parsed.value.type_declarations.get(0).unwrap().is_exported);
    }

    #[test]
    fn declaration_with_export_decorator_is_exported() {
        let input = ParserInput::new("@export\nhello = world");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.variable_declarations.len(), 1);
        assert!(
            parsed
                .value
                .variable_declarations
                .get(0)
                .unwrap()
                .is_exported
        );
    }

    #[test]
    fn type_declaration_with_export_decorator_is_exported() {
        let input = ParserInput::new("@export\nHello = World");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.type_declarations.len(), 1);
        assert!(parsed.value.type_declarations.get(0).unwrap().is_exported);
    }

    #[test]
    fn can_have_spaces_after_export_decorator() {
        let input = ParserInput::new("@export      \nhello = world");
        let result = document()(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.variable_declarations.len(), 1);
        assert!(
            parsed
                .value
                .variable_declarations
                .get(0)
                .unwrap()
                .is_exported
        );
    }

    #[test]
    fn can_have_multiline_if_statements_in_declarations() {
        let input = ParserInput::new("foo = if #true do\n    5\nelse\n    3");
        let result = document()(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder.value(), "");
        assert_eq!(parsed.value.variable_declarations.len(), 1);
    }

    #[test]
    fn can_have_complex_functions_in_declarations() {
        let input = ParserInput::new("ifTrueFiveElseThree = (a) => if a do 5 else 3");
        let result = document()(input);
        let (remainder, parsed) = result.unwrap();
        assert_eq!(remainder.value(), "");
        assert_eq!(parsed.value.variable_declarations.len(), 1);
    }

    #[test]
    fn cannot_export_newline() {
        let input = ParserInput::new("@export\n\n");
        let (_, parsed) = document()(input).unwrap();
        // The input was not parsed
        assert_eq!(parsed.source, "");
    }

    #[test]
    fn cannot_export_an_import() {
        let input = ParserInput::new("@export\nimport a from \"file.buri\"");
        let (_, parsed) = document()(input).unwrap();
        // The input was not parsed
        assert_eq!(parsed.source, "");
    }

    #[test]
    fn cannot_export_an_expression() {
        let input = ParserInput::new("@export\n5");
        let (_, parsed) = document()(input).unwrap();
        // The input was not parsed
        assert_eq!(parsed.source, "");
    }
}
