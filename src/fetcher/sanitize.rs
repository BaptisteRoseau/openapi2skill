use serde_json::{Map, Value};
use tracing::warn;

/// Normalizes non-compliant fields found in real-world OpenAPI specs.
///
/// - `type: "any"` / `"any"` in a type array -> dropped.
/// - boolean `required` in a schema body -> dropped.
/// - boolean `exclusiveMinimum`/`exclusiveMaximum` paired with a numeric `minimum`/`maximum`
///   -> converted to the OpenAPI 3.1 numeric-bound form; unpaired -> dropped.
/// - non-string `description` -> dropped.
/// - null security requirement scopes -> `[]`.
/// - non-absolute `url`/`termsOfService`/`authorizationUrl`/`tokenUrl`/`refreshUrl` -> dropped.
pub(super) fn sanitize_invalid_types(value: Value) -> Value {
    sanitize(value, false)
}

fn sanitize(value: Value, in_schema: bool) -> Value {
    match value {
        Value::Object(map) => sanitize_object(map, in_schema),
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(|v| sanitize(v, in_schema)).collect())
        }
        other => other,
    }
}

fn sanitize_object(map: Map<String, Value>, in_schema: bool) -> Value {
    let map = if in_schema {
        normalize_exclusive_bounds(map)
    } else {
        map
    };
    let mut out = Map::with_capacity(map.len());
    for (key, value) in map {
        if let Some(sanitized) = sanitize_entry(&key, value, in_schema) {
            out.insert(key, sanitized);
        }
    }
    Value::Object(out)
}

/// Returns the value to keep under `key`, or `None` to drop the entry entirely.
fn sanitize_entry(key: &str, value: Value, in_schema: bool) -> Option<Value> {
    match key {
        "example" | "examples" | "default" => Some(value),
        "type" => normalize_type_value(&value),
        "required" if in_schema => sanitize_schema_required(value),
        "exclusiveMinimum" | "exclusiveMaximum" if in_schema => {
            sanitize_leftover_exclusive_bound(key, value)
        }
        "schema" => Some(sanitize(value, true)),
        "schemas" => Some(sanitize_map_values(value, true)),
        "items" | "additionalProperties" | "not" if in_schema => Some(sanitize(value, true)),
        "properties" if in_schema => Some(sanitize_map_values(value, true)),
        "allOf" | "anyOf" | "oneOf" | "prefixItems" if in_schema => {
            Some(sanitize_array_items(value, true))
        }
        "description" => sanitize_description(value),
        "security" => Some(sanitize_security_requirements(value)),
        "externalDocs" => sanitize_external_docs(value),
        "license" | "contact" => Some(sanitize_url_field(value, "url")),
        "termsOfService" | "authorizationUrl" | "tokenUrl" | "refreshUrl" => {
            sanitize_absolute_url(key, value)
        }
        _ => Some(sanitize(value, in_schema)),
    }
}

fn sanitize_schema_required(value: Value) -> Option<Value> {
    if matches!(value, Value::Bool(_)) {
        warn!("Dropping boolean `required` inside schema body (not valid in OpenAPI 3.x schemas)");
        return None;
    }
    Some(sanitize(value, false))
}

fn sanitize_leftover_exclusive_bound(key: &str, value: Value) -> Option<Value> {
    if matches!(value, Value::Bool(_)) {
        warn!(
            "Dropping boolean `{key}` inside schema body (OpenAPI 3.0 draft-4 style; no numeric minimum/maximum sibling to convert)"
        );
        return None;
    }
    Some(value)
}

fn sanitize_description(value: Value) -> Option<Value> {
    match value {
        Value::String(_) => Some(value),
        other => {
            warn!(
                "Dropping non-string `description` value (unsupported external reference): {other:?}"
            );
            None
        }
    }
}

fn sanitize_absolute_url(key: &str, value: Value) -> Option<Value> {
    if let Value::String(s) = &value
        && !is_absolute_url(s)
    {
        warn!("Dropping non-absolute `{key}` value: {s:?}");
        return None;
    }
    Some(value)
}

/// Converts OpenAPI 3.0 draft-4 boolean `exclusiveMinimum`/`exclusiveMaximum` (paired with a
/// numeric `minimum`/`maximum`) into the OpenAPI 3.1 form, where the bound itself is exclusive.
/// `false` is simply dropped, since inclusive is already `minimum`/`maximum`'s default meaning.
fn normalize_exclusive_bounds(mut map: Map<String, Value>) -> Map<String, Value> {
    for (excl_key, bound_key) in [
        ("exclusiveMinimum", "minimum"),
        ("exclusiveMaximum", "maximum"),
    ] {
        match map.get(excl_key) {
            Some(Value::Bool(true)) => {
                if let Some(bound) = map.remove(bound_key) {
                    map.insert(excl_key.to_string(), bound);
                }
            }
            Some(Value::Bool(false)) => {
                map.remove(excl_key);
            }
            _ => {}
        }
    }
    map
}

fn sanitize_map_values(value: Value, in_schema: bool) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(k, v)| (k, sanitize(v, in_schema)))
                .collect(),
        ),
        other => other,
    }
}

