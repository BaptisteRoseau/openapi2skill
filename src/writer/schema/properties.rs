use oas3::spec::{ObjectOrReference, ObjectSchema, Schema};
use tracing::warn;

use super::{
    composition::merge_all_of,
    context::RenderCtx,
    types::{primitive_example, primitive_type_name, type_comment},
};
use crate::writer::utils::{ref_display_name, schema_doc_link, schema_ref_name};

pub(super) fn render_properties_lines(
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

pub(super) fn property_lines(
    name: &str,
    schema: &Schema,
    is_required: bool,
    trail: &str,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
    if let Some(ref_name) = linked_ref_name(schema, ctx) {
        let indent = indent(depth);
        let req = requirement_label(is_required);
        return vec![format!(
            "{indent}\"{name}\": {{ /* [{ref_name}]({}) */ }}{trail}  // object, {req}",
            schema_doc_link(ref_name)
        )];
    }
    with_visiting(schema_ref_name(schema), ctx, |ctx| {
        resolved_property_lines(name, schema, is_required, trail, depth, ctx)
    })
}

pub(super) fn array_item_lines(
    items: &Schema,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
    if let Some(ref_name) = linked_ref_name(items, ctx) {
        return vec![format!(
            "{}{{ /* [{ref_name}]({}) */ }}",
            indent(depth),
            schema_doc_link(ref_name)
        )];
    }
    with_visiting(schema_ref_name(items), ctx, |ctx| {
        resolved_array_item_lines(items, depth, ctx)
    })
}

/// The referenced schema name when it should be linked rather than expanded inline —
/// either because it is shared by several endpoints or because it is already on the
/// recursion stack.
fn linked_ref_name<'a>(schema: &'a Schema, ctx: &RenderCtx<'_>) -> Option<&'a str> {
    let name = schema_ref_name(schema)?;
    (ctx.multi_use.contains(name) || ctx.visiting.contains(name)).then_some(name)
}

/// Runs `render` with `ref_name` on the recursion stack so nested cycles get linked instead
/// of expanded forever.
fn with_visiting<T>(
    ref_name: Option<&str>,
    ctx: &mut RenderCtx<'_>,
    render: impl FnOnce(&mut RenderCtx<'_>) -> T,
) -> T {
    let pushed = ref_name.map(str::to_string);
    if let Some(name) = &pushed {
        ctx.visiting.insert(name.clone());
    }
    let rendered = render(ctx);
    if let Some(name) = &pushed {
        ctx.visiting.remove(name);
    }
    rendered
}

fn indent(depth: usize) -> String {
    "  ".repeat(depth)
}

fn requirement_label(is_required: bool) -> &'static str {
    if is_required { "required" } else { "optional" }
}

fn resolved_property_lines(
    name: &str,
    schema: &Schema,
    is_required: bool,
    trail: &str,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
    let indent = indent(depth);
    let req = requirement_label(is_required);
    let resolved = match schema.resolve(ctx.spec) {
        Ok(s) => s,
        Err(err) => {
            warn!(
                property = name,
                "could not resolve schema for property: {err}; rendering as null"
            );
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
            ObjectOrReference::Ref { ref_path, .. } => {
                warn!(
                    property = name,
                    ref_path = %ref_path,
                    "property resolved to an unresolved $ref; rendering as null"
                );
                vec![format!(
                    "{indent}\"{name}\": null{trail}  // unresolved ref, {req}"
                )]
            }
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
    let indent = indent(depth);
    let req = requirement_label(is_required);
    let merged = merge_all_of(obj, ctx);

    if is_array(&merged) {
        let item_type = array_item_type_label(&merged);
        let mut lines = vec![format!(
            "{indent}\"{name}\": [  // array of {item_type}, {req}"
        )];
        lines.extend(item_lines_or_null(&merged, depth + 1, ctx));
        lines.push(format!("{indent}]{trail}"));
        return lines;
    }

    if !merged.properties.is_empty() {
        let mut lines = vec![format!("{indent}\"{name}\": {{")];
        lines.extend(render_properties_lines(&merged, depth + 1, ctx));
        lines.push(format!("{indent}}}{trail}"));
        return lines;
    }

    let example = primitive_example(&merged);
    let comment = type_comment(&merged, req);
    vec![format!(
        "{indent}\"{name}\": {example}{trail}  // {comment}"
    )]
}

pub(super) fn is_array(obj: &ObjectSchema) -> bool {
    obj.schema_type
        .as_ref()
        .map(|ts| ts.is_array_or_nullable_array())
        .unwrap_or(false)
}

/// Renders an array's item example, falling back to `null` for arrays with no `items`.
pub(super) fn item_lines_or_null(
    array_obj: &ObjectSchema,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
    match &array_obj.items {
        Some(items) => array_item_lines(items, depth, ctx),
        None => vec![format!("{}null", indent(depth))],
    }
}

fn resolved_array_item_lines(items: &Schema, depth: usize, ctx: &mut RenderCtx<'_>) -> Vec<String> {
    let indent = indent(depth);
    let resolved = match items.resolve(ctx.spec) {
        Ok(s) => s,
        Err(err) => {
            warn!("could not resolve array item schema: {err}; rendering as null");
            return vec![format!("{indent}null")];
        }
    };

    match resolved {
        Schema::Boolean(b) => vec![format!("{indent}{}", b.0)],
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => {
                let merged = merge_all_of(obj, ctx);
                if merged.properties.is_empty() {
                    return vec![format!("{indent}{}", primitive_example(&merged))];
                }
                let mut lines = vec![format!("{indent}{{")];
                lines.extend(render_properties_lines(&merged, depth + 1, ctx));
                lines.push(format!("{indent}}}"));
                lines
            }
            ObjectOrReference::Ref { ref_path, .. } => {
                warn!(
                    ref_path = %ref_path,
                    "array item resolved to an unresolved $ref; rendering as null"
                );
                vec![format!("{indent}null")]
            }
        },
    }
}

fn array_item_type_label(array_obj: &ObjectSchema) -> String {
    let Some(items) = &array_obj.items else {
        return "any".to_string();
    };
    match items.as_ref() {
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Ref { ref_path, .. } => ref_display_name(ref_path).to_string(),
            ObjectOrReference::Object(obj) => primitive_type_name(obj),
        },
        Schema::Boolean(_) => "boolean".to_string(),
    }
}
