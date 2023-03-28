use compiler::compile_buri_file;
use mjolnirjs_spark::get_file_paths;
use std::env;
use std::fs::File;
use std::io::Write;

fn main_impl() -> Result<(), String> {
    let arguments: Vec<String> = env::args().collect();
    let file_paths = get_file_paths(&arguments)?;
    let compiled_output = match std::fs::read_to_string(&file_paths.source) {
        Ok(x) => compile_buri_file(&x)?,
        Err(e) => {
            return Err(format!(
                "Error reading source file {}: {e}",
                file_paths.source
            ))
        }
    };
    let mut output = match File::create(&file_paths.destination) {
        Ok(x) => x,
        Err(e) => {
            return Err(format!(
                "Error creating output file {}: {e}",
                file_paths.destination
            ))
        }
    };
    match write!(output, "{compiled_output}") {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "Error writing to output file {}: {e}",
                file_paths.destination
            ))
        }
    };
    Ok(())
}

fn main() -> Result<(), String> {
    match main_impl() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{e}");
            Err(String::from("Could not compile Buri file"))
        }
    }
}
