use std::cmp::Reverse;

use regex::Regex;

/// Represents a single regex substitution rule with pattern and replacement
///
/// Each rule defines a pattern to match and its corresponding replacement value,
/// where the replacement is automatically wrapped in cookiecutter template syntax.
pub struct SubstitutionRule {
    pub pattern: String,  // The regex pattern to match
    pub replacement: String,  // The value to replace matches with
}

impl SubstitutionRule {
    /// Creates a new substitution rule with the specified pattern and replacement
    pub fn new(pattern: String, replacement: String) -> Self {
        Self {
            pattern,
            replacement: String::from("{{cookiecutter.") + replacement.as_str() + "}}"
        }
    }
}

/// Manages multiple regex substitution rules and performs combined replacements
///
/// Combines multiple patterns into a single regex for efficient matching and
/// applies corresponding replacements based on the matched patterns.
pub struct RegexReplacer {
    combined_regex: Regex,
    rules: Vec<SubstitutionRule>
}

impl RegexReplacer {
    /// Creates a new RegexReplacer from a collection of substitution rules
    pub fn new(mut rules: Vec<SubstitutionRule>) -> Self {
        // Sort rules by pattern length (descending)
        // This ensure that the longest pattern will matches first
        rules.sort_by_key(|r| Reverse(r.pattern.len()));

        // Combine all rules into a single regex with capture groups
        let combined_pattern = rules.iter()
            .map(|rule| format!("({})", rule.pattern.as_str()))
            .collect::<Vec<_>>()
            .join("|");
        let combined_regex = Regex::new(&combined_pattern).expect("Invalid combined regex");
        Self { combined_regex, rules }
    }

    /// Applies all substitution rules to the provided content
    ///
    /// # Arguments
    /// * `content` - Input string to process
    ///
    /// # Returns
    /// * String with all matching patterns replaced according to the rules
    ///
    /// # Behavior
    /// * Matches all patterns simultaneously using a combined regex
    /// * Each pattern match is processed exactly once
    /// * Rules are applied independently and do not cascade:
    ///   - If rule1 replaces "a" with "b" and rule2 replaces "b" with "c",
    ///   - An input "a" will become "b", not "c"
    /// * Non-matching sections remain unchanged
    pub fn replace(&self, content: &str) -> String {
        self.combined_regex.replace_all(content, |caps: &regex::Captures| {
            for (i, rule) in self.rules.iter().enumerate() {
                if let Some(m) = caps.get(i + 1) {
                    // Apply the specific replacement for this pattern
                    return rule.pattern.replace(m.as_str(), &rule.replacement);
                }
            }
            caps[0].to_string()
        }).into_owned()
    }
}