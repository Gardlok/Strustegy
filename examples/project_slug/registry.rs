//! A dependency-free stand-in for an asynchronous project registry.

use strustegy::Validated;

use crate::types::{AvailableProjectSlug, ProjectSlugPolicy, RegistrationError};

#[derive(Debug, Clone, Copy)]
pub struct SlugRegistry<const N: usize> {
    existing: [&'static str; N],
}

impl<const N: usize> SlugRegistry<N> {
    pub const fn new(existing: [&'static str; N]) -> Self {
        Self { existing }
    }

    pub async fn ensure_available(
        &self,
        slug: Validated<String, ProjectSlugPolicy>,
    ) -> Result<AvailableProjectSlug, RegistrationError> {
        // This await marks the boundary where a real implementation could query
        // a database or service. The example intentionally needs no runtime.
        core::future::ready(()).await;

        let unavailable = self
            .existing
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(slug.get()));

        if unavailable {
            Err(RegistrationError::Unavailable)
        } else {
            Ok(AvailableProjectSlug::new(slug))
        }
    }
}
