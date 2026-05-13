use std::path::{Path, PathBuf};

use oas3::OpenApiV3Spec;
use tracing::info;

use super::utils::{CollectWrites, category_label, op_category};

pub(super) struct Writer;

impl CollectWrites for Writer {
    fn collect_writes(
        &self,
        spec: &OpenApiV3Spec,
        dir: &Path,
        writes: &mut Vec<(PathBuf, String)>,
    ) {
        let write_path = (dir.join("SKILL.md"), render(spec));
        info!("Writing {:?}", write_path);
        writes.push(write_path);
    }
}

fn render(spec: &OpenApiV3Spec) -> String {
    let title = &spec.info.title;
    let description = spec.info.description.as_deref().unwrap_or("");

    let mut categories: Vec<(String, String)> = Vec::new();
    for (path, _, op) in spec.operations() {
        let slug = op_category(op, &path);
        if !categories.iter().any(|(s, _)| s == &slug) {
            let desc = category_label(&slug);
            categories.push((slug, desc));
        }
    }

    let has_auth = spec
        .components
        .as_ref()
        .map(|c| !c.security_schemes.is_empty())
        .unwrap_or(false);

    let has_schemas = spec
        .components
        .as_ref()
        .map(|c| !c.schemas.is_empty())
        .unwrap_or(false);

    let mut out = format!(
        "---\nname: {title}\ndescription: {description}\nallowed-tools:\n  - Read\n  - Bash(ls *)\n  - Bash(grep *)\n  - Bash(find *)\n---\n\n# {title} Documentation\n\n"
    );
    out.push_str(&render_metadata(spec));
    out.push_str(&render_index(has_auth, has_schemas, &categories));
    out
}

fn render_metadata(spec: &OpenApiV3Spec) -> String {
    let mut out = format!("**Version:** {}", spec.info.version);

    if let Some(license) = &spec.info.license {
        if let Some(url) = &license.url {
            out.push_str(&format!(" | **License:** [{}]({})", license.name, url));
        } else {
            out.push_str(&format!(" | **License:** {}", license.name));
        }
    }
    if let Some(tos) = &spec.info.terms_of_service {
        out.push_str(&format!(" | **Terms of Service:** {tos}"));
    }
    out.push_str("\n\n");

    if !spec.servers.is_empty() {
        out.push_str("**Servers:**\n");
        for server in &spec.servers {
            if let Some(desc) = &server.description {
                out.push_str(&format!("- {} — {}\n", server.url, desc));
            } else {
                out.push_str(&format!("- {}\n", server.url));
            }
        }
        out.push('\n');
    }

    if let Some(ext) = &spec.external_docs {
        if let Some(desc) = &ext.description {
            out.push_str(&format!("**External Docs:** [{desc}]({})\n\n", ext.url));
        } else {
            out.push_str(&format!("**External Docs:** {}\n\n", ext.url));
        }
    }

    out
}

fn render_index(has_auth: bool, has_schemas: bool, categories: &[(String, String)]) -> String {
    let mut out = "Read the following files depending on your current needs:\n\n".to_string();
    if has_auth {
        out.push_str(
            "- [authentication/index.md](./authentication/index.md): Authentication workflows\n",
        );
    }
    if !categories.is_empty() {
        out.push_str("- [endpoints/index.md](./endpoints/index.md): API endpoints\n");
    }
    if has_schemas {
        out.push_str("- [schemas/index.md](./schemas/index.md): Data schemas, only if you need them alone. They are already included in endpoints.\n");
    }
    out
}
