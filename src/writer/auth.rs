use std::path::{Path, PathBuf};

use oas3::{
    OpenApiV3Spec,
    spec::{Flows, ObjectOrReference, SecurityScheme},
};
use tracing::info;

use super::utils::{CollectWrites, build_index};

pub(super) struct Writer;

impl CollectWrites for Writer {
    fn collect_writes(
        &self,
        spec: &OpenApiV3Spec,
        dir: &Path,
        writes: &mut Vec<(PathBuf, String)>,
    ) {
        let Some(components) = &spec.components else {
            return;
        };
        if components.security_schemes.is_empty() {
            return;
        }

        let auth_dir = dir.join("authentication");
        let mut index_links: Vec<(String, String)> = Vec::new();

        for (name, scheme_ref) in &components.security_schemes {
            let scheme = match scheme_ref {
                ObjectOrReference::Object(s) => s,
                ObjectOrReference::Ref { .. } => continue,
            };
            let filename = format!("{}.md", name.to_lowercase().replace(' ', "-"));
            let content = render_scheme(name, scheme);
            let write_path = (auth_dir.join(&filename), content);
            info!("Writing {:?}", write_path.0);
            writes.push(write_path);
            index_links.push((filename, name.clone()));
        }

        let write_path = (auth_dir.join("index.md"), build_index(&index_links));
        info!("Writing {:?}", write_path.0);
        writes.push(write_path);
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
            out.push_str(&format!(
                "Add the following header to every request:\n\n| Header | Value |\n|--------|-------|\n| `{header_name}` | Your API key |\n\nLocation: `{location}`\n\n```http\nGET /example HTTP/1.1\n{header_name}: your-key-here\n```\n"
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
                "HTTP `{scheme}` authentication{format_hint}.\n\n```http\nGET /example HTTP/1.1\nAuthorization: {scheme_header} {placeholder}\n```\n"
            ));
            out
        }

        SecurityScheme::OAuth2 { description, flows } => {
            let mut out = render_header(name, description.as_deref());
            out.push_str("OAuth 2.0 authentication.\n\n");
            out.push_str(&render_flows(flows));
            out.push_str(
                "\n```http\nGET /example HTTP/1.1\nAuthorization: Bearer <access_token>\n```\n",
            );
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
    let mut out = format!("# {name}\n\n");
    if let Some(desc) = description.filter(|d| !d.is_empty()) {
        out.push_str(&format!("{desc}\n\n"));
    }
    out
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
        out.push_str(&format!(
            "**Authorization URL:** `{}`\n\n",
            f.authorization_url
        ));
        out.push_str(&render_scopes(&f.scopes));
    }
    if let Some(f) = &flows.password {
        out.push_str(&format!("**Token URL:** `{}`\n\n", f.token_url));
        out.push_str(&render_scopes(&f.scopes));
    }
    if let Some(f) = &flows.client_credentials {
        out.push_str(&format!("**Token URL:** `{}`\n\n", f.token_url));
        out.push_str(&render_scopes(&f.scopes));
    }
    if let Some(f) = &flows.authorization_code {
        out.push_str(&format!(
            "**Authorization URL:** `{}`\n**Token URL:** `{}`\n\n",
            f.authorization_url, f.token_url
        ));
        out.push_str(&render_scopes(&f.scopes));
    }

    out
}

fn render_scopes(scopes: &oas3::Map<String, String>) -> String {
    if scopes.is_empty() {
        return String::new();
    }
    let mut out = "**Scopes:**\n\n| Scope | Description |\n|-------|-------------|\n".to_string();
    for (scope, desc) in scopes {
        out.push_str(&format!("| `{scope}` | {desc} |\n"));
    }
    out.push('\n');
    out
}
