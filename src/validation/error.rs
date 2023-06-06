

use std::any::Any;



// validation/error.rs

// This file contains definitions for various types of validation errors. These errors are used
// to provide detailed information about why a validation failed.

use std::fmt;

// A general validation error that can represent any kind of validation failure.
#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        ValidationError { message }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation error: {}", self.message)
    }
}

impl std::error::Error for ValidationError {}

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

// We can also store multiple errors in a single ValidationError. This is useful for when we want
// to validate a single input against multiple strategies and return all of the errors to the user.
pub struct MultipleValidationError {
    message: String,
    errors: Vec<Box<dyn Any>>,
}

impl MultipleValidationError {
    pub fn new(message: String, errors: Vec<Box<dyn Any>>) -> Self {
        MultipleValidationError { message, errors }
    }

    pub fn get_errors(&self) -> &Vec<Box<dyn Any>> {
        &self.errors
    }
}




