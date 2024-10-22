use std::fmt::Debug;

use crate::core::{Should, Checked};

impl<T> Should<Option<T>> {
    pub fn be_none(self) -> Checked<Option<T>> {
        self.match_predicate(|inner| -> Checked<Option<T>> {
            if inner.is_none() {
                Checked::valid(None)
            } else {
                Checked::invalid(inner, "Should be None".to_string())
            }
        })
    }

    pub fn not_be_none(self) -> Checked<Option<T>> {
        self.match_predicate(|inner| -> Checked<Option<T>> {
            if inner.is_none() {
                Checked::invalid(None, "Should not be None".to_string())
            } else {
                Checked::valid(inner)
            }
        })
    }

    pub fn be_some_with_value<V: PartialEq<T> + Debug>(self, value: V) -> Checked<Option<T>> {
        self.match_predicate(|inner| -> Checked<Option<T>> {
            if inner.is_none() {
                Checked::invalid(inner, format!("Should be Some({value:?})"))
            } else if &value != inner.as_ref().unwrap() {
                Checked::invalid(inner, format!("Should be Some({value:?})"))
            } else {
                Checked::valid(inner)
            }
        })
    }
}