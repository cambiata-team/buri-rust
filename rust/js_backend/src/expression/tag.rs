use typed_ast::ConcreteTagExpression;

pub fn print_tag(tag: &ConcreteTagExpression) -> String {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use typed_ast::{ConcreteExpression, ConcreteType};

    #[test]
    fn tags_without_contents_are_arrays_when_some_tags_have_content() {
        let tag = ConcreteTagExpression {
            name: "foo".to_string(),
            expression_type: ConcreteType::default_tag_union_for_test(),
            contents: vec![],
        };
        assert_eq!(print_tag(&tag), "[\"foo\"]");
    }

    #[test]
    fn tags_with_contents_are_arrays_when_some_tags_have_content() {
        let tag = ConcreteTagExpression {
            name: "foo".to_string(),
            expression_type: ConcreteType::default_tag_union_for_test(),
            contents: vec![ConcreteExpression::integer_for_test(42)],
        };
        assert_eq!(print_tag(&tag), "[\"foo\",42]");
    }
}
