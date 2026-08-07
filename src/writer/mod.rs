//! Writer modules that convert an [`oas3::OpenApiV3Spec`] into a tree of markdown skill files.
//! See each submodule's doc comment for the output format it produces.

mod auth;
mod endpoint;
pub(crate) mod extensions;
mod manifest;
mod pipeline;
mod schema;
mod skill;
pub(crate) mod utils;

pub use pipeline::{GenerationContext, openapi2skill};
