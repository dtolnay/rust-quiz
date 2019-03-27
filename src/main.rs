mod ahead;
mod broker;
mod error;
mod render;
mod serve;

use colored::Colorize;

use std::env;
use std::io::{self, Write};
use std::process;

use crate::error::Result;

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
        let _ = writeln!(
            io::stderr(),
            "{error}{colon} {message}",
            error = "ERROR".bold().red(),
            colon = ":".bold(),
            message = err.to_string().bold(),
        );
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
