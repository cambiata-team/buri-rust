pub fn mangle_variable_name(name: &str) -> String {
    let mut result = String::from("B");
    result.push_str(name);
    result
}
