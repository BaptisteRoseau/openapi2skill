use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use oas3::{
    OpenApiV3Spec,
    spec::{
        ObjectOrReference, ObjectSchema, Operation, Parameter, ParameterIn, Response, Schema,
        SchemaType,
    },
};
use tracing::info;

use super::utils::{
    CollectWrites, camel_to_kebab, category_label, op_category, path_to_slug, primary_type,
};
use crate::writer::schema as schema_writer;

pub(super) struct Writer;

impl CollectWrites for Writer {
    fn collect_writes(
        &self,
        spec: &OpenApiV3Spec,
        dir: &Path,
        writes: &mut Vec<(PathBuf, String)>,
    ) {
        let multi_use = collect_multi_use_schemas(spec);
        let mut by_category: HashMap<String, Vec<(String, String, String)>> = HashMap::new();

        for (path, method, op) in spec.operations() {
            let cat_slug = op_category(op, &path);
            let filename = format!(
                "{}-{}.md",
                method.as_str().to_lowercase(),
                path_to_slug(&path)
            );
            let summary = op.summary.as_deref().unwrap_or(path.as_str()).to_string();
            let content = render_endpoint(&path, method.as_str(), op, spec, &multi_use);
            by_category
                .entry(cat_slug)
                .or_default()
                .push((filename, summary, content));
        }

        let mut sorted_cats: Vec<&String> = by_category.keys().collect();
        sorted_cats.sort();

        let top_index: String = sorted_cats
            .iter()
            .map(|slug| format!("- [{}](./{slug}/index.md)\n", category_label(slug)))
            .collect();
        let write_path = (dir.join("endpoints").join("index.md"), top_index);
        info!("Writing {:?}", write_path.0);
        writes.push(write_path);

        for (cat_slug, entries) in &by_category {
            push_category_writes(cat_slug, entries, dir, writes);
        }
    }
}

