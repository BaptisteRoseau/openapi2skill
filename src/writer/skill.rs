//! Generates `SKILL.md` — the top-level entry point that the agent reads first.
//!
//! Output: one file at `{output_dir}/SKILL.md`.
//!
//! ---
//! name: Grafana HTTP API.
//! description: The API documentation and specifications of Grafana HTTP API.
//! allowed-tools:
//!   - Read
//!   - Bash(ls *)
//!   - Bash(grep *)
//!   - Bash(find *)
//! ---
//!
//! # Grafana HTTP API. Documentation
//!
//! **Version:** 0.0.1
//!
//! **Source:** https://example.com/grafana-openapi.json
//!
//! **Generated with:** `openapi2skill https://example.com/grafana-openapi.json`
//!
//! **Servers:**
//! - /api
//!
//! ## API Description
//!
//! The Grafana backend exposes an HTTP API, the same API is used by the frontend to do
//! everything from saving dashboards, creating users and updating data sources.
//!
//! ## Navigation
//!
//! Given your goal, read the relevant index.md file links bellow and the ones they will be
//! pointing to to read the endpoints descriptions you will need.
//! ...
//! Read the following files depending on your current needs:
//!
//! - [authentication/index.md](./authentication/index.md): Authentication workflows
//! - [endpoints/index.md](./endpoints/index.md): API endpoints
//! - [schemas/index.md](./schemas/index.md): Data schemas, only if you need them alone. They are already included in endpoints.
//!
//! ## OpenAPI Manifest
//!
//! The raw OpenAPI manifest is available at [`openapi.json`](./openapi.json), fetched from
//! https://example.com/grafana-openapi.json. It is provided only for tools that require the raw
//! spec — e.g. generating an SDK, a Swagger/OpenAPI client, or a fuzzer. Do **not** read it to
//! navigate this API: the indexed markdown files above contain the same information organized
//! for far fewer tokens.

use std::path::Path;

use oas3::OpenApiV3Spec;
use url::Url;

use super::utils::{CollectWrites, Writes};

pub(super) struct Writer {
    pub(super) name: String,
    pub(super) servers_override: Vec<String>,
    pub(super) source_url: Option<String>,
    pub(super) manifest_filename: String,
    pub(super) command: String,
}

impl CollectWrites for Writer {
    fn collect_writes(&self, spec: &OpenApiV3Spec, dir: &Path, writes: &mut Writes) {
        writes.push(dir.join("SKILL.md"), self.render(spec));
    }
}

impl Writer {
    fn render(&self, spec: &OpenApiV3Spec) -> String {
        let title = &spec.info.title;
        let description = spec.info.description.as_deref().unwrap_or("");
        let source_url = self.source_url.as_deref();

        let mut out = render_skill_header(&self.name, title);
        out.push_str(&render_metadata(
            spec,
            &self.servers_override,
            source_url,
            &self.command,
        ));
        out.push_str(&render_decription_and_navigation(description, spec));
        out.push_str(&render_index(spec));
        out.push_str(&render_manifest_section(
            &self.manifest_filename,
            source_url,
        ));
        out
    }
}

fn render_skill_header(name: &str, title: &str) -> String {
    format!(
        "---\nname: {name}\ndescription: The API documentation and specifications of {title}\nallowed-tools:\n  - Read\n  - Bash(ls *)\n  - Bash(grep *)\n  - Bash(find *)\n---\n\n# {title} Documentation\n\n"
    )
}

fn render_metadata(
    spec: &OpenApiV3Spec,
    servers_override: &[String],
    source_url: Option<&str>,
    command: &str,
) -> String {
    let mut out = render_version_line(spec);
    if let Some(url) = source_url {
        out.push_str(&format!("**Source:** {url}\n\n"));
    }
    out.push_str(&format!("**Generated with:** `{command}`\n\n"));
    out.push_str(&render_servers_section(spec, servers_override));
    if let Some(ext) = &spec.external_docs {
        out.push_str(&format!(
            "**External Docs:** {}\n\n",
            labelled_link(ext.description.as_deref(), ext.url.as_str())
        ));
    }
    out
}

