use multiplier;

mod common;

#[test]
fn it_adds_multiplies_three_and_four() {
    common::setup();
    assert_eq!(12, multiplier::multiply(3, 4));
}