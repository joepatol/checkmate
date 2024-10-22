use std::collections::HashMap;

use checkmate::prelude::*;

#[test]
fn vec_contain() {
    let c = vec![1, 2, 3];

    c
        .should()
        .contain(1, AtLeast::once())
        .and()
        .contain_any_of(vec![2, 3, 4])
        .which()
        .len()
        .should()
        .be(3)
        .assert_ok();
}

#[test]
fn list_contain() {
    let list = &[1, 2, 3];

    list
    .should()
    .contain(&2, AtLeast::once())
    .assert_ok();
}

#[test]
fn list_contain_and_err() {
    let list = &[1, 2, 3];

    list
    .should()
    .contain(&3, AtLeast::once())
    .and()
    .contain(&4, AtLeast::once())
    .assert_err_with_message("Should contain 4 at least 1 times");
}

#[test]
fn contain_at_least() {
    let list = &[1, 2, 3, 3];

    list
    .should()
    .contain(&3, AtLeast::twice())
    .assert_ok();
}

#[test]
fn contain_at_least_err() {
    let list = &[1, 2, 3, 3];

    list
    .should()
    .contain(&3, AtLeast::times(3))
    .assert_err_with_message("Should contain 3 at least 3 times");
}

#[test]
fn contain_at_least_once() {
    let list = &[1, 2, 3, 3];

    list
    .should()
    .contain(&3, AtLeast::once())
    .assert_ok();
}

#[test]
fn contain_at_least_once_err() {
    let list = &[1, 2, 3, 3];

    list
    .should()
    .contain(&5, AtLeast::once())
    .assert_err();
}

#[test]
fn hashmap_contain() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    map
    .should()
    .contain(("a", 1), Exactly::once())
    .assert_ok();
}