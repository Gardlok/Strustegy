use strustegy::prelude::*;

pub enum ToolNamePolicy {}

impl Policy<String> for ToolNamePolicy {
    type Rules = hlist_ty![NonEmpty, MaxBytes<16>, AsciiIdentifier];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxBytes::<16>, AsciiIdentifier]
    }
}

pub enum UnicodeScalarPolicy {}

impl Policy<String> for UnicodeScalarPolicy {
    type Rules = hlist_ty![NonEmpty, MaxUnicodeScalars<3>];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxUnicodeScalars::<3>]
    }
}

pub enum RefreshPolicy {}

impl Policy<u64> for RefreshPolicy {
    type Rules = hlist_ty![InclusiveU64<1, 60_000>];

    fn rules() -> Self::Rules {
        hlist![InclusiveU64::<1, 60_000>]
    }
}

#[test]
fn valid_input_produces_a_policy_typed_proof() {
    let validated: Validated<String, ToolNamePolicy> =
        validate_all::<ToolNamePolicy, _>(String::from("sync_status")).unwrap();

    assert_eq!(validated.get(), "sync_status");
    assert_eq!(validated.into_inner(), "sync_status");
}

#[test]
fn validate_with_produces_an_owned_policy_typed_proof_without_a_policy_value() {
    fn assert_copy<T: Copy>() {}
    fn assert_clone<T: Clone>() {}
    fn assert_default<T: Default>() {}

    assert_copy::<ValidateWith<ToolNamePolicy>>();
    assert_clone::<ValidateWith<ToolNamePolicy>>();
    assert_default::<ValidateWith<ToolNamePolicy>>();

    let strategy = ValidateWith::<ToolNamePolicy>::new();
    assert_eq!(format!("{strategy:?}"), "ValidateWith");

    let validated: Validated<String, ToolNamePolicy> =
        strategy.apply(String::from("sync_status")).unwrap();

    assert_eq!(validated.get(), "sync_status");
    assert_eq!(validated.into_inner(), "sync_status");
}

#[test]
fn validate_with_preserves_validate_all_error_order_metadata_and_redaction() {
    let rejected = "this name is far too long!";
    let via_strategy = ValidateWith::<ToolNamePolicy>::new()
        .apply(String::from(rejected))
        .unwrap_err();
    let direct = validate_all::<ToolNamePolicy, _>(String::from(rejected)).unwrap_err();

    assert_eq!(via_strategy, direct);
    assert_eq!(via_strategy.len(), 2);
    assert_eq!(via_strategy.as_slice()[0].rule(), "max_bytes");
    assert_eq!(via_strategy.as_slice()[0].code(), "too_long");
    assert_eq!(via_strategy.as_slice()[1].rule(), "ascii_identifier");
    assert_eq!(
        via_strategy.as_slice()[1].code(),
        "invalid_character"
    );
    assert!(!via_strategy.to_string().contains(rejected));
    assert!(!format!("{via_strategy:?}").contains(rejected));
}

#[test]
fn validate_with_composes_after_infallible_canonicalization() {
    fn canonicalize(value: &str) -> String {
        value.trim().to_ascii_lowercase()
    }

    let pipeline = strategy_fn(canonicalize).then(ValidateWith::<ToolNamePolicy>::new());

    let validated = pipeline
        .apply("  SYNC_STATUS  ")
        .expect("canonicalized tool name should validate");
    assert_eq!(validated.get(), "sync_status");

    let rejected = "  THIS NAME IS FAR TOO LONG!  ";
    let via_pipeline = pipeline.apply(rejected).unwrap_err();
    let direct = validate_all::<ToolNamePolicy, _>(canonicalize(rejected)).unwrap_err();
    assert_eq!(via_pipeline, direct);
}

#[test]
fn fail_fast_reports_the_first_policy_rule() {
    let error = validate_first::<ToolNamePolicy, _>(String::new()).unwrap_err();

    assert_eq!(error.rule(), "non_empty");
    assert_eq!(error.code(), "empty");
}

#[test]
fn accumulating_validation_reports_each_failed_rule_once() {
    let errors =
        validate_all::<ToolNamePolicy, _>(String::from("this name is far too long!")).unwrap_err();

    assert_eq!(errors.len(), 2);
    assert_eq!(errors.as_slice()[0].rule(), "max_bytes");
    assert_eq!(errors.as_slice()[1].rule(), "ascii_identifier");
}

