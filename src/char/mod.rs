#[macro_export]
macro_rules! cr {
    () => {
        0x0du8
    };
}

#[test]
fn rf_test() {
    assert_eq!(cr!(), '\r' as u8);
}

#[macro_export]
macro_rules! lf {
    () => {
        0x0au8
    };
}

#[test]
fn lf_test() {
    assert_eq!(lf!(), '\n' as u8);
}

#[macro_export]
macro_rules! sp {
    () => {
        0x20u8
    };
}

#[test]
fn space_bar_test() {
    assert_eq!(sp!(), ' ' as u8);
}

#[macro_export]
macro_rules! ht {
    () => {
        0x9u8
    };
}
