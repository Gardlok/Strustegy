# Strustegy

Strustegy is a small Rust library for building typed processing pipelines.

It combines:

* statically composed strategies;
* heterogeneous lists;
* borrowed refinement;
* policy-based validation;
* named evidence and validated values.

The main idea is simple: start with ordinary input, process it through a known set of steps, and produce values whose types record what happened.

```rust
use strustegy::prelude::*;

let pipeline =
    strategy_fn(|value: i32| i64::from(value))
        .then(strategy_fn(|value: i64| format!("value:{value}")));

assert_eq!(pipeline.apply(7), "value:7");
```

Strustegy uses static dispatch throughout. It does not require a runtime registry, boxed strategies, or a validation framework.

## Features

* Synchronous and asynchronous strategies
* Static strategy composition
* Fallible short-circuiting pipelines
* Function and closure adapters
* Heterogeneous lists
* Static HList indexing
* Borrowed HList views
* GAT-backed borrowed refinement
* Policy-owned validation rules
* Named proof projections
* Non-forgeable validated values
* Redaction-safe built-in diagnostics
* No runtime dependencies
* No unsafe code

## Strategies

A strategy transforms one type into another.

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

assert_eq!(Widen.apply(4), 4_i64);
```

Strategies can be composed with `.then(...)`.

```rust
use strustegy::prelude::*;

let pipeline =
    strategy_fn(|value: i32| value + 1)
        .then(strategy_fn(|value: i32| value * 2));

assert_eq!(pipeline.apply(4), 10);
```

Fallible strategies can be composed with `.and_then(...)`.

```rust
use strustegy::prelude::*;

let parse =
    strategy_fn(|input: &str| input.parse::<u32>().map_err(|_| "invalid number"));

let limit = strategy_fn(|value: u32| {
    if value <= 10 {
        Ok(value)
    } else {
        Err("out of range")
    }
});

let pipeline = parse.and_then(limit);

assert_eq!(pipeline.apply("7"), Ok(7));
assert_eq!(pipeline.apply("20"), Err("out of range"));
```

## Async strategies

`AsyncStrategy` is the asynchronous counterpart to `Strategy`.

Async functions and closures can be adapted with `async_strategy_fn(...)`.

```rust
use strustegy::prelude::*;

# async fn example() {
let fetch = async_strategy_fn(async |value: u32| value + 1);

let result = fetch.apply_async(4).await;

assert_eq!(result, 5);
# }
```

Async strategies compose with `.then_async(...)` and `.and_then_async(...)`.

Synchronous strategies can also be lifted with `into_async(...)`.

Strustegy does not provide an async runtime. The caller chooses the executor.

## HLists

An HList is a list whose elements may have different types.

```rust
use strustegy::prelude::*;

let values = hlist![
    42_u32,
    "hello",
    true,
];
```

The shape of the list is part of its type.

```rust
use strustegy::prelude::*;

type Values = hlist_ty![
    u32,
    &'static str,
    bool,
];
```

HLists support:

* construction with `hlist!`;
* type expressions with `hlist_ty!`;
* pattern matching with `hlist_pat!`;
* static indexing;
* shared and mutable borrowed views;
* mapping strategies over heterogeneous values.

HLists may also contain other HLists or ordinary domain types. They are useful when each stage or field has a different type but the overall structure is known at compile time.

## Borrowed refinement

Refinement examines borrowed input and produces evidence tied to the input lifetime.

```rust
use strustegy::prelude::*;

pub enum ToolNameProof {}

impl ProofPolicy<str> for ToolNameProof {
    type Refiners = hlist_ty![
        TrimmedAsciiIdentifier,
        ByteLen,
    ];

    fn refiners() -> Self::Refiners {
        hlist![
            TrimmedAsciiIdentifier,
            ByteLen,
        ]
    }
}
```

Each refiner chooses its own output type through a generic associated type.

That means evidence may borrow directly from the original input without allocating a new value.

For application code, raw HList evidence can be projected into a named type.

```rust
use strustegy::prelude::*;

# pub enum ToolNameProof {}
#
# impl ProofPolicy<str> for ToolNameProof {
#     type Refiners = hlist_ty![TrimmedAsciiIdentifier, ByteLen];
#
#     fn refiners() -> Self::Refiners {
#         hlist![TrimmedAsciiIdentifier, ByteLen]
#     }
# }
#
pub struct ToolEvidence<'input> {
    pub name: &'input str,
    pub source_bytes: usize,
}

