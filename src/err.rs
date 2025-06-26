use std::error::Error;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// Results returned by the `build-timing` build process.
/// For more information see [`BuildTimingError`].
pub type BtResult<T> = Result<T, BuildTimingError>;

/// `build-timing` build process errors.
/// This type wraps multiple kinds of underlying errors that can occur downstream of `build-timing`, such as [`std::io::Error`].
#[derive(Debug)]
pub enum BuildTimingError {
    String(String),
}

impl BuildTimingError {
    pub fn new(err: impl Error) -> Self {
        BuildTimingError::String(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for BuildTimingError {
    fn from(e: FromUtf8Error) -> Self {
        BuildTimingError::String(e.to_string())
    }
}

impl From<std::io::Error> for BuildTimingError {
    fn from(e: std::io::Error) -> Self {
        BuildTimingError::String(e.to_string())
    }
}

impl From<String> for BuildTimingError {
    fn from(e: String) -> Self {
        BuildTimingError::String(e)
    }
}

impl From<&str> for BuildTimingError {
    fn from(e: &str) -> Self {
        BuildTimingError::String(e.to_string())
    }
}

impl From<std::env::VarError> for BuildTimingError {
    fn from(e: std::env::VarError) -> Self {
        BuildTimingError::String(e.to_string())
    }
}

impl From<std::num::ParseIntError> for BuildTimingError {
    fn from(e: std::num::ParseIntError) -> Self {
        BuildTimingError::String(e.to_string())
    }
}

impl Display for BuildTimingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildTimingError::String(err) => f.write_str(err),
        }
    }
}

impl StdError for BuildTimingError {}
