use std::path::{Path, PathBuf};

use oas3::OpenApiV3Spec;
use tracing::info;

/// Accumulates the `(path, content)` pairs every writer produces, logging each as it lands.
#[derive(Default)]
pub(crate) struct Writes(Vec<(PathBuf, String)>);

impl Writes {
    pub(crate) fn push(&mut self, path: PathBuf, content: String) {
        info!("Writing {path:?}");
        self.0.push((path, content));
    }

    pub(crate) fn into_vec(self) -> Vec<(PathBuf, String)> {
        self.0
    }
}

pub(crate) trait CollectWrites {
    fn collect_writes(&self, spec: &OpenApiV3Spec, dir: &Path, writes: &mut Writes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_preserves_order() {
        let mut writes = Writes::default();
        writes.push(PathBuf::from("a.md"), "a".to_string());
        writes.push(PathBuf::from("b.md"), "b".to_string());
        let collected = writes.into_vec();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].0, PathBuf::from("a.md"));
        assert_eq!(collected[1].0, PathBuf::from("b.md"));
    }

    #[test]
    fn default_is_empty() {
        assert!(Writes::default().into_vec().is_empty());
    }
}
