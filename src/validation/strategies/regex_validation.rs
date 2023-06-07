use crate::ValidationStrategy;
use regex::Regex;

pub struct RegexValidation {
    regex: Regex,
}

impl RegexValidation {
    pub fn new(pattern: &str) -> Self {
        let regex = Regex::new(pattern).unwrap();
        Self { regex }
    }
}

impl ValidationStrategy<String> for RegexValidation {
    fn is_valid(&self, input: &String) -> bool {
        self.regex.is_match(input)
    }
}