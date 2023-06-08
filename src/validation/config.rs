



use validation::{ValidationError, Validator, ValidationStrategy, StrategyMap, Strategy, Context, StrategyContext};


#[cfg(test)]
use validation::tests;
use std::marker::PhantomData;
use dashmap::DashMap as HashMap;
use crossbeam::queue::{ArrayQueue, SegQueue};
use crossbeam::sync::ShardedLock;

pub enum ValidatorLogic {AlwaysValid, AlwaysInvalid, And, Or, Not(Box<ValidatorLogic>), Gate(Vec<ValidatorLogic>) }



pub fn is_valid(&self, input: &T) -> bool {
    let validator_results: Vec<bool> = self.validators.iter().map(|validator| validator.validate(input).into()).collect();
    let child_results: Vec<bool> = self.children.iter().map(|child| child.is_valid(input)).collect();
    let all_results = [validator_results, child_results].concat();
    match &self.logic {
        ValidatorLogic::AlwaysValid => true,
        ValidatorLogic::AlwaysInvalid => false,
        ValidatorLogic::And => all_results.iter().all(|&result| result),
        ValidatorLogic::Or => all_results.iter().any(|&result| result),
        ValidatorLogic::Not(boxed_logic) => !self.is_valid_logic(input, &**boxed_logic),
        ValidatorLogic::Gate(logics) => logics.iter().all(|logic| self.is_valid_logic(input, logic)),
    }
}

fn is_valid_logic(&self, input: &T, logic: &ValidatorLogic) -> bool {
    match logic {
        ValidatorLogic::AlwaysValid => true,
        ValidatorLogic::AlwaysInvalid => false,
        ValidatorLogic::And => self.children.iter().all(|child| child.is_valid(input)),
        ValidatorLogic::Or => self.children.iter().any(|child| child.is_valid(input)),
        ValidatorLogic::Not(boxed_logic) => !self.is_valid_logic(input, &**boxed_logic),
        ValidatorLogic::Gate(logics) => logics.iter().all(|logic| self.is_valid_logic(input, logic)),
    }
}





pub trait Validation<T> {
    fn validate(&self, input: &T) -> Validity<T>;
    fn validations(&self) -> &ValidationConfig<T>;
}



pub struct ValidationConfig<T> {
    pub validators: ArrayQueue<Validator<T>>,
    pub sub_validator_config: SegQueue<ValidationConfig<T>>,
    pub logic: ShardedLock<ValidatorLogic>,
}

impl<T> ValidationConfig<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            validators: ArrayQueue::new(capacity),
            sub_validator_config: SegQueue::<ValidationConfig<T>>::new(),
            logic: ValidatorLogic::And,
        }
    }

    pub fn with_logic(capacity: usize, logic: ValidatorLogic) -> Self {
        Self {
            validators: ArrayQueue::new(capacity),
            sub_validator_config: SegQueue::<ValidationConfig<T>>::new(),
            logic,
        }
    }

    pub fn add_validator(&self, validator: Validator<T>) {
        self.validators.push(validator);
    }

    pub fn add_sub_validator_config(&self, config: ValidationConfig<T>) {
        self.sub_validator_config.push(config);
    }

    pub fn remove_validator(&self, validator: TypeId) -> Result<(), ValidationError> {
        let mut validators = self.validators.clone();
        let mut removed = false;
        while let Some(validator) = validators.pop() {
            if validator.id == validator {
                removed = true;
                break;
            }
        }
        if removed {
            Ok(())
        } else {
            Err(ValidationError::ValidatorNotFound(validator))
        }
    }

    pub fn remove_sub_validator_config(&self, config: TypeId) -> Result<(), ValidationError> {
        let mut configs = self.sub_validator_config.clone();
        let mut removed = false;
        while let Some(config) = configs.pop() {
            if config.id == config {
                removed = true;
                break;
            }
        }
        if removed {
            Ok(())
        } else {
            Err(ValidationError::ValidatorNotFound(config))
        }
    }

    pub fn set_logic(&self, logic: ValidatorLogic) {
        self.logic = logic;
    }

    pub fn get_logic(&self) -> ValidatorLogic {
        self.logic
    }

    pub fn get_validators(&self) -> ArrayQueue<Validator<T>> {
        self.validators.clone()
    }

    pub fn get_sub_validator_configs(&self) -> SegQueue<ValidationConfig<T>> {
        self.sub_validator_config.clone()
    }

    pub fn get_sub_validator_config(&self, id: TypeId) -> Result<ValidationConfig<T>, ValidationError> {
        let mut configs = self.sub_validator_config.clone();
        while let Some(config) = configs.pop() {
            if config.id == id {
                return Ok(config);
            }
        }
        Err(ValidationError::ValidatorNotFound(id))
    }

    
}

impl<T> Validation<T> for ValidationConfig<T> {
    fn validate(&self, input: &T) -> Validity<T> {
        let validator_results: Vec<bool> = self.validators.iter().map(|validator| validator.validate(input).into()).collect();
        let child_results: Vec<bool> = self.children.iter().map(|child| child.is_valid(input)).collect();
        let all_results = [validator_results, child_results].concat();
        match &self.logic {
            ValidatorLogic::AlwaysValid => true,
            ValidatorLogic::AlwaysInvalid => false,
            ValidatorLogic::And => all_results.iter().all(|&result| result),
            ValidatorLogic::Or => all_results.iter().any(|&result| result),
            ValidatorLogic::Not(boxed_logic) => !self.is_valid_logic(input, &**boxed_logic),
            ValidatorLogic::Gate(logics) => logics.iter().all(|logic| self.is_valid_logic(input, logic)),
        }
    }


}













#[derive(Clone)]
pub enum Validity<T> {
    Valid(T),
    Invalid((T, Vec<TypeId>)),
    NotChecked,
}



impl<T> Into<bool> for Validity<T> {
    fn into(self) -> bool {
        match self {
            Validity::Valid(_) => true,
            Validity::Invalid(_) => false,
            Validity::NotChecked => false,
        }
    }
}



impl<T> Validity<T> {
    pub fn is_valid(&self) -> bool {
        match self {
            Validity::Valid(_) => true,
            Validity::Invalid(_) => false,
            Validity::NotChecked => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self {
            Validity::Valid(_) => false,
            Validity::Invalid(_) => true,
            Validity::NotChecked => false,
        }
    }
}

pub trait ValidateOption<T: Clone + Send + Sync>: Sized {
    fn validate(self, validator: &Validator<T>) -> Option<T>;
}

impl<T> ValidateOption<T> for Option<T> 
where
    T: Clone + Send + Sync + 'static,
{
    fn validate(self, validator: &Validator<T>) -> Option<T> {
        match &self {
            Some(value) => match validator.validate(value) {
                Validity::Valid(_) => Some(value.clone()),
                _ => None,  // Return None if validation fails or is not checked
            },
            None => None,
        }
    }
}
