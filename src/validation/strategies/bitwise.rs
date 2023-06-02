

pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    Not,
}

pub struct BitwiseValidation<T> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
    operator: BitwiseOperator,
}

impl<T: 'static> BitwiseValidation<T> {
    pub fn new(operator: BitwiseOperator) -> Self {
        Self {
            strategies: Vec::new(),
            operator,
        }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T>>) {
        self.strategies.push(strategy);
    }

    pub fn validate(&self, input: &T) -> bool {
        match self.operator {
            BitwiseOperator::And => self.strategies.iter().all(|strategy| strategy.is_valid(input)),
            BitwiseOperator::Or => self.strategies.iter().any(|strategy| strategy.is_valid(input)),
            BitwiseOperator::Xor => self.strategies.iter().fold(false, |acc, strategy| acc ^ strategy.is_valid(input)),
            BitwiseOperator::Not => self.strategies.iter().fold(false, |acc, strategy| acc ^ strategy.is_valid(input)),
        }
    }
}