use clap::Parser;
use std::path::PathBuf;

/// Convert an Open API specification into an agent Skill.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {
    /// Open API source (URL or file path)
    pub path_or_url: String,

    /// Output directory
    #[arg(short, long)]
    pub output_dir: Option<PathBuf>,

    /// Enable output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Remove existing output directory
    #[arg(short, long, default_value_t = false)]
    pub force: bool,

    /// Override the server URL(s) from the spec. Can be specified multiple times
    /// (e.g. `--server api.example.com --server https://staging.example.com`).
    /// If the URL has no scheme, `https://` is prepended automatically.
    #[arg(long, value_name = "URL")]
    pub server: Vec<String>,
}

pub fn normalize_server_url(url: &str) -> String {
    if url.contains("://") {
        url.to_owned()
    } else {
        format!("https://{url}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_keeps_https_scheme() {
        assert_eq!(
            normalize_server_url("https://api.example.com"),
            "https://api.example.com"
        );
    }

    #[test]
    fn normalize_keeps_http_scheme() {
        assert_eq!(
            normalize_server_url("http://api.example.com"),
            "http://api.example.com"
        );
    }

    #[test]
    fn normalize_prepends_https_when_no_scheme() {
        assert_eq!(
            normalize_server_url("api.example.com"),
            "https://api.example.com"
        );
    }

    #[test]
    fn normalize_prepends_https_with_path() {
        assert_eq!(
            normalize_server_url("api.example.com/v1"),
            "https://api.example.com/v1"
        );
    }
}
