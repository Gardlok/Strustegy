use strustegy::prelude::*;

pub enum ToolNamePolicy {}

impl Policy<String> for ToolNamePolicy {
    type Rules = hlist_ty![NonEmpty, MaxBytes<64>, AsciiIdentifier];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxBytes::<64>, AsciiIdentifier]
    }
}

fn main() {
    let candidate = String::from("sync_status");

    match validate_all::<ToolNamePolicy, _>(candidate) {
        Ok(validated) => println!("accepted: {}", validated.get()),
        Err(errors) => eprintln!("rejected: {errors}"),
    }
}
