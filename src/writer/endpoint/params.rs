use oas3::{
    OpenApiV3Spec,
    spec::{
        MediaType, ObjectOrReference, ObjectSchema, Parameter, Schema, SchemaType, SchemaTypeSet,
    },
};
use tracing::warn;

/// Returns `?key=val&key=val` for all required query parameters, or `""` if none.
pub(super) fn required_query_string(params: &[&Parameter], spec: &OpenApiV3Spec) -> String {
    let pairs: Vec<String> = params
        .iter()
        .filter(|p| p.required.unwrap_or(false))
        .map(|p| format!("{}={}", p.name, query_param_example(p, spec)))
        .collect();
    if pairs.is_empty() {
        return String::new();
    }
    format!("?{}", pairs.join("&"))
}

fn query_param_example(p: &Parameter, spec: &OpenApiV3Spec) -> String {
    query_example_from_schema(effective_schema(p), spec)
}

fn query_example_from_schema(schema: Option<&Schema>, spec: &OpenApiV3Spec) -> String {
    let schema = match schema {
        None => return "string".to_string(),
        Some(s) => s,
    };
    let resolved = match schema.resolve(spec) {
        Ok(r) => r,
        Err(_) => return "string".to_string(),
    };
    match resolved {
        Schema::Boolean(_) => "false".to_string(),
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => query_example_from_object(obj),
            ObjectOrReference::Ref { .. } => "string".to_string(),
        },
    }
}

fn query_example_from_object(obj: &ObjectSchema) -> String {
    if let Some(ex) = &obj.example {
        if let Some(s) = ex.as_str() {
            return s.to_string();
        }
        if let Some(n) = ex.as_i64() {
            return n.to_string();
        }
        if let Some(b) = ex.as_bool() {
            return b.to_string();
        }
    }
    if let Some(first) = obj.enum_values.first() {
        if let Some(s) = first.as_str() {
            return s.to_string();
        }
        if let Some(n) = first.as_i64() {
            return n.to_string();
        }
    }
    type_based_query_example(obj.schema_type.as_ref())
}

fn type_based_query_example(ts: Option<&SchemaTypeSet>) -> String {
    match ts {
        None => "string".to_string(),
        Some(SchemaTypeSet::Single(t)) => single_type_query_example(*t),
        Some(SchemaTypeSet::Multiple(types)) => types
            .first()
            .map(|t| single_type_query_example(*t))
            .unwrap_or_else(|| "string".to_string()),
    }
}

fn single_type_query_example(t: SchemaType) -> String {
    match t {
        SchemaType::Integer | SchemaType::Number => "0".to_string(),
        SchemaType::Boolean => "false".to_string(),
        _ => "string".to_string(),
    }
}

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
            render_param_type(effective_schema(p), spec),
            p.description
                .as_deref()
                .map(crate::writer::utils::normalize_desc)
                .as_deref()
                .unwrap_or("-"),
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
            render_param_type(effective_schema(p), spec),
            p.description
                .as_deref()
                .map(crate::writer::utils::normalize_desc)
                .as_deref()
                .unwrap_or("-"),
        ));
    }
    out.push('\n');
    out
}

fn effective_schema(p: &Parameter) -> Option<&Schema> {
    p.schema.as_ref().or_else(|| {
        p.content
            .as_ref()?
            .values()
            .next()
            .and_then(|mt: &MediaType| mt.schema.as_ref())
    })
}

