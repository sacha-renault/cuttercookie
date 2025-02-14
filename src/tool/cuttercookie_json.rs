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

/// Reads and parses substitution rules from a JSON file at the specified path
///
/// # Arguments
/// * `path` - File path to the JSON configuration file
///
/// # Returns
/// * `RegexReplacer` - A RegexReplacer configured with the rules from the file
///
/// # Panics
/// * Panics if the file cannot be read
/// * Panics if the JSON content cannot be parsed into valid substitution rules
///
/// # Expected JSON Format
/// ```json
/// {
///     "pattern1": "replacement1",
///     "pattern2": "replacement2"
/// }
/// ```
pub fn read_json_pairs(path: &str) -> RegexReplacer {
    let json_str = match fs::read_to_string(path) {
        Ok(json) => json,
        Err(_) => panic!("Couldn't read \"cuttercookie.json\" at the root path")
    };
    parse_json_pairs(&json_str).expect("Couldn't parse the \"cuttercookie.json\" at the root path")
}