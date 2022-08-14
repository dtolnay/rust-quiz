#![allow(
    clippy::let_underscore_drop,
    clippy::match_bool,
    // Clippy bug: https://github.com/rust-lang/rust-clippy/issues/7422
    clippy::nonstandard_macro_braces,
)]

mod serve;

use rust_quiz::error::{Error, Result};
use rust_quiz::parser::ParseOption::RenderHtml;
use rust_quiz::render::render_questions;

use oqueue::{Color::Red, Sequencer};
use std::env;
use std::io::{self, Write};
use std::process;

fn should_serve() -> bool {
    let mut args = env::args_os().skip(1);

    let arg = match args.next() {
        Some(arg) => arg,
        None => return false,
    };

    if arg == "serve" {
        true
    } else {
        let _ = writeln!(
            io::stderr(),
            "Unrecognized argument: `{}`",
            arg.to_string_lossy()
        );
        process::exit(1);
    }
}

fn report(result: Result<()>) {
    if let Err(err) = result {
        let task = Sequencer::stderr().begin();
        task.bold_color(Red);
        write!(task, "ERROR");
        task.bold();
        writeln!(task, ": {}", err);
        task.reset_color();
        process::exit(1);
    }
}

#[tokio::main]
async fn main() {
    report(render_main());

    if should_serve() {
        let _ = writeln!(io::stderr());
        report(serve::main().await);
    }
}

fn render_main() -> Result<()> {
    let questions = render_questions(RenderHtml)?;

    let json_object = serde_json::to_string_pretty(&questions)?;
    let javascript = format!("var questions = {};\n", json_object);
    std::fs::write("docs/questions.js", javascript)?;

    Ok(())
}
