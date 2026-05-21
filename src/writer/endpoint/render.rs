use std::collections::HashSet;

use oas3::{
    OpenApiV3Spec,
    spec::{Operation, ParameterIn},
};
use tracing::warn;

use super::{
    body::{render_payload_section, render_responses_section},
    info::render_info_table,
    params::{render_path_params_table, render_query_params_table},
};

pub(super) fn render_endpoint(
    path: &str,
    method: &str,
    op: &Operation,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let mut out = format!("# {method} {path}\n\n");
    out.push_str(&render_info_table(path, method, op, spec));
    out.push_str(&render_input_section(op, spec, multi_use));
    out.push_str(&render_responses_section(op, spec, multi_use));
    out
}

fn render_input_section(
    op: &Operation,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let params: Vec<_> = op
        .parameters
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
        .collect();
    let path_params: Vec<_> = params
        .iter()
        .filter(|p| p.location == ParameterIn::Path)
        .collect();
    let query_params: Vec<_> = params
        .iter()
        .filter(|p| p.location == ParameterIn::Query)
        .collect();

    if path_params.is_empty() && query_params.is_empty() && op.request_body.is_none() {
        return String::new();
    }

    let mut out = "## Input\n\n".to_string();
    out.push_str(&render_path_params_table(&path_params, spec));
    out.push_str(&render_query_params_table(&query_params, spec));
    out.push_str(&render_payload_section(op, spec, multi_use));
    out
}
