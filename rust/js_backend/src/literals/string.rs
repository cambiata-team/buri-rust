use typed_ast::ConcreteStringLiteralExpression;

static HEX_VALUE_TO_HEX_DIGIT: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn print_string_literal(node: &ConcreteStringLiteralExpression) -> String {
    let mut result = String::new();
    result.push('\"');
    for character in node.value.chars() {
        match character {
            '\x08' => {
                result.push_str("\\b");
            }
            '\t' => {
                result.push_str("\\t");
            }
            '\n' => {
                result.push_str("\\n");
            }
            '\x0B' => {
                result.push_str("\\v");
            }
            '\x0C' => {
                result.push_str("\\f");
            }
            '\r' => {
                result.push_str("\\r");
            }
            '\"' => {
                result.push_str("\\\"");
            }
            '\'' => {
                result.push_str("\\\'");
            }
            '\\' => {
                result.push_str("\\\\");
            }
            '\0'..='\x1F' | '\x7F' => {
                let unicode_codepoint = character as usize;
                result.push_str("\\x");
                result.push(HEX_VALUE_TO_HEX_DIGIT[unicode_codepoint >> 4]);
                result.push(HEX_VALUE_TO_HEX_DIGIT[unicode_codepoint & 0xF]);
            }
            _ => {
                result.push(character);
            }
        }
    }
    result.push('\"');
    result
}

#[cfg(test)]
mod test {
    use super::*;

    use typed_ast::{ConcreteType, PrimitiveType};

    #[test]
    fn simple_string_literal() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "hello".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"hello\"");
    }

    #[test]
    fn newline_is_escaped() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "\n".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"\\n\"");
    }

    #[test]
    fn non_graphic_ascii_is_escaped() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "\x1F".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"\\x1f\"");
    }

    #[test]
    fn null_is_escaped_with_hex_sequence() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "\0".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"\\x00\"");
    }

    #[test]
    fn non_ascii_character_is_not_escaped() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "π".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"π\"");
    }
}
