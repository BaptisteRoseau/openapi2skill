mod cli;
mod error;
mod fetcher;
mod logging;
mod writer;

use clap::Parser;
use cli::{CliConfig, normalize_server_url};
use fetcher::load_oapi;
use writer::{GenerationContext, openapi2skill};

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
    let command = generation_command(&config);

    let loaded = load_oapi(config.path_or_url.as_str()).await?;
    let generation = GenerationContext {
        source_url,
        manifest_raw: loaded.raw,
        manifest_extension: loaded.manifest_extension,
        command,
        token_env_var: config.token_env_var.clone(),
    };
    openapi2skill(
        &loaded.spec,
        config.output_dir.as_deref(),
        config.force,
        servers,
        generation,
    )
    .await?;

    Ok(())
}

/// The literal command as typed, taken from `argv`. Falls back to rebuilding it from the
/// parsed config on the rare platforms where `argv` isn't available.
fn generation_command(config: &CliConfig) -> String {
    let argv: Vec<String> = std::env::args().collect();
    if argv.is_empty() {
        config.to_command_string(env!("CARGO_BIN_NAME"))
    } else {
        argv.join(" ")
    }
}
