use std::path::{Path, PathBuf};

use oas3::{
    OpenApiV3Spec,
    spec::{Operation, SchemaType, SchemaTypeSet},
};

pub(crate) trait CollectWrites {
    fn collect_writes(&self, spec: &OpenApiV3Spec, dir: &Path, writes: &mut Vec<(PathBuf, String)>);
}

pub(crate) fn op_category(op: &Operation, path: &str) -> String {
    if let Some(tag) = op.tags.first() {
        return to_snake_case(tag);
    }
    path.split('/')
        .filter(|s| !s.is_empty())
        .find(|s| {
            !(s.starts_with('v') && s.len() > 1 && s[1..].chars().all(|c| c.is_ascii_digit()))
        })
        .map(to_snake_case)
        .unwrap_or_else(|| "general".to_string())
}

pub(crate) fn category_label(slug: &str) -> String {
    let name = slug.replace('_', " ");
    let mut chars = name.chars();
    let capitalized = match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    };
    format!("{capitalized} endpoints")
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

pub(crate) fn primary_type(ts: &SchemaTypeSet) -> SchemaType {
    match ts {
        SchemaTypeSet::Single(t) => *t,
        SchemaTypeSet::Multiple(types) => types
            .iter()
            .copied()
            .find(|t| *t != SchemaType::Null)
            .unwrap_or(SchemaType::Object),
    }
}

pub(crate) fn build_index(links: &[(String, String)]) -> String {
    links
        .iter()
        .map(|(file, name)| format!("- [{name}](./{file})"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}
