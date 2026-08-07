use oas3::spec::{ObjectOrReference, Schema};

use super::naming::camel_to_kebab;

const SCHEMA_REF_PREFIX: &str = "#/components/schemas/";

/// The component schema name a `$ref` points at, or `None` when the schema is inline
/// or references something outside `#/components/schemas`.
pub(crate) fn schema_ref_name(schema: &Schema) -> Option<&str> {
    let Schema::Object(oor) = schema else {
        return None;
    };
    let ObjectOrReference::Ref { ref_path, .. } = oor.as_ref() else {
        return None;
    };
    ref_path.strip_prefix(SCHEMA_REF_PREFIX)
}

/// The bare name to show for a `$ref` path, falling back to the whole path when it
/// doesn't point into `#/components/schemas`.
pub(crate) fn ref_display_name(ref_path: &str) -> &str {
    ref_path.strip_prefix(SCHEMA_REF_PREFIX).unwrap_or(ref_path)
}

/// Link from an endpoint file to a schema file.
pub(crate) fn schema_doc_link(ref_name: &str) -> String {
    format!("../../schemas/{}.md", camel_to_kebab(ref_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema(value: serde_json::Value) -> Schema {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn ref_name_of_component_schema() {
        let s = schema(serde_json::json!({"$ref": "#/components/schemas/Pet"}));
        assert_eq!(schema_ref_name(&s), Some("Pet"));
    }

    #[test]
    fn ref_name_none_for_inline_schema() {
        let s = schema(serde_json::json!({"type": "string"}));
        assert_eq!(schema_ref_name(&s), None);
    }

    #[test]
    fn ref_name_none_for_non_schema_ref() {
        let s = schema(serde_json::json!({"$ref": "#/components/responses/NotFound"}));
        assert_eq!(schema_ref_name(&s), None);
    }

    #[test]
    fn display_name_strips_known_prefix() {
        assert_eq!(ref_display_name("#/components/schemas/Pet"), "Pet");
    }

    #[test]
    fn display_name_keeps_unknown_path() {
        assert_eq!(
            ref_display_name("./external.json#/Pet"),
            "./external.json#/Pet"
        );
    }

    #[test]
    fn doc_link_uses_kebab_filename() {
        assert_eq!(
            schema_doc_link("AddDataSourceCommand"),
            "../../schemas/add-data-source-command.md"
        );
    }
}
