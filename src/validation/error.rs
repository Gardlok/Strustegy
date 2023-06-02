

use std::any::Any;



pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        ValidationError { message }
    }
}

// For extensive error handling, we can use the Any trait to store any type of error in the
// ValidationError. This allows us to store errors from other crates, such as serde_json, and
// return them to the user.
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

impl From<ValidationError> for AnyValidationError {
    fn from(error: ValidationError) -> Self {
        AnyValidationError {
            message: error.message,
            error: Box::new(error),
        }
    }
}

// We can also create a MultipleValidationError that contains a list of AnyValidationErrors. This
// allows us to return multiple errors to the user. This is useful when we want to validate a
// struct and return all errors that occurred during validation. This is also useful when we want
// to validate a list of structs and return all errors that occurred during validation. 
// 
pub struct MultipleValidationError {
    message: String,
    errors: Vec<AnyValidationError>,
}

impl MultipleValidationError {
    pub fn new(message: String, errors: Vec<AnyValidationError>) -> Self {
        MultipleValidationError { message, errors }
    }
    pub fn get_errors(&self) -> &Vec<AnyValidationError> {
        &self.errors
    }
}

impl From<MultipleValidationError> for AnyValidationError {
    fn from(error: MultipleValidationError) -> Self {
        AnyValidationError {
            message: error.message,
            error: Box::new(error),
        }
    }
}

impl From<AnyValidationError> for ValidationError {
    fn from(error: AnyValidationError) -> Self {
        ValidationError {
            message: error.message,
        }
    }
}

impl From<MultipleValidationError> for ValidationError {
    fn from(error: MultipleValidationError) -> Self {
        ValidationError {
            message: error.message,
        }
    }
}


