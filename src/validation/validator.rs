
use crate::validation::error::ValidationError;
use crate::validation::{StrategyMap, ValidationStrategy, StrategyContext, Context};
use crate::Validity;





pub struct Validator<T> where T: 'static + Send + Sync + Clone {
    strategies: StrategyMap<T>,
    context: Context<StrategyContext>,
}

impl<T: 'static> Validator<T> where T: 'static + Send + Sync + Clone{
    pub fn new() -> Self {
        Validator {
            strategies: StrategyMap::<T>::new(),
            context: Context::<StrategyContext>::new(1), 
        }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T> + 'static>) {
        self.strategies.insert_strategy(strategy);
    }

    pub fn add_strategies(&mut self, strategies: Vec<Box<dyn ValidationStrategy<T> + 'static>>) {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    pub fn remove_strategy(&mut self, strategy: &dyn ValidationStrategy<T>) -> Result<(), ValidationError> {
        let strategy_type_id = strategy.type_id();
        self.strategies.remove_strategy(strategy_type_id);
        Ok(())
    }

    pub fn validate<'a>(&'a self, data: &'a T ) -> Validity<T> {
        self.strategies.validate(&data)

    }



}

impl<T: 'static> Clone for Validator<T> where T: 'static + Send + Sync + Clone {
    fn clone(&self) -> Self {
        let strategies = self.strategies.clone();
        let context: Context<StrategyContext> = Context::<StrategyContext>::new(self.context.len());
        
        Validator { strategies, context }
    }
}
