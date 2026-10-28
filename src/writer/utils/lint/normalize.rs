//! Rewrites a finished markdown file into the shape markdownlint expects: clean
//! whitespace, blank lines around block constructs, normalized lists and tables, and
//! prose wrapped at [`MAX_LINE_WIDTH`].

use super::{
    prose::{escape_lone_markers, sanitize_inline},
    table::{compact_row, is_delimiter_row},
    wrap::{is_thematic_break, marker_width, wrap_line},
};

/// Indentation at which a line stops being prose and becomes an indented code block.
const CODE_INDENT: usize = 4;

pub(crate) fn normalize_markdown(content: &str) -> String {
    let raw = split_blocks(content);
    let raw = fence_indented_code(raw);
    let raw = join_paragraphs(raw);
    let lines = classify(raw);
    let lines = with_block_spacing(lines);
    let lines = label_standalone_emphasis(lines);
    render(&lines)
}

/// What a line is for the purposes of spacing and wrapping.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// The YAML frontmatter block and its `---` fences, which no rule applies to.
    Frontmatter,
    /// The ``` line opening a fenced code block.
    FenceOpen,
    /// The ``` line closing a fenced code block.
    FenceClose,
    /// A line inside a fenced code block, kept as it is.
    Code,
    Blank,
    Heading,
    ThematicBreak,
    ListItem,
    TableRow,
    Prose,
}

struct Line {
    text: String,
    kind: Kind,
}

impl Line {
    fn blank() -> Self {
        Self {
            text: String::new(),
            kind: Kind::Blank,
        }
    }
}

/// A line before its inline markdown is rewritten, tagged with whether it is verbatim
/// content that the later passes must leave alone.
struct Raw {
    text: String,
    verbatim: bool,
}

impl Raw {
    fn prose(text: String) -> Self {
        Self {
            text,
            verbatim: false,
        }
    }

    fn verbatim(text: String) -> Self {
        Self {
            text,
            verbatim: true,
        }
    }

    fn indent(&self) -> usize {
        self.text.len() - self.text.trim_start().len()
    }

    fn is_blank(&self) -> bool {
        self.text.trim().is_empty()
    }
}

/// Splits the file into lines, cleaning their whitespace and marking the frontmatter and
/// fenced code blocks as verbatim.
fn split_blocks(content: &str) -> Vec<Raw> {
    let mut lines = Vec::new();
    let mut in_frontmatter = false;
    let mut in_fence = false;

    for (i, raw) in content.lines().enumerate() {
        let text = raw.trim_end().replace('\t', "    ");

        if i == 0 && text == "---" {
            in_frontmatter = true;
            lines.push(Raw::verbatim(text));
            continue;
        }
        if in_frontmatter {
            in_frontmatter = text != "---";
            lines.push(Raw::verbatim(text));
            continue;
        }
        if is_fence(&text) {
            let opening = !in_fence;
            in_fence = !in_fence;
            lines.push(Raw::verbatim(if opening {
                with_language(&text)
            } else {
                text
            }));
            continue;
        }
        if in_fence {
            lines.push(Raw::verbatim(text));
            continue;
        }
        lines.push(Raw::prose(text));
    }
    lines
}

/// Wraps the indented code blocks a description brings along in fences, so that every code
/// block in the file is written the same way (MD046).
fn fence_indented_code(lines: Vec<Raw>) -> Vec<Raw> {
    let mut out: Vec<Raw> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        if !opens_indented_code(&lines[i], &out) {
            let line = &lines[i];
            out.push(Raw {
                text: line.text.clone(),
                verbatim: line.verbatim,
            });
            i += 1;
            continue;
        }
        let end = indented_code_end(&lines, i);
        out.push(Raw::verbatim("```text".to_string()));
        for line in &lines[i..end] {
            out.push(Raw::verbatim(dedent(&line.text, CODE_INDENT)));
        }
        out.push(Raw::verbatim("```".to_string()));
        i = end;
    }
    out
}

