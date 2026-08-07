//! Generates `authentication/index.md` and one file per security scheme.
//!
//! **`authentication/index.md`**
//!
//! - [api_key](./api_key.md)
//! - [basic](./basic.md)
//!
//! **`authentication/api_key.md`** (ApiKey scheme)
//!
//! # api_key
//!
//! Add the following header to every request:
//!
//! | Header | Value |
//! |--------|-------|
//! | `Authorization` | Your API key |
//!
//! Location: `header`
//!
//! ```http
//! GET /example HTTP/1.1
//! Authorization: your-key-here
//! ```
//!
//! **`authentication/basic.md`** (HTTP scheme)
//!
//! # basic
//!
//! HTTP `basic` authentication.
//!
//! ```http
//! GET /example HTTP/1.1
//! Authorization: Basic <base64(username:password)>
//! ```

use std::path::Path;

use oas3::{
    OpenApiV3Spec,
    spec::{Flows, ObjectOrReference, SecurityScheme},
};
use tracing::warn;

use super::utils::{CollectWrites, Table, Writes, build_index, desc_cell, desc_paragraph};

pub(super) struct Writer;

impl CollectWrites for Writer {
    fn collect_writes(&self, spec: &OpenApiV3Spec, dir: &Path, writes: &mut Writes) {
        let Some(components) = &spec.components else {
            return;
        };
        if components.security_schemes.is_empty() {
            return;
        }

        let auth_dir = dir.join("authentication");
        let mut index_links: Vec<(String, String)> = Vec::new();

        for (name, scheme_ref) in &components.security_schemes {
            let ObjectOrReference::Object(scheme) = scheme_ref else {
                warn!(
                    scheme = name,
                    "security scheme is a $ref; skipping (refs to other schemes are not supported)"
                );
                continue;
            };
            let filename = format!("{}.md", name.to_lowercase().replace(' ', "-"));
            writes.push(auth_dir.join(&filename), render_scheme(name, scheme));
            index_links.push((filename, name.clone()));
        }

        writes.push(auth_dir.join("index.md"), build_index(&index_links));
    }
}

fn render_scheme(name: &str, scheme: &SecurityScheme) -> String {
    match scheme {
        SecurityScheme::ApiKey {
            description,
            name: header_name,
            location,
        } => {
            let mut out = render_header(name, description.as_deref());
            out.push_str("Add the following header to every request:\n\n");
            let mut table = Table::new(&["Header", "Value"]);
            table.row(&[format!("`{header_name}`"), "Your API key".to_string()]);
            out.push_str(&table.finish());
            out.push_str(&format!(
                "Location: `{location}`\n\n{}\n",
                http_example(&format!("{header_name}: your-key-here"))
            ));
            out
        }

        SecurityScheme::Http {
            description,
            scheme,
            bearer_format,
        } => {
            let mut out = render_header(name, description.as_deref());
            let format_hint = bearer_format
                .as_deref()
                .map(|f| format!(" ({f})"))
                .unwrap_or_default();
            let scheme_header = capitalize_first(scheme);
            let placeholder = match scheme.to_ascii_lowercase().as_str() {
                "basic" => "<base64(username:password)>",
                "bearer" => "<token>",
                _ => "<credentials>",
            };
            out.push_str(&format!(
                "HTTP `{scheme}` authentication{format_hint}.\n\n{}\n",
                http_example(&format!("Authorization: {scheme_header} {placeholder}"))
            ));
            out
        }

        SecurityScheme::OAuth2 { description, flows } => {
            let mut out = render_header(name, description.as_deref());
            out.push_str("OAuth 2.0 authentication.\n\n");
            out.push_str(&render_flows(flows));
            out.push_str(&format!(
                "\n{}\n",
                http_example("Authorization: Bearer <access_token>")
            ));
            out
        }

        SecurityScheme::OpenIdConnect {
            description,
            open_id_connect_url,
        } => {
            let mut out = render_header(name, description.as_deref());
            out.push_str(&format!(
                "OpenID Connect — discovery URL: `{open_id_connect_url}`\n"
            ));
            out
        }

        SecurityScheme::MutualTls { description } => {
            let mut out = render_header(name, description.as_deref());
            out.push_str("Mutual TLS authentication.\n");
            out
        }
    }
}

fn render_header(name: &str, description: Option<&str>) -> String {
    format!("# {name}\n\n{}", desc_paragraph(description))
}

fn http_example(auth_line: &str) -> String {
    format!("```http\nGET /example HTTP/1.1\n{auth_line}\n```")
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn render_flows(flows: &Flows) -> String {
    let mut out = String::new();
    if let Some(f) = &flows.implicit {
        out.push_str(&render_flow(
            &[("Authorization URL", f.authorization_url.to_string())],
            &f.scopes,
        ));
    }
    if let Some(f) = &flows.password {
        out.push_str(&render_flow(
            &[("Token URL", f.token_url.to_string())],
            &f.scopes,
        ));
    }
    if let Some(f) = &flows.client_credentials {
        out.push_str(&render_flow(
            &[("Token URL", f.token_url.to_string())],
            &f.scopes,
        ));
    }
    if let Some(f) = &flows.authorization_code {
        out.push_str(&render_flow(
            &[
                ("Authorization URL", f.authorization_url.to_string()),
                ("Token URL", f.token_url.to_string()),
            ],
            &f.scopes,
        ));
    }
    out
}

fn render_flow(urls: &[(&str, String)], scopes: &oas3::Map<String, String>) -> String {
    let mut out = String::new();
    for (label, url) in urls {
        out.push_str(&format!("**{label}:** `{url}`\n"));
    }
    out.push('\n');
    out.push_str(&render_scopes(scopes));
    out
}

fn render_scopes(scopes: &oas3::Map<String, String>) -> String {
    if scopes.is_empty() {
        return String::new();
    }
    let mut table = Table::new(&["Scope", "Description"]);
    for (scope, desc) in scopes {
        table.row(&[format!("`{scope}`"), desc_cell(Some(desc))]);
    }
    format!("**Scopes:**\n\n{}", table.finish())
}
