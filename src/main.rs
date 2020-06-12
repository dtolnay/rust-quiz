mod error;
mod render;
mod serve;

use crate::error::Result;
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

fn exec(f: fn() -> Result<()>) {
    if let Err(err) = f() {
        let task = Sequencer::stderr().begin();
        task.bold_color(Red);
        write!(task, "ERROR");
        task.bold();
        writeln!(task, ": {}", err);
        task.reset_color();
        process::exit(1);
    }
}

fn main() {
    exec(render::main);

    if should_serve() {
        let _ = writeln!(io::stderr());
        exec(serve::main);
    }
}
