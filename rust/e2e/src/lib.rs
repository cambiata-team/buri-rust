use compiler::compile_buri_file;
use std::fs;
use std::io::Write;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use walkdir::DirEntry;
use walkdir::WalkDir;

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

fn get_test_directory(workspace_directory: &Path) -> Result<PathBuf, String> {
    workspace_directory
        .join("tests/js")
        .canonicalize()
        .map_or(Err(String::from("Cannot find tests directory")), Ok)
}

fn get_valid_directory(path: &Path) -> PathBuf {
    path.join("valid")
}

fn get_invalid_directory(path: &Path) -> PathBuf {
    path.join("invalid")
}

fn get_directories() -> Result<(PathBuf, PathBuf, PathBuf), String> {
    let workspace_directory = get_workspace_directory()?;
    let test_directory = get_test_directory(&workspace_directory)?;
    Ok((
        workspace_directory,
        get_valid_directory(&test_directory),
        get_invalid_directory(&test_directory),
    ))
}

struct FileFailedWithReason {
    file_path: String,
    reason: String,
}

/// Returns a vector of all file paths that could be built (when they
/// shouldn't).
#[allow(clippy::unwrap_used)] // this is essentially a test
fn build_valid_files(
    workspace_directory: &Path,
    directory: &Path,
) -> (usize, Vec<FileFailedWithReason>) {
    let files = WalkDir::new(directory)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|item| {
            item.path()
                .extension()
                .map_or(false, |os_str| os_str.eq_ignore_ascii_case("buri"))
        });
    let mut failed_builds = Vec::new();
    let mut passed_count = 0;
    for file_path in files {
        if let Ok(contents) = std::fs::read_to_string(file_path.path()) {
            match compile_buri_file(&contents) {
                Ok(new_contents) => {
                    let mut output_path =
                        workspace_directory.join(PathBuf::from(".buri/dist").join(PathBuf::from(
                            file_path.path().strip_prefix(workspace_directory).unwrap(),
                        )));
                    output_path.set_extension("mjs");
                    fs::create_dir_all(output_path.parent().unwrap()).unwrap();
                    let mut output = File::create(output_path).unwrap();
                    write!(output, "{new_contents}").unwrap();
                    println!(
                        "PASS: {}",
                        dir_entry_to_string(&file_path, workspace_directory)
                    );
                    passed_count += 1;
                }
                Err(error) => {
                    failed_builds.push(FileFailedWithReason {
                        file_path: dir_entry_to_string(&file_path, workspace_directory),
                        reason: error,
                    });
                }
            }
        } else {
            failed_builds.push(FileFailedWithReason {
                file_path: dir_entry_to_string(&file_path, workspace_directory),
                reason: "E2E: Error loading file".to_owned(),
            });
        };
    }
    (passed_count, failed_builds)
}

#[allow(clippy::unwrap_used)] // this is essentially a test
fn dir_entry_to_string(path: &DirEntry, workspace_directory: &Path) -> String {
    path.path()
        .strip_prefix(workspace_directory)
        .unwrap()
        .as_os_str()
        .to_os_string()
        .into_string()
        .unwrap()
}

/// Returns a vector of all file paths that could be built (when they
/// shouldn't).
fn build_invalid_files(workspace_directory: &Path, directory: &Path) -> (usize, Vec<String>) {
    let files = WalkDir::new(directory)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|item| {
            item.path()
                .extension()
                .map_or(false, |os_str| os_str.eq_ignore_ascii_case("buri"))
        });
    let mut successful_builds = Vec::new();
    let mut passed_count = 0;
    for file_path in files {
        if let Ok(contents) = std::fs::read_to_string(file_path.path()) {
            if compile_buri_file(&contents).is_ok() {
                successful_builds.push(dir_entry_to_string(&file_path, workspace_directory));
            } else {
                println!(
                    "PASS: {}",
                    dir_entry_to_string(&file_path, workspace_directory)
                );
                passed_count += 1;
            }
        } else {
            successful_builds.push(dir_entry_to_string(&file_path, workspace_directory));
        };
    }
    (passed_count, successful_builds)
}

#[allow(clippy::result_unit_err)] // we just care if it passes or errors
pub fn build_tests() -> Result<String, String> {
    let (workspace_directory, valid_directory, invalid_directory) = get_directories()?;

    println!("\nInvalid tests:");
    let (invalid_tests_passed_count, invalid_test_errors) =
        build_invalid_files(&workspace_directory, &invalid_directory);
    println!("\nValid tests:");
    let (valid_tests_passed_count, valid_test_errors) =
        build_valid_files(&workspace_directory, &valid_directory);
    println!();
    for error in &valid_test_errors {
        println!("FAIL: {0} could not be built", error.file_path);
        println!("REASON: {0}", error.reason);
        println!();
    }
    for error in &invalid_test_errors {
        println!("FAIL: {error} unexpectedly built successfully");
        println!();
    }
    if (valid_test_errors.len() + invalid_test_errors.len()) > 0 {
        return Err(format!(
            "{} tests passed, {} tests failed",
            invalid_tests_passed_count + valid_tests_passed_count,
            valid_test_errors.len() + invalid_test_errors.len()
        ));
    }
    Ok(format!(
        "{} tests passed",
        invalid_tests_passed_count + valid_tests_passed_count
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing() {
        assert!(build_tests().is_ok());
    }
}
