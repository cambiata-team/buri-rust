use js_backend::print_js_document;
use parser::parse_buri_file;
use type_checker_resolver::resolve_concrete_types;
use type_checker_translator::apply_constraints;

/// Compiles a single Buri file. Do not use to compile Buri programs with
/// multiple files.
///
/// This function accepts the string contents of the Buri file, then returns
/// the compiled JS output (or an error if the input is invalid). The caller
/// must read the Buri file itself—this function does not do that.
pub fn compile_buri_file(contents: &str) -> Result<String, String> {
    let parsed_ast = match parse_buri_file(contents) {
        Ok(ast) => ast,
        Err(error) => {
            let mut message = "Parsing Error: ".to_owned();
            message.push_str(error.as_str());
            return Err(message);
        }
    };
    let (generic_document, type_schema) = apply_constraints(parsed_ast)?;
    let concrete_document = resolve_concrete_types(type_schema, generic_document);
    Ok(print_js_document(&concrete_document))
}
