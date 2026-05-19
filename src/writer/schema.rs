//! Generates `schemas/index.md` and one `schemas/{name}.md` per component schema.
//! `render_schema_jsonc` is also used by `endpoint.rs` for inline request/response bodies.
//!
//! **`schemas/index.md`**
//!
//! - [Ack](./ack.md)
//! - [ActiveSyncStatusDTO](./active-sync-status-d-t-o.md)
//! - [AddDataSourceCommand](./add-data-source-command.md)
//! - ...
//!
//! **`schemas/ack.md`** (empty object)
//!
//! # Ack
//!
//! ```jsonc
//! {}
//! ```
//!
//! **`schemas/add-data-source-command.md`** (object with properties)
//!
//! # AddDataSourceCommand
//!
//! ```jsonc
//! {
//!   "access": "string",       // string, optional
//!   "basicAuth": false,        // boolean, optional
//!   "basicAuthUser": "string", // string, optional
//!   "isDefault": false,        // boolean, optional
//!   "name": "string",          // string, required
//!   "type": "string",          // string, required
//!   "uid": "string",           // string, optional
//!   "url": "string",           // string, optional
//!   "jsonData": { /* [Json](../../schemas/json.md) */ },  // object, optional
//!   "secureJsonData": { /* [Json](../../schemas/json.md) */ }  // object, optional
//! }
//! ```

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, ObjectSchema, Schema, SchemaType},
};
use tracing::info;

use super::utils::{CollectWrites, build_index, camel_to_kebab, primary_type};

pub(super) struct Writer;

/// State threaded through schema rendering. `visiting` tracks ref names currently
/// on the recursion stack so cyclic `$ref`s render as links rather than overflowing.
struct RenderCtx<'a> {
    spec: &'a OpenApiV3Spec,
    multi_use: &'a HashSet<String>,
    visiting: HashSet<String>,
}

impl CollectWrites for Writer {
    fn collect_writes(
        &self,
        spec: &OpenApiV3Spec,
        dir: &Path,
        writes: &mut Vec<(PathBuf, String)>,
    ) {
        let Some(components) = &spec.components else {
            return;
        };
        if components.schemas.is_empty() {
            return;
        }

        let schema_dir = dir.join("schemas");
        let mut index_links: Vec<(String, String)> = Vec::new();

        for (name, schema) in &components.schemas {
            let filename = format!("{}.md", camel_to_kebab(name));
            let content = render_schema_file(name, schema, spec);
            let write_path = (schema_dir.join(&filename), content);
            info!("Writing {:?}", write_path.0);
            writes.push(write_path);
            index_links.push((filename, name.clone()));
        }

        let write_path = (schema_dir.join("index.md"), build_index(&index_links));
        info!("Writing {:?}", write_path.0);
        writes.push(write_path);
    }
}

fn render_schema_file(name: &str, schema: &Schema, spec: &OpenApiV3Spec) -> String {
    let description = schema_description(schema, spec);
    let mut out = format!("# {name}\n\n");
    if let Some(desc) = description {
        out.push_str(&format!("{desc}\n\n"));
    }
    out.push_str("```jsonc\n");
    out.push_str(&render_schema_jsonc(schema, spec, &HashSet::new()));
    out.push('\n');
    out.push_str("```\n");
    out
}

fn schema_description(schema: &Schema, spec: &OpenApiV3Spec) -> Option<String> {
    match schema.resolve(spec) {
        Ok(Schema::Object(oor)) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => obj.description.clone(),
            _ => None,
        },
        _ => None,
    }
}

pub fn render_schema_jsonc(
    schema: &Schema,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let mut ctx = RenderCtx {
        spec,
        multi_use,
        visiting: HashSet::new(),
    };
    render_schema_jsonc_inner(schema, &mut ctx)
}

fn render_schema_jsonc_inner(schema: &Schema, ctx: &mut RenderCtx<'_>) -> String {
    let resolved = match schema.resolve(ctx.spec) {
        Ok(s) => s,
        Err(_) => return "{}".to_string(),
    };

    match resolved {
        Schema::Boolean(b) => b.0.to_string(),
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => render_top_level_object(obj, ctx),
            ObjectOrReference::Ref { .. } => "{}".to_string(),
        },
    }
}

fn render_top_level_object(obj: &ObjectSchema, ctx: &mut RenderCtx<'_>) -> String {
    if obj
        .schema_type
        .as_ref()
        .map(|ts| ts.is_array_or_nullable_array())
        .unwrap_or(false)
    {
        return render_top_level_array(obj, ctx);
    }
    if obj.properties.is_empty() {
        return "{}".to_string();
    }
    let mut lines = vec!["{".to_string()];
    lines.extend(render_properties_lines(obj, 1, ctx));
    lines.push("}".to_string());
    lines.join("\n")
}

