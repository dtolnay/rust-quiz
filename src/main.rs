use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
struct Question {
    code: String,
    difficulty: u8,
    answer: String,
    hint: String,
    explanation: String,
}

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

const MARKDOWN_FORMAT: &str = "
    Answer: 999
    Difficulty: 1|2|3

    # Hint

    <!-- markdown -->

    # Explanation

    <!-- markdown -->
";

fn main() {
    if let Err(err) = try_main() {
        let _ = write!(io::stderr(), "ERROR: {}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let mut question_files = Vec::new();
    for entry in fs::read_dir("questions")? {
        let entry = entry?;
        let path = entry.path();
        if path.to_string_lossy().ends_with(".rs") {
            question_files.push(path);
        }
    }

    question_files.sort();

    let mut questions = BTreeMap::new();
    for path in question_files {
        println!("evaluating {}", path.display());
        let code = fs::read_to_string(&path)?;

        let Markdown {
            answer,
            difficulty,
            hint,
            explanation,
        } = parse_markdown(path.with_extension("md"))?;

        check_answer(&path, &answer);

        let re = Regex::new(r"questions/([0-9]{3})[a-z0-9-]+\.rs").unwrap();
        let number = match re.captures(&path.to_str().unwrap()) {
            Some(cap) => cap[1].parse::<u16>().unwrap(),
            None => return Err(Error::FilenameFormat),
        };

        questions.insert(
            number,
            Question {
                code,
                answer,
                difficulty,
                hint,
                explanation,
            },
        );
    }

    let json_object = serde_json::to_string_pretty(&questions)?;
    let javascript = format!("var questions = {};\n", json_object);
    fs::write("docs/questions.js", javascript)?;

    Ok(())
}

struct Markdown {
    answer: String,
    difficulty: u8,
    hint: String,
    explanation: String,
}

fn parse_markdown(path: PathBuf) -> Result<Markdown> {
    let content = fs::read_to_string(&path)?;
    let re = Regex::new(MARKDOWN_REGEX).unwrap();
    let cap = match re.captures(&content) {
        Some(cap) => cap,
        None => return Err(Error::MarkdownFormat(path)),
    };

    Ok(Markdown {
        answer: cap[1].to_owned(),
        difficulty: cap[2].parse().unwrap(),
        hint: render_to_html(&cap[3]),
        explanation: render_to_html(&cap[4]),
    })
}

fn render_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html = html.replace("<a href=\"", "<a target=\"_blank\" href=\"");
    html
}

fn check_answer(path: &Path, expected: &str) {
    let stem = path.file_stem().unwrap().to_str().unwrap();

    let status = Command::new("rustc")
        .arg(path)
        .arg("--edition=2018")
        .arg("--out-dir=/tmp/rust-quiz")
        .stderr(Stdio::null())
        .status()
        .expect("failed to execute rustc");

    match expected {
        "undefined" => {
            assert!(status.success());
            return;
        }
        "error" => {
            assert!(!status.success(), "expected program to fail to compile");
            return;
        }
        _ => {
            assert!(status.success());
        }
    }

    let output = Command::new(format!("/tmp/rust-quiz/{}", stem))
        .output()
        .expect("failed to execute quiz question");
    let output_string = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, output_string, "{}", path.display());
}

enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    FilenameFormat,
    MarkdownFormat(PathBuf),
}

type Result<T> = std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            Io(e) => write!(f, "{}", e),
            Json(e) => write!(f, "{}", e),
            FilenameFormat => write!(f, "wrong filename format"),
            MarkdownFormat(path) => write!(
                f,
                "{} does not match the expected format.\n{}",
                path.display(),
                MARKDOWN_FORMAT,
            ),
        }
    }
}
