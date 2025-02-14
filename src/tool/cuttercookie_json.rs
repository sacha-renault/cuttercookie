use std::fs;

use serde_json::Value;
use anyhow::Result;

use super::{SubstitutionRule, RegexReplacer};

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

pub fn read_json_pairs(path: &str) -> RegexReplacer {
    let json_str = match fs::read_to_string(path) {
        Ok(json) => json,
        Err(_) => panic!("Couldn't read \"cuttercookie.json\" at the root path")
    };
    parse_json_pairs(&json_str).expect("Couldn't parse the \"cuttercookie.json\" at the root path")
}