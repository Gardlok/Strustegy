//! Proof policies and the end-to-end zero-copy parser.

use strustegy::{
    ByteLen, ProjectEvidence, ProofPolicy, Prove, Utf8, ValidationError, hlist, hlist_pat,
    hlist_ty, prove_projected,
};

use super::grammar::{
    ExactTokenCount, ParsedMethod, ParsedPath, ParsedSegments, ParsedVersion, Token,
};
use super::types::{ProvenRequest, RequestEvidence, Utf8RequestLine};

pub enum Utf8RequestLinePolicy {}

impl ProofPolicy<[u8]> for Utf8RequestLinePolicy {
    type Refiners = hlist_ty![Utf8, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![Utf8, ByteLen]
    }
}

impl ProjectEvidence<[u8]> for Utf8RequestLinePolicy {
    type Output<'input> = Utf8RequestLine<'input>;

    fn project<'input>(
        _input: &'input [u8],
        evidence: <Self::Refiners as Prove<[u8]>>::Evidence<'input>,
    ) -> Self::Output<'input> {
        let hlist_pat![line, byte_len] = evidence;
        Utf8RequestLine { line, byte_len }
    }
}

pub enum RequestLinePolicy {}

impl ProofPolicy<str> for RequestLinePolicy {
    type Refiners = hlist_ty![
        ExactTokenCount<3>,
        Token<0>,
        Token<1>,
        Token<2>,
        ParsedMethod,
        ParsedPath,
        ParsedVersion,
        ParsedSegments,
    ];

    fn refiners() -> Self::Refiners {
        hlist![
            ExactTokenCount::<3>,
            Token::<0>,
            Token::<1>,
            Token::<2>,
            ParsedMethod,
            ParsedPath,
            ParsedVersion,
            ParsedSegments,
        ]
    }
}

impl ProjectEvidence<str> for RequestLinePolicy {
    type Output<'input> = RequestEvidence<'input>;

    fn project<'input>(
        _input: &'input str,
        evidence: <Self::Refiners as Prove<str>>::Evidence<'input>,
    ) -> Self::Output<'input> {
        let hlist_pat![
            token_count,
            raw_method,
            raw_path,
            raw_version,
            method,
            path,
            version,
            segments,
        ] = evidence;

        debug_assert_eq!(raw_method, method.as_str());
        debug_assert_eq!(raw_path, path.as_str());
        debug_assert_eq!(raw_version, version.as_str());

        RequestEvidence {
            token_count,
            method,
            path,
            version,
            segments,
        }
    }
}

/// Prove a request line and return a named domain value borrowing from `input`.
pub fn parse_request(input: &[u8]) -> Result<ProvenRequest<'_>, ValidationError> {
    let utf8 = prove_projected::<Utf8RequestLinePolicy, _>(input)?;
    let evidence = prove_projected::<RequestLinePolicy, _>(utf8.line)?;

    Ok(ProvenRequest::new(utf8.line, utf8.byte_len, evidence))
}
