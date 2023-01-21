use concrete_ast::ConcreteTagExpression;

pub fn print_tag(tag: &ConcreteTagExpression) -> String {
    if tag.concrete_type.some_tags_have_content {
        let mut output = String::new();
        output.push_str("[\"");
        output.push_str(&tag.name);
        output.push('"');
        if !tag.contents.is_empty() {
            let contents = tag
                .contents
                .iter()
                .map(super::print_expression)
                .collect::<Vec<_>>()
                .join(", ");
            output.push(',');
            output.push_str(&contents);
        }
        output.push(']');
        output
    } else {
        format!("\"{}\"", tag.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use concrete_ast::{ConcreteExpression, ConcreteIntegerLiteralExpression};

    #[test]
    fn tags_in_tag_unions_without_contents_are_strings() {
        let tag = ConcreteTagExpression {
            name: "foo".to_string(),
            concrete_type: concrete_ast::ConcreteTagUnionType {
                some_tags_have_content: false,
                ..Default::default()
            },
            contents: vec![],
        };
        assert_eq!(print_tag(&tag), "\"foo\"");
    }

    #[test]
    fn tags_without_contents_are_arrays_when_some_tags_have_content() {
        let tag = ConcreteTagExpression {
            name: "foo".to_string(),
            concrete_type: concrete_ast::ConcreteTagUnionType {
                some_tags_have_content: true,
                ..Default::default()
            },
            contents: vec![],
        };
        assert_eq!(print_tag(&tag), "[\"foo\"]");
    }

    #[test]
    fn tags_with_contents_are_arrays_when_some_tags_have_content() {
        let tag = ConcreteTagExpression {
            name: "foo".to_string(),
            concrete_type: concrete_ast::ConcreteTagUnionType {
                some_tags_have_content: true,
                ..Default::default()
            },
            contents: vec![ConcreteExpression::Integer(Box::new(
                ConcreteIntegerLiteralExpression { value: 42 },
            ))],
        };
        assert_eq!(print_tag(&tag), "[\"foo\",42]");
    }
}
