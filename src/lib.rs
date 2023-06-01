


use std::any::Any;
use std::marker::PhantomData;


pub struct ComboValidationStrategy<T: 'static> {
    static_strategies: Vec<Box<dyn ValidationStrategy<T>>>,
    dynamic_strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ComboValidationStrategy<T> {
    // Creates a new ComboValidationStrategy with the given static and dynamic strategies to be used
    // when validating input. The static strategies are executed first, followed by the dynamic
    // strategies. If any of the static strategies fail, the input is considered invalid and the
    // dynamic strategies are not executed. If any of the dynamic strategies fail, the input is 
    // considered invalid. If all of the static and dynamic strategies pass, the input is considered
    // valid. 
    pub fn new(static_strategies: Vec<Box<dyn ValidationStrategy<T>>>, dynamic_strategies: Vec<Box<dyn ValidationStrategy<T>>>) -> Self {
        ComboValidationStrategy {
            static_strategies,
            dynamic_strategies,
        }
    }
}

impl<T: 'static> ValidationStrategy<T> for ComboValidationStrategy<T> {
    fn is_valid(&self, input: &T) -> bool {
        // All static strategies must pass
        if !self.static_strategies.iter().all(|strategy| strategy.is_valid(input)) {
            return false;
        }

        // All dynamic strategies must pass
        if !self.dynamic_strategies.iter().all(|strategy| strategy.is_valid(input)) {
            return false;
        }

        // If all static and dynamic strategies pass, the input is valid
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NestedValidationStrategy {
    nested_validation: Validation<i32>,
}

impl ValidationStrategy<i32> for NestedValidationStrategy {
    fn is_valid(&self, data: &i32) -> bool {
        self.nested_validation.is_valid(data)
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}


pub struct ClosureValidationStrategy<T> {
    validation_fn: Box<dyn Fn(&T) -> bool>,
    _phantom: PhantomData<T>,
}

impl<T> ClosureValidationStrategy<T> {
    pub fn new(validation_fn: Box<dyn Fn(&T) -> bool>) -> Self {
        ClosureValidationStrategy {
            validation_fn,
            _phantom: PhantomData,
        }
    }
}

impl<T: 'static> ValidationStrategy<T> for ClosureValidationStrategy<T> {
    fn is_valid(&self, value: &T) -> bool {
        (self.validation_fn)(value)
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}


pub struct CustomValidationStrategy<T: 'static, F: Fn(&T) -> bool + 'static>(
    F,
    PhantomData<T>,
);

impl<T: 'static, F: Fn(&T) -> bool + 'static> CustomValidationStrategy<T, F> {
    pub fn new(strategy: F) -> Self {
        CustomValidationStrategy(strategy, PhantomData)
    }
}

impl<T: 'static, F> ValidationStrategy<T> for CustomValidationStrategy<T, F>
where
    F: Fn(&T) -> bool + 'static,
{
    fn is_valid(&self, input: &T) -> bool {
        (self.0)(input)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}



pub struct LengthValidation;

impl ValidationStrategy<String> for LengthValidation {
    fn is_valid(&self, data: &String) -> bool {
        data.len() > 5
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NumberValidation;

impl ValidationStrategy<i32> for NumberValidation {
    fn is_valid(&self, data: &i32) -> bool  {
        *data > 5
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait ValidationStrategy<T: 'static> {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub struct Validation<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
    children: Vec<Validation<T>>,
}

impl<T: 'static> Validation<T> {


    // Creates a new Validation with no strategies or children
    pub fn new() -> Self {
        Validation {
            strategies: Vec::new(),
            children: Vec::new(),
        }
    }


    // Add a strategy to the validation. The strategy will be executed when the validation is
    // performed. 
    pub fn add_strategy<S>(&mut self, strategy: S)
    where
        S: ValidationStrategy<T> + 'static,
    {
        self.strategies.push(Box::new(strategy));
    }
    
    // Add a list of strategies to the validation. The strategies will be executed when the
    // validation is performed. 
    pub fn add_strategies<S>(&mut self, strategies: Vec<S>)
    where
        S: ValidationStrategy<T> + 'static,
    {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    // Add a child validation to the validation. The child validation will be executed when the
    // validation of the parent is performed. 
    pub fn add_child(&mut self, child: Validation<T>) {
        self.children.push(child);
    }

    // Remove a strategy from the validation. The strategy will no longer be executed when the
    // validation is performed.
    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        self.strategies.retain(|s| !std::ptr::eq(s.as_any(), strategy));
    }

    // Remove a child validation from the validation. The child validation will no longer be
    // executed when the validation of the parent is performed.
    pub fn remove_child(&mut self, child: &Validation<T>) {
        self.children.retain(|c| !std::ptr::eq(c, child));
    }

    // Perform the validation on a list of inputs. Returns a list of booleans indicating whether
    // each input is valid or not.
    pub fn batch_process(&self, inputs: &[T]) -> Vec<bool> {
        inputs.iter().map(|input| self.is_valid(input)).collect()
    }

    // Perform the validation on a list of inputs. Returns a list of booleans indicating whether
    // each input is valid or not. The context is passed to the validation strategies and can be
    // used to store state between validations. This is useful when the validation strategies
    // need to be stateful.
    pub fn batch_process_with_context<C>(&self, inputs: &[T], context: &C) -> Vec<bool>
    where
        C: 'static,
    {
        inputs
            .iter()
            .map(|input| self.is_valid_with_context(input, context))
            .collect()
    }


    // Perform the validation on a single input. Returns a boolean indicating whether the input is
    // valid or not. The context is passed to the validation strategies and can be used to store 
    // state between validations. 
    pub fn is_valid_with_context<C>(&self, input: &T, context: &C) -> bool
    where
        C: 'static,
    {
        self.strategies.iter().all(|strategy| strategy.is_valid(input)) &&
        self.children.iter().all(|child| child.is_valid_with_context(input, context))
    }

    // Perform the validation on a single input. Returns a boolean indicating whether the input
    // is valid or not. 
    pub fn is_valid(&self, input: &T) -> bool {
        self.strategies.iter().all(|strategy| strategy.is_valid(input)) &&
        self.children.iter().all(|child| child.is_valid(input))
    }
}


trait ValidatorStrategy<T> {
    fn validate(&self, data: &T) -> bool;
}

pub struct Validator<T> {
    // Stores the data that will be validated by the strategies in the vector
    // T is a generic type that will be specified when the Validator is created
    // 'static is a lifetime specifier that means the Validator will live for 
    // the entire duration of the program (which is what we want) 
    data: T,
    // Stores a list of strategies that will be used to validate the data
    // Box<dyn ValidatorStrategy<T>> is a trait object that can hold any type
    // that implements the ValidatorStrategy<T> trait (which is all of them)
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> Validator<T> {
    // Creates a new Validator with the given data and an empty vector of strategies
    // TODO: Consider Atomic queue from crossbeam crate it could look like this:
    pub fn new(data: T) -> Self {
        Validator {
            data,
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T>>) {
        self.strategies.push(strategy);
    }

    pub fn add_strategies(&mut self, strategies: Vec<Box<dyn ValidationStrategy<T>>>) {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        self.strategies.retain(|s| !std::ptr::eq(s.as_any(), strategy));
    }

    // Perform the validation on the data. Returns a boolean indicating whether the data is valid
    // or not. 
    pub fn validate(&self) -> bool {
        self.strategies.iter().all(|strategy| strategy.is_valid(&self.data))
    }
}



pub struct ValidatorFactory<T: 'static> {
    validators: Vec<Validation<T>>,
}

impl<T: 'static> ValidatorFactory<T> {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn create_validator(&mut self) -> &mut Validation<T> {
        let validator = Validation::new();
        self.validators.push(validator);
        self.validators.last_mut().unwrap()
    }

    pub fn remove_validator(&mut self, validator: &Validation<T>) {
        self.validators.retain(|v| !std::ptr::eq(v, validator));
    }

    pub fn remove_strategy(&mut self, validator: &mut Validation<T>, strategy: &dyn Any) {
        validator.remove_strategy(strategy);
    }
}

pub struct ValidationBuilder<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ValidationBuilder<T> {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy<S: ValidationStrategy<T> + 'static>(mut self, strategy: S) -> Self {
        self.strategies.push(Box::new(strategy));
        self
    }

    pub fn add_strategies<S: ValidationStrategy<T> + 'static>(mut self, strategies: Vec<S>) -> Self {
        for strategy in strategies {
            self.strategies.push(Box::new(strategy));
        }
        self
    }   

    pub fn build(self) -> Validation<T> {
        Validation {
            strategies: self.strategies,
            children: Vec::new(),
        }
    }
}

pub struct ValidationStrategyBuilder<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ValidationStrategyBuilder<T> {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy<S: ValidationStrategy<T> + 'static>(mut self, strategy: S) -> Self {
        self.strategies.push(Box::new(strategy));
        self
    }

    pub fn build(self) -> Vec<Box<dyn ValidationStrategy<T>>> {
        self.strategies
    }
}

pub struct ValidationConfigBuilder<T> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T> ValidationConfigBuilder<T> {
    pub fn new() -> Self {
        ValidationConfigBuilder {
            strategies: Vec::new(),
        }
    }

    pub fn with_strategy(mut self, strategy: Box<dyn ValidationStrategy<T>>) -> Self {
        self.strategies.push(strategy);
        self
    }

    pub fn build(self) -> ValidationConfig<T> {
        ValidationConfig {
            strategies: self.strategies,
        }
    }
}

pub struct ValidationConfig<T> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ValidationConfig<T> {
    pub fn new() -> Self {
        Self { strategies: Vec::new() }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T>>) {
        self.strategies.push(strategy);
    }

    

    pub fn validate(&self, input: &T) -> bool {
        self.strategies.iter().all(|strategy| strategy.is_valid(input))
    }
}


pub enum ValidationLogic<T> {
    // All strategies must be valid for the input to be valid
    All(Vec<Box<dyn ValidationStrategy<T>>>), 
    // Any strategy must be valid for the input to be valid
    Any(Vec<Box<dyn ValidationStrategy<T>>>),
    // The input must be valid for all strategies to be valid (the opposite of All)
    None(Vec<Box<dyn ValidationStrategy<T>>>),
    // The input must be valid for any strategy to be valid (the opposite of Any)
    Not(Vec<Box<dyn ValidationStrategy<T>>>),
}
    
impl<T: 'static> ValidationLogic<T> {
    pub fn validate(&self, input: &T) -> bool {
        match self {
            ValidationLogic::All(strategies) => strategies.iter().all(|strategy| strategy.is_valid(input)),
            ValidationLogic::Any(strategies) => strategies.iter().any(|strategy| strategy.is_valid(input)),
            ValidationLogic::None(strategies) => strategies.iter().all(|strategy| !strategy.is_valid(input)),
            ValidationLogic::Not(strategies) => strategies.iter().any(|strategy| !strategy.is_valid(input)),
        }
    }
}



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






#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_length_validation() {
        let strategy = LengthValidation;
        assert!(strategy.is_valid(&"abcdef".to_string()));
        assert!(!strategy.is_valid(&"abcde".to_string()));
    }

    #[test]
    fn test_number_validation() {
        let strategy = NumberValidation;
        assert!(strategy.is_valid(&6));
        assert!(!strategy.is_valid(&5));
    }

    #[test]
    fn test_validator() {
        let mut validator = Validator::new(6);
        validator.add_strategy(Box::new(NumberValidation));
        assert!(validator.validate());
    }

    #[test]
    fn test_validator_factory() {
        let mut factory: ValidatorFactory<i32> = ValidatorFactory::new();
        let validator = factory.create_validator();
        validator.add_strategy(NumberValidation);
        assert!(factory.validators[0].is_valid(&6));
        assert!(!factory.validators[0].is_valid(&5));
    }


  
     


}

