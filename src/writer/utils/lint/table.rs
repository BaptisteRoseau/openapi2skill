//! Re-emits pipe tables — ours and the ones spec descriptions bring along — in the single
//! column style markdownlint accepts (MD055, MD056, MD060).

use std::sync::LazyLock;

use regex::Regex;

/// A row of nothing but dashes, colons and pipes: the line under a table's header.
static DELIMITER_ROW: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*\|?\s*:?-+:?\s*(\|\s*:?-+:?\s*)*\|?\s*$").unwrap());

pub(super) fn is_delimiter_row(line: &str) -> bool {
    line.contains('-') && line.contains('|') && DELIMITER_ROW.is_match(line)
}

/// Rewrites one row with a leading and trailing pipe and a single space around each cell.
pub(super) fn compact_row(line: &str) -> String {
    let cells = split_cells(line);
    if cells.is_empty() {
        return line.to_string();
    }
    let rendered: Vec<String> = cells
        .iter()
        .map(|cell| match cell.trim() {
            "" => String::new(),
            trimmed if is_delimiter_cell(trimmed) => normalize_delimiter(trimmed),
            trimmed => trimmed.to_string(),
        })
        .collect();
    let mut out = String::from("|");
    for cell in rendered {
        if cell.is_empty() {
            out.push_str(" |");
        } else {
            out.push_str(&format!(" {cell} |"));
        }
    }
    out
}

fn is_delimiter_cell(cell: &str) -> bool {
    let core = cell.trim_start_matches(':').trim_end_matches(':');
    !core.is_empty() && core.chars().all(|c| c == '-')
}

/// Keeps the alignment colons, which carry meaning, and drops the dash padding, which
/// does not.
fn normalize_delimiter(cell: &str) -> String {
    let left = cell.starts_with(':');
    let right = cell.ends_with(':');
    match (left, right) {
        (true, true) => ":---:".to_string(),
        (true, false) => ":---".to_string(),
        (false, true) => "---:".to_string(),
        (false, false) => "---".to_string(),
    }
}

/// Splits a row on its column pipes, ignoring the escaped pipes and the ones inside code
/// spans, and dropping the empty cells the leading and trailing pipes produce.
fn split_cells(line: &str) -> Vec<String> {
    let mut cells = vec![String::new()];
    let mut in_code = false;
    let mut escaped = false;

    for ch in line.trim().chars() {
        let current = cells.last_mut().expect("cells is never empty");
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        match ch {
            '\\' => {
                current.push(ch);
                escaped = true;
            }
            '`' => {
                in_code = !in_code;
                current.push(ch);
            }
            '|' if !in_code => cells.push(String::new()),
            _ => current.push(ch),
        }
    }

    // A leading or trailing pipe yields an empty outer cell that is not a column.
    if cells.first().is_some_and(|c| c.trim().is_empty()) {
        cells.remove(0);
    }
    if cells.len() > 1 && cells.last().is_some_and(|c| c.trim().is_empty()) {
        cells.pop();
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_delimiter_rows() {
        assert!(is_delimiter_row("|---|---|"));
        assert!(is_delimiter_row("| :--- | ---: |"));
        assert!(is_delimiter_row("--- | ---"));
        assert!(!is_delimiter_row("| a | b |"));
        assert!(!is_delimiter_row("some - prose"));
    }

    #[test]
    fn pads_cells_to_the_compact_style() {
        assert_eq!(compact_row("| a |b| c |"), "| a | b | c |");
    }

    #[test]
    fn adds_the_outer_pipes() {
        assert_eq!(compact_row("a | b"), "| a | b |");
    }

    #[test]
    fn shortens_delimiter_cells_and_keeps_alignment() {
        assert_eq!(
            compact_row("|-----|:---|---:|:--:|"),
            "| --- | :--- | ---: | :---: |"
        );
    }

    #[test]
    fn keeps_empty_header_cells() {
        assert_eq!(compact_row("| | |"), "| | |");
    }

    #[test]
    fn leaves_escaped_pipes_inside_a_cell() {
        assert_eq!(compact_row(r"| a \| b | c |"), r"| a \| b | c |");
    }

    #[test]
    fn leaves_pipes_inside_code_spans_inside_a_cell() {
        assert_eq!(compact_row("| `a | b` | c |"), "| `a | b` | c |");
    }
}