fn render_top_level_array(obj: &ObjectSchema, ctx: &mut RenderCtx<'_>) -> String {
    let item_lines = obj
        .items
        .as_ref()
        .map(|items| array_item_lines(items, 1, ctx))
        .unwrap_or_else(|| vec!["  null".to_string()]);
    let mut lines = vec!["[".to_string()];
    lines.extend(item_lines);
    lines.push("]".to_string());
    lines.join("\n")
}

fn render_properties_lines(
    obj: &ObjectSchema,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
    let props: Vec<_> = obj.properties.iter().collect();
    let n = props.len();
    let mut out = Vec::with_capacity(n);
    for (i, (name, schema)) in props.into_iter().enumerate() {
        let trail = if i + 1 == n { "" } else { "," };
        let is_req = obj.required.contains(name);
        out.extend(property_lines(name, schema, is_req, trail, depth, ctx));
    }
    out
}

fn property_lines(
    name: &str,
    schema: &Schema,
    is_required: bool,
    trail: &str,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
    let indent = "  ".repeat(depth);
    let req = if is_required { "required" } else { "optional" };

    if let Some(ref_name) = schema_ref_name(schema)
        && (ctx.multi_use.contains(ref_name) || ctx.visiting.contains(ref_name))
    {
        let slug = camel_to_kebab(ref_name);
        let link = format!("../../schemas/{slug}.md");
        return vec![format!(
            "{indent}\"{name}\": {{ /* [{ref_name}]({link}) */ }}{trail}  // object, {req}"
        )];
    }

    let pushed = schema_ref_name(schema).map(|n| n.to_string());
    if let Some(n) = &pushed {
        ctx.visiting.insert(n.clone());
    }

    let lines = resolved_property_lines(name, schema, is_required, trail, depth, ctx);

    if let Some(n) = &pushed {
        ctx.visiting.remove(n);
    }
    lines
}

fn resolved_property_lines(
    name: &str,
    schema: &Schema,
    is_required: bool,
    trail: &str,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
    let indent = "  ".repeat(depth);
    let req = if is_required { "required" } else { "optional" };
    let resolved = match schema.resolve(ctx.spec) {
        Ok(s) => s,
        Err(_) => {
            return vec![format!(
                "{indent}\"{name}\": null{trail}  // unknown, {req}"
            )];
        }
    };

    match resolved {
        Schema::Boolean(b) => vec![format!(
            "{indent}\"{name}\": {}{trail}  // boolean, {req}",
            b.0
        )],
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => {
                object_property_lines(name, obj, is_required, trail, depth, ctx)
            }
            ObjectOrReference::Ref { .. } => vec![format!(
                "{indent}\"{name}\": null{trail}  // unresolved ref, {req}"
            )],
        },
    }
}

fn object_property_lines(
    name: &str,
    obj: &ObjectSchema,
    is_required: bool,
    trail: &str,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
    let indent = "  ".repeat(depth);
    let req = if is_required { "required" } else { "optional" };

    if obj
        .schema_type
        .as_ref()
        .map(|ts| ts.is_array_or_nullable_array())
        .unwrap_or(false)
    {
        let item_type = array_item_type_label(obj, ctx.spec);
        let item_lines = obj
            .items
            .as_ref()
            .map(|items| array_item_lines(items, depth + 1, ctx))
            .unwrap_or_else(|| vec![format!("{}null", "  ".repeat(depth + 1))]);
        let mut lines = vec![format!(
            "{indent}\"{name}\": [  // array of {item_type}, {req}"
        )];
        lines.extend(item_lines);
        lines.push(format!("{indent}]{trail}"));
        return lines;
    }

    if !obj.properties.is_empty() {
        let mut lines = vec![format!("{indent}\"{name}\": {{")];
        lines.extend(render_properties_lines(obj, depth + 1, ctx));
        lines.push(format!("{indent}}}{trail}"));
        return lines;
    }

    let example = primitive_example(obj);
    let comment = type_comment(obj, req);
    vec![format!(
        "{indent}\"{name}\": {example}{trail}  // {comment}"
    )]
}

fn array_item_lines(items: &Schema, depth: usize, ctx: &mut RenderCtx<'_>) -> Vec<String> {
    let indent = "  ".repeat(depth);

    if let Some(ref_name) = schema_ref_name(items)
        && (ctx.multi_use.contains(ref_name) || ctx.visiting.contains(ref_name))
    {
        let slug = camel_to_kebab(ref_name);
        let link = format!("../../schemas/{slug}.md");
        return vec![format!("{indent}{{ /* [{ref_name}]({link}) */ }}")];
    }

    let pushed = schema_ref_name(items).map(|n| n.to_string());
    if let Some(n) = &pushed {
        ctx.visiting.insert(n.clone());
    }

    let lines = resolved_array_item_lines(items, depth, ctx, &indent);

    if let Some(n) = &pushed {
        ctx.visiting.remove(n);
    }
    lines
}

