
use std::marker::PhantomData;

use crate::validation::target::Target;
use crate::validation::strategy::Strategy;
use crate::validation::proof::Proof;



#[cfg(test)]
mod tests_orig {
    use super::*;

    struct TestTarget {
        value: bool,
    }

    impl<'a> Target<'a> for TestTarget {
        type Value = bool;
        fn value(&'a self) -> Self::Value {
            self.value
        }
    }

    struct TestStrategy;

    impl Strategy<TestTarget> for TestStrategy {
        fn apply(&self, target: &TestTarget) -> bool {
            target.value()
        }
    }

    struct TestProof<'a> {
        strategy: TestStrategy,
        _phantom: PhantomData<&'a TestTarget>,
    }

    impl<'a> Proof<'a, TestTarget> for TestProof<'a> {
        type Strategy = TestStrategy;
        fn validate(&'a self, strategy: &Self::Strategy, target: &TestTarget) -> bool {
            strategy.apply(target)
        }
    }

    #[test]
    fn test_proof() {
        let target = TestTarget { value: true };
        let proof = TestProof {
            strategy: TestStrategy,
            _phantom: PhantomData,
        };
        assert!(proof.validate(&proof.strategy, &target));
    }

   
}

#[cfg(test)]
mod tests_basics {
    use super::*;

    use crate::validation::logic::Scope;
    use crate::validation::logic::CompositionOperator;
    use crate::validation::validator::Validator;
    use crate::validation::strategy::GenericStrategy;

    use crate::validation::proof::Proof;


    #[test]
    fn test_generic_validator() {

        struct Target {
            value: i32,
        }

        impl Target {
            fn new(value: i32) -> Self {
                Self { value }
            }
        }

        impl<'a> Target<'a> for Target {
            type Value = i32;
            fn value(&'a self) -> Self::Value {
                self.value
            }
        }

        // Create a strategy
        struct Strategy {
            value: i32,
        }

        impl Strategy {
            fn new(value: i32) -> Self {
                Self { value }
            }
        }

        impl Strategy<Target> for Strategy {
            fn apply(&self, target: &Target) -> bool {
                target.value() == self.value
            }
        }

        // Create a proof
        struct Proof {
            strategy: Strategy,
        }

        impl<'a> Proof<'a, Target> for Proof {
            type Strategy = Strategy;
            fn validate(&'a self, strategy: &Self::Strategy, target: &Target) -> bool {
                strategy.apply(target)
            }
        }

        // Create a scope
        struct Scope {
            proof: Proof,
        }

        impl<'a> Scope<'a, Target> for Scope {
            type Proof = Proof;
            fn proof<'s>(&'s self) -> &'s Self::Proof {
                &self.proof
            }
            fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &Target) -> bool {
                proof.validate(&self.proof.strategy, target)
            }
        }

        // Create a validator
        struct Validator {
            scope: Scope,
        }

        impl<'a> Validator<'a, Target> for Validator {
            type Scope = Scope;
            fn validate(&'a self, scope: &Self::Scope, target: &Target) -> bool {
                let proof = scope.proof();
                scope.validate(proof, target)
            }
        }

        // Create a generic validator
        let validator = GenericValidator::new(Scope {
            proof: Proof {
                strategy: Strategy::new(42),
            },
        });

        // Test the validator
        let target = Target::new(42);
        assert!(validator.validate(&validator.scope, &target));

        let target = Target::new(43);
        assert!(!validator.validate(&validator.scope, &target));
    }
}