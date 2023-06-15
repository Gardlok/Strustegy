


use std::any::{Any, TypeId};
use std::fmt;


#[derive(Debug, PartialEq, Eq, Clone)]
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

impl std::error::Error for ValidationError {}

impl From<String> for ValidationError {
    fn from(message: String) -> Self {
        ValidationError::new(message)
    }
}

impl From<&str> for ValidationError {
    fn from(message: &str) -> Self {
        ValidationError::new(message.to_string())
    }
}

impl From<ValidationError> for Vec<ValidationError> {
    fn from(error: ValidationError) -> Self {
        vec![error]
    }
}

impl From<Vec<ValidationError>> for ValidationError {
    fn from(errors: Vec<ValidationError>) -> Self {
        ValidationError::multiple_errors(errors)
    }
}

impl From<ValidationError> for Box<dyn Any + Send + Sync> {
    fn from(error: ValidationError) -> Self {
        Box::new(error)
    }
}

impl From<Box<dyn Any + Send + Sync>> for ValidationError {
    fn from(error: Box<dyn Any + Send + Sync>) -> Self {
        *error.downcast::<ValidationError>().unwrap()
    }
}



