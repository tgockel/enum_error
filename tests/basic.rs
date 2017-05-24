#[macro_use]
extern crate enum_error;

use std::error::Error;
use std::string::ToString;

#[derive(Debug, EnumDisplay, EnumError)]
enum BasicError {
    A,
    B,
    C,
}

#[test]
fn display() {
    assert_eq!("A", BasicError::A.to_string());
    assert_eq!("B", BasicError::B.to_string());
    assert_eq!("C", BasicError::C.to_string());
}

#[test]
fn description() {
    assert_eq!("A", BasicError::A.description());
    assert_eq!("B", BasicError::B.description());
    assert_eq!("C", BasicError::C.description());
}
