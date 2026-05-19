//! Shared utilities used by all writers. Produces no output files of its own.
//!
//! - [`CollectWrites`]: trait implemented by each writer to append `(path, content)` pairs.
//! - [`op_category`]: derives the snake_case category slug for an operation (first tag, or first path segment).
//! - [`category_label`]: turns a slug like `admin_users` into `"Admin users endpoints"`.
//! - [`path_to_slug`]: converts `/admin/users/{userId}` to `admin-users-user-id`.
//! - [`camel_to_kebab`]: converts `AddDataSourceCommand` to `add-data-source-command`.
//! - [`to_snake_case`]: converts arbitrary strings to `snake_case` (used for output dir name and category slugs).
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

#[cfg(test)]
mod tests {
    use super::*;
    use oas3::spec::{SchemaType, SchemaTypeSet};

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
