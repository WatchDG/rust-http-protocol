mod utils;

use std::collections::BTreeMap;
use std::ops::Add;
pub use utils::get_header_enum;
pub use utils::is_allowed_header_value;
pub use utils::HeaderChar;

#[derive(Debug, PartialEq, Clone, Ord, Eq, PartialOrd)]
pub enum Header {
    Null,
    Host,
    Connection,
    CacheControl,
    UpgradeInsecureRequests,
    UserAgent,
    Accept,
    SecFetchSite,
    SecFetchMode,
    SecFetchDest,
    AcceptEncoding,
    AcceptLanguage,
}

#[derive(Debug, Clone)]
pub struct Headers<T> {
    inner: BTreeMap<Header, T>,
}

impl<T> Headers<T> {
    pub fn new() -> Self {
        Headers {
            inner: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: Header, value: T) {
        self.inner.insert(key, value);
    }

    pub fn get(&self, key: Header) -> Option<&T> {
        self.inner.get(&key)
    }

    pub fn remove(&mut self, key: Header) {
        self.inner.remove(&key);
    }

    pub fn append(&mut self, mut other: Headers<T>) {
        self.inner.append(&mut other.inner);
    }
}

impl<T> Add<Headers<T>> for Headers<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.append(other);
        self
    }
}
