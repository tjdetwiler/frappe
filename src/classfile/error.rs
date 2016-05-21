use std::convert::From;
use std::string::FromUtf8Error;
use std::error;
use std::result;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IOError,
    MalformedUtf8String,
    InvalidConstantPoolTag(u8),
    InvalidTargetTypeTag(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BOOM")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "BOOM"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error::IOError
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Error {
        Error::MalformedUtf8String
    }
}

pub type Result<T> = result::Result<T, Error>;
