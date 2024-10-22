use std::error::Error;
use std::fmt;

use checkmate::prelude::*;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {}

#[test]
fn have_error_message() {
    let the_result: Result<usize, MyError> = Result::Err(MyError::new("Oh no"));

    the_result
    .should()
    .be_err()
    .which_err()
    .should()
    .have_message("Oh no")
    .assert_ok();
}

#[test]
fn ok_should_be() {
    let the_result: Result<usize, MyError> = Result::Ok(3);

    the_result
    .should()
    .be_ok()
    .which_ok()
    .should()
    .be(3)
    .assert_ok();
}