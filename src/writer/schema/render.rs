use std::collections::HashSet;

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, ObjectSchema, Schema, SchemaType},
};
use tracing::warn;

use super::{
    composition::merge_all_of,
    context::RenderCtx,
    properties::{is_array, item_lines_or_null, render_properties_lines},
    types::{primitive_example, type_comment},
};
use crate::writer::utils::primary_type;

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

pub(super) fn render_schema_jsonc_inner(schema: &Schema, ctx: &mut RenderCtx<'_>) -> String {
    let resolved = match schema.resolve(ctx.spec) {
        Ok(s) => s,
        Err(err) => {
            warn!(
                "could not resolve top-level schema: {err}; falling back to empty object \"{{}}\""
            );
            return "{}".to_string();
        }
    };

    match resolved {
        Schema::Boolean(b) => b.0.to_string(),
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => render_top_level_object(obj, ctx),
            ObjectOrReference::Ref { ref_path, .. } => {
                warn!(
                    ref_path = %ref_path,
                    "top-level schema resolved to an unresolved $ref; falling back to empty object \"{{}}\""
                );
                "{}".to_string()
            }
        },
    }
}

pub(super) fn render_top_level_object(obj: &ObjectSchema, ctx: &mut RenderCtx<'_>) -> String {
    let merged = merge_all_of(obj, ctx);
    if is_array(&merged) {
        return wrap_lines("[", item_lines_or_null(&merged, 1, ctx), "]");
    }
    if is_primitive(&merged) {
        return render_top_level_primitive(&merged);
    }
    if merged.properties.is_empty() {
        return "{\n  // empty object\n}".to_string();
    }
    wrap_lines("{", render_properties_lines(&merged, 1, ctx), "}")
}

fn wrap_lines(open: &str, inner: Vec<String>, close: &str) -> String {
    let mut lines = vec![open.to_string()];
    lines.extend(inner);
    lines.push(close.to_string());
    lines.join("\n")
}

fn is_primitive(obj: &ObjectSchema) -> bool {
    matches!(
        obj.schema_type.as_ref().map(primary_type),
        Some(SchemaType::String | SchemaType::Integer | SchemaType::Number | SchemaType::Boolean)
    )
}

fn render_top_level_primitive(obj: &ObjectSchema) -> String {
    let example = primitive_example(obj);
    let comment = type_comment(obj, "");
    if comment.is_empty() {
        example
    } else {
        format!("{example}  // {comment}")
    }
}
