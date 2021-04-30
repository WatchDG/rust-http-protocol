use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct RequestUri {
    inner: Bytes,
}

impl RequestUri {
    pub fn new<T: Into<Bytes>>(r: T) -> Self {
        Self { inner: r.into() }
    }
}

impl From<Bytes> for RequestUri {
    fn from(b: Bytes) -> Self {
        Self { inner: b }
    }
}

impl From<RequestUri> for Bytes {
    fn from(r: RequestUri) -> Self {
        r.inner
    }
}
