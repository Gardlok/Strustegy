# Strustegy boundary patterns

Strustegy is designed to help applications make stable input-boundary facts explicit in Rust types. Its proof wrappers are receipts for specific policy execution, not transferable authority and not permanent statements about mutable or external state.

The exact guarantees and limitations of `Validated` and `Witnessed` are defined in [PROOF_MODEL.md](PROOF_MODEL.md). This document focuses on practical application structure around those guarantees.

## `Validated<T, P>` != authorization

Treat this distinction as a hard boundary:

> `Validated<T, P>` means only that the wrapped value passed `P` when the receipt was constructed.

It does **not** establish authorization, permission, ownership, database existence, availability, external freshness, successful persistence or execution, retry safety, remote effects, or current runtime authority.

For example:

- a valid tool identifier does not prove that the current caller may execute that tool;
- a valid operation identifier does not prove that the operation exists, belongs to the caller, or may be replayed.

Those questions remain application authority and lifecycle concerns. Do not encode them as though a Strustegy validation receipt grants permission.

Likewise, `Witnessed` means only what the selected refiners actually establish for the borrowed input. It does not make external facts durable.

## Pattern 1: Put validated values inside application domain types

A common boundary shape is:

```text
primitive / untrusted value
        ↓
Policy validation
        ↓
Validated<T, Policy>
        ↓
private application-owned domain newtype
```

The Strustegy wrapper records the validation event. The application-owned type decides what operations should be available after that boundary.

```rust
use core::fmt;
use strustegy::prelude::*;

pub enum ProjectKeyPolicy {}

impl Policy<String> for ProjectKeyPolicy {
    type Rules = hlist_ty![NonEmpty, MaxBytes<64>, AsciiIdentifier];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxBytes::<64>, AsciiIdentifier]
    }
}

pub struct ProjectKey {
    value: Validated<String, ProjectKeyPolicy>,
}

impl ProjectKey {
    // Keep construction at the application's trusted boundary rather than
    // exposing a way to manufacture the domain type from unchecked data.
    pub(crate) fn from_untrusted(value: String) -> Result<Self, ValidationErrors> {
        let value = validate_all::<ProjectKeyPolicy, _>(value)?;
        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        self.value.get()
    }

    pub fn into_inner(self) -> String {
        self.value.into_inner()
    }
}

impl fmt::Debug for ProjectKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProjectKey(<redacted>)")
    }
}
```

Depending on the domain, an application type may also implement `AsRef`, `Eq`/`PartialEq`, or domain-specific behavior. Security-sensitive types can keep `Debug` redacted even when their inner primitive would normally print its contents.

This wrapper is application code. Strustegy does not need a macro that generates it, and `0.1` deliberately leaves its public surface and domain meaning under application control.

For long-lived invariants, keep construction private and expose only mutations that preserve the domain guarantee. Remember that `Validated` itself cannot prevent interior mutation in arbitrary `T`; see `PROOF_MODEL.md` for that limitation.

## Pattern 2: Canonicalize before validating canonical form

If a policy is intended to establish that a value is already in canonical representation, normalize first and run that policy against the normalized value:

```text
raw input
    ↓
normalization / canonicalization
    ↓
Policy validation
    ↓
domain value
```

For this example, the application's canonical representation permits only lowercase ASCII letters, digits, and hyphens:

```rust
use strustegy::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
struct CanonicalNameSyntax;

impl Rule<String> for CanonicalNameSyntax {
    fn check(&self, value: &String) -> Result<(), ValidationError> {
        let valid = value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        });

        if valid {
            Ok(())
        } else {
            Err(ValidationError::new(
                "canonical_name_syntax",
                "noncanonical",
            ))
        }
    }
}

enum CanonicalNamePolicy {}

impl Policy<String> for CanonicalNamePolicy {
    type Rules = hlist_ty![NonEmpty, MaxBytes<64>, CanonicalNameSyntax];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxBytes::<64>, CanonicalNameSyntax]
    }
}

fn prepare_name(
    raw: &str,
) -> Result<Validated<String, CanonicalNamePolicy>, ValidationErrors> {
    let canonical = raw.trim().to_ascii_lowercase().replace('_', "-");
    validate_all::<CanonicalNamePolicy, _>(canonical)
}
```

The important ordering is that `CanonicalNamePolicy` inspects the representation the application intends to keep.

Do not hide canonicalization inside a rule whose advertised meaning is "this value is canonical." A rule that silently changes input would blur two different operations: transforming a value and proving a property of the resulting value. Keeping the transformation explicit also makes the ownership/allocation boundary visible.

The `project_slug` and `nested_manifest` examples use this structure: borrowed input is refined, an owned canonical value is created, and the canonical value is then validated.