pub(super) fn render_param_type(schema: Option<&Schema>, spec: &OpenApiV3Spec) -> String {
    let schema = match schema {
        None => return "string".to_string(),
        Some(s) => s,
    };
    let resolved = match schema.resolve(spec) {
        Ok(r) => r,
        Err(err) => {
            warn!("could not resolve parameter schema: {err}; rendering type as \"unknown\"");
            return "unknown".to_string();
        }
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
    let mut base = param_base_type(obj.schema_type.as_ref(), obj.format.as_deref());
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

fn param_base_type(ts: Option<&SchemaTypeSet>, fmt: Option<&str>) -> String {
    match ts {
        None => "any".to_string(),
        Some(SchemaTypeSet::Single(t)) => single_param_type(*t, fmt),
        Some(SchemaTypeSet::Multiple(types)) => {
            let inner: Vec<String> = types.iter().copied().map(bare_param_type).collect();
            format!("array[{}]", inner.join(", "))
        }
    }
}

fn single_param_type(t: SchemaType, fmt: Option<&str>) -> String {
    match t {
        SchemaType::Integer => fmt
            .map(|f| format!("integer ({f})"))
            .unwrap_or_else(|| "integer".to_string()),
        SchemaType::Number => fmt
            .map(|f| format!("number ({f})"))
            .unwrap_or_else(|| "number".to_string()),
        SchemaType::String => fmt
            .map(|f| format!("string ({f})"))
            .unwrap_or_else(|| "string".to_string()),
        _ => bare_param_type(t),
    }
}

fn bare_param_type(t: SchemaType) -> String {
    match t {
        SchemaType::Integer => "integer".to_string(),
        SchemaType::Number => "number".to_string(),
        SchemaType::String => "string".to_string(),
        SchemaType::Boolean => "boolean".to_string(),
        SchemaType::Array => "array".to_string(),
        SchemaType::Object => "object".to_string(),
        SchemaType::Null => "null".to_string(),
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
    use oas3::spec::{SchemaType, SchemaTypeSet};

    fn single(t: SchemaType) -> SchemaTypeSet {
        SchemaTypeSet::Single(t)
    }

    fn multi(types: Vec<SchemaType>) -> SchemaTypeSet {
        SchemaTypeSet::Multiple(types)
    }

    #[test]
    fn param_base_type_none_returns_any() {
        assert_eq!(param_base_type(None, None), "any");
    }

    #[test]
    fn param_base_type_integer_no_format() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::Integer)), None),
            "integer"
        );
    }

    #[test]
    fn param_base_type_integer_with_format() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::Integer)), Some("int64")),
            "integer (int64)"
        );
    }

    #[test]
    fn param_base_type_string_no_format() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::String)), None),
            "string"
        );
    }

    #[test]
    fn param_base_type_string_with_format() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::String)), Some("date-time")),
            "string (date-time)"
        );
    }

    #[test]
    fn param_base_type_number_with_format() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::Number)), Some("float")),
            "number (float)"
        );
    }

    #[test]
    fn param_base_type_boolean() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::Boolean)), None),
            "boolean"
        );
    }

    #[test]
    fn param_base_type_array() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::Array)), None),
            "array"
        );
    }

    #[test]
    fn param_base_type_object() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::Object)), None),
            "object"
        );
    }

    #[test]
    fn param_base_type_null() {
        assert_eq!(
            param_base_type(Some(&single(SchemaType::Null)), None),
            "null"
        );
    }

    #[test]
    fn param_base_type_multi_string_null() {
        assert_eq!(
            param_base_type(
                Some(&multi(vec![SchemaType::String, SchemaType::Null])),
                None
            ),
            "array[string, null]"
        );
    }

    #[test]
    fn param_base_type_multi_object_null() {
        assert_eq!(
            param_base_type(
                Some(&multi(vec![SchemaType::Object, SchemaType::Null])),
                None
            ),
            "array[object, null]"
        );
    }

    #[test]
    fn param_base_type_multi_preserves_order() {
        assert_eq!(
            param_base_type(
                Some(&multi(vec![SchemaType::Null, SchemaType::Integer])),
                None
            ),
            "array[null, integer]"
        );
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

    fn empty_spec() -> OpenApiV3Spec {
        oas3::from_json(r#"{"openapi":"3.1.0","info":{"title":"Test","version":"1.0"},"paths":{}}"#)
            .unwrap()
    }

    fn make_param(value: serde_json::Value) -> Parameter {
        let mut base = serde_json::json!({"name": "p", "in": "query"});
        if let (Some(obj), Some(extra)) = (base.as_object_mut(), value.as_object()) {
            for (k, v) in extra {
                obj.insert(k.clone(), v.clone());
            }
        }
        serde_json::from_value(base).unwrap()
    }

    #[test]
    fn effective_schema_prefers_schema_over_content() {
        let p = make_param(serde_json::json!({
            "schema": {"type": "integer"},
            "content": {"application/json": {"schema": {"type": "object"}}}
        }));
        let spec = empty_spec();
        assert_eq!(render_param_type(effective_schema(&p), &spec), "integer");
    }

    #[test]
    fn effective_schema_falls_back_to_content_schema() {
        let p = make_param(serde_json::json!({
            "content": {"application/json": {"schema": {"type": "object"}}}
        }));
        let spec = empty_spec();
        assert_eq!(render_param_type(effective_schema(&p), &spec), "object");
    }

    #[test]
    fn effective_schema_content_with_no_schema_returns_string() {
        let p = make_param(serde_json::json!({
            "content": {"application/json": {}}
        }));
        let spec = empty_spec();
        assert_eq!(render_param_type(effective_schema(&p), &spec), "string");
    }

    #[test]
    fn effective_schema_neither_schema_nor_content_returns_string() {
        let p = make_param(serde_json::json!({}));
        let spec = empty_spec();
        assert_eq!(render_param_type(effective_schema(&p), &spec), "string");
    }

    fn required_query_param(name: &str, schema: serde_json::Value) -> Parameter {
        let value =
            serde_json::json!({"name": name, "in": "query", "required": true, "schema": schema});
        serde_json::from_value(value).unwrap()
    }

    fn optional_query_param(name: &str, schema: serde_json::Value) -> Parameter {
        let value =
            serde_json::json!({"name": name, "in": "query", "required": false, "schema": schema});
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn required_query_string_empty_when_no_params() {
        let spec = empty_spec();
        assert_eq!(required_query_string(&[], &spec), "");
    }

    #[test]
    fn required_query_string_skips_optional_params() {
        let spec = empty_spec();
        let p = optional_query_param("page", serde_json::json!({"type": "integer"}));
        assert_eq!(required_query_string(&[&p], &spec), "");
    }

    #[test]
    fn required_query_string_single_string_param() {
        let spec = empty_spec();
        let p = required_query_param("from", serde_json::json!({"type": "string"}));
        assert_eq!(required_query_string(&[&p], &spec), "?from=string");
    }

    #[test]
    fn required_query_string_single_integer_param() {
        let spec = empty_spec();
        let p = required_query_param("limit", serde_json::json!({"type": "integer"}));
        assert_eq!(required_query_string(&[&p], &spec), "?limit=0");
    }

    #[test]
    fn required_query_string_single_boolean_param() {
        let spec = empty_spec();
        let p = required_query_param("active", serde_json::json!({"type": "boolean"}));
        assert_eq!(required_query_string(&[&p], &spec), "?active=false");
    }

    #[test]
    fn required_query_string_multiple_required_params() {
        let spec = empty_spec();
        let from = required_query_param("from", serde_json::json!({"type": "string"}));
        let to = required_query_param("to", serde_json::json!({"type": "string"}));
        assert_eq!(
            required_query_string(&[&from, &to], &spec),
            "?from=string&to=string"
        );
    }

    #[test]
    fn required_query_string_mixed_required_and_optional() {
        let spec = empty_spec();
        let from = required_query_param("from", serde_json::json!({"type": "string"}));
        let page = optional_query_param("page", serde_json::json!({"type": "integer"}));
        let to = required_query_param("to", serde_json::json!({"type": "string"}));
        assert_eq!(
            required_query_string(&[&from, &page, &to], &spec),
            "?from=string&to=string"
        );
    }

    #[test]
    fn required_query_string_uses_example_value() {
        let spec = empty_spec();
        let p = required_query_param(
            "status",
            serde_json::json!({"type": "string", "example": "active"}),
        );
        assert_eq!(required_query_string(&[&p], &spec), "?status=active");
    }

    #[test]
    fn required_query_string_uses_first_enum_value() {
        let spec = empty_spec();
        let p = required_query_param(
            "sort",
            serde_json::json!({"type": "string", "enum": ["asc", "desc"]}),
        );
        assert_eq!(required_query_string(&[&p], &spec), "?sort=asc");
    }

    #[test]
    fn required_query_string_no_schema_defaults_to_string() {
        let spec = empty_spec();
        let p: Parameter = serde_json::from_value(
            serde_json::json!({"name": "q", "in": "query", "required": true}),
        )
        .unwrap();
        assert_eq!(required_query_string(&[&p], &spec), "?q=string");
    }
}
