//! Borrowed domain types produced by request-line refinement.

use core::fmt;

/// Methods recognized by the miniature STR/1 protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Put,
    Delete,
}

impl Method {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A validated request path borrowing from the original request line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestPath<'input>(&'input str);

impl<'input> RequestPath<'input> {
    pub(crate) const fn new(path: &'input str) -> Self {
        Self(path)
    }

    pub const fn as_str(&self) -> &'input str {
        self.0
    }
}

impl fmt::Display for RequestPath<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

/// A validated protocol-version token borrowing from the original request line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolVersion<'input>(&'input str);

impl<'input> ProtocolVersion<'input> {
    pub(crate) const fn new(version: &'input str) -> Self {
        Self(version)
    }

    pub const fn as_str(&self) -> &'input str {
        self.0
    }
}

impl fmt::Display for ProtocolVersion<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

/// An allocation-free iterable view over validated path segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathSegments<'input> {
    path: &'input str,
}

impl<'input> PathSegments<'input> {
    pub(crate) const fn new(path: &'input str) -> Self {
        Self { path }
    }

    pub fn iter(self) -> impl Iterator<Item = &'input str> {
        self.path
            .strip_prefix('/')
            .unwrap_or(self.path)
            .split('/')
            .filter(|segment| !segment.is_empty())
    }

    pub fn count(self) -> usize {
        self.iter().count()
    }
}

/// A request whose syntax has been proven by the request-line policy.
///
/// Every borrowed field remains tied to the lifetime of the original byte
/// buffer. The type owns no request text and performs no heap allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProvenRequest<'input> {
    line: &'input str,
    byte_len: usize,
    token_count: usize,
    method: Method,
    path: RequestPath<'input>,
    version: ProtocolVersion<'input>,
    segments: PathSegments<'input>,
}

pub(crate) struct RequestParts<'input> {
    pub line: &'input str,
    pub byte_len: usize,
    pub token_count: usize,
    pub method: Method,
    pub path: RequestPath<'input>,
    pub version: ProtocolVersion<'input>,
    pub segments: PathSegments<'input>,
}

impl<'input> ProvenRequest<'input> {
    pub(crate) const fn new(parts: RequestParts<'input>) -> Self {
        Self {
            line: parts.line,
            byte_len: parts.byte_len,
            token_count: parts.token_count,
            method: parts.method,
            path: parts.path,
            version: parts.version,
            segments: parts.segments,
        }
    }

    pub const fn line(self) -> &'input str {
        self.line
    }

    pub const fn byte_len(self) -> usize {
        self.byte_len
    }

    pub const fn token_count(self) -> usize {
        self.token_count
    }

    pub const fn method(self) -> Method {
        self.method
    }

    pub const fn path(self) -> RequestPath<'input> {
        self.path
    }

    pub const fn version(self) -> ProtocolVersion<'input> {
        self.version
    }

    pub const fn segments(self) -> PathSegments<'input> {
        self.segments
    }
}
