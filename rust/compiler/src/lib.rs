use js_backend::print_js_document;
use parser::parse_buri_file;
use type_checker::{apply_constraints, resolve_concrete_types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationError {
    ParseError(String),
    TypeError(()),
    TypeResolutionError(()),
}

/// Compiles a single Buri file. Do not use to compile Buri programs with
/// multiple files.
///
/// This function accepts the string contents of the Buri file, then returns
/// the compiled JS output (or an error if the input is invalid). The caller
/// must read the Buri file itself—this function does not do that.
pub fn compile_buri_file(contents: &str) -> Result<String, CompilationError> {
    let parsed_ast = match parse_buri_file(contents) {
        Ok(ast) => ast,
        Err(error) => return Err(CompilationError::ParseError(error)),
    };
    let generic_document = match apply_constraints(parsed_ast) {
        Ok(document) => document,
        Err(error) => return Err(CompilationError::TypeError(error)),
    };
    let concrete_document = match resolve_concrete_types(generic_document) {
        Ok(document) => document,
        Err(error) => return Err(CompilationError::TypeResolutionError(error)),
    };
    Ok(print_js_document(&concrete_document))
}
