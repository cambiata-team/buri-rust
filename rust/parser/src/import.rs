use crate::{
    identifier::identifier, intra_expression_whitespace::intra_expression_whitespace,
    type_identifier::type_identifier, ExpressionContext,
};
use ast::{IResult, ParserInput};
use ast::{ImportNode, ImportValue, ImportedIdentifier};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::char,
    combinator::{consumed, map, opt, recognize},
    multi::separated_list1,
    sequence::{delimited, tuple},
};

fn import_path(input: ParserInput) -> IResult<ParserInput> {
    delimited(
        char('\"'),
        recognize(take_while(|c: char| {
            c != '\"' && c.is_ascii() && !c.is_ascii_control()
        })),
        char('\"'),
    )(input)
}

pub fn import(input: ParserInput) -> IResult<ImportNode> {
    map(
        consumed(tuple((
            delimited(
                tuple((
                    tag("import"),
                    intra_expression_whitespace(
                        ExpressionContext::new().allow_newlines_in_expressions(),
                    ),
                )),
                separated_list1(
                    tuple((
                        opt(intra_expression_whitespace(
                            ExpressionContext::new().allow_newlines_in_expressions(),
                        )),
                        char(','),
                        opt(intra_expression_whitespace(
                            ExpressionContext::new().allow_newlines_in_expressions(),
                        )),
                    )),
                    alt((
                        map(identifier, ImportedIdentifier::Identifier),
                        map(type_identifier, ImportedIdentifier::TypeIdentifier),
                    )),
                ),
                tuple((
                    intra_expression_whitespace(
                        ExpressionContext::new().allow_newlines_in_expressions(),
                    ),
                    tag("from"),
                    intra_expression_whitespace(
                        ExpressionContext::new().allow_newlines_in_expressions(),
                    ),
                )),
            ),
            import_path,
        ))),
        |(consumed, (identifiers, path))| ImportNode {
            value: ImportValue {
                path: path.value(),
                identifiers,
            },
            source: consumed,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn imports_a_single_identifier() {
        let input = "import a from \"file.buri\"";
        let input = ParserInput::new(input);
        let (_, parsed) = import(input).unwrap();
        assert_eq!(parsed.value.identifiers.len(), 1);
        if let ImportedIdentifier::Identifier(node) = &parsed.value.identifiers[0] {
            assert_eq!(node.value.name, "a".to_string());
        } else {
            panic!("Expected identifier");
        }
    }

    #[test]
    fn imports_multiple_identifiers() {
        let input = "import a, b, c from \"file.buri\"";
        let input = ParserInput::new(input);
        let (_, parsed) = import(input).unwrap();
        assert_eq!(parsed.value.identifiers.len(), 3);
        assert!(matches!(
            parsed.value.identifiers[0],
            ImportedIdentifier::Identifier(_)
        ));
        assert!(matches!(
            parsed.value.identifiers[1],
            ImportedIdentifier::Identifier(_)
        ));
        assert!(matches!(
            parsed.value.identifiers[2],
            ImportedIdentifier::Identifier(_)
        ));
    }

    #[test]
    fn imports_single_type_identifier() {
        let input = "import A from \"file.buri\"";
        let input = ParserInput::new(input);
        let (_, parsed) = import(input).unwrap();
        assert_eq!(parsed.value.identifiers.len(), 1);
        if let ImportedIdentifier::TypeIdentifier(node) = &parsed.value.identifiers[0] {
            assert_eq!(node.value, "A".to_string());
        } else {
            panic!("Expected type identifier");
        }
    }

    #[test]
    fn imports_multiple_type_identifiers() {
        let input = "import A, B, C from \"file.buri\"";
        let input = ParserInput::new(input);
        let (_, parsed) = import(input).unwrap();
        assert_eq!(parsed.value.identifiers.len(), 3);
        assert!(matches!(
            parsed.value.identifiers[0],
            ImportedIdentifier::TypeIdentifier(_)
        ));
        assert!(matches!(
            parsed.value.identifiers[1],
            ImportedIdentifier::TypeIdentifier(_)
        ));
        assert!(matches!(
            parsed.value.identifiers[2],
            ImportedIdentifier::TypeIdentifier(_)
        ));
    }

    #[test]
    fn imports_from_provided_file() {
        let input = "import a from \"file.buri\"";
        let input = ParserInput::new(input);
        let (_, parsed) = import(input).unwrap();
        assert_eq!(parsed.value.path, "file.buri");
    }

    #[test]
    fn can_include_newlines_anywhere() {
        let input = "import\na\n,\nb\n,\nc\nfrom\n\"file.buri\"";
        let input = ParserInput::new(input);
        let (remainder, _) = import(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn can_include_comments_anywhere() {
        let input = "import--comment\na--comment\n,\n--comment\nb--comment\n,\n--comment\nc--comment\nfrom--comment\n\"file.buri\"";
        let input = ParserInput::new(input);
        let (remainder, _) = import(input).unwrap();
        assert!(remainder.is_empty());
    }

    #[test]
    fn errors_with_missing_import_keyword() {
        let input = "a from \"file.buri\"";
        let input = ParserInput::new(input);
        let result = import(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_without_identifiers() {
        let input = "import from \"file.buri\"";
        let input = ParserInput::new(input);
        let result = import(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_without_from_keyword() {
        let input = "import a \"file.buri\"";
        let input = ParserInput::new(input);
        let result = import(input);
        assert!(result.is_err());
    }

    #[test]
    fn errors_without_path() {
        let input = "import a from";
        let input = ParserInput::new(input);
        let result = import(input);
        assert!(result.is_err());
    }

    #[test]
    fn no_space_between_import_keyword_and_item_errors() {
        let input = "importa from \"file.buri\"";
        let input = ParserInput::new(input);
        let result = import(input);
        assert!(result.is_err());
    }

    #[test]
    fn no_space_between_item_and_from_keyword_errors() {
        let input = "import afrom \"file.buri\"";
        let input = ParserInput::new(input);
        let result = import(input);
        assert!(result.is_err());
    }

    #[test]
    fn no_space_between_from_keyword_and_file_path_errors() {
        let input = "import a from\"file.buri\"";
        let input = ParserInput::new(input);
        let result = import(input);
        assert!(result.is_err());
    }

    #[test]
    fn path_does_not_allow_ascii_control_characters() {
        // newline (\n) is an ASCII control character
        let input = "import a from\"file.\nburi\"";
        let input = ParserInput::new(input);
        let result = import(input);
        assert!(result.is_err());
    }
}
