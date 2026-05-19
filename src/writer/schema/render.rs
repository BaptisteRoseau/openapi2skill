use std::collections::HashSet;

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, ObjectSchema, Schema},
};

use super::{
    context::RenderCtx,
    properties::{array_item_lines, render_properties_lines},
};

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

pub(super) fn render_top_level_object(obj: &ObjectSchema, ctx: &mut RenderCtx<'_>) -> String {
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
