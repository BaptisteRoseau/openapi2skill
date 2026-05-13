use oas3::{
    OpenApiV3Spec,
    spec::{Flows, ObjectOrReference, SecurityScheme},
};
use std::path::{Path, PathBuf};

pub fn collect_writes(spec: &OpenApiV3Spec, dir: &Path, writes: &mut Vec<(PathBuf, String)>) {
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
        writes.push((auth_dir.join(&filename), content));
        index_links.push((filename, name.clone()));
    }

    let index: String = index_links
        .iter()
        .map(|(file, name)| format!("- [{name}](./{file})"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    writes.push((auth_dir.join("index.md"), index));
}

fn render_scheme(name: &str, scheme: &SecurityScheme) -> String {
    match scheme {
        SecurityScheme::ApiKey {
            description,
            name: header_name,
            location,
        } => {
            let desc = description.as_deref().unwrap_or("");
            let mut out = format!("# {name}\n\n");
            if !desc.is_empty() {
                out.push_str(&format!("{desc}\n\n"));
            }
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
            let desc = description.as_deref().unwrap_or("");
            let mut out = format!("# {name}\n\n");
            if !desc.is_empty() {
                out.push_str(&format!("{desc}\n\n"));
            }
            let format_hint = bearer_format
                .as_deref()
                .map(|f| format!(" ({f})"))
                .unwrap_or_default();
            // Capitalize scheme for the Authorization header (Basic, Bearer, Digest, …).
            let scheme_header = {
                let mut s = scheme.clone();
                if let Some(c) = s.get_mut(0..1) {
                    c.make_ascii_uppercase();
                }
                s
            };
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
            let desc = description.as_deref().unwrap_or("");
            let mut out = format!("# {name}\n\n");
            if !desc.is_empty() {
                out.push_str(&format!("{desc}\n\n"));
            }
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
            let desc = description.as_deref().unwrap_or("");
            let mut out = format!("# {name}\n\n");
            if !desc.is_empty() {
                out.push_str(&format!("{desc}\n\n"));
            }
            out.push_str(&format!(
                "OpenID Connect — discovery URL: `{open_id_connect_url}`\n"
            ));
            out
        }

        SecurityScheme::MutualTls { description } => {
            let desc = description.as_deref().unwrap_or("");
            let mut out = format!("# {name}\n\n");
            if !desc.is_empty() {
                out.push_str(&format!("{desc}\n\n"));
            }
            out.push_str("Mutual TLS authentication.\n");
            out
        }
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
