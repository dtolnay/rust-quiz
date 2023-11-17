use rayon::ThreadPoolBuildError;
use std::fmt::{self, Display};
use std::io;
use std::path::PathBuf;
use std::string::FromUtf8Error;
use thiserror::Error;

#[remain::sorted]
#[derive(Error, Debug)]
pub enum Error {
    #[error("program compiled with warnings; make sure every expected warning is listed in a 'Warnings:' section")]
    CompiledWithWarnings,

    #[error("failed to execute quiz question: {0}")]
    Execute(io::Error),

    #[error("wrong filename format")]
    FilenameFormat,

    #[error(transparent)]
    Http(#[from] http::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(
        "{0} does not match the expected format.\n{}",
        crate::render::MARKDOWN_FORMAT
    )]
    MarkdownFormat(PathBuf),

    #[error("program compiled without expected warning: {}", CommaSep(.0))]
    MissingExpectedWarning(Vec<String>),

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

struct CommaSep<'a>(&'a [String]);

impl<'a> Display for CommaSep<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for (i, string) in self.0.iter().enumerate() {
            if i > 0 {
                formatter.write_str(", ")?;
            }
            formatter.write_str(string)?;
        }
        Ok(())
    }
}
