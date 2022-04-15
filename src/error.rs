use std::io;
use std::num;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GoukiError {
    #[error("No file to read")]
    NoFileError { source: io::Error },
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    ParseIntError(#[from] num::ParseIntError),
    #[error("Gouki Error 1")]
    GoukiError1,
    #[error("Gouki Error 2")]
    GoukiError2,
    #[error("Gouki Error 3")]
    GoukiError3,
}
