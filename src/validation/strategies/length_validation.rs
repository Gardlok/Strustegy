use crate::ValidationStrategy;

pub struct LengthValidation {
    min_length: usize,
    max_length: usize,
}

impl LengthValidation {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        Self { min_length, max_length }
    }
}

impl ValidationStrategy<String> for LengthValidation {
    fn is_valid(&self, input: &String) -> bool {
        let length = input.len();
        length >= self.min_length && length <= self.max_length
    }
}