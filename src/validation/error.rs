

use std::any::{TypeId, Any};
use std::fmt;

pub enum ValidationError {
    StrategyError {
        strategy_type_id: TypeId,
        message: String,
    },
    MultipleErrors(Vec<ValidationError>),
    Other(String),
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        ValidationError::Other(message)
    }

    pub fn strategy_error(strategy_type_id: TypeId, message: String) -> Self {
        ValidationError::StrategyError {
            strategy_type_id,
            message,
        }
    }

    pub fn multiple_errors(errors: Vec<ValidationError>) -> Self {
        ValidationError::MultipleErrors(errors)
    }

    pub fn get_message(&self) -> String {
        match self {
            ValidationError::StrategyError { strategy_type_id, message } => {
                format!("Strategy error: {} ({:?})", message, strategy_type_id)
            }
            ValidationError::MultipleErrors(errors) => {
                let mut message = String::from("Multiple errors:");
                for error in errors {
                    message.push_str(&format!("\n\t{}", error.get_message()));
                }
                message
            }
            ValidationError::Other(message) => message.clone(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation error: {}", self.get_message())
    }
}

// impl std::error::Error for ValidationError {}

pub struct AnyValidationError {
    message: String,
    error: Box<dyn Any>,
}

impl AnyValidationError {
    pub fn new<T: Any>(message: String, error: T) -> Self {
        AnyValidationError {
            message,
            error: Box::new(error),
        }
    }

    pub fn get_error<T: Any>(&self) -> Option<&T> {
        self.error.downcast_ref::<T>()
    }
}

pub struct MultipleValidationError {
    errors: Vec<Box<dyn Any>>,
}

impl MultipleValidationError {
    pub fn new(errors: Vec<Box<dyn Any>>) -> Self {
        MultipleValidationError { errors }
    }

    pub fn get_errors(&self) -> &[Box<dyn Any>] {
        &self.errors
    }
}



