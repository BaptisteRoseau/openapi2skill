use oas3::{
    Map, OpenApiV3Spec,
    spec::{Header, ObjectOrReference},
};
use tracing::warn;

use super::params::render_param_type;

pub(super) fn render_response_headers_table(
    headers: &Map<String, ObjectOrReference<Header>>,
    spec: &OpenApiV3Spec,
) -> String {
    if headers.is_empty() {
        return String::new();
    }
    let mut out =
        "### Response Headers\n\n| Header | Type | Description |\n|--------|------|-------------|\n"
            .to_string();
    for (name, header_ref) in headers {
        let header = match header_ref.resolve(spec) {
            Ok(h) => h,
            Err(err) => {
                warn!(header = %name, "could not resolve response header: {err}; skipping");
                continue;
            }
        };
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            name,
            render_param_type(&header.schema, spec),
            header.description.as_deref().unwrap_or("-"),
        ));
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use oas3::spec::{Header, ObjectOrReference};

    use super::*;

    fn empty_spec() -> OpenApiV3Spec {
        oas3::from_json(r#"{"openapi":"3.1.0","info":{"title":"Test","version":"1.0"},"paths":{}}"#)
            .unwrap()
    }

    #[test]
    fn empty_headers_returns_empty_string() {
        let headers: Map<String, ObjectOrReference<Header>> = Map::new();
        let spec = empty_spec();
        assert_eq!(render_response_headers_table(&headers, &spec), "");
    }

    #[test]
    fn single_header_with_schema_renders_table() {
        let header: Header = serde_json::from_value(serde_json::json!({
            "description": "Requests per minute allowed",
            "schema": {"type": "integer"}
        }))
        .unwrap();
        let mut headers: Map<String, ObjectOrReference<Header>> = Map::new();
        headers.insert(
            "X-Rate-Limit".to_string(),
            ObjectOrReference::Object(header),
        );
        let spec = empty_spec();
        let result = render_response_headers_table(&headers, &spec);
        assert!(result.contains("### Response Headers"));
        assert!(result.contains("| `X-Rate-Limit` | integer | Requests per minute allowed |"));
    }

    #[test]
    fn header_with_no_description_shows_dash() {
        let header: Header =
            serde_json::from_value(serde_json::json!({"schema": {"type": "integer"}})).unwrap();
        let mut headers: Map<String, ObjectOrReference<Header>> = Map::new();
        headers.insert("Retry-After".to_string(), ObjectOrReference::Object(header));
        let spec = empty_spec();
        let result = render_response_headers_table(&headers, &spec);
        assert!(result.contains("| `Retry-After` | integer | - |"));
    }

    #[test]
    fn header_with_no_schema_shows_string_fallback() {
        let header: Header =
            serde_json::from_value(serde_json::json!({"description": "Some header"})).unwrap();
        let mut headers: Map<String, ObjectOrReference<Header>> = Map::new();
        headers.insert("X-Custom".to_string(), ObjectOrReference::Object(header));
        let spec = empty_spec();
        let result = render_response_headers_table(&headers, &spec);
        assert!(result.contains("| `X-Custom` | string | Some header |"));
    }
}
