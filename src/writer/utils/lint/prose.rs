//! Rewrites the inline constructs of spec-supplied description text into the markdown
//! markdownlint accepts, keeping the text itself intact.

use std::sync::LazyLock;

use regex::{Captures, Regex};

/// Applies every inline rewrite to one logical line of prose.
pub(super) fn sanitize_inline(line: &str) -> String {
    let out = rewrite_html(line);
    let out = trim_emphasis_padding(&out);
    let out = STRONG_UNDERSCORE.replace_all(&out, "**${1}**").into_owned();
    let out = LINK_TEXT_PADDING
        .replace_all(&out, "[${1}](${2})")
        .into_owned();
    let out = trim_code_spans(&out);
    let out = flatten_dead_links(&out);
    let out = describe_vague_links(&out);
    let out = drop_untitled_images(&out);
    let out = outside_code_spans(&out, autolink_urls);
    outside_code_spans(&out, autolink_emails)
}

/// `** text **` instead of `**text**` (MD037).
static STRONG_PADDING: LazyLock<[Regex; 4]> = LazyLock::new(|| {
    [
        Regex::new(r"\*\*([^*\s](?:[^*]*[^*\s])?)[ \t]+\*\*").unwrap(),
        Regex::new(r"\*\*[ \t]+([^*\s](?:[^*]*[^*\s])?)\*\*").unwrap(),
        Regex::new(r"__([^_\s](?:[^_]*[^_\s])?)[ \t]+__").unwrap(),
        Regex::new(r"__[ \t]+([^_\s](?:[^_]*[^_\s])?)__").unwrap(),
    ]
});

fn trim_emphasis_padding(line: &str) -> String {
    let [strong_right, strong_left, under_right, under_left] = &*STRONG_PADDING;
    let out = strong_right.replace_all(line, "**${1}** ").into_owned();
    let out = strong_left.replace_all(&out, " **${1}**").into_owned();
    let out = under_right.replace_all(&out, "__${1}__ ").into_owned();
    under_left.replace_all(&out, " __${1}__").into_owned()
}

/// `__strong__` instead of `**strong**` (MD050).
static STRONG_UNDERSCORE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"__([^_\s](?:[^_]*[^_\s])?)__").unwrap());

/// `[ text ](url)` instead of `[text](url)` (MD039).
static LINK_TEXT_PADDING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[[ \t]*([^\]]*?)[ \t]*\]\(([^)]*)\)").unwrap());

/// Trims the padding inside code spans (MD038), pairing backticks so the text between two
/// spans is left alone.
fn trim_code_spans(line: &str) -> String {
    let segments: Vec<&str> = line.split('`').collect();
    let last = segments.len() - 1;
    let mut out = String::with_capacity(line.len());
    for (i, segment) in segments.iter().enumerate() {
        if i > 0 {
            out.push('`');
        }
        // Odd segments sit between a pair of backticks, so they are code spans. The final
        // segment never is: nothing closes it.
        if i % 2 == 1 && i != last {
            out.push_str(segment.trim());
        } else {
            out.push_str(segment);
        }
    }
    out
}

/// Links no reader can follow: empty targets (MD042), bare fragments that name no heading
/// in the generated file (MD051), and `[text][label]` references whose definition lived in
/// the original documentation site (MD052). The text is kept, the link dropped.
static DEAD_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]*)\]\((?:|#[^)]*)\)|\[([^\]]*)\]\[[^\]]*\]").unwrap());

fn flatten_dead_links(line: &str) -> String {
    DEAD_LINK
        .replace_all(line, |caps: &Captures| {
            caps.get(1)
                .or_else(|| caps.get(2))
                .map_or(String::new(), |m| m.as_str().to_string())
        })
        .into_owned()
}

/// Link labels that say nothing about the destination (MD059). The label is kept as prose
/// and the destination moved next to it.
static VAGUE_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\[(click here|here|link|more|read more|this)\]\(([^)\s]+)\)").unwrap()
});

fn describe_vague_links(line: &str) -> String {
    VAGUE_LINK.replace_all(line, "${1} (<${2}>)").into_owned()
}

/// Images with no alternate text (MD045) carry nothing an agent can read.
static UNTITLED_IMAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"!\[[ \t]*\]\([^)]*\)").unwrap());

fn drop_untitled_images(line: &str) -> String {
    UNTITLED_IMAGE.replace_all(line, "").into_owned()
}

/// Runs `rewrite` over the parts of the line that are not code spans, where a URL or an
/// address is prose rather than part of an example command.
fn outside_code_spans(line: &str, rewrite: fn(&str, &mut String)) -> String {
    let mut out = String::with_capacity(line.len());
    for (i, segment) in line.split('`').enumerate() {
        if i > 0 {
            out.push('`');
        }
        // Odd segments sit between backticks, so they are code spans.
        if i % 2 == 1 {
            out.push_str(segment);
        } else {
            rewrite(segment, &mut out);
        }
    }
    out
}

