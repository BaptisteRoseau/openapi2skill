use std::collections::HashSet;

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, ObjectSchema, Schema, SchemaType},
};
use tracing::warn;

use super::{
    composition::merge_all_of,
    context::RenderCtx,
    properties::{array_item_lines, render_properties_lines},
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
    if merged
        .schema_type
        .as_ref()
        .map(|ts| ts.is_array_or_nullable_array())
        .unwrap_or(false)
    {
        return render_top_level_array(&merged, ctx);
    }
    if is_primitive(&merged) {
        return render_top_level_primitive(&merged);
    }
    if merged.properties.is_empty() {
        return "{\n  // empty object\n}".to_string();
    }
    let mut lines = vec!["{".to_string()];
    lines.extend(render_properties_lines(&merged, 1, ctx));
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
