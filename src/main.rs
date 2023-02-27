#![allow(
    clippy::let_underscore_untyped,
    clippy::match_bool,
    clippy::uninlined_format_args
)]

mod error;
mod render;
mod serve;

use crate::error::{Error, Result};
use oqueue::{Color::Red, Sequencer};
use std::env;
use std::io::{self, Write};
use std::process;

fn should_serve() -> bool {
    let mut args = env::args_os().skip(1);

    let Some(arg) = args.next() else {
        return false;
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
    report(render::main());

    if should_serve() {
        let _ = writeln!(io::stderr());
        report(serve::main().await);
    }
}
