# Strustegy

**Static, type-directed strategy composition for Rust.**

Strustegy is an experimental library for assembling strategies whose structure and output types are checked at compile time. Its small heterogeneous-list core supports type-indexed transformation pipelines, borrowed and mutable views through GATs, compile-time indexing, and policy-backed validation that returns proof-carrying values.

The crate is intentionally focused. It is not a general-purpose functional-programming toolbox and does not attempt to replace dynamic collections.

## Design goals

- Stable Rust with no `unsafe` code.
- Static dispatch and no required heap allocation in the HList or strategy core.
- Different strategy implementations for different input types.
- Composable strategies through `.then(...)`.
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

The initial crate is organized around five layers:

- `hlist`: heterogeneous structure, borrowing, mutation, and static indexing.
- `strategy`: type-indexed strategies and composition.
- `pipeline`: applying strategies across HLists.
- `validate`: rule lists, policies, fail-fast and accumulating validation.
- `proof`: non-forgeable validated values.

## Status

This is an early `0.1` foundation. The API is expected to evolve before stabilization.

## License

MIT
