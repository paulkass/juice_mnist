use std::convert::From;

use url::ParseError;

use std::backtrace::Backtrace;
use std::sync::PoisonError;
use coaster::SharedTensor;

#[derive(Debug)]
pub struct Error {
    message: String,
    backtrace: Backtrace,
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error {
            message: String::from(message),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error {
            message: e.to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<curl::Error> for Error {
    fn from(e: curl::Error) -> Self {
        Error {
            message: e.to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            message: e.to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<std::sync::PoisonError<&mut coaster::tensor::SharedTensor<f32>>> for Error {
    fn from(e: PoisonError<&mut SharedTensor<f32>>) -> Self {
       Error {
           message: e.to_string(),
           backtrace: Backtrace::capture()
       }
    }
}

impl From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, coaster::tensor::SharedTensor<f32>>>> for Error {
    fn from(e: PoisonError<std::sync::RwLockWriteGuard<'_, coaster::tensor::SharedTensor<f32>>>) -> Self {
	    Error {
            message: e.to_string(),
            backtrace: Backtrace::capture()
        }
    }
}
