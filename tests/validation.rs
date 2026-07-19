use strustegy::prelude::*;

pub enum ToolNamePolicy {}

impl Policy<String> for ToolNamePolicy {
    type Rules = hlist_ty![NonEmpty, MaxBytes<16>, AsciiIdentifier];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxBytes::<16>, AsciiIdentifier]
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
