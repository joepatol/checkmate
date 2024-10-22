use checkmate::prelude::*;

#[test]
fn start_with() {
    String::from("hello")
    .should()
    .start_with("he")
    .assert_ok();
}

#[test]
fn start_with_err() {
    String::from("hello")
    .should()
    .start_with("r")
    .assert_err();
}

#[test]
fn start_and_end_with() {
    String::from("hello")
    .should()
    .start_with("h")
    .and()
    .end_with("o")
    .assert_ok();
}

#[test]
fn start_and_end_with_err() {
    String::from("hello")
    .should()
    .start_with("h")
    .and()
    .end_with("i")
    .assert_err();
}

#[test]
fn contain() {
    String::from("hello")
    .should()
    .contain_substring("ello")
    .assert_ok();
}

#[test]
fn contain_any_substring() {
    String::from("hello")
    .should()
    .contain_any_of_the_substrings(["ello", "fd"])
    .assert_ok();
}

#[test]
fn contain_any_substring_err() {
    String::from("hello")
    .should()
    .contain_any_of_the_substrings(["er", "fd"])
    .assert_err();
}

#[test]
fn contain_and_start_with() {
    String::from("hello")
    .should()
    .contain_substring("ello")
    .and()
    .start_with("he")
    .assert_ok();
}

#[test]
fn be() {
    String::from("hello")
    .should()
    .be(String::from("hello"))
    .assert_ok();
}

#[test]
fn be_err() {
    String::from("hello")
    .should()
    .be(String::from("helloo"))
    .assert_err();
}

#[test]
fn be_one_of() {
    String::from("hello")
    .should()
    .be_one_of(["hello", "world"])
    .assert_ok();
}

#[test]
fn be_one_of_err() {
    String::from("hello")
    .should()
    .be_one_of(["goodbye", "world"])
    .assert_err();
}