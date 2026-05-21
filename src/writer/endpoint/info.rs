use oas3::{OpenApiV3Spec, spec::Operation};
use tracing::warn;

pub(super) fn render_info_table(
    path: &str,
    method: &str,
    op: &Operation,
    spec: &OpenApiV3Spec,
) -> String {
    let mut out = "| | |\n|--|--|\n".to_string();
    out.push_str(&format!("| **Method** | `{method}` |\n"));
    out.push_str(&format!("| **URL** | `{path}` |\n"));
    if spec.servers.len() == 1 {
        let base = spec.servers[0].url.trim_end_matches('/');
        out.push_str(&format!("| **Full URL** | `{base}{path}` |\n"));
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
