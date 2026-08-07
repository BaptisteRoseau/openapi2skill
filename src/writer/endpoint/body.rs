use std::collections::HashSet;

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, Operation, Response, Schema},
};
use tracing::warn;

use super::headers::render_response_headers_table;
use super::links::render_response_links_table;
use super::refs::resolve_response;
use crate::writer::{
    schema as schema_writer,
    utils::{desc_paragraph, schema_doc_link, schema_ref_name},
};

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
    if let Some(schema) = preferred_schema(&body.content) {
        out.push_str(&render_schema_block(schema, spec, multi_use));
    }
    out
}

/// The JSON media type when present, else whichever content type comes first.
fn preferred_schema(content: &oas3::Map<String, oas3::spec::MediaType>) -> Option<&Schema> {
    content
        .get("application/json")
        .or_else(|| content.values().next())?
        .schema
        .as_ref()
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
    out.push_str(&desc_paragraph(resp.description.as_deref()));
    out.push_str(&render_response_headers_table(&resp.headers, spec));
    if let Some(schema) = preferred_schema(&resp.content) {
        out.push_str(&render_schema_block(schema, spec, multi_use));
    }
    out.push_str(&render_response_links_table(&resp.links, spec));
    out
}

fn render_schema_block(
    schema: &Schema,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    if let Some(ref_name) = schema_ref_name(schema)
        && multi_use.contains(ref_name)
    {
        return format!("See [{ref_name}]({})\n\n", schema_doc_link(ref_name));
    }
    format!(
        "```jsonc\n{}\n```\n\n",
        schema_writer::render_schema_jsonc(schema, spec, multi_use)
    )
}