fn sanitize_array_items(value: Value, in_schema: bool) -> Value {
    match value {
        Value::Array(arr) => Value::Array(
            arr.into_iter()
                .map(|item| sanitize(item, in_schema))
                .collect(),
        ),
        other => other,
    }
}

fn is_absolute_url(s: &str) -> bool {
    url::Url::parse(s).is_ok()
}

/// Drops `externalDocs` when its `url` is missing or not absolute.
fn sanitize_external_docs(value: Value) -> Option<Value> {
    match value {
        Value::Object(map) => match map.get("url") {
            Some(Value::String(s)) if is_absolute_url(s) => Some(Value::Object(map)),
            other => {
                warn!("Dropping `externalDocs` with missing/invalid absolute `url`: {other:?}");
                None
            }
        },
        other => Some(other),
    }
}

/// Drops `field` from `value` when it isn't a valid absolute URL.
fn sanitize_url_field(value: Value, field: &str) -> Value {
    match value {
        Value::Object(mut map) => {
            if let Some(Value::String(s)) = map.get(field)
                && !is_absolute_url(s)
            {
                warn!("Dropping non-absolute `{field}` value: {s:?}");
                map.remove(field);
            }
            Value::Object(map)
        }
        other => other,
    }
}

fn sanitize_security_requirements(value: Value) -> Value {
    match value {
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(sanitize_security_requirement).collect())
        }
        other => other,
    }
}

fn sanitize_security_requirement(req: Value) -> Value {
    match req {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(scheme, scopes)| {
                    if scopes.is_null() {
                        warn!("Replacing null security scopes for `{scheme}` with an empty array");
                        (scheme, Value::Array(Vec::new()))
                    } else {
                        (scheme, scopes)
                    }
                })
                .collect(),
        ),
        other => other,
    }
}

