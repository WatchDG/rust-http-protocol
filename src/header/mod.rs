mod utils;

use bytes::BufMut;
use bytes::Bytes;
use std::collections::BTreeMap;
use std::ops::Add;
pub use utils::get_header_enum;
pub use utils::is_allowed_header_value;
pub use utils::HeaderChar;

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
