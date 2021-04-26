use bytes::Bytes;
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
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl From<Method> for Bytes {
    fn from(s: Method) -> Self {
        match s {
            Method::Get => Bytes::from_static(b"GET"),
            Method::Head => Bytes::from_static(b"HEAD"),
            Method::Post => Bytes::from_static(b"POST"),
            Method::Put => Bytes::from_static(b"PUT"),
            Method::Delete => Bytes::from_static(b"DELETE"),
            Method::Connect => Bytes::from_static(b"CONNECT"),
            Method::Options => Bytes::from_static(b"OPTIONS"),
            Method::Trace => Bytes::from_static(b"TRACE"),
            Method::Patch => Bytes::from_static(b"PATCH"),
        }
    }
}

#[inline]
pub fn get_method(v: &[u8]) -> Result<Method, MethodError> {
    if v.len() < 3 || v.len() > 7 {
        return Err(MethodError::InvalidMethod);
    }

    let mut buf = [0u8; 8];
    &buf[0..v.len()].copy_from_slice(v);

    let val = u64::from_be_bytes(buf);

    match val {
        0x4745540000000000 => Ok(Method::Get),
        0x4845414400000000 => Ok(Method::Head),
        0x504f535400000000 => Ok(Method::Post),
        0x5055540000000000 => Ok(Method::Put),
        0x44454c4554450000 => Ok(Method::Delete),
        0x434f4e4e45435400 => Ok(Method::Connect),
        0x4f5054494f4e5300 => Ok(Method::Options),
        0x5452414345000000 => Ok(Method::Trace),
        0x5041544348000000 => Ok(Method::Patch),
        _ => Err(MethodError::InvalidMethod),
    }
}
