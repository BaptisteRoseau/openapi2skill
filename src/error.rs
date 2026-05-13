use sppparse::SparseError;

#[derive(Debug, thiserror::Error)]
pub enum O2SError {
    #[error("Failed to fetch URL: {0}")]
    HttpFetch(#[from] reqwest::Error),

    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Failed to parse YAML: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    #[error("Failed to parse OpenAPI schema: {0}")]
    OApiParse(#[from] SparseError),

    #[error("Unknown file extension '{0}': expected json, yaml, or yml")]
    UnknownExtension(String),
}
