//! Closure and function adapters for synchronous strategies.

use crate::strategy::Strategy;

/// Adapt a shared closure or function into a [`Strategy`].
///
/// The wrapped callable must implement [`Fn`], matching `Strategy::apply`'s
/// shared receiver. Closures that require mutable access to captured state are
/// intentionally excluded from this adapter.
#[derive(Debug, Clone, Copy)]
pub struct FnStrategy<F> {
    function: F,
}

impl<F> FnStrategy<F> {
    /// Create a strategy backed by `function`.
    pub const fn new(function: F) -> Self {
        Self { function }
    }

    /// Borrow the wrapped callable.
    pub const fn get_ref(&self) -> &F {
        &self.function
    }

    /// Consume the adapter and return the wrapped callable.
    pub fn into_inner(self) -> F {
        self.function
    }
}

impl<Input, Output, F> Strategy<Input> for FnStrategy<F>
where
    F: Fn(Input) -> Output,
{
    type Output = Output;

    fn apply(&self, input: Input) -> Self::Output {
        (self.function)(input)
    }
}

/// Construct a [`FnStrategy`] with type inference for the callable.
pub const fn strategy_fn<F>(function: F) -> FnStrategy<F> {
    FnStrategy::new(function)
}
