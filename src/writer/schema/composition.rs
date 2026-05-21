use oas3::spec::{ObjectOrReference, ObjectSchema, Schema};
use tracing::warn;

use super::context::RenderCtx;

/// Returns an effective [`ObjectSchema`] with every `allOf` subschema merged in.
///
/// Properties, `required`, and `schema_type` are inherited from subschemas when
/// the outer schema does not already define them. The merge is recursive so a
/// subschema's own `allOf` is also expanded. Cyclic `$ref`s in `allOf` are
/// detected via [`RenderCtx::visiting`].
pub(super) fn merge_all_of(obj: &ObjectSchema, ctx: &mut RenderCtx<'_>) -> ObjectSchema {
    if obj.all_of.is_empty() {
        return obj.clone();
    }
    let mut merged = obj.clone();
    merged.all_of = Vec::new();
    for sub in &obj.all_of {
        merge_subschema_into(&mut merged, sub, ctx);
    }
    merged
}

fn merge_subschema_into(merged: &mut ObjectSchema, sub: &Schema, ctx: &mut RenderCtx<'_>) {
    let pushed = subschema_ref_name(sub).map(str::to_string);
    if let Some(name) = &pushed {
        if ctx.visiting.contains(name) {
            return;
        }
        ctx.visiting.insert(name.clone());
    }

    match sub.resolve(ctx.spec) {
        Ok(Schema::Object(oor)) => match oor.as_ref() {
            ObjectOrReference::Object(sub_obj) => {
                let sub_merged = merge_all_of(sub_obj, ctx);
                for (key, schema) in sub_merged.properties.iter() {
                    if !merged.properties.contains_key(key) {
                        merged.properties.insert(key.clone(), schema.clone());
                    }
                }
                for req in &sub_merged.required {
                    if !merged.required.contains(req) {
                        merged.required.push(req.clone());
                    }
                }
                if merged.schema_type.is_none() {
                    merged.schema_type = sub_merged.schema_type.clone();
                }
            }
            ObjectOrReference::Ref { ref_path, .. } => {
                warn!(
                    ref_path = %ref_path,
                    "allOf subschema resolved to an unresolved $ref; skipping its properties"
                );
            }
        },
        Ok(Schema::Boolean(_)) => {
            warn!("allOf subschema is a boolean schema; skipping (no properties to merge)");
        }
        Err(err) => {
            warn!(
                ref_path = ?pushed,
                "could not resolve allOf subschema: {err}; skipping its properties"
            );
        }
    }

    if let Some(name) = pushed {
        ctx.visiting.remove(&name);
    }
}

fn subschema_ref_name(schema: &Schema) -> Option<&str> {
    let Schema::Object(oor) = schema else {
        return None;
    };
    let ObjectOrReference::Ref { ref_path, .. } = oor.as_ref() else {
        return None;
    };
    ref_path.strip_prefix("#/components/schemas/")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use oas3::OpenApiV3Spec;

    use super::*;

    fn spec_with_components(components_json: serde_json::Value) -> OpenApiV3Spec {
        let spec = serde_json::json!({
            "openapi": "3.0.0",
            "info": {"title": "t", "version": "1"},
            "paths": {},
            "components": components_json,
        });
        oas3::from_json(spec.to_string()).expect("valid spec")
    }

    fn merge<'a>(obj: &ObjectSchema, spec: &'a OpenApiV3Spec) -> ObjectSchema {
        let multi_use: HashSet<String> = HashSet::new();
        // Leak so we keep the borrow alive for the test lifetime.
        let multi_use: &'a HashSet<String> = Box::leak(Box::new(multi_use));
        let mut ctx = RenderCtx {
            spec,
            multi_use,
            visiting: HashSet::new(),
        };
        merge_all_of(obj, &mut ctx)
    }

    #[test]
    fn passthrough_when_no_all_of() {
        let spec = spec_with_components(serde_json::json!({"schemas": {}}));
        let obj: ObjectSchema = serde_json::from_value(serde_json::json!({
            "type": "object",
            "properties": {"name": {"type": "string"}},
            "required": ["name"],
        }))
        .unwrap();
        let merged = merge(&obj, &spec);
        assert!(merged.properties.contains_key("name"));
        assert_eq!(merged.required, vec!["name".to_string()]);
    }

    #[test]
    fn merges_properties_from_inline_all_of() {
        let spec = spec_with_components(serde_json::json!({"schemas": {}}));
        let obj: ObjectSchema = serde_json::from_value(serde_json::json!({
            "type": "object",
            "allOf": [
                {"type": "object", "properties": {"a": {"type": "string"}}, "required": ["a"]},
                {"type": "object", "properties": {"b": {"type": "integer"}}},
            ],
        }))
        .unwrap();
        let merged = merge(&obj, &spec);
        assert!(merged.properties.contains_key("a"));
        assert!(merged.properties.contains_key("b"));
        assert_eq!(merged.required, vec!["a".to_string()]);
    }

    #[test]
    fn merges_properties_from_referenced_all_of() {
        let spec = spec_with_components(serde_json::json!({
            "schemas": {
                "Base": {
                    "type": "object",
                    "properties": {"name": {"type": "string"}},
                    "required": ["name"],
                },
            },
        }));
        let obj: ObjectSchema = serde_json::from_value(serde_json::json!({
            "type": "object",
            "allOf": [
                {"$ref": "#/components/schemas/Base"},
                {"type": "object", "properties": {"kind": {"type": "string"}}},
            ],
        }))
        .unwrap();
        let merged = merge(&obj, &spec);
        assert!(merged.properties.contains_key("name"));
        assert!(merged.properties.contains_key("kind"));
        assert!(merged.required.contains(&"name".to_string()));
    }

    #[test]
    fn outer_properties_take_precedence() {
        let spec = spec_with_components(serde_json::json!({"schemas": {}}));
        let obj: ObjectSchema = serde_json::from_value(serde_json::json!({
            "type": "object",
            "properties": {"x": {"type": "string", "description": "outer"}},
            "allOf": [
                {"type": "object", "properties": {"x": {"type": "integer", "description": "inner"}}},
            ],
        }))
        .unwrap();
        let merged = merge(&obj, &spec);
        let x = merged.properties.get("x").unwrap();
        // Outer "string" wins over inner "integer".
        let resolved = x.resolve(&spec).unwrap();
        if let Schema::Object(oor) = resolved
            && let ObjectOrReference::Object(o) = oor.as_ref()
        {
            assert_eq!(o.description.as_deref(), Some("outer"));
        } else {
            panic!("expected object schema");
        }
    }

    #[test]
    fn handles_cyclic_all_of_refs() {
        // Self-referential allOf should not recurse infinitely.
        let spec = spec_with_components(serde_json::json!({
            "schemas": {
                "Loop": {
                    "type": "object",
                    "allOf": [{"$ref": "#/components/schemas/Loop"}],
                    "properties": {"x": {"type": "string"}},
                },
            },
        }));
        let schema = spec
            .components
            .as_ref()
            .unwrap()
            .schemas
            .get("Loop")
            .unwrap();
        let Schema::Object(oor) = schema else {
            panic!("expected object schema");
        };
        let ObjectOrReference::Object(obj) = oor.as_ref() else {
            panic!("expected inline object");
        };
        let merged = merge(obj, &spec);
        assert!(merged.properties.contains_key("x"));
    }
}