fn normalize_type_value(v: &Value) -> Option<Value> {
    match v {
        Value::String(s) if s == "any" => {
            warn!("Stripping invalid schema type \"any\" (not part of OpenAPI 3.x)");
            None
        }
        Value::Array(arr) => {
            let kept: Vec<Value> = arr
                .iter()
                .filter(|item| matches!(item, Value::String(s) if s != "any"))
                .cloned()
                .collect();
            if kept.len() != arr.len() {
                warn!("Stripping invalid entries from schema type array: {arr:?} → {kept:?}");
            }
            if kept.is_empty() {
                warn!("Dropping schema type array with no usable type: {arr:?}");
                return None;
            }
            Some(Value::Array(kept))
        }
        // Pass through valid string types, sub-schemas under a property literally
        // named "type", etc.
        other => Some(other.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- normalize_type_value ---

    #[test]
    fn normalize_any_returns_none() {
        assert_eq!(normalize_type_value(&json!("any")), None);
    }

    #[test]
    fn normalize_valid_string_passthrough() {
        assert_eq!(
            normalize_type_value(&json!("string")),
            Some(json!("string"))
        );
    }

    #[test]
    fn normalize_array_preserves_nullable_string() {
        assert_eq!(
            normalize_type_value(&json!(["string", "null"])),
            Some(json!(["string", "null"]))
        );
    }

    #[test]
    fn normalize_array_preserves_ordering() {
        assert_eq!(
            normalize_type_value(&json!(["null", "integer"])),
            Some(json!(["null", "integer"]))
        );
    }

    #[test]
    fn normalize_array_filters_any_keeps_rest() {
        assert_eq!(
            normalize_type_value(&json!(["null", "any", "integer"])),
            Some(json!(["null", "integer"]))
        );
    }

    #[test]
    fn normalize_array_only_null_passes_through() {
        // A schema saying "this value is always null" is valid in 3.1.
        assert_eq!(
            normalize_type_value(&json!(["null"])),
            Some(json!(["null"]))
        );
    }

    #[test]
    fn normalize_array_only_any_returns_none() {
        assert_eq!(normalize_type_value(&json!(["any"])), None);
    }

    #[test]
    fn normalize_array_only_any_and_nothing_else_returns_none() {
        assert_eq!(normalize_type_value(&json!(["any", "any"])), None);
    }

    // --- sanitize_invalid_types ---

    #[test]
    fn sanitize_drops_any_type() {
        let input = json!({"type": "any", "description": "x"});
        let out = sanitize_invalid_types(input);
        assert!(out.get("type").is_none());
        assert_eq!(out.get("description"), Some(&json!("x")));
    }

    #[test]
    fn sanitize_preserves_type_array() {
        let input = json!({"type": ["string", "null"]});
        let out = sanitize_invalid_types(input);
        assert_eq!(out.get("type"), Some(&json!(["string", "null"])));
    }

    #[test]
    fn sanitize_recurses_into_nested_objects() {
        let input = json!({"properties": {"name": {"type": "any"}}});
        let out = sanitize_invalid_types(input);
        let name = &out["properties"]["name"];
        assert!(name.get("type").is_none());
    }

    #[test]
    fn sanitize_recurses_into_arrays() {
        let input = json!([{"type": "any"}, {"type": "string"}]);
        let out = sanitize_invalid_types(input);
        assert!(out[0].get("type").is_none());
        assert_eq!(out[1].get("type"), Some(&json!("string")));
    }

    #[test]
    fn sanitize_drops_boolean_required_in_schema() {
        let input = json!({"schema": {"type": "object", "required": true}});
        let out = sanitize_invalid_types(input);
        assert!(out["schema"].get("required").is_none());
    }

    #[test]
    fn sanitize_keeps_required_list_in_schema() {
        let input = json!({"schema": {"type": "object", "required": ["a"]}});
        let out = sanitize_invalid_types(input);
        assert_eq!(out["schema"].get("required"), Some(&json!(["a"])));
    }

    #[test]
    fn sanitize_drops_non_string_description() {
        let input = json!({"description": {"$ref": "./other.md"}});
        let out = sanitize_invalid_types(input);
        assert!(out.get("description").is_none());
    }

    #[test]
    fn sanitize_drops_external_docs_without_absolute_url() {
        let input = json!({"externalDocs": {"url": "/relative/path"}});
        let out = sanitize_invalid_types(input);
        assert!(out.get("externalDocs").is_none());
    }

    #[test]
    fn sanitize_keeps_external_docs_with_absolute_url() {
        let input = json!({"externalDocs": {"url": "https://example.com/docs"}});
        let out = sanitize_invalid_types(input);
        assert!(out.get("externalDocs").is_some());
    }

    #[test]
    fn sanitize_drops_non_absolute_license_url_but_keeps_object() {
        let input = json!({"license": {"name": "MIT", "url": "/license"}});
        let out = sanitize_invalid_types(input);
        assert_eq!(out["license"].get("name"), Some(&json!("MIT")));
        assert!(out["license"].get("url").is_none());
    }

    #[test]
    fn sanitize_drops_non_absolute_terms_of_service() {
        let input = json!({"termsOfService": "/tos"});
        let out = sanitize_invalid_types(input);
        assert!(out.get("termsOfService").is_none());
    }

    #[test]
    fn sanitize_replaces_null_security_scopes_with_empty_array() {
        let input = json!({"security": [{"oauth": null}]});
        let out = sanitize_invalid_types(input);
        assert_eq!(out["security"][0]["oauth"], json!([]));
    }

    #[test]
    fn sanitize_recurses_into_all_of_subschemas() {
        let input = json!({"schema": {"allOf": [{"type": "any"}, {"type": "string"}]}});
        let out = sanitize_invalid_types(input);
        assert!(out["schema"]["allOf"][0].get("type").is_none());
        assert_eq!(
            out["schema"]["allOf"][1].get("type"),
            Some(&json!("string"))
        );
    }

    // --- normalize_exclusive_bounds / exclusiveMinimum-Maximum sanitization ---

    #[test]
    fn exclusive_minimum_true_converts_to_numeric_form() {
        let input = json!({"schema": {"properties": {"age": {"type": "integer", "minimum": 5, "exclusiveMinimum": true}}}});
        let out = sanitize_invalid_types(input);
        let age = &out["schema"]["properties"]["age"];
        assert_eq!(age.get("exclusiveMinimum"), Some(&json!(5)));
        assert!(age.get("minimum").is_none());
    }

    #[test]
    fn exclusive_maximum_true_converts_to_numeric_form() {
        let input = json!({"schema": {"properties": {"age": {"type": "integer", "maximum": 120, "exclusiveMaximum": true}}}});
        let out = sanitize_invalid_types(input);
        let age = &out["schema"]["properties"]["age"];
        assert_eq!(age.get("exclusiveMaximum"), Some(&json!(120)));
        assert!(age.get("maximum").is_none());
    }

    #[test]
    fn exclusive_minimum_false_is_dropped_and_minimum_kept() {
        let input = json!({"schema": {"properties": {"age": {"type": "integer", "minimum": 5, "exclusiveMinimum": false}}}});
        let out = sanitize_invalid_types(input);
        let age = &out["schema"]["properties"]["age"];
        assert!(age.get("exclusiveMinimum").is_none());
        assert_eq!(age.get("minimum"), Some(&json!(5)));
    }

    #[test]
    fn exclusive_minimum_true_without_sibling_is_dropped() {
        let input = json!({"schema": {"properties": {"age": {"type": "integer", "exclusiveMinimum": true}}}});
        let out = sanitize_invalid_types(input);
        let age = &out["schema"]["properties"]["age"];
        assert!(age.get("exclusiveMinimum").is_none());
    }

    #[test]
    fn exclusive_minimum_numeric_form_passes_through() {
        // Already OpenAPI 3.1 style; nothing to convert.
        let input =
            json!({"schema": {"properties": {"age": {"type": "integer", "exclusiveMinimum": 5}}}});
        let out = sanitize_invalid_types(input);
        let age = &out["schema"]["properties"]["age"];
        assert_eq!(age.get("exclusiveMinimum"), Some(&json!(5)));
    }
}
