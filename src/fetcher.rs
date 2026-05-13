use crate::error::O2SError;
use oas3::OpenApiV3Spec;
use std::path::PathBuf;

pub async fn load_oapi(link: &str) -> Result<OpenApiV3Spec, O2SError> {
    if link.starts_with("http://") || link.starts_with("https://") {
        load_http(link).await
    } else {
        load_file(link).await
    }
}

async fn load_http(url: &str) -> Result<OpenApiV3Spec, O2SError> {
    let content = reqwest::get(url).await?.error_for_status()?.text().await?;
    let ext = url_extension(url);
    parse_content(&content, &ext)
}

async fn load_file(path_str: &str) -> Result<OpenApiV3Spec, O2SError> {
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
    match ext {
        "json" => Ok(oas3::from_json(content)?),
        "yaml" | "yml" => Ok(serde_yaml::from_str(content)?),
        other => {
            if let Ok(parsed) = oas3::from_json(content) {
                return Ok(parsed);
            }
            if let Ok(parsed) = serde_yaml::from_str(content) {
                return Ok(parsed);
            }
            Err(O2SError::UnknownExtension(other.to_string()))
        }
    }
}
