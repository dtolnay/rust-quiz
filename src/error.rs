use rayon::ThreadPoolBuildError;
use std::io;
use std::path::PathBuf;
use std::string::FromUtf8Error;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[remain::sorted]
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to execute quiz question: {0}")]
    Execute(io::Error),

    #[error("wrong filename format")]
    FilenameFormat,

    #[error(transparent)]
    Http(#[from] http::Error),

    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(
        "{0} does not match the expected format.\n{}",
        crate::render::MARKDOWN_FORMAT
    )]
    MarkdownFormat(PathBuf),

    #[error(transparent)]
    Rayon(ThreadPoolBuildError),

    #[error("failed to execute rustc: {0}")]
    Rustc(io::Error),

    #[error("program failed to compile")]
    ShouldCompile,

    #[error("program should fail to compile")]
    ShouldNotCompile,

    #[error("program with undefined behavior should compile")]
    UndefinedShouldCompile,

    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),

    #[error("wrong output! expected: {expected}, actual: {output}")]
    WrongOutput { expected: String, output: String },
}
