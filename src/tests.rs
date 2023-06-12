
// use std::marker::PhantomData;
use std::any::{Any, TypeId};
use std::error::Error;




#[cfg(test)]
mod tests_orig {
    use super::*;

    use crate::validation::proof::Proof;
    use crate::validation::logic::Scope;
    use crate::validation::strategy::Strategy;
    use crate::validation::strategy::Target;
    use std::marker::PhantomData;

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



    // use crate::validation::logic::Scope;
    use crate::validation::validator::Validator;
    use crate::validation::proof::Proof;
    use crate::validation::strategy::Strategy;
    use crate::validation::strategy::Target;
    use crate::validation::logic::Scope;


    
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::validation::strategy::*;

    // #[test]
    // fn test_number_validation() {
    //     // Create a set of numbers to validate
    //     let numbers = vec![2, 4, 6, 8, 10];

    //     // Create a proof that validates whether a number is positive
    //     let positive_proof = GenericProof::new(|&n: &i32| n > 0);

    //     // Create a proof that validates whether a number is even
    //     let even_proof = GenericProof::new(|&n: &i32| n % 2 == 0);

    //     // Create a strategy that uses both proofs
    //     let strategy = GenericStrategy::new(vec![positive_proof, even_proof]);

    //     // Validate the numbers using the strategy
    //     for &number in &numbers {
    //         assert!(strategy.validate(&number));
    //     }

    //     // Save the strategy to a binary tree
    //     let tree = strategy.to_binary_tree();

    //     // Load a new strategy from the binary tree
    //     let loaded_strategy = GenericStrategy::from_binary_tree(tree);

    //     // Validate the numbers using the loaded strategy
    //     for &number in &numbers {
    //         assert!(loaded_strategy.validate(&number));
    //     }
    // }
}






// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rand::Rng;

//     use crate::validation::logic::*;
//     use crate::validation::proof::*;
//     use crate::validation::strategy::*;
//     use crate::validation::validator::*;

//     struct TestFunctor<'a, T: 'a> {
//         value: Option<T>,
//         phantom: std::marker::PhantomData<&'a T>,
//     }

//     impl<'a, T> Functor<'a> for TestFunctor<'a, T> {
//         type Raw = T;
//         type Target<'s, B> = TestFunctor<'s, B>;

//         fn map<'s, F, B>(self, f: F) -> Self::Target<'s, B>
//         where
//             F: FnMut(Self::Raw) -> B,
//         {
//             TestFunctor {
//                 value: self.value.map(f),
//                 phantom: std::marker::PhantomData,
//             }
//         }
//     }

//     #[test]
//     fn test_functor() {
//         let mut rng = rand::thread_rng();

//         for _ in 0..100 {
//             let x: i32 = rng.gen();
//             let functor = TestFunctor {
//                 value: Some(x),
//                 phantom: std::marker::PhantomData,
//             };
//             let result = functor.map(|x| x.to_string());
//             assert_eq!(result.value, Some(x.to_string()));
//         }
//     }
// }