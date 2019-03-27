use std::fmt::{self, Display};
use std::io;
use std::path::PathBuf;
use std::string::FromUtf8Error;

use rayon::ThreadPoolBuildError;

#[remain::sorted]
pub enum Error {
    Execute(io::Error),
    FilenameFormat,
    Io(io::Error),
    Json(serde_json::Error),
    MarkdownFormat(PathBuf),
    Rayon(ThreadPoolBuildError),
    Rustc(io::Error),
    ShouldCompile,
    ShouldNotCompile,
    UndefinedShouldCompile,
    Utf8(FromUtf8Error),
    WrongOutput { expected: String, output: String },
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    #[remain::check]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        #[sorted]
        match self {
            Execute(e) => write!(f, "failed to execute quiz question: {}", e),
            FilenameFormat => write!(f, "wrong filename format"),
            Io(e) => write!(f, "{}", e),
            Json(e) => write!(f, "{}", e),
            MarkdownFormat(path) => write!(
                f,
                "{} does not match the expected format.\n{}",
                path.display(),
                crate::render::MARKDOWN_FORMAT,
            ),
            Rayon(e) => write!(f, "{}", e),
            Rustc(e) => write!(f, "failed to execute rustc: {}", e),
            ShouldCompile => write!(f, "program failed to compile"),
            ShouldNotCompile => write!(f, "program should fail to compile"),
            UndefinedShouldCompile => write!(f, "program with undefined behavior should compile"),
            Utf8(e) => write!(f, "{}", e),
            WrongOutput { expected, output } => write!(
                f,
                "wrong output! expected: {}, actual: {}",
                expected, output
            ),
        }
    }
}

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

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::Utf8(err)
    }
}
