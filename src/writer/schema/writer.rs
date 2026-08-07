use std::{collections::HashSet, path::Path};

use oas3::{
    Map, OpenApiV3Spec,
    spec::{ObjectOrReference, Schema},
};
use serde_json::Value;

use super::render::render_schema_jsonc;
use crate::writer::{
    extensions::render_extensions_table,
    utils::{CollectWrites, Writes, build_index, camel_to_kebab, desc_paragraph},
};

pub(in crate::writer) struct Writer;

impl CollectWrites for Writer {
    fn collect_writes(&self, spec: &OpenApiV3Spec, dir: &Path, writes: &mut Writes) {
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
            writes.push(
                schema_dir.join(&filename),
                render_schema_file(name, schema, spec),
            );
            index_links.push((filename, name.clone()));
        }

        writes.push(schema_dir.join("index.md"), build_index(&index_links));
    }
}

fn render_schema_file(name: &str, schema: &Schema, spec: &OpenApiV3Spec) -> String {
    let mut out = format!("# {name}\n\n");
    out.push_str(&desc_paragraph(schema_description(schema, spec).as_deref()));
    out.push_str(&render_extensions_table(schema_extensions(schema)));
    out.push_str(&format!(
        "```jsonc\n{}\n```\n",
        render_schema_jsonc(schema, spec, &HashSet::new())
    ));
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

fn schema_extensions(schema: &Schema) -> &Map<String, Value> {
    static EMPTY: std::sync::OnceLock<Map<String, Value>> = std::sync::OnceLock::new();
    match schema {
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Object(obj) => &obj.extensions,
            _ => EMPTY.get_or_init(Map::new),
        },
        _ => EMPTY.get_or_init(Map::new),
    }
}
