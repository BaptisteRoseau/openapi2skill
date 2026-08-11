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
//!
//! With `--token-env-var TOKEN_API`, static credential schemes (ApiKey and HTTP) render a
//! credential note before the example and use `$TOKEN_API` as the placeholder:
//!
//! The api key, token or basic auth is stored under the environment variable $TOKEN_API.
//! You are not allowed to read the content of that variable, you must call it as $TOKEN_API
//! If the following commands shows the token is not set, ask the user to provide it:
//!
//! ```bash
//! [ -n "${TOKEN_API:+x}" ] && echo set || echo "unset or empty"
//! ```
//!
//! ```http
//! GET /example HTTP/1.1
//! Authorization: Basic $TOKEN_API
//! ```

use std::path::Path;

use oas3::{
    OpenApiV3Spec,
    spec::{Flows, ObjectOrReference, SecurityScheme},
};
use tracing::warn;

use super::utils::{CollectWrites, Table, Writes, build_index, desc_cell, desc_paragraph};

pub(super) struct Writer {
    pub(super) token_env_var: Option<String>,
}

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
            writes.push(
                auth_dir.join(&filename),
                render_scheme(name, scheme, self.token_env_var.as_deref()),
            );
            index_links.push((filename, name.clone()));
        }

        writes.push(auth_dir.join("index.md"), build_index(&index_links));
    }
}

fn render_scheme(name: &str, scheme: &SecurityScheme, token_env_var: Option<&str>) -> String {
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
            let credential = credential_placeholder(token_env_var, "your-key-here");
            out.push_str(&format!(
                "Location: `{location}`\n\n{}{}\n",
                token_env_var_note(token_env_var),
                http_example(&format!("{header_name}: {credential}"))
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
            let default_placeholder = match scheme.to_ascii_lowercase().as_str() {
                "basic" => "<base64(username:password)>",
                "bearer" => "<token>",
                _ => "<credentials>",
            };
            let credential = credential_placeholder(token_env_var, default_placeholder);
            out.push_str(&format!(
                "HTTP `{scheme}` authentication{format_hint}.\n\n{}{}\n",
                token_env_var_note(token_env_var),
                http_example(&format!("Authorization: {scheme_header} {credential}"))
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

/// The paragraph placed right before the HTTP example of every static credential scheme.
/// Empty when no environment variable was given.
fn token_env_var_note(token_env_var: Option<&str>) -> String {
    match token_env_var {
        Some(var) => format!(
            "The api key, token or basic auth is stored under the environment variable ${var}.\n\
             You are not allowed to read the content of that variable, you must call it as ${var}\n\
             If the following commands shows the token is not set, ask the user to provide it:\n\n\
             ```bash\n\
             [ -n \"${{{var}:+x}}\" ] && echo set || echo \"unset or empty\"\n\
             ```\n\n"
        ),
        None => String::new(),
    }
}

fn credential_placeholder(token_env_var: Option<&str>, default: &str) -> String {
    match token_env_var {
        Some(var) => format!("${var}"),
        None => default.to_string(),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn api_key_scheme() -> SecurityScheme {
        SecurityScheme::ApiKey {
            description: None,
            name: "X-API-Key".to_string(),
            location: "header".to_string(),
        }
    }

    fn http_scheme(scheme: &str) -> SecurityScheme {
        SecurityScheme::Http {
            description: None,
            scheme: scheme.to_string(),
            bearer_format: None,
        }
    }

    fn oauth2_scheme() -> SecurityScheme {
        SecurityScheme::OAuth2 {
            description: None,
            flows: Flows::default(),
        }
    }

    const NOTE: &str = "The api key, token or basic auth is stored under the environment variable $TOKEN_API.\nYou are not allowed to read the content of that variable, you must call it as $TOKEN_API\nIf the following commands shows the token is not set, ask the user to provide it:\n\n```bash\n[ -n \"${TOKEN_API:+x}\" ] && echo set || echo \"unset or empty\"\n```";

    #[test]
    fn api_key_renders_note_before_http_example() {
        let out = render_scheme("api_key", &api_key_scheme(), Some("TOKEN_API"));
        let note_at = out.find(NOTE).expect("note should be rendered");
        let example_at = out
            .find("```http")
            .expect("http example should be rendered");
        assert!(note_at < example_at);
        assert!(out.contains("X-API-Key: $TOKEN_API"));
    }

    #[test]
    fn api_key_without_env_var_keeps_default_placeholder() {
        let out = render_scheme("api_key", &api_key_scheme(), None);
        assert!(!out.contains("environment variable"));
        assert!(out.contains("X-API-Key: your-key-here"));
    }

    #[test]
    fn basic_auth_renders_note_and_env_var_credential() {
        let out = render_scheme("basic", &http_scheme("basic"), Some("TOKEN_API"));
        assert!(out.contains(NOTE));
        assert!(out.contains("Authorization: Basic $TOKEN_API"));
    }

    #[test]
    fn bearer_auth_renders_note_and_env_var_credential() {
        let out = render_scheme("bearer", &http_scheme("bearer"), Some("TOKEN_API"));
        assert!(out.contains(NOTE));
        assert!(out.contains("Authorization: Bearer $TOKEN_API"));
    }

    #[test]
    fn oauth2_ignores_env_var() {
        let out = render_scheme("oauth", &oauth2_scheme(), Some("TOKEN_API"));
        assert!(!out.contains("environment variable"));
        assert!(out.contains("Authorization: Bearer <access_token>"));
    }
}
