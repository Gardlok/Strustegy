#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![doc = include_str!("../README.md")]

pub mod hlist;
pub mod pipeline;
pub mod proof;
pub mod strategy;
pub mod validate;

pub use hlist::{Get, GetExt, HCons, HList, HNil, Here, NonEmptyHList, There};
pub use pipeline::{HMap, HMapRefExt};
pub use proof::Validated;
pub use strategy::{Compose, Identity, Strategy, StrategyExt};
pub use validate::rules::{AsciiIdentifier, MaxBytes, NonEmpty};
pub use validate::{
    Policy, Rule, ValidateRules, ValidationError, ValidationErrors, validate_all, validate_first,
};

/// Commonly used Strustegy traits, types, rules, and macros.
pub mod prelude {
    pub use crate::hlist::{Get, GetExt, HCons, HList, HNil, Here, NonEmptyHList, There};
    pub use crate::pipeline::{HMap, HMapRefExt};
    pub use crate::proof::Validated;
    pub use crate::strategy::{Compose, Identity, Strategy, StrategyExt};
    pub use crate::validate::rules::{AsciiIdentifier, MaxBytes, NonEmpty};
    pub use crate::validate::{
        Policy, Rule, ValidateRules, ValidationError, ValidationErrors, validate_all,
        validate_first,
    };
    pub use crate::{hlist, hlist_ty};
}
