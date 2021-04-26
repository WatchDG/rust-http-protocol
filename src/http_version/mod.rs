use bytes::Bytes;
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
    Http10,
    Http11,
}

impl HttpVersion {
    #[inline]
    pub fn major(&self) -> u8 {
        match self {
            HttpVersion::Http10 => 1,
            HttpVersion::Http11 => 1,
        }
    }

    #[inline]
    pub fn minor(&self) -> u8 {
        match self {
            HttpVersion::Http10 => 0,
            HttpVersion::Http11 => 1,
        }
    }
}

impl From<HttpVersion> for Bytes {
    #[inline]
    fn from(v: HttpVersion) -> Self {
        match v {
            HttpVersion::Http10 => Bytes::from_static(b"HTTP/1.0"),
            HttpVersion::Http11 => Bytes::from_static(b"HTTP/1.1"),
        }
    }
}

#[inline]
pub fn get_http_version(v: &[u8]) -> Result<HttpVersion, HttpVersionError> {
    if v.len() != 8 {
        return Err(HttpVersionError::InvalidHttpVersion);
    }

    let mut buf = [0u8; 8];
    buf.copy_from_slice(v);

    let val = u64::from_be_bytes(buf);

    match val {
        0x485454502f312e30 => Ok(HttpVersion::Http10),
        0x485454502f312e31 => Ok(HttpVersion::Http11),
        _ => Err(HttpVersionError::InvalidHttpVersion),
    }
}
