use std::collections::HashSet;

use oas3::{
    OpenApiV3Spec,
    spec::{Operation, Parameter, ParameterIn},
};
use tracing::warn;

use super::{
    body::{render_payload_section, render_responses_section},
    info::render_info_table,
    params::{render_path_params_table, render_query_params_table, required_query_string},
};
use crate::writer::extensions::render_extensions_table;

pub(super) fn render_endpoint(
    path: &str,
    method: &str,
    op: &Operation,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
    servers: &[String],
) -> String {
    let mut out = format!("# {method} {path}\n\n");
    if op.deprecated == Some(true) {
        out.push_str("> **Deprecated.** Avoid using this endpoint when an alternative exists.\n\n");
    }
    out.push_str(&render_extensions_table(&op.extensions));

    let resolved_params = resolve_params(op, spec);
    let path_params = params_in(&resolved_params, ParameterIn::Path);
    let query_params = params_in(&resolved_params, ParameterIn::Query);
    let query_suffix = required_query_string(&query_params, spec);

    out.push_str(&render_info_table(
        path,
        method,
        op,
        spec,
        servers,
        &query_suffix,
    ));
    out.push_str(&render_input_section(
        op,
        &path_params,
        &query_params,
        spec,
        multi_use,
    ));
    out.push_str(&render_responses_section(op, spec, multi_use));
    out
}

fn params_in(params: &[Parameter], location: ParameterIn) -> Vec<&Parameter> {
    params.iter().filter(|p| p.location == location).collect()
}

fn resolve_params(op: &Operation, spec: &OpenApiV3Spec) -> Vec<Parameter> {
    op.parameters
        .iter()
        .filter_map(|p| match p.resolve(spec) {
            Ok(param) => Some(param),
            Err(err) => {
                warn!(
                    operation_id = ?op.operation_id,
                    "could not resolve parameter: {err}; dropping it from the rendered table"
                );
                None
            }
        })
        .collect()
}

fn render_input_section(
    op: &Operation,
    path_params: &[&Parameter],
    query_params: &[&Parameter],
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    if path_params.is_empty() && query_params.is_empty() && op.request_body.is_none() {
        return String::new();
    }

    let mut out = "## Input\n\n".to_string();
    out.push_str(&render_path_params_table(path_params, spec));
    out.push_str(&render_query_params_table(query_params, spec));
    out.push_str(&render_payload_section(op, spec, multi_use));
    out
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::json;

    use super::render_endpoint;
    use crate::writer::testutil::{first_operation, spec_with_paths};

    fn render_first(deprecated: bool) -> String {
        let spec = spec_with_paths(json!({
            "/test": {
                "get": {
                    "deprecated": deprecated,
                    "responses": {"200": {"description": "OK"}},
                }
            }
        }));
        let (path, method, op) = first_operation(&spec);
        render_endpoint(&path, &method, &op, &spec, &HashSet::new(), &[])
    }

    #[test]
    fn non_deprecated_has_no_notice() {
        let out = render_first(false);
        assert!(
            !out.contains("Deprecated"),
            "should not contain Deprecated:\n{out}"
        );
    }

    #[test]
    fn deprecated_has_notice() {
        let out = render_first(true);
        assert!(
            out.contains("Deprecated"),
            "should contain Deprecated:\n{out}"
        );
    }
}
