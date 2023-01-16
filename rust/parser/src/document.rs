use crate::{
    import::import, line::line, newline::newline, type_declaration::type_declaration,
    variable_declaration::variable_declaration, ExpressionContext,
};
use ast::{
    DocumentNode, DocumentValue, Expression, IResult, ImportNode, ParserInput, TypeDeclarationNode,
    VariableDeclarationNode,
};
use nom::{
    branch::alt,
    combinator::{consumed, eof, map},
    multi::many0,
    sequence::terminated,
};

enum DocumentElement<'a> {
    None,
    Import(ImportNode<'a>),
    TypeDeclaration(TypeDeclarationNode<'a>),
    VariableDeclaration(VariableDeclarationNode<'a>),
    Expression(Expression<'a>),
}

pub fn document<'a>(
    context: ExpressionContext,
) -> impl FnMut(ParserInput<'a>) -> IResult<'a, DocumentNode<'a>> {
    map(
        consumed(many0(alt((
            map(
                terminated(import, alt((newline, eof))),
                DocumentElement::Import,
            ),
            map(
                terminated(type_declaration, alt((newline, eof))),
                DocumentElement::TypeDeclaration,
            ),
            map(
                terminated(variable_declaration, alt((newline, eof))),
                DocumentElement::VariableDeclaration,
            ),
            map(line(ExpressionContext::new()), |maybe_expression| {
                maybe_expression.map_or(DocumentElement::None, |expression| {
                    DocumentElement::Expression(expression)
                })
            }),
        )))),
        |(source, document_elements)| {
            let mut document = DocumentValue {
                imports: vec![],
                type_declarations: vec![],
                variable_declarations: vec![],
                expressions: vec![],
            };
            for element in document_elements {
                match element {
                    DocumentElement::None => {}
                    DocumentElement::Import(elem) => document.imports.push(elem),
                    DocumentElement::TypeDeclaration(elem) => document.type_declarations.push(elem),
                    DocumentElement::VariableDeclaration(elem) => {
                        document.variable_declarations.push(elem);
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
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn import_is_document() {
        let input = ParserInput::new("import a from \"file.buri\"");
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn import_value_is_preserved() {
        let input = ParserInput::new("import a from \"file.buri\"");
        let result = document(ExpressionContext::new())(input);
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
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn type_declaration_value_is_preserved() {
        let input = ParserInput::new("Hello = World");
        let result = document(ExpressionContext::new())(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.type_declarations.len(), 1);
        assert_eq!(
            parsed
                .value
                .type_declarations
                .get(0)
                .unwrap()
                .value
                .identifier
                .value,
            "Hello"
        );
    }

    #[test]
    fn literal_is_document() {
        let input = ParserInput::new("0");
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn literal_value_is_preserved() {
        let input = ParserInput::new("314");
        let result = document(ExpressionContext::new())(input);
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
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn variable_declaration_is_document() {
        let input = ParserInput::new("hello = world");
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn function_call_is_document() {
        let input = ParserInput::new("main()");
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn variable_declaration_value_is_preserved() {
        let input = ParserInput::new("hello = world");
        let result = document(ExpressionContext::new())(input);
        let (_, parsed) = result.unwrap();
        assert_eq!(parsed.value.variable_declarations.len(), 1);
        assert_eq!(
            parsed
                .value
                .variable_declarations
                .get(0)
                .unwrap()
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
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn document_can_contain_empty_lines() {
        let input = ParserInput::new("import a from \"file.buri\"\n\na");
        let result = document(ExpressionContext::new())(input);
        let (remainder, _) = result.unwrap();
        assert_eq!(remainder, "");
    }
}
