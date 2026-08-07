use std::{collections::BTreeMap, path::Path};

use oas3::OpenApiV3Spec;

use super::{refs::collect_multi_use_schemas, render::render_endpoint};
use crate::writer::utils::{
    CollectWrites, Writes, category_label, effective_server_bases, endpoint_filename, op_category,
};

pub(in crate::writer) struct Writer {
    pub(in crate::writer) servers_override: Vec<String>,
}

struct Endpoint {
    filename: String,
    summary: String,
    content: String,
    deprecated: bool,
}

impl Endpoint {
    fn index_entry(&self) -> String {
        if self.deprecated {
            format!(
                "- ~~[{}](./{})~~ *(deprecated)*",
                self.summary, self.filename
            )
        } else {
            format!("- [{}](./{})", self.summary, self.filename)
        }
    }
}

impl CollectWrites for Writer {
    fn collect_writes(&self, spec: &OpenApiV3Spec, dir: &Path, writes: &mut Writes) {
        let endpoints_dir = dir.join("endpoints");
        let by_category = self.group_by_category(spec);

        let top_index: String = by_category
            .keys()
            .map(|slug| format!("- [{}](./{slug}/index.md)\n", category_label(slug)))
            .collect();
        writes.push(endpoints_dir.join("index.md"), top_index);

        for (cat_slug, endpoints) in by_category {
            push_category_writes(&endpoints_dir.join(cat_slug), &endpoints, writes);
        }
    }
}

impl Writer {
    /// Groups every operation under its category slug, sorted so the top-level index is stable.
    fn group_by_category(&self, spec: &OpenApiV3Spec) -> BTreeMap<String, Vec<Endpoint>> {
        let multi_use = collect_multi_use_schemas(spec);
        let servers = effective_server_bases(spec, &self.servers_override);
        let mut by_category: BTreeMap<String, Vec<Endpoint>> = BTreeMap::new();

        for (path, method, op) in spec.operations() {
            by_category
                .entry(op_category(op, &path))
                .or_default()
                .push(Endpoint {
                    filename: endpoint_filename(method.as_str(), &path),
                    summary: op.summary.as_deref().unwrap_or(path.as_str()).to_string(),
                    content: render_endpoint(
                        &path,
                        method.as_str(),
                        op,
                        spec,
                        &multi_use,
                        &servers,
                    ),
                    deprecated: op.deprecated == Some(true),
                });
        }
        by_category
    }
}

fn push_category_writes(cat_dir: &Path, endpoints: &[Endpoint], writes: &mut Writes) {
    let index = endpoints
        .iter()
        .map(Endpoint::index_entry)
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    writes.push(cat_dir.join("index.md"), index);

    for endpoint in endpoints {
        writes.push(cat_dir.join(&endpoint.filename), endpoint.content.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn endpoint(summary: &str, deprecated: bool) -> Endpoint {
        Endpoint {
            filename: "get-pet.md".to_string(),
            summary: summary.to_string(),
            content: String::new(),
            deprecated,
        }
    }

    #[test]
    fn index_entry_plain() {
        assert_eq!(
            endpoint("Find a pet", false).index_entry(),
            "- [Find a pet](./get-pet.md)"
        );
    }

    #[test]
    fn index_entry_strikes_through_deprecated() {
        assert_eq!(
            endpoint("Find a pet", true).index_entry(),
            "- ~~[Find a pet](./get-pet.md)~~ *(deprecated)*"
        );
    }
}
