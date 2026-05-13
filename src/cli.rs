use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

/// Convert an Open API specification into an agent Skill.
#[derive(Parser, Deserialize, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {
    /// Open API source (URL or file path)
    #[arg(short, long)]
    pub path_or_url: String,

    /// Output directory
    #[arg(short, long)]
    pub output_dir: Option<PathBuf>,

    /// Disable output
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,
}
