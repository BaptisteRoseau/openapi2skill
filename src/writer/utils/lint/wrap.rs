/// Column at which prose lines are wrapped, matching markdownlint's `MD013` default.
pub(super) const MAX_LINE_WIDTH: usize = 80;

/// Wraps one prose line, keeping a list marker on the first line and indenting the
/// continuations to the item's text column.
pub(super) fn wrap_line(line: &str) -> Vec<String> {
    if line.chars().count() <= MAX_LINE_WIDTH {
        return vec![line.to_string()];
    }
    let (prefix, body) = split_marker(line);
    let cont_indent = " ".repeat(prefix.chars().count());
    let mut lines = Vec::new();
    let mut current: Option<String> = None;

    for word in body.split_whitespace() {
        match current.take() {
            Some(line) if line.chars().count() + 1 + word.chars().count() <= MAX_LINE_WIDTH => {
                current = Some(format!("{line} {word}"));
            }
            Some(line) => {
                lines.push(line);
                current = Some(format!("{cont_indent}{}", escaped_if_markup(word)));
            }
            None => current = Some(format!("{prefix}{}", escaped_if_markup(word))),
        }
    }
    lines.extend(current);
    lines
}

/// Backslash-escapes a word that would be read as block markup when it starts a line.
fn escaped_if_markup(word: &str) -> String {
    if starts_block_markup(word) {
        format!("\\{word}")
    } else {
        word.to_string()
    }
}

pub(super) fn is_thematic_break(trimmed: &str) -> bool {
    let chars: Vec<char> = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
    chars.len() >= 3
        && (chars.iter().all(|&c| c == '-')
            || chars.iter().all(|&c| c == '*')
            || chars.iter().all(|&c| c == '_'))
}

/// Splits a line into the indent-plus-list-marker that only the first wrapped line keeps,
/// and the body that gets wrapped.
fn split_marker(line: &str) -> (&str, &str) {
    let indent_len = line.len() - line.trim_start().len();
    let (indent, rest) = line.split_at(indent_len);
    let marker_len = marker_width(rest);
    let prefix_len = indent.len() + marker_len;
    (&line[..prefix_len], &rest[marker_len..])
}

/// Length of the `- `, `1. ` or `> ` marker opening `rest`, or 0 when there is none.
pub(super) fn marker_width(rest: &str) -> usize {
    let Some(first) = rest.chars().next() else {
        return 0;
    };
    if matches!(first, '-' | '*' | '+' | '>') {
        return if rest[1..].starts_with(' ') { 2 } else { 0 };
    }
    if !first.is_ascii_digit() {
        return 0;
    }
    let digits = rest.chars().take_while(char::is_ascii_digit).count();
    let after = &rest[digits..];
    if (after.starts_with('.') || after.starts_with(')')) && after[1..].starts_with(' ') {
        digits + 2
    } else {
        0
    }
}

/// Whether `word` at the start of a line would be parsed as a list bullet, blockquote,
/// heading, table row, fence or thematic break.
fn starts_block_markup(word: &str) -> bool {
    matches!(word, "-" | "*" | "+" | "|")
        // A hash opens a heading and an angle bracket a blockquote with or without a space
        // after it.
        || word.starts_with(['#', '>'])
        || is_thematic_break(word)
        || word.starts_with("```")
        || word.starts_with("~~~")
        || ordered_list_marker(word)
}

fn ordered_list_marker(word: &str) -> bool {
    let digits = word.chars().take_while(char::is_ascii_digit).count();
    digits > 0 && matches!(&word[digits..], "." | ")")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_keeps_short_lines() {
        assert_eq!(wrap_line("short"), vec!["short"]);
    }

    #[test]
    fn wrap_indents_continuation_to_list_text_column() {
        let line = format!("- {}", "word ".repeat(25).trim());
        let lines = wrap_line(&line);
        assert!(lines[0].starts_with("- "));
        assert!(lines[1].starts_with("  w"));
    }

    #[test]
    fn wrap_escapes_a_bullet_that_lands_at_a_line_start() {
        let line = format!("{} - trailing clause goes here", "word ".repeat(16).trim());
        let lines = wrap_line(&line);
        assert!(lines.len() > 1);
        assert!(!lines.iter().any(|l| l.trim_start().starts_with("- ")));
        assert!(lines.iter().any(|l| l.trim_start().starts_with("\\-")));
    }

    #[test]
    fn wrap_puts_oversized_token_on_its_own_line() {
        let url = "https://example.com/".to_string() + &"x".repeat(90);
        let lines = wrap_line(&format!("see {url} for details"));
        assert!(lines.iter().any(|l| l.trim() == url));
    }

    #[test]
    fn marker_width_detects_markers() {
        assert_eq!(marker_width("- item"), 2);
        assert_eq!(marker_width("1. item"), 3);
        assert_eq!(marker_width("12) item"), 4);
        assert_eq!(marker_width("-no-space"), 0);
        assert_eq!(marker_width("plain"), 0);
    }

    #[test]
    fn starts_block_markup_matches_markers_only() {
        assert!(starts_block_markup("-"));
        assert!(starts_block_markup("1."));
        assert!(starts_block_markup("---"));
        assert!(starts_block_markup("##"));
        assert!(starts_block_markup("#swagger](http://x.test/irc/)"));
        assert!(starts_block_markup(">quoted"));
        assert!(!starts_block_markup("-dash-prefixed"));
        assert!(!starts_block_markup("*emphasis*"));
    }

    #[test]
    fn thematic_break_needs_three_chars() {
        assert!(is_thematic_break("---"));
        assert!(!is_thematic_break("--"));
        assert!(!is_thematic_break("-a-"));
    }
}
