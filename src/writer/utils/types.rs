use oas3::spec::{SchemaType, SchemaTypeSet};

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

pub(crate) fn bare_type_name(t: SchemaType) -> &'static str {
    match t {
        SchemaType::Integer => "integer",
        SchemaType::Number => "number",
        SchemaType::Boolean => "boolean",
        SchemaType::String => "string",
        SchemaType::Array => "array",
        SchemaType::Object => "object",
        SchemaType::Null => "null",
    }
}

/// Renders a type set as a display label. `inlines_format` decides which single types absorb
/// `fmt` into the label (e.g. `integer (int64)`); callers render the format separately for the
/// rest. Multi-type sets never inline the format.
pub(crate) fn type_label(
    ts: Option<&SchemaTypeSet>,
    fmt: Option<&str>,
    inlines_format: fn(SchemaType) -> bool,
) -> String {
    match ts {
        None => "any".to_string(),
        Some(SchemaTypeSet::Single(t)) => match fmt {
            Some(f) if inlines_format(*t) => format!("{} ({f})", bare_type_name(*t)),
            _ => bare_type_name(*t).to_string(),
        },
        Some(SchemaTypeSet::Multiple(types)) => {
            let inner: Vec<&str> = types.iter().copied().map(bare_type_name).collect();
            format!("array[{}]", inner.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single(t: SchemaType) -> SchemaTypeSet {
        SchemaTypeSet::Single(t)
    }

    fn multi(types: Vec<SchemaType>) -> SchemaTypeSet {
        SchemaTypeSet::Multiple(types)
    }

    fn integer_only(t: SchemaType) -> bool {
        t == SchemaType::Integer
    }

    fn never(_: SchemaType) -> bool {
        false
    }

    // --- primary_type ---

    #[test]
    fn primary_type_single() {
        assert_eq!(
            primary_type(&single(SchemaType::Integer)),
            SchemaType::Integer
        );
    }

    #[test]
    fn primary_type_multiple_picks_non_null() {
        assert_eq!(
            primary_type(&multi(vec![SchemaType::Null, SchemaType::String])),
            SchemaType::String
        );
    }

    #[test]
    fn primary_type_all_null_falls_back_to_object() {
        assert_eq!(
            primary_type(&multi(vec![SchemaType::Null])),
            SchemaType::Object
        );
    }

    // --- bare_type_name ---

    #[test]
    fn bare_names_cover_every_type() {
        assert_eq!(bare_type_name(SchemaType::Integer), "integer");
        assert_eq!(bare_type_name(SchemaType::Number), "number");
        assert_eq!(bare_type_name(SchemaType::Boolean), "boolean");
        assert_eq!(bare_type_name(SchemaType::String), "string");
        assert_eq!(bare_type_name(SchemaType::Array), "array");
        assert_eq!(bare_type_name(SchemaType::Object), "object");
        assert_eq!(bare_type_name(SchemaType::Null), "null");
    }

    // --- type_label ---

    #[test]
    fn label_none_is_any() {
        assert_eq!(type_label(None, None, never), "any");
    }

    #[test]
    fn label_single_without_format() {
        assert_eq!(
            type_label(Some(&single(SchemaType::String)), None, never),
            "string"
        );
    }

    #[test]
    fn label_inlines_format_when_predicate_allows() {
        assert_eq!(
            type_label(
                Some(&single(SchemaType::Integer)),
                Some("int64"),
                integer_only
            ),
            "integer (int64)"
        );
    }

    #[test]
    fn label_omits_format_when_predicate_declines() {
        assert_eq!(
            type_label(
                Some(&single(SchemaType::String)),
                Some("date-time"),
                integer_only
            ),
            "string"
        );
    }

    #[test]
    fn label_multi_never_inlines_format() {
        assert_eq!(
            type_label(
                Some(&multi(vec![SchemaType::String, SchemaType::Null])),
                Some("date-time"),
                integer_only
            ),
            "array[string, null]"
        );
    }

    #[test]
    fn label_multi_preserves_order() {
        assert_eq!(
            type_label(
                Some(&multi(vec![SchemaType::Null, SchemaType::Integer])),
                None,
                never
            ),
            "array[null, integer]"
        );
    }
}
