use std::{fmt::Display, convert::Infallible, any::Any};

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Self(ioe.to_string())
    }
}

impl From<Box<dyn Any + Send>> for Error {
    fn from(_: Box<dyn Any + Send>) -> Self {
        Self("unknown thread error".to_string())
    }
}

impl std::error::Error for Error {}
unsafe impl Sync for Error {}
unsafe impl Send for Error {}

pub type Result<T> = std::result::Result<T, Error>;