/// Wraps every URL that is not already a link target, an autolink or an HTML attribute in
/// the angle brackets markdownlint wants (MD034).
fn autolink_urls(segment: &str, out: &mut String) {
    let mut rest = segment;
    while let Some(start) = next_url_start(rest) {
        let (before, from_url) = rest.split_at(start);
        let (url, after) = split_url(from_url);
        out.push_str(before);
        if is_already_linked(before) {
            out.push_str(url);
        } else {
            out.push_str(&format!("<{url}>"));
        }
        rest = after;
    }
    out.push_str(rest);
}

fn next_url_start(text: &str) -> Option<usize> {
    let https = text.find("https://");
    let http = text.find("http://");
    https.into_iter().chain(http).min()
}

/// Splits the URL off the front of `text`, leaving the sentence punctuation that follows it.
fn split_url(text: &str) -> (&str, &str) {
    let end = text
        .find(|c: char| c.is_whitespace() || matches!(c, '<' | '>' | '[' | ']' | '(' | ')' | '"'))
        .unwrap_or(text.len());
    let trimmed = text[..end].trim_end_matches(['.', ',', ';', ':', '!', '?']);
    text.split_at(trimmed.len())
}

fn is_already_linked(before: &str) -> bool {
    before.ends_with("](") || before.ends_with(['[', '<', '"', '\'', '='])
}

/// An email address, which markdownlint wants in angle brackets too (MD034).
static EMAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[A-Za-z0-9._%+\-]+@[A-Za-z0-9\-]+(\.[A-Za-z0-9\-]+)*\.[A-Za-z]{2,}").unwrap()
});

fn autolink_emails(segment: &str, out: &mut String) {
    let mut last = 0;
    for found in EMAIL.find_iter(segment) {
        out.push_str(&segment[last..found.start()]);
        let before = &segment[..found.start()];
        let after = &segment[found.end()..];
        if before.ends_with(['<', '/', ':', '=', '@', '.']) || after.starts_with('>') {
            out.push_str(found.as_str());
        } else {
            out.push_str(&format!("<{}>", found.as_str()));
        }
        last = found.end();
    }
    out.push_str(&segment[last..]);
}

/// A `*` or `_` standing alone between spaces, which a description's flattened bullet list
/// leaves behind and markdownlint reads as an unclosed emphasis marker (MD037).
static LONE_MARKER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^|[ \t])(\*\*|__|\*|_)([ \t]|$)").unwrap());

/// Escapes the lone emphasis markers in a line's body, which the caller passes without any
/// list marker of its own.
pub(super) fn escape_lone_markers(body: &str) -> String {
    LONE_MARKER.replace_all(body, "${1}\\${2}${3}").into_owned()
}

/// Anything shaped like an HTML tag (MD033): the elements with a markdown equivalent are
/// translated, and the rest — mostly `<placeholder>` notation from the spec's own prose —
/// is escaped so it reads as the literal text it was meant to be.
static HTML_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<(/?)([A-Za-z][A-Za-z0-9-]*)((?:\s[^<>]*)?)/?>").unwrap());

static HTML_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<!--.*?-->").unwrap());

static HREF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)href\s*=\s*["']?([^"'\s>]+)"#).unwrap());

static ALT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)alt\s*=\s*["']([^"']*)["']"#).unwrap());

