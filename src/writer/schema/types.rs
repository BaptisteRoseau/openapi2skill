use oas3::spec::{ObjectSchema, SchemaType, SchemaTypeSet};

use crate::writer::utils::primary_type;

pub(super) fn primitive_type_name(obj: &ObjectSchema) -> String {
    match obj.schema_type.as_ref().map(primary_type) {
        Some(SchemaType::String) => "string".to_string(),
        Some(SchemaType::Integer) => "integer".to_string(),
        Some(SchemaType::Number) => "number".to_string(),
        Some(SchemaType::Boolean) => "boolean".to_string(),
        _ => "object".to_string(),
    }
}

pub(super) fn primitive_example(obj: &ObjectSchema) -> String {
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

pub(super) fn type_comment(obj: &ObjectSchema, req: &str) -> String {
    let ts = obj.schema_type.as_ref();
    let fmt = obj.format.as_deref();
    let mut parts = vec![type_label(ts, fmt)];
    let is_single_integer = matches!(ts, Some(SchemaTypeSet::Single(SchemaType::Integer)));
    if let Some(f) = fmt
        && !is_single_integer
    {
        parts.push(format!("format: {f}"));
    }
    if !req.is_empty() {
        parts.push(req.to_string());
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
    // Always put description at the end of the line
    if let Some(desc) = &obj.description {
        let trimmed = desc.trim();
        if !trimmed.is_empty() {
            parts.push(trimmed.to_string());
        }
    }
    parts.join(", ")
}

fn type_label(ts: Option<&SchemaTypeSet>, fmt: Option<&str>) -> String {
    match ts {
        None => "any".to_string(),
        Some(SchemaTypeSet::Single(t)) => single_type_label(*t, fmt),
        Some(SchemaTypeSet::Multiple(types)) => {
            let inner: Vec<String> = types.iter().copied().map(bare_type_name).collect();
            format!("array[{}]", inner.join(", "))
        }
    }
}

fn single_type_label(t: SchemaType, fmt: Option<&str>) -> String {
    match t {
        SchemaType::Integer => fmt
            .map(|f| format!("integer ({f})"))
            .unwrap_or_else(|| "integer".to_string()),
        _ => bare_type_name(t),
    }
}

fn bare_type_name(t: SchemaType) -> String {
    match t {
        SchemaType::Integer => "integer".to_string(),
        SchemaType::Number => "number".to_string(),
        SchemaType::Boolean => "boolean".to_string(),
        SchemaType::String => "string".to_string(),
        SchemaType::Array => "array".to_string(),
        SchemaType::Object => "object".to_string(),
        SchemaType::Null => "null".to_string(),
    }
}

pub(super) fn collect_type_constraints(obj: &ObjectSchema) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn obj(v: serde_json::Value) -> ObjectSchema {
        serde_json::from_value(v).unwrap()
    }

    // --- collect_type_constraints ---

    #[test]
    fn constraints_empty_by_default() {
        assert!(collect_type_constraints(&obj(json!({}))).is_empty());
    }

    #[test]
    fn constraints_minimum() {
        assert_eq!(
            collect_type_constraints(&obj(json!({"minimum": 3}))),
            vec!["min: 3"]
        );
    }

    #[test]
    fn constraints_maximum() {
        assert_eq!(
            collect_type_constraints(&obj(json!({"maximum": 99}))),
            vec!["max: 99"]
        );
    }

    #[test]
    fn constraints_exclusive_bounds() {
        let cs = collect_type_constraints(&obj(
            json!({"exclusiveMinimum": 0, "exclusiveMaximum": 100}),
        ));
        assert_eq!(cs, vec!["xmin: 0", "xmax: 100"]);
    }

    #[test]
    fn constraints_string_lengths() {
        let cs = collect_type_constraints(&obj(json!({"minLength": 2, "maxLength": 64})));
        assert_eq!(cs, vec!["minLength: 2", "maxLength: 64"]);
    }

    #[test]
    fn constraints_pattern() {
        let cs = collect_type_constraints(&obj(json!({"pattern": "^[a-z]+$"})));
        assert_eq!(cs, vec!["pattern: \"^[a-z]+$\""]);
    }

    #[test]
    fn constraints_array_items() {
        let cs = collect_type_constraints(&obj(json!({"minItems": 1, "maxItems": 10})));
        assert_eq!(cs, vec!["minItems: 1", "maxItems: 10"]);
    }

    // --- primitive_example ---

    #[test]
    fn example_uses_example_field_first() {
        let o = obj(json!({"type": "integer", "example": 42, "enum": [1], "default": 0}));
        assert_eq!(primitive_example(&o), "42");
    }

    #[test]
    fn example_skips_object_example_falls_back_to_enum() {
        let o = obj(json!({"type": "string", "example": {"key": "val"}, "enum": ["foo"]}));
        assert_eq!(primitive_example(&o), "\"foo\"");
    }

    #[test]
    fn example_uses_enum_when_no_example() {
        let o = obj(json!({"type": "string", "enum": ["available", "sold"]}));
        assert_eq!(primitive_example(&o), "\"available\"");
    }

    #[test]
    fn example_uses_default_when_no_example_or_enum() {
        let o = obj(json!({"type": "boolean", "default": true}));
        assert_eq!(primitive_example(&o), "true");
    }

    #[test]
    fn example_type_based_fallback_integer() {
        let o = obj(json!({"type": "integer"}));
        assert_eq!(primitive_example(&o), "0");
    }

    #[test]
    fn example_type_based_fallback_number() {
        let o = obj(json!({"type": "number"}));
        assert_eq!(primitive_example(&o), "0.0");
    }

    #[test]
    fn example_type_based_fallback_boolean() {
        let o = obj(json!({"type": "boolean"}));
        assert_eq!(primitive_example(&o), "false");
    }

    #[test]
    fn example_type_based_fallback_string() {
        let o = obj(json!({"type": "string"}));
        assert_eq!(primitive_example(&o), "\"string\"");
    }

    #[test]
    fn example_type_based_fallback_null_for_unknown() {
        let o = obj(json!({}));
        assert_eq!(primitive_example(&o), "null");
    }

    // --- type_comment ---

    #[test]
    fn type_comment_simple_string_optional() {
        let o = obj(json!({"type": "string"}));
        assert_eq!(type_comment(&o, "optional"), "string, optional");
    }

    #[test]
    fn type_comment_integer_with_format() {
        let o = obj(json!({"type": "integer", "format": "int64"}));
        assert_eq!(type_comment(&o, "required"), "integer (int64), required");
    }

    #[test]
    fn type_comment_string_with_format_includes_format_label() {
        let o = obj(json!({"type": "string", "format": "date-time"}));
        assert_eq!(
            type_comment(&o, "optional"),
            "string, format: date-time, optional"
        );
    }

    #[test]
    fn type_comment_includes_description() {
        let o = obj(json!({"type": "string", "description": "The pet name"}));
        assert_eq!(
            type_comment(&o, "required"),
            "string, required, The pet name"
        );
    }

    #[test]
    fn type_comment_includes_enum_values() {
        let o = obj(json!({"type": "string", "enum": ["a", "b"]}));
        assert!(type_comment(&o, "optional").contains("enum: \"a\", \"b\""));
    }

    #[test]
    fn type_comment_includes_constraints() {
        let o = obj(json!({"type": "integer", "minimum": 1, "maximum": 10}));
        let c = type_comment(&o, "optional");
        assert!(c.contains("min: 1"));
        assert!(c.contains("max: 10"));
    }

    // --- multi-type (OpenAPI 3.1 type arrays) rendering ---

    #[test]
    fn type_comment_nullable_string_renders_as_array() {
        let o = obj(json!({"type": ["string", "null"]}));
        assert_eq!(
            type_comment(&o, "optional"),
            "array[string, null], optional"
        );
    }

    #[test]
    fn type_comment_nullable_object_renders_as_array() {
        let o = obj(json!({"type": ["object", "null"]}));
        assert_eq!(
            type_comment(&o, "required"),
            "array[object, null], required"
        );
    }

    #[test]
    fn type_comment_preserves_array_order() {
        let o = obj(json!({"type": ["null", "integer"]}));
        assert_eq!(
            type_comment(&o, "optional"),
            "array[null, integer], optional"
        );
    }

    #[test]
    fn type_comment_multi_type_with_format() {
        let o = obj(json!({"type": ["string", "null"], "format": "date-time"}));
        // Format is emitted as a separate part for multi-types — never inlined.
        assert_eq!(
            type_comment(&o, "optional"),
            "array[string, null], format: date-time, optional"
        );
    }

    #[test]
    fn type_comment_multi_type_with_description_and_enum() {
        let o = obj(json!({
            "type": ["string", "null"],
            "enum": ["a", "b"],
            "description": "Nullable enum",
        }));
        let c = type_comment(&o, "optional");
        assert!(c.starts_with("array[string, null], optional"));
        assert!(c.contains("enum: \"a\", \"b\""));
        assert!(c.ends_with("Nullable enum"));
    }
}
