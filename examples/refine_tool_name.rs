use strustegy::prelude::*;

pub enum ToolNameProof {}

impl ProofPolicy<str> for ToolNameProof {
    type Refiners = hlist_ty![TrimmedAsciiIdentifier, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![TrimmedAsciiIdentifier, ByteLen]
    }
}

fn main() -> Result<(), ValidationError> {
    let input = String::from("  sync_status  ");
    let witnessed = prove::<ToolNameProof, _>(input.as_str())?;
    let evidence = witnessed.evidence();

    println!("original bytes: {}", evidence.tail.head);
    println!("refined view: {}", evidence.head);

    Ok(())
}