impl ProjectEvidence<str> for ToolNameProof {
    type Output<'input> = ToolEvidence<'input>;

    fn project<'input>(
        _input: &'input str,
        evidence: <Self::Refiners as Prove<str>>::Evidence<'input>,
    ) -> Self::Output<'input> {
        let hlist_pat![name, source_bytes] = evidence;

        ToolEvidence {
            name,
            source_bytes,
        }
    }
}
```

The projected evidence cannot outlive the original input.

## Validation policies

Validation policies own their rule lists.

```rust
use strustegy::prelude::*;

pub enum ToolNamePolicy {}

impl Policy<String> for ToolNamePolicy {
    type Rules = hlist_ty![
        NonEmpty,
        MaxBytes<64>,
        AsciiIdentifier,
    ];

    fn rules() -> Self::Rules {
        hlist![
            NonEmpty,
            MaxBytes::<64>,
            AsciiIdentifier,
        ]
    }
}
```

A value can then be checked against the policy.

```rust
use strustegy::prelude::*;

# pub enum ToolNamePolicy {}
#
# impl Policy<String> for ToolNamePolicy {
#     type Rules = hlist_ty![
#         NonEmpty,
#         MaxBytes<64>,
#         AsciiIdentifier,
#     ];
#
#     fn rules() -> Self::Rules {
#         hlist![NonEmpty, MaxBytes::<64>, AsciiIdentifier]
#     }
# }
#
let name: Validated<String, ToolNamePolicy> =
    validate_all::<ToolNamePolicy, _>("sync_status".to_owned())?;

assert_eq!(name.get(), "sync_status");

# Ok::<(), ValidationErrors>(())
```

`Validated<T, Policy>` means that the value passed that policy when the wrapper was created.

It does not prove:

* authorization;
* permission;
* ownership;
* database state;
* availability;
* uniqueness;
* external freshness;
* successful execution.

See [PROOF_MODEL.md](PROOF_MODEL.md) for the exact guarantees and limitations of `Validated` and `Witnessed`.

## Examples

### Project slug

```bash
cargo run --example project_slug
```

Demonstrates:

* borrowed refinement;
* canonicalization;
* owned validation;
* synchronous and asynchronous composition.

### Request line

```bash
cargo run --example request_line
```

Parses and proves a small request-line format while projecting heterogeneous borrowed evidence into named domain types.

### Nested manifest

```bash
cargo run --example nested_manifest
```

Builds a typed deployment-manifest structure containing:

* ordinary structs and enums;
* nested HLists;
* borrowed evidence;
* validated owned values;
* static indexing;
* nested pattern matching.

The example also shows that HList borrowing is intentionally shallow. Nested groups are borrowed explicitly rather than traversed recursively.

## Crate layout

* `strategy` — synchronous strategies and composition
* `fn_strategy` — synchronous function and closure adapters
* `async_strategy` — asynchronous strategies, adapters, composition, and sync lifting
* `hlist` — heterogeneous lists, borrowing, patterns, and indexing
* `pipeline` — applying strategies across HLists
* `refine` — borrowed refinement and evidence projection
* `validate` — validation rules and policies
* `proof` — validated and witnessed value wrappers

## Design boundaries

Strustegy is intentionally focused.

It is not:

* an authorization system;
* a runtime plugin registry;
* a schema language;
* a serialization framework;
* a general-purpose policy engine;
* an async runtime;
* a replacement for domain-specific application types.

The crate provides static building blocks. Applications remain responsible for deciding what their policies mean and which authority is required for an operation.

## Rust version

Strustegy supports Rust 1.85 and newer.

The crate uses Rust 2024 edition features and forbids unsafe code.

## Performance

Strustegy uses static dispatch and avoids boxing in its strategy core.

Some refinement stages borrow directly from input, while other operations intentionally create owned values.

Benchmark methodology, allocation expectations, binary-size measurements, and compile-depth tests are documented in [BENCHMARKS.md](BENCHMARKS.md).

## Project status

Strustegy is currently an early `0.1` library.

The core design is usable, but the public API may continue to evolve as it is tested in real applications.

## License

MIT
