use oas3::{OpenApiV3Spec, spec::Operation};
use tracing::warn;

pub(super) fn render_info_table(
    path: &str,
    method: &str,
    op: &Operation,
    spec: &OpenApiV3Spec,
    servers: &[String],
    query_suffix: &str,
) -> String {
    let mut out = "| | |\n|--|--|\n".to_string();
    out.push_str(&format!("| **Method** | `{method}` |\n"));
    out.push_str(&format!("| **URL** | `{path}` |\n"));
    for base in servers {
        out.push_str(&format!(
            "| **Full URL** | `{base}{path}{query_suffix}` |\n"
        ));
    }
    out.push_str(&format!(
        "| **Auth** | {} |\n",
        render_security(&op.security, spec)
    ));
    if let Some(ct) = render_content_type(method, op, spec) {
        out.push_str(&format!("| **Request Content-Type** | {ct} |\n"));
    }
    out.push('\n');
    out
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
    use super::render_info_table;

    fn spec_with_operation(server_url: &str) -> oas3::OpenApiV3Spec {
        let json = format!(
            r#"{{"openapi":"3.0.0","info":{{"title":"T","version":"1"}},"servers":[{{"url":"{server_url}"}}],"paths":{{"/query":{{"get":{{"responses":{{"200":{{"description":"OK"}}}}}}}}}}}}"#
        );
        oas3::from_json(json).unwrap()
    }

    fn render(spec: &oas3::OpenApiV3Spec, servers: &[String]) -> String {
        render_with_suffix(spec, servers, "")
    }

    fn render_with_suffix(
        spec: &oas3::OpenApiV3Spec,
        servers: &[String],
        query_suffix: &str,
    ) -> String {
        let ops: Vec<_> = spec.operations().collect();
        let (path, method, op) = &ops[0];
        render_info_table(path, method.as_str(), op, spec, servers, query_suffix)
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
}
