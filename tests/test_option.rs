use checkmate::prelude::*;

#[test]
fn option_some() {
    Some(4)
    .should()
    .not_be_none()
    .assert_ok();
}

#[test]
fn option_none() {
    None::<usize>
    .should()
    .be_none()
    .assert_ok();
}

#[test]
fn option_should_be() {
    Some(4)
    .should()
    .be(Some(4))
    .assert_ok();
}

#[test]
fn option_should_be_some_value() {
    Some(4)
    .should()
    .be_some_with_value(4)
    .assert_ok();
}

#[test]
fn option_and() {
    Some(3)
    .should()
    .not_be_none()
    .and()
    .contain_any_of(vec![3, 4, 5])
    .assert_ok();
}

#[test]
fn option_chain_inner_value() {
    Some(4)
    .should()
    .not_be_none()
    .which_some()
    .should()
    .be_greater_than(2)
    .assert_ok();
}