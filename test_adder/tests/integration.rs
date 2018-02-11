extern crate test_adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_adder::add_two(2));
    common::teardown();
}