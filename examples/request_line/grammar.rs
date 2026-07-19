//! Zero-copy request-line refiners.

use strustegy::{Refine, ValidationError};

use crate::types::{Method, PathSegments, ProtocolVersion, RequestPath};

fn token_at<const INDEX: usize>(input: &str) -> Result<&str, ValidationError> {
    input
        .split_ascii_whitespace()
        .nth(INDEX)
        .ok_or_else(|| ValidationError::new("request_token", "missing_token"))
}

fn valid_path(path: &str) -> bool {
    if path == "/" {
        return true;
    }

    path.starts_with('/')
        && path
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'_' | b'-'))
        && path
            .split('/')
            .skip(1)
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}

/// Borrow token `INDEX` from an ASCII-whitespace-delimited request line.
#[derive(Debug, Clone, Copy, Default)]
pub struct Token<const INDEX: usize>;

impl<const INDEX: usize> Refine<str> for Token<INDEX> {
    type Output<'input>
        = &'input str
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        token_at::<INDEX>(input)
    }
}

/// Require exactly `EXPECTED` request-line tokens and return the observed count.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExactTokenCount<const EXPECTED: usize>;

impl<const EXPECTED: usize> Refine<str> for ExactTokenCount<EXPECTED> {
    type Output<'input>
        = usize
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        let count = input.split_ascii_whitespace().count();

        if count == EXPECTED {
            Ok(count)
        } else {
            Err(ValidationError::new(
                "request_token_count",
                "unexpected_token_count",
            ))
        }
    }
}

/// Parse the method token into an owned method enum.
#[derive(Debug, Clone, Copy, Default)]
pub struct ParsedMethod;

impl Refine<str> for ParsedMethod {
    type Output<'input>
        = Method
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        match token_at::<0>(input)? {
            "GET" => Ok(Method::Get),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            _ => Err(ValidationError::new("request_method", "unsupported_method")),
        }
    }
}

/// Validate and borrow the request path token.
#[derive(Debug, Clone, Copy, Default)]
pub struct ParsedPath;

impl Refine<str> for ParsedPath {
    type Output<'input>
        = RequestPath<'input>
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        let path = token_at::<1>(input)?;

        if valid_path(path) {
            Ok(RequestPath::new(path))
        } else {
            Err(ValidationError::new("request_path", "invalid_path"))
        }
    }
}

/// Validate and borrow the STR protocol-version token.
#[derive(Debug, Clone, Copy, Default)]
pub struct ParsedVersion;

impl Refine<str> for ParsedVersion {
    type Output<'input>
        = ProtocolVersion<'input>
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        let version = token_at::<2>(input)?;

        if version == "STR/1" {
            Ok(ProtocolVersion::new(version))
        } else {
            Err(ValidationError::new(
                "protocol_version",
                "unsupported_version",
            ))
        }
    }
}

/// Produce an iterable borrowed view over the validated path segments.
#[derive(Debug, Clone, Copy, Default)]
pub struct ParsedSegments;

impl Refine<str> for ParsedSegments {
    type Output<'input>
        = PathSegments<'input>
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        let path = ParsedPath.refine(input)?;
        Ok(PathSegments::new(path.as_str()))
    }
}