#[test]
fn validation_errors_first_returns_first_failure_without_consuming_collection() {
    let rejected = "this name is far too long!";
    let errors = validate_all::<ToolNamePolicy, _>(String::from(rejected)).unwrap_err();

    {
        let first = errors.first().expect("multi-error policy must fail");
        assert_eq!(first.rule(), "max_bytes");
        assert_eq!(first.code(), "too_long");
        assert_eq!(Some(first), errors.as_slice().first());
        assert!(!format!("{first:?}").contains(rejected));
    }

    assert_eq!(errors.len(), 2);
    assert_eq!(errors.as_slice()[1].rule(), "ascii_identifier");
    assert_eq!(
        errors.iter().next().map(ValidationError::rule),
        Some("max_bytes")
    );
}

#[test]
fn validation_errors_iter_projects_metadata_in_rule_order() {
    let errors =
        validate_all::<ToolNamePolicy, _>(String::from("this name is far too long!")).unwrap_err();

    let projected: Vec<_> = errors
        .iter()
        .map(|error| (error.rule(), error.code()))
        .collect();
    assert_eq!(
        projected,
        vec![
            ("max_bytes", "too_long"),
            ("ascii_identifier", "invalid_character"),
        ]
    );

    let slice_projection: Vec<_> = errors
        .as_slice()
        .iter()
        .map(|error| (error.rule(), error.code()))
        .collect();
    assert_eq!(projected, slice_projection);
    assert_eq!(errors.len(), 2);

    let owned = errors.into_vec();
    let owned_projection: Vec<_> = owned
        .iter()
        .map(|error| (error.rule(), error.code()))
        .collect();
    assert_eq!(projected, owned_projection);
}

#[test]
fn validation_errors_do_not_echo_rejected_input() {
    let rejected = "secret value with spaces";
    let errors = validate_all::<ToolNamePolicy, _>(String::from(rejected)).unwrap_err();

    assert!(!errors.to_string().contains(rejected));
    assert!(!format!("{errors:?}").contains(rejected));
}

#[test]
fn validated_debug_output_is_redacted() {
    let validated = validate_all::<ToolNamePolicy, _>(String::from("sync_status")).unwrap();

    assert_eq!(format!("{validated:?}"), "Validated(<redacted>)");
}

#[test]
fn max_unicode_scalars_accepts_empty_without_replacing_non_empty() {
    let empty = String::new();

    assert!(MaxUnicodeScalars::<0>.check(&empty).is_ok());
    assert!(NonEmpty.check(&empty).is_err());
}

#[test]
fn max_unicode_scalars_enforces_scalar_count_not_byte_count() {
    assert!(MaxUnicodeScalars::<3>.check(&"abc").is_ok());

    let rejected = "abcd";
    let error = MaxUnicodeScalars::<3>.check(&rejected).unwrap_err();
    assert_eq!(error.rule(), "max_unicode_scalars");
    assert_eq!(error.code(), "too_long");
    assert!(!error.to_string().contains(rejected));
    assert!(!format!("{error:?}").contains(rejected));

    let roses = "🌹🌹🌹";
    assert_eq!(roses.chars().count(), 3);
    assert_eq!(roses.len(), 12);
    assert!(MaxUnicodeScalars::<3>.check(&roses).is_ok());
    assert!(MaxBytes::<3>.check(&roses).is_err());
}

#[test]
fn max_unicode_scalars_composes_in_a_policy_hlist() {
    let validated =
        validate_all::<UnicodeScalarPolicy, _>(String::from("🌹ab")).expect("three scalars pass");
    assert_eq!(validated.get(), "🌹ab");

    let errors = validate_all::<UnicodeScalarPolicy, _>(String::from("🌹abc")).unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors.as_slice()[0].rule(), "max_unicode_scalars");
}

#[test]
fn inclusive_u64_accepts_closed_interval_boundaries_and_interior() {
    for value in [1, 30_000, 60_000] {
        assert!(InclusiveU64::<1, 60_000>.check(&value).is_ok());
    }

    assert!(InclusiveU64::<0, 0>.check(&0).is_ok());
}

#[test]
fn inclusive_u64_rejects_outside_values_with_redacted_metadata() {
    for rejected in [0, 60_001] {
        let error = InclusiveU64::<1, 60_000>.check(&rejected).unwrap_err();
        let rejected = rejected.to_string();
        assert_eq!(error.rule(), "inclusive_u64");
        assert_eq!(error.code(), "out_of_range");
        assert!(!error.to_string().contains(rejected.as_str()));
        assert!(!format!("{error:?}").contains(rejected.as_str()));
    }

    assert!(InclusiveU64::<5, 4>.check(&5).is_err());
}

#[test]
fn inclusive_u64_composes_in_a_policy_hlist() {
    let validated = validate_all::<RefreshPolicy, _>(30_000).expect("interior value should pass");
    assert_eq!(*validated.get(), 30_000);

    let errors = validate_all::<RefreshPolicy, _>(60_001).unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors.as_slice()[0].rule(), "inclusive_u64");
    assert_eq!(errors.as_slice()[0].code(), "out_of_range");
}
