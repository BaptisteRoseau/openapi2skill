use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use oas3::OpenApiV3Spec;
use tracing::info;

use super::{refs::collect_multi_use_schemas, render::render_endpoint};
use crate::writer::utils::{CollectWrites, category_label, op_category, path_to_slug};

pub(in crate::writer) struct Writer;

impl CollectWrites for Writer {
    fn collect_writes(
        &self,
        spec: &OpenApiV3Spec,
        dir: &Path,
        writes: &mut Vec<(PathBuf, String)>,
    ) {
        let multi_use = collect_multi_use_schemas(spec);
        let mut by_category: HashMap<String, Vec<(String, String, String)>> = HashMap::new();

        for (path, method, op) in spec.operations() {
            let cat_slug = op_category(op, &path);
            let filename = format!(
                "{}-{}.md",
                method.as_str().to_lowercase(),
                path_to_slug(&path)
            );
            let summary = op.summary.as_deref().unwrap_or(path.as_str()).to_string();
            let content = render_endpoint(&path, method.as_str(), op, spec, &multi_use);
            by_category
                .entry(cat_slug)
                .or_default()
                .push((filename, summary, content));
        }

        let mut sorted_cats: Vec<&String> = by_category.keys().collect();
        sorted_cats.sort();

        let top_index: String = sorted_cats
            .iter()
            .map(|slug| format!("- [{}](./{slug}/index.md)\n", category_label(slug)))
            .collect();
        let write_path = (dir.join("endpoints").join("index.md"), top_index);
        info!("Writing {:?}", write_path.0);
        writes.push(write_path);

        for (cat_slug, entries) in &by_category {
            push_category_writes(cat_slug, entries, dir, writes);
        }
    }
}

fn push_category_writes(
    cat_slug: &str,
    entries: &[(String, String, String)],
    dir: &Path,
    writes: &mut Vec<(PathBuf, String)>,
) {
    let cat_dir = dir.join("endpoints").join(cat_slug);

    let index = entries
        .iter()
        .map(|(filename, summary, _)| format!("- [{summary}](./{filename})"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let write_path = (cat_dir.join("index.md"), index);
    info!("Writing {:?}", write_path.0);
    writes.push(write_path);

    for (filename, _, content) in entries {
        let write_path = (cat_dir.join(filename), content.clone());
        info!("Writing {:?}", write_path.0);
        writes.push(write_path);
    }
}
