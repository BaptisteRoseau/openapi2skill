use clap::Parser;
use std::path::PathBuf;

/// Convert an Open API specification into an agent Skill.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {
    /// Open API source (URL or file path)
    pub path_or_url: String,

    /// Output directory, also used as the name ok the skill when provided.
    #[arg(short, long)]
    pub output_dir: Option<PathBuf>,

    /// Enable stdout output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Remove existing output directory
    #[arg(short, long, default_value_t = false)]
    pub force: bool,

    /// Override the server URL(s) from the spec. Can be specified multiple times
    /// (e.g. `--server api.example.com --server https://staging.example.com`).
    /// If the URL has no scheme, `https://` is prepended automatically. The override
    /// must include any base path the endpoints are relative to (e.g.
    /// `--server http://host:9090/api/v1`)
    #[arg(long, value_name = "URL")]
    pub server: Vec<String>,
}

impl CliConfig {
    /// Rebuilds an invocation command from the parsed config. Used as a fallback when the
    /// original `argv` isn't available to record verbatim (see [`std::env::args`]).
    pub fn to_command_string(&self, program: &str) -> String {
        let mut parts = vec![program.to_string(), self.path_or_url.clone()];

        if let Some(dir) = &self.output_dir {
            parts.push("--output-dir".to_string());
            parts.push(dir.display().to_string());
        }
        if self.verbose {
            parts.push("--verbose".to_string());
        }
        if self.force {
            parts.push("--force".to_string());
        }
        for server in &self.server {
            parts.push("--server".to_string());
            parts.push(server.clone());
        }

        parts.join(" ")
    }
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

    fn config(path_or_url: &str) -> CliConfig {
        CliConfig {
            path_or_url: path_or_url.to_string(),
            output_dir: None,
            verbose: false,
            force: false,
            server: Vec::new(),
        }
    }

    #[test]
    fn command_string_bare_invocation() {
        let cfg = config("spec.json");
        assert_eq!(
            cfg.to_command_string("openapi2skill"),
            "openapi2skill spec.json"
        );
    }

    #[test]
    fn command_string_includes_all_flags() {
        let mut cfg = config("spec.json");
        cfg.output_dir = Some(PathBuf::from("my_skill"));
        cfg.verbose = true;
        cfg.force = true;
        cfg.server = vec!["https://a.example.com".to_string()];
        assert_eq!(
            cfg.to_command_string("openapi2skill"),
            "openapi2skill spec.json --output-dir my_skill --verbose --force --server https://a.example.com"
        );
    }
}
