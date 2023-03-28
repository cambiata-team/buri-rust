use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

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
        None => Err(format!(
            "No extension provided for {}",
            stringify_path(path)?
        )),
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

fn get_workspace_directory() -> Result<PathBuf, String> {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../"))
        .canonicalize()
        .map_or(
            Err(String::from(
                "Cannot CARGO_MANIFEST_DIR environment variable",
            )),
            Ok,
        )
}

fn derive_destination_path_from_source_path(source_path: &Path) -> Result<String, String> {
    let workspace_directory = get_workspace_directory()?;
    let output_path =
        &mut workspace_directory.join(PathBuf::from(".buri/dist").join(PathBuf::from(source_path)));
    stringify_path(output_path.with_extension("mjs").as_path())
}

fn create_destination_directory(destination_path: &Path) -> Result<(), String> {
    match destination_path.parent() {
        None => Ok(()),
        Some(parent_path) => match create_dir_all(parent_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Error creating destination directory {}: {}",
                stringify_path(parent_path)?,
                e
            )),
        },
    }
}

pub fn get_file_paths(arguments: &Vec<String>) -> Result<CliArguments, String> {
    if arguments.len() > 2 {
        return Err(String::from("Too many arguments provided"));
    }
    let source_path = match arguments.get(1) {
        Some(source_path) => Path::new(source_path),
        None => return Err(String::from("No source file provided")),
    };
    verify_path(source_path, "buri")?;
    let derived_destination_path = derive_destination_path_from_source_path(source_path)?;
    let destination_path = Path::new(&derived_destination_path);
    create_destination_directory(destination_path)?;
    Ok(CliArguments {
        source: stringify_path(source_path)?,
        destination: stringify_path(destination_path)?,
    })
}
