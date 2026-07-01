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
    let sanitized = sanitize_invalid_types(value);
    let json_text = serde_json::to_string(&sanitized)?;
    Ok(oas3::from_json(&json_text)?)
}

fn parse_unknown(content: &str) -> Option<Value> {
    serde_json::from_str(content)
        .ok()
        .or_else(|| serde_yaml::from_str(content).ok())
}

/// Normalize spec fields for minor non-compliances encountered in real-world OpenAPI specs.
///
/// Runs two passes in one tree walk, tracking whether we are inside a schema object:
///
/// - `"type": "any"` → drop (not a valid OpenAPI 3.x type).
/// - `"any"` inside a `"type"` array → filtered out; field dropped if nothing remains.
/// - `"required": <bool>` inside a schema body → drop. OpenAPI schemas require `required` to
///   be a string array; some specs wrongly embed the parameter-level `required` flag inside
///   the schema object itself (e.g. Spotify's `image/jpeg` upload schema).
fn sanitize_invalid_types(value: Value) -> Value {
    sanitize(value, false)
}

fn sanitize(value: Value, in_schema: bool) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (k, v) in map {
                match k.as_str() {
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
}