fn resolved_array_item_lines(
    items: &Schema,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
    indent: &str,
) -> Vec<String> {
    let resolved = match items.resolve(ctx.spec) {
        Ok(s) => s,
        Err(_) => return vec![format!("{indent}null")],
    };

    match resolved {
        Schema::Boolean(b) => vec![format!("{indent}{}", b.0)],
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) if !obj.properties.is_empty() => {
                let mut lines = vec![format!("{indent}{{")];
                lines.extend(render_properties_lines(obj, depth + 1, ctx));
                lines.push(format!("{indent}}}"));
                lines
            }
            ObjectOrReference::Object(obj) => vec![format!("{indent}{}", primitive_example(obj))],
            ObjectOrReference::Ref { .. } => vec![format!("{indent}null")],
        },
    }
}

fn schema_ref_name(schema: &Schema) -> Option<&str> {
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

fn array_item_type_label(array_obj: &ObjectSchema, _spec: &OpenApiV3Spec) -> String {
    let Some(items) = &array_obj.items else {
        return "any".to_string();
    };
    match items.as_ref() {
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Ref { ref_path, .. } => ref_path
                .strip_prefix("#/components/schemas/")
                .unwrap_or(ref_path)
                .to_string(),
            ObjectOrReference::Object(obj) => primitive_type_name(obj),
        },
        Schema::Boolean(_) => "boolean".to_string(),
    }
}

fn primitive_type_name(obj: &ObjectSchema) -> String {
    match obj.schema_type.as_ref().map(primary_type) {
        Some(SchemaType::String) => "string".to_string(),
        Some(SchemaType::Integer) => "integer".to_string(),
        Some(SchemaType::Number) => "number".to_string(),
        Some(SchemaType::Boolean) => "boolean".to_string(),
        _ => "object".to_string(),
    }
}

fn primitive_example(obj: &ObjectSchema) -> String {
    if let Some(ex) = &obj.example
        && !ex.is_object()
        && !ex.is_array()
    {
        return ex.to_string();
    }
    if let Some(val) = obj.enum_values.first() {
        return val.to_string();
    }
    if let Some(def) = &obj.default
        && !def.is_object()
        && !def.is_array()
    {
        return def.to_string();
    }
    match obj.schema_type.as_ref().map(primary_type) {
        Some(SchemaType::Integer) => "0".to_string(),
        Some(SchemaType::Number) => "0.0".to_string(),
        Some(SchemaType::Boolean) => "false".to_string(),
        Some(SchemaType::String) => "\"string\"".to_string(),
        _ => "null".to_string(),
    }
}

fn type_comment(obj: &ObjectSchema, req: &str) -> String {
    let ty = obj.schema_type.as_ref().map(primary_type);
    let fmt = obj.format.as_deref();
    let mut parts = vec![type_base_name(ty, fmt)];
    if let Some(f) = fmt
        && !matches!(ty, Some(SchemaType::Integer))
    {
        parts.push(format!("format: {f}"));
    }
    parts.push(req.to_string());
    if let Some(desc) = &obj.description {
        parts.push(desc.trim().to_string());
    }
    parts.extend(collect_type_constraints(obj));
    if !obj.enum_values.is_empty() {
        let vals = obj
            .enum_values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        parts.push(format!("enum: {vals}"));
    }
    parts.join(", ")
}

fn type_base_name(ty: Option<SchemaType>, fmt: Option<&str>) -> String {
    match ty {
        Some(SchemaType::Integer) => fmt
            .map(|f| format!("integer ({f})"))
            .unwrap_or_else(|| "integer".to_string()),
        Some(SchemaType::Number) => "number".to_string(),
        Some(SchemaType::Boolean) => "boolean".to_string(),
        Some(SchemaType::String) => "string".to_string(),
        Some(SchemaType::Array) => "array".to_string(),
        Some(SchemaType::Object) => "object".to_string(),
        Some(SchemaType::Null) => "null".to_string(),
        None => "any".to_string(),
    }
}

fn collect_type_constraints(obj: &ObjectSchema) -> Vec<String> {
    let mut parts = Vec::new();
    if let Some(min) = &obj.minimum {
        parts.push(format!("min: {min}"));
    }
    if let Some(xmin) = &obj.exclusive_minimum {
        parts.push(format!("xmin: {xmin}"));
    }
    if let Some(max) = &obj.maximum {
        parts.push(format!("max: {max}"));
    }
    if let Some(xmax) = &obj.exclusive_maximum {
        parts.push(format!("xmax: {xmax}"));
    }
    if let Some(min_len) = obj.min_length {
        parts.push(format!("minLength: {min_len}"));
    }
    if let Some(max_len) = obj.max_length {
        parts.push(format!("maxLength: {max_len}"));
    }
    if let Some(pat) = &obj.pattern {
        parts.push(format!("pattern: \"{pat}\""));
    }
    if let Some(min_items) = obj.min_items {
        parts.push(format!("minItems: {min_items}"));
    }
    if let Some(max_items) = obj.max_items {
        parts.push(format!("maxItems: {max_items}"));
    }
    parts
}
