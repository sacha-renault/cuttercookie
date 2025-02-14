use super::file::File;
use super::RegexReplacer;

pub fn process_file(file: File, replacer: &RegexReplacer) -> File {
    File::new(replacer.replace(&file.name()), replacer.replace(&file.content()))
}