/// An indented code block starts where a deeply indented line follows a paragraph that was
/// not a list — under a list, that indentation is a continuation rather than code.
fn opens_indented_code(line: &Raw, out: &[Raw]) -> bool {
    if line.verbatim || line.is_blank() || line.indent() < CODE_INDENT {
        return false;
    }
    match out.iter().rev().find(|l| !l.is_blank()) {
        None => true,
        Some(previous) => {
            !previous.verbatim
                && previous.indent() == 0
                && marker_width(previous.text.trim_start()) == 0
        }
    }
}

/// The end of the indented block that starts at `start`: indented lines, plus the blank
/// lines between them, which belong to the same block.
fn indented_code_end(lines: &[Raw], start: usize) -> usize {
    let mut end = start;
    for (i, line) in lines.iter().enumerate().skip(start) {
        if !line.verbatim && line.indent() >= CODE_INDENT && !line.is_blank() {
            end = i + 1;
        } else if !line.is_blank() {
            break;
        }
    }
    end
}

fn dedent(text: &str, columns: usize) -> String {
    let strip = text.len() - text.trim_start().len();
    text[strip.min(columns)..].to_string()
}

/// Joins the lines of each paragraph into one logical line, so that inline markdown
/// spanning a line break is rewritten as a whole and the result is wrapped afresh.
fn join_paragraphs(lines: Vec<Raw>) -> Vec<Raw> {
    let mut out: Vec<Raw> = Vec::with_capacity(lines.len());
    for line in lines {
        let continues = out
            .last()
            .is_some_and(|previous| continues_paragraph(previous, &line));
        match out.last_mut() {
            Some(previous) if continues => {
                previous.text.push(' ');
                previous.text.push_str(line.text.trim());
            }
            _ => out.push(line),
        }
    }
    out
}

fn continues_paragraph(previous: &Raw, line: &Raw) -> bool {
    if previous.verbatim || line.verbatim || previous.is_blank() || line.is_blank() {
        return false;
    }
    let previous_text = previous.text.trim_start();
    let text = line.text.trim_start();
    let starts_new_block = text.starts_with('#')
        || text.starts_with('|')
        || is_thematic_break(text)
        || is_delimiter_row(text)
        || marker_width(text) > 0;
    // A heading, a break or a table row ends at its line: the next line is not its body.
    let previous_ends_block = previous_text.starts_with('#')
        || previous_text.starts_with('|')
        || is_thematic_break(previous_text)
        || is_delimiter_row(previous_text);
    !starts_new_block && !previous_ends_block
}

/// Rewrites each line's inline markdown and records what kind of block it belongs to.
fn classify(lines: Vec<Raw>) -> Vec<Line> {
    let mut out = Vec::with_capacity(lines.len());
    let mut list_indents: Vec<ListLevel> = Vec::new();
    let mut in_frontmatter = false;
    let mut in_fence = false;

    for (i, raw) in lines.iter().enumerate() {
        if raw.verbatim {
            out.push(verbatim_line(
                &raw.text,
                i,
                &mut in_frontmatter,
                &mut in_fence,
            ));
            continue;
        }
        let sanitized = sanitize_inline(&raw.text);
        let text = sanitized.trim_end();
        if text.trim().is_empty() {
            out.push(Line::blank());
            continue;
        }
        let trimmed = text.trim_start();
        if trimmed.starts_with('#') {
            list_indents.clear();
            out.push(Line {
                text: trim_heading_punctuation(text),
                kind: Kind::Heading,
            });
        } else if is_thematic_break(trimmed) {
            list_indents.clear();
            out.push(Line {
                text: text.to_string(),
                kind: Kind::ThematicBreak,
            });
        } else if trimmed.starts_with('|') || is_delimiter_row(trimmed) {
            list_indents.clear();
            out.push(Line {
                text: compact_row(trimmed),
                kind: Kind::TableRow,
            });
        } else if marker_width(trimmed) > 0 {
            out.push(Line {
                text: normalized_list_item(text, &mut list_indents),
                kind: Kind::ListItem,
            });
        } else {
            list_indents.clear();
            out.push(Line {
                text: escape_lone_markers(text),
                kind: Kind::Prose,
            });
        }
    }
    out
}

