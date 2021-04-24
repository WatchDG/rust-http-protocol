use crate::{Headers, HttpVersion, Method};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Request<UT> {
    method: Method,
    request_uri: UT,
    http_version: HttpVersion,
    headers: Headers,
}

#[derive(Debug, Clone)]
pub struct RequestPart<UT> {
    method: Option<Method>,
    request_uri: Option<UT>,
    http_version: Option<HttpVersion>,
    headers: Option<Headers>,
}

impl<UT> Add<RequestPart<UT>> for RequestPart<UT> {
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
        self
    }
}
