


#[macro_use]


use crate::{Validator, ValidationStrategy, ValidateOption, Validation, Validity, ValidatorLogic, StrategyMap, Strategy, StrategyContext};
// use crate::strategy_fn;
use std::any::TypeId;

#[macro_use]

#[derive(Debug, Clone)]
struct Puppy {
    name: String,
    age: u8,
    breed: String,
    weight: f32,
}

#[derive(Debug, Clone)]
pub struct NameNotEmptyStrategy;
impl ValidationStrategy<Puppy> for NameNotEmptyStrategy {
    fn is_valid(&self, puppy: &Puppy) -> bool {
        !puppy.name.is_empty()
    }

}

#[derive(Debug, Clone)]
pub struct AgeInRangeStrategy;
impl ValidationStrategy<Puppy> for AgeInRangeStrategy {
    fn is_valid(&self, puppy: &Puppy) -> bool {
        puppy.age >= 1 && puppy.age <= 10
    }

}

#[derive(Debug, Clone)]
pub struct WeightPositiveStrategy;
impl ValidationStrategy<Puppy> for WeightPositiveStrategy {
    fn is_valid(&self, puppy: &Puppy) -> bool {
        puppy.weight > 0.0
    }


}

#[cfg(test)]
mod tests {
    use super::*;
       

    
    #[derive(Clone)]
    struct TestInput {
        value: i32,
    }

    struct TestValidator;
    impl Validator<TestInput> for TestValidator {
        fn validate(&self, input: &TestInput) -> ValidationResult {
            if input.value > 0 {
                ValidationResult::Valid
            } else {
                ValidationResult::Invalid(ValidationError::new("Value must be greater than 0"))
            }
        }
    }

    #[test]
    fn test_validation() {
        let mut validation = Validation::<TestInput>::new();
        validation.add_validator(Box::new(TestValidator));

        let valid_input = TestInput { value: 1 };
        assert!(validation.is_valid(&valid_input));

        let invalid_input = TestInput { value: -1 };
        assert!(!validation.is_valid(&invalid_input));
    }

    #[test]
    fn test_validation_with_child() {
        let mut child_validation = Validation::<TestInput>::new();
        child_validation.add_validator(Box::new(TestValidator));

        let mut parent_validation = Validation::<TestInput>::new();
        parent_validation.add_child(child_validation);

        let valid_input = TestInput { value: 1 };
        assert!(parent_validation.is_valid(&valid_input));

        let invalid_input = TestInput { value: -1 };
        assert!(!parent_validation.is_valid(&invalid_input));
    }

    #[test]
    fn test_validation_errors() {
        let mut validation = Validation::<TestInput>::new();
        validation.add_validator(Box::new(TestValidator));

        let invalid_input = TestInput { value: -1 };
        assert!(!validation.is_valid(&invalid_input));

        let errors = validation.get_errors();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Value must be greater than 0");
    }

    #[test]
    fn test_validation_context() {
        let mut validation = Validation::<TestInput>::new();
        validation.update_context(Context::<StrategyContext>::new(2));

        let context = validation.get_context();
        assert_eq!(context.get_value(), 2);
    }


    #[test]
    fn test_validator_with_strategies() {
        let mut validator = Validator::<Puppy>::new();

        validator.add_strategy(Box::new(NameNotEmptyStrategy));
        validator.add_strategy(Box::new(AgeInRangeStrategy));
        validator.add_strategy(Box::new(WeightPositiveStrategy));

        let puppy = Puppy {
            name: "Buddy".to_string(),
            age: 5,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };

        assert!(validator.validate(&puppy).is_valid());
    }


    #[test]
    fn test_validator_with_invalid_puppy() {
        let mut validator = Validator::<Puppy>::new();

        validator.add_strategy(Box::new(NameNotEmptyStrategy));
        validator.add_strategy(Box::new(AgeInRangeStrategy));
        validator.add_strategy(Box::new(WeightPositiveStrategy));

        let puppy = Puppy {
            name: "".to_string(), // Invalid name
            age: 5,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };

        assert_eq!(validator.validate(&puppy).is_invalid(), false); // 
    }

    #[test]
    fn test_validator_with_no_strategies() {
        let validator = Validator::<Puppy>::new(); // No strategies added

        let puppy = Puppy {
            name: "Buddy".to_string(),
            age: 5,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };

        // Should return Ok(true) because there are no strategies to invalidate the puppy
        assert!(validator.validate(&puppy).is_valid());
    }

    #[test]
    fn test_validator_with_removed_strategy() {
        let mut validator = Validator::<Puppy>::new();

        let strategy = Box::new(NameNotEmptyStrategy);
        validator.add_strategy(strategy.clone());
        validator.add_strategy(Box::new(AgeInRangeStrategy));
        validator.add_strategy(Box::new(WeightPositiveStrategy));

        validator.remove_strategy(strategy.as_ref()).unwrap();

        let puppy = Puppy {
            name: "".to_string(), // Invalid name, but the strategy checking this has been removed
            age: 5,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };

        assert!(validator.validate(&puppy).is_valid());
    }

    #[test]
    fn test_name_not_empty_strategy() {
        let strategy = NameNotEmptyStrategy;
        let puppy = Puppy {
            name: "".to_string(),
            age: 5,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };
        assert!(!strategy.is_valid(&puppy));
    }

