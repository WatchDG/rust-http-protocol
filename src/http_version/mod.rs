use bytes::Bytes;
use std::convert::{TryFrom, TryInto};
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

impl TryFrom<[u8; 8]> for HttpVersion {
    type Error = HttpVersionError;

    fn try_from(v: [u8; 8]) -> Result<Self, Self::Error> {
        match v {
            [72, 84, 84, 80, 47, 49, 46, 49] => Ok(HttpVersion::Http11),
            _ => Err(HttpVersionError::InvalidHttpVersion),
        }
    }
}

pub fn get_http_version(v: &[u8]) -> Result<HttpVersion, HttpVersionError> {
    if v.len() != 8 {
        return Err(HttpVersionError::InvalidHttpVersion);
    }

    let mut buf = [0u8; 8];
    buf.copy_from_slice(v);

    let val = u64::from_be_bytes(buf);

    match val {
        0x485454502f312e31 => Ok(HttpVersion::Http11),
        _ => Err(HttpVersionError::InvalidHttpVersion),
    }
}
