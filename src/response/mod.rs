use crate::status_code::get_reason_phrase_by_status_code;
use crate::{Body, Headers, HttpVersion, ReasonPhrase, StatusCode};
use bytes::{BufMut, Bytes};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ResponseBuilderError {
    HttpVersionIsNone,
    StatusCodeIsNone,
    ReasonPhraseIsNone,
    HeadersIsNone,
    BodyIsNone,
}

impl fmt::Display for ResponseBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseBuilderError::HttpVersionIsNone => write!(f, "HTTP version is None."),
            ResponseBuilderError::StatusCodeIsNone => write!(f, "Status code is None."),
            ResponseBuilderError::ReasonPhraseIsNone => write!(f, "Reason phrase is None."),
            ResponseBuilderError::HeadersIsNone => write!(f, "Headers is None."),
            ResponseBuilderError::BodyIsNone => write!(f, "Body is None."),
        }
    }
}

impl Error for ResponseBuilderError {}

#[derive(Debug, Clone)]
pub struct ResponseBuilder {
    http_version: Option<HttpVersion>,
    status_code: Option<StatusCode>,
    reason_phrase: Option<ReasonPhrase>,
    headers: Option<Headers>,
    body: Option<Body>,
}

impl ResponseBuilder {
    pub fn http_version(&mut self, http_version: HttpVersion) -> &mut Self {
        self.http_version = Some(http_version);
        self
    }
    pub fn status_code(&mut self, status_code: StatusCode) -> &mut Self {
        if let Some(reason_phrase) = get_reason_phrase_by_status_code(&status_code) {
            self.reason_phrase = Some(reason_phrase)
        }
        self.status_code = Some(status_code);
        self
    }
    pub fn reason_phrase(&mut self, reason_phrase: ReasonPhrase) -> &mut Self {
        self.reason_phrase = Some(reason_phrase);
        self
    }
    pub fn headers(&mut self, headers: Headers) -> &mut Self {
        self.headers = Some(headers);
        self
    }
    pub fn body(&mut self, body: Body) -> &mut Self {
        self.body = Some(body);
        self
    }
    pub fn build(self) -> Result<Response, ResponseBuilderError> {
        let http_version = self
            .http_version
            .ok_or(ResponseBuilderError::HttpVersionIsNone)?;
        let status_code = self
            .status_code
            .ok_or(ResponseBuilderError::StatusCodeIsNone)?;
        let reason_phrase = self
            .reason_phrase
            .ok_or(ResponseBuilderError::ReasonPhraseIsNone)?;
        let headers = self.headers.ok_or(ResponseBuilderError::HeadersIsNone)?;
        let body = self.body.ok_or(ResponseBuilderError::BodyIsNone)?;
        Ok(Response {
            http_version,
            status_code,
            reason_phrase,
            headers,
            body,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    http_version: HttpVersion,
    status_code: StatusCode,
    reason_phrase: ReasonPhrase,
    headers: Headers,
    body: Body,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder {
            http_version: None,
            status_code: None,
            reason_phrase: None,
            headers: None,
            body: None,
        }
    }
}

impl From<Response> for Bytes {
    fn from(r: Response) -> Self {
        let mut buf = Vec::<u8>::new();
        buf.put(Bytes::from(r.http_version));
        buf.put(Bytes::from_static(&[sp!()]));
        buf.put(Bytes::from(r.status_code));
        buf.put(Bytes::from_static(&[sp!()]));
        buf.put(Bytes::from(r.reason_phrase));
        buf.put(Bytes::from_static(&[cr!(), lf!()]));
        buf.put(Bytes::from(r.headers));
        buf.put(Bytes::from_static(&[cr!(), lf!()]));
        buf.put(Bytes::from(r.body));
        Bytes::from(buf)
    }
}
