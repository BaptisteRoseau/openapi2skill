use oas3::{OpenApiV3Spec, spec::Operation};
use tracing::warn;

use crate::writer::utils::Table;

pub(super) fn render_info_table(
    path: &str,
    method: &str,
    op: &Operation,
    spec: &OpenApiV3Spec,
    servers: &[String],
    query_suffix: &str,
) -> String {
    let mut table = Table::unlabeled(2);
    table.row(&["**Method**", &format!("`{method}`")]);
    table.row(&["**URL**", &format!("`{path}`")]);
    for base in servers {
        table.row(&["**Full URL**", &format!("`{base}{path}{query_suffix}`")]);
    }
    table.row(&["**Auth**", &render_security(&op.security, spec)]);
    if let Some(ct) = render_content_type(method, op, spec) {
        table.row(&["**Request Content-Type**", &ct]);
    }
    if let Some(docs) = &op.external_docs {
        table.row(&["**Docs**", &render_external_docs(docs)]);
    }
    table.finish()
}

fn render_external_docs(docs: &oas3::spec::ExternalDoc) -> String {
    match &docs.description {
        Some(desc) => format!("[{desc}]({})", docs.url),
        None => docs.url.to_string(),
    }
}

fn render_content_type(method: &str, op: &Operation, spec: &OpenApiV3Spec) -> Option<String> {
    if !matches!(method, "POST" | "PUT" | "PATCH") {
        return None;
    }
    let body_ref = op.request_body.as_ref()?;
    let body = match body_ref.resolve(spec) {
        Ok(b) => b,
        Err(err) => {
            warn!(
                operation_id = ?op.operation_id,
                "could not resolve request body for content-type detection: {err}; omitting Request Content-Type row"
            );
            return None;
        }
    };
    if body.content.is_empty() {
        return None;
    }
    Some(
        body.content
            .keys()
            .map(|k| format!("`{k}`"))
            .collect::<Vec<_>>()
            .join(", "),
    )
}

fn render_security(
    op_security: &[oas3::spec::SecurityRequirement],
    spec: &OpenApiV3Spec,
) -> String {
    let effective = if op_security.is_empty() {
        &spec.security
    } else {
        op_security
    };
    if effective.is_empty() {
        return "None".to_string();
    }
    let parts: Vec<String> = effective
        .iter()
        .flat_map(|req| req.0.iter())
        .map(|(scheme, scopes)| format_security_scheme(scheme, scopes))
        .collect();
    if parts.is_empty() {
        "None".to_string()
    } else {
        parts.join("; ")
    }
}

fn format_security_scheme(scheme: &str, scopes: &[String]) -> String {
    match scopes {
        [] => scheme.to_string(),
        _ => format!("{scheme} (scopes: {})", scopes.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::render_info_table;
    use crate::writer::testutil::{first_operation, spec_from, spec_with_paths};

    fn get_query_op(extra: serde_json::Value) -> serde_json::Value {
        let mut op = json!({"responses": {"200": {"description": "OK"}}});
        if let (Some(target), Some(extra)) = (op.as_object_mut(), extra.as_object()) {
            for (key, value) in extra {
                target.insert(key.clone(), value.clone());
            }
        }
        json!({"/query": {"get": op}})
    }

    fn spec_with_operation(server_url: &str) -> oas3::OpenApiV3Spec {
        spec_from(json!({
            "servers": [{"url": server_url}],
            "paths": get_query_op(json!({})),
        }))
    }

    fn render(spec: &oas3::OpenApiV3Spec, servers: &[String]) -> String {
        render_with_suffix(spec, servers, "")
    }

    fn render_with_suffix(
        spec: &oas3::OpenApiV3Spec,
        servers: &[String],
        query_suffix: &str,
    ) -> String {
        let (path, method, op) = first_operation(spec);
        render_info_table(&path, &method, &op, spec, servers, query_suffix)
    }

    #[test]
    fn full_url_uses_provided_server() {
        let spec = spec_with_operation("http://spec-host:9090");
        let servers = vec!["http://cli-host:9090".to_string()];
        let out = render(&spec, &servers);
        assert!(
            out.contains("| **Full URL** | `http://cli-host:9090/query` |"),
            "expected CLI server in Full URL:\n{out}"
        );
        assert!(
            !out.contains("spec-host"),
            "spec server must not leak into Full URL:\n{out}"
        );
    }

    #[test]
    fn full_url_renders_one_row_per_server() {
        let spec = spec_with_operation("http://spec-host:9090");
        let servers = vec![
            "http://first:9090".to_string(),
            "https://second:9090".to_string(),
        ];
        let out = render(&spec, &servers);
        assert!(
            out.contains("| **Full URL** | `http://first:9090/query` |"),
            "expected first server:\n{out}"
        );
        assert!(
            out.contains("| **Full URL** | `https://second:9090/query` |"),
            "expected second server:\n{out}"
        );
    }

    #[test]
    fn no_full_url_row_when_no_servers() {
        let spec = spec_with_operation("http://spec-host:9090");
        let out = render(&spec, &[]);
        assert!(
            !out.contains("Full URL"),
            "no Full URL row expected when server list is empty:\n{out}"
        );
    }

    #[test]
    fn full_url_appends_query_suffix() {
        let spec = spec_with_operation("https://api.example.com");
        let servers = vec!["https://api.example.com".to_string()];
        let out = render_with_suffix(&spec, &servers, "?from=string&to=string");
        assert!(
            out.contains(
                "| **Full URL** | `https://api.example.com/query?from=string&to=string` |"
            ),
            "expected query suffix in Full URL:\n{out}"
        );
    }

    #[test]
    fn full_url_appends_suffix_to_all_servers() {
        let spec = spec_with_operation("https://api.example.com");
        let servers = vec![
            "https://prod.example.com".to_string(),
            "https://staging.example.com".to_string(),
        ];
        let out = render_with_suffix(&spec, &servers, "?q=string");
        assert!(
            out.contains("| **Full URL** | `https://prod.example.com/query?q=string` |"),
            "expected suffix on first server:\n{out}"
        );
        assert!(
            out.contains("| **Full URL** | `https://staging.example.com/query?q=string` |"),
            "expected suffix on second server:\n{out}"
        );
    }

    #[test]
    fn no_docs_row_when_no_external_docs() {
        let spec = spec_with_operation("https://api.example.com");
        let out = render(&spec, &[]);
        assert!(!out.contains("**Docs**"), "unexpected Docs row:\n{out}");
    }

    #[test]
    fn docs_row_renders_link_with_description() {
        let spec = spec_with_paths(get_query_op(json!({
            "externalDocs": {"url": "https://docs.example.com/query", "description": "More info"}
        })));
        let out = render(&spec, &[]);
        assert!(
            out.contains("| **Docs** | [More info](https://docs.example.com/query) |"),
            "expected Docs row with link:\n{out}"
        );
    }

    #[test]
    fn docs_row_renders_bare_url_without_description() {
        let spec = spec_with_paths(get_query_op(json!({
            "externalDocs": {"url": "https://docs.example.com/query"}
        })));
        let out = render(&spec, &[]);
        assert!(
            out.contains("| **Docs** | https://docs.example.com/query |"),
            "expected Docs row with bare url:\n{out}"
        );
    }
}
