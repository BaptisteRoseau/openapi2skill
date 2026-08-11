//! Entry point for the writer pipeline. Orchestrates all writers and flushes output to disk.
//!
//! [`openapi2skill`] runs each writer ([`skill`], [`auth`], [`endpoint`], [`schema`]) synchronously
//! to collect `(PathBuf, String)` pairs, then spawns one `tokio::task` per pair for parallel async
//! file I/O via [`write_all`].

use std::path::{Path, PathBuf};

use oas3::OpenApiV3Spec;
use tokio::fs;
use tracing::info;

use crate::error::O2SError;

use super::utils::{CollectWrites, Writes, infer_skill_name};
use super::{auth, endpoint, manifest, schema, skill};

/// Everything about how and where this skill was generated, as opposed to the spec content
/// itself: where the spec came from, its verbatim text for the manifest file, and the command
/// that produced this skill.
pub struct GenerationContext {
    pub source_url: Option<String>,
    pub manifest_raw: String,
    pub manifest_extension: &'static str,
    pub command: String,
    /// Environment variable holding the API credentials, surfaced in the authentication files.
    pub token_env_var: Option<String>,
}

pub async fn openapi2skill(
    spec: &OpenApiV3Spec,
    output_dir: Option<&Path>,
    force: bool,
    servers_override: Vec<String>,
    generation: GenerationContext,
) -> Result<(), anyhow::Error> {
    let skill_name = infer_skill_name(&spec.info.title, output_dir);
    let dir: PathBuf = output_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(&skill_name));

    if dir.exists() {
        if force {
            fs::remove_dir_all(&dir).await?;
        } else {
            return Err(O2SError::OutputDirExists(dir).into());
        }
    }

    let mut writes = Writes::default();

    let manifest_writer = manifest::Writer {
        raw: generation.manifest_raw,
        extension: generation.manifest_extension,
    };
    let skill_writer = skill::Writer {
        name: skill_name,
        servers_override: servers_override.clone(),
        source_url: generation.source_url,
        manifest_filename: manifest_writer.filename(),
        command: generation.command,
    };
    let endpoint_writer = endpoint::Writer { servers_override };
    let auth_writer = auth::Writer {
        token_env_var: generation.token_env_var,
    };
    let writers: &[&dyn CollectWrites] = &[
        &skill_writer,
        &manifest_writer,
        &auth_writer,
        &endpoint_writer,
        &schema::Writer,
    ];
    for w in writers {
        w.collect_writes(spec, &dir, &mut writes);
    }

    write_all(writes.into_vec()).await?;
    info!("Wrote skill under {:?}", dir);

    Ok(())
}

async fn write_all(writes: Vec<(PathBuf, String)>) -> Result<(), anyhow::Error> {
    let tasks: Vec<_> = writes
        .into_iter()
        .map(|(path, content)| {
            tokio::spawn(async move {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).await?;
                }
                fs::write(&path, content.as_bytes()).await?;
                Ok::<(), std::io::Error>(())
            })
        })
        .collect();

    for task in tasks {
        task.await??;
    }

    Ok(())
}
