use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    #[error("Test error: {0}")]
    Test(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Error {
    pub fn http<S: Into<String>>(msg: S) -> Self {
        Error::Http(msg.into())
    }

    pub fn parse<S: Into<String>>(msg: S) -> Self {
        Error::Parse(msg.into())
    }

    pub fn config<S: Into<String>>(msg: S) -> Self {
        Error::Config(msg.into())
    }

    pub fn assertion<S: Into<String>>(msg: S) -> Self {
        Error::AssertionFailed(msg.into())
    }

    pub fn test<S: Into<String>>(msg: S) -> Self {
        Error::Test(msg.into())
    }
}
