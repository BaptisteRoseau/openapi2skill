mod cli;
mod error;
mod fetcher;
mod logging;
mod writer;

use clap::Parser;
use cli::CliConfig;
use fetcher::load_oapi;
use writer::openapi2skill;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = CliConfig::parse();
    logging::init_logger(config.verbose);

    let doc = load_oapi(config.path_or_url.as_str()).await?;
    openapi2skill(&doc, config.output_dir.as_deref()).await?;

    Ok(())
}
