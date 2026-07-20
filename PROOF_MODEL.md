# Strustegy proof model

Strustegy uses Rust types to record that a value crossed a specific, statically
selected boundary policy. These types are practical validation receipts. They
are not formal proofs, authorization grants, or permanent statements about
mutable or external state.

## `Witnessed`

`Witnessed<'input, Input, Policy, Evidence>` means that:

- a borrowed `Input` was processed through `Policy`;
- `Evidence` was produced by the exact refiner HList selected by that policy;
- any borrowed evidence is tied to `'input` and cannot outlive the input;
- the evidence may mix borrowed views and owned values;
- the witness covers only the properties actually implemented by those
  refiners.

A `Witnessed` value does **not** establish authorization, trust in a database or
service, freshness of external state, or formal verification of either the
policy or its implementation.

The wrapper's constructor is not public. Safe downstream code therefore cannot
construct one without using Strustegy's proof entry points. The original input
and evidence can still be consumed through the public accessors, so trusted
code must continue to respect the documented meaning of the selected policy.

## `Validated`

The intended model is:

> `Validated<T, Policy>` means that the wrapped value passed `Policy` when the
> wrapper was created.

It is a validation receipt, not a universal promise that an arbitrary `T`
continues to satisfy the policy forever.

### Mutation and cloning

A generic wrapper cannot prevent the value from changing through interior
mutability. For example, a `Cell`, `RefCell`, `Mutex`, atomic, or a custom type
with internal mutable state can change after validation even when only `&T` is
available. A custom `Clone` implementation can also produce a value whose state
differs from the original in ways the policy did not inspect.

`Validated` therefore retains its conditional `Clone` implementation. Cloning
copies the receipt along with `T::clone`; it does not rerun the policy. Consumers
must not treat `Clone` as revalidation.

For durable domain invariants, prefer a dedicated domain type with:

- private fields and constructors;
- an immutable representation where practical;
- narrowly scoped mutation methods that preserve the invariant;
- explicit revalidation after any operation that can invalidate it.

A domain newtype can contain a `Validated` value, but it should expose only the
operations consistent with the domain guarantee it wants to preserve.

## Validation versus authority

Strustegy is strongest for stable properties of the input itself.

| Stable input property | Temporal or authority property |
| --- | --- |
| valid UTF-8 | currently available |
| canonical slug syntax | authorized for this caller |
| maximum byte length | exists in the database now |
| fixed command grammar | permission is still granted |

Stable properties can usually be represented by borrowed evidence or a
validated domain value. Temporal or authority facts can become stale immediately
after a check. They normally require an operation-scoped mechanism such as:

- a reservation;
- a lease;
- a transaction;
- a versioned snapshot;
- a capability tied to the operation that consumes it.

A proof that a project slug is syntactically canonical is durable while the
bytes remain unchanged. A check that the slug is available is not durable unless
the surrounding system reserves it or completes the write atomically.

## Policy identity and rule lists

A `Policy` or `ProofPolicy` chooses its rule or refiner list through an associated
type. Callers cannot ask for the same policy marker while substituting a weaker
list. Rust coherence also prevents a downstream crate from replacing an
implementation owned by another crate.

This does not make a policy correct. The policy owner can choose incomplete
rules, implement a refiner incorrectly, or attach misleading documentation.
The semantic meaning of a proof is exactly the behavior of the selected
implementation, no more.

## Threat model

Strustegy is designed to provide the following protections in safe Rust:

- proof wrappers cannot be forged through their public constructors because the
  constructors are crate-private;
- callers cannot bypass a policy's fixed rule or refiner list while requesting
  that policy's proof type;
- borrowed evidence cannot be extended beyond the input lifetime through the
  public safe API;
- diagnostics produced by built-in proof and validation wrappers do not echo
  rejected input.

Strustegy does not protect against:

- interior mutation after validation;
- incorrect policy, rule, or refiner implementations;
- stale decisions based on external state;
- logic bugs in user-defined strategies;
- unsafe code in downstream crates that violates Rust's guarantees;
- serialization formats that label untrusted data as proof-like values;
- reconstruction of trusted domain types without rerunning their boundary
  policy.

Strustegy does not implement serialization for `Validated` or `Witnessed`.
Applications that serialize related domain values should serialize the ordinary
data and revalidate it when reconstructing a trusted boundary type. Persisted
metadata may identify which policy was used historically, but it is not a
substitute for executing the current policy.

## Practical interpretation

Treat Strustegy proof types as typed boundary receipts:

1. untrusted data enters through a narrow parser, rule set, or refiner policy;
2. the policy produces borrowed evidence or an owned validated value;
3. named domain code consumes that result;
4. authority checks, transactions, and mutable lifecycle rules remain the
   responsibility of the surrounding application.
