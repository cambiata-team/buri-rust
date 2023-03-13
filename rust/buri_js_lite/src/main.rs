use buri_js_lite::get_file_paths;
use compiler::compile_buri_file;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), String> {
    let arguments: Vec<String> = env::args().collect();
    let file_paths = get_file_paths(&arguments)?;
    let compiled_output = match std::fs::read_to_string(&file_paths.source) {
        Ok(x) => compile_buri_file(&x),
        Err(e) => {
            return Err(format!(
                "Error reading source file {}: {}",
                file_paths.source, e
            ))
        }
    }?;
    let mut output = match File::create(&file_paths.destination) {
        Ok(x) => x,
        Err(e) => {
            return Err(format!(
                "Error creating output file {}: {}",
                file_paths.destination, e
            ))
        }
    };
    match write!(output, "{compiled_output}") {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "Error writing to output file {}: {}",
                file_paths.destination, e
            ))
        }
    };
    Ok(())
}
