use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, ObjectSchema, Schema, SchemaType, SchemaTypeSet},
};

use super::camel_to_kebab;

pub fn collect_writes(spec: &OpenApiV3Spec, dir: &Path, writes: &mut Vec<(PathBuf, String)>) {
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
        writes.push((schema_dir.join(&filename), content));
        index_links.push((filename, name.clone()));
    }

    let index: String = index_links
        .iter()
        .map(|(file, name)| format!("- [{name}](./{file})"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    writes.push((schema_dir.join("index.md"), index));
}

fn render_schema_file(name: &str, schema: &Schema, spec: &OpenApiV3Spec) -> String {
    let mut out = format!("# {name}\n\n");

    let description = match schema.resolve(spec) {
        Ok(Schema::Object(oor)) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => obj.description.clone(),
            _ => None,
        },
        _ => None,
    };

    if let Some(desc) = description {
        out.push_str(&format!("{desc}\n\n"));
    }

    out.push_str("```jsonc\n");
    // Schema definition files always inline everything (empty multi_use set).
    out.push_str(&render_schema_jsonc(schema, spec, &HashSet::new()));
    out.push('\n');
    out.push_str("```\n");
    out
}

/// Render a schema as a jsonc block.
///
/// `multi_use` — set of component schema names (e.g. `"error"`, `"payment_method"`) that appear
/// in 2+ endpoint bodies. When a `$ref` to one of these is encountered it is replaced with a
/// markdown link rather than inlined, keeping endpoint files concise.
/// Pass an empty set to always inline (used for standalone schema files).
pub fn render_schema_jsonc(
    schema: &Schema,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    let resolved = match schema.resolve(spec) {
        Ok(s) => s,
        Err(_) => return "{}".to_string(),
    };

    match resolved {
        Schema::Boolean(b) => b.0.to_string(),
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => render_top_level_object(obj, spec, multi_use),
            ObjectOrReference::Ref { .. } => "{}".to_string(),
        },
    }
}

