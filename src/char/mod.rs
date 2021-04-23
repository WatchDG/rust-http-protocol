#[macro_export]
macro_rules! rf {
    () => {
        0x0d
    };
}

#[test]
fn rf_test() {
    assert_eq!(rf!(), '\r' as u8);
}

#[macro_export]
macro_rules! lf {
    () => {
        0x0a
    };
}

#[test]
fn lf_test() {
    assert_eq!(lf!(), '\n' as u8);
}

#[macro_export]
macro_rules! sp {
    () => {
        0x20
    };
}

#[test]
fn space_bar_test() {
    assert_eq!(sp!(), ' ' as u8);
}

#[macro_export]
macro_rules! ht {
    () => {
        0x9
    };
}
