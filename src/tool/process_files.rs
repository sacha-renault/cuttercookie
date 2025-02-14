use std::fs::{self, FileType};
use std::path::PathBuf;

use super::file::File;
use super::RegexReplacer;
use walkdir::WalkDir;

fn process_file(file: File, replacer: &RegexReplacer) -> File {
    File::new(replacer.replace(&file.name()), replacer.replace(&file.content()))
}

pub fn process_files(path: &str, excluded_dirs: Vec<String>, replacer: RegexReplacer) -> Result<(), String> {
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
        if entry.file_type().is_file() {
            let file = File::new(
                entry.file_name()
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in filename")).map_err(|err| err.to_string())?
                    .to_string(),
                fs::read_to_string(entry.path()).map_err(|err| err.to_string())?
            );
            let new_file = process_file(file, &replacer);
        } else {
            let dirname = entry.file_name()
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in filename")).map_err(|err| err.to_string())?
                .to_string();
            let new_dirname = replacer.replace(&dirname);
        }

    }
    Ok(())
}

