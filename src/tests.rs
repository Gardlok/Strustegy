
use std::marker::PhantomData;




#[cfg(test)]
mod tests_orig {
    use super::*;
    use crate::validation::logic::Scope;
    use crate::validation::proof::Proof;
    use crate::validation::strategy::Strategy;
    use crate::validation::logic::Target;

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



    use crate::validation::logic::Scope;
    use crate::validation::validator::Validator;
    use crate::validation::proof::Proof;
    use crate::validation::strategy::Strategy;
    use crate::validation::logic::Target;


    
    #[test]
    fn test_generic_validator() {
        // Create a target
        struct TestTarget {
            value: i32,
        }
        impl Target<'_> for TestTarget {
            type Value = i32;
            fn value(&self) -> Self::Value {
                self.value
            }
        }
        let target = TestTarget { value: 42 };

        // Create a strategy
        struct TestStrategy;
        impl Strategy<TestTarget> for TestStrategy {
            fn apply(&self, target: &TestTarget) -> bool {
                target.value == 42
            }
        }

        // Create a proof
        struct TestProof {
            strategy: TestStrategy,
        }
        impl Proof<'_, TestTarget> for TestProof {
            type Strategy = TestStrategy;
            fn validate(&self, strategy: &Self::Strategy, target: &TestTarget) -> bool {
                strategy.apply(target)
            }
        }

        // Create a scope
        struct TestScope {
            proof: TestProof,
        }
        impl Scope<'_, TestTarget> for TestScope {
            type Proof = TestProof;
            fn proof<'s>(&'s self) -> &'s Self::Proof {
                &self.proof
            }
            fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &TestTarget) -> bool {
                proof.validate(&self.proof.strategy, target)
            }
        }

        // Create a validator
        struct TestValidator {
            scope: TestScope,
        }
        impl <'a>Validator<'a,  TestTarget> for TestValidator {
            type Strategy<'s> = TestStrategy where Self: 's;
            type Proof<'s>  = TestProof where Self: 's;
            type Scope<'s>  = TestScope where Self: 's;

            fn validate<'s>(&self, scope: &Self::Scope<'s>, target: &TestTarget) -> bool {
                let proof = scope.proof();
                scope.validate(proof, target)
            }


        }

        // Create a generic validator
        let generic_validator = TestValidator {
            scope: TestScope {
                proof: TestProof {
                    strategy: TestStrategy,
                },
            },
        };

        // Create a generic strategy
        let generic_strategy = TestStrategy;

        // Create a generic proof
        let generic_proof = TestProof {
            strategy: TestStrategy,
        };

        // Create a generic scope
        let generic_scope = TestScope {
            proof: TestProof {
                strategy: TestStrategy,
            },
        };
        

        // Test the generic validator
        assert!(generic_validator.validate(&generic_scope, &target));

        // Test the generic strategy
        assert!(generic_strategy.apply(&target));

        // Test the generic scope
        assert!(generic_scope.validate(&generic_proof, &target));

        // Test the generic proof
        assert!(generic_proof.validate(&TestStrategy, &target));

        // Test the concrete validator
        let validator = TestValidator {
            scope: TestScope {
                proof: TestProof {
                    strategy: TestStrategy,
                },
            },
        };
        assert!(validator.validate(&validator.scope, &target));
    }
}