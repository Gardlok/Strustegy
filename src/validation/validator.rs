



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