use integrationtests::adder;
mod common;
use pretty_assertions::assert_eq;
#[test]
fn test_adder() {
    common::setup();
    assert_eq!(adder(3,2),5);
}
