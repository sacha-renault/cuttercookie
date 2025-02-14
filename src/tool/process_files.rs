use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use super::RegexReplacer;
use walkdir::{WalkDir, DirEntry};

/// Files and directories that should be skipped during processing
const SKIP_ITEMS: [&str;2] = ["", "cuttercookie.json"];

/// Processes a single directory entry by applying regex replacements to its path and content
///
/// # Arguments
/// * `entry` - Directory entry to process
/// * `source_path` - Base path for calculating relative paths
/// * `dest_path` - Destination directory where processed files will be written
/// * `replacer` - RegexReplacer instance containing replacement rules
///
/// # Returns
/// * `Result<(), String>` - Success or error message
fn process_entry(entry: DirEntry, source_path: &PathBuf, dest_path: & PathBuf, replacer: &RegexReplacer) -> Result<(), String> {
    // Get the relative path
    let item_rpath = entry.path()
        .strip_prefix(source_path)
        .map_err(|e| e.to_string())?;

    // Convert to string and apply replacements
    let item_rpath_string = item_rpath
        .to_str()
        .ok_or("Invalid UTF-8 in filename")?
        .to_string();
    let new_item_rpath = replacer.replace(&item_rpath_string);

    // Create path relative to current directory
    let new_dest_path  = dest_path.join(&new_item_rpath);

    // We don't recreate a root i guess ?
    if SKIP_ITEMS.contains(&new_item_rpath.as_str()) {
        return Ok(()) // End the loop iteration instantly
    }

    if entry.file_type().is_file() {
        // Get content of the file
        let content = fs::read_to_string(entry.path()).map_err(|err| err.to_string())?;

        // Replacement in the content
        let new_content = replacer.replace(&content);

        // Create a new file at the correct path with new content
        let mut file = File::create(&new_dest_path)
            .map_err(|err| err.to_string())?;
        file.write_all(new_content.as_bytes())
            .map_err(|err| err.to_string())?;
    } else {
        // Create a new directory with th new path
        fs::create_dir(&new_dest_path)
            .map_err(|err: std::io::Error| err.to_string())?;
    }

    Ok(())
}

