use crate::error::{Error, Result};
use crate::parser::{parse_markdown, Markdown, ParseOption, Question};
use oqueue::{Color::Red, Sequencer};
use parking_lot::Mutex;
use rayon::ThreadPoolBuilder;
use regex::Regex;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

/// Parse markdown files into questions.
pub fn render_questions(option: ParseOption) -> Result<BTreeMap<u16, Question>> {
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
            scope.spawn(|_| worker(&oqueue, &question_files, &questions, option.clone()));
        }
    });

    let questions = questions.into_inner();
    if questions.len() < question_files.len() {
        // Error already printed.
        process::exit(1);
    }

    Ok(questions)
}

fn worker(
    oqueue: &Sequencer,
    files: &[PathBuf],
    out: &Mutex<BTreeMap<u16, Question>>,
    option: ParseOption,
) {
    loop {
        let task = oqueue.begin();
        let path = match files.get(task.index) {
            Some(path) => path,
            None => return,
        };

        writeln!(task, "evaluating {}", path.display());

        if let Err(err) = work(path, out, option.clone()) {
            task.bold_color(Red);
            write!(task, "ERROR");
            task.bold();
            writeln!(task, ": {}", err);
        }
    }
}

fn work(path: &Path, out: &Mutex<BTreeMap<u16, Question>>, option: ParseOption) -> Result<()> {
    let code = fs::read_to_string(path)?;
    let content = fs::read_to_string(&path.with_extension("md"))?;

    let Markdown {
        answer,
        difficulty,
        hint,
        explanation,
    } = parse_markdown(content, option)?;

    check_answer(path, &answer)?;

    let re = Regex::new(r"questions/([0-9]{3})[a-z0-9-]+\.rs").expect("valid regex");
    let number = match re.captures(path.to_str().unwrap()) {
        Some(cap) => cap[1].parse::<u16>().expect("three decimal digits"),
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
