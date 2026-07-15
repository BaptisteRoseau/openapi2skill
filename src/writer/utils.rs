//! Shared utilities used by all writers. Produces no output files of its own.
//!
//! - [`CollectWrites`]: trait implemented by each writer to append `(path, content)` pairs.
//! - [`op_category`]: derives the snake_case category slug for an operation (first tag, or first path segment).
//! - [`category_label`]: turns a slug like `admin_users` into `"Admin users endpoints"`.
//! - [`path_to_slug`]: converts `/admin/users/{userId}` to `admin-users-user-id`.
//! - [`camel_to_kebab`]: converts `AddDataSourceCommand` to `add-data-source-command`.
//! - [`to_snake_case`]: converts arbitrary strings to `snake_case` (used for category slugs).
//! - [`to_dashed_case`]: converts arbitrary strings to `kebab-case` ASCII (used for skill name).
//! - [`infer_skill_name`]: derives the skill name from the output dir or spec title.
//! - [`primary_type`]: extracts the non-null type from a `SchemaTypeSet`.
//! - [`build_index`]: builds a markdown bullet list of `[name](./file.md)` links.

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

pub(crate) fn infer_skill_name(title: &str, output_dir: Option<&Path>) -> String {
    if let Some(dir) = output_dir
        && let Some(name) = dir.file_name()
    {
        return name.to_string_lossy().into_owned();
    }
    format!("api-{}", to_dashed_case(title))
}

fn strip_accents(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => {
                'a'
            }
            'è' | 'é' | 'ê' | 'ë' | 'È' | 'É' | 'Ê' | 'Ë' => 'e',
            'ì' | 'í' | 'î' | 'ï' | 'Ì' | 'Í' | 'Î' | 'Ï' => 'i',
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => {
                'o'
            }
            'ù' | 'ú' | 'û' | 'ü' | 'Ù' | 'Ú' | 'Û' | 'Ü' => 'u',
            'ý' | 'ÿ' | 'Ý' => 'y',
            'ñ' | 'Ñ' => 'n',
            'ç' | 'Ç' => 'c',
            'ß' => 's',
            _ => c,
        })
        .collect()
}

