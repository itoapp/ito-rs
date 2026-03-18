use std::fmt;

#[derive(Debug)]
pub enum Error {
    Net(String),
    Html(String),
    Postcard(postcard::Error),
    Utf8(std::string::FromUtf8Error),
    Host(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Net(msg) => write!(f, "Network error: {}", msg),
            Error::Html(msg) => write!(f, "HTML parsing error: {}", msg),
            Error::Postcard(e) => write!(f, "Serialization error: {}", e),
            Error::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            Error::Host(msg) => write!(f, "Host error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<postcard::Error> for Error {
    fn from(e: postcard::Error) -> Self {
        Error::Postcard(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::Utf8(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
