use crate::http_version::get_http_version;
use crate::method::get_method;
use crate::request::{RequestError, RequestPart};
use crate::{Method, RequestUri};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum ParsePosition {
    RequestLine,
    Headers,
    Body,
    RequestEnd,
}

#[inline]
pub fn parse_request_line(
    stop_position: &mut ParsePosition,
    buffer: &mut Vec<u8>,
    mut index: &mut usize,
) -> Result<RequestPart, Box<dyn Error>> {
    // meta[0] - sp1
    // meta[1] - sp2
    // meta[2] - request line end
    let mut meta = [0usize; 3];
    let mut meta_count = 0;

    for i in *index..buffer.len() {
        if buffer[i] == sp!() {
            meta[meta_count % 2] = i;
            meta_count += 1;
            if meta_count > 2 {
                return Err(RequestError::InvalidRequestLine.into());
            }
            continue;
        }
        if buffer[i] == lf!() {
            if (i - *index) < 14 {
                return Err(RequestError::InvalidRequestLine.into());
            }
            if buffer[i - 1] == cr!() {
                meta[2] = i - 2;
                break;
            }
            return Err(RequestError::InvalidRequestLine.into());
        }
    }

    if meta[2] == 0 {
        return Ok(RequestPart::empty());
    }

    if !(meta[0] > *index && meta[1] > meta[0] && meta[2] > meta[1]) {
        return Err(RequestError::InvalidRequestLine.into());
    }

    let method = get_method(&buffer[*index..meta[0]])?;
    let http_version = get_http_version(&buffer[(meta[1] + 1)..=meta[2]])?;
    let request_uri = RequestUri::new(buffer[(meta[0] + 1)..meta[1]].to_owned());

    *stop_position = match method {
        Method::Get => ParsePosition::Body,
        Method::Post => ParsePosition::RequestEnd,
        _ => ParsePosition::Body,
    };

    *index += meta[2] + 3;

    Ok(RequestPart {
        method: Some(method),
        request_uri: Some(request_uri),
        http_version: Some(http_version),
        headers: None,
        body: None,
    })
}