    #[test]
    fn test_age_in_range_strategy() {
        let strategy = AgeInRangeStrategy;
        let puppy = Puppy {
            name: "Buddy".to_string(),
            age: 15,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };
        assert!(!strategy.is_valid(&puppy));
    }

    #[test]
    fn test_weight_positive_strategy() {
        let strategy = WeightPositiveStrategy;
        let puppy = Puppy {
            name: "Buddy".to_string(),
            age: 5,
            weight: -3.5,
            breed: "Poodle".to_string(),
        };
        assert!(!strategy.is_valid(&puppy));
    }

    #[test]
    fn test_validator_with_multiple_invalid_strategies() {
        let mut validator = Validator::<Puppy>::new();

        validator.add_strategy(Box::new(NameNotEmptyStrategy));
        validator.add_strategy(Box::new(AgeInRangeStrategy));
        validator.add_strategy(Box::new(WeightPositiveStrategy));

        let puppy = Puppy {
            name: "".to_string(),
            age: 15,
            weight: -3.5,
            breed: "Poodle".to_string(),
        };

        assert!(validator.validate(&puppy).is_invalid());
    }

    #[test]
    fn test_validator_with_all_valid_strategies() {
        let mut validator = Validator::<Puppy>::new();

        validator.add_strategy(Box::new(NameNotEmptyStrategy));
        validator.add_strategy(Box::new(AgeInRangeStrategy));
        validator.add_strategy(Box::new(WeightPositiveStrategy));

        let puppy = Puppy {
            name: "Buddy".to_string(),
            age: 5,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };

        assert!(validator.validate(&puppy).is_valid());
    }

    #[test]
    fn test_validator_with_duplicate_strategy() {
        let mut validator = Validator::<Puppy>::new();

        let strategy = Box::new(NameNotEmptyStrategy);
        validator.add_strategy(strategy.clone());
        validator.add_strategy(strategy.clone());

        let puppy = Puppy {
            name: "Buddy".to_string(),
            age: 5,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };

        assert!(validator.validate(&puppy).is_valid());
    }

    #[test]
    fn test_validator_with_non_existent_strategy() {
        let mut validator = Validator::<Puppy>::new();

        let strategy = Box::new(NameNotEmptyStrategy);
        validator.add_strategy(strategy.clone());

        let puppy = Puppy {
            name: "Buddy".to_string(),
            age: 5,
            weight: 3.5,
            breed: "Poodle".to_string(),
        };

        assert!(validator.remove_strategy(strategy.as_ref()).is_ok());
        assert!(validator.validate(&puppy).is_valid());
    }


}


#[cfg(test)]
mod strategy_tests {
    use super::*;

    pub use crate::validation::strategies::*;
    pub use crate::validation::{ValidationStrategy, Validator, Context, StrategyContext};


    #[test]
    fn test_length_validation() {
        let validator = LengthValidation::new(5, 10);
        assert!(validator.is_valid(&"hello".to_string()));
        assert!(!validator.is_valid(&"hi".to_string()));
        assert!(!validator.is_valid(&"hello world".to_string()));
    }

   
    #[test]
    fn test_regex_validation() {
        let validator = RegexValidation::new(r"^\d+$");
        assert!(validator.is_valid(&"12345".to_string()));
        assert!(!validator.is_valid(&"hello".to_string()));
    }


    #[test]
    fn test_range_validation() {
        let validator = RangeValidation::new(5, 10);
        assert!(validator.is_valid(&7));
        assert!(!validator.is_valid(&4));
        assert!(!validator.is_valid(&11));
    }


    #[test]
    fn test_validate_option() {
        // Create a validator
        let mut validator = Validator::<i32>::new();
        validator.add_strategy(Box::new(RangeValidation::new(0, 10))); // Assuming RangeValidation::new takes two arguments for the range

        // Test with Some value
        let option = Some(5);
        assert_eq!(option.validate(&validator), Some(5));

        // Test with None value
        let option = None;
        assert_eq!(option.validate(&validator), None);

        // Test with invalid value
        let option = Some(-5);
        assert_eq!(option.validate(&validator), None);
    }



 
    #[test]
    fn test_range_validation_strategy() {
        let mut strategy = ValidationRangeStrategy::new(1, 5);

        // Test is_valid method
        assert_eq!(strategy.is_valid(&1), false); // validations = 0, less than min_validations
        strategy.validations = 1;
        assert_eq!(strategy.is_valid(&1), true); // validations = 1, equal to min_validations
        strategy.validations = 5;
        assert_eq!(strategy.is_valid(&1), true); // validations = 5, equal to max_validations
        strategy.validations = 6;
        assert_eq!(strategy.is_valid(&1), false); // validations = 6, greater than max_validations

        // Test update_context method
        let mut context = Context::new(6);
        strategy.validations = 0;
        assert_eq!(strategy.update_context(&mut context, &1), Ok(())); // validations = 0, less than min_validations
        assert_eq!(context.get_by_type_id(TypeId::of::<ValidationRangeStrategy<i32>>()).unwrap().disabled, false);
        context.alter_by_type_id(TypeId::of::<&ValidationRangeStrategy<i32>>(), |strategy_context| {
            strategy_context.disabled = false;
        });
        strategy.validations = 6;
        assert_eq!(strategy.update_context(&mut context, &1), Ok(())); // validations = 6, greater than max_validations
        assert_eq!(context.get_by_type_id(TypeId::of::<ValidationRangeStrategy<i32>>()).unwrap().disabled, true);
    }

}