fn verbatim_line(text: &str, i: usize, in_frontmatter: &mut bool, in_fence: &mut bool) -> Line {
    let kind = if i == 0 && text == "---" {
        *in_frontmatter = true;
        Kind::Frontmatter
    } else if *in_frontmatter {
        *in_frontmatter = text != "---";
        Kind::Frontmatter
    } else if is_fence(text) {
        *in_fence = !*in_fence;
        if *in_fence {
            Kind::FenceOpen
        } else {
            Kind::FenceClose
        }
    } else {
        Kind::Code
    };
    Line {
        text: text.to_string(),
        kind,
    }
}

/// One open list level: where the item was indented in the source, and where its nested
/// items have to start so markdownlint reads them as nested.
struct ListLevel {
    source_indent: usize,
    child_indent: usize,
}

/// Re-indents a list item so each nesting level starts at its parent's text column, and
/// leaves one space after the marker (MD005, MD007, MD030).
fn normalized_list_item(text: &str, levels: &mut Vec<ListLevel>) -> String {
    let trimmed = text.trim_start();
    let indent = text.len() - trimmed.len();
    while levels
        .last()
        .is_some_and(|level| level.source_indent > indent)
    {
        levels.pop();
    }
    let rendered_indent = match levels.last() {
        Some(level) if level.source_indent == indent => level.child_indent - marker_width(trimmed),
        Some(level) => level.child_indent,
        None => 0,
    };
    let marker_len = marker_width(trimmed);
    if levels
        .last()
        .is_none_or(|level| level.source_indent < indent)
    {
        levels.push(ListLevel {
            source_indent: indent,
            child_indent: rendered_indent + marker_len,
        });
    }
    let marker = trimmed[..marker_len].trim_end();
    let body = escape_lone_markers(trimmed[marker_len..].trim_start());
    format!("{}{marker} {body}", " ".repeat(rendered_indent))
}

/// Trailing punctuation in a heading (MD026), which spec descriptions bring in when they
/// open with a heading of their own.
fn trim_heading_punctuation(text: &str) -> String {
    text.trim_end_matches(['.', ',', ';', ':', '!', '。', '，', '；', '：', '！'])
        .to_string()
}

/// A fenced block with no language (MD040) stops syntax highlighting and tells a reader
/// nothing about the content.
fn with_language(fence: &str) -> String {
    let (marker, info) = fence.split_at(fence.len() - fence.trim_start_matches(['`', '~']).len());
    if info.trim().is_empty() {
        format!("{marker}text")
    } else {
        fence.to_string()
    }
}

/// Inserts the blank lines markdownlint wants around headings, fenced blocks, lists and
/// thematic breaks (MD022, MD031, MD032), and collapses runs of blanks to one (MD012).
fn with_block_spacing(lines: Vec<Line>) -> Vec<Line> {
    let mut out: Vec<Line> = Vec::with_capacity(lines.len());
    let mut iter = lines.into_iter().peekable();

    while let Some(line) = iter.next() {
        if line.kind == Kind::Blank {
            if out.last().is_some_and(|l| l.kind != Kind::Blank) {
                out.push(line);
            }
            continue;
        }
        if needs_blank_before(&line, out.last()) {
            out.push(Line::blank());
        }
        let kind = line.kind;
        out.push(line);
        if needs_blank_after(kind, iter.peek()) {
            out.push(Line::blank());
        }
    }
    while out.last().is_some_and(|l| l.kind == Kind::Blank) {
        out.pop();
    }
    out
}

fn needs_blank_before(line: &Line, previous: Option<&Line>) -> bool {
    let Some(previous) = previous else {
        return false;
    };
    if matches!(previous.kind, Kind::Blank | Kind::Frontmatter) {
        return false;
    }
    match line.kind {
        Kind::Heading | Kind::FenceOpen | Kind::ThematicBreak => true,
        // Only the first item of a list needs the blank line above it.
        Kind::ListItem => previous.kind != Kind::ListItem,
        Kind::Prose | Kind::TableRow => previous.kind == Kind::ListItem,
        _ => false,
    }
}

