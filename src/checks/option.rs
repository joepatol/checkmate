use std::fmt::Debug;

use crate::{core::{CheckState, Checked, Should}, message::format_value};

impl<T, S: CheckState<Option<T>>> Should<Option<T>, S> {
    pub fn be_none(self) -> S {
        self.match_predicate(|inner| -> Checked<Option<T>> {
            if inner.is_none() {
                Checked::valid(None)
            } else {
                Checked::invalid(inner, "Option should be None".to_string())
            }
        })
    }

    pub fn not_be_none(self) -> S {
        self.match_predicate(|inner| -> Checked<Option<T>> {
            if inner.is_none() {
                Checked::invalid(None, "Option should not be None".to_string())
            } else {
                Checked::valid(inner)
            }
        })
    }

    pub fn be_some_with_value<V: PartialEq<T> + Debug>(self, value: V) -> S 
    where
        V: 'static
    {
        self.match_predicate(|inner| -> Checked<Option<T>> {
            if inner.is_none() {
                Checked::invalid(inner, format!("Option should be Some({})", format_value(&value)))
            } else if &value != inner.as_ref().unwrap() {
                Checked::invalid(inner, format!("Option should be Some({})", format_value(&value)))
            } else {
                Checked::valid(inner)
            }
        })
    }
}