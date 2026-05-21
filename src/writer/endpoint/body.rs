use std::collections::HashSet;

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, Operation, Response},
};
use tracing::warn;

use super::refs::{resolve_response, top_level_ref_name};
use crate::writer::{schema as schema_writer, utils::camel_to_kebab};

pub(super) fn render_payload_section(
    op: &Operation,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let Some(body_ref) = &op.request_body else {
        return String::new();
    };
    let body = match body_ref.resolve(spec) {
        Ok(b) => b,
        Err(err) => {
            warn!(
                operation_id = ?op.operation_id,
                "could not resolve request body: {err}; omitting payload section"
            );
            return String::new();
        }
    };
    let mut out = "### Payload\n\n".to_string();
    let media = body
        .content
        .get("application/json")
        .or_else(|| body.content.values().next());
    if let Some(mt) = media
        && let Some(schema) = &mt.schema
    {
        out.push_str(&render_schema_block(schema, spec, multi_use));
    }
    out
}

pub(super) fn render_responses_section(
    op: &Operation,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let Some(responses) = &op.responses else {
        return String::new();
    };
    responses
        .iter()
        .map(|(code, resp_ref)| render_response(code, resp_ref, spec, multi_use))
        .collect()
}

fn render_response(
    code: &str,
    resp_ref: &ObjectOrReference<Response>,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let Some(resp) = resolve_response(resp_ref, spec) else {
        warn!(
            status = code,
            "could not resolve response; omitting response section"
        );
        return String::new();
    };
    let mut out = format!("## Response {code}\n\n");
    if !resp.content.is_empty() {
        let types = resp
            .content
            .keys()
            .map(|k| format!("`{k}`"))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("**Response Content-Type:** {types}\n\n"));
    }
    if let Some(desc) = &resp.description {
        out.push_str(desc);
        out.push_str("\n\n");
    }
    out.push_str(&render_response_body(&resp, spec, multi_use));
    out
}

fn render_response_body(
    resp: &Response,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let media = resp
        .content
        .get("application/json")
        .or_else(|| resp.content.values().next());
    if let Some(mt) = media
        && let Some(schema) = &mt.schema
    {
        render_schema_block(schema, spec, multi_use)
    } else {
        String::new()
    }
}

fn render_schema_block(
    schema: &oas3::spec::Schema,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    if let Some(ref_name) = top_level_ref_name(schema)
        && multi_use.contains(ref_name)
    {
        let slug = camel_to_kebab(ref_name);
        return format!("See [{ref_name}](../../schemas/{slug}.md)\n\n");
    }
    format!(
        "```jsonc\n{}\n```\n\n",
        schema_writer::render_schema_jsonc(schema, spec, multi_use)
    )
}
