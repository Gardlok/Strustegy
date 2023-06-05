
#[cfg(test)]
mod tests {
    use super::*;


    // Test for lib.rs
    #[test]
    fn test_lib() {
        // Instantiate the library and check the default strategy
        let lib = Strustegy::new();
        assert_eq!(lib.default_strategy(), "Expected Default Strategy");
    }

    // Test for validator.rs
    #[test]
    fn test_validator() {
        // Instantiate the validator and check its behavior
        let validator = Validator::new();
        assert!(validator.validate("Test Input"));
    }

    // Test for builder.rs
    #[test]
    fn test_builder() {
        // Instantiate the builder and check its behavior
        let builder = Builder::new();
        assert_eq!(builder.build(), "Expected Output");
    }

    // Test for config.rs
    #[test]
    fn test_config() {
        // Instantiate the config and check its behavior
        let config = Config::new();
        assert_eq!(config.get_config(), "Expected Config");
    }

    // Test for error.rs
    #[test]
    fn test_error() {
        // Instantiate the error and check its behavior
        let error = Error::new();
        assert_eq!(error.get_message(), "Expected Error Message");
    }

    // Test for strategy.rs
    #[test]
    fn test_strategy() {
        // Instantiate the strategy and check its behavior
        let strategy = Strategy::new();
        assert_eq!(strategy.execute(), "Expected Result");
    }


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
  