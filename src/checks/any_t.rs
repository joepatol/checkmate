use std::fmt::Debug;

use crate::core::{Checked, Should};

impl<T> Should<T> {
    pub fn match_predicate(self, predicate: impl FnOnce(T) -> Checked<T>) -> Checked<T> {
        match self.check_state {
            Checked::Valid { value } => {
                (predicate)(value)
            },
            Checked::Invalid { value, message } => {
                Checked::invalid(value, message.clone())
            }
        }
    }

    pub fn be<U: PartialEq<T> + Debug>(self, value: U) -> Checked<T> {
        self.match_predicate(|inner| {
            if value == inner {
                Checked::valid(inner)
            } else {
                Checked::invalid(inner, format!("Should be {value:?}"))
            }
        })
    }

    pub fn be_one_of<U: PartialEq<T> + Debug>(self, values: impl IntoIterator<Item = U> + std::fmt::Debug) -> Checked<T> {
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