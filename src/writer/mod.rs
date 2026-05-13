mod auth;
mod endpoint;
mod schema;
mod skill;

use oas3::OpenApiV3Spec;
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn openapi2skill(
    spec: &OpenApiV3Spec,
    output_dir: Option<&Path>,
) -> Result<(), anyhow::Error> {
    let dir: PathBuf = output_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(to_snake_case(&spec.info.title)));

    let mut writes: Vec<(PathBuf, String)> = Vec::new();

    skill::collect_writes(spec, &dir, &mut writes);
    auth::collect_writes(spec, &dir, &mut writes);
    endpoint::collect_writes(spec, &dir, &mut writes);
    schema::collect_writes(spec, &dir, &mut writes);

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

pub(crate) fn to_snake_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

pub(crate) fn camel_to_kebab(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('-');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

pub(crate) fn path_to_slug(path: &str) -> String {
    path.split('/')
        .filter(|s| !s.is_empty())
        .map(|seg| {
            let seg = seg.trim_start_matches('{').trim_end_matches('}');
            camel_to_kebab(seg)
        })
        .collect::<Vec<_>>()
        .join("-")
}
