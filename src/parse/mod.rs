use crate::header::{check_header_value, get_header};
use crate::http_version::get_http_version;
use crate::method::parse_method;
// use crate::parse::ParsePosition::Headers;
use crate::request::{RequestBuilder, RequestError};
use crate::{Headers, Method, RequestUri};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum ParsePosition {
    RequestLine,
    Headers,
    Body,
    RequestEnd,
}

#[inline]
fn parse_request_line(
    current_position: &mut ParsePosition,
    stop_position: &mut ParsePosition,
    buffer: &mut Vec<u8>,
    index: &mut usize,
) -> Result<RequestBuilder, Box<dyn Error>> {
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
        return Ok(RequestBuilder::empty());
    }

    if !(meta[0] > *index && meta[1] > meta[0] && meta[2] > meta[1]) {
        return Err(RequestError::InvalidRequestLine.into());
    }

    let method = parse_method(&buffer[*index..meta[0]])?;
    let http_version = get_http_version(&buffer[(meta[1] + 1)..=meta[2]])?;
    let request_uri = RequestUri::new(buffer[(meta[0] + 1)..meta[1]].to_owned());

    *current_position = ParsePosition::Headers;

    *stop_position = match method {
        Method::Get => ParsePosition::Body,
        Method::Post => ParsePosition::RequestEnd,
        _ => ParsePosition::Body,
    };

    *index += meta[2] + 3;

    Ok(RequestBuilder {
        method: Some(method),
        request_uri: Some(request_uri),
        http_version: Some(http_version),
        headers: None,
        body: None,
    })
}

#[test]
fn parse_request_line_0() {
    let reference = RequestBuilder {
        method: Some(Method::Get),
        request_uri: Some(RequestUri::new(b"/path".to_vec())),
        http_version: Some(HttpVersion::Http11),
        headers: None,
        body: None,
    };

    let mut current_position = ParsePosition::RequestLine;
    let mut stop_position = ParsePosition::RequestEnd;
    let mut index = 0usize;
    let mut buffer = b"GET /path HTTP/1.1\r\n".to_vec();
    let request_part = parse_request_line(
        &mut current_position,
        &mut stop_position,
        &mut buffer,
        &mut index,
    )
    .unwrap();
    assert_eq!(request_part, reference);
}

#[inline]
fn check_min_header_length(s_idx: usize, e_idx: usize) -> Result<(), Box<dyn Error>> {
    if e_idx - s_idx < 5 {
        return Err(RequestError::InvalidHeader.into());
    }
    Ok(())
}

fn parse_headers(
    current_position: &mut ParsePosition,
    buffer: &mut Vec<u8>,
    index: &mut usize,
) -> Result<RequestBuilder, Box<dyn Error>> {
    let mut index_pairs = Vec::<(usize, usize)>::new();

    let mut s_idx = *index;
    let mut e_idx = *index;

    for i in *index..buffer.len() {
        if buffer[i] == lf!() {
            if buffer[i - 1] != cr!() {
                return Err(RequestError::InvalidHeader.into());
            }
            if buffer[i - 2] == lf!() {
                break;
            }
            e_idx = i - 2;
            check_min_header_length(s_idx, e_idx)?;
            index_pairs.push((s_idx, e_idx));
            s_idx = i + 1;
            e_idx = i + 1;
        }
    }

    println!("{:?}", index_pairs);

    let mut headers = Headers::new();

    for (s_idx, e_idx) in index_pairs {
        let header = &buffer[s_idx..=e_idx];

        let mut m = 0;
        for i in 0..header.len() {
            if header[i] == (':' as u8) {
                m = i - 1;
                break;
            }
        }

        let header_name = &header[0..=m];
        let header_value = &header[m + 3..];
        let header_enum = get_header(header_name)?;
        check_header_value(header_value)?;
        headers.insert(header_enum, header_value.to_vec());
    }

    if *index - e_idx > 0 && buffer[e_idx] == lf!() && buffer[e_idx + 1] == cr!() {
        *current_position = ParsePosition::Body;
        *index = e_idx + 2;
    }

    Ok(RequestBuilder {
        method: None,
        request_uri: None,
        http_version: None,
        headers: Some(headers),
        body: None,
    })
}

pub fn parse_request(
    current_position: &mut ParsePosition,
    stop_position: &mut ParsePosition,
    buffer: &mut Vec<u8>,
) -> Result<RequestBuilder, Box<dyn Error>> {
    let mut index = 0;

    let mut request_part = RequestBuilder::empty();

    if *current_position == ParsePosition::RequestLine {
        request_part =
            request_part + parse_request_line(current_position, stop_position, buffer, &mut index)?;
    }

    if *current_position == ParsePosition::Headers {
        request_part = request_part + parse_headers(current_position, buffer, &mut index)?;
    }

    *buffer = buffer.split_off(index);

    Ok(request_part)
}
