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