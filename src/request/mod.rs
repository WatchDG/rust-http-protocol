use crate::{Body, Headers, HttpVersion, Method, RequestUri};
use std::error::Error;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum RequestError {
    MethodIsNone,
    RequestUriIsNone,
    HttpVersionIsNone,
    HeadersIsNone,
    BodyIsNone,
    InvalidRequestLine,
    InvalidHeader,
    InvalidHeaderValue,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::MethodIsNone => write!(f, "Method is None."),
            RequestError::RequestUriIsNone => write!(f, "Request URI is None."),
            RequestError::HttpVersionIsNone => write!(f, "HTTP version is None."),
            RequestError::HeadersIsNone => write!(f, "Headers is None."),
            RequestError::BodyIsNone => write!(f, "Body is None."),
            RequestError::InvalidRequestLine => write!(f, "Invalid request line."),
            RequestError::InvalidHeader => write!(f, "Invalid header."),
            RequestError::InvalidHeaderValue => write!(f, "Invalid header value."),
        }
    }
}

impl Error for RequestError {}

#[derive(Debug, Clone)]
pub struct RequestPart {
    pub method: Option<Method>,
    pub request_uri: Option<RequestUri>,
    pub http_version: Option<HttpVersion>,
    pub headers: Option<Headers>,
    pub body: Option<Body>,
}

impl RequestPart {
    pub fn build(self) -> Result<Request, RequestError> {
        let method = self.method.ok_or(RequestError::MethodIsNone)?;
        let request_uri = self.request_uri.ok_or(RequestError::RequestUriIsNone)?;
        let http_version = self.http_version.ok_or(RequestError::HttpVersionIsNone)?;
        let headers = self.headers.ok_or(RequestError::HeadersIsNone)?;
        let body = self.body.ok_or(RequestError::BodyIsNone)?;
        Ok(Request {
            method,
            request_uri,
            http_version,
            headers,
            body,
        })
    }

    #[inline]
    pub fn empty() -> RequestPart {
        RequestPart {
            method: None,
            request_uri: None,
            http_version: None,
            headers: None,
            body: None,
        }
    }
}

impl Add<RequestPart> for RequestPart {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self.method = self.method.or(other.method);
        self.request_uri = self.request_uri.or(other.request_uri);
        self.http_version = self.http_version.or(other.http_version);
        self.headers = match (self.headers, other.headers) {
            (a, None) => a,
            (None, b) => b,
            (Some(a), Some(b)) => Some(a + b),
        };
        self.body = match (self.body, other.body) {
            (a, None) => a,
            (None, b) => b,
            (Some(a), Some(b)) => Some(a + b),
        };
        self
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    request_uri: RequestUri,
    http_version: HttpVersion,
    headers: Headers,
    body: Body,
}
