# Strustegy

**Static, type-directed strategy composition for Rust.**

Strustegy is an experimental library for assembling strategies whose structure and output types are checked at compile time. Its small heterogeneous-list core supports type-indexed transformation pipelines, borrowed and mutable views through GATs, compile-time indexing, and policy-backed validation that returns proof-carrying values.

The crate is intentionally focused. It is not a general-purpose functional-programming toolbox and does not attempt to replace dynamic collections.

## Design goals

- Stable Rust with no `unsafe` code.
- Static dispatch and no required heap allocation in the HList or strategy core.
- Different strategy implementations for different input types.
- Composable synchronous and asynchronous strategies.
- Zero-copy refinements whose evidence may borrow from input through GATs.
- Validation policies fixed by a policy type rather than selected ad hoc by callers.
- `Validated<T, Policy>` values whose constructors are not publicly forgeable.

## Strategy composition

```rust
use strustegy::prelude::*;

#[derive(Clone, Copy)]
struct Widen;

impl Strategy<i32> for Widen {
    type Output = i64;

    fn apply(&self, input: i32) -> Self::Output {
        i64::from(input)
    }
}

#[derive(Clone, Copy)]
struct Describe;

impl Strategy<i64> for Describe {
    type Output = String;

    fn apply(&self, input: i64) -> Self::Output {
        format!("value:{input}")
    }
}

let values = hlist![7_i32];
let pipeline = Widen.then(Describe);
let result = values.hmap(&pipeline);

assert_eq!(result, hlist![String::from("value:7")]);
```

## Closure-backed strategies

Named strategy types remain useful for public APIs, while `strategy_fn` adapts
ordinary functions and shared closures:

```rust
use strustegy::prelude::*;

let pipeline = strategy_fn(|value: i32| i64::from(value))
    .then(strategy_fn(|value: i64| format!("value:{value}")));

assert_eq!(pipeline.apply(7), "value:7");
```

## Zero-copy refinement and proof evidence

`Refine::Output<'input>` is a GAT. A refiner may return a view tied to the
borrowed input, while a `ProofPolicy` fixes an HList of refiners and computes a
matching heterogeneous HList of evidence.

```rust
use strustegy::prelude::*;

pub enum ToolNameProof {}

impl ProofPolicy<str> for ToolNameProof {
    type Refiners = hlist_ty![TrimmedAsciiIdentifier, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![TrimmedAsciiIdentifier, ByteLen]
    }
}

let input = String::from("  sync_status  ");
let witnessed = prove::<ToolNameProof, _>(input.as_str())?;
let evidence = witnessed.evidence();

assert_eq!(evidence.head, "sync_status");
assert_eq!(evidence.tail.head, input.len());
# Ok::<(), ValidationError>(())
```

`Witnessed` keeps the original input borrowed and redacts its `Debug` output.
The evidence cannot outlive the input from which it was derived.

## Asynchronous strategies

`AsyncFnStrategy` accepts Rust 2024 async closures. `AsyncCompose` composes them
without boxing, allocation, or a required executor dependency. Synchronous
strategies can be lifted with `into_async`.

```rust,no_run
use strustegy::prelude::*;

async fn example() {
    let pipeline = async_strategy_fn(async |value: i32| value + 1)
        .then_async(async_strategy_fn(async |value: i32| value * 2));

    assert_eq!(pipeline.apply_async(20).await, 42);
}
```

## Policy-backed validation

A policy fixes its rule list through an associated type. Callers cannot substitute an empty or weaker rule list while requesting proof for that policy.

```rust
use strustegy::prelude::*;

pub enum ToolNamePolicy {}

impl Policy<String> for ToolNamePolicy {
    type Rules = hlist_ty![NonEmpty, MaxBytes<64>, AsciiIdentifier];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxBytes::<64>, AsciiIdentifier]
    }
}

let name: Validated<String, ToolNamePolicy> =
    validate_all::<ToolNamePolicy, _>(String::from("sync_status"))?;

assert_eq!(name.get(), "sync_status");
# Ok::<(), ValidationErrors>(())
```

Validation is not authorization, and a successful proof only represents the documented policy. Security-sensitive consumers should keep policy types and trusted entry points narrowly scoped.

## Planned direction

The crate is organized around eight focused layers:

- `hlist`: heterogeneous structure, borrowing, mutation, and static indexing.
- `strategy`: type-indexed strategies and synchronous composition.
- `fn_strategy`: adapters for ordinary functions and shared closures.
- `async_strategy`: async-closure adapters, static composition, and sync lifting.
- `pipeline`: applying strategies across HLists.
- `refine`: GAT-backed zero-copy refinement and heterogeneous evidence.
- `validate`: rule lists, policies, fail-fast and accumulating validation.
- `proof`: non-forgeable validated and witnessed values.

## Status

This is an early `0.1` foundation. The API is expected to evolve before stabilization.

## License

MIT
