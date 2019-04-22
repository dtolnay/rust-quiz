use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

use parking_lot::Mutex;
use pulldown_cmark::{html as markdown_html, Parser as MarkdownParser};
use rayon::ThreadPoolBuilder;
use regex::Regex;
use serde::Serialize;
use termcolor::{Color, ColorSpec, WriteColor};

use crate::broker::Broker;
use crate::error::{Error, Result};

#[derive(Serialize)]
struct Question {
    code: String,
    difficulty: u8,
    answer: String,
    hint: String,
    explanation: String,
}

pub const MARKDOWN_REGEX: &str = r"(?msx)
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

pub const MARKDOWN_FORMAT: &str = "
    Answer: 999
    Difficulty: 1|2|3

    # Hint

    <!-- markdown -->

    # Explanation

    <!-- markdown -->
";

pub fn main() -> Result<()> {
    let mut question_files = Vec::new();
    for entry in fs::read_dir("questions")? {
        let entry = entry?;
        let path = entry.path();
        if path.to_string_lossy().ends_with(".rs") {
            question_files.push(path);
        }
    }
    question_files.sort();

    let cpus = num_cpus::get();
    let pool = ThreadPoolBuilder::new()
        .num_threads(cpus)
        .build()
        .map_err(Error::Rayon)?;

    let broker = Broker::new();
    let questions = Mutex::new(BTreeMap::new());
    pool.scope(|scope| {
        for _ in 0..cpus {
            scope.spawn(|_| worker(&broker, &question_files, &questions));
        }
    });

    let questions = questions.into_inner();
    if questions.len() < question_files.len() {
        // Error already printed.
        process::exit(1);
    }

    let json_object = serde_json::to_string_pretty(&questions)?;
    let javascript = format!("var questions = {};\n", json_object);
    fs::write("docs/questions.js", javascript)?;

    Ok(())
}

fn worker(broker: &Broker, files: &[PathBuf], out: &Mutex<BTreeMap<u16, Question>>) {
    loop {
        let mut task = broker.begin();
        let path = match files.get(task.index) {
            Some(path) => path,
            None => return,
        };

        writeln!(task, "evaluating {}", path.display());

        if let Err(err) = work(path, out) {
            let _ = task.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)));
            write!(task, "ERROR");
            let _ = task.set_color(ColorSpec::new().set_bold(true));
            writeln!(task, ": {}", err);
            let _ = task.reset();
        }
    }
}

fn work(path: &Path, out: &Mutex<BTreeMap<u16, Question>>) -> Result<()> {
    let code = fs::read_to_string(path)?;

    let Markdown {
        answer,
        difficulty,
        hint,
        explanation,
    } = parse_markdown(path.with_extension("md"))?;

    check_answer(path, &answer)?;

    let re = Regex::new(r"questions/([0-9]{3})[a-z0-9-]+\.rs").expect("valid regex");
    let number = match re.captures(&path.to_str().unwrap()) {
        Some(cap) => cap[1].parse::<u16>().expect("three decimal digits"),
        None => return Err(Error::FilenameFormat),
    };

    let mut map = out.lock();
    map.insert(
        number,
        Question {
            code,
            answer,
            difficulty,
            hint,
            explanation,
        },
    );

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
    let re = Regex::new(MARKDOWN_REGEX).expect("valid regex");
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
    let parser = MarkdownParser::new(markdown);
    let mut html = String::new();
    markdown_html::push_html(&mut html, parser);
    html = html.replace("<a href=\"", "<a target=\"_blank\" href=\"");
    html
}

enum Status {
    Ok,
    Err,
}

fn check_answer(path: &Path, expected: &str) -> Result<()> {
    let status = Command::new("rustc")
        .arg(path)
        .arg("--edition=2018")
        .arg("--out-dir=/tmp/rust-quiz")
        .stderr(Stdio::null())
        .status()
        .map_err(Error::Rustc)?;

    let status = match status.success() {
        true => Status::Ok,
        false => Status::Err,
    };

    match (expected, status) {
        ("undefined", Status::Ok) | ("error", Status::Err) => Ok(()),
        ("undefined", Status::Err) => Err(Error::UndefinedShouldCompile),
        ("error", Status::Ok) => Err(Error::ShouldNotCompile),
        (_, Status::Err) => Err(Error::ShouldCompile),
        (_, Status::Ok) => run(path, expected),
    }
}

fn run(path: &Path, expected: &str) -> Result<()> {
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let output = Command::new(format!("/tmp/rust-quiz/{}", stem))
        .output()
        .map_err(Error::Execute)?;
    let output = String::from_utf8(output.stdout)?;

    if output == expected {
        Ok(())
    } else {
        Err(Error::WrongOutput {
            expected: expected.to_owned(),
            output,
        })
    }
}
