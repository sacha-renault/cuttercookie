use std::path::PathBuf;
use std::fs;

use serde_json::Value;
use anyhow::Result;

fn parse_json_pairs(json_str: &str) -> Result<Vec<(String, String)>> {
    let parsed: Value = serde_json::from_str(json_str)?;

    match parsed {
        Value::Object(map) => {
            Ok(map
                .into_iter()
                .filter_map(|(k, v)| {
                    v.as_str().map(|s| (k, s.to_string()))
                })
                .collect())
        },
        _ => Err(anyhow::anyhow!("Expected JSON object"))
    }
}

pub fn read_json_pairs(path: &str) -> Vec<(String, String)> {
    let json_str = match fs::read_to_string(path) {
        Ok(json) => json,
        Err(_) => panic!("Couldn't read \"cuttercookie.json\" at the root path")
    };
    parse_json_pairs(&json_str).expect("Couldn't parse the \"cuttercookie.json\" at the root path")
}