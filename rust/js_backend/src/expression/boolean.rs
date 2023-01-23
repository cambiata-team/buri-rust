use typed_ast::ConcreteBooleanExpression;

pub fn print_boolean(boolean: &ConcreteBooleanExpression) -> String {
    boolean.value.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use typed_ast::{ConcreteBooleanExpression, ConcreteType, PrimitiveType};

    #[test]
    fn print_true() {
        let boolean = ConcreteBooleanExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::CompilerBoolean),
            value: true,
        };
        assert_eq!(print_boolean(&boolean), "true");
    }

    #[test]
    fn print_false() {
        let boolean = ConcreteBooleanExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::CompilerBoolean),
            value: false,
        };
        assert_eq!(print_boolean(&boolean), "false");
    }
}
