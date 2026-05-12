mod models;
mod fetcher;
use clap::Parser;

use crate::{cli::CliConfig, fetcher::load_oapi};

mod cli;

fn main() -> Result<(), anyhow::Error> {
    let config = CliConfig::parse();
    let doc = load_oapi(config.config)
    Ok(())
}
