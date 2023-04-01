use crate::error::Error;
use oqueue::{Color::Red, Sequencer};
use parking_lot::Mutex;
use pulldown_cmark::{html as markdown_html, Parser as MarkdownParser};
use rayon::ThreadPoolBuilder;
use regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::env::consts::EXE_EXTENSION;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

#[derive(Serialize)]
struct Question {
    code: String,
    difficulty: u8,
    answer: String,
    hint: String,
    explanation: String,
}

pub const MARKDOWN_REGEX: &str = r"(?msx)
    \AAnswer:\x20(?P<answer>undefined|error|[0-9]+)\n
    Difficulty:\x20(?P<difficulty>1|2|3)\n
    (?:Warnings:\x20(?P<warnings>[a-z_,\x20]+)\n
    )?\n
    \x23\x20Hint\n
    \n
    (?P<hint>.*)
    \n
    \x23\x20Explanation\n
    \n
    (?P<explanation>.*)
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

pub fn main() -> Result<(), Error> {
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

    let oqueue = Sequencer::stderr();
    let questions = Mutex::new(BTreeMap::new());
    pool.scope(|scope| {
        for _ in 0..cpus {
            scope.spawn(|_| worker(&oqueue, &question_files, &questions));
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

fn worker(oqueue: &Sequencer, files: &[PathBuf], out: &Mutex<BTreeMap<u16, Question>>) {
    loop {
        let task = oqueue.begin();
        let Some(rs_path) = files.get(task.index) else {
            return;
        };

        writeln!(task, "evaluating {}", rs_path.display());

        if let Err(err) = work(rs_path, out) {
            task.bold_color(Red);
            write!(task, "ERROR");
            task.bold();
            writeln!(task, ": {}", err);
        }
    }
}

fn work(rs_path: &Path, out: &Mutex<BTreeMap<u16, Question>>) -> Result<(), Error> {
    let code = fs::read_to_string(rs_path)?;

    let md_path = rs_path.with_extension("md");
    let md_content = fs::read_to_string(&md_path)?;
    let markdown_regex = Regex::new(MARKDOWN_REGEX).expect("valid regex");
    let Some(markdown_cap) = markdown_regex.captures(&md_content) else {
        return Err(Error::MarkdownFormat(md_path));
    };

    let mut warnings = Vec::new();
    if let Some(regex_match) = markdown_cap.name("warnings") {
        for word in regex_match.as_str().split(',') {
            warnings.push(word.trim().to_owned());
        }
    }

    let answer = markdown_cap["answer"].to_owned();
    let difficulty = markdown_cap["difficulty"].parse().unwrap();
    let hint = render_to_html(&markdown_cap["hint"]);
    let explanation = render_to_html(&markdown_cap["explanation"]);

    check_answer(rs_path, &answer, &warnings)?;

    let path_regex = Regex::new(r"questions/(?P<num>[0-9]{3})[a-z0-9-]+\.rs").expect("valid regex");
    let number = match path_regex.captures(rs_path.to_str().unwrap()) {
        Some(path_cap) => path_cap["num"]
            .parse::<u16>()
            .expect("three decimal digits"),
        None => return Err(Error::FilenameFormat),
    };

    let mut map = out.lock();
    map.insert(
        number,
        Question {
            code,
            difficulty,
            answer,
            hint,
            explanation,
        },
    );

    Ok(())
}

fn render_to_html(markdown: &str) -> String {
    let parser = MarkdownParser::new(markdown);
    let mut html = String::new();
    markdown_html::push_html(&mut html, parser);
    html = html.replace("<a href=\"", "<a target=\"_blank\" href=\"");
    html
}

#[derive(Copy, Clone)]
enum Status {
    Ok,
    Err,
}

fn check_answer(rs_path: &Path, expected: &str, warnings: &[String]) -> Result<(), Error> {
    let out_dir = env::temp_dir().join("rust-quiz");

    let mut cmd = rustc(&out_dir, rs_path);
    cmd.arg("--deny=warnings");
    for warning in warnings {
        cmd.arg("--allow").arg(warning);
    }

    let status = cmd.status().map_err(Error::Rustc)?;
    let status = match status.success() {
        true => Status::Ok,
        false => Status::Err,
    };

    if let Status::Err = status {
        if rustc(&out_dir, rs_path)
            .arg("--allow=warnings")
            .status()
            .map_err(Error::Rustc)?
            .success()
        {
            return Err(Error::CompiledWithWarnings);
        }
    }

    match (expected, status) {
        ("undefined", Status::Ok) | ("error", Status::Err) => {}
        ("undefined", Status::Err) => return Err(Error::UndefinedShouldCompile),
        ("error", Status::Ok) => return Err(Error::ShouldNotCompile),
        (_, Status::Err) => return Err(Error::ShouldCompile),
        (_, Status::Ok) => run(&out_dir, rs_path, expected)?,
    }

    if let Status::Ok = status {
        let mut missing_warnings = Vec::new();
        for check_warning in warnings {
            let mut cmd = rustc(&out_dir, rs_path);
            cmd.arg("--deny=warnings");
            for warning in warnings {
                if warning != check_warning {
                    cmd.arg("--allow").arg(warning);
                }
            }
            if cmd.status().map_err(Error::Rustc)?.success() {
                missing_warnings.push(check_warning.clone());
            }
        }
        if !missing_warnings.is_empty() {
            return Err(Error::MissingExpectedWarning(missing_warnings));
        }
    }

    Ok(())
}

fn rustc(out_dir: &Path, rs_path: &Path) -> Command {
    let mut cmd = Command::new("rustc");
    cmd.arg(rs_path)
        .arg("--edition=2021")
        .arg("--out-dir")
        .arg(out_dir)
        .stderr(Stdio::null());
    cmd
}

fn run(out_dir: &Path, rs_path: &Path, expected: &str) -> Result<(), Error> {
    let stem = rs_path.file_stem().unwrap();
    let exe = out_dir.join(stem).with_extension(EXE_EXTENSION);
    let output = Command::new(exe).output().map_err(Error::Execute)?;
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
