
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
  