use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, Schema},
};
use tracing::info;

use super::render::render_schema_jsonc;
use crate::writer::utils::{CollectWrites, build_index, camel_to_kebab};

pub(in crate::writer) struct Writer;

impl CollectWrites for Writer {
    fn collect_writes(
        &self,
        spec: &OpenApiV3Spec,
        dir: &Path,
        writes: &mut Vec<(PathBuf, String)>,
    ) {
        let Some(components) = &spec.components else {
            return;
        };
        if components.schemas.is_empty() {
            return;
        }

        let schema_dir = dir.join("schemas");
        let mut index_links: Vec<(String, String)> = Vec::new();

        for (name, schema) in &components.schemas {
            let filename = format!("{}.md", camel_to_kebab(name));
            let content = render_schema_file(name, schema, spec);
            let write_path = (schema_dir.join(&filename), content);
            info!("Writing {:?}", write_path.0);
            writes.push(write_path);
            index_links.push((filename, name.clone()));
        }

        let write_path = (schema_dir.join("index.md"), build_index(&index_links));
        info!("Writing {:?}", write_path.0);
        writes.push(write_path);
    }
}

fn render_schema_file(name: &str, schema: &Schema, spec: &OpenApiV3Spec) -> String {
    let description = schema_description(schema, spec);
    let mut out = format!("# {name}\n\n");
    if let Some(desc) = description {
        out.push_str(&format!("{desc}\n\n"));
    }
    out.push_str("```jsonc\n");
    out.push_str(&render_schema_jsonc(schema, spec, &HashSet::new()));
    out.push('\n');
    out.push_str("```\n");
    out
}

fn schema_description(schema: &Schema, spec: &OpenApiV3Spec) -> Option<String> {
    match schema.resolve(spec) {
        Ok(Schema::Object(oor)) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => obj.description.clone(),
            _ => None,
        },
        _ => None,
    }
}
