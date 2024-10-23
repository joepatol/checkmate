use std::fmt::Debug;

use crate::core::{CheckState, Checked, Should};

impl<T, S: CheckState<T>> Should<T, S> {
    pub fn be<U: PartialEq<T> + Debug>(self, value: U) -> S {
        self.match_predicate(|inner| {
            if value == inner {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be '{value:?}'"))
            }
        })
    }

    pub fn be_one_of<U: PartialEq<T> + Debug>(self, values: impl IntoIterator<Item = U> + std::fmt::Debug) -> S {
        let msg = format!("Should be one of {values:?}");
        self.match_predicate(|inner| -> Checked<T> {
            for value in values.into_iter(){
                if value == inner {
                    return Checked::valid(inner)
                };
            };
            Checked::invalid(inner, msg)
        })
    }
}