fn push_category_writes(
    cat_slug: &str,
    entries: &[(String, String, String)],
    dir: &Path,
    writes: &mut Vec<(PathBuf, String)>,
) {
    let cat_dir = dir.join("endpoints").join(cat_slug);

    let index = entries
        .iter()
        .map(|(filename, summary, _)| format!("- [{summary}](./{filename})"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let write_path = (cat_dir.join("index.md"), index);
    info!("Writing {:?}", write_path.0);
    writes.push(write_path);

    for (filename, _, content) in entries {
        let write_path = (cat_dir.join(filename), content.clone());
        info!("Writing {:?}", write_path.0);
        writes.push(write_path);
    }
}

fn collect_multi_use_schemas(spec: &OpenApiV3Spec) -> HashSet<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for (_, _, op) in spec.operations() {
        for name in collect_op_refs(op, spec) {
            *counts.entry(name).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .map(|(name, _)| name)
        .collect()
}

fn collect_op_refs(op: &Operation, spec: &OpenApiV3Spec) -> HashSet<String> {
    collect_body_refs(op, spec)
        .into_iter()
        .chain(collect_response_refs(op, spec))
        .collect()
}

fn collect_body_refs(op: &Operation, spec: &OpenApiV3Spec) -> Vec<String> {
    let Some(body_ref) = &op.request_body else {
        return Vec::new();
    };
    let Ok(body) = body_ref.resolve(spec) else {
        return Vec::new();
    };
    body.content
        .values()
        .filter_map(|mt| mt.schema.as_ref().and_then(top_level_ref_name))
        .map(str::to_string)
        .collect()
}

fn collect_response_refs(op: &Operation, spec: &OpenApiV3Spec) -> Vec<String> {
    let Some(responses) = &op.responses else {
        return Vec::new();
    };
    responses
        .values()
        .filter_map(|resp_ref| resolve_response(resp_ref, spec))
        .flat_map(|resp| {
            resp.content
                .values()
                .filter_map(|mt| {
                    mt.schema
                        .as_ref()
                        .and_then(top_level_ref_name)
                        .map(str::to_string)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn resolve_response(
    resp_ref: &ObjectOrReference<Response>,
    spec: &OpenApiV3Spec,
) -> Option<Response> {
    match resp_ref {
        ObjectOrReference::Object(r) => Some(r.clone()),
        ObjectOrReference::Ref { .. } => resp_ref.resolve(spec).ok(),
    }
}

fn top_level_ref_name(schema: &Schema) -> Option<&str> {
    match schema {
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Ref { ref_path, .. } => {
                ref_path.strip_prefix("#/components/schemas/")
            }
            _ => None,
        },
        _ => None,
    }
}

fn render_endpoint(
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

fn render_info_table(path: &str, method: &str, op: &Operation, spec: &OpenApiV3Spec) -> String {
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
    let body = op.request_body.as_ref()?.resolve(spec).ok()?;
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

fn render_input_section(
    op: &Operation,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let params: Vec<_> = op
        .parameters
        .iter()
        .filter_map(|p| p.resolve(spec).ok())
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

fn render_path_params_table(params: &[&Parameter], spec: &OpenApiV3Spec) -> String {
    if params.is_empty() {
        return String::new();
    }
    let mut out = "### Path Parameters\n\n| Parameter | Type | Required | Description |\n|-----------|------|----------|-------------|\n".to_string();
    for p in params {
        let req = if p.required.unwrap_or(true) {
            "Yes"
        } else {
            "No"
        };
        out.push_str(&format!(
            "| `{}` | {} | {req} | {} |\n",
            p.name,
            render_param_type(&p.schema, spec),
            p.description.as_deref().unwrap_or("-"),
        ));
    }
    out.push('\n');
    out
}

fn render_query_params_table(params: &[&Parameter], spec: &OpenApiV3Spec) -> String {
    if params.is_empty() {
        return String::new();
    }
    let mut out = "### Query Parameters\n\n| Parameter | Type | Required | Description |\n|-----------|------|----------|-------------|\n".to_string();
    for p in params {
        let req = if p.required.unwrap_or(false) {
            "Yes"
        } else {
            "No"
        };
        out.push_str(&format!(
            "| `{}` | {} | {req} | {} |\n",
            p.name,
            render_param_type(&p.schema, spec),
            p.description.as_deref().unwrap_or("-"),
        ));
    }
    out.push('\n');
    out
}

fn render_param_type(schema: &Option<Schema>, spec: &OpenApiV3Spec) -> String {
    let schema = match schema {
        None => return "string".to_string(),
        Some(s) => s,
    };
    let resolved = match schema.resolve(spec) {
        Ok(r) => r,
        Err(_) => return "unknown".to_string(),
    };
    match resolved {
        Schema::Boolean(_) => "boolean".to_string(),
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => render_param_object_type(obj),
            ObjectOrReference::Ref { ref_path, .. } => ref_path
                .strip_prefix("#/components/schemas/")
                .unwrap_or(ref_path)
                .to_string(),
        },
    }
}

fn render_param_object_type(obj: &ObjectSchema) -> String {
    let ty = obj.schema_type.as_ref().map(primary_type);
    let mut base = param_base_type(ty, obj.format.as_deref());
    let constraints = param_constraints(obj);
    if !constraints.is_empty() {
        base = format!("{base} ({})", constraints.join(", "));
    }
    if obj.enum_values.is_empty() {
        return base;
    }
    let vals = obj
        .enum_values
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| format!("`{s}`"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{base} ({vals})")
}

fn param_base_type(ty: Option<SchemaType>, fmt: Option<&str>) -> String {
    match ty {
        None => "any".to_string(),
        Some(SchemaType::Integer) => fmt
            .map(|f| format!("integer ({f})"))
            .unwrap_or_else(|| "integer".to_string()),
        Some(SchemaType::Number) => fmt
            .map(|f| format!("number ({f})"))
            .unwrap_or_else(|| "number".to_string()),
        Some(SchemaType::String) => fmt
            .map(|f| format!("string ({f})"))
            .unwrap_or_else(|| "string".to_string()),
        Some(SchemaType::Boolean) => "boolean".to_string(),
        Some(SchemaType::Array) => "array".to_string(),
        Some(SchemaType::Object) => "object".to_string(),
        Some(SchemaType::Null) => "null".to_string(),
    }
}

fn param_constraints(obj: &ObjectSchema) -> Vec<String> {
    let mut cs = Vec::new();
    match (obj.minimum.as_ref(), obj.maximum.as_ref()) {
        (Some(lo), Some(hi)) => cs.push(format!("{lo}..{hi}")),
        (Some(lo), None) => cs.push(format!("≥{lo}")),
        (None, Some(hi)) => cs.push(format!("≤{hi}")),
        _ => {}
    }
    if let Some(max_len) = obj.max_length {
        cs.push(format!("max_len:{max_len}"));
    }
    cs
}

fn render_payload_section(
    op: &Operation,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let Some(body_ref) = &op.request_body else {
        return String::new();
    };
    let Ok(body) = body_ref.resolve(spec) else {
        return String::new();
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

fn render_schema_block(
    schema: &Schema,
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

fn render_responses_section(
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
