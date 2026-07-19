//! Proof policies and the end-to-end zero-copy parser.

use strustegy::{ByteLen, HCons, HNil, ProofPolicy, Utf8, ValidationError, hlist, hlist_ty, prove};

use crate::grammar::{
    ExactTokenCount, ParsedMethod, ParsedPath, ParsedSegments, ParsedVersion, Token,
};
use crate::types::{ProvenRequest, RequestParts};

pub enum Utf8RequestLinePolicy {}

impl ProofPolicy<[u8]> for Utf8RequestLinePolicy {
    type Refiners = hlist_ty![Utf8, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![Utf8, ByteLen]
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

/// Prove a request line and return a domain value borrowing from `input`.
pub fn parse_request<'input>(
    input: &'input [u8],
) -> Result<ProvenRequest<'input>, ValidationError> {
    let HCons {
        head: line,
        tail: HCons {
            head: byte_len,
            tail: HNil,
        },
    } = prove::<Utf8RequestLinePolicy, _>(input)?.into_evidence();

    let HCons {
        head: token_count,
        tail:
            HCons {
                head: raw_method,
                tail:
                    HCons {
                        head: raw_path,
                        tail:
                            HCons {
                                head: raw_version,
                                tail:
                                    HCons {
                                        head: method,
                                        tail:
                                            HCons {
                                                head: path,
                                                tail:
                                                    HCons {
                                                        head: version,
                                                        tail:
                                                            HCons {
                                                                head: segments,
                                                                tail: HNil,
                                                            },
                                                    },
                                            },
                                    },
                            },
                    },
            },
    } = prove::<RequestLinePolicy, _>(line)?.into_evidence();

    debug_assert_eq!(raw_method, method.as_str());
    debug_assert_eq!(raw_path, path.as_str());
    debug_assert_eq!(raw_version, version.as_str());

    Ok(ProvenRequest::new(RequestParts {
        line,
        byte_len,
        token_count,
        method,
        path,
        version,
        segments,
    }))
}
