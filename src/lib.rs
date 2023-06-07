
use std::any::{TypeId, Any};

mod validation;
use validation::*;

#[cfg(test)]
use tests;



pub enum ValidationLogic {
    AlwaysValid,
    AlwaysInvalid,
    And,
    Or,
    Not(Box<ValidationLogic>),
    // The Gate variant should contain a list of ValidationLogic enums 
    // that represent a complex combination of logical gates
    Gate(Vec<ValidationLogic>),
}

pub struct Validation<T: 'static> where T: 'static + Send + Sync + Clone {
    pub validators: Vec<Validator<T>>,
    pub children: Vec<Validation<T>>,
    pub logic: ValidationLogic,
}

impl<T: 'static> Validation<T> where T: 'static + Send + Sync + Clone{
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            children: Vec::new(),
            logic: ValidationLogic::And,
        }
    }

    pub fn add_validator(&mut self, validator: Validator<T>) {
        self.validators.push(validator);
    }

    pub fn remove_validator(&mut self, validator: &Validator<T>) {
        self.validators.retain(|v| !std::ptr::eq(v, validator));
    }

    pub fn add_child(&mut self, child: Validation<T>) {
        self.children.push(child);
    }

    pub fn add_children(&mut self, children: Vec<Validation<T>>) {
        for child in children {
            self.add_child(child);
        }
    }

    pub fn remove_child(&mut self, child: &Validation<T>) {
        self.children.retain(|c| !std::ptr::eq(c, child));
    }

    pub fn remove_children(&mut self, children: &[Validation<T>]) {
        for child in children {
            self.remove_child(child);
        }
    }


    // Perform the validation on a list of inputs. Returns a list of booleans indicating whether
    // each input is valid or not.
    pub fn batch_process(&self, inputs: &[T]) -> Vec<bool> {
        inputs.iter().map(|input| self.is_valid(input)).collect()
    }

    pub fn is_valid(&self, input: &T) -> bool {
        let validator_results: Vec<bool> = self.validators.iter().map(|validator| validator.validate(input).into()).collect();
        let child_results: Vec<bool> = self.children.iter().map(|child| child.is_valid(input)).collect();
        let all_results = [validator_results, child_results].concat();

        match &self.logic {
            ValidationLogic::AlwaysValid => true,
            ValidationLogic::AlwaysInvalid => false,
            ValidationLogic::And => all_results.iter().all(|&result| result),
            ValidationLogic::Or => all_results.iter().any(|&result| result),
            ValidationLogic::Not(boxed_logic) => !self.is_valid_logic(input, &**boxed_logic),
            ValidationLogic::Gate(logics) => logics.iter().all(|logic| self.is_valid_logic(input, logic)),
        }
    }

    fn is_valid_logic(&self, input: &T, logic: &ValidationLogic) -> bool {
        match logic {
            ValidationLogic::AlwaysValid => true,
            ValidationLogic::AlwaysInvalid => false,
            ValidationLogic::And => self.children.iter().all(|child| child.is_valid(input)),
            ValidationLogic::Or => self.children.iter().any(|child| child.is_valid(input)),
            ValidationLogic::Not(boxed_logic) => !self.is_valid_logic(input, &**boxed_logic),
            ValidationLogic::Gate(logics) => logics.iter().all(|logic| self.is_valid_logic(input, logic)),
        }
    }


}

impl<T: 'static + Clone + Send + Sync> Default for Validation<T> {
    fn default() -> Self {
        Self::new()
    }
}

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

// impl<T> Into<Option<Vec<TypeId>>> for Validity<T> {
//     fn into(self) -> Option<Vec<TypeId>> {
//         match self {
//             Validity::Valid(_) => None,
//             Validity::Invalid((_, type_ids)) => Some(type_ids),
//         }
//     }
// }

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


