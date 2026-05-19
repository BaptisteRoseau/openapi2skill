use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, ObjectSchema, Schema},
};

use super::{
    context::RenderCtx,
    types::{primitive_example, primitive_type_name, type_comment},
};
use crate::writer::utils::camel_to_kebab;

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

pub(super) fn array_item_lines(
    items: &Schema,
    depth: usize,
    ctx: &mut RenderCtx<'_>,
) -> Vec<String> {
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
