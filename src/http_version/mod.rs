#[derive(Debug, PartialEq, Clone)]
pub enum HttpVersion {
    Null,
    Http11,
}

pub fn align_http_version(vec: &[u8]) -> [u8; 8] {
    if vec.len() != 8 {
        return [0; 8];
    }
    let mut buf = [0; 8];
    buf.copy_from_slice(vec);
    buf
}

#[macro_export]
macro_rules! get_http_version_enum {
    ($v: expr) => {
        match $v {
            [72, 84, 84, 80, 47, 49, 46, 49] => HttpVersion::Http11,
            _ => HttpVersion::Null,
        }
    };
}

#[test]
fn get_http_version_enum_test_get() {
    assert_eq!(
        get_http_version_enum!(align_http_version(b"HTTP/1.1")),
        HttpVersion::Http11
    );
}

#[test]
fn get_http_version_enum_test_null() {
    assert_eq!(
        get_http_version_enum!(align_http_version(b"NULL")),
        HttpVersion::Null
    );
}

#[macro_export]
macro_rules! get_http_version_enum_value {
    ($a: expr) => {
        match $a {
            HttpVersion::Http11 => vec![72u8, 84, 84, 80, 47, 49, 46, 49],
            _ => vec![],
        }
    };
}

#[test]
fn get_method_enum_value_test_get() {
    assert_eq!(
        get_http_version_enum_value!(HttpVersion::Http11),
        b"HTTP/1.1"
    );
}

#[test]
fn get_method_enum_value_test_null() {
    assert_eq!(get_http_version_enum_value!(HttpVersion::Null), b"");
}