static SRC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)src\s*=\s*["']?([^"'\s>]+)"#).unwrap());

fn rewrite_html(line: &str) -> String {
    let line = HTML_COMMENT.replace_all(line, "");
    let rewritten = HTML_TAG
        .replace_all(&line, |caps: &Captures| {
            let closing = !caps[1].is_empty();
            let tag = caps[2].to_ascii_lowercase();
            let attrs = &caps[3];
            match tag.as_str() {
                "br" => " ".to_string(),
                "b" | "strong" => "**".to_string(),
                "i" | "em" => "*".to_string(),
                "code" | "tt" => "`".to_string(),
                "del" | "s" | "strike" => "~~".to_string(),
                "li" if !closing => "- ".to_string(),
                "p" | "div" | "span" | "ul" | "ol" | "li" | "blockquote" | "pre" | "tbody"
                | "thead" | "tr" | "td" | "th" | "table" | "h1" | "h2" | "h3" | "h4" | "h5"
                | "h6" => String::new(),
                "a" if closing => "]".to_string(),
                "a" => match HREF.captures(attrs) {
                    Some(href) => format!("](#SPLIT#{})", &href[1]),
                    None => String::new(),
                },
                "img" => match (SRC.captures(attrs), ALT.captures(attrs)) {
                    (Some(src), Some(alt)) if !alt[1].trim().is_empty() => {
                        format!("![{}]({})", &alt[1], &src[1])
                    }
                    _ => String::new(),
                },
                _ => format!("\\<{}{}{}>", &caps[1], &caps[2], attrs),
            }
        })
        .into_owned();
    repair_anchors(rewritten)
}

/// `<a href="url">text</a>` arrives as `](#SPLIT#url)text]`; this puts the pieces in
/// markdown order.
fn repair_anchors(line: String) -> String {
    if !line.contains("#SPLIT#") {
        return line;
    }
    ANCHOR
        .replace_all(&line, "[${2}](${1})")
        .replace("#SPLIT#", "")
}

static ANCHOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\]\(#SPLIT#([^)]*)\)([^\]]*)\]").unwrap());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_underscore_strong() {
        assert_eq!(
            sanitize_inline("__HTTP Method__: GET"),
            "**HTTP Method**: GET"
        );
    }

    #[test]
    fn leaves_snake_case_identifiers_alone() {
        assert_eq!(
            sanitize_inline("field __x and y__ and a_b_c"),
            "field **x and y** and a_b_c"
        );
    }

    #[test]
    fn moves_padding_outside_emphasis_markers() {
        assert_eq!(
            sanitize_inline("**requires scope: **`read`"),
            "**requires scope:** `read`"
        );
    }

    #[test]
    fn trims_padded_link_text() {
        assert_eq!(
            sanitize_inline("[Get Album ](https://x.test/a)"),
            "[Get Album](https://x.test/a)"
        );
    }

    #[test]
    fn trims_padded_code_span() {
        assert_eq!(sanitize_inline("use ` id ` here"), "use `id` here");
    }

    #[test]
    fn leaves_the_space_before_a_cell_pipe() {
        assert_eq!(
            sanitize_inline("| `raw-request` | `true` |"),
            "| `raw-request` | `true` |"
        );
    }

    #[test]
    fn flattens_empty_and_fragment_links() {
        assert_eq!(sanitize_inline("a [nested field]() b"), "a nested field b");
        assert_eq!(sanitize_inline("see [the guide](#guide)"), "see the guide");
    }

    #[test]
    fn flattens_undefined_reference_links() {
        assert_eq!(sanitize_inline("see [the guide][guide]"), "see the guide");
    }

    #[test]
    fn moves_destination_out_of_vague_link_text() {
        assert_eq!(
            sanitize_inline("documented [here](https://x.test/d)"),
            "documented here (<https://x.test/d>)"
        );
    }

    #[test]
    fn drops_images_without_alt_text() {
        assert_eq!(sanitize_inline("a ![](https://x.test/i.png) b"), "a  b");
    }

    #[test]
    fn keeps_images_with_alt_text() {
        let input = "![a diagram](https://x.test/i.png)";
        assert_eq!(sanitize_inline(input), input);
    }

    #[test]
    fn autolinks_bare_urls() {
        assert_eq!(
            sanitize_inline("See https://x.test/a#L11 and more"),
            "See <https://x.test/a#L11> and more"
        );
        assert_eq!(
            sanitize_inline("(https://x.test/a)."),
            "(<https://x.test/a>)."
        );
    }

    #[test]
    fn autolinks_bare_emails() {
        assert_eq!(
            sanitize_inline("mail user@example.test now"),
            "mail <user@example.test> now"
        );
    }

    #[test]
    fn leaves_a_url_used_as_link_text_alone() {
        let input = "[http://x.test](http://x.test)";
        assert_eq!(sanitize_inline(input), input);
    }

    #[test]
    fn leaves_markdown_links_and_autolinks_alone() {
        let input = "[a](https://x.test/a) and <https://x.test/b> and <u@x.test>";
        assert_eq!(sanitize_inline(input), input);
    }

    #[test]
    fn leaves_urls_inside_code_spans_alone() {
        let input = "`curl https://x.test/a`";
        assert_eq!(sanitize_inline(input), input);
    }

    #[test]
    fn converts_inline_html_to_markdown() {
        assert_eq!(
            sanitize_inline("<b>bold</b> and <em>italic</em> and <code>x</code>"),
            "**bold** and *italic* and `x`"
        );
    }

    #[test]
    fn converts_anchors_to_links() {
        assert_eq!(
            sanitize_inline(r#"see <a href="https://x.test/a">the docs</a>"#),
            "see [the docs](https://x.test/a)"
        );
    }

    #[test]
    fn drops_block_html_wrappers() {
        assert_eq!(sanitize_inline("<ul><li>one</li></ul>"), "- one");
    }

    #[test]
    fn escapes_placeholder_pseudo_tags() {
        assert_eq!(
            sanitize_inline("filter[<name>] and <invocationId>"),
            r"filter[\<name>] and \<invocationId>"
        );
    }

    #[test]
    fn strips_html_comments() {
        assert_eq!(sanitize_inline("a <!-- hidden --> b"), "a  b");
    }

    #[test]
    fn escapes_a_lone_emphasis_marker() {
        assert_eq!(
            escape_lone_markers("binds together: * a name, and * a secret"),
            r"binds together: \* a name, and \* a secret"
        );
    }

    #[test]
    fn leaves_paired_emphasis_alone() {
        let input = "a **bold** and *italic* run";
        assert_eq!(escape_lone_markers(input), input);
    }

    #[test]
    fn leaves_plain_prose_untouched() {
        let input = "A plain sentence with `code`, **bold** and [a link](./x.md).";
        assert_eq!(sanitize_inline(input), input);
    }
}
