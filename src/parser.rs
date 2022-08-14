use crate::error::{Error, Result};
use pulldown_cmark::{html as markdown_html, Parser as MarkdownParser};
use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
pub struct Question {
    pub code: String,
    pub difficulty: u8,
    pub answer: String,
    pub hint: String,
    pub explanation: String,
}

pub struct Markdown {
    pub answer: String,
    pub difficulty: u8,
    pub hint: String,
    pub explanation: String,
}

/// The format of a valid rust-quiz markdown file.
///
/// e.g:
/// ```markdown
/// Answer: 999
/// Difficulty: 1|2|3
///
/// # Hint
/// <!-- markdown -->
/// # Explanation
/// <!-- markdown -->
/// ```
///
pub const MARKDOWN_FORMAT: &str = "
    Answer: 999
    Difficulty: 1|2|3

    # Hint

    <!-- markdown -->

    # Explanation

    <!-- markdown -->
";

const MARKDOWN_REGEX: &str = r"(?msx)
    \AAnswer:\x20(undefined|error|[0-9]+)\n
    Difficulty:\x20(1|2|3)\n
    \n
    \x23\x20Hint\n
    \n
    (.*)
    \n
    \x23\x20Explanation\n
    \n
    (.*)
    \z
";

#[derive(Clone)]
pub enum ParseOption {
    OriginMarkdown,
    RenderHtml,
}

/// Parse a format markdown file and return a `Markdown` struct.
/// The markdown file should be in `MARKDOWN_FORMAT`.
pub fn parse_markdown(content: String, option: ParseOption) -> Result<Markdown> {
    let re = Regex::new(MARKDOWN_REGEX).expect("valid regex");
    let cap = match re.captures(&content) {
        Some(cap) => cap,
        None => return Err(Error::MarkdownFormat(content)),
    };

    let (hint, explanation) = match option {
        ParseOption::OriginMarkdown => (cap[3].to_owned(), cap[4].to_owned()),
        ParseOption::RenderHtml => (render_to_html(&cap[3]), render_to_html(&cap[4])),
    };

    Ok(Markdown {
        answer: cap[1].to_owned(),
        difficulty: cap[2].parse().unwrap(),
        hint,
        explanation,
    })
}

fn render_to_html(markdown: &str) -> String {
    let parser = MarkdownParser::new(markdown);
    let mut html = String::new();
    markdown_html::push_html(&mut html, parser);
    html = html.replace("<a href=\"", "<a target=\"_blank\" href=\"");
    html
}
