use clap::Parser;
use openapi2skill::{cli::CliConfig, fetcher::load_oapi, writer::openapi2skill};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = CliConfig::parse();
    let doc = load_oapi(config.path_or_url.as_str()).await?;
    openapi2skill(&doc, config.output_dir.as_deref()).await?;
    Ok(())
}
