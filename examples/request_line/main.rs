//! Zero-copy STR/1 request-line parsing with GAT-computed evidence.

mod grammar;
mod policy;
mod route;
mod types;

use strustegy::ValidationError;

use policy::parse_request;
use route::{dispatch, recognize};

fn borrowed_from(input: &[u8], view: &str) -> bool {
    let input_start = input.as_ptr() as usize;
    let input_end = input_start + input.len();
    let view_start = view.as_ptr() as usize;
    let view_end = view_start + view.len();

    input_start <= view_start && view_end <= input_end
}

fn print_segments(request: types::ProvenRequest<'_>) {
    print!("segments: [");

    for (index, segment) in request.segments().iter().enumerate() {
        if index > 0 {
            print!(", ");
        }

        print!("{segment:?}");
    }

    println!("]");
}

fn main() -> Result<(), ValidationError> {
    let input = b"GET /projects/rose STR/1";
    let request = parse_request(input)?;

    println!("input: {}", request.line());
    println!("bytes: {}", request.byte_len());
    println!("tokens: {}", request.token_count());
    println!("method: {}", request.method());
    println!("path: {}", request.path());
    println!("version: {}", request.version());
    println!("path segments: {}", request.segments().count());
    print_segments(request);

    assert!(borrowed_from(input, request.line()));
    assert!(borrowed_from(input, request.path().as_str()));
    assert!(borrowed_from(input, request.version().as_str()));

    println!("zero-copy: line, path, version, and segments borrow the input buffer");
    dispatch(recognize(request));

    Ok(())
}
