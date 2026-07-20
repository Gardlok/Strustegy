#[allow(dead_code)]
#[path = "../examples/request_line/mod.rs"]
mod request_line;

use request_line::policy::parse_request;
use request_line::types::Method;

fn inside(input: &[u8], view: &str) -> bool {
    let input_start = input.as_ptr() as usize;
    let input_end = input_start + input.len();
    let view_start = view.as_ptr() as usize;
    let view_end = view_start + view.len();
    input_start <= view_start && view_end <= input_end
}

#[test]
fn request_projection_returns_named_borrowed_evidence() {
    let input = b"GET /projects/demo STR/1";
    let request = parse_request(input).expect("request should parse");

    assert_eq!(request.method(), Method::Get);
    assert_eq!(request.path().as_str(), "/projects/demo");
    assert_eq!(request.version().as_str(), "STR/1");
    assert_eq!(
        request.segments().iter().collect::<Vec<_>>(),
        ["projects", "demo"]
    );
    assert!(inside(input, request.line()));
    assert!(inside(input, request.path().as_str()));
    assert!(inside(input, request.version().as_str()));
}

#[test]
fn malformed_request_lines_are_rejected_without_echoing_input() {
    let rejected = b"GET /private/../secret STR/1";
    let error = parse_request(rejected).expect_err("invalid path should fail");
    let rejected_text = String::from_utf8_lossy(rejected);

    assert!(!error.to_string().contains(rejected_text.as_ref()));
    assert!(!format!("{error:?}").contains(rejected_text.as_ref()));
}