fn needs_blank_after(kind: Kind, next: Option<&Line>) -> bool {
    let Some(next) = next else {
        return false;
    };
    if next.kind == Kind::Blank {
        return false;
    }
    matches!(kind, Kind::Heading | Kind::ThematicBreak | Kind::FenceClose)
}

/// A paragraph that is nothing but emphasis reads as a heading to markdownlint (MD036);
/// a colon inside the markers makes it the label it was meant to be.
fn label_standalone_emphasis(mut lines: Vec<Line>) -> Vec<Line> {
    for i in 0..lines.len() {
        if lines[i].kind != Kind::Prose {
            continue;
        }
        let surrounded = lines[..i].last().is_none_or(|l| l.kind == Kind::Blank)
            && lines[i + 1..].first().is_none_or(|l| l.kind == Kind::Blank);
        if !surrounded {
            continue;
        }
        if let Some(labelled) = emphasis_label(&lines[i].text) {
            lines[i].text = labelled;
        }
    }
    lines
}

fn emphasis_label(text: &str) -> Option<String> {
    for marker in ["**", "__", "*", "_"] {
        let Some(inner) = text
            .strip_prefix(marker)
            .and_then(|rest| rest.strip_suffix(marker))
        else {
            continue;
        };
        if !inner.is_empty()
            && !inner.contains(marker)
            && !inner.ends_with(['.', ',', ';', ':', '!', '?'])
        {
            return Some(format!("{marker}{inner}:{marker}"));
        }
    }
    None
}

fn render(lines: &[Line]) -> String {
    let mut out = String::new();
    for line in lines {
        match line.kind {
            Kind::Prose | Kind::ListItem => {
                for wrapped in wrap_line(&line.text) {
                    out.push_str(&wrapped);
                    out.push('\n');
                }
            }
            _ => {
                out.push_str(&line.text);
                out.push('\n');
            }
        }
    }
    if out.is_empty() {
        out.push('\n');
    }
    out
}

fn is_fence(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("```") || trimmed.starts_with("~~~")
}

#[cfg(test)]
mod tests {
    use super::{super::wrap::MAX_LINE_WIDTH, *};

    // --- whitespace ---

    #[test]
    fn strips_trailing_spaces() {
        assert_eq!(normalize_markdown("a line   \n"), "a line\n");
    }

    #[test]
    fn collapses_consecutive_blank_lines() {
        assert_eq!(normalize_markdown("a\n\n\n\nb\n"), "a\n\nb\n");
    }

    #[test]
    fn ensures_single_trailing_newline() {
        assert_eq!(normalize_markdown("a\n\n\n"), "a\n");
        assert_eq!(normalize_markdown("a"), "a\n");
    }

    #[test]
    fn replaces_hard_tabs() {
        assert_eq!(normalize_markdown("a\tb\n"), "a    b\n");
    }

    #[test]
    fn drops_a_line_left_blank_by_stripping_html() {
        assert_eq!(normalize_markdown("a\n\n  <tr>\n\nb\n"), "a\n\nb\n");
    }

    // --- blocks left alone ---

    #[test]
    fn leaves_fenced_code_untouched() {
        let input = "```jsonc\n{\n  \"a\": 1,  // a very long comment that runs well past the eighty column limit\n}\n```\n";
        assert_eq!(normalize_markdown(input), input);
    }

    #[test]
    fn leaves_long_table_rows_unwrapped() {
        let row = "| `petId` | integer | Yes | An identifier long enough to run past the eighty column limit |\n";
        assert_eq!(normalize_markdown(row), row);
    }

    #[test]
    fn leaves_headings_unwrapped() {
        let heading = "# GET /a/very/long/path/that/goes/past/the/eighty/column/limit/for/sure\n";
        assert_eq!(normalize_markdown(heading), heading);
    }

    #[test]
    fn leaves_frontmatter_untouched() {
        let input = "---\nname: x\ndescription: a description that is deliberately longer than the eighty column limit\n---\n\n# Title\n";
        assert_eq!(normalize_markdown(input), input);
    }

