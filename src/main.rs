mod ahead;
mod broker;
mod error;
mod render;
mod serve;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
        let mut stderr = StandardStream::stderr(ColorChoice::Auto);
        let _ = stderr.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)));
        let _ = write!(stderr, "ERROR");
        let _ = stderr.set_color(ColorSpec::new().set_bold(true));
        let _ = writeln!(stderr, ": {}", err);
        let _ = stderr.reset();
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
