#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![doc = include_str!("../README.md")]

pub mod async_strategy;
pub mod fn_strategy;
pub mod hlist;
pub mod pipeline;
pub mod proof;
pub mod refine;
pub mod strategy;
pub mod validate;

pub use async_strategy::{
    AsyncCompose, AsyncFnStrategy, AsyncStrategy, AsyncStrategyExt, IntoAsync, async_strategy_fn,
    into_async,
};
pub use fn_strategy::{FnStrategy, strategy_fn};
pub use hlist::{Get, GetExt, HCons, HList, HNil, Here, NonEmptyHList, There};
pub use pipeline::{HMap, HMapRefExt};
pub use proof::{Validated, Witnessed};
pub use refine::refiners::{ByteLen, Trimmed, TrimmedAsciiIdentifier, TrimmedNonEmpty, Utf8};
pub use refine::{ProofPolicy, Prove, Refine, prove};
pub use strategy::{Compose, Identity, Strategy, StrategyExt};
pub use validate::rules::{AsciiIdentifier, MaxBytes, NonEmpty};
pub use validate::{
    Policy, Rule, ValidateRules, ValidationError, ValidationErrors, validate_all, validate_first,
};

/// Commonly used Strustegy traits, types, rules, and macros.
pub mod prelude {
    pub use crate::async_strategy::{
        AsyncCompose, AsyncFnStrategy, AsyncStrategy, AsyncStrategyExt, IntoAsync,
        async_strategy_fn, into_async,
    };
    pub use crate::fn_strategy::{FnStrategy, strategy_fn};
    pub use crate::hlist::{Get, GetExt, HCons, HList, HNil, Here, NonEmptyHList, There};
    pub use crate::pipeline::{HMap, HMapRefExt};
    pub use crate::proof::{Validated, Witnessed};
    pub use crate::refine::refiners::{
        ByteLen, Trimmed, TrimmedAsciiIdentifier, TrimmedNonEmpty, Utf8,
    };
    pub use crate::refine::{ProofPolicy, Prove, Refine, prove};
    pub use crate::strategy::{Compose, Identity, Strategy, StrategyExt};
    pub use crate::validate::rules::{AsciiIdentifier, MaxBytes, NonEmpty};
    pub use crate::validate::{
        Policy, Rule, ValidateRules, ValidationError, ValidationErrors, validate_all,
        validate_first,
    };
    pub use crate::{hlist, hlist_ty};
}
