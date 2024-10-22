use std::collections::HashMap;

use checkmate::prelude::*;

#[derive(Debug, PartialEq)]
enum MyEnum {
    OptionA,
    OptionB,
}

#[test]
fn enumerated() {
    let value = MyEnum::OptionA;

    value
    .should()
    .be(MyEnum::OptionA)
    .assert_ok();
}

#[test]
fn enumerated_err() {
    let value = MyEnum::OptionA;

    value
    .should()
    .be(MyEnum::OptionB)
    .assert_err();
}

#[test]
fn i32_err() {
    14
    .should()
    .be(15)
    .assert_err();
}

#[test]
fn i32() {
    14
    .should()
    .be(14)
    .assert_ok();
}

#[test]
fn ref_i32() {
    let num: &i32 = &12;
    num
    .should()
    .be(&12)
    .assert_ok();
}

#[test]
fn hashmap() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let mut map2 = HashMap::new();
    map2.insert("a", 1);
    map2.insert("b", 2);

    map
    .should()
    .be(map2)
    .assert_ok();
}

#[test]
fn hashmap_where() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    map
    .should()
    .have_count(2)
    .where_(|map| 
        map
        .get("a")
        .should()
        .not_be_none()
        .which_some()
        .to_owned()
        .should()
        .be(1)
    )
    .assert_ok();
}

#[test]
fn hashmap_where_none() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    map
    .should()
    .have_count(2)
    .where_(|map| 
        map
        .clone()
        .remove("c")
        .should()
        .be_none()
    )
    .assert_ok();
}

#[test]
fn hashmap_where_none_err_with_message() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    map
    .should()
    .have_count(2)
    .where_(|map| 
        map
        .clone()
        .remove("a")
        .should()
        .be_some_with_value(1)
        .with_message("Key 'a' should be 1".to_string())
    )
    .where_(|map| 
        map
        .clone()
        .remove("c")
        .should()
        .not_be_none()
        .with_message("Key 'c' should not be none".to_string())
    )
    .assert_err_with_message("Key 'c' should not be none");
}

#[test]
fn hashmap_where_wrong_value_with_message() {
    let mut map = HashMap::new();
    map.insert("a", 3);
    map.insert("b", 2);

    map
    .should()
    .have_count(2)
    .where_(|map| 
        map
        .clone()
        .remove("a")
        .should()
        .be_some_with_value(1)
        .with_message("Key 'a' should be 1".to_string())
    )
    .where_(|map| 
        map
        .clone()
        .remove("c")
        .should()
        .not_be_none()
        .with_message("Key 'c' should not be none".to_string())
    )
    .assert_err_with_message("Key 'a' should be 1");
}


#[test]
fn hashmap_err() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let mut map2 = HashMap::new();
    map2.insert("a", 1);
    map2.insert("b", 3);

    map
    .should()
    .be(map2)
    .assert_err();
}