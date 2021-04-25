use crate::header::{Header, HeaderError};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
pub struct HeaderChar {
    char_code: u8,
    next_char: Vec<HeaderChar>,
    header: Option<Header>,
}

fn add_header(header_chars: &mut Vec<HeaderChar>, header: &[u8], header_enum: Header) {
    let mut ptr = header_chars;
    let header_enum_value = header.to_ascii_lowercase();
    let end_idx = header_enum_value.len() - 1;
    for (idx, char_code) in header_enum_value.iter().enumerate() {
        let mut search_result =
            ptr.binary_search_by(|header_char| (header_char.char_code).cmp(char_code));
        if search_result.is_err() {
            let header = if idx == end_idx {
                Some(header_enum.clone())
            } else {
                None
            };
            ptr.push(HeaderChar {
                char_code: *char_code,
                next_char: Vec::new(),
                header,
            });
            ptr.sort_unstable_by(|a, b| a.char_code.cmp(&b.char_code));
            search_result =
                ptr.binary_search_by(|header_char| (header_char.char_code).cmp(char_code));
        }
        let index = search_result.unwrap();
        ptr = &mut ptr[index].next_char;
    }
}

lazy_static! {
    pub static ref HEADER_CHARS: Vec<HeaderChar> = {
        let mut h = Vec::<HeaderChar>::new();
        add_header(&mut h, b"Referer", Header::Referer);
        add_header(&mut h, b"Pragma", Header::Pragma);
        add_header(&mut h, b"Upgrade", Header::Upgrade);
        add_header(&mut h, b"Content-Type", Header::ContentType);
        add_header(&mut h, b"Content-Length", Header::ContentLength);
        add_header(&mut h, b"Content-Encoding", Header::ContentEncoding);
        add_header(&mut h, b"Authorization", Header::Authorization);
        add_header(&mut h, b"Accept", Header::Accept);
        add_header(&mut h, b"Accept-Encoding", Header::AcceptEncoding);
        add_header(&mut h, b"Accept-Language", Header::AcceptLanguage);
        add_header(&mut h, b"Sec-Fetch-Dest", Header::SecFetchDest);
        add_header(&mut h, b"Sec-Fetch-User", Header::SecFetchUser);
        add_header(&mut h, b"Sec-Fetch-Site", Header::SecFetchSite);
        add_header(&mut h, b"Sec-Fetch-Mode", Header::SecFetchMode);
        add_header(&mut h, b"Host", Header::Host);
        add_header(&mut h, b"Cache-Control", Header::CacheControl);
        add_header(&mut h, b"Connection", Header::Connection);
        add_header(&mut h, b"User-Agent", Header::UserAgent);
        add_header(
            &mut h,
            b"Upgrade-Insecure-Requests",
            Header::UpgradeInsecureRequests,
        );
        h
    };
}

pub fn get_header(value: &[u8]) -> Result<Header, HeaderError> {
    let lowercase_value = value.to_ascii_lowercase();
    let mut ptr = &*HEADER_CHARS;
    let end_idx = lowercase_value.len() - 1;
    for (idx, char_code) in lowercase_value.iter().enumerate() {
        let search_result =
            ptr.binary_search_by(|header_char| (header_char.char_code).cmp(&char_code));
        let index = search_result.map_err(|_| HeaderError::InvalidHeader)?;
        if idx == end_idx {
            return ptr[index].header.clone().ok_or(HeaderError::InvalidHeader);
        }
        ptr = &ptr[index].next_char;
    }
    Err(HeaderError::InvalidHeader)
}

pub fn check_header_value(v: &[u8]) -> Result<(), HeaderError> {
    for e in v.iter() {
        if *e < 32 || *e > 126 {
            return Err(HeaderError::InvalidHeaderValue);
        }
    }
    Ok(())
}
