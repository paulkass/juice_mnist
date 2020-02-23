use std::convert::From;

use url::ParseError;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error {
            message: String::from(message),
        }
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error {
            message: e.to_string(),
        }
    }
}

impl From<curl::Error> for Error {
    fn from(e: curl::Error) -> Self {
        Error {
            message: e.to_string(),
        }
    }
}
