//! Writes the raw OpenAPI document verbatim as `openapi.json`/`openapi.yml`, alongside `SKILL.md`.
//!
//! This file exists for tooling that needs the raw spec (SDK generators, fuzzers, Swagger UIs),
//! not for the agent — see the "OpenAPI Manifest" section [`skill`](super::skill) renders in
//! `SKILL.md`, which tells the agent to read the indexed markdown files instead.

use std::path::Path;

use oas3::OpenApiV3Spec;

use super::utils::{CollectWrites, Writes};

pub(super) struct Writer {
    pub(super) raw: String,
    pub(super) extension: &'static str,
}

impl Writer {
    pub(super) fn filename(&self) -> String {
        format!("openapi.{}", self.extension)
    }
}

impl CollectWrites for Writer {
    fn collect_writes(&self, _spec: &OpenApiV3Spec, dir: &Path, writes: &mut Writes) {
        writes.push(dir.join(self.filename()), self.raw.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_uses_extension() {
        let writer = Writer {
            raw: String::new(),
            extension: "yml",
        };
        assert_eq!(writer.filename(), "openapi.yml");
    }
}
