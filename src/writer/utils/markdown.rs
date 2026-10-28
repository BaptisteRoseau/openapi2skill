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
            .map(|cell| padded_cell(cell.as_ref()))
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
        for _ in &self.header_cells {
            out.push_str(" --- |");
        }
        out.push('\n');
        out.extend(self.rows);
        out.push('\n');
        out
    }
}

/// Wraps a cell in the single space of markdownlint's `compact` table column style,
/// escaping the pipes that would otherwise open a new column (MD056).
fn padded_cell(cell: &str) -> String {
    let cell = balanced_backticks(&cell.replace('|', "\\|"));
    if cell.is_empty() {
        " |".to_string()
    } else {
        format!(" {cell} |")
    }
}

/// Escapes a cell's backticks when they do not pair up, so a stray one cannot open a code
/// span that swallows the next column separator.
fn balanced_backticks(cell: &str) -> String {
    if cell.matches('`').count().is_multiple_of(2) {
        cell.to_string()
    } else {
        cell.replace('`', "\\`")
    }
}

/// Flattens a spec description onto the single line a table cell has room for: block
/// markup that would be read as inline markup once the newlines are gone gets escaped.
pub(crate) fn normalize_desc(s: &str) -> String {
    s.lines()
        .map(escape_block_markers)
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

/// Escapes the list bullet, heading or quote marker opening a line, which only means what
/// it means at the start of a line.
fn escape_block_markers(line: &str) -> String {
    let trimmed = line.trim_start();
    let marker_len = trimmed
        .find(|c: char| !matches!(c, '-' | '*' | '+' | '>' | '#'))
        .unwrap_or(trimmed.len());
    if marker_len == 0 {
        return trimmed.to_string();
    }
    format!("\\{trimmed}")
}

/// Renders a description as its own paragraph, or nothing when absent or blank. The
/// description's own line structure is kept: it is markdown in its own right.
pub(crate) fn desc_paragraph(description: Option<&str>) -> String {
    match description.map(str::trim) {
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

/// Renders an index file: a top-level heading followed by one bullet link per entry.
pub(crate) fn build_index(title: &str, links: &[(String, String)]) -> String {
    let bullets = links
        .iter()
        .map(|(file, name)| format!("- [{name}](./{file})"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("# {title}\n\n{bullets}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Table ---

    #[test]
    fn table_header_separator_matches_header_width() {
        let table = Table::new(&["Extension", "Value"]);
        assert_eq!(table.finish(), "| Extension | Value |\n| --- | --- |\n\n");
    }

    #[test]
    fn table_renders_rows() {
        let mut table = Table::new(&["Header", "Value"]);
        table.row(&["`X-Key`", "Your API key"]);
        assert_eq!(
            table.finish(),
            "| Header | Value |\n| --- | --- |\n| `X-Key` | Your API key |\n\n"
        );
    }

    #[test]
    fn table_unlabeled_uses_blank_header() {
        let mut table = Table::unlabeled(2);
        table.row(&["**Method**", "`GET`"]);
        assert_eq!(
            table.finish(),
            "| | |\n| --- | --- |\n| **Method** | `GET` |\n\n"
        );
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
    fn normalize_desc_escapes_a_flattened_bullet() {
        assert_eq!(
            normalize_desc("cases:\n* one\n* two"),
            "cases: \\* one \\* two"
        );
    }

    #[test]
    fn table_cell_escapes_pipes() {
        let mut table = Table::new(&["A"]);
        table.row(&["a | b"]);
        assert!(table.finish().contains("| a \\| b |"));
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
    fn desc_paragraph_keeps_the_description_line_structure() {
        assert_eq!(desc_paragraph(Some("a\nb")), "a\nb\n\n");
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
            build_index("Schemas", &links),
            "# Schemas\n\n- [Pet](./pet.md)\n- [Tag](./tag.md)\n"
        );
    }

    #[test]
    fn build_index_empty_is_just_the_heading() {
        assert_eq!(build_index("Schemas", &[]), "# Schemas\n\n\n");
    }
}
