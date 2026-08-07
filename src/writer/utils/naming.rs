use std::path::Path;

use oas3::spec::Operation;

const MAX_FILENAME_LEN: usize = 200;

/// The category slug an operation's file lands under: its first tag, else the first
/// non-version path segment, else `general`.
pub(crate) fn op_category(op: &Operation, path: &str) -> String {
    if let Some(tag) = op.tags.first() {
        return to_snake_case(tag);
    }
    path.split('/')
        .filter(|s| !s.is_empty())
        .find(|s| !is_version_segment(s))
        .map(to_snake_case)
        .unwrap_or_else(|| "general".to_string())
}

fn is_version_segment(segment: &str) -> bool {
    segment.starts_with('v')
        && segment.len() > 1
        && segment[1..].chars().all(|c| c.is_ascii_digit())
}

pub(crate) fn category_label(slug: &str) -> String {
    format!("{} endpoints", capitalize_first(&slug.replace('_', " ")))
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
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

/// Lowercases kept characters and collapses every other run into a single `separator`.
fn separated_case(s: &str, separator: char, is_kept: fn(char) -> bool) -> String {
    s.chars()
        .map(|c| {
            if is_kept(c) {
                c.to_ascii_lowercase()
            } else {
                separator
            }
        })
        .collect::<String>()
        .split(separator)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(&separator.to_string())
}

pub(crate) fn to_dashed_case(s: &str) -> String {
    separated_case(&strip_accents(s), '-', |c| c.is_ascii_alphanumeric())
}

pub(crate) fn to_snake_case(s: &str) -> String {
    separated_case(s, '_', char::is_alphanumeric)
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

fn path_to_slug(path: &str) -> String {
    path.split('/')
        .filter(|s| !s.is_empty())
        .map(|seg| camel_to_kebab(seg.trim_start_matches('{').trim_end_matches('}')))
        .collect::<Vec<_>>()
        .join("-")
}

/// Builds the `{method}-{slug}.md` filename for an operation, truncating (and disambiguating
/// with a path hash) when it would exceed the filesystem's max filename length.
pub(crate) fn endpoint_filename(method: &str, path: &str) -> String {
    let method = method.to_lowercase();
    let slug = path_to_slug(path);
    let filename = format!("{method}-{slug}.md");
    if filename.len() <= MAX_FILENAME_LEN {
        return filename;
    }

    let hash = format!("{:016x}", hash_str(path));
    let fixed_len = method.len() + "--.md".len() + hash.len();
    let budget = MAX_FILENAME_LEN.saturating_sub(fixed_len);
    let truncated = truncate_at_char_boundary(&slug, budget);
    format!("{method}-{truncated}-{hash}.md")
}

fn truncate_at_char_boundary(s: &str, max_len: usize) -> &str {
    let mut end = max_len.min(s.len());
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn hash_str(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn snake_case_keeps_non_ascii_alphanumerics() {
        assert_eq!(to_snake_case("café"), "café");
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

    // --- endpoint_filename ---

    #[test]
    fn endpoint_filename_lowercases_method() {
        assert_eq!(
            endpoint_filename("GET", "/pet/{petId}"),
            "get-pet-pet-id.md"
        );
    }

    #[test]
    fn endpoint_filename_truncates_and_hashes_long_paths() {
        let long_path = format!("/{}", "segment/".repeat(60));
        let filename = endpoint_filename("GET", &long_path);
        assert!(
            filename.len() <= MAX_FILENAME_LEN,
            "filename too long: {} chars",
            filename.len()
        );
        assert!(filename.starts_with("get-"));
        assert!(filename.ends_with(".md"));
    }

    #[test]
    fn endpoint_filename_hash_disambiguates_distinct_long_paths() {
        let a = endpoint_filename("GET", &format!("/{}a", "segment/".repeat(60)));
        let b = endpoint_filename("GET", &format!("/{}b", "segment/".repeat(60)));
        assert_ne!(a, b);
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

    // --- op_category ---

    fn operation(value: serde_json::Value) -> Operation {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn op_category_prefers_first_tag() {
        let op = operation(serde_json::json!({"tags": ["Pet Store", "other"]}));
        assert_eq!(op_category(&op, "/admin/users"), "pet_store");
    }

    #[test]
    fn op_category_falls_back_to_first_path_segment() {
        let op = operation(serde_json::json!({}));
        assert_eq!(op_category(&op, "/admin/users"), "admin");
    }

    #[test]
    fn op_category_skips_version_segment() {
        let op = operation(serde_json::json!({}));
        assert_eq!(op_category(&op, "/v1/admin/users"), "admin");
    }

    #[test]
    fn op_category_keeps_segment_starting_with_v() {
        let op = operation(serde_json::json!({}));
        assert_eq!(op_category(&op, "/venues/list"), "venues");
    }

    #[test]
    fn op_category_general_when_root_path_and_no_tags() {
        let op = operation(serde_json::json!({}));
        assert_eq!(op_category(&op, "/"), "general");
    }
}
