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

use super::utils::{CollectWrites, infer_skill_name};
use super::{auth, endpoint, schema, skill};

pub async fn openapi2skill(
    spec: &OpenApiV3Spec,
    output_dir: Option<&Path>,
    force: bool,
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

    let mut writes: Vec<(PathBuf, String)> = Vec::new();

    let skill_writer = skill::Writer { name: skill_name };
    let writers: &[&dyn CollectWrites] = &[
        &skill_writer,
        &auth::Writer,
        &endpoint::Writer,
        &schema::Writer,
    ];
    for w in writers {
        w.collect_writes(spec, &dir, &mut writes);
    }

    write_all(writes).await?;
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
