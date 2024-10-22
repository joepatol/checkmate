use std::fmt::Debug;

use crate::core::{Should, Checked};

impl<T> Should<T> {
    pub fn be_greater_than<U: PartialOrd<T> + Debug>(self, value: U) -> Checked<T> {
        self.match_predicate(|inner| -> Checked<T> {
            if value < inner {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be > {value:?}"))
            }
        })
    }

    pub fn be_smaller_than<U: PartialOrd<T> + Debug>(self, value: U) -> Checked<T> {
        self.match_predicate(|inner| -> Checked<T> {
            if value > inner {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be < {value:?}"))
            }
        })
    }
}