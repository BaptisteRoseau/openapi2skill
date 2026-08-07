mod cli;
mod error;
mod fetcher;
mod logging;
mod writer;

use clap::Parser;
use cli::{CliConfig, normalize_server_url};
use fetcher::load_oapi;
use writer::openapi2skill;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = CliConfig::parse();
    logging::init_logger(config.verbose);

    let servers: Vec<String> = config
        .server
        .iter()
        .map(|s| normalize_server_url(s))
        .collect();

    let source_url = fetcher::is_url(&config.path_or_url).then(|| config.path_or_url.clone());

    let doc = load_oapi(config.path_or_url.as_str()).await?;
    openapi2skill(
        &doc,
        config.output_dir.as_deref(),
        config.force,
        servers,
        source_url,
    )
    .await?;

    Ok(())
}
