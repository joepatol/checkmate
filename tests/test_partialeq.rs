use checkmate::prelude::*;

#[derive(Debug, PartialEq)]
enum MyEnum {
    OptionA,
    OptionB,
}

#[test]
fn be_int() {
    11
    .value()
    .should()
    .be(11)
    .assert_ok();
}

#[test]
fn be_int_err() {
    11
    .value()
    .should()
    .be(12)
    .assert_err_with_message("Should be '12'");
}

#[test]
fn be_int_err_custom_message() {
    11
    .value()
    .should()
    .be(12)
    .with_message("This value should be '12'")
    .assert_err_with_message("This value should be '12'");
}

#[test]
fn be_one_of() {
    10
    .value()
    .should()
    .be_one_of([11, 12, 10])
    .assert_ok();
}

#[test]
fn be_one_of_err() {
    9
    .value()
    .should()
    .be_one_of([11, 12, 10])
    .with_message("Should be in the list!")
    .assert_err_with_message("Should be in the list!");
}

#[test]
fn test_enum() {
    let value = MyEnum::OptionA;

    value
    .value()
    .should()
    .be(MyEnum::OptionA)
    .assert_ok();
}

#[test]
fn test_err() {
    let value = MyEnum::OptionA;

    value
    .value()
    .should()
    .be(MyEnum::OptionB)
    .assert_err_with_message("Should be 'OptionB'");
}