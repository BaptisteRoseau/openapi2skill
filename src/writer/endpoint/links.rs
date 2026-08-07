use oas3::{
    Map, OpenApiV3Spec,
    spec::{Link, ObjectOrReference},
};
use tracing::warn;

use crate::writer::utils::{Table, desc_cell};

pub(super) fn render_response_links_table(
    links: &Map<String, ObjectOrReference<Link>>,
    spec: &OpenApiV3Spec,
) -> String {
    if links.is_empty() {
        return String::new();
    }
    let mut table = Table::new(&["Link", "Operation", "Parameters", "Description"]);
    for (name, link_ref) in links {
        let Some(link) = resolve_link(link_ref, spec) else {
            warn!(link = %name, "could not resolve response link; skipping");
            continue;
        };
        table.row(&link_row_cells(name, &link));
    }
    format!("### Links\n\n{}", table.finish())
}

fn resolve_link(link_ref: &ObjectOrReference<Link>, spec: &OpenApiV3Spec) -> Option<Link> {
    match link_ref {
        ObjectOrReference::Object(link) => Some(link.clone()),
        ObjectOrReference::Ref { ref_path, .. } => {
            let name = ref_path.strip_prefix("#/components/links/")?;
            let component_ref = spec.components.as_ref()?.links.get(name)?;
            match component_ref {
                ObjectOrReference::Object(link) => Some(link.clone()),
                ObjectOrReference::Ref { .. } => None,
            }
        }
    }
}

fn link_row_cells(name: &str, link: &Link) -> [String; 4] {
    let (operation, parameters, description) = match link {
        Link::Id {
            operation_id,
            parameters,
            description,
            ..
        } => (operation_id.as_str(), parameters, description.as_deref()),
        Link::Ref {
            operation_ref,
            parameters,
            description,
            ..
        } => (operation_ref.as_str(), parameters, description.as_deref()),
    };
    let params = if parameters.is_empty() {
        "-".to_string()
    } else {
        parameters
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    [
        format!("`{name}`"),
        format!("`{operation}`"),
        params,
        desc_cell(description),
    ]
}

#[cfg(test)]
mod tests {
    use oas3::spec::{Link, ObjectOrReference};

    use super::*;
    use crate::writer::testutil::empty_spec;

    #[test]
    fn empty_links_returns_empty_string() {
        let links: Map<String, ObjectOrReference<Link>> = Map::new();
        let spec = empty_spec();
        assert_eq!(render_response_links_table(&links, &spec), "");
    }

    #[test]
    fn link_id_with_params_and_description_renders_table_row() {
        let link: Link = serde_json::from_value(serde_json::json!({
            "operationId": "getUserByUsername",
            "parameters": {
                "username": "$response.body#/username"
            },
            "description": "Gets user address"
        }))
        .unwrap();
        let mut links: Map<String, ObjectOrReference<Link>> = Map::new();
        links.insert("address".to_string(), ObjectOrReference::Object(link));
        let spec = empty_spec();
        let result = render_response_links_table(&links, &spec);
        assert!(result.contains("### Links"));
        assert!(result.contains("| `address` | `getUserByUsername` | username: $response.body#/username | Gets user address |"));
    }

    #[test]
    fn link_id_with_no_params_shows_dash() {
        let link: Link = serde_json::from_value(serde_json::json!({
            "operationId": "listRepos"
        }))
        .unwrap();
        let mut links: Map<String, ObjectOrReference<Link>> = Map::new();
        links.insert(
            "userRepositories".to_string(),
            ObjectOrReference::Object(link),
        );
        let spec = empty_spec();
        let result = render_response_links_table(&links, &spec);
        assert!(result.contains("| `userRepositories` | `listRepos` | - | - |"));
    }

    #[test]
    fn link_ref_uses_operation_ref_in_operation_column() {
        let link: Link = serde_json::from_value(serde_json::json!({
            "operationRef": "#/paths/~1users~1{username}/get",
            "parameters": {
                "username": "$response.body#/username"
            }
        }))
        .unwrap();
        let mut links: Map<String, ObjectOrReference<Link>> = Map::new();
        links.insert("userRef".to_string(), ObjectOrReference::Object(link));
        let spec = empty_spec();
        let result = render_response_links_table(&links, &spec);
        assert!(result.contains("| `userRef` | `#/paths/~1users~1{username}/get` |"));
    }
}
