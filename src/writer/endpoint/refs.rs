use std::collections::{HashMap, HashSet};

use oas3::{
    OpenApiV3Spec,
    spec::{ObjectOrReference, Operation, Response, Schema},
};

pub(super) fn collect_multi_use_schemas(spec: &OpenApiV3Spec) -> HashSet<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for (_, _, op) in spec.operations() {
        for name in collect_op_refs(op, spec) {
            *counts.entry(name).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .map(|(name, _)| name)
        .collect()
}

fn collect_op_refs(op: &Operation, spec: &OpenApiV3Spec) -> HashSet<String> {
    collect_body_refs(op, spec)
        .into_iter()
        .chain(collect_response_refs(op, spec))
        .collect()
}

fn collect_body_refs(op: &Operation, spec: &OpenApiV3Spec) -> Vec<String> {
    let Some(body_ref) = &op.request_body else {
        return Vec::new();
    };
    let Ok(body) = body_ref.resolve(spec) else {
        return Vec::new();
    };
    body.content
        .values()
        .filter_map(|mt| mt.schema.as_ref().and_then(top_level_ref_name))
        .map(str::to_string)
        .collect()
}

fn collect_response_refs(op: &Operation, spec: &OpenApiV3Spec) -> Vec<String> {
    let Some(responses) = &op.responses else {
        return Vec::new();
    };
    responses
        .values()
        .filter_map(|resp_ref| resolve_response(resp_ref, spec))
        .flat_map(|resp| {
            resp.content
                .values()
                .filter_map(|mt| {
                    mt.schema
                        .as_ref()
                        .and_then(top_level_ref_name)
                        .map(str::to_string)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub(super) fn resolve_response(
    resp_ref: &ObjectOrReference<Response>,
    spec: &OpenApiV3Spec,
) -> Option<Response> {
    match resp_ref {
        ObjectOrReference::Object(r) => Some(r.clone()),
        ObjectOrReference::Ref { .. } => resp_ref.resolve(spec).ok(),
    }
}

pub(super) fn top_level_ref_name(schema: &Schema) -> Option<&str> {
    match schema {
        Schema::Object(oor) => match oor.as_ref() {
            ObjectOrReference::Ref { ref_path, .. } => {
                ref_path.strip_prefix("#/components/schemas/")
            }
            _ => None,
        },
        _ => None,
    }
}
