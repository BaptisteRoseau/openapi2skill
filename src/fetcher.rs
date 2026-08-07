use crate::error::O2SError;
use oas3::OpenApiV3Spec;
use serde_json::Value;
use std::path::PathBuf;
use tracing::{info, warn};

pub async fn load_oapi(link: &str) -> Result<OpenApiV3Spec, O2SError> {
    if link.starts_with("http://") || link.starts_with("https://") {
        load_http(link).await
    } else {
        load_file(link).await
    }
}

async fn load_http(url: &str) -> Result<OpenApiV3Spec, O2SError> {
    info!("Fetching {url}");
    let content = reqwest::get(url).await?.error_for_status()?.text().await?;
    let ext = url_extension(url);
    parse_content(&content, &ext)
}

async fn load_file(path_str: &str) -> Result<OpenApiV3Spec, O2SError> {
    info!("Loading file {path_str}");
    let content = tokio::fs::read_to_string(path_str).await?;
    let path = PathBuf::from(path_str);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    parse_content(&content, &ext)
}

fn url_extension(url: &str) -> String {
    let without_query = url.split('?').next().unwrap_or(url);
    let last_segment = without_query.rsplit('/').next().unwrap_or(without_query);
    match last_segment.rfind('.') {
        Some(dot_pos) => last_segment[dot_pos + 1..].to_lowercase(),
        None => String::new(),
    }
}

fn parse_content(content: &str, ext: &str) -> Result<OpenApiV3Spec, O2SError> {
    let value: Value = match ext {
        "json" => serde_json::from_str(content)?,
        "yaml" | "yml" => serde_yaml::from_str(content)?,
        other => {
            parse_unknown(content).ok_or_else(|| O2SError::InvalidFormat(other.to_string()))?
        }
    };
    if let Some(version) = value.get("swagger").and_then(|v| v.as_str()) {
        return Err(O2SError::UnsupportedSwaggerVersion(version.to_string()));
    }
    let sanitized = sanitize_invalid_types(value);
    let json_text = serde_json::to_string(&sanitized)?;
    Ok(oas3::from_json(&json_text)?)
}

fn parse_unknown(content: &str) -> Option<Value> {
    serde_json::from_str(content)
        .ok()
        .or_else(|| serde_yaml::from_str(content).ok())
}

/// Normalizes non-compliant fields found in real-world OpenAPI specs.
///
/// - `type: "any"` / `"any"` in a type array -> dropped.
/// - boolean `required` in a schema body -> dropped.
/// - boolean `exclusiveMinimum`/`exclusiveMaximum` -> dropped.
/// - non-string `description` -> dropped.
/// - null security requirement scopes -> `[]`.
/// - non-absolute `url`/`termsOfService`/`authorizationUrl`/`tokenUrl`/`refreshUrl` -> dropped.
fn sanitize_invalid_types(value: Value) -> Value {
    sanitize(value, false)
}

fn sanitize(value: Value, in_schema: bool) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (k, v) in map {
                match k.as_str() {
                    // Opaque sample data; left untouched.
                    "example" | "examples" | "default" => {
                        out.insert(k, v);
                    }
                    "type" => {
                        if let Some(normalized) = normalize_type_value(&v) {
                            out.insert(k, normalized);
                        }
                    }
                    // `required` in a schema body must be Vec<String>; a boolean is invalid.
                    "required" if in_schema => {
                        if matches!(v, Value::Bool(_)) {
                            warn!(
                                "Dropping boolean `required` inside schema body (not valid in OpenAPI 3.x schemas)"
                            );
                        } else {
                            out.insert(k, sanitize(v, false));
                        }
                    }
                    // OpenAPI 3.0 boolean form; oas3 expects a numeric bound.
                    "exclusiveMinimum" | "exclusiveMaximum" if in_schema => {
                        if matches!(v, Value::Bool(_)) {
                            warn!(
                                "Dropping boolean `{k}` inside schema body (OpenAPI 3.0 draft-4 style, not valid in OpenAPI 3.1 schemas)"
                            );
                        } else {
                            out.insert(k, v);
                        }
                    }
                    // These keys always introduce a schema value.
                    "schema" => {
                        out.insert(k, sanitize(v, true));
                    }
                    // `schemas` under `components` is a map of schema objects.
                    "schemas" => {
                        out.insert(k, sanitize_map_values(v, true));
                    }
                    // Sub-schema keys that are only meaningful inside a schema object.
                    "items" | "additionalProperties" | "not" if in_schema => {
                        out.insert(k, sanitize(v, true));
                    }
                    // Each property value is a schema.
                    "properties" if in_schema => {
                        out.insert(k, sanitize_map_values(v, true));
                    }
                    // Each array item is a schema.
                    "allOf" | "anyOf" | "oneOf" | "prefixItems" if in_schema => {
                        let new_v = match v {
                            Value::Array(arr) => {
                                Value::Array(arr.into_iter().map(|i| sanitize(i, true)).collect())
                            }
                            other => other,
                        };
                        out.insert(k, new_v);
                    }
                    // description must be a string here.
                    "description" => match v {
                        Value::String(_) => {
                            out.insert(k, v);
                        }
                        other => {
                            warn!(
                                "Dropping non-string `description` value (unsupported external reference): {other:?}"
                            );
                        }
                    },
                    // null scopes -> empty array.
                    "security" => {
                        out.insert(k, sanitize_security_requirements(v));
                    }
                    // url is required here; drop the whole object if invalid.
                    "externalDocs" => {
                        if let Some(sanitized) = sanitize_external_docs(v) {
                            out.insert(k, sanitized);
                        }
                    }
                    // url is optional here; just drop the field.
                    "license" | "contact" => {
                        out.insert(k, sanitize_url_field(v, "url"));
                    }
                    // Standalone fields that oas3 requires to be absolute URLs.
                    "termsOfService" | "authorizationUrl" | "tokenUrl" | "refreshUrl" => {
                        if let Value::String(s) = &v
                            && !is_absolute_url(s)
                        {
                            warn!("Dropping non-absolute `{k}` value: {s:?}");
                        } else {
                            out.insert(k, v);
                        }
                    }
                    _ => {
                        out.insert(k, sanitize(v, in_schema));
                    }
                }
            }
            Value::Object(out)
        }
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(|v| sanitize(v, in_schema)).collect())
        }
        other => other,
    }
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

    // --- url_extension ---

    #[test]
    fn url_extension_json() {
        assert_eq!(url_extension("https://example.com/spec.json"), "json");
    }

    #[test]
    fn url_extension_yaml() {
        assert_eq!(url_extension("https://example.com/api.yaml"), "yaml");
    }

    #[test]
    fn url_extension_strips_query_string() {
        assert_eq!(
            url_extension("https://example.com/spec.json?version=2"),
            "json"
        );
    }

    #[test]
    fn url_extension_empty_when_no_dot() {
        assert_eq!(url_extension("https://example.com/spec"), "");
    }

    #[test]
    fn url_extension_lowercases() {
        assert_eq!(url_extension("https://example.com/spec.JSON"), "json");
    }

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

    // --- parse_content ---

    #[test]
    fn parse_content_rejects_swagger_2() {
        let err =
            parse_content(r#"{"swagger": "2.0", "info": {}, "paths": {}}"#, "json").unwrap_err();
        assert!(matches!(err, O2SError::UnsupportedSwaggerVersion(v) if v == "2.0"));
    }
}
