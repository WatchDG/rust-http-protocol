mod utils;

use bytes::BufMut;
use bytes::Bytes;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::ops::Add;
pub use utils::check_header_value;
pub use utils::get_header;
pub use utils::HeaderChar;

#[derive(Debug, Clone)]
pub enum HeaderError {
    InvalidHeader,
    InvalidHeaderValue,
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderError::InvalidHeader => write!(f, "Invalid header."),
            HeaderError::InvalidHeaderValue => write!(f, "Invalid header value."),
        }
    }
}

impl Error for HeaderError {}

#[derive(Debug, PartialEq, Clone, Ord, Eq, PartialOrd)]
pub enum Header {
    Host,
    Connection,
    CacheControl,
    UpgradeInsecureRequests,
    UserAgent,
    Accept,
    SecFetchSite,
    SecFetchMode,
    SecFetchDest,
    SecFetchUser,
    AcceptEncoding,
    AcceptLanguage,
    Authorization,
    ContentEncoding,
    ContentLength,
    ContentType,
    Upgrade,
    Pragma,
    Referer,
}

impl From<Header> for Bytes {
    fn from(h: Header) -> Self {
        match h {
            Header::Host => Bytes::from_static(b"Host"),
            Header::Connection => Bytes::from_static(b"Connection"),
            Header::CacheControl => Bytes::from_static(b"Cache-Control"),
            Header::UpgradeInsecureRequests => Bytes::from_static(b"Upgrade-Insecure-Requests"),
            Header::UserAgent => Bytes::from_static(b"User-Agent"),
            Header::Accept => Bytes::from_static(b"Accept"),
            Header::SecFetchSite => Bytes::from_static(b"Sec-Fetch-Site"),
            Header::SecFetchMode => Bytes::from_static(b"Sec-Fetch-Mode"),
            Header::SecFetchDest => Bytes::from_static(b"Sec-Fetch-Dest"),
            Header::SecFetchUser => Bytes::from_static(b"Sec-Fetch-User"),
            Header::AcceptEncoding => Bytes::from_static(b"Accept-Encoding"),
            Header::AcceptLanguage => Bytes::from_static(b"Accept-Language"),
            Header::Authorization => Bytes::from_static(b"Authorization"),
            Header::ContentEncoding => Bytes::from_static(b"Content-Encoding"),
            Header::ContentLength => Bytes::from_static(b"Content-Length"),
            Header::ContentType => Bytes::from_static(b"Content-Type"),
            Header::Upgrade => Bytes::from_static(b"Upgrade"),
            Header::Pragma => Bytes::from_static(b"Pragma"),
            Header::Referer => Bytes::from_static(b"Referer"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Headers {
    inner: BTreeMap<Header, Bytes>,
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            inner: BTreeMap::new(),
        }
    }

    pub fn insert<T>(&mut self, key: Header, value: T)
    where
        T: Into<Bytes>,
    {
        self.inner.insert(key, value.into());
    }

    pub fn get(&self, key: Header) -> Option<&Bytes> {
        self.inner.get(&key)
    }

    pub fn remove(&mut self, key: Header) {
        self.inner.remove(&key);
    }

    pub fn append(&mut self, mut other: Headers) {
        self.inner.append(&mut other.inner);
    }
}

impl Add<Headers> for Headers {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.append(other);
        self
    }
}

impl From<Headers> for Bytes {
    fn from(h: Headers) -> Self {
        let mut buf = Vec::<u8>::new();
        for (k, v) in h.inner {
            buf.put(Bytes::from(k));
            buf.put(Bytes::from_static(&[58u8, sp!()]));
            buf.put(v);
            buf.put(Bytes::from_static(&[cr!(), lf!()]));
        }
        Bytes::from(buf)
    }
}
