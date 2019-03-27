mod ahead;
mod broker;
mod error;
mod render;

use std::io::{self, Write};
use std::process;

fn main() {
    if let Err(err) = render::main() {
        let _ = writeln!(io::stderr(), "ERROR: {}", err);
        process::exit(1);
    }
}