/// Recursively processes files and directories, applying regex replacements
///
/// # Arguments
/// * `path` - Root path to start processing from
/// * `destination` - Target directory where processed files will be written
/// * `excluded_dirs` - List of directory names to exclude from processing
/// * `replacer` - RegexReplacer containing the replacement rules
///
/// # Returns
/// * `Result<(), String>` - Success or error message
///
/// # Errors
/// * Returns error if file operations fail (read/write/create)
/// * Returns error if path manipulation fails
/// * Returns error if UTF-8 conversion fails
pub fn process_files(path: &str, destination: &str, excluded_dirs: Vec<String>, replacer: RegexReplacer) -> Result<(), String> {
    // Convert input path to PathBuf for easier manipulation
    let source_path = PathBuf::from(path);
    let dest_path = PathBuf::from(destination);

    // Walk recursively into the directory
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            !excluded_dirs.iter().any(|dir| {
                e.path()
                    .components()
                    .any(|c| c.as_os_str().to_string_lossy() == *dir)
            })
        })
    {
        process_entry(entry, &source_path, &dest_path, &replacer)?
    }

    // Function processing ended successfully
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::SubstitutionRule;
    use std::fs;
    use tempfile::TempDir;
    use std::collections::HashMap;

    /// Test helper to create a temporary directory structure
    ///
    /// # Arguments
    /// * `files` - Map of file paths to their content
    ///
    /// # Returns
    /// * Tuple of (TempDir, PathBuf) containing the temp directory and its path
    fn setup_test_directory(files: HashMap<&str, &str>) -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_path_buf();

        for (file_path, content) in files {
            let full_path = temp_path.join(file_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directory");
            }
            fs::write(full_path, content).expect("Failed to write test file");
        }

        (temp_dir, temp_path)
    }

    /// Creates a simple RegexReplacer for testing
    ///
    /// # Returns
    /// * RegexReplacer configured with test patterns
    fn create_test_replacer() -> RegexReplacer {
        let mut replacements = Vec::new();
        replacements.push(SubstitutionRule::new("value".to_string(), "value_placeholder".to_string()));
        replacements.push(SubstitutionRule::new("filename".to_string(), "filename_placeholder".to_string()));
        replacements.push(SubstitutionRule::new("old".to_string(), "new".to_string()));
        RegexReplacer::new(replacements)
    }

    /// Tests basic file content replacement
    #[test]
    fn test_basic_file_processing() {
        let mut files = HashMap::new();
        files.insert("filename.txt", "x = value");

        let (_temp_dir, temp_path) = setup_test_directory(files);
        let replacer = create_test_replacer();

        process_files(
            temp_path.to_str().unwrap(),
            temp_path.to_str().unwrap(),
            vec![],
            replacer
        ).expect("Processing should succeed");

        let new_filepath_expected = temp_path.join("{{cookiecutter.filename_placeholder}}.txt");
        assert!(new_filepath_expected.exists());
        let processed_content = fs::read_to_string(new_filepath_expected)
            .expect("Should read processed file");
        assert_eq!(processed_content, "x = {{cookiecutter.value_placeholder}}");
    }

    /// Tests directory structure preservation and path replacements
    #[test]
    fn test_directory_structure() {
        let mut files = HashMap::new();
        files.insert("old_dir/filename.txt", "content");
        files.insert("old_dir/nested/filename.txt", "nested content");

        let (_temp_dir, temp_path) = setup_test_directory(files);
        let (_target_dir, target_path) = setup_test_directory(HashMap::new());
        let replacer = create_test_replacer();

        process_files(
            temp_path.to_str().unwrap(),
            target_path.to_str().unwrap(),
            vec![],
            replacer
        ).expect("Processing should succeed");

        assert!(target_path.join("{{cookiecutter.new}}_dir").exists());
        assert!(target_path.join("{{cookiecutter.new}}_dir/{{cookiecutter.filename_placeholder}}.txt").exists());
        assert!(target_path.join("{{cookiecutter.new}}_dir/nested/{{cookiecutter.filename_placeholder}}.txt").exists());
    }

    /// Tests directory exclusion functionality
    #[test]
    fn test_excluded_directories() {
        let mut files = HashMap::new();
        files.insert("filename.txt", "content");
        files.insert("excluded/test.txt", "should not process");

        let (_temp_dir, temp_path) = setup_test_directory(files);
        let (_temp_dir, dest_path) = setup_test_directory(HashMap::new());
        let replacer = create_test_replacer();

        process_files(
            temp_path.to_str().unwrap(),
            dest_path.to_str().unwrap(),
            vec!["excluded".to_string()],
            replacer
        ).expect("Processing should succeed");

        assert!(dest_path.join("{{cookiecutter.filename_placeholder}}.txt").exists());
        assert!(!dest_path.join("excluded/{{cookiecutter.filename_placeholder}}.txt").exists());
    }

    /// Tests handling of special items defined in SKIP_ITEMS
    #[test]
    fn test_skip_items() {
        let mut files = HashMap::new();
        files.insert("cuttercookie.json", "{ \"config\": true }");
        files.insert("test.txt", "content");

        let (_temp_dir, temp_path) = setup_test_directory(files);
        let (_temp_dir, dest_path) = setup_test_directory(HashMap::new());
        let replacer = create_test_replacer();

        process_files(
            temp_path.to_str().unwrap(),
            dest_path.to_str().unwrap(),
            vec![],
            replacer
        ).expect("Processing should succeed");

        assert!(temp_path.join("cuttercookie.json").exists());
        assert!(!dest_path.join("cuttercookie.json").exists());
        assert_eq!(
            fs::read_to_string(temp_path.join("cuttercookie.json")).unwrap(),
            "{ \"config\": true }"
        );
    }
}