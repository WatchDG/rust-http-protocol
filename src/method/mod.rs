use bytes::Bytes;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum MethodError {
    InvalidMethod,
}

impl fmt::Display for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodError::InvalidMethod => write!(f, "Invalid method."),
        }
    }
}

impl Error for MethodError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    Get,
}

pub fn align_method(vec: &[u8]) -> [u8; 3] {
    let len = vec.len();
    if len < 3 || len > 3 {
        return [0; 3];
    }
    let mut buf = [0; 3];
    buf.copy_from_slice(vec);
    buf
}

impl From<Method> for Bytes {
    fn from(s: Method) -> Self {
        match s {
            Method::Get => Bytes::from_static(b"GET"),
        }
    }
}

impl TryFrom<[u8; 3]> for Method {
    type Error = MethodError;
    fn try_from(v: [u8; 3]) -> Result<Self, Self::Error> {
        match v {
            [71, 69, 84] => Ok(Method::Get),
            _ => Err(MethodError::InvalidMethod),
        }
    }
}
