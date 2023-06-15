// mod benches;

mod validation;



mod tests;
#[cfg(test)]
use tests::*;

// Consider the information below and determine the correct ruleset of higher-kindness as close as we can get using GATs, now stable in Rust. 
// But the exact links are difficult to wrap my head around, I did muster up some data perhaps we can has a correct solution. I want to make my library as flexible as possible, and deal with the fancy ergo's later. Here's what we have: 
// Per component
// Template //
// DESC: Template for a new component in higher-Fn-like validation system
// LIFE&OWNS: (Option<Box<dyn Fn(T) -> Result<T, E>>>), (Result<T, E>), (T)
// IO: (T) -> T or Result<T, E>
// INNARDS:
// (T)
// fn validate
// ph Type constraints
// NB&SB: [T] -> [T] or [Result<T, E>]
//
// ** Body of Code **
//
// End Template //


// graph TB
//   SF["StrategyFn"]
//   St["Strategy"]
//   V["Validator"]
//   S["Scope"]
//   P["Proof"]
//   T["Target"]
//   Va["Validity"]
  
//   V -- "uses" --> S
//   S -- "uses" --> P
//   P -- "uses" --> St
//   V -- "validates" --> T
//   S -- "validates" --> T
//   P -- "validates" --> T
//   St -- "applies to" --> T
//   T -- "is validated by" --> V
//   T -- "Transforms to" --> V,S,P,St,T,Va
//   Va -- "Transforms to" --> V,S,P,St,T,Va
  
//   SF -- "Defines a specific way to validate a target" --> T
//   St -- "Wraps a Strategy and allows it to be used as a function" --> S
//   S -- "Defines a scope for validation" --> P
//   S -- "Defines a scope for validation" --> T
//   T -- "The object to be validated" --> Va
//   P -- "Defines a proof of validity for a target" --> S
//   P -- "Defines a proof of validity for a target" --> T
//   V -- "Validates a target using a scope" --> T








// Ok, so we have a few things to consider here. 
// 1. We have a target, which is the object to be validated.
// 2. We have a scope, which is the set of rules that the target must follow.
// 3. We have a proof, which is the set of rules that the scope must follow.
// 4. We have a validator, which is the set of rules that the target must follow, and the set of rules that the scope must follow.
// 5. We have a strategy, which is the set of rules that the target must follow, and the set of rules that the scope must follow, 
// and the set of rules that the proof must follow.
// 6. We have a strategy function, which is the set of rules that the target must follow, and the set of rules that the scope must
//  follow, and the set of rules that the proof must follow, and the set of rules that the strategy must follow.
// 7. We have a validity, which is the set of rules that the target must follow, and the set of rules that the scope must follow, 
// and the set of rules that the proof must follow, and the set of rules that the strategy must follow, and the set of rules that
// the validator must follow.
// 8. We have a validity function, which is the set of rules that the target must follow, and the set of rules that the scope must
// follow, and the set of rules that the proof must follow, and the set of rules that the strategy must follow, and the set of rules
// that the validator must follow, and the set of rules that the validity must follow.

// Here is the Mermaid MD diagram for the above information:
// graph TB
//   SF["StrategyFn"]
//   St["Strategy"]
//   V["Validator"]
//   S["Scope"]
//   P["Proof"]
//   T["Target"]
//   Va["Validity"]

//   V -- "uses" --> S
//   S -- "uses" --> P
//   P -- "uses" --> St
//   V -- "validates" --> T
//   S -- "validates" --> T
//   P -- "validates" --> T
//   St -- "applies to" --> T
//   T -- "is validated by" --> V
//   T -- "Transforms to" --> V,S,P,St,T,Va
//   Va -- "Transforms to" --> V,S,P,St,T,Va

//   SF -- "Defines a specific way to validate a target" --> T
//   St -- "Wraps a Strategy and allows it to be used as a function" --> S
//   S -- "Defines a scope for validation" --> P
//   S -- "Defines a scope for validation" --> T
//   T -- "The object to be validated" --> Va
//   P -- "Defines a proof of validity for a target" --> S
//   P -- "Defines a proof of validity for a target" --> T
//   V -- "Validates a target using a scope" --> T

// Nice!!!!

// Whats next? Well, we need to define the rules for each of these components.
// We can do this by defining the rules for each component, and then defining the rules for each component in terms of the rules
// for each component.
// Starting with? The target. The target is the object to be validated. It is the object that we want to validate. It is the object
use crate::validation::*;
use crate::validation::validity::Validity;
use crate::strategies::generic::{GenericProof, GenericScope, GenericStrategy};


// 













#[cfg(test)]
fn main() {

    use crate::validation::validity::Validity;



    // let validity: Validity<i32> = Validity::new(5);
    // let is_positive = |x: &i32| *x > 0;
    // let validity = validity.map(is_positive);
}