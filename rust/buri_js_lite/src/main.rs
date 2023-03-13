use buri_js_lite::get_file_paths;
use std::env;

fn main() -> Result<(), String> {
    let arguments: Vec<String> = env::args().collect();
    let file_paths = match get_file_paths(&arguments) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    Ok(())
}
