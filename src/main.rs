mod cli;
mod error;
mod fetcher;
mod writer;
use crate::{cli::CliConfig, fetcher::load_oapi, writer::openapi2skill};
use clap::Parser;

fn main() -> Result<(), anyhow::Error> {
    let config = CliConfig::parse();
    let doc = load_oapi(config.path_or_url.as_str())?;
    openapi2skill(&doc, config.output_dir.as_deref())?;
    Ok(())
}
