use checkmate::prelude::*;
use checkmate::Checked;

struct MyStruct {
    attr_1: usize,
    attr_2: String,
}

#[test]
fn check_struct_fields() {
    let s = MyStruct { attr_1: 10, attr_2: String::from("hello") };

    s.value()
    .then(|s| -> Checked<String> {
        s.attr_2.value_cloned().should().be("hello")
    })
    .then(|s| -> Checked<usize> {
        s.attr_1.value().should().be_greater_than(5)
    })
    .assert_valid();
}

#[test]
fn check_vec() {
    let vector = vec![1, 2, 3, 4, 5];

    vector.value().should().have_count(5)
        .then(|v| -> Checked<i32> {
            v[2].value().should().be(3).with_message("This element should be '3'")
        })
        .and()
        .contain_all_of([1, 2, 3])
        .assert_valid();
}

#[test]
fn check_vec_err() {
    let vector = vec![1, 2, 3, 4, 5];

    vector.value().should().have_count(5)
        .then(|v| -> Checked<i32> {
            v[3].value().should().be(3).with_message("This element should be '3'")
        })
        .and()
        .contain_all_of([1, 2, 3])
        .assert_invalid_with_message("This element should be '3'");
}