use std::collections::HashMap;

use checkmate::{prelude::*, times::Exactly};

#[test]
fn hashmap_iterator_ok() {
    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);

    map
    .value()
    .should()
    .contain(("b", 2), Exactly::once())
    .assert_valid();
}

#[test]
fn hashmap_iterator_err() {
    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);

    map
    .value()
    .should()
    .contain(("d", 2), Exactly::once())
    .assert_invalid_with_message("Iterator should contain <type does not implement Display or Debug> exactly 1 times");
}

#[test]
fn hashmap_contain_key() {
    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);

    map.value().should().have_count(3).and().contain_key("b").assert_valid();
}

#[test]
fn hashmap_contain_value() {
    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);

    map.value().should().contain_value(2).assert_valid();
}
