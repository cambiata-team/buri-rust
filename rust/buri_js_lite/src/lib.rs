use std::ffi::OsStr;
use std::path::Path;

pub struct CliArguments {
    pub source: String,
    pub destination: String,
}

fn stringify_path(path: &Path) -> Result<String, String> {
    path.to_str().map_or_else(
        || Err(String::from("Invalid Unicode encountered in file path")),
        |x| Ok(x.to_owned()),
    )
}

/// Verify that a string encodes a valid file path with a specific extension.
fn verify_path(path: &Path, required_extension: &str) -> Result<(), String> {
    match path.extension() {
        None => {
            return Err(format!(
                "No extension provided for {}",
                stringify_path(path)?
            ))
        }
        Some(extension) => {
            if extension != OsStr::new(required_extension) {
                return Err(format!(
                    "Invalid extension provided for {}: {}",
                    stringify_path(path)?,
                    match extension.to_str() {
                        Some(x) => x,
                        None =>
                            return Err(
                                "Invalid Unicode encountered when generating another error message"
                                    .to_owned()
                            ),
                    }
                ));
            }
            Ok(())
        }
    }
}

fn derive_destination_path_from_source_path(source_path: &Path) -> Result<String, String> {
    stringify_path(source_path.with_extension("mjs").as_path())
}

pub fn get_file_paths(arguments: &Vec<String>) -> Result<CliArguments, String> {
    if arguments.len() > 3 {
        return Err(String::from("Too many arguments provided"));
    }
    let source_path = match arguments.get(1) {
        Some(source_path) => Path::new(source_path),
        None => return Err(String::from("No source file provided")),
    };
    verify_path(source_path, "buri")?;
    if !source_path.is_file() {
        return Err(format!(
            "Source file {} does not exist",
            stringify_path(source_path)?
        ));
    };
    let derived_destination_path = derive_destination_path_from_source_path(source_path)?;
    let destination_path = arguments.get(2).map_or_else(
        || Path::new(&derived_destination_path),
        |destination_path| Path::new(destination_path),
    );
    verify_path(destination_path, "mjs")?;
    if let Some(parent) = destination_path.parent() {
        if !parent.is_dir() {
            return Err(format!(
                "Destination directory {} does not exist",
                stringify_path(parent)?
            ));
        }
    };
    Ok(CliArguments {
        source: stringify_path(source_path)?,
        destination: stringify_path(destination_path)?,
    })
}
