use oas3::OpenApiV3Spec;

/// Returns the base server URLs for "Full URL" fields, trimmed of trailing slashes.
/// Prefers `servers_override` over the spec's declared servers.
pub(crate) fn effective_server_bases(
    spec: &OpenApiV3Spec,
    servers_override: &[String],
) -> Vec<String> {
    let sources: Box<dyn Iterator<Item = &str>> = if servers_override.is_empty() {
        Box::new(spec.servers.iter().map(|s| s.url.as_str()))
    } else {
        Box::new(servers_override.iter().map(String::as_str))
    };
    sources
        .map(|url| url.trim_end_matches('/').to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writer::testutil::spec_with_servers;

    #[test]
    fn prefers_override() {
        let spec = spec_with_servers(&["http://spec-host:9090"]);
        let overrides = vec!["http://cli-host:9090".to_string()];
        assert_eq!(
            effective_server_bases(&spec, &overrides),
            vec!["http://cli-host:9090".to_string()]
        );
    }

    #[test]
    fn keeps_override_list_order() {
        let spec = spec_with_servers(&["http://spec-host:9090"]);
        let overrides = vec![
            "http://first:9090".to_string(),
            "https://second:9090".to_string(),
        ];
        assert_eq!(effective_server_bases(&spec, &overrides), overrides);
    }

    #[test]
    fn falls_back_to_spec() {
        let spec = spec_with_servers(&["http://spec-host:9090"]);
        assert_eq!(
            effective_server_bases(&spec, &[]),
            vec!["http://spec-host:9090".to_string()]
        );
    }

    #[test]
    fn trims_trailing_slash() {
        let spec = spec_with_servers(&["http://spec-host:9090/"]);
        assert_eq!(
            effective_server_bases(&spec, &[]),
            vec!["http://spec-host:9090".to_string()]
        );
        let overrides = vec!["http://cli-host:9090/api/v1/".to_string()];
        assert_eq!(
            effective_server_bases(&spec, &overrides),
            vec!["http://cli-host:9090/api/v1".to_string()]
        );
    }

    #[test]
    fn empty_when_no_servers() {
        let spec = spec_with_servers(&[]);
        assert!(effective_server_bases(&spec, &[]).is_empty());
    }
}
