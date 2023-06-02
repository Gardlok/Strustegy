



pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,    
}

pub struct ComparisonValidation<T> {
    value: T,
    operator: ComparisonOperator,
}

impl<T: PartialOrd> ComparisonValidation<T> {
    pub fn new(value: T, operator: ComparisonOperator) -> Self {
        Self {
            value,
            operator,
        }
    }

    pub fn validate(&self, input: &T) -> bool {
        match self.operator {
            ComparisonOperator::Equal => input == &self.value,
            ComparisonOperator::NotEqual => input != &self.value,
            ComparisonOperator::GreaterThan => input > &self.value,
            ComparisonOperator::GreaterThanOrEqual => input >= &self.value,
            ComparisonOperator::LessThan => input < &self.value,
            ComparisonOperator::LessThanOrEqual => input <= &self.value,
        }
    }
}