use crate::error::O2SError;
use oapi::{OApi, OApiDocument};
use sppparse::SparseRoot;
use std::path::PathBuf;

pub fn load_oapi(link: &str) -> Result<OApi, O2SError> {
    if link.starts_with("http://") || link.starts_with("https://") {
        load_http(link)
    } else {
        load_file(link)
    }
}

fn load_http(url: &str) -> Result<OApi, O2SError> {
    let content = reqwest::blocking::get(url)?
        .error_for_status()?
        .text()?;
    let ext = url_extension(url);
    let value = parse_content(&content, &ext)?;
    let root = SparseRoot::<OApiDocument>::new_from_value(value, PathBuf::from("openapi.json"), vec![])?;
    Ok(OApi::new(root))
}

fn load_file(path_str: &str) -> Result<OApi, O2SError> {
    let content = std::fs::read_to_string(path_str)?;
    let path = PathBuf::from(path_str);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let value = parse_content(&content, &ext)?;
    let root = SparseRoot::<OApiDocument>::new_from_value(value, path, vec![])?;
    Ok(OApi::new(root))
}

fn url_extension(url: &str) -> String {
    let without_query = url.split('?').next().unwrap_or(url);
    let last_segment = without_query.rsplit('/').next().unwrap_or(without_query);
    match last_segment.rfind('.') {
        Some(dot_pos) => last_segment[dot_pos + 1..].to_lowercase(),
        None => String::new(),
    }
}

fn parse_content(content: &str, ext: &str) -> Result<serde_json::Value, O2SError> {
    match ext {
        "json" => Ok(serde_json::from_str(content)?),
        "yaml" | "yml" => Ok(serde_yaml::from_str(content)?),
        other => {
            if let Ok(parsed) = serde_json::from_str(content) {
                return Ok(parsed);
            }
            if let Ok(parsed) = serde_yaml::from_str(content) {
                return Ok(parsed);
            }
            Err(O2SError::UnknownExtension(other.to_string()))
        }
    }
}
