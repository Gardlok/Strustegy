
pub struct ValidationConfigBuilder {
    strategies: Vec<Box<dyn ValidationStrategy>>,
}

impl ValidationConfigBuilder {
    pub fn new() -> Self {
        ValidationConfigBuilder {
            strategies: Vec::new(),
        }
    }

    pub fn with_strategy(mut self, strategy: Box<dyn ValidationStrategy>) -> Self {
        self.strategies.push(strategy);
        self
    }

    pub fn build(self) -> ValidationConfig {
        ValidationConfig {
            strategies: self.strategies,
        }
    }
}

pub struct ValidationConfig<T> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T> ValidationConfig<T> {
    pub fn new() -> Self {
        Self { strategies: Vec::new() }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T>>) {
        self.strategies.push(strategy);
    }

    pub fn validate(&self, input: &T) -> bool {
        self.strategies.iter().all(|strategy| strategy.validate(input))
    }
}