fn render_top_level_object(
    obj: &ObjectSchema,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> String {
    if obj
        .schema_type
        .as_ref()
        .map(|ts| ts.is_array_or_nullable_array())
        .unwrap_or(false)
    {
        let item_lines = obj
            .items
            .as_ref()
            .map(|items| array_item_lines(items, 1, spec, multi_use))
            .unwrap_or_else(|| vec!["  null".to_string()]);
        let mut lines = vec!["[".to_string()];
        lines.extend(item_lines);
        lines.push("]".to_string());
        return lines.join("\n");
    }

    if obj.properties.is_empty() {
        return "{}".to_string();
    }

    let props: Vec<_> = obj.properties.iter().collect();
    let n = props.len();
    let mut lines = vec!["{".to_string()];
    for (i, (name, schema)) in props.into_iter().enumerate() {
        let trail = if i + 1 == n { "" } else { "," };
        let is_req = obj.required.contains(name);
        lines.extend(property_lines(name, schema, is_req, trail, 1, spec, multi_use));
    }
    lines.push("}".to_string());
    lines.join("\n")
}

pub(crate) fn property_lines(
    name: &str,
    schema: &Schema,
    is_required: bool,
    trail: &str,
    depth: usize,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> Vec<String> {
    let indent = "  ".repeat(depth);
    let req = if is_required { "required" } else { "optional" };

    // Before resolving, check if this is a $ref to a multi-use named schema.
    // If so, emit a link instead of inlining the full definition.
    if let Some(ref_name) = schema_ref_name(schema) {
        if multi_use.contains(ref_name) {
            let slug = camel_to_kebab(ref_name);
            let link = format!("../../schemas/{slug}.md");
            return vec![format!(
                "{indent}\"{name}\": {{ /* [{ref_name}]({link}) */ }}{trail}  // object, {req}"
            )];
        }
    }

    let resolved = match schema.resolve(spec) {
        Ok(s) => s,
        Err(_) => {
            return vec![format!(
                "{indent}\"{name}\": null{trail}  // unknown, {req}"
            )];
        }
    };

    match resolved {
        Schema::Boolean(b) => {
            vec![format!(
                "{indent}\"{name}\": {}{trail}  // boolean, {req}",
                b.0
            )]
        }
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => {
                object_property_lines(name, obj, is_required, trail, depth, spec, multi_use)
            }
            ObjectOrReference::Ref { .. } => {
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
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> Vec<String> {
    let indent = "  ".repeat(depth);
    let req = if is_required { "required" } else { "optional" };

    let is_array = obj
        .schema_type
        .as_ref()
        .map(|ts| ts.is_array_or_nullable_array())
        .unwrap_or(false);

    if is_array {
        let item_type = array_item_type_label(obj, spec);
        let comment = format!("array of {item_type}, {req}");
        let close = "  ".repeat(depth);
        let item_lines = obj
            .items
            .as_ref()
            .map(|items| array_item_lines(items, depth + 1, spec, multi_use))
            .unwrap_or_else(|| vec![format!("{}null", "  ".repeat(depth + 1))]);

        let mut lines = vec![format!("{indent}\"{name}\": [  // {comment}")];
        lines.extend(item_lines);
        lines.push(format!("{close}]{trail}"));
        return lines;
    }

    if !obj.properties.is_empty() {
        let close = "  ".repeat(depth);
        let props: Vec<_> = obj.properties.iter().collect();
        let n = props.len();
        let mut lines = vec![format!("{indent}\"{name}\": {{")];
        for (i, (pname, pschema)) in props.into_iter().enumerate() {
            let ptrail = if i + 1 == n { "" } else { "," };
            let preq = obj.required.contains(pname);
            lines.extend(property_lines(
                pname,
                pschema,
                preq,
                ptrail,
                depth + 1,
                spec,
                multi_use,
            ));
        }
        lines.push(format!("{close}}}{trail}"));
        return lines;
    }

    // Primitive
    let example = primitive_example(obj);
    let comment = type_comment(obj, req);
    vec![format!(
        "{indent}\"{name}\": {example}{trail}  // {comment}"
    )]
}

fn array_item_lines(
    items: &Schema,
    depth: usize,
    spec: &OpenApiV3Spec,
    multi_use: &HashSet<String>,
) -> Vec<String> {
    let indent = "  ".repeat(depth);

    // Check for $ref to multi-use schema before resolving.
    if let Some(ref_name) = schema_ref_name(items) {
        if multi_use.contains(ref_name) {
            let slug = camel_to_kebab(ref_name);
            let link = format!("../../schemas/{slug}.md");
            return vec![format!(
                "{indent}{{ /* [{ref_name}]({link}) */ }}"
            )];
        }
    }

    let resolved = match items.resolve(spec) {
        Ok(s) => s,
        Err(_) => return vec![format!("{indent}null")],
    };

    match resolved {
        Schema::Boolean(b) => vec![format!("{indent}{}", b.0)],
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) if !obj.properties.is_empty() => {
                let close = "  ".repeat(depth);
                let props: Vec<_> = obj.properties.iter().collect();
                let n = props.len();
                let mut lines = vec![format!("{indent}{{")];
                for (i, (pname, pschema)) in props.into_iter().enumerate() {
                    let ptrail = if i + 1 == n { "" } else { "," };
                    let preq = obj.required.contains(pname);
                    lines.extend(property_lines(
                        pname,
                        pschema,
                        preq,
                        ptrail,
                        depth + 1,
                        spec,
                        multi_use,
                    ));
                }
                lines.push(format!("{close}}}"));
                lines
            }
            ObjectOrReference::Object(obj) => {
                vec![format!("{indent}{}", primitive_example(obj))]
            }
            ObjectOrReference::Ref { .. } => vec![format!("{indent}null")],
        },
    }
}

/// Extract the component schema name from a `$ref`, if present.
/// Returns `Some("payment_method")` for `$ref: "#/components/schemas/payment_method"`.
fn schema_ref_name(schema: &Schema) -> Option<&str> {
    match schema {
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Ref { ref_path, .. } => ref_path
                .strip_prefix("#/components/schemas/"),
            _ => None,
        },
        _ => None,
    }
}

fn array_item_type_label(array_obj: &ObjectSchema, _spec: &OpenApiV3Spec) -> String {
    let Some(items) = &array_obj.items else {
        return "any".to_string();
    };

    // Check for $ref before resolving to preserve the name
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
    let base = match ty {
        Some(SchemaType::Integer) => {
            if let Some(fmt) = &obj.format {
                format!("integer ({fmt})")
            } else {
                "integer".to_string()
            }
        }
        Some(SchemaType::Number) => "number".to_string(),
        Some(SchemaType::Boolean) => "boolean".to_string(),
        Some(SchemaType::String) => "string".to_string(),
        Some(SchemaType::Array) => "array".to_string(),
        Some(SchemaType::Object) => "object".to_string(),
        Some(SchemaType::Null) => "null".to_string(),
        None => "any".to_string(),
    };

    let mut parts = vec![base];

    // format — for non-integer types (integers already include format in `base`)
    if let Some(fmt) = &obj.format {
        if !matches!(ty, Some(SchemaType::Integer)) {
            parts.push(format!("format: {fmt}"));
        }
    }

    parts.push(req.to_string());

    // Numeric range constraints
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

    // String length / pattern constraints
    if let Some(min_len) = obj.min_length {
        parts.push(format!("minLength: {min_len}"));
    }
    if let Some(max_len) = obj.max_length {
        parts.push(format!("maxLength: {max_len}"));
    }
    if let Some(pat) = &obj.pattern {
        // Truncate very long patterns to stay readable
        if pat.len() > 60 {
            parts.push(format!("pattern: \"{}…\"", &pat[..57]));
        } else {
            parts.push(format!("pattern: \"{pat}\""));
        }
    }

    // Array size constraints
    if let Some(min_items) = obj.min_items {
        parts.push(format!("minItems: {min_items}"));
    }
    if let Some(max_items) = obj.max_items {
        parts.push(format!("maxItems: {max_items}"));
    }

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

fn primary_type(ts: &SchemaTypeSet) -> SchemaType {
    match ts {
        SchemaTypeSet::Single(t) => *t,
        SchemaTypeSet::Multiple(types) => types
            .iter()
            .copied()
            .find(|t| *t != SchemaType::Null)
            .unwrap_or(SchemaType::Object),
    }
}
