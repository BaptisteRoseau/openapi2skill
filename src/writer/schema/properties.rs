use oas3::spec::{ObjectOrReference, ObjectSchema, Schema};
use tracing::warn;

use super::{
    composition::merge_all_of,
    context::RenderCtx,
    types::{primitive_example, primitive_type_name, type_comment},
};
use crate::writer::utils::{ref_display_name, ref_path_of, schema_doc_link, schema_ref_name};

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

/// Placeholder body for a `$ref` the spec never defines, keeping the referenced name
/// visible instead of collapsing to `null`.
fn undefined_ref_placeholder(ref_path: &str) -> String {
    format!(
        "{{ /* {} (not defined in this spec) */ }}",
        ref_display_name(ref_path)
    )
}

fn unresolved_property_line(
    name: &str,
    schema: &Schema,
    trail: &str,
    depth: usize,
    req: &str,
) -> String {
    let indent = indent(depth);
    match ref_path_of(schema) {
        Some(ref_path) => format!(
            "{indent}\"{name}\": {}{trail}  // unresolved ref, {req}",
            undefined_ref_placeholder(ref_path)
        ),
        None => format!("{indent}\"{name}\": null{trail}  // unknown, {req}"),
    }
}

fn unresolved_item_line(items: &Schema, depth: usize) -> String {
    let indent = indent(depth);
    match ref_path_of(items) {
        Some(ref_path) => format!("{indent}{}", undefined_ref_placeholder(ref_path)),
        None => format!("{indent}null"),
    }
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
                "could not resolve schema for property: {err}; rendering a placeholder"
            );
            return vec![unresolved_property_line(name, schema, trail, depth, req)];
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
                    "property resolved to an unresolved $ref; rendering a placeholder"
                );
                vec![format!(
                    "{indent}\"{name}\": {}{trail}  // unresolved ref, {req}",
                    undefined_ref_placeholder(ref_path)
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
            warn!("could not resolve array item schema: {err}; rendering a placeholder");
            return vec![unresolved_item_line(items, depth)];
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
                    "array item resolved to an unresolved $ref; rendering a placeholder"
                );
                vec![format!("{indent}{}", undefined_ref_placeholder(ref_path))]
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::{Value, json};

    use crate::writer::schema::render::render_schema_jsonc;
    use crate::writer::testutil::spec_with_schemas;

    /// Renders a `Thing` schema with `properties`, against a spec where `Thing` is the only
    /// defined component schema.
    fn render_thing(properties: Value) -> String {
        let spec = spec_with_schemas(json!({
            "Thing": {"type": "object", "properties": properties},
        }));
        let schema = serde_json::from_value(json!({"$ref": "#/components/schemas/Thing"}))
            .expect("valid ref schema");
        render_schema_jsonc(&schema, &spec, &HashSet::new())
    }

    #[test]
    fn undefined_array_item_ref_renders_named_placeholder() {
        let out = render_thing(json!({
            "allocations": {
                "type": "array",
                "items": {"$ref": "#/components/schemas/AppFeeAllocation"},
            },
        }));
        assert!(
            out.contains("\"allocations\": [  // array of AppFeeAllocation, optional"),
            "missing array type label in:\n{out}"
        );
        assert!(
            out.contains("{ /* AppFeeAllocation (not defined in this spec) */ }"),
            "missing named placeholder in:\n{out}"
        );
        assert!(
            !out.contains("null"),
            "item should not render as null:\n{out}"
        );
    }

    #[test]
    fn undefined_property_ref_renders_named_placeholder() {
        let out = render_thing(json!({
            "buyer_currency_exchange": {"$ref": "#/components/schemas/CurrencyExchange"},
        }));
        assert!(
            out.contains(
                "\"buyer_currency_exchange\": { /* CurrencyExchange (not defined in this spec) */ }  // unresolved ref, optional"
            ),
            "missing named placeholder in:\n{out}"
        );
    }

    #[test]
    fn undefined_external_ref_keeps_full_path() {
        let out = render_thing(json!({
            "phone": {"$ref": "https://example.com/common.json#Phone"},
        }));
        assert!(
            out.contains(
                "{ /* https://example.com/common.json#Phone (not defined in this spec) */ }"
            ),
            "external ref should keep its full path in:\n{out}"
        );
    }

    #[test]
    fn array_without_items_still_renders_null() {
        let out = render_thing(json!({"allocations": {"type": "array"}}));
        assert!(
            out.contains("\"allocations\": [  // array of any, optional"),
            "missing array line in:\n{out}"
        );
        assert!(out.contains("null"), "expected null item in:\n{out}");
    }

    #[test]
    fn defined_ref_still_expands_inline() {
        let spec = spec_with_schemas(json!({
            "Thing": {
                "type": "object",
                "properties": {"tag": {"$ref": "#/components/schemas/Tag"}},
            },
            "Tag": {"type": "object", "properties": {"name": {"type": "string"}}},
        }));
        let schema = serde_json::from_value(json!({"$ref": "#/components/schemas/Thing"}))
            .expect("valid ref schema");
        let out = render_schema_jsonc(&schema, &spec, &HashSet::new());
        assert!(
            out.contains("\"name\": \"string\""),
            "resolvable ref should expand inline in:\n{out}"
        );
        assert!(
            !out.contains("not defined in this spec"),
            "resolvable ref should not use the placeholder in:\n{out}"
        );
    }
}
