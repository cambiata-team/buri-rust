use super::print_expression;
use crate::expression::mangle_variable_name;
use typed_ast::{ConcreteWhenCaseName, ConcreteWhenExpression};

pub fn print_when(when: &ConcreteWhenExpression) -> String {
    let printed_when_condition = print_expression(&when.condition);
    print_when_case(when, &printed_when_condition, 0)
}

fn print_when_case(
    when: &ConcreteWhenExpression,
    printed_when_condition: &String,
    index: usize,
) -> String {
    when.cases.get(index).map_or_else(
        || String::from("0"),
        |case| {
            let mut result = String::new();
            result.push('(');
            match &case.case_name {
                ConcreteWhenCaseName::Name(name) => {
                    result.push_str(&format!(
                        "{}[0]==\"{}\"",
                        printed_when_condition.clone(),
                        name
                    ));
                }
                ConcreteWhenCaseName::DefaultCase => result.push_str("true"),
            };
            result.push('?');
            result.push_str("(()=>{");
            for (index, argument) in case.case_arguments.iter().enumerate() {
                result.push_str(&format!(
                    "let {}={}[{}];",
                    mangle_variable_name(&argument.name),
                    printed_when_condition.clone(),
                    index + 1
                ));
            }
            result.push_str("return ");
            result.push_str(&print_expression(&case.case_expression));
            result.push_str("})()");
            result.push(':');
            result.push_str(&print_when_case(when, printed_when_condition, index + 1));
            result.push(')');
            result
        },
    )
}
