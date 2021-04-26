use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum StatusCode {
    S200,
    CE400,
}

impl From<StatusCode> for Bytes {
    fn from(s: StatusCode) -> Self {
        match s {
            StatusCode::S200 => Bytes::from_static(b"200"),
            StatusCode::CE400 => Bytes::from_static(b"400"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReasonPhrase {
    Ok,
}

impl From<ReasonPhrase> for Bytes {
    fn from(r: ReasonPhrase) -> Self {
        match r {
            ReasonPhrase::Ok => Bytes::from_static(&[79u8, 75]),
        }
    }
}

pub fn get_reason_phrase_by_status_code(s: &StatusCode) -> Option<ReasonPhrase> {
    match s {
        StatusCode::S200 => Some(ReasonPhrase::Ok),
        _ => None,
    }
}
