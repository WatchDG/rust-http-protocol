use bytes::Bytes;

#[derive(Debug, Clone)]
enum BodyInner {
    Empty,
    Bytes(Bytes),
}

#[derive(Debug, Clone)]
pub struct Body {
    inner: BodyInner,
}

impl Body {
    pub fn empty() -> Self {
        Self {
            inner: BodyInner::Empty,
        }
    }
}

impl From<Bytes> for Body {
    fn from(bytes: Bytes) -> Self {
        let inner = if bytes.is_empty() {
            BodyInner::Empty
        } else {
            BodyInner::Bytes(bytes)
        };

        Self { inner }
    }
}

impl From<Body> for Bytes {
    fn from(b: Body) -> Self {
        match b.inner {
            BodyInner::Empty => Bytes::new(),
            BodyInner::Bytes(bytes) => bytes,
        }
    }
}
