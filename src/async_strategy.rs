//! Statically dispatched asynchronous strategies and async-closure adapters.

use core::future::{Future, ready};
use core::ops::AsyncFn;

use crate::strategy::Strategy;

/// A shared, statically dispatched asynchronous strategy.
///
/// The returned future may borrow from `self`. The trait deliberately uses
/// return-position `impl Future` instead of boxing or dynamic dispatch.
pub trait AsyncStrategy<Input> {
    type Output;

    fn apply_async(&self, input: Input) -> impl Future<Output = Self::Output>;
}

/// Adapt an async closure or async function into an [`AsyncStrategy`].
#[derive(Debug, Clone, Copy)]
pub struct AsyncFnStrategy<F> {
    function: F,
}

impl<F> AsyncFnStrategy<F> {
    /// Create an asynchronous strategy backed by `function`.
    pub const fn new(function: F) -> Self {
        Self { function }
    }

    /// Borrow the wrapped asynchronous callable.
    pub const fn get_ref(&self) -> &F {
        &self.function
    }

    /// Consume the adapter and return the wrapped callable.
    pub fn into_inner(self) -> F {
        self.function
    }
}

impl<Input, Output, F> AsyncStrategy<Input> for AsyncFnStrategy<F>
where
    F: AsyncFn(Input) -> Output,
{
    type Output = Output;

    async fn apply_async(&self, input: Input) -> Self::Output {
        (self.function)(input).await
    }
}

/// Construct an [`AsyncFnStrategy`] with type inference for the callable.
pub const fn async_strategy_fn<F>(function: F) -> AsyncFnStrategy<F> {
    AsyncFnStrategy::new(function)
}

/// Static composition of two asynchronous strategies.
#[derive(Debug, Clone, Copy)]
pub struct AsyncCompose<F, G> {
    pub first: F,
    pub second: G,
}

impl<F, G> AsyncCompose<F, G> {
    pub const fn new(first: F, second: G) -> Self {
        Self { first, second }
    }
}

impl<Input, F, G> AsyncStrategy<Input> for AsyncCompose<F, G>
where
    F: AsyncStrategy<Input>,
    G: AsyncStrategy<<F as AsyncStrategy<Input>>::Output>,
{
    type Output = <G as AsyncStrategy<<F as AsyncStrategy<Input>>::Output>>::Output;

    async fn apply_async(&self, input: Input) -> Self::Output {
        let intermediate = self.first.apply_async(input).await;
        self.second.apply_async(intermediate).await
    }
}

/// Static short-circuiting composition for asynchronous strategies sharing one
/// error type.
#[derive(Debug, Clone, Copy)]
pub struct AsyncAndThen<F, G> {
    pub first: F,
    pub second: G,
}

impl<F, G> AsyncAndThen<F, G> {
    pub const fn new(first: F, second: G) -> Self {
        Self { first, second }
    }
}

impl<Input, Intermediate, Output, Error, F, G> AsyncStrategy<Input> for AsyncAndThen<F, G>
where
    F: AsyncStrategy<Input, Output = Result<Intermediate, Error>>,
    G: AsyncStrategy<Intermediate, Output = Result<Output, Error>>,
{
    type Output = Result<Output, Error>;

    async fn apply_async(&self, input: Input) -> Self::Output {
        let intermediate = self.first.apply_async(input).await?;
        self.second.apply_async(intermediate).await
    }
}

/// Fluent asynchronous strategy composition.
pub trait AsyncStrategyExt: Sized {
    /// Compose two ordinary asynchronous strategies.
    fn then_async<G>(self, next: G) -> AsyncCompose<Self, G> {
        AsyncCompose::new(self, next)
    }

    /// Compose two asynchronous strategies returning `Result<_, E>` and
    /// short-circuit on the first error.
    fn and_then_async<G>(self, next: G) -> AsyncAndThen<Self, G> {
        AsyncAndThen::new(self, next)
    }
}

impl<S> AsyncStrategyExt for S {}

/// Lift an existing synchronous [`Strategy`] into an [`AsyncStrategy`].
#[derive(Debug, Clone, Copy)]
pub struct IntoAsync<S> {
    strategy: S,
}

impl<S> IntoAsync<S> {
    pub const fn new(strategy: S) -> Self {
        Self { strategy }
    }

    pub const fn get_ref(&self) -> &S {
        &self.strategy
    }

    pub fn into_inner(self) -> S {
        self.strategy
    }
}

impl<Input, S> AsyncStrategy<Input> for IntoAsync<S>
where
    S: Strategy<Input>,
{
    type Output = <S as Strategy<Input>>::Output;

    fn apply_async(&self, input: Input) -> impl Future<Output = Self::Output> {
        ready(self.strategy.apply(input))
    }
}

/// Lift a synchronous strategy into the asynchronous strategy world.
pub const fn into_async<S>(strategy: S) -> IntoAsync<S> {
    IntoAsync::new(strategy)
}
