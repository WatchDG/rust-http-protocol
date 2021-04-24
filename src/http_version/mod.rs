use bytes::Bytes;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum HttpVersionError {
    InvalidHttpVersion,
}

impl fmt::Display for HttpVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpVersionError::InvalidHttpVersion => write!(f, "Invalid HTTP version."),
        }
    }
}

impl Error for HttpVersionError {}

#[derive(Debug, PartialEq, Clone)]
pub enum HttpVersion {
    Http11,
}

impl From<HttpVersion> for Bytes {
    fn from(v: HttpVersion) -> Self {
        match v {
            HttpVersion::Http11 => Bytes::from_static(&[72u8, 84, 84, 80, 47, 49, 46, 49]),
        }
    }
}

impl TryInto<HttpVersion> for &[u8] {
    type Error = HttpVersionError;

    fn try_into(self) -> Result<HttpVersion, Self::Error> {
        match self {
            [72, 84, 84, 80, 47, 49, 46, 49] => Ok(HttpVersion::Http11),
            _ => Err(HttpVersionError::InvalidHttpVersion),
        }
    }
}
