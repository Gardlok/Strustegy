//! Type-directed strategies and static composition.

/// A shared, type-directed strategy from `Input` to an associated output type.
///
/// Implementations intended to participate in composition laws should behave
/// extensionally: equal inputs should produce equal outputs without observable
/// strategy-state changes.
pub trait Strategy<Input> {
    type Output;

    fn apply(&self, input: Input) -> Self::Output;
}

/// The identity strategy.
#[derive(Debug, Clone, Copy, Default)]
pub struct Identity;

impl<T> Strategy<T> for Identity {
    type Output = T;

    fn apply(&self, input: T) -> Self::Output {
        input
    }
}

/// Static composition of two strategies.
#[derive(Debug, Clone, Copy)]
pub struct Compose<F, G> {
    pub first: F,
    pub second: G,
}

impl<F, G> Compose<F, G> {
    pub const fn new(first: F, second: G) -> Self {
        Self { first, second }
    }
}

impl<Input, F, G> Strategy<Input> for Compose<F, G>
where
    F: Strategy<Input>,
    G: Strategy<<F as Strategy<Input>>::Output>,
{
    type Output = <G as Strategy<<F as Strategy<Input>>::Output>>::Output;

    fn apply(&self, input: Input) -> Self::Output {
        let intermediate = self.first.apply(input);
        self.second.apply(intermediate)
    }
}

/// Fluent strategy composition.
pub trait StrategyExt: Sized {
    fn then<G>(self, next: G) -> Compose<Self, G> {
        Compose::new(self, next)
    }
}

impl<S> StrategyExt for S {}
