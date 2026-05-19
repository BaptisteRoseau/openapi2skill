use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, ObjectSchema, Parameter, Schema, SchemaType},
};

use crate::writer::utils::primary_type;

pub(super) fn render_path_params_table(params: &[&Parameter], spec: &OpenApiV3Spec) -> String {
    if params.is_empty() {
        return String::new();
    }
    let mut out = "### Path Parameters\n\n| Parameter | Type | Required | Description |\n|-----------|------|----------|-------------|\n".to_string();
    for p in params {
        let req = if p.required.unwrap_or(true) {
            "Yes"
        } else {
            "No"
        };
        out.push_str(&format!(
            "| `{}` | {} | {req} | {} |\n",
            p.name,
            render_param_type(&p.schema, spec),
            p.description.as_deref().unwrap_or("-"),
        ));
    }
    out.push('\n');
    out
}

pub(super) fn render_query_params_table(params: &[&Parameter], spec: &OpenApiV3Spec) -> String {
    if params.is_empty() {
        return String::new();
    }
    let mut out = "### Query Parameters\n\n| Parameter | Type | Required | Description |\n|-----------|------|----------|-------------|\n".to_string();
    for p in params {
        let req = if p.required.unwrap_or(false) {
            "Yes"
        } else {
            "No"
        };
        out.push_str(&format!(
            "| `{}` | {} | {req} | {} |\n",
            p.name,
            render_param_type(&p.schema, spec),
            p.description.as_deref().unwrap_or("-"),
        ));
    }
    out.push('\n');
    out
}

fn render_param_type(schema: &Option<Schema>, spec: &OpenApiV3Spec) -> String {
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
            ObjectOrReference::Object(obj) => render_param_object_type(obj),
            ObjectOrReference::Ref { ref_path, .. } => ref_path
                .strip_prefix("#/components/schemas/")
                .unwrap_or(ref_path)
                .to_string(),
        },
    }
}

fn render_param_object_type(obj: &ObjectSchema) -> String {
    let ty = obj.schema_type.as_ref().map(primary_type);
    let mut base = param_base_type(ty, obj.format.as_deref());
    let constraints = param_constraints(obj);
    if !constraints.is_empty() {
        base = format!("{base} ({})", constraints.join(", "));
    }
    if obj.enum_values.is_empty() {
        return base;
    }
    let vals = obj
        .enum_values
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| format!("`{s}`"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{base} ({vals})")
}

fn param_base_type(ty: Option<SchemaType>, fmt: Option<&str>) -> String {
    match ty {
        None => "any".to_string(),
        Some(SchemaType::Integer) => fmt
            .map(|f| format!("integer ({f})"))
            .unwrap_or_else(|| "integer".to_string()),
        Some(SchemaType::Number) => fmt
            .map(|f| format!("number ({f})"))
            .unwrap_or_else(|| "number".to_string()),
        Some(SchemaType::String) => fmt
            .map(|f| format!("string ({f})"))
            .unwrap_or_else(|| "string".to_string()),
        Some(SchemaType::Boolean) => "boolean".to_string(),
        Some(SchemaType::Array) => "array".to_string(),
        Some(SchemaType::Object) => "object".to_string(),
        Some(SchemaType::Null) => "null".to_string(),
    }
}

fn param_constraints(obj: &ObjectSchema) -> Vec<String> {
    let mut cs = Vec::new();
    match (obj.minimum.as_ref(), obj.maximum.as_ref()) {
        (Some(lo), Some(hi)) => cs.push(format!("{lo}..{hi}")),
        (Some(lo), None) => cs.push(format!("≥{lo}")),
        (None, Some(hi)) => cs.push(format!("≤{hi}")),
        _ => {}
    }
    if let Some(max_len) = obj.max_length {
        cs.push(format!("max_len:{max_len}"));
    }
    cs
}

#[cfg(test)]
mod tests {
    use super::*;
    use oas3::spec::SchemaType;

    #[test]
    fn param_base_type_none_returns_any() {
        assert_eq!(param_base_type(None, None), "any");
    }

    #[test]
    fn param_base_type_integer_no_format() {
        assert_eq!(param_base_type(Some(SchemaType::Integer), None), "integer");
    }

    #[test]
    fn param_base_type_integer_with_format() {
        assert_eq!(
            param_base_type(Some(SchemaType::Integer), Some("int64")),
            "integer (int64)"
        );
    }

    #[test]
    fn param_base_type_string_no_format() {
        assert_eq!(param_base_type(Some(SchemaType::String), None), "string");
    }

    #[test]
    fn param_base_type_string_with_format() {
        assert_eq!(
            param_base_type(Some(SchemaType::String), Some("date-time")),
            "string (date-time)"
        );
    }

    #[test]
    fn param_base_type_number_with_format() {
        assert_eq!(
            param_base_type(Some(SchemaType::Number), Some("float")),
            "number (float)"
        );
    }

    #[test]
    fn param_base_type_boolean() {
        assert_eq!(param_base_type(Some(SchemaType::Boolean), None), "boolean");
    }

    #[test]
    fn param_base_type_array() {
        assert_eq!(param_base_type(Some(SchemaType::Array), None), "array");
    }

    #[test]
    fn param_base_type_object() {
        assert_eq!(param_base_type(Some(SchemaType::Object), None), "object");
    }

    #[test]
    fn param_base_type_null() {
        assert_eq!(param_base_type(Some(SchemaType::Null), None), "null");
    }

    #[test]
    fn param_constraints_empty_when_no_bounds() {
        let obj: ObjectSchema = serde_json::from_value(serde_json::json!({})).unwrap();
        assert!(param_constraints(&obj).is_empty());
    }

    #[test]
    fn param_constraints_both_bounds() {
        let obj: ObjectSchema =
            serde_json::from_value(serde_json::json!({"minimum": 1, "maximum": 10})).unwrap();
        assert_eq!(param_constraints(&obj), vec!["1..10"]);
    }

    #[test]
    fn param_constraints_lower_only() {
        let obj: ObjectSchema = serde_json::from_value(serde_json::json!({"minimum": 5})).unwrap();
        assert_eq!(param_constraints(&obj), vec!["≥5"]);
    }

    #[test]
    fn param_constraints_upper_only() {
        let obj: ObjectSchema =
            serde_json::from_value(serde_json::json!({"maximum": 100})).unwrap();
        assert_eq!(param_constraints(&obj), vec!["≤100"]);
    }

    #[test]
    fn param_constraints_max_length() {
        let obj: ObjectSchema =
            serde_json::from_value(serde_json::json!({"maxLength": 255})).unwrap();
        assert_eq!(param_constraints(&obj), vec!["max_len:255"]);
    }

    #[test]
    fn param_constraints_bounds_and_max_length() {
        let obj: ObjectSchema = serde_json::from_value(
            serde_json::json!({"minimum": 0, "maximum": 50, "maxLength": 8}),
        )
        .unwrap();
        assert_eq!(param_constraints(&obj), vec!["0..50", "max_len:8"]);
    }
}
