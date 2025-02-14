use regex::Regex;

pub struct SubstitutionRule {
    pub pattern: String,  // The regex pattern to match
    pub replacement: String,  // The value to replace matches with
}

impl SubstitutionRule {
    pub fn new(pattern: String, replacement: String) -> Self {
        Self {
            pattern,
            replacement
        }
    }
}

pub struct RegexReplacer {
    combined_regex: Regex,
    rules: Vec<SubstitutionRule>
}

impl RegexReplacer {
    pub fn new(rules: Vec<SubstitutionRule>) -> Self {
        // Combine all rules into a single regex with capture groups
        let combined_pattern = rules.iter()
            .map(|rule| format!("({})", rule.pattern.as_str()))
            .collect::<Vec<_>>()
            .join("|");
        let combined_regex = Regex::new(&combined_pattern).expect("Invalid combined regex");
        Self { combined_regex, rules }
    }

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