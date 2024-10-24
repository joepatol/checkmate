use checkmate::prelude::*;
use checkmate::times::Exactly;

#[derive(Debug, Clone)]
struct MyStruct {
    attr_1: usize,
    attr_2: String,
    coll: Vec<f32>,
}

impl IntoIterator for MyStruct {
    type Item = f32;
    type IntoIter = <Vec<f32> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.coll.into_iter()
    }
}

#[test]
fn check_struct_fields_which() {
    let s = MyStruct { attr_1: 10, attr_2: String::from("hello"), coll: vec![1.2, 3.4, 5.6] };

    s.value()
        .should()
        .contain(1.2, Exactly::once())
        .also(|s| s.attr_1).should().be(10).done()
        .also(|s| s.attr_2.clone()).should().have_length(5).and().start_with("he").done()
        .assert_valid();
}