pub(crate) fn to_dashed_case(s: &str) -> String {
    let s = strip_accents(s);
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
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

/// Normalises a description string from a spec for use in markdown output.
///
/// Single `\n` inside descriptions is a soft wrap in the source spec but renders as a visible
/// line break in raw-file readers (AI agents). Replacing every `\n` with a space keeps
/// descriptions single-line and compact. Leading/trailing whitespace is also trimmed.
pub(crate) fn normalize_desc(s: &str) -> String {
    let replaced = s.replace('\n', " ");
    replaced.trim().to_string()
}

/// Returns the base server URLs to use when rendering "Full URL" fields, each trimmed of a
/// trailing slash.
///
/// The CLI `--server` override (`servers_override`) takes precedence over the spec's declared
/// servers so that every endpoint file documents the host the agent will actually call. When no
/// override is given, the spec's own servers are used.
pub(crate) fn effective_server_bases(
    spec: &OpenApiV3Spec,
    servers_override: &[String],
) -> Vec<String> {
    let sources: Vec<String> = if servers_override.is_empty() {
        spec.servers.iter().map(|s| s.url.clone()).collect()
    } else {
        servers_override.to_vec()
    };
    sources
        .iter()
        .map(|url| url.trim_end_matches('/').to_string())
        .collect()
}

pub(crate) fn build_index(links: &[(String, String)]) -> String {
    links
        .iter()
        .map(|(file, name)| format!("- [{name}](./{file})"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;
    use oas3::spec::{SchemaType, SchemaTypeSet};

    // --- normalize_desc ---

    #[test]
    fn normalize_desc_replaces_single_newline_with_space() {
        assert_eq!(normalize_desc("foo\nbar"), "foo bar");
    }

    #[test]
    fn normalize_desc_trims_trailing_newline() {
        assert_eq!(normalize_desc("foo\n"), "foo");
    }

    #[test]
    fn normalize_desc_trims_trailing_whitespace() {
        assert_eq!(normalize_desc("foo   "), "foo");
    }

    #[test]
    fn normalize_desc_double_newline_collapses_to_two_spaces() {
        assert_eq!(normalize_desc("foo\n\nbar"), "foo  bar");
    }

    #[test]
    fn normalize_desc_empty_string_stays_empty() {
        assert_eq!(normalize_desc(""), "");
    }

    #[test]
    fn normalize_desc_no_newlines_passthrough() {
        assert_eq!(normalize_desc("hello world"), "hello world");
    }

    // --- infer_skill_name ---

    #[test]
    fn infer_skill_name_no_output_dir() {
        assert_eq!(
            infer_skill_name("My REST API aïé", None),
            "api-my-rest-api-aie"
        );
    }

    #[test]
    fn infer_skill_name_with_output_dir() {
        assert_eq!(
            infer_skill_name("Swagger Petstore", Some(Path::new("my-custom-dir"))),
            "my-custom-dir"
        );
    }

    #[test]
    fn infer_skill_name_output_dir_nested() {
        assert_eq!(
            infer_skill_name("Swagger Petstore", Some(Path::new("/some/path/my-api"))),
            "my-api"
        );
    }

    // --- to_dashed_case ---

    #[test]
    fn to_dashed_case_simple() {
        assert_eq!(to_dashed_case("Swagger Petstore"), "swagger-petstore");
    }

    #[test]
    fn to_dashed_case_accents() {
        assert_eq!(to_dashed_case("aïé"), "aie");
    }

    #[test]
    fn to_dashed_case_mixed() {
        assert_eq!(to_dashed_case("My REST API aïé"), "my-rest-api-aie");
    }

    #[test]
    fn to_dashed_case_deduplicates_dashes() {
        assert_eq!(to_dashed_case("foo--bar"), "foo-bar");
    }

    // --- to_snake_case ---

    #[test]
    fn snake_case_lowercases() {
        assert_eq!(to_snake_case("PetStore"), "petstore");
    }

    #[test]
    fn snake_case_replaces_spaces_with_underscore() {
        assert_eq!(to_snake_case("foo bar"), "foo_bar");
    }

    #[test]
    fn snake_case_deduplicates_underscores() {
        assert_eq!(to_snake_case("foo--bar"), "foo_bar");
    }

    #[test]
    fn snake_case_trims_leading_and_trailing() {
        assert_eq!(to_snake_case("-foo-"), "foo");
    }

    #[test]
    fn snake_case_empty_string() {
        assert_eq!(to_snake_case(""), "");
    }

    // --- camel_to_kebab ---

    #[test]
    fn camel_to_kebab_pascal_case() {
        assert_eq!(
            camel_to_kebab("AddDataSourceCommand"),
            "add-data-source-command"
        );
    }

    #[test]
    fn camel_to_kebab_already_lowercase_passthrough() {
        assert_eq!(camel_to_kebab("simple"), "simple");
    }

    #[test]
    fn camel_to_kebab_single_uppercase_char() {
        assert_eq!(camel_to_kebab("A"), "a");
    }

    #[test]
    fn camel_to_kebab_consecutive_uppercase() {
        assert_eq!(camel_to_kebab("MyDTO"), "my-d-t-o");
    }

    // --- path_to_slug ---

    #[test]
    fn path_to_slug_basic() {
        assert_eq!(path_to_slug("/pet"), "pet");
    }

    #[test]
    fn path_to_slug_nested() {
        assert_eq!(path_to_slug("/pet/findByStatus"), "pet-find-by-status");
    }

    #[test]
    fn path_to_slug_with_path_param() {
        assert_eq!(path_to_slug("/pet/{petId}"), "pet-pet-id");
    }

    #[test]
    fn path_to_slug_root_is_empty() {
        assert_eq!(path_to_slug("/"), "");
    }

    // --- category_label ---

    #[test]
    fn category_label_single_word() {
        assert_eq!(category_label("pet"), "Pet endpoints");
    }

    #[test]
    fn category_label_underscore_becomes_space() {
        assert_eq!(category_label("admin_users"), "Admin users endpoints");
    }

    #[test]
    fn category_label_empty_slug() {
        assert_eq!(category_label(""), " endpoints");
    }

    // --- build_index ---

    #[test]
    fn build_index_produces_bullet_list() {
        let links = vec![
            ("pet.md".to_string(), "Pet".to_string()),
            ("tag.md".to_string(), "Tag".to_string()),
        ];
        assert_eq!(
            build_index(&links),
            "- [Pet](./pet.md)\n- [Tag](./tag.md)\n"
        );
    }

    #[test]
    fn build_index_empty_is_just_newline() {
        assert_eq!(build_index(&[]), "\n");
    }

    // --- effective_server_bases ---

    fn spec_with_servers(urls: &[&str]) -> OpenApiV3Spec {
        let servers_json = urls
            .iter()
            .map(|u| format!(r#"{{"url":"{u}"}}"#))
            .collect::<Vec<_>>()
            .join(",");
        let json = format!(
            r#"{{"openapi":"3.0.0","info":{{"title":"T","version":"1"}},"servers":[{servers_json}],"paths":{{}}}}"#
        );
        oas3::from_json(json).unwrap()
    }

    #[test]
    fn effective_server_bases_prefers_override() {
        let spec = spec_with_servers(&["http://spec-host:9090"]);
        let overrides = vec!["http://cli-host:9090".to_string()];
        assert_eq!(
            effective_server_bases(&spec, &overrides),
            vec!["http://cli-host:9090".to_string()]
        );
    }

    #[test]
    fn effective_server_bases_keeps_override_list_order() {
        let spec = spec_with_servers(&["http://spec-host:9090"]);
        let overrides = vec![
            "http://first:9090".to_string(),
            "https://second:9090".to_string(),
        ];
        assert_eq!(effective_server_bases(&spec, &overrides), overrides);
    }

    #[test]
    fn effective_server_bases_falls_back_to_spec() {
        let spec = spec_with_servers(&["http://spec-host:9090"]);
        assert_eq!(
            effective_server_bases(&spec, &[]),
            vec!["http://spec-host:9090".to_string()]
        );
    }

    #[test]
    fn effective_server_bases_trims_trailing_slash() {
        let spec = spec_with_servers(&["http://spec-host:9090/"]);
        assert_eq!(
            effective_server_bases(&spec, &[]),
            vec!["http://spec-host:9090".to_string()]
        );
        let overrides = vec!["http://cli-host:9090/api/v1/".to_string()];
        assert_eq!(
            effective_server_bases(&spec, &overrides),
            vec!["http://cli-host:9090/api/v1".to_string()]
        );
    }

    #[test]
    fn effective_server_bases_empty_when_no_servers() {
        let spec = spec_with_servers(&[]);
        assert!(effective_server_bases(&spec, &[]).is_empty());
    }

    // --- primary_type ---

    #[test]
    fn primary_type_single() {
        assert_eq!(
            primary_type(&SchemaTypeSet::Single(SchemaType::Integer)),
            SchemaType::Integer
        );
    }

    #[test]
    fn primary_type_multiple_picks_non_null() {
        assert_eq!(
            primary_type(&SchemaTypeSet::Multiple(vec![
                SchemaType::Null,
                SchemaType::String
            ])),
            SchemaType::String
        );
    }

    #[test]
    fn primary_type_all_null_falls_back_to_object() {
        assert_eq!(
            primary_type(&SchemaTypeSet::Multiple(vec![SchemaType::Null])),
            SchemaType::Object
        );
    }
}