fn render_version_line(spec: &OpenApiV3Spec) -> String {
    let mut out = format!("**Version:** {}", spec.info.version);
    if let Some(license) = &spec.info.license {
        let url = license.url.as_ref().map(Url::to_string).unwrap_or_default();
        out.push_str(&format!(
            " | **License:** {}",
            labelled_link(Some(&license.name), &url)
        ));
    }
    if let Some(tos) = &spec.info.terms_of_service {
        out.push_str(&format!(" | **Terms of Service:** {tos}"));
    }
    out.push_str("\n\n");
    out
}

/// A markdown link when both parts are present, otherwise whichever one is.
fn labelled_link(label: Option<&str>, url: &str) -> String {
    match label {
        Some(label) if url.is_empty() => label.to_string(),
        Some(label) => format!("[{label}]({url})"),
        None => url.to_string(),
    }
}

fn render_servers_section(spec: &OpenApiV3Spec, servers_override: &[String]) -> String {
    let entries: Vec<(&str, Option<&str>)> = if servers_override.is_empty() {
        spec.servers
            .iter()
            .map(|s| (s.url.as_str(), s.description.as_deref()))
            .collect()
    } else {
        servers_override
            .iter()
            .map(|url| (url.as_str(), None))
            .collect()
    };
    if entries.is_empty() {
        return String::new();
    }
    let mut out = "**Servers:**\n".to_string();
    for (url, description) in entries {
        match description {
            Some(desc) => out.push_str(&format!("- {url} — {desc}\n")),
            None => out.push_str(&format!("- {url}\n")),
        }
    }
    out.push('\n');
    out
}

fn has_deprecated_operations(spec: &OpenApiV3Spec) -> bool {
    spec.operations()
        .any(|(_, _, op)| op.deprecated == Some(true))
}

fn render_decription_and_navigation(description: &str, spec: &OpenApiV3Spec) -> String {
    let mut out = "".to_string();
    if !description.is_empty() {
        out.push_str(&format!("## API Description\n\n{description}\n\n"));
    }
    out.push_str("## Navigation\n\nGiven your goal, read the relevant index.md file links bellow and subsequent file to the endpoints required to achieve your task.\nAvoid using `ls` and `grep`, use them only when after the indexes if they did not provide the information required, or if you have to search for a specific pattern.\nOnly follow markdown links references required to achieve your goal. The less files you read, the better.");
    if has_deprecated_operations(spec) {
        out.push_str("\n\nSome endpoints are marked as deprecated. Prefer non-deprecated alternatives when available.\n\n");
    }
    out
}

fn render_index(spec: &OpenApiV3Spec) -> String {
    let has_auth = has_components(spec, |c| !c.security_schemes.is_empty());
    let has_schemas = has_components(spec, |c| !c.schemas.is_empty());
    let has_endpoints = spec.operations().next().is_some();

    let mut out = "Read the following files depending on your current needs:\n\n".to_string();
    if has_auth {
        out.push_str(
            "- [authentication/index.md](./authentication/index.md): Authentication workflows\n",
        );
    }
    if has_endpoints {
        out.push_str("- [endpoints/index.md](./endpoints/index.md): API endpoints\n");
    }
    if has_schemas {
        out.push_str("- [schemas/index.md](./schemas/index.md): Data schemas, only if you need them alone. They are already included in endpoints.\n");
    }
    out
}

fn has_components(spec: &OpenApiV3Spec, predicate: fn(&oas3::spec::Components) -> bool) -> bool {
    spec.components.as_ref().map(predicate).unwrap_or(false)
}

