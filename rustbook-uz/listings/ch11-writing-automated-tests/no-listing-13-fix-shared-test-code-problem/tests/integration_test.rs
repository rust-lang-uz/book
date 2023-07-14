use qoshuvchi;

mod common;

#[test]
fn ikkita_qoshish() {
    common::setup();
    assert_eq!(4, qoshuvchi::ikkita_qosh(2));
}
