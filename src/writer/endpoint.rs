use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, Operation, ParameterIn, Response, Schema},
};

use super::{op_category, path_to_slug};
use crate::writer::schema as schema_writer;

pub fn collect_writes(spec: &OpenApiV3Spec, dir: &Path, writes: &mut Vec<(PathBuf, String)>) {
    // Pass 1 — count how many distinct operations directly reference each named schema
    // as a request/response body $ref.  Schemas referenced in 2+ operations are rendered
    // as a markdown link instead of being inlined, preventing giant repeated blocks.
    let multi_use = collect_multi_use_schemas(spec);

    // category slug -> Vec<(filename, summary, rendered_content)>
    let mut by_category: HashMap<String, Vec<(String, String, String)>> = HashMap::new();

    for (path, method, op) in spec.operations() {
        let cat_slug = op_category(op, &path);
        let method_str = method.as_str().to_lowercase();
        let slug = path_to_slug(&path);
        let filename = format!("{method_str}-{slug}.md");
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
        .map(|slug| {
            let desc = super::category_label(slug);
            format!("- [{desc}](./{slug}/index.md)\n")
        })
        .collect();
    writes.push((dir.join("endpoints").join("index.md"), top_index));

    for (cat_slug, entries) in &by_category {
        let cat_dir = dir.join("endpoints").join(cat_slug);

        let index_links: Vec<String> = entries
            .iter()
            .map(|(filename, summary, _)| format!("- [{summary}](./{filename})"))
            .collect();
        let index = index_links.join("\n") + "\n";
        writes.push((cat_dir.join("index.md"), index));

        for (filename, _, content) in entries {
            writes.push((cat_dir.join(filename), content.clone()));
        }
    }
}

/// Return the set of component schema names that are directly referenced (as a `$ref`) in the
/// request or response body of 2 or more distinct operations.
fn collect_multi_use_schemas(spec: &OpenApiV3Spec) -> HashSet<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for (_, _, op) in spec.operations() {
        // Collect all top-level $ref names touched by this operation (deduped per operation).
        let mut op_refs: HashSet<String> = HashSet::new();

        // Request body
        if let Some(body_ref) = &op.request_body {
            if let Ok(body) = body_ref.resolve(spec) {
                for mt in body.content.values() {
                    if let Some(schema) = &mt.schema {
                        if let Some(name) = top_level_ref_name(schema) {
                            op_refs.insert(name.to_string());
                        }
                    }
                }
            }
        }

        // Responses
        if let Some(responses) = &op.responses {
            for resp_ref in responses.values() {
                let resp = match resp_ref {
                    ObjectOrReference::Object(r) => r.clone(),
                    ObjectOrReference::Ref { .. } => match resp_ref.resolve(spec) {
                        Ok(r) => r,
                        Err(_) => continue,
                    },
                };
                for mt in resp.content.values() {
                    if let Some(schema) = &mt.schema {
                        if let Some(name) = top_level_ref_name(schema) {
                            op_refs.insert(name.to_string());
                        }
                    }
                }
            }
        }

        for name in op_refs {
            *counts.entry(name).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .map(|(name, _)| name)
        .collect()
}

/// If `schema` is a bare `$ref` to a component schema, return the schema name.
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
    let mut out = String::new();

    out.push_str(&format!("# {method} {path}\n\n"));

    // Info table
    out.push_str("| | |\n|--|--|\n");
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
    if matches!(method, "POST" | "PUT" | "PATCH")
        && let Some(body_ref) = &op.request_body
        && let Ok(body) = body_ref.resolve(spec)
        && !body.content.is_empty()
    {
        let types = body
            .content
            .keys()
            .map(|k| format!("`{k}`"))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("| **Request Content-Type** | {types} |\n"));
    }
    out.push('\n');

    // Resolve parameters
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

    if !path_params.is_empty() || !query_params.is_empty() || op.request_body.is_some() {
        out.push_str("## Input\n\n");
    }

    if !path_params.is_empty() {
        out.push_str("### Path Parameters\n\n");
        out.push_str("| Parameter | Type | Required | Description |\n");
        out.push_str("|-----------|------|----------|-------------|\n");
        for p in &path_params {
            let type_ = render_param_type(&p.schema, spec);
            let req = if p.required.unwrap_or(true) {
                "Yes"
            } else {
                "No"
            };
            let desc = p.description.as_deref().unwrap_or("-");
            out.push_str(&format!("| `{}` | {type_} | {req} | {desc} |\n", p.name));
        }
        out.push('\n');
    }

    if !query_params.is_empty() {
        out.push_str("### Query Parameters\n\n");
        out.push_str("| Parameter | Type | Required | Description |\n");
        out.push_str("|-----------|------|----------|-------------|\n");
        for p in &query_params {
            let type_ = render_param_type(&p.schema, spec);
            let req = if p.required.unwrap_or(false) {
                "Yes"
            } else {
                "No"
            };
            let desc = p.description.as_deref().unwrap_or("-");
            out.push_str(&format!("| `{}` | {type_} | {req} | {desc} |\n", p.name));
        }
        out.push('\n');
    }

    if let Some(body_ref) = &op.request_body
        && let Ok(body) = body_ref.resolve(spec)
    {
        out.push_str("### Payload\n\n");
        let media = body
            .content
            .get("application/json")
            .or_else(|| body.content.values().next());
        if let Some(mt) = media
            && let Some(schema) = &mt.schema
        {
            render_schema_block(schema, spec, multi_use, &mut out);
        }
    }

    // Responses
    if let Some(responses) = &op.responses {
        for (code, resp_ref) in responses {
            let resp = match resp_ref {
                ObjectOrReference::Object(r) => r.clone(),
                ObjectOrReference::Ref { .. } => match resp_ref.resolve(spec) {
                    Ok(r) => r,
                    Err(_) => continue,
                },
            };
            out.push_str(&format!("## Response {code}\n\n"));
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
            render_response_body(&resp, spec, multi_use, &mut out);
        }
    }

    out
}

