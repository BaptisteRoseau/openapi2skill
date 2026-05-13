use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use oas3::{
    spec::{ObjectOrReference, Operation, ParameterIn, Response},
    OpenApiV3Spec,
};

use super::{path_to_slug, to_snake_case};
use crate::writer::schema;

pub fn collect_writes(spec: &OpenApiV3Spec, dir: &Path, writes: &mut Vec<(PathBuf, String)>) {
    // category slug -> Vec<(filename, summary, rendered_content)>
    let mut by_category: HashMap<String, Vec<(String, String, String)>> = HashMap::new();

    for (path, method, op) in spec.operations() {
        let cat = op.tags.first().cloned().unwrap_or_else(|| "general".to_string());
        let cat_slug = to_snake_case(&cat);
        let method_str = method.as_str().to_lowercase();
        let slug = path_to_slug(&path);
        let filename = format!("{method_str}-{slug}.md");
        let summary = op.summary.as_deref().unwrap_or(path.as_str()).to_string();
        let content = render_endpoint(&path, method.as_str(), op, spec);
        by_category.entry(cat_slug).or_default().push((filename, summary, content));
    }

    for (cat_slug, entries) in &by_category {
        let cat_dir = dir.join(cat_slug);

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

fn render_endpoint(path: &str, method: &str, op: &Operation, spec: &OpenApiV3Spec) -> String {
    let mut out = String::new();

    out.push_str(&format!("# {method} {path}\n\n"));

    // Info table
    out.push_str("| | |\n|--|--|\n");
    out.push_str(&format!("| **Method** | `{method}` |\n"));
    out.push_str(&format!("| **URL** | `{path}` |\n"));
    out.push_str(&format!("| **Auth** | {} |\n", render_security(&op.security)));
    if op.request_body.is_some() {
        out.push_str("| **Content-Type** | `application/json` |\n");
    }
    out.push('\n');

    // Resolve parameters
    let params: Vec<_> = op
        .parameters
        .iter()
        .filter_map(|p| p.resolve(spec).ok())
        .collect();

    let path_params: Vec<_> = params.iter().filter(|p| p.location == ParameterIn::Path).collect();
    let query_params: Vec<_> = params.iter().filter(|p| p.location == ParameterIn::Query).collect();

    if !path_params.is_empty() || !query_params.is_empty() || op.request_body.is_some() {
        out.push_str("## Input\n\n");
    }

    if !path_params.is_empty() {
        out.push_str("### Path Parameters\n\n");
        out.push_str("| Parameter | Type | Required | Description |\n");
        out.push_str("|-----------|------|----------|-------------|\n");
        for p in &path_params {
            let type_ = render_param_type(&p.schema, spec);
            let req = if p.required.unwrap_or(true) { "Yes" } else { "No" };
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
            let req = if p.required.unwrap_or(false) { "Yes" } else { "No" };
            let desc = p.description.as_deref().unwrap_or("-");
            out.push_str(&format!("| `{}` | {type_} | {req} | {desc} |\n", p.name));
        }
        out.push('\n');
    }

    if let Some(body_ref) = &op.request_body {
        if let Ok(body) = body_ref.resolve(spec) {
            out.push_str("### Payload\n\n");
            let media = body.content.get("application/json").or_else(|| body.content.values().next());
            if let Some(mt) = media {
                if let Some(schema) = &mt.schema {
                    out.push_str("```jsonc\n");
                    out.push_str(&schema::render_schema_jsonc(schema, spec));
                    out.push('\n');
                    out.push_str("```\n\n");
                }
            }
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
            if let Some(desc) = &resp.description {
                out.push_str(desc);
                out.push_str("\n\n");
            }
            render_response_body(&resp, spec, &mut out);
        }
    }

    out
}

fn render_response_body(resp: &Response, spec: &OpenApiV3Spec, out: &mut String) {
    let media = resp.content.get("application/json").or_else(|| resp.content.values().next());
    if let Some(mt) = media {
        if let Some(schema) = &mt.schema {
            out.push_str("```jsonc\n");
            out.push_str(&schema::render_schema_jsonc(schema, spec));
            out.push('\n');
            out.push_str("```\n\n");
        }
    }
}

fn render_security(security: &[oas3::spec::SecurityRequirement]) -> String {
    if security.is_empty() {
        return "None".to_string();
    }
    let parts: Vec<String> = security
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
    if parts.is_empty() { "None".to_string() } else { parts.join("; ") }
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
                let base = match obj.schema_type.as_ref() {
                    None => "any".to_string(),
                    Some(ts) => {
                        let t = match ts {
                            SchemaTypeSet::Single(t) => *t,
                            SchemaTypeSet::Multiple(ts) => {
                                ts.iter().copied().find(|t| *t != SchemaType::Null).unwrap_or(SchemaType::Object)
                            }
                        };
                        match t {
                            SchemaType::Integer => {
                                if let Some(fmt) = &obj.format { format!("integer ({fmt})") }
                                else { "integer".to_string() }
                            }
                            SchemaType::Number => "number".to_string(),
                            SchemaType::Boolean => "boolean".to_string(),
                            SchemaType::String => "string".to_string(),
                            SchemaType::Array => "array".to_string(),
                            SchemaType::Object => "object".to_string(),
                            SchemaType::Null => "null".to_string(),
                        }
                    }
                };
                if !obj.enum_values.is_empty() {
                    let vals = obj.enum_values.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| format!("`{s}`"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{base} ({vals})")
                } else {
                    base
                }
            }
            ObjectOrReference::Ref { ref_path, .. } => {
                ref_path.strip_prefix("#/components/schemas/").unwrap_or(ref_path).to_string()
            }
        },
    }
}
