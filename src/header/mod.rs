use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Ord, Eq, PartialOrd)]
pub enum Header {
    Null,
    Host,
    Connection,
    CacheControl,
    UpgradeInsecureRequests,
    UserAgent,
    Accept,
    SecFetchSite,
    SecFetchMode,
    SecFetchDest,
    AcceptEncoding,
    AcceptLanguage,
}

#[derive(Debug, PartialEq)]
pub struct HeaderChar {
    char_code: u8,
    next_char: Vec<HeaderChar>,
    header: Header,
}

fn add_header(header_chars: &mut Vec<HeaderChar>, header_enum_value: &[u8], header_enum: Header) {
    let mut ptr = header_chars;
    let end_idx = header_enum_value.len() - 1;
    for (idx, char_code) in header_enum_value.iter().enumerate() {
        let mut search_result =
            ptr.binary_search_by(|header_char| (header_char.char_code).cmp(&char_code));
        if search_result.is_err() {
            let header = if idx == end_idx {
                header_enum.clone()
            } else {
                Header::Null
            };
            ptr.push(HeaderChar {
                char_code: *char_code,
                next_char: Vec::new(),
                header,
            });
            ptr.sort_unstable_by(|a, b| a.char_code.cmp(&b.char_code));
            search_result =
                ptr.binary_search_by(|header_char| (header_char.char_code).cmp(&char_code));
        }
        let index = search_result.unwrap();
        ptr = &mut ptr[index].next_char;
    }
}

lazy_static! {
    pub static ref HEADER_CHARS: Vec<HeaderChar> = {
        let mut header_chars = Vec::<HeaderChar>::new();
        add_header(
            &mut header_chars,
            b"Host".to_ascii_lowercase().as_slice(),
            Header::Host,
        );
        add_header(
            &mut header_chars,
            b"Connection".to_ascii_lowercase().as_slice(),
            Header::Connection,
        );
        add_header(
            &mut header_chars,
            b"Cache-Control".to_ascii_lowercase().as_slice(),
            Header::CacheControl,
        );
        add_header(
            &mut header_chars,
            b"Upgrade-Insecure-Requests".to_ascii_lowercase().as_slice(),
            Header::UpgradeInsecureRequests,
        );
        add_header(
            &mut header_chars,
            b"User-Agent".to_ascii_lowercase().as_slice(),
            Header::UserAgent,
        );
        add_header(
            &mut header_chars,
            b"Accept".to_ascii_lowercase().as_slice(),
            Header::Accept,
        );
        add_header(
            &mut header_chars,
            b"Sec-Fetch-Site".to_ascii_lowercase().as_slice(),
            Header::SecFetchSite,
        );
        add_header(
            &mut header_chars,
            b"Sec-Fetch-Mode".to_ascii_lowercase().as_slice(),
            Header::SecFetchMode,
        );
        add_header(
            &mut header_chars,
            b"Sec-Fetch-Dest".to_ascii_lowercase().as_slice(),
            Header::SecFetchDest,
        );
        add_header(
            &mut header_chars,
            b"Accept-Encoding".to_ascii_lowercase().as_slice(),
            Header::AcceptEncoding,
        );
        add_header(
            &mut header_chars,
            b"Accept-Language".to_ascii_lowercase().as_slice(),
            Header::AcceptLanguage,
        );
        header_chars
    };
}

pub fn get_header_enum(value: &[u8]) -> Header {
    let lowercase_value = value.to_ascii_lowercase();
    let mut ptr = &*HEADER_CHARS;
    let end_idx = lowercase_value.len() - 1;
    for (idx, char_code) in lowercase_value.iter().enumerate() {
        let search_result =
            ptr.binary_search_by(|header_char| (header_char.char_code).cmp(&char_code));
        if search_result.is_err() {
            return Header::Null;
        }
        let index = search_result.unwrap();
        if idx == end_idx {
            return ptr[index].header.clone();
        }
        ptr = &ptr[index].next_char;
    }
    Header::Null
}

#[derive(Debug)]
pub struct Headers<T> {
    map: BTreeMap<Header, T>,
}

impl<T> Headers<T> {
    pub fn new() -> Self {
        Headers {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: Header, value: T) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: Header) -> Option<&T> {
        self.map.get(&key)
    }

    pub fn remove(&mut self, key: Header) {
        self.map.remove(&key);
    }
}

impl<T> Add<Headers<T>> for Headers<T> {
    type Output = Self;
    fn add(self, mut other: Self) -> Self {
        let mut map = self.map;
        map.append(&mut other.map);
        Headers { map }
    }
}

pub fn is_allowed_header_value(v: &[u8]) -> bool {
    for e in v.iter() {
        if *e < 32 || *e > 126 {
            return false;
        }
    }
    return true;
}