fn render_manifest_section(manifest_filename: &str, source_url: Option<&str>) -> String {
    let mut out = format!(
        "\n## OpenAPI Manifest\n\nThe raw OpenAPI manifest is available at [`{manifest_filename}`](./{manifest_filename})"
    );
    if let Some(url) = source_url {
        out.push_str(&format!(", fetched from {url}"));
    }
    out.push_str(
        ". It is provided only for tools that require the raw spec — e.g. generating an SDK, a Swagger/OpenAPI client, or a fuzzer. Do **not** read it to navigate this API: the indexed markdown files above contain the same information organized for far fewer tokens.\n",
    );
    out
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::writer::testutil::{spec_with_paths, spec_with_servers as minimal_spec};

    fn minimal_spec_with_deprecated() -> OpenApiV3Spec {
        spec_with_paths(json!({
            "/test": {"get": {"deprecated": true, "responses": {"200": {"description": "OK"}}}}
        }))
    }

    #[test]
    fn server_override_replaces_spec_servers() {
        let spec = minimal_spec(&["https://spec.example.com"]);
        let overrides = vec![
            "https://override1.example.com".to_string(),
            "https://override2.example.com".to_string(),
        ];
        let out = render_metadata(&spec, &overrides, None, "openapi2skill spec.json");
        assert!(
            out.contains("https://override1.example.com"),
            "expected override1 in:\n{out}"
        );
        assert!(
            out.contains("https://override2.example.com"),
            "expected override2 in:\n{out}"
        );
        assert!(
            !out.contains("https://spec.example.com"),
            "spec server should be suppressed:\n{out}"
        );
    }

    #[test]
    fn empty_override_falls_back_to_spec_servers() {
        let spec = minimal_spec(&["https://spec.example.com"]);
        let out = render_metadata(&spec, &[], None, "openapi2skill spec.json");
        assert!(
            out.contains("https://spec.example.com"),
            "expected spec server in:\n{out}"
        );
    }

    #[test]
    fn no_servers_section_when_both_empty() {
        let spec = minimal_spec(&[]);
        let out = render_metadata(&spec, &[], None, "openapi2skill spec.json");
        assert!(
            !out.contains("**Servers:**"),
            "servers section should be absent:\n{out}"
        );
    }

    #[test]
    fn source_url_rendered_when_present() {
        let spec = minimal_spec(&[]);
        let out = render_metadata(
            &spec,
            &[],
            Some("https://example.com/spec.json"),
            "openapi2skill spec.json",
        );
        assert!(
            out.contains("**Source:** https://example.com/spec.json"),
            "expected source line in:\n{out}"
        );
    }

    #[test]
    fn no_source_section_when_absent() {
        let spec = minimal_spec(&[]);
        let out = render_metadata(&spec, &[], None, "openapi2skill spec.json");
        assert!(
            !out.contains("**Source:**"),
            "source section should be absent:\n{out}"
        );
    }

    #[test]
    fn command_is_always_rendered() {
        let spec = minimal_spec(&[]);
        let out = render_metadata(&spec, &[], None, "openapi2skill spec.json --force");
        assert!(
            out.contains("**Generated with:** `openapi2skill spec.json --force`"),
            "expected generated-with line in:\n{out}"
        );
    }

    #[test]
    fn manifest_section_links_filename() {
        let out = render_manifest_section("openapi.json", None);
        assert!(
            out.contains("[`openapi.json`](./openapi.json)"),
            "expected manifest link in:\n{out}"
        );
        assert!(
            out.contains("Do **not** read it"),
            "expected do-not-read caveat in:\n{out}"
        );
    }

    #[test]
    fn manifest_section_includes_source_when_present() {
        let out = render_manifest_section("openapi.yml", Some("https://example.com/spec.yml"));
        assert!(
            out.contains("fetched from https://example.com/spec.yml"),
            "expected source mention in:\n{out}"
        );
    }

    #[test]
    fn manifest_section_omits_source_when_absent() {
        let out = render_manifest_section("openapi.json", None);
        assert!(
            !out.contains("fetched from"),
            "should not mention a source:\n{out}"
        );
    }

    #[test]
    fn navigation_has_no_deprecated_note_when_no_deprecated_ops() {
        let spec = minimal_spec(&[]);
        let out = render_decription_and_navigation("", &spec);
        assert!(
            !out.contains("deprecated"),
            "should not contain deprecated note:\n{out}"
        );
    }

    #[test]
    fn navigation_has_deprecated_note_when_deprecated_ops_exist() {
        let spec = minimal_spec_with_deprecated();
        let out = render_decription_and_navigation("", &spec);
        assert!(
            out.contains("deprecated"),
            "should contain deprecated note:\n{out}"
        );
    }
}
