use compiler::compile_buri_file;
use std::io::Write;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

fn get_test_directory() -> Result<PathBuf, ()> {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/js"))
        .canonicalize()
        .map_or(Err(()), Ok)
}

fn get_valid_directory(path: &Path) -> PathBuf {
    path.join("valid")
}

fn get_invalid_directory(path: &Path) -> PathBuf {
    path.join("invalid")
}

fn get_directories() -> Result<(PathBuf, PathBuf), ()> {
    let test_directory = get_test_directory()?;
    Ok((
        get_valid_directory(&test_directory),
        get_invalid_directory(&test_directory),
    ))
}

/// Returns a vector of all file paths that could be built (when they
/// shouldn't).
#[allow(clippy::unwrap_used)] // this is essentially a test
fn build_valid_files(directory: &Path) -> Vec<String> {
    let files = WalkDir::new(directory)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|item| {
            item.path()
                .extension()
                .map_or(false, |os_str| os_str.eq_ignore_ascii_case("buri"))
        });
    let mut failed_builds = Vec::new();
    for file_path in files {
        if let Ok(contents) = std::fs::read_to_string(file_path.path()) {
            compile_buri_file(&contents).map_or_else(
                |_| failed_builds.push(format!("{file_path:?}")),
                |new_contents| {
                    let mut output_path =
                        PathBuf::from(".buri/dist").join(PathBuf::from(file_path.path()));
                    output_path.set_extension("mjs");
                    let mut output = File::create(output_path).unwrap();
                    write!(output, "{new_contents}").unwrap();
                    println!("PASS: {file_path:?} built as expected");
                },
            );
        } else {
            failed_builds.push(format!("{file_path:?}"));
        };
    }
    failed_builds
}

/// Returns a vector of all file paths that could be built (when they
/// shouldn't).
fn build_invalid_files(directory: &Path) -> Vec<String> {
    let files = WalkDir::new(directory)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|item| {
            item.path()
                .extension()
                .map_or(false, |os_str| os_str.eq_ignore_ascii_case("buri"))
        });
    let mut successful_builds = Vec::new();
    for file_path in files {
        if let Ok(contents) = std::fs::read_to_string(file_path.path()) {
            match compile_buri_file(&contents) {
                Ok(_) => successful_builds.push(format!("{file_path:?}")),
                Err(_) => println!("PASS: {file_path:?} failed to build as expected"),
            };
        } else {
            successful_builds.push(format!("{file_path:?}"));
        };
    }
    successful_builds
}

#[allow(clippy::result_unit_err)] // we just care if it passes or errors
pub fn build_tests() -> Result<(), ()> {
    let (valid_directory, invalid_directory) = get_directories()?;

    let invalid_test_errors = build_invalid_files(&invalid_directory);
    let valid_test_errors = build_valid_files(&valid_directory);
    println!("\n\n");
    for error in valid_test_errors {
        println!("FAIL: {error} could not be built");
    }
    println!("\n");
    for error in invalid_test_errors {
        println!("FAIL: {error} unexpectedly built successfully");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO(B-220): remove this ignore statement to ensure the e2e tests build.
    #[test]
    #[ignore]
    fn testing() {
        assert!(build_tests().is_ok());
    }
}
