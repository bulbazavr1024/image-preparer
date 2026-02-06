use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::error::ProcessingError;
use crate::format::ImageFormat;

/// Collect all supported image files from the input path.
/// If `recursive` is true, walk subdirectories.
pub fn collect_files(input: &Path, recursive: bool) -> Result<Vec<PathBuf>, ProcessingError> {
    if input.is_file() {
        return Ok(vec![input.to_path_buf()]);
    }

    if !input.is_dir() {
        return Err(ProcessingError::ReadFile {
            path: input.to_path_buf(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "not a file or directory"),
        });
    }

    let max_depth = if recursive { usize::MAX } else { 1 };

    let files: Result<Vec<_>, _> = WalkDir::new(input)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|entry| {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => return Some(Err(ProcessingError::from(e))),
            };
            if !entry.file_type().is_file() {
                return None;
            }
            let path = entry.into_path();
            if ImageFormat::from_path(&path).is_some() {
                Some(Ok(path))
            } else {
                None
            }
        })
        .collect();

    files
}

/// Resolve the output path for a given input file.
/// If `output_base` is None, return the input path (overwrite in-place).
/// If `output_base` is a directory, mirror the relative structure.
pub fn resolve_output(
    input_file: &Path,
    input_base: &Path,
    output_base: Option<&Path>,
) -> PathBuf {
    match output_base {
        None => input_file.to_path_buf(),
        Some(out) => {
            if input_base.is_file() {
                // Single file → single output
                if out.extension().is_some() {
                    out.to_path_buf()
                } else {
                    out.join(input_file.file_name().unwrap())
                }
            } else {
                // Directory → mirror structure
                let relative = input_file.strip_prefix(input_base).unwrap_or(input_file.as_ref());
                out.join(relative)
            }
        }
    }
}

/// Create a .bak backup of the file if it exists.
pub fn create_backup(path: &Path) -> Result<(), ProcessingError> {
    if path.exists() {
        let backup = path.with_extension(format!(
            "{}.bak",
            path.extension().unwrap_or_default().to_string_lossy()
        ));
        fs::copy(path, &backup).map_err(|e| ProcessingError::WriteFile {
            path: backup,
            source: e,
        })?;
    }
    Ok(())
}

/// Read file contents.
pub fn read_file(path: &Path) -> Result<Vec<u8>, ProcessingError> {
    fs::read(path).map_err(|e| ProcessingError::ReadFile {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Write file contents, creating parent directories as needed.
pub fn write_file(path: &Path, data: &[u8]) -> Result<(), ProcessingError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| ProcessingError::WriteFile {
            path: parent.to_path_buf(),
            source: e,
        })?;
    }
    fs::write(path, data).map_err(|e| ProcessingError::WriteFile {
        path: path.to_path_buf(),
        source: e,
    })
}
