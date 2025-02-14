use std::fs;

use serde_json::Value;
use anyhow::Result;

use super::{SubstitutionRule, RegexReplacer};

/// Parses a JSON string into a RegexReplacer containing substitution rules
///
/// # Arguments
/// * `json_str` - JSON string containing pattern-replacement pairs where:
///   - Keys are regex patterns
///   - Values are replacement strings
///
/// # Returns
/// * `Result<RegexReplacer>` - A RegexReplacer configured with the parsed rules
///
/// # Errors
/// * Returns error if input is not a valid JSON object
/// * Returns error if any value in the JSON is not a string
fn parse_json_pairs(json_str: &str) -> Result<RegexReplacer> {
    let parsed: Value = serde_json::from_str(json_str)?;

    match parsed {
        Value::Object(map) => {
            let subs = map
                .into_iter()
                .map(|(pattern, value)| {
                    let replacement = value.as_str()
                        .ok_or_else(|| anyhow::anyhow!("Value must be a string"))?
                        .to_string();
                    Ok(SubstitutionRule::new(pattern, replacement))
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(RegexReplacer::new(subs))
        },

        _ => Err(anyhow::anyhow!("Expected JSON object"))
    }
}

/// Reads and parses substitution rules from a JSON file, creating a configured RegexReplacer
///
/// # Arguments
/// * `path` - Path to the JSON configuration file containing pattern-replacement pairs
///
/// # Returns
/// * `Result<RegexReplacer>` - Successfully configured RegexReplacer, or an error if:
///   - File cannot be read from the specified path
///   - JSON content is malformed or invalid
///   - Pattern-replacement pairs cannot be parsed
///
/// # JSON File Structure
/// The configuration file must contain key-value pairs where:
/// - Keys are string patterns to match
/// - Values are their corresponding replacement strings
pub fn read_json_pairs(path: &str) -> Result<RegexReplacer> {
    let json_str = match fs::read_to_string(path) {
        Ok(json) => json,
        Err(_) => return Err(anyhow::anyhow!("Couldn't read \"cuttercookie.json\" at the root path"))
    };
    Ok(parse_json_pairs(&json_str)?)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Creates a temporary JSON file with given content for testing
    ///
    /// # Arguments
    /// * `content` - JSON content to write to the temporary file
    ///
    /// # Returns
    /// * `Result<NamedTempFile>` - Temporary file handle or error if file creation fails
    fn create_temp_json_file(content: &str) -> Result<NamedTempFile> {
        let mut temp_file = NamedTempFile::new()?;
        write!(temp_file, "{}", content)?;
        Ok(temp_file)
    }

    #[test]
    fn test_parse_valid_json() -> Result<()> {
        let json_str = r#"{
            "hello world": "greeting",
            "test": "number"
        }"#;

        let replacer = parse_json_pairs(json_str)?;

        // Test the replacer's functionality
        assert_eq!(replacer.replace("hello world"), "{{cookiecutter.greeting}}");
        assert_eq!(replacer.replace("test.123"), "{{cookiecutter.number}}.123");
        assert_eq!(replacer.replace("unmatched"), "unmatched");
        Ok(())
    }

    #[test]
    fn test_parse_empty_json() -> Result<()> {
        let json_str = "{}";
        let replacer = parse_json_pairs(json_str)?;

        // Empty replacer should return input unchanged
        assert_eq!(replacer.replace("test"), "test");
        assert_eq!(replacer.replace("hello world"), "hello world");
        Ok(())
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_jsons = [
            r#"{"pattern": "replacement", invalid_json}"#,
            r#"["pattern", "replacement"]"#,
            r#"{"pattern": 42}"#,
            r#""just a string""#
        ];

        for json in invalid_jsons {
            assert!(parse_json_pairs(json).is_err());
        }
    }

    #[test]
    fn test_read_valid_json_file() -> Result<()> {
        let json_content = r#"{
            "hello world": "greeting",
            "test": "number"
        }"#;

        let temp_file = create_temp_json_file(json_content)?;
        let replacer = read_json_pairs(temp_file.path().to_str().unwrap())
            .expect("Failed to read valid JSON file");

        assert_eq!(replacer.replace("hello world"), "{{cookiecutter.greeting}}");
        assert_eq!(replacer.replace("test.123"), "{{cookiecutter.number}}.123");
        Ok(())
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_json_pairs("nonexistent_file.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_invalid_json_file() -> Result<()> {
        let invalid_json = r#"{
            "pattern1": "replacement1",
            invalid_content
        }"#;

        let temp_file = create_temp_json_file(invalid_json)?;
        let result = read_json_pairs(temp_file.path().to_str().unwrap());

        assert!(result.is_err());
        Ok(())
    }
}