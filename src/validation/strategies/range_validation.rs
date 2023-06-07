use crate::ValidationStrategy;

pub struct RangeValidation {
    min: i32,
    max: i32,
}

impl RangeValidation {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

impl ValidationStrategy<i32> for RangeValidation {
    fn is_valid(&self, input: &i32) -> bool {
        *input >= self.min && *input <= self.max
    }
}