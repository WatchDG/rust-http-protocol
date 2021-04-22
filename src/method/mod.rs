#[derive(Debug, PartialEq)]
pub enum Method {
    Null,
    Get,
}

pub fn align_method(vec: &[u8]) -> [u8; 3] {
    let len = vec.len();
    if len < 3 || len > 3 {
        return [0, 0, 0];
    }
    let mut buf = [0; 3];
    buf.copy_from_slice(vec);
    buf
}

macro_rules! get_method_enum {
    ($v: expr) => {
        match $v {
            [71, 69, 84] => Method::Get,
            _ => Method::Null,
        }
    };
}

#[test]
fn get_method_enum_test_get() {
    assert_eq!(get_method_enum!(align_method(b"GET")), Method::Get);
}

#[test]
fn get_method_enum_test_null() {
    assert_eq!(get_method_enum!(align_method(b"NULL")), Method::Null);
}

macro_rules! get_method_enum_value {
    ($a: expr) => {
        match $a {
            Method::Get => vec![71u8, 69, 84],
            _ => vec![],
        }
    };
}

#[test]
fn get_method_enum_value_test_get() {
    assert_eq!(get_method_enum_value!(Method::Get), b"GET");
}

#[test]
fn get_method_enum_value_test_null() {
    assert_eq!(get_method_enum_value!(Method::Null), b"");
}