## Pattern 3: Wire values are ordinary values, not transferable proof authority

Use ordinary stable representations across serialization, IPC, or persistence boundaries:

```text
producer
    ↓
validated application domain value
    ↓
serialize ordinary stable representation

wire / IPC / persistence

deserialize ordinary representation
    ↓
revalidate
    ↓
reconstruct trusted application domain value
```

Assume an application-owned `ProjectSlug` follows Pattern 1 and its `from_untrusted` constructor runs the current Strustegy policy. The wire adapter should move ordinary data, then call that boundary constructor again:

```rust
struct WireProject {
    slug: String,
}

fn to_wire(project: &ProjectSlug) -> WireProject {
    WireProject {
        slug: project.as_str().to_owned(),
    }
}

fn from_wire(wire: WireProject) -> Result<ProjectSlug, ValidationErrors> {
    ProjectSlug::from_untrusted(wire.slug)
}
```

The serialization mechanism is application-owned; Strustegy does not add one.

In particular:

```text
serializing data associated with Validated
!=
serializing transferable proof authority
```

Strustegy intentionally does not implement serialization for `Validated` or `Witnessed`. A receiver should deserialize ordinary data and execute the current policy before reconstructing its trusted application type.

Persisted policy names, versions, timestamps, or rule metadata can be useful historical records of processing. They do not replace executing the current policy and do not turn historical validation into present authority.

## Pattern 4: Validate cells, then enforce aggregate relationships in domain logic

Many useful policies apply independently to one value. Relationships between values, aggregate limits, and lifecycle transitions often belong one layer above them:

```text
validate individual values
        ↓
construct aggregate/domain object
        ↓
check cross-value or lifecycle invariants
```

Examples include:

- `start <= end`;
- unique tags;
- maximum collection count;
- revision progression;
- archive-state transition rules.

For example, separate boundary policies can establish the allowed representation or range of two timestamps, while the aggregate constructor enforces their ordering:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Start(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct End(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DomainError {
    StartAfterEnd,
}

struct Window {
    start: Start,
    end: End,
}

impl Window {
    fn new(start: Start, end: End) -> Result<Self, DomainError> {
        if start.0 > end.0 {
            return Err(DomainError::StartAfterEnd);
        }

        Ok(Self { start, end })
    }
}
```

This does not mean a `Policy<Aggregate>` is always wrong. A stable intrinsic property of an aggregate can be represented that way when it genuinely fits the application's design. The point is to avoid turning every relationship, workflow state, authorization decision, or external-state fact into one giant validation policy merely because individual fields use Strustegy.

Application authority and lifecycle logic stay in the application.

## Application-owned diagnostic projection

Strustegy's built-in validation diagnostics are deliberately redaction-safe. `ValidationError` exposes stable static metadata through `rule()` and `code()`, while `ValidationErrors` provides `first()` and `iter()` for borrowed navigation, `as_slice()` for slice access, and `into_vec()` for owned extraction.

Security-sensitive applications can project that metadata into their own machine-facing taxonomy without echoing rejected input:

```rust
use strustegy::{ValidationError, ValidationErrors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppValidationCode {
    Required,
    TooLong,
    InvalidIdentifier,
    BoundaryRejected,
}

fn project_error(error: ValidationError) -> AppValidationCode {
    match (error.rule(), error.code()) {
        ("non_empty", "empty") => AppValidationCode::Required,
        ("max_bytes", "too_long") => AppValidationCode::TooLong,
        ("ascii_identifier", "invalid_character") => {
            AppValidationCode::InvalidIdentifier
        }
        _ => AppValidationCode::BoundaryRejected,
    }
}

fn project_borrowed(errors: &ValidationErrors) -> Vec<AppValidationCode> {
    errors
        .iter()
        .copied()
        .map(project_error)
        .collect()
}

fn project_owned(errors: ValidationErrors) -> Vec<AppValidationCode> {
    errors.into_vec().into_iter().map(project_error).collect()
}
```

The application owns these codes, their stability policy, and how they are presented externally. Strustegy does not define an application-error taxonomy.

Most importantly, the projection above handles only rule/code metadata. It does not attach, log, or return the rejected value. Preserve that property when adding application context around validation failures.

## Related documentation

- [PROOF_MODEL.md](PROOF_MODEL.md) defines the exact proof semantics, threat model, mutation caveats, and serialization boundary.
- [BENCHMARKS.md](BENCHMARKS.md) records the `0.1.0` performance and compile-depth baseline.
- The `project_slug`, `request_line`, and `nested_manifest` examples demonstrate the current public architecture without adding runtime registries, proc macros, serialization support, or framework-owned authority concepts.