    // --- block spacing ---

    #[test]
    fn inserts_blank_lines_around_headings() {
        assert_eq!(
            normalize_markdown("text\n\n## Heading\nmore\n"),
            "text\n\n## Heading\n\nmore\n"
        );
    }

    #[test]
    fn inserts_blank_lines_around_lists() {
        assert_eq!(
            normalize_markdown("intro\n- one\n- two\n\noutro\n"),
            "intro\n\n- one\n- two\n\noutro\n"
        );
    }

    #[test]
    fn inserts_blank_lines_around_fences() {
        assert_eq!(
            normalize_markdown("intro\n```sh\nls\n```\noutro\n"),
            "intro\n\n```sh\nls\n```\n\noutro\n"
        );
    }

    #[test]
    fn inserts_blank_line_before_thematic_break() {
        assert_eq!(
            normalize_markdown("text\n\n---\nmore\n"),
            "text\n\n---\n\nmore\n"
        );
    }

    // --- content fixes ---

    #[test]
    fn adds_a_language_to_bare_fences() {
        assert_eq!(normalize_markdown("```\nls\n```\n"), "```text\nls\n```\n");
    }

    #[test]
    fn fences_an_indented_code_block() {
        assert_eq!(
            normalize_markdown("Example:\n\n    a: 1\n    b: 2\n"),
            "Example:\n\n```text\na: 1\nb: 2\n```\n"
        );
    }

    #[test]
    fn keeps_an_indented_list_continuation_out_of_a_fence() {
        assert_eq!(
            normalize_markdown("- item\n    continued here\n"),
            "- item continued here\n"
        );
    }

    #[test]
    fn trims_trailing_punctuation_from_headings() {
        assert_eq!(
            normalize_markdown("## Authentication.\n"),
            "## Authentication\n"
        );
    }

    #[test]
    fn labels_a_paragraph_that_is_only_emphasis() {
        assert_eq!(
            normalize_markdown("**Conditional**\n"),
            "**Conditional:**\n"
        );
    }

    #[test]
    fn leaves_emphasis_with_punctuation_alone() {
        assert_eq!(normalize_markdown("**Note:**\n"), "**Note:**\n");
    }

    #[test]
    fn sanitizes_inline_markdown_in_prose() {
        assert_eq!(normalize_markdown("see <b>this</b>\n"), "see **this**\n");
    }

    #[test]
    fn compacts_a_table_from_a_description() {
        assert_eq!(
            normalize_markdown("| a | b |\n|---|---|\n| 1 | 2 |\n"),
            "| a | b |\n| --- | --- |\n| 1 | 2 |\n"
        );
    }

    #[test]
    fn dedents_a_top_level_list() {
        assert_eq!(
            normalize_markdown("Codes:\n\n  * one\n  * two\n"),
            "Codes:\n\n* one\n* two\n"
        );
    }

    #[test]
    fn keeps_nested_list_depth_at_two_columns() {
        assert_eq!(
            normalize_markdown("- one\n    - nested\n"),
            "- one\n  - nested\n"
        );
    }

    #[test]
    fn leaves_one_space_after_a_list_marker() {
        assert_eq!(normalize_markdown("-   item\n"), "- item\n");
    }

    // --- paragraph joining ---

    #[test]
    fn joins_a_paragraph_and_wraps_it_afresh() {
        assert_eq!(normalize_markdown("one\ntwo\nthree\n"), "one two three\n");
    }

    #[test]
    fn joining_exposes_a_link_split_over_two_lines() {
        assert_eq!(
            normalize_markdown("a [Sigma scheduled\nquery](#scheduled_queries).\n"),
            "a Sigma scheduled query.\n"
        );
    }

    // --- wrapping ---

    #[test]
    fn wraps_long_prose() {
        let long = "word ".repeat(30);
        let out = normalize_markdown(&long);
        assert!(out.lines().count() > 1);
        assert!(out.lines().all(|l| l.chars().count() <= MAX_LINE_WIDTH));
    }
}
