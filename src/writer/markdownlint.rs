//! Writes the `.markdownlint.jsonc` the generated markdown complies with, alongside `SKILL.md`.

use std::path::Path;

use oas3::OpenApiV3Spec;

use super::utils::{CollectWrites, Writes};

pub(super) const FILENAME: &str = ".markdownlint.jsonc";

const CONFIG: &str = include_str!("../../.markdownlint.jsonc");

pub(super) struct Writer;

impl CollectWrites for Writer {
    fn collect_writes(&self, _spec: &OpenApiV3Spec, dir: &Path, writes: &mut Writes) {
        writes.push(dir.join(FILENAME), CONFIG.to_string());
    }
}
