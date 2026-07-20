//! End-to-end project-slug registration using Strustegy's public layers.

mod pipeline;
mod registry;
mod runtime;
mod types;

use registry::SlugRegistry;
use runtime::block_on;
use strustegy::{
    AsyncStrategy, AsyncStrategyExt, Validated, async_strategy_fn, into_async, strategy_fn,
};
use types::{AvailableProjectSlug, ProjectSlugPolicy, RegistrationError};

fn register_project(slug: AvailableProjectSlug) {
    println!("registered project: {}", slug.as_str());
}

fn main() -> Result<(), RegistrationError> {
    let raw_input = b"  Strustegy_Demo  ".as_slice();
    let registry = SlugRegistry::new(["rose", "strustegy", "pennywise"]);

    let prepare = strategy_fn(pipeline::prepare_slug);
    let ensure_available = async_strategy_fn(async |slug: Validated<String, ProjectSlugPolicy>| {
        let available = registry.ensure_available(slug).await?;

        println!("availability proof: slug is not registered");

        Ok::<AvailableProjectSlug, RegistrationError>(available)
    });
    let registration = into_async(prepare).and_then_async(ensure_available);
    let available = block_on(registration.apply_async(raw_input))?;

    println!("raw input: {}", String::from_utf8_lossy(raw_input));
    register_project(available);

    Ok(())
}
