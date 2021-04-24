use bytes::BufMut;
use bytes::Bytes;
use std::ops::Add;

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

impl Add<Body> for Body {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        match (self.inner, other.inner) {
            (BodyInner::Empty, BodyInner::Bytes(b)) => b.into(),
            (BodyInner::Bytes(a), BodyInner::Empty) => a.into(),
            (BodyInner::Bytes(a), BodyInner::Bytes(b)) => {
                let mut buf = Vec::<u8>::with_capacity(a.len() + b.len());
                buf.put(a);
                buf.put(b);
                Bytes::from(buf).into()
            }
            _ => Self {
                inner: BodyInner::Empty,
            },
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
