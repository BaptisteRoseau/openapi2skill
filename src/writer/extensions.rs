use oas3::Map;
use serde_json::Value;

use crate::writer::utils::Table;

pub(crate) fn render_extensions_table(extensions: &Map<String, Value>) -> String {
    let rows: Vec<(&str, String)> = extensions
        .iter()
        .filter_map(|(k, v)| scalar_display(v).map(|s| (k.as_str(), s)))
        .collect();

    if rows.is_empty() {
        return String::new();
    }

    let mut table = Table::new(&["Extension", "Value"]);
    for (key, val) in rows {
        table.row(&[format!("`{key}`"), format!("`{val}`")]);
    }
    format!("### Extensions\n\n{}", table.finish())
}

fn scalar_display(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::render_extensions_table;

    fn make_map(pairs: &[(&str, Value)]) -> oas3::Map<String, Value> {
        let mut m = oas3::Map::default();
        for (k, v) in pairs {
            m.insert(k.to_string(), v.clone());
        }
        m
    }

    #[test]
    fn only_scalars_renders_table() {
        let m = make_map(&[
            ("x-foo", json!("bar")),
            ("x-count", json!(42)),
            ("x-flag", json!(true)),
        ]);
        let out = render_extensions_table(&m);
        assert!(out.contains("### Extensions"));
        assert!(out.contains("| `x-foo` | `bar` |"));
        assert!(out.contains("| `x-count` | `42` |"));
        assert!(out.contains("| `x-flag` | `true` |"));
    }

    #[test]
    fn mixed_values_only_scalars_appear() {
        let m = make_map(&[
            ("x-scalar", json!("hello")),
            ("x-array", json!(["a", "b"])),
            ("x-object", json!({"key": "val"})),
        ]);
        let out = render_extensions_table(&m);
        assert!(out.contains("| `x-scalar` | `hello` |"));
        assert!(!out.contains("x-array"));
        assert!(!out.contains("x-object"));
    }

    #[test]
    fn empty_map_returns_empty_string() {
        let m = make_map(&[]);
        assert_eq!(render_extensions_table(&m), "");
    }

    #[test]
    fn all_object_or_array_returns_empty_string() {
        let m = make_map(&[
            ("x-arr", json!([1, 2, 3])),
            ("x-obj", json!({"nested": true})),
        ]);
        assert_eq!(render_extensions_table(&m), "");
    }
}
