use std::convert::From;
use std::string::FromUtf8Error;
use std::error;
use std::result;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ClassError {
    IOError,
    MalformedUtf8String,
    InvalidConstantPoolTag(u8)
}

impl fmt::Display for ClassError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BOOM")
    }
}

impl error::Error for ClassError {
    fn description(&self) -> &str {
        "BOOM"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<io::Error> for ClassError {
    fn from(_: io::Error) -> ClassError {
        ClassError::IOError
    }
}

impl From<FromUtf8Error> for ClassError {
    fn from(_: FromUtf8Error) -> ClassError {
        ClassError::MalformedUtf8String
    }
}

pub type ClassResult<T> = result::Result<T, ClassError>;
