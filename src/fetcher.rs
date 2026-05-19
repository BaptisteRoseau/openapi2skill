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

/// Normalize schema `type` fields so the `oas3` crate (OpenAPI 3.0 semantics) can
/// parse OpenAPI 3.1 specs and minor non-compliances:
/// - `"type": "any"` → drop the field (not a valid type in any spec version).
/// - `"type": ["string", "null"]` (3.1 nullable form) → `"type": "string"`. Picks
///   the first non-null/non-"any" string; drops the field if none qualify.
fn sanitize_invalid_types(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (k, v) in map {
                if k == "type" {
                    if let Some(normalized) = normalize_type_value(&v) {
                        out.insert(k, normalized);
                    }
                    continue;
                }
                out.insert(k, sanitize_invalid_types(v));
            }
            Value::Object(out)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(sanitize_invalid_types).collect()),
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
            let picked = arr.iter().find_map(|item| match item {
                Value::String(s) if s != "null" && s != "any" => Some(s.clone()),
                _ => None,
            });
            match picked {
                Some(t) => {
                    warn!("Normalizing OpenAPI 3.1 type array {arr:?} → \"{t}\"");
                    Some(Value::String(t))
                }
                None => {
                    warn!("Dropping schema type array with no usable type: {arr:?}");
                    None
                }
            }
        }
        // Pass through valid string types, sub-schemas under a property literally
        // named "type", etc.
        other => Some(other.clone()),
    }
}