fn render_response_body(
    resp: &Response,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
    out: &mut String,
) {
    let media = resp
        .content
        .get("application/json")
        .or_else(|| resp.content.values().next());
    if let Some(mt) = media
        && let Some(schema) = &mt.schema
    {
        render_schema_block(schema, spec, multi_use, out);
    }
}

/// Render a schema as either a markdown link (when it's a top-level $ref to a multi-use schema)
/// or a full jsonc block.
fn render_schema_block(
    schema: &Schema,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
    out: &mut String,
) {
    use super::camel_to_kebab;

    if let Some(ref_name) = top_level_ref_name(schema) {
        if multi_use.contains(ref_name) {
            let slug = camel_to_kebab(ref_name);
            out.push_str(&format!(
                "See [{}](../../schemas/{}.md)\n\n",
                ref_name, slug
            ));
            return;
        }
    }

    out.push_str("```jsonc\n");
    out.push_str(&schema_writer::render_schema_jsonc(schema, spec, multi_use));
    out.push('\n');
    out.push_str("```\n\n");
}

fn render_security(
    op_security: &[oas3::spec::SecurityRequirement],
    spec: &OpenApiV3Spec,
) -> String {
    // Per OpenAPI 3: absent per-op security inherits the global spec security.
    // The oas3 crate deserialises both absent and explicit [] as an empty Vec,
    // so we can't distinguish them — falling back is the safe default for APIs
    // (like Stripe) that authenticate every endpoint globally.
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
        .map(|(scheme, scopes)| {
            if scopes.is_empty() {
                scheme.clone()
            } else {
                format!("{scheme} (scopes: {})", scopes.join(", "))
            }
        })
        .collect();
    if parts.is_empty() {
        "None".to_string()
    } else {
        parts.join("; ")
    }
}

fn render_param_type(schema: &Option<oas3::spec::Schema>, spec: &OpenApiV3Spec) -> String {
    use oas3::spec::{Schema, SchemaType, SchemaTypeSet};

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
            ObjectOrReference::Object(obj) => {
                let ty = obj.schema_type.as_ref().map(|ts| match ts {
                    SchemaTypeSet::Single(t) => *t,
                    SchemaTypeSet::Multiple(ts) => ts
                        .iter()
                        .copied()
                        .find(|t| *t != SchemaType::Null)
                        .unwrap_or(SchemaType::Object),
                });
                let mut base = match ty {
                    None => "any".to_string(),
                    Some(SchemaType::Integer) => {
                        if let Some(fmt) = &obj.format {
                            format!("integer ({fmt})")
                        } else {
                            "integer".to_string()
                        }
                    }
                    Some(SchemaType::Number) => {
                        if let Some(fmt) = &obj.format {
                            format!("number ({fmt})")
                        } else {
                            "number".to_string()
                        }
                    }
                    Some(SchemaType::Boolean) => "boolean".to_string(),
                    Some(SchemaType::String) => {
                        if let Some(fmt) = &obj.format {
                            format!("string ({fmt})")
                        } else {
                            "string".to_string()
                        }
                    }
                    Some(SchemaType::Array) => "array".to_string(),
                    Some(SchemaType::Object) => "object".to_string(),
                    Some(SchemaType::Null) => "null".to_string(),
                };

                // Append concise constraints: range and length
                let mut constraints: Vec<String> = Vec::new();
                match (obj.minimum.as_ref(), obj.maximum.as_ref()) {
                    (Some(lo), Some(hi)) => constraints.push(format!("{lo}..{hi}")),
                    (Some(lo), None) => constraints.push(format!("≥{lo}")),
                    (None, Some(hi)) => constraints.push(format!("≤{hi}")),
                    _ => {}
                }
                if let Some(max_len) = obj.max_length {
                    constraints.push(format!("max_len:{max_len}"));
                }
                if !constraints.is_empty() {
                    base = format!("{base} ({})", constraints.join(", "));
                }

                if !obj.enum_values.is_empty() {
                    let vals = obj
                        .enum_values
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| format!("`{s}`"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{base} ({vals})")
                } else {
                    base
                }
            }
            ObjectOrReference::Ref { ref_path, .. } => ref_path
                .strip_prefix("#/components/schemas/")
                .unwrap_or(ref_path)
                .to_string(),
        },
    }
}
