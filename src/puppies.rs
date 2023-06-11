





use crate::*;
// use crate::strategy_fn;
use std::any::TypeId;

#[cfg(test)]
mod tests {
    use super::*;





use super::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn pet_store_integration_test() {
    let pet_store = Arc::new(Mutex::new(PetStore::new()));
    let pet_store_clone = pet_store.clone();

    // Spawn a thread to handle the delivery of puppies
    let delivery_thread = thread::spawn(move || {
        loop {
            let mut pet_store = pet_store_clone.lock().unwrap();
            pet_store.deliver_puppy();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Spawn a thread to handle the customers
    let customer_thread = thread::spawn(move || {
        loop {
            let mut pet_store = pet_store.lock().unwrap();
            pet_store.get_puppy();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for the threads to finish
    delivery_thread.join().unwrap();
    customer_thread.join().unwrap();

    // Assert that the pet store is in the "green" zone
    let pet_store = pet_store.lock().unwrap();
    assert!(pet_store.is_in_green_zone());
}
}

// Pet Store
struct PetStore {
puppies: Vec<Puppy>,
validator: GeneralValidationStrategy<Puppy>,
}

impl PetStore {
fn new() -> Self {
    let mut validator = GeneralValidationStrategy::new();
    validator.add_strategy(PuppySizeValidator::new(), 0, false);
    validator.add_strategy(PuppyColorValidator::new(), 1, false);
    validator.add_strategy(PuppyAgeValidator::new(), 2, false);
    validator.add_strategy(PuppyWeightValidator::new(), 3, false);

    PetStore {
        puppies: Vec::new(),
        validator,
    }
}

fn deliver_puppies(&mut self) {
    let mut puppy = Puppy::random();
    self.validator.apply(&mut puppy).unwrap();
    self.puppies.push(puppy);
}

fn get_puppy(&mut self) {
    if let Some(puppy) = self.puppies.pop() {
        println!("Got a puppy: {:?}", puppy);
    }
}

fn is_in_green_zone(&self) -> bool {
    self.puppies.len() >= 8 && self.puppies.len() <= 12
}
}

// Puppy
#[derive(Debug)]
struct Puppy {
size: PuppySize,
color: PuppyColor,
age: u8,
weight: u16,
}

impl Puppy {
fn random() -> Self {
    let size = PuppySize::random();
    let color = PuppyColor::random();
    let age = rand::random::<u8>();
    let weight = rand::random::<u16>();

    Puppy {
        size,
        color,
        age,
        weight,
    }
}
}

// Puppy Size
#[derive(Debug)]
enum PuppySize {
Small,
Medium,
Large,
ExtraLarge,
}

impl PuppySize {
fn random() -> Self {
    let random_num = rand::random::<u8>();
    match random_num {
        0..=25 => PuppySize::Small,
        26..=50 => PuppySize::Medium,
        51..=75 => PuppySize::Large,
        _ => PuppySize::ExtraLarge,
    }
}
}

// Puppy Color
#[derive(Debug)]
enum PuppyColor {
White,
Brown,
Black,
Tan,
}

impl PuppyColor {
fn random() -> Self {
    let random_num = rand::random::<u8>();
    match random_num {
        0..=25 => PuppyColor::White,
        26..=50 => PuppyColor::Brown,
        51..=75 => PuppyColor::Black,
        _ => PuppyColor::Tan,
    }
}
}

// Puppy Size Validator
struct PuppySizeValidator;

impl PuppySizeValidator {
fn new() -> Self {
    PuppySizeValidator
}
}

impl Strategy for PuppySizeValidator {
type Target = Puppy;
type Error = ValidationError;

fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
    match target.size {
        PuppySize::Small | PuppySize::Medium | PuppySize::Large | PuppySize::ExtraLarge => {
            Ok(())
        }
    }
}
}

// Puppy Color Validator
struct PuppyColorValidator;

impl PuppyColorValidator {
fn new() -> Self {
    PuppyColorValidator
}
}

impl Strategy for PuppyColorValidator {
type Target = Puppy;
type Error = ValidationError;

fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
    match target.color {
        PuppyColor::White | PuppyColor::Brown | PuppyColor::Black | PuppyColor::Tan => {
            Ok(())
        }
    }
}
}

// Puppy Age Validator
struct PuppyAgeValidator;

impl PuppyAgeValidator {
fn new() -> Self {
    PuppyAgeValidator
}
}

impl Strategy for PuppyAgeValidator {
type Target = Puppy;
type Error = ValidationError;

fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
    if target.age > 0 && target.age < 12 {
        Ok(())
    } else {
        Err(ValidationError::strategy_error(
            TypeId::of::<Self>(),
            "Puppy age must be between 0 and 12".to_string(),
        ))
    }
}
}

// Puppy Weight Validator
struct PuppyWeightValidator;

impl PuppyWeightValidator {
fn new() -> Self {
    PuppyWeightValidator
}
}

impl Strategy for PuppyWeightValidator {
type Target = Puppy;
type Error = ValidationError;

fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
    if target.weight > 0 && target.weight < 20 {
        Ok(())
    } else {
        Err(ValidationError::strategy_error(
            TypeId::of::<Self>(),
            "Puppy weight must be between 0 and 20".to_string(),
        ))
    }
}
}