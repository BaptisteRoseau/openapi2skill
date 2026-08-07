use oas3::{
    OpenApiV3Spec,
    spec::{ObjectSchema, Operation},
};
use serde_json::{Value, json};

/// A minimal valid spec with `overrides` merged over its top-level fields.
pub(crate) fn spec_from(overrides: Value) -> OpenApiV3Spec {
    let mut doc = json!({
        "openapi": "3.1.0",
        "info": {"title": "Test API", "version": "1.0.0"},
        "paths": {},
    });
    if let (Some(target), Some(extra)) = (doc.as_object_mut(), overrides.as_object()) {
        for (key, value) in extra {
            target.insert(key.clone(), value.clone());
        }
    }
    oas3::from_json(doc.to_string()).expect("valid test spec")
}

pub(crate) fn empty_spec() -> OpenApiV3Spec {
    spec_from(json!({}))
}

pub(crate) fn spec_with_servers(urls: &[&str]) -> OpenApiV3Spec {
    let servers: Vec<Value> = urls.iter().map(|url| json!({"url": url})).collect();
    spec_from(json!({ "servers": servers }))
}

pub(crate) fn spec_with_paths(paths: Value) -> OpenApiV3Spec {
    spec_from(json!({ "paths": paths }))
}

pub(crate) fn spec_with_schemas(schemas: Value) -> OpenApiV3Spec {
    spec_from(json!({"components": {"schemas": schemas}}))
}

/// The spec's first operation as `(path, method, operation)`.
pub(crate) fn first_operation(spec: &OpenApiV3Spec) -> (String, String, Operation) {
    let (path, method, op) = spec
        .operations()
        .next()
        .expect("spec should declare at least one operation");
    (path, method.as_str().to_string(), op.clone())
}

pub(crate) fn object_schema(value: Value) -> ObjectSchema {
    serde_json::from_value(value).expect("valid object schema")
}
