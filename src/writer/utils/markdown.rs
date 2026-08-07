/// Builds a markdown pipe table, keeping header/separator/row syntax in one place.
pub(crate) struct Table {
    header_cells: Vec<String>,
    rows: Vec<String>,
}

impl Table {
    pub(crate) fn new(headers: &[&str]) -> Self {
        Self::with_header_cells(headers.iter().map(|h| format!(" {h} ")).collect())
    }

    /// A table with no column names, used for the key/value layout of endpoint info tables.
    pub(crate) fn unlabeled(columns: usize) -> Self {
        Self::with_header_cells(vec![" ".to_string(); columns])
    }

    fn with_header_cells(header_cells: Vec<String>) -> Self {
        Self {
            header_cells,
            rows: Vec::new(),
        }
    }

    pub(crate) fn row<S: AsRef<str>>(&mut self, cells: &[S]) {
        let joined: String = cells
            .iter()
            .map(|cell| format!(" {} |", cell.as_ref()))
            .collect();
        self.rows.push(format!("|{joined}\n"));
    }

    pub(crate) fn finish(self) -> String {
        let mut out = String::new();
        out.push('|');
        for cell in &self.header_cells {
            out.push_str(cell);
            out.push('|');
        }
        out.push_str("\n|");
        for cell in &self.header_cells {
            out.push_str(&"-".repeat(cell.len().max(2)));
            out.push('|');
        }
        out.push('\n');
        out.extend(self.rows);
        out.push('\n');
        out
    }
}

/// Collapses newlines to spaces and trims a spec description for markdown output.
pub(crate) fn normalize_desc(s: &str) -> String {
    s.replace('\n', " ").trim().to_string()
}

/// Renders a description as its own paragraph, or nothing when absent or blank.
pub(crate) fn desc_paragraph(description: Option<&str>) -> String {
    match description.map(normalize_desc) {
        Some(desc) if !desc.is_empty() => format!("{desc}\n\n"),
        _ => String::new(),
    }
}

/// Renders a description for a table cell, falling back to `-` when absent or blank.
pub(crate) fn desc_cell(description: Option<&str>) -> String {
    match description.map(normalize_desc) {
        Some(desc) if !desc.is_empty() => desc,
        _ => "-".to_string(),
    }
}

pub(crate) fn build_index(links: &[(String, String)]) -> String {
    links
        .iter()
        .map(|(file, name)| format!("- [{name}](./{file})"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Table ---

    #[test]
    fn table_header_separator_matches_header_width() {
        let table = Table::new(&["Extension", "Value"]);
        assert_eq!(
            table.finish(),
            "| Extension | Value |\n|-----------|-------|\n\n"
        );
    }

    #[test]
    fn table_renders_rows() {
        let mut table = Table::new(&["Header", "Value"]);
        table.row(&["`X-Key`", "Your API key"]);
        assert_eq!(
            table.finish(),
            "| Header | Value |\n|--------|-------|\n| `X-Key` | Your API key |\n\n"
        );
    }

    #[test]
    fn table_unlabeled_uses_blank_header() {
        let mut table = Table::unlabeled(2);
        table.row(&["**Method**", "`GET`"]);
        assert_eq!(table.finish(), "| | |\n|--|--|\n| **Method** | `GET` |\n\n");
    }

    #[test]
    fn table_accepts_owned_cells() {
        let mut table = Table::new(&["A"]);
        table.row(&[String::from("x")]);
        assert!(table.finish().contains("| x |"));
    }

    // --- normalize_desc ---

    #[test]
    fn normalize_desc_replaces_single_newline_with_space() {
        assert_eq!(normalize_desc("foo\nbar"), "foo bar");
    }

    #[test]
    fn normalize_desc_trims_trailing_newline() {
        assert_eq!(normalize_desc("foo\n"), "foo");
    }

    #[test]
    fn normalize_desc_trims_trailing_whitespace() {
        assert_eq!(normalize_desc("foo   "), "foo");
    }

    #[test]
    fn normalize_desc_double_newline_collapses_to_two_spaces() {
        assert_eq!(normalize_desc("foo\n\nbar"), "foo  bar");
    }

    #[test]
    fn normalize_desc_empty_string_stays_empty() {
        assert_eq!(normalize_desc(""), "");
    }

    #[test]
    fn normalize_desc_no_newlines_passthrough() {
        assert_eq!(normalize_desc("hello world"), "hello world");
    }

    // --- desc_paragraph ---

    #[test]
    fn desc_paragraph_appends_blank_line() {
        assert_eq!(desc_paragraph(Some("A pet")), "A pet\n\n");
    }

    #[test]
    fn desc_paragraph_empty_when_none() {
        assert_eq!(desc_paragraph(None), "");
    }

    #[test]
    fn desc_paragraph_empty_when_blank() {
        assert_eq!(desc_paragraph(Some("  \n ")), "");
    }

    #[test]
    fn desc_paragraph_normalizes_newlines() {
        assert_eq!(desc_paragraph(Some("a\nb")), "a b\n\n");
    }

    // --- desc_cell ---

    #[test]
    fn desc_cell_normalizes() {
        assert_eq!(desc_cell(Some("a\nb")), "a b");
    }

    #[test]
    fn desc_cell_dash_when_none() {
        assert_eq!(desc_cell(None), "-");
    }

    #[test]
    fn desc_cell_dash_when_blank() {
        assert_eq!(desc_cell(Some("   ")), "-");
    }

    // --- build_index ---

    #[test]
    fn build_index_produces_bullet_list() {
        let links = vec![
            ("pet.md".to_string(), "Pet".to_string()),
            ("tag.md".to_string(), "Tag".to_string()),
        ];
        assert_eq!(
            build_index(&links),
            "- [Pet](./pet.md)\n- [Tag](./tag.md)\n"
        );
    }

    #[test]
    fn build_index_empty_is_just_newline() {
        assert_eq!(build_index(&[]), "\n");
    }
}
