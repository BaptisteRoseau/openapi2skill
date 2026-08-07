use crate::error::O2SError;
use oas3::OpenApiV3Spec;
use serde_json::Value;
use std::path::Path;
use tracing::info;

use super::sanitize::sanitize_invalid_types;

/// The parsed spec together with the verbatim source text, kept so it can be written out
/// unmodified as the skill's `openapi.json`/`openapi.yml` manifest.
pub struct LoadedSpec {
    pub spec: OpenApiV3Spec,
    pub raw: String,
    pub manifest_extension: &'static str,
}

pub async fn load_oapi(link: &str) -> Result<LoadedSpec, O2SError> {
    let (content, ext) = if is_url(link) {
        fetch_url(link).await?
    } else {
        read_file(link).await?
    };
    let spec = parse_content(&content, &ext)?;
    Ok(LoadedSpec {
        manifest_extension: manifest_extension(&content, &ext),
        spec,
        raw: content,
    })
}

pub fn is_url(link: &str) -> bool {
    link.starts_with("http://") || link.starts_with("https://")
}

async fn fetch_url(url: &str) -> Result<(String, String), O2SError> {
    info!("Fetching {url}");
    let content = reqwest::get(url).await?.error_for_status()?.text().await?;
    Ok((content, url_extension(url)))
}

async fn read_file(path: &str) -> Result<(String, String), O2SError> {
    info!("Loading file {path}");
    let content = tokio::fs::read_to_string(path).await?;
    Ok((content, path_extension(path)))
}

fn path_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_lowercase()
}

fn url_extension(url: &str) -> String {
    let without_query = url.split('?').next().unwrap_or(url);
    let last_segment = without_query.rsplit('/').next().unwrap_or(without_query);
    match last_segment.rfind('.') {
        Some(dot_pos) => last_segment[dot_pos + 1..].to_lowercase(),
        None => String::new(),
    }
}

/// `yaml` is normalized to `yml` per the skill's manifest naming convention. When the
/// source extension isn't recognized, the format is detected from content.
fn manifest_extension(content: &str, ext: &str) -> &'static str {
    match ext {
        "yaml" | "yml" => "yml",
        "json" => "json",
        _ if serde_json::from_str::<Value>(content).is_ok() => "json",
        _ => "yml",
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
    let json_text = serde_json::to_string(&sanitize_invalid_types(value))?;
    Ok(oas3::from_json(&json_text)?)
}

fn parse_unknown(content: &str) -> Option<Value> {
    serde_json::from_str(content)
        .ok()
        .or_else(|| serde_yaml::from_str(content).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_url ---

    #[test]
    fn is_url_http() {
        assert!(is_url("http://example.com/spec.json"));
    }

    #[test]
    fn is_url_https() {
        assert!(is_url("https://example.com/spec.json"));
    }

    #[test]
    fn is_url_rejects_local_path() {
        assert!(!is_url("tests/assets/petstore.json"));
    }

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

    // --- path_extension ---

    #[test]
    fn path_extension_json() {
        assert_eq!(path_extension("tests/assets/petstore.json"), "json");
    }

    #[test]
    fn path_extension_lowercases() {
        assert_eq!(path_extension("spec.YAML"), "yaml");
    }

    #[test]
    fn path_extension_empty_when_none() {
        assert_eq!(path_extension("spec"), "");
    }

    // --- manifest_extension ---

    #[test]
    fn manifest_extension_json_passthrough() {
        assert_eq!(manifest_extension("{}", "json"), "json");
    }

    #[test]
    fn manifest_extension_normalizes_yaml_to_yml() {
        assert_eq!(manifest_extension("openapi: 3.0.0", "yaml"), "yml");
    }

    #[test]
    fn manifest_extension_keeps_yml() {
        assert_eq!(manifest_extension("openapi: 3.0.0", "yml"), "yml");
    }

    #[test]
    fn manifest_extension_detects_json_from_content_when_ext_unknown() {
        assert_eq!(manifest_extension(r#"{"openapi":"3.0.0"}"#, ""), "json");
    }

    #[test]
    fn manifest_extension_falls_back_to_yml_when_ext_unknown_and_not_json() {
        assert_eq!(manifest_extension("openapi: 3.0.0", ""), "yml");
    }

    // --- parse_content ---

    #[test]
    fn parse_content_rejects_swagger_2() {
        let err =
            parse_content(r#"{"swagger": "2.0", "info": {}, "paths": {}}"#, "json").unwrap_err();
        assert!(matches!(err, O2SError::UnsupportedSwaggerVersion(v) if v == "2.0"));
    }

    #[test]
    fn parse_content_reads_yaml() {
        let spec = parse_content(
            "openapi: 3.0.0\ninfo:\n  title: T\n  version: '1'\npaths: {}\n",
            "yaml",
        )
        .unwrap();
        assert_eq!(spec.info.title, "T");
    }

    #[test]
    fn parse_content_sniffs_unknown_extension() {
        let spec = parse_content(
            r#"{"openapi":"3.0.0","info":{"title":"T","version":"1"},"paths":{}}"#,
            "",
        )
        .unwrap();
        assert_eq!(spec.info.title, "T");
    }

    #[test]
    fn parse_content_rejects_unparseable_unknown_extension() {
        let err = parse_content("not json or yaml: [", "txt").unwrap_err();
        assert!(matches!(err, O2SError::InvalidFormat(ext) if ext == "txt"));
    }
}
