# Strustegy

**Strustegy turns untrusted boundary input into named, policy-backed Rust values
using static strategy composition and zero-copy borrowed evidence.**

It is an experimental, dependency-free toolkit for trusted boundary processing
on stable Rust. Its HList core computes heterogeneous strategy and proof outputs
at compile time, while named policies and projected evidence let ordinary code
work with domain types instead of recursive `HCons` signatures.

Strustegy is deliberately narrow. It is not a runtime plugin registry, dynamic
validation framework, authorization system, schema engine, Serde replacement,
or general-purpose functional-programming crate.

## Design goals

- Stable Rust 1.85 or newer.
- `#![forbid(unsafe_code)]` throughout the library.
- Static dispatch and no required heap allocation in the HList or strategy core.
- Borrowed refinement evidence tied to the input lifetime through GATs.
- Policy types that fix their rule or refiner lists.
- Named projections for everyday domain code while retaining raw HLists for
  advanced use.
- Redaction-safe built-in proof and validation diagnostics.

The exact semantics and limitations of `Validated` and `Witnessed` are defined
in [PROOF_MODEL.md](PROOF_MODEL.md). In particular, validation is not
authorization, and `Validated<T, Policy>` records that `T` passed the policy when
the wrapper was created; arbitrary interior-mutable values can later change.

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

let pipeline = Widen.then(strategy_fn(|value: i64| format!("value:{value}")));
assert_eq!(pipeline.apply(7), "value:7");
```

Fallible strategies compose with `.and_then(...)`, so the second strategy
receives only the successful value:

```rust
use strustegy::prelude::*;

let parse = strategy_fn(|input: &str| input.parse::<u32>().map_err(|_| "parse"));
let bound = strategy_fn(|value: u32| {
    if value <= 10 { Ok(value * 2) } else { Err("range") }
});

let pipeline = parse.and_then(bound);
assert_eq!(pipeline.apply("4"), Ok(8));
assert_eq!(pipeline.apply("20"), Err("range"));
```

The asynchronous equivalent is `.and_then_async(...)`. Both forms remain
statically dispatched and do not require a boxed future or executor dependency.

## Zero-copy refinement and named evidence

`Refine::Output<'input>` is a GAT. Each refiner chooses an output family tied to
the borrowed input, while `Prove::Evidence<'input>` recursively assembles the
policy's heterogeneous evidence HList.

Advanced code can retain that raw evidence through `prove`. Everyday boundaries
can implement `ProjectEvidence` and call `prove_projected` to return a named
structure:

```rust
use strustegy::prelude::*;

pub enum ToolNameProof {}

impl ProofPolicy<str> for ToolNameProof {
    type Refiners = hlist_ty![TrimmedAsciiIdentifier, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![TrimmedAsciiIdentifier, ByteLen]
    }
}

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
        ToolEvidence { name, source_bytes }
    }
}

let input = String::from("  sync_status  ");
let evidence = prove_projected::<ToolNameProof, _>(input.as_str())?;
assert_eq!(evidence.name, "sync_status");
assert_eq!(evidence.source_bytes, input.len());
# Ok::<(), ValidationError>(())
```

The projection output cannot outlive the input. The request-line example uses
this mechanism to keep HList destructuring inside the policy while its parser
returns `ProvenRequest<'input>`.

## Policy-backed validation

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

A policy fixes its exact rule-list type. Callers cannot request the same policy
marker while substituting an empty or weaker list. The policy implementation
itself still has to be correct, and temporal facts such as availability or
permission require an operation-scoped authority mechanism.

## Examples

- `cargo run --example project_slug` demonstrates borrowed proof stages,
  canonicalization into an owned slug, policy validation, and an async
  availability check composed with `.and_then_async(...)`.
- `cargo run --example request_line` proves a small STR/1 request line and
  projects raw evidence into named borrowed domain types.
## Performance

The lightweight benchmark harness, allocation expectations, binary-size checks,
and compile-depth measurements are documented in
[BENCHMARKS.md](BENCHMARKS.md). Static dispatch does not by itself make an
operation zero-cost; the report distinguishes borrowed stages from stages that
create owned values.

## Architecture

- `hlist`: heterogeneous structure, borrowing, mutation, and static indexing.
- `strategy`: synchronous composition, including fallible composition.
- `fn_strategy`: adapters for functions and shared closures.
- `async_strategy`: async-closure adapters, static composition, and sync lifting.
- `pipeline`: applying strategies across HLists.
- `refine`: GAT-backed borrowed refinement, proof evidence, and named projection.
- `validate`: rule lists, policies, fail-fast and accumulating validation.
- `proof`: non-public proof construction and redacted wrappers.

## Status

This is an early `0.1` foundation. The API is expected to evolve before
stabilization.

## License

MIT
