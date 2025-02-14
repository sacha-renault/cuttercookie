use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use super::RegexReplacer;
use walkdir::{WalkDir, DirEntry};

const SKIP_ITEMS: [&str;2] = ["", "cuttercookie.json"];

fn process_entry(entry: DirEntry, source_path: &PathBuf, replacer: &RegexReplacer) -> Result<(), String> {
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
    let new_full_path = PathBuf::from(&new_item_rpath);

    // Ensure parent directory exists
    if let Some(parent) = new_full_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

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
        let mut file = File::create(new_item_rpath)
            .map_err(|err| err.to_string())?;
        file.write_all(new_content.as_bytes())
            .map_err(|err| err.to_string())?;
    } else {
        // Create a new directory with th new path
        fs::create_dir(new_item_rpath)
            .map_err(|err| err.to_string())?;
    }

    Ok(())
}

pub fn process_files(path: &str, excluded_dirs: Vec<String>, replacer: RegexReplacer) -> Result<(), String> {
    // Convert input path to PathBuf for easier manipulation
    let source_path = PathBuf::from(path);

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
        process_entry(entry, &source_path, &replacer)?
    }

    // Function processing ended successfully
    Ok(